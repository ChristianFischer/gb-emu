/*
 * Copyright (C) 2022-2025 by Christian Fischer
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

use crate::apu::audio_output::AudioOutput;
use crate::apu::channels::channel::features::*;
use crate::apu::channels::channel::{Channel, ChannelType};
use crate::apu::channels::noise::NoiseGenerator;
use crate::apu::channels::pulse::PulseGenerator;
use crate::apu::channels::wave::WaveGenerator;
use crate::apu::mixer::Mixer;
use crate::device_type::DeviceConfig;
use crate::emulator_core::Clock;
use crate::mmu::locations::*;
use crate::mmu::memory_bus::MemoryBusConnection;
use crate::utils::{as_bit_flag, get_bit};


pub const APU_UPDATE_PERIOD : Clock = 8_192;

const NR52_NON_READABLE_BITS : u8   = 0b_0111_0000;


type Channel1 = Channel<
    PulseGenerator,
    FEATURE_LENGTH_TIMER_6_BIT,
    FEATURE_FREQUENCY_SWEEP_ENABLED,
    FEATURE_VOLUME_ENVELOPE_ENABLED,
>;

type Channel2 = Channel<
    PulseGenerator,
    FEATURE_LENGTH_TIMER_6_BIT,
    FEATURE_FREQUENCY_SWEEP_DISABLED,
    FEATURE_VOLUME_ENVELOPE_ENABLED,
>;

type Channel3 = Channel<
    WaveGenerator,
    FEATURE_LENGTH_TIMER_8_BIT,
    FEATURE_FREQUENCY_SWEEP_DISABLED,
    FEATURE_VOLUME_ENVELOPE_DISABLED,
>;

type Channel4 = Channel<
    NoiseGenerator,
    FEATURE_LENGTH_TIMER_6_BIT,
    FEATURE_FREQUENCY_SWEEP_DISABLED,
    FEATURE_VOLUME_ENVELOPE_ENABLED,
>;


/// The current state of the APU.
/// This includes the information, if the APU is enabled or not
/// and information about the current frame sequencer state.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApuState {
    pub apu_on: bool,

    /// Current device config
    pub device_config: DeviceConfig,

    /// Frame Sequencer clock
    pub fs_clock: Clock,

    /// The frame sequencer to activate channel components periodically.
    pub fs: FrameSequencer,
}


/// The frame sequencer holds a counter which is tied to the device's DIV counter
/// to periodically activate and deactivate components of the APU channels.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FrameSequencer {
    /// The next step of the frame sequencer.
    /// In real hardware, this is a 3 bit value. Here we're using a 8 bit integer
    /// and just ignoring the higher 5 bits on reading.
    pub fs_next_step: u8,
}


/// Represents the GameBoys Audio Processing Unit.
/// The APU contains various components
/// * 4 Channels with each their distinct sound generator to create sound waves.
/// * A frame sequencer to periodically trigger subcomponents of each sound generator.
/// * A mixer to mix the sound waves generated by each channel into left and right
/// output channels.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Apu {
    /// Information about the APUs current state.
    state: ApuState,

    ch1: Channel1,
    ch2: Channel2,
    ch3: Channel3,
    ch4: Channel4,

    /// The mixer used to mix the signals of each input channel into stereo output channels
    mixer: Mixer,

    /// An object receiving audio data to provide audio samples to the emulator frontend.
    audio_output: AudioOutput,
}


impl FrameSequencer {
    pub fn new() -> Self {
        Self {
            fs_next_step: 0,
        }
    }


    /// Resets the frame sequencer next step to zero.
    pub fn reset(&mut self) {
        self.fs_next_step = 0;
    }


    /// Let the frame sequencer increment it's internal counter.
    /// This changes the state which channel components will be active or not.
    /// This does NOT invoke the channel components themself.
    pub fn increment(&mut self) {
        self.fs_next_step = self.fs_next_step.wrapping_add(1);
    }


    /// Checks if the length timer component is active.
    /// The sound length timer has a tick rate of 256Hz and therefor is
    /// activated every 2nd tick of the frame sequencer.
    pub fn is_length_timer_active(&self) -> bool {
        (self.fs_next_step & 0b0001) == 0
    }


    /// Checks if the frequency sweep component is active.
    /// The frequency sweep has a tick rate of 128Hz and therefor is
    /// activated every 4th tick of the frame sequencer.
    pub fn is_freq_sweep_active(&self) -> bool {
        (self.fs_next_step & 0b0011) == 0b010
    }


    /// Checks if the volume envelope component is active.
    /// The volume envelope has a tick rate of 64Hz and therefor is
    /// activated every 8th tick of the frame sequencer.
    pub fn is_volume_envelope_active(&self) -> bool {
        (self.fs_next_step & 0b0111) == 0b0111
    }
}


impl Apu {
    /// Creates a new APU object.
    pub fn new(device_config: DeviceConfig) -> Self {
        Self {
            state: ApuState {
                apu_on:     true,
                fs_clock:   0,
                fs:         FrameSequencer::new(),
                device_config,
            },

            ch1: Channel::new(ChannelType::Ch1Pulse1),
            ch2: Channel::new(ChannelType::Ch2Pulse2),
            ch3: Channel::new(ChannelType::Ch3Wave),
            ch4: Channel::new(ChannelType::Ch4Noise),

            mixer: Mixer::new(),

            audio_output: AudioOutput::new(device_config),
        }
    }


    /// Updates the APUs internal components with the time passed.
    pub fn update(&mut self, cycles: Clock) {
        if self.state.apu_on {
            self.update_frame_sequencer(cycles);
        }

        self.update_channels(cycles);
    }


    /// Updates the frame sequencer with the time passed.
    /// This will periodically trigger some sound generator subcomponents.
    fn update_frame_sequencer(&mut self, cycles: Clock) {
        self.state.fs_clock = self.state.fs_clock.wrapping_add(cycles);

        while self.state.fs_clock >= APU_UPDATE_PERIOD {
            self.state.fs_clock -= APU_UPDATE_PERIOD;
            self.next_frame_sequencer_step();
        }
    }


    /// Process the next step of the frame sequencer to trigger sound generator subcomponents.
    fn next_frame_sequencer_step(&mut self) {
        // 256Hz -> Sound length
        if self.state.fs.is_length_timer_active() {
            self.ch1.tick_length_timer();
            self.ch2.tick_length_timer();
            self.ch3.tick_length_timer();
            self.ch4.tick_length_timer();
        }

        // 128Hz -> CH1 freq sweep
        if self.state.fs.is_freq_sweep_active() {
            self.ch1.tick_freq_sweep();
            self.ch2.tick_freq_sweep();
            self.ch3.tick_freq_sweep();
            self.ch4.tick_freq_sweep();
        }

        // 64Hz -> Envelope sweep
        if self.state.fs.is_volume_envelope_active() {
            self.ch1.tick_envelope_sweep();
            self.ch2.tick_envelope_sweep();
            self.ch3.tick_envelope_sweep();
            self.ch4.tick_envelope_sweep();
        }

        // increment after invoking components to get the next FS state.
        self.state.fs.increment();
    }


    /// Updates each channel with the time passed.
    fn update_channels(&mut self, cycles: Clock) {
        for _ in 0..cycles {
            let run_cycles = 1;

            {
                self.ch1.update(run_cycles);
                self.ch2.update(run_cycles);
                self.ch3.update(run_cycles);
                self.ch4.update(run_cycles);
            }

            {
                self.mixer.put(&self.ch1, &self.state);
                self.mixer.put(&self.ch2, &self.state);
                self.mixer.put(&self.ch3, &self.state);
                self.mixer.put(&self.ch4, &self.state);

                // mix all input values into left & right channels
                // according to their mixer settings
                let sample = self.mixer.mix();

                // push into samples buffer
                self.audio_output.push(sample, run_cycles);
            }
        }
    }


    /// The APU was powered on after being disabled before.
    fn power_on(&mut self) {
        self.state.fs.reset();
    }


    /// Reset the APUs internal data.
    fn reset(&mut self) {
        self.mixer.reset();
        self.ch1.reset(&self.state);
        self.ch2.reset(&self.state);
        self.ch3.reset(&self.state);
        self.ch4.reset(&self.state);
    }


    /// Check whether there is the DAC of at least one channel enabled.
    pub fn is_any_dac_enabled(&self) -> bool {
            self.ch1.get_dac().is_enabled()
        ||  self.ch2.get_dac().is_enabled()
        ||  self.ch3.get_dac().is_enabled()
        ||  self.ch4.get_dac().is_enabled()
    }


    /// Get the audio output object which allows the frontend to control the sound generation
    /// and receive the generated sample data.
    pub fn get_audio_output(&mut self) -> &mut AudioOutput {
        &mut self.audio_output
    }
}


impl MemoryBusConnection for Apu {
    fn on_read(&self, address: u16) -> u8 {
        match address {
            // Channel 1
            MEMORY_LOCATION_APU_NR10 ..= MEMORY_LOCATION_APU_NR14 => {
                let number = address - MEMORY_LOCATION_APU_NR10;
                self.ch1.on_read_register(number, &self.state)
            }

            // Channel 2
            MEMORY_LOCATION_APU_NR21 ..= MEMORY_LOCATION_APU_NR24 => {
                let number = address - MEMORY_LOCATION_APU_NR20;
                self.ch2.on_read_register(number, &self.state)
            }

            // Channel 3
            MEMORY_LOCATION_APU_NR30 ..= MEMORY_LOCATION_APU_NR34 => {
                let number = address - MEMORY_LOCATION_APU_NR30;
                self.ch3.on_read_register(number, &self.state)
            }

            // Channel 4
            MEMORY_LOCATION_APU_NR41 ..= MEMORY_LOCATION_APU_NR44 => {
                let number = address - MEMORY_LOCATION_APU_NR40;
                self.ch4.on_read_register(number, &self.state)
            }

            // Volume / VIN settings
            MEMORY_LOCATION_APU_NR50 => {
                self.mixer.read_nr50()
            },

            // Channel panning
            MEMORY_LOCATION_APU_NR51 => {
                self.mixer.read_nr51()
            },

            // APU powered state / channel active states
            MEMORY_LOCATION_APU_NR52 => {
                if self.state.apu_on {
                        NR52_NON_READABLE_BITS
                    |   as_bit_flag(self.state.apu_on, 7)
                    |   as_bit_flag(self.ch1.is_channel_enabled(), 0)
                    |   as_bit_flag(self.ch2.is_channel_enabled(), 1)
                    |   as_bit_flag(self.ch3.is_channel_enabled(), 2)
                    |   as_bit_flag(self.ch4.is_channel_enabled(), 3)
                }
                else {
                    NR52_NON_READABLE_BITS
                }
            },

            // Wave RAM
            MEMORY_LOCATION_APU_WAVE_RAM_BEGIN ..= MEMORY_LOCATION_APU_WAVE_RAM_END => {
                // Wave RAM will be forwarded into channel 3
                self.ch3.on_read_register(address, &self.state)
            }

            _ => 0xff
        }
    }


    fn on_write(&mut self, address: u16, value: u8) {
        match address {
            // Channel 1
            MEMORY_LOCATION_APU_NR10 ..= MEMORY_LOCATION_APU_NR14 => {
                let number = address - MEMORY_LOCATION_APU_NR10;
                self.ch1.on_write_register(number, value, &self.state);
            }

            // Channel 2
            MEMORY_LOCATION_APU_NR20 ..= MEMORY_LOCATION_APU_NR24 => {
                let number = address - MEMORY_LOCATION_APU_NR20;
                self.ch2.on_write_register(number, value, &self.state);
            }

            // Channel 3
            MEMORY_LOCATION_APU_NR30 ..= MEMORY_LOCATION_APU_NR34 => {
                let number = address - MEMORY_LOCATION_APU_NR30;
                self.ch3.on_write_register(number, value, &self.state);
            }

            // Channel 4
            MEMORY_LOCATION_APU_NR40 ..= MEMORY_LOCATION_APU_NR44 => {
                let number = address - MEMORY_LOCATION_APU_NR40;
                self.ch4.on_write_register(number, value, &self.state);
            }

            // Volume / VIN settings
            MEMORY_LOCATION_APU_NR50 => {
                if self.state.apu_on {
                    self.mixer.write_nr50(value);
                }
            },

            // Channel panning
            MEMORY_LOCATION_APU_NR51 => {
                if self.state.apu_on {
                    self.mixer.write_nr51(value);
                }
            },

            // Enable / Disable APU
            MEMORY_LOCATION_APU_NR52 => {
                let enabled = get_bit(value, 7);

                if self.state.apu_on != enabled {
                    self.state.apu_on = enabled;

                    if enabled {
                        self.power_on();
                    }
                    else {
                        self.reset();
                    }
                }
            },

            // Wave RAM
            MEMORY_LOCATION_APU_WAVE_RAM_BEGIN ..= MEMORY_LOCATION_APU_WAVE_RAM_END => {
                // Wave RAM will be forwarded into channel 3
                self.ch3.on_write_register(address, value, &self.state);
            },

            _ => { }
        };
    }
}
