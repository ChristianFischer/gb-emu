/*
 * Copyright (C) 2022-2024 by Christian Fischer
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
use crate::apu::sample::{Sample, SampleResult};
use crate::gameboy::Clock;
use crate::utils::get_bit;


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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// Checks whether this component can be written to in the current state.
    /// The default implementation disallows write while the APU is turned off, which
    /// is the case for most components, except the length timer.
    fn can_write_register(&self, number: u16, apu_state: &ApuState) -> bool {
        _ = number;
        apu_state.apu_on
    }

    /// Called to read the value of a register.
    fn on_read_register(&self, number: u16, apu_state: &ApuState) -> u8 {
        default_on_read_register(number, apu_state)
    }

    /// Called when the value of a register was written.
    fn on_write_register(&mut self, number: u16, value: u8, apu_state: &ApuState) -> TriggerAction {
       default_on_write_register(number, value, apu_state)
    }

    /// Called when the channel was triggered by setting bit 7 of it's NRx4 register.
    /// This should start the channel to generate sound.
    fn on_trigger_event(&mut self, apu_state: &ApuState) -> TriggerAction {
        default_on_trigger_event(apu_state)
    }

    /// Called when this channel was disabled.
    fn on_channel_disabled(&mut self) { }

    /// Called when the APU was reset by turning it off.
    /// It's expected to every component to set it's data to '0'.
    fn on_reset(&mut self, apu_state: &ApuState);
}


/// Placeholder for `on_write_register` implementations, which do not result in any special behaviour.
pub fn default_on_write_register(number: u16, value: u8, apu_state: &ApuState) -> TriggerAction {
    _ = (number, value, apu_state);
    TriggerAction::None
}


/// Placeholder for `on_read_register` implementations, which do not result in any special behaviour.
pub fn default_on_read_register(number: u16, apu_state: &ApuState) -> u8 {
    _ = (number, apu_state);
    0x00
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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


    /// Get the channel's DAC.
    pub fn get_dac(&self) -> &DigitalAudioConverter {
        &self.dac
    }


    /// Invokes a functor on each active component of this channel,
    /// including the generator component.
    /// Each component is readonly and is expected to return a numeric
    /// value as a result of this operation.
    fn for_each_component<F, T>(&self, func: F) -> T
        where F : Fn(&dyn ChannelComponent) -> T,
              T : Default + core::ops::BitOr + core::ops::BitOrAssign
    {
        let mut results: T = Default::default();

        if Self::has_feature_length_timer() {
            results |= func(&self.length_timer);
        }

        if Self::has_feature_frequency_sweep() {
            results |= func(&self.freq_sweep);
        }

        if Self::has_feature_volume_envelope() {
            results |= func(&self.vol_envelope);
        }

        results |= func(&self.generator);

        results
    }


    /// Invokes a functor on each active component of this channel,
    /// including the generator component.
    /// Any component will be mutable and is expected to return a TriggerAction
    /// to invoke actions in result of this call.
    fn for_each_component_mut<F, T>(&mut self, mut func: F) -> TriggerActionSet
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
            self.disable_channel();
        }

        actions_applied
    }


    /// Reads from a register which belongs to this channel.
    pub fn on_read_register(&self, number: u16, apu_state: &ApuState) -> u8 {
        self.for_each_component(|c| c.on_read_register(number, apu_state))
    }


    /// Writes to a register which belongs to this channel.
    /// When NRx4 bit 7 was set, this will also fire the trigger event for this channel.
    pub fn on_write_register(&mut self, number: u16, value: u8, apu_state: &ApuState) -> TriggerActionSet {
        let mut actions = self.for_each_component_mut(
            |c| {
                // block writing to this component, if not allowed
                if !c.can_write_register(number, apu_state) {
                    return TriggerAction::None;
                }

                c.on_write_register(number, value, apu_state)
            }
        );

        // a channel may only be triggered, if the APU is not turned off
        if apu_state.apu_on {
            // check whether the trigger bit was set
            if number == 4 && get_bit(value, 7) {
                actions |= self.fire_trigger_event(apu_state);
            }

            // apply requested actions
            actions = self.apply_actions(actions);
        }

        actions
    }


    /// Fires the trigger event when the channel was triggered by writing NRx4 bit 7.
    fn fire_trigger_event(&mut self, apu_state: &ApuState) -> TriggerActionSet {
        self.channel_enabled = true;

        // when the channel has a frequency sweep, it will initialize it's shadow frequency
        // at the beginning of it's trigger event
        if Self::has_feature_frequency_sweep() {
            let frequency = self.generator.get_frequency();
            self.freq_sweep.init_shadow_frequency(frequency);
        }

        // invoke trigger event for each component
        let mut actions = self.for_each_component_mut(
            |c| c.on_trigger_event(apu_state)
        );

        // DAC disabled prevents the channel from being enabled
        if !self.dac.is_enabled() {
            actions |= TriggerAction::DisableChannel;
        }

        actions
    }


    /// Disables this channel and notifies all components.
    fn disable_channel(&mut self) {
        // disables the channel
        self.channel_enabled = false;

        // notify all components
        self.for_each_component_mut(
            |c| {
                c.on_channel_disabled();
                TriggerAction::None
            }
        );
    }


    /// Reset this channel when the APU was turned off.
    pub fn reset(&mut self, apu_state: &ApuState) {
        // notify each component to reset
        self.for_each_component_mut(
            |c| {
                c.on_reset(apu_state);
                TriggerAction::None
            }
        );

        // reset DAC
        self.dac.on_reset();

        // channels become disabled on reset
        self.channel_enabled = false;
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
            let result = self.freq_sweep.tick();

            match result {
                // the channel was disabled because of an overflow
                FrequencySweepResult::DisableChannel => {
                    self.channel_enabled = false;
                }

                // apply the changed frequency
                FrequencySweepResult::FrequencyChanged(new_frequency) => {
                    self.generator.set_frequency(new_frequency);
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
    pub fn get_sample(&self, apu_state: &ApuState) -> SampleResult<Sample> {
        let value = if self.channel_enabled {
            // take the current sample from the sound generator
            let generated_sample = self.generator.get_sample(apu_state);

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

