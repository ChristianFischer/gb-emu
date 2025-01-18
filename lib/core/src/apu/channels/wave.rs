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

use core::cmp::min;

use crate::apu::apu::ApuState;
use crate::apu::channels::channel::{default_on_read_register, default_on_trigger_event, default_on_write_register, ChannelComponent, TriggerAction};
use crate::apu::channels::frequency::Frequency;
use crate::apu::channels::generator::SoundGenerator;
use crate::apu::channels::wave_ram::{WaveRam, WaveRamPositionCursor};
use crate::emulator_core::Clock;
use crate::mmu::locations::*;
use crate::utils::{as_bit_flag, get_bit};


const NR30_NON_READABLE_BITS : u8       = 0b_0111_1111;
const NR32_NON_READABLE_BITS : u8       = 0b_1001_1111;
const NR33_WRITE_ONLY_FREQUENCY : u8    = 0b_1111_1111;
const NR34_NON_READABLE_BITS : u8       = 0b_0011_1000;
const NR34_WRITE_ONLY_FREQUENCY : u8    = 0b_0000_0111;
const NR34_WRITE_ONLY_TRIGGER_BIT : u8  = 0b_1000_0000;


/// The time, the Wave RAM is accessible during a read operation
/// when the sound generator is active.
const WAVE_RAM_ACCESS_TIME: Clock = 2;

/// The value to multiply the frequency with to get the number of CPU cycles for the wave timer.
const FREQUENCY_CYCLES : Clock = 2;


/// A sound generator reading wave data from a dedicated memory location called Wave RAM.
/// Each time the frequency timer expires, a sample is read from the Wave RAM and the reading
/// cursor get incremented.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WaveGenerator {
    /// Stores whether the DAC of this channel is enabled or not.
    dac_enabled: bool,

    /// Flag to store whether the channel is currently enabled or not.
    channel_enabled: bool,

    /// The output level value read from the NR32 register.
    output_level: u8,

    /// The number of bits to shift the value of the sound sample to lower it's volume.
    volume_shift: u8,

    /// The frequency is controlling how fast the wave will be played.
    /// The value will be read from NRx3 and NRx4 register of the channel.
    frequency: Frequency,

    /// Current value of the wave timer. The value is built based on the frequency
    /// configured by the application. It will be decreased with each CPU T-Cycle
    /// and restarts when reaching zero.
    wave_timer: Clock,

    /// The current position where to read the value from the wave duty.
    wave_ram_position: WaveRamPositionCursor,

    /// The last byte accessed by reading a sample from the Wave RAM.
    /// While the channel is enabled, read/write operations from the memory bus will be
    /// restricted to this address.
    wave_ram_last_address: u8,

    /// The time window when the Wave RAM is accessible after a read operation on DMG.
    /// When the value becomes zero, the time window is closed and Wave RAM gets inaccessible.
    wave_ram_access_timeout: Clock,

    /// The current sample read from Wave RAM.
    wave_ram_current_sample: u8,

    /// Wave pattern RAM.
    wave_ram: WaveRam,

}


impl WaveGenerator {
    pub fn new() -> Self {
        Self {
            dac_enabled:                false,
            channel_enabled:            false,
            output_level:               0,
            volume_shift:               0,
            frequency:                  Frequency::default(),
            wave_timer:                 0,
            wave_ram_position:          WaveRamPositionCursor::default(),
            wave_ram_last_address:      0,
            wave_ram_access_timeout:    0,
            wave_ram_current_sample:    0,
            wave_ram:                   WaveRam::default(),
        }
    }


    /// A read or write operation on Wave RAM may not access the requested index, depending on
    /// the current state of the wave channel and the device we're running.
    /// This function maps the requested byte into the address which will actually be accessed
    /// or `None` if the wave ram is currently not accessible.
    pub fn get_wave_ram_access(&self, requested_address: u16, apu_state: &ApuState) -> Option<u8> {
        if !self.channel_enabled {
            // Wave RAM is completely accessible, when channel is disabled
            let index = (requested_address - MEMORY_LOCATION_APU_WAVE_RAM_BEGIN) & 0x0f;
            Some(index as u8)
        }
        else if apu_state.device_config.is_gbc_enabled() || (self.wave_ram_access_timeout > 0) {
            // with the channel enabled, the access is restricted to the last
            // address being read by the sound generator.
            // On DMG this is only possible within 2 cycles after the data was read,
            // otherwise reading or writing the wave ram will fail.
            Some(self.wave_ram_last_address & 0x0f)
        }
        else {
            // on DMG, after the time window is closed,
            // access on wave ram is no longer possible
            None
        }
    }


    /// Reads data from the Wave RAM given the requested memory address.
    /// This takes into account when the access is restricted when the channel is enabled.
    pub fn read_wave_ram(&self, requested_address: u16, apu_state: &ApuState) -> u8 {
        if let Some(address) = self.get_wave_ram_access(requested_address, apu_state) {
            self.wave_ram[address]
        }
        else {
            0xff
        }
    }


