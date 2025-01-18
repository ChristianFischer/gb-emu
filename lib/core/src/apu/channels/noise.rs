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
use crate::emulator_core::Clock;
use crate::utils::{as_bit_flag, get_bit};


const NR41_NON_READABLE_BITS : u8       = 0b_1100_0000;
const NR44_NON_READABLE_BITS : u8       = 0b_0011_1111;
const NR44_WRITE_ONLY_TRIGGER_BIT : u8  = 0b_1000_0000;


/// A sound generator to generate random noise using a Linear Feedback Shift Register (LFSR)
/// This will generate a random sequence of 0 and 1, modified with the volume from
/// the volume envelope function.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NoiseGenerator {
    /// A Linear Feedback Shift Register used to generate a random sequence of 0 an 1.
    lfsr: u16,

    /// Width of the LFSR in bits.
    lfsr_width: u16,

    /// The divider code read from the NR43 register.
    divider_code: u8,

    /// The base value of the frequencies divider value.
    frequency_divider: Clock,

    /// The number of bits to shift the base value left to get the actual divider.
    frequency_shift: Clock,

    /// The time left until the next LFSR iteration.
    frequency_timer: Clock,
}


impl NoiseGenerator {
    pub fn new() -> Self {
        Self {
            lfsr:               0,
            lfsr_width:         15,
            divider_code:       0,
            frequency_divider:  8,
            frequency_shift:    0,
            frequency_timer:    0,
        }
    }


    /// Compute the time until next LFSR iteration.
    pub fn reset_timer(&mut self) {
        // frequency is 4194304 / (divider << shift)
        // so we compute the number of cycles until next value change
        self.frequency_timer = self.frequency_divider << self.frequency_shift
    }
}


impl ChannelComponent for NoiseGenerator {
    fn on_read_register(&self, number: u16, apu_state: &ApuState) -> u8 {
        match number {
            1 => NR41_NON_READABLE_BITS, // unused bits

            3 => {
                let lfsr_is_short = match self.lfsr_width {
                    7 => true,
                    _ => false,
                };

                    self.divider_code
                |   as_bit_flag(lfsr_is_short, 3)
                |   (((self.frequency_shift & 0x0f) as u8) << 4)
            },

            4 => NR44_NON_READABLE_BITS | NR44_WRITE_ONLY_TRIGGER_BIT,

            _ => default_on_read_register(number, apu_state)
        }
    }


    fn on_write_register(&mut self, number: u16, value: u8, apu_state: &ApuState) -> TriggerAction {
        match number {
            3 => {
                let shift        = (value >> 4) & 0x0f;
                let divider_code = (value >> 0) & 0x07;

                self.divider_code      = divider_code;
                self.frequency_shift   = shift as Clock;
                self.frequency_divider = match divider_code {
                    0 => 8,
                    _ => (divider_code as Clock) << 4,
                };

                // bit 3 determines the length of the LFSR, either 7 or 15 bits.
                let lfsr_is_short = get_bit(value, 3);
                self.lfsr_width = match lfsr_is_short {
                    false => 15,
                    true  => 7,
                };
            }

            _ => { }
        }

        default_on_write_register(number, value, apu_state)
    }


    fn on_trigger_event(&mut self, apu_state: &ApuState) -> TriggerAction {
        self.reset_timer();

        // reset lfsr to zero
        self.lfsr = 0;

        default_on_trigger_event(apu_state)
    }


    fn on_reset(&mut self, _apu_state: &ApuState) {
        *self = Self::new();
    }
}


impl SoundGenerator for NoiseGenerator {
    fn create() -> Self {
        Self::new()
    }


    fn get_frequency(&self) -> Frequency {
        Frequency::default()
    }


    fn set_frequency(&mut self, frequency: Frequency) {
        _ = frequency;
    }


    fn update(&mut self, cycles: Clock) {
        let mut remaining_cycles = cycles;

        while remaining_cycles > 0 {
            let run_cycles = min(self.frequency_timer, remaining_cycles);

            self.frequency_timer = self.frequency_timer.saturating_sub(run_cycles);

            // when the timer expires
            if self.frequency_timer == 0 {
                // timer will be restarted
                self.reset_timer();

                // determine the new bit to insert by XOR of bit 0 and 1
                let insert_bit = (self.lfsr ^ (self.lfsr >> 1) ^ 1) & 0x01;

                // the XOR bit will be inserted at the position left of the lfsr width;
                // the following shift will move the new bit in it's desired position
                self.lfsr |= insert_bit << self.lfsr_width;

                // the whole register gets shifted to the right
                self.lfsr >>= 1;
            }

            remaining_cycles = remaining_cycles.saturating_sub(run_cycles);
        }
    }


    fn get_sample(&self, _apu_state: &ApuState) -> u8 {
        // take bit 0 to determine whether a tone is generated or not
        let sample = (self.lfsr & 0x01) as u8;
        sample
    }
}
