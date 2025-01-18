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
use crate::apu::channels::channel::{default_on_read_register, default_on_write_register, ChannelComponent, TriggerAction};
use crate::apu::channels::frequency::Frequency;
use crate::apu::channels::generator::SoundGenerator;
use crate::apu::channels::wave_duty::WaveDuty;
use crate::emulator_core::Clock;


const NRX3_WRITE_ONLY_FREQUENCY : u8    = 0b_1111_1111;
const NRX4_NON_READABLE_BITS : u8       = 0b_0011_1000;
const NRX4_WRITE_ONLY_FREQUENCY : u8    = 0b_0000_0111;
const NRX4_WRITE_ONLY_TRIGGER_BIT : u8  = 0b_1000_0000;


/// The value to multiply the frequency with to get the number of CPU cycles for the wave timer.
const FREQUENCY_CYCLES : Clock = 4;


/// A sound generator to generate a pulse wave. The wave is based is based on a wave duty value
/// and a volume computed by an envelope function.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PulseGenerator {
    /// The base of the sound wave to be played, represented by a square wave.
    wave_duty: WaveDuty,

    /// The frequency is controlling how fast the wave will be played.
    /// The value will be read from NRx3 and NRx4 register of the channel.
    frequency: Frequency,

    /// Current value of the wave timer. The value is built based on the frequency
    /// configured by the application. It will be decreased with each CPU T-Cycle
    /// and restarts when reaching zero.
    wave_timer: Clock,

    /// The current position where to read the value from the wave duty.
    wave_duty_step: u8,
}


impl PulseGenerator {
    pub fn new() -> Self {
        Self {
            wave_duty:          WaveDuty::default(),
            frequency:          Frequency::default(),
            wave_timer:         0,
            wave_duty_step:     0,
        }
    }
}


impl ChannelComponent for PulseGenerator {
    fn on_read_register(&self, number: u16, apu_state: &ApuState) -> u8 {
        match number {
            1 => self.wave_duty.get_index() << 6,
            3 => NRX3_WRITE_ONLY_FREQUENCY,
            4 => NRX4_WRITE_ONLY_FREQUENCY | NRX4_NON_READABLE_BITS | NRX4_WRITE_ONLY_TRIGGER_BIT,
            _ => default_on_read_register(number, apu_state)
        }
    }


    fn on_write_register(&mut self, number: u16, value: u8, apu_state: &ApuState) -> TriggerAction {
        match number {
            1 => {
                let wave_duty_index = (value >> 6) & 0x03;
                self.wave_duty = WaveDuty::by_index(wave_duty_index);
            }

            3 | 4 => {
                self.frequency.set_by_register(number, value);
            }

            _ => { }
        }

        default_on_write_register(number, value, apu_state)
    }


    fn on_reset(&mut self, _apu_state: &ApuState) {
        *self = Self::new();
    }
}


impl SoundGenerator for PulseGenerator {
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

            // when the wave timer expires, it will be restarted and the
            // position inside the wave duty proceeds
            if self.wave_timer == 0 {
                self.wave_timer     = self.frequency.to_countdown(FREQUENCY_CYCLES);
                self.wave_duty_step = self.wave_duty_step.wrapping_add(1);
            }

            remaining_cycles = remaining_cycles.saturating_sub(run_cycles);
        }
    }


    fn get_sample(&self, _apu_state: &ApuState) -> u8 {
        let wave = self.wave_duty.get_wave_at(self.wave_duty_step);
        wave
    }
}
