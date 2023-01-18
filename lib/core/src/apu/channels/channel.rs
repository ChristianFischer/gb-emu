/*
 * Copyright (C) 2022-2023 by Christian Fischer
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use flagset::{flags, FlagSet};
use crate::apu::apu::ApuState;
use crate::apu::channels::channel::features::{FEATURE_FREQUENCY_SWEEP_DISABLED, FEATURE_LENGTH_TIMER_DISABLED, FEATURE_VOLUME_ENVELOPE_DISABLED};
use crate::apu::channels::envelope::Envelope;
use crate::apu::channels::freq_sweep::{FrequencySweep, FrequencySweepResult};
use crate::apu::channels::generator::SoundGenerator;
use crate::apu::channels::length_timer::LengthTimer;
use crate::apu::dac::DigitalAudioConverter;
use crate::apu::registers::{ApuChannelRegisters, ApuRegisters};
use crate::gameboy::Clock;

pub mod features {
    pub const FEATURE_LENGTH_TIMER_DISABLED : u8        = 0;
    pub const FEATURE_LENGTH_TIMER_6_BIT : u8           = 6;
    pub const FEATURE_LENGTH_TIMER_8_BIT : u8           = 8;

    pub const FEATURE_FREQUENCY_SWEEP_DISABLED : u8     = 0;
    pub const FEATURE_FREQUENCY_SWEEP_ENABLED : u8      = 1;

    pub const FEATURE_VOLUME_ENVELOPE_DISABLED : u8     = 0;
    pub const FEATURE_VOLUME_ENVELOPE_ENABLED : u8      = 1;
}


/// The type of a channel.
pub enum ChannelType {
    Ch1Pulse1,
    Ch2Pulse2,
    Ch3Wave,
    Ch4Noise,
}


flags! {
    /// An action to be performed as the result of a `on_trigger` or `on_register_changed`
    /// invocation of a `ChannelComponent`.
    pub enum TriggerAction : u8 {
        /// No particular action to be done.
        None,

        /// The channels DAC should be enabled.
        EnableDac,

        /// The channels DAC should be disabled.
        /// As a consequence, the channel itself will be disabled as well.
        DisableDac,

        /// The channel will be disabled.
        /// This flag may be set indirectly after the DAC was disabled before.
        DisableChannel,
    }
}

/// A set of trigger actions returned by a trigger or register changed event.
pub type TriggerActionSet = FlagSet<TriggerAction>;


/// A trait for any component used inside an audio channel.
/// This trait allows components to receive changes on their registers and to get notified
/// when a channel was triggered by setting the trigger bit.
pub trait ChannelComponent {
    /// Called when the value of a register was changed by writing on it.
    fn on_register_changed(&mut self, number: u16, registers: &ApuChannelRegisters, apu_state: &ApuState) -> TriggerAction {
       default_on_register_changed(number, registers, apu_state)
    }

    /// Called when the channel was triggered by setting bit 7 of it's NRx4 register.
    /// This should start the channel to generate sound.
    fn on_trigger_event(&mut self, apu_state: &ApuState) -> TriggerAction {
        default_on_trigger_event(apu_state)
    }
}


/// Placeholder for `on_register_changed` implementations, which do not result in any special behaviour.
pub fn default_on_register_changed(number: u16, registers: &ApuChannelRegisters, apu_state: &ApuState) -> TriggerAction {
    _ = (number, registers, apu_state);
    TriggerAction::None
}


/// Placeholder for `on_trigger_event` implementations, which do not result in any special behaviour.
pub fn default_on_trigger_event(apu_state: &ApuState) -> TriggerAction {
    _ = apu_state;
    TriggerAction::None
}



/// Represents a single channel inside the GameBoy APU.
/// Each channel contains a distinct sound generator which generates
/// an audio signal and a DAC to convert the digital value into an
/// analogue sound wave.
pub struct Channel<
    G : SoundGenerator,
    const FEATURE_LENGTH_TIMER : u8,
    const FEATURE_FREQUENCY_SWEEP : u8,
    const FEATURE_VOLUME_ENVELOPE : u8,
> {
    /// Stores whether the channel is enabled or not.
    channel_enabled: bool,

    /// The current channels type.
    channel_type: ChannelType,

    /// The sound generator associated with this channel.
    generator: G,

    /// A length timer controlling how long the sound generator will run.
    length_timer: LengthTimer<FEATURE_LENGTH_TIMER>,

    /// A frequency sweep function to modify the generators wave length.
    freq_sweep: FrequencySweep,

    /// An envelope function to provide the volume for the generated wave.
    vol_envelope: Envelope,

    /// A digital audio converter to convert the digital sound value
    /// into a sound wave.
    dac: DigitalAudioConverter,
}


impl<
    G : SoundGenerator,
    const FEATURE_LENGTH_TIMER: u8,
    const FEATURE_FREQUENCY_SWEEP : u8,
    const FEATURE_VOLUME_ENVELOPE : u8,
> Channel<
    G,
    FEATURE_LENGTH_TIMER,
    FEATURE_FREQUENCY_SWEEP,
    FEATURE_VOLUME_ENVELOPE,
> {
    /// Creates a new sound channel with it's type and sound generator instance.
    pub fn new(channel_type: ChannelType) -> Self {
        Self {
            channel_enabled: true,
            channel_type,

            generator:      G::create(),

            length_timer:   LengthTimer::default(),
            freq_sweep:     FrequencySweep::default(),
            vol_envelope:   Envelope::default(),

            dac:            DigitalAudioConverter::new(),
        }
    }


    /// Checks whether this channel has a length timer.
    pub fn has_feature_length_timer() -> bool {
        FEATURE_LENGTH_TIMER != FEATURE_LENGTH_TIMER_DISABLED
    }


    /// Checks whether this channel has frequency sweep.
    pub fn has_feature_frequency_sweep() -> bool {
        FEATURE_FREQUENCY_SWEEP != FEATURE_FREQUENCY_SWEEP_DISABLED
    }


    /// Checks whether this channel has a volume envelope.
    pub fn has_feature_volume_envelope() -> bool {
        FEATURE_VOLUME_ENVELOPE != FEATURE_VOLUME_ENVELOPE_DISABLED
    }


    /// Get the type of this channel.
    pub fn get_channel_type(&self) -> &ChannelType {
        &self.channel_type
    }


    /// Get the ordinal number of this channel, starting with zero.
    /// So CH1 has the ordinal 0, CH2 ordinal 1 and so on.
    pub fn get_channel_ordinal(&self) -> u8 {
        match self.channel_type {
            ChannelType::Ch1Pulse1 => 0,
            ChannelType::Ch2Pulse2 => 1,
            ChannelType::Ch3Wave   => 2,
            ChannelType::Ch4Noise  => 3,
        }
    }


    /// Checks whether the current channel is enabled or not.
    pub fn is_channel_enabled(&self) -> bool {
        self.channel_enabled
    }


    /// Get the sound generator of this channel.
    pub fn get_generator_mut(&mut self) -> &mut G {
        &mut self.generator
    }


    /// Invokes a functor on each active component of this channel,
    /// including the generator component.
    fn for_each_component<F, T>(&mut self, mut func: F) -> TriggerActionSet
        where F : FnMut(&mut dyn ChannelComponent) -> T,
              T : Into<FlagSet<TriggerAction>>
    {
        let mut results: TriggerActionSet = Default::default();

        if Self::has_feature_length_timer() {
            results |= func(&mut self.length_timer);
        }

        if Self::has_feature_frequency_sweep() {
            results |= func(&mut self.freq_sweep);
        }

        if Self::has_feature_volume_envelope() {
            results |= func(&mut self.vol_envelope);
        }

        results |= func(&mut self.generator);

        results
    }


    /// Applies a set of actions delivered by a call to `on_trigger` or `on_register_changed`
    /// events. A modified set of the actually applied results will be returned.
    fn apply_actions(&mut self, actions: impl Into<TriggerActionSet>) -> TriggerActionSet {
        // create a mutable clone of the initial set to store the actually applied actions
        let mut actions_applied : TriggerActionSet = actions.into();

        // enable DAC
        if actions_applied.contains(TriggerAction::EnableDac) {
            self.dac.set_enabled(true);
        }

        // disable DAC
        if actions_applied.contains(TriggerAction::DisableDac) {
            self.dac.set_enabled(false);

            // disabling DAC also disables the channel
            actions_applied |= TriggerAction::DisableChannel;
        }

        // disable the channel
        if actions_applied.contains(TriggerAction::DisableChannel) {
            self.channel_enabled = false;
        }

        actions_applied
    }


    /// Fires the notification when a register of this channel was written to.
    pub fn fire_register_changed(&mut self, number: u16, registers: &ApuRegisters, apu_state: &ApuState) -> TriggerActionSet {
        let channel_registers = &registers.channels[self.get_channel_ordinal() as usize];
        let actions = self.for_each_component(
            |c| c.on_register_changed(number, channel_registers, apu_state)
        );

        // apply requested actions
        let actions_applied = self.apply_actions(actions);

        actions_applied
    }


    /// Fires the trigger event when the channel was triggered by writing NRx4 bit 7.
    pub fn fire_trigger_event(&mut self, apu_state: &ApuState) -> TriggerActionSet {
        self.channel_enabled = true;

        let actions = self.for_each_component(
            |c| c.on_trigger_event(apu_state)
        );

        // apply requested actions
        let actions_applied = self.apply_actions(actions);

        actions_applied
    }


    /// Called by the frame sequencer to update the channels sound length timer.
    pub fn tick_length_timer(&mut self) {
        if Self::has_feature_length_timer() {
            let action = self.length_timer.tick();

            self.apply_actions(action);
        }
    }


    /// Called by the frame sequencer to update the frequency sweep of channel 1.
    pub fn tick_freq_sweep(&mut self) {
        if Self::has_feature_frequency_sweep() {
            let frequency = self.generator.get_frequency();
            let result    = self.freq_sweep.tick(frequency);

            match result {
                // the channel was disabled because of an overflow
                FrequencySweepResult::DisableChannel => {
                    self.channel_enabled = false;
                }

                // apply the changed frequency
                FrequencySweepResult::FrequencyChanged(new_wave_length) => {
                    self.generator.set_frequency(new_wave_length);
                }

                _ => { }
            }
        }
    }


    /// Called by the frame sequencer to update the envelope function.
    pub fn tick_envelope_sweep(&mut self) {
        if Self::has_feature_volume_envelope() {
            self.vol_envelope.tick();
        }
    }


    /// Updates the current channel with the time passed since last call.
    pub fn update(&mut self, cycles: Clock) {
        self.generator.update(cycles);
    }


    /// Get the audio sample generated by the channels sound generator and
    /// converted by the channels DAC.
    pub fn get_sample(&self, registers: &ApuRegisters) -> i16 {
        let value = if self.channel_enabled {
            // take the current sample from the sound generator
            let generated_sample = self.generator.get_sample(registers);

            // get the volume level from the envelope function, if available
            let volume = if Self::has_feature_volume_envelope() {
                self.vol_envelope.get_current_volume()
            }
            else {
                1
            };

            // compute the samples amplitude, modified by volume
            generated_sample * volume
        }
        else {
            // a disabled channel just spawns zero
            0
        };

        // convert into 'analogue' signal via DAC
        let sample = self.dac.convert(value);

        sample
    }
}

