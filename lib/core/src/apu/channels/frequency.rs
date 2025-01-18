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

#[cfg(feature = "std")]
use std::fmt::{Display, Formatter};

use crate::emulator_core::Clock;
use crate::utils::{to_u16, to_u8};


/// Represents a channels frequency as written via NRx3 and NRx4 registers.
#[derive(Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Frequency {
    value: u16,
}


impl Frequency {
    /// The maximum possible frequency, which can be stored in 11 bits being used in NRx3 and NRx4.
    /// Used to check for a possible overflow.
    pub const MAX_FREQUENCY : u16 = 0b_0000_0111_1111_1111;


    /// Creates a new frequency object with an existing value.
    /// This function checks whether the input value is greater than the maximum frequency. If so,
    /// the result will be `None`, otherwise it will return a valid Frequency object.
    pub fn new(value: u16) -> Option<Self> {
        if value <= Self::MAX_FREQUENCY {
            Some(
                Self {
                    value
                }
            )
        }
        else {
            None
        }
    }


    /// Get the frequencies value.
    pub fn get_value(&self) -> u16 {
        self.value
    }


    /// Get the high byte of the frequencies value.
    pub fn get_high(&self) -> u8 {
        to_u8(self.value).0
    }


    /// Get the low byte of the frequencies value.
    pub fn get_low(&self) -> u8 {
        to_u8(self.value).1
    }


    /// Set the high byte of the frequency.
    pub fn set_high(&mut self, high: u8) {
        let (_, low) = to_u8(self.value);
        self.value = to_u16(high, low);
    }


    /// Set the low byte of the frequency.
    pub fn set_low(&mut self, low: u8) {
        let (high, _) = to_u8(self.value);
        self.value = to_u16(high, low);
    }


    /// Change the frequencies value based on the value of a NRx3 or NRx4 register.
    pub fn set_by_register(&mut self, number: u16, value: u8) {
        match number {
            3 => self.set_low(value),
            4 => self.set_high(value & 0x07),
            _ => { }
        }
    }


    /// Reset the frequency to zero.
    pub fn reset(&mut self) {
        self.value = 0;
    }


    /// Get the countdown value used to measure the time to the next wave iteration.
    pub fn to_countdown(&self, cycles: Clock) -> Clock {
        ((2048 - self.value) as Clock) * cycles
    }
}


impl Default for Frequency {
    fn default() -> Self {
        Self {
            value: 0
        }
    }
}


#[cfg(feature = "std")]
impl Display for Frequency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04x}", self.value)
    }
}