    /// Writes data to the Wave RAM given the requested memory address.
    /// This takes into account when the access is restricted when the channel is enabled.
    pub fn write_wave_ram(&mut self, requested_address: u16, value: u8, apu_state: &ApuState) {
        if let Some(address) = self.get_wave_ram_access(requested_address, apu_state) {
            self.wave_ram[address] = value;
        }
    }
}


impl ChannelComponent for WaveGenerator {
    fn can_write_register(&self, number: u16, apu_state: &ApuState) -> bool {
        match number {
            // Wave RAM is always writable
            MEMORY_LOCATION_APU_WAVE_RAM_BEGIN ..= MEMORY_LOCATION_APU_WAVE_RAM_END => true,
            _ => apu_state.apu_on,
        }
    }


    fn on_read_register(&self, number: u16, apu_state: &ApuState) -> u8 {
        match number {
            0 => NR30_NON_READABLE_BITS | as_bit_flag(self.dac_enabled, 7),
            2 => NR32_NON_READABLE_BITS | (self.output_level << 5),
            3 => NR33_WRITE_ONLY_FREQUENCY,
            4 => NR34_WRITE_ONLY_FREQUENCY | NR34_NON_READABLE_BITS | NR34_WRITE_ONLY_TRIGGER_BIT,

            // Wave RAM
            MEMORY_LOCATION_APU_WAVE_RAM_BEGIN ..= MEMORY_LOCATION_APU_WAVE_RAM_END => {
                self.read_wave_ram(number, apu_state)
            }

            _ => default_on_read_register(number, apu_state)
        }
    }


    fn on_write_register(&mut self, number: u16, value: u8, apu_state: &ApuState) -> TriggerAction {
        match number {
            0 => {
                self.dac_enabled = get_bit(value, 7);
                return if self.dac_enabled {
                    TriggerAction::EnableDac
                }
                else {
                    TriggerAction::DisableDac
                };
            }

            2 => {
                self.output_level = (value >> 5) & 0x03;
                self.volume_shift = match self.output_level {
                    0b00 => 4, // mute
                    0b01 => 0, // 100% (no change)
                    0b10 => 1, //  50%
                    0b11 => 2, //  25%
                    _ => unreachable!()
                };
            }

            3 | 4 => {
                self.frequency.set_by_register(number, value);
            }

            // Wave RAM
            MEMORY_LOCATION_APU_WAVE_RAM_BEGIN ..= MEMORY_LOCATION_APU_WAVE_RAM_END => {
                self.write_wave_ram(number, value, apu_state);
            }

            _ => { }
        }

        default_on_write_register(number, value, apu_state)
    }


    fn on_trigger_event(&mut self, apu_state: &ApuState) -> TriggerAction {
        if
                self.channel_enabled // was already enabled
            &&  self.wave_timer == 2 // wave ram is about to be read when the timer expires
            &&  !apu_state.device_config.is_gbc_enabled()
        {
            self.wave_ram.do_wave_ram_corruption(&self.wave_ram_position);
        }

        self.wave_ram_position.reset();

        self.channel_enabled        = self.is_dac_enabled();
        self.wave_ram_last_address  = 0;
        self.wave_timer             = 6; // required to pass Blargg's test 09-wave-read-while-on

        default_on_trigger_event(apu_state)
    }


    fn on_channel_disabled(&mut self) {
        self.channel_enabled = false;
    }


    fn on_reset(&mut self, _apu_state: &ApuState) {
        // reset everything except wave ram
        *self = Self {
            wave_ram: self.wave_ram,
            .. Self::new()
        }
    }
}


impl SoundGenerator for WaveGenerator {
    fn create() -> Self {
        Self::new()
    }


    fn get_frequency(&self) -> Frequency {
        self.frequency
    }


    fn set_frequency(&mut self, frequency: Frequency) {
        self.frequency = frequency;
    }


    fn update(&mut self, cycles: Clock) {
        let mut remaining_cycles = cycles;

        while remaining_cycles > 0 {
            let run_cycles = min(self.wave_timer, remaining_cycles);

            self.wave_timer  = self.wave_timer.saturating_sub(run_cycles);

            // reduce the time the wave ram cursor is active
            self.wave_ram_access_timeout = self.wave_ram_access_timeout.saturating_sub(run_cycles);

            // when the wave timer expires, it will be restarted and the
            // position inside the wave ram proceeds
            if self.wave_timer == 0 {
                self.wave_timer              = self.frequency.to_countdown(FREQUENCY_CYCLES);
                self.wave_ram_current_sample = self.wave_ram.get_sample(&self.wave_ram_position);
                self.wave_ram_last_address   = self.wave_ram_position.get_index();
                self.wave_ram_access_timeout = WAVE_RAM_ACCESS_TIME;

                self.wave_ram_position.advance();
            }

            remaining_cycles = remaining_cycles.saturating_sub(run_cycles);
        }
    }


    fn is_dac_enabled(&self) -> bool {
        self.dac_enabled
    }


    fn get_sample(&self, _apu_state: &ApuState) -> u8 {
        // get the sample amplitude at the current wave ram position
        let amp = self.wave_ram_current_sample;

        // apply the volume shift to the amplitude to get the final sample
        let sample = amp >> self.volume_shift;

        sample
    }
}
