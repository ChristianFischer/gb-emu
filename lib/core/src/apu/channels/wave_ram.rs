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

#[cfg(feature = "std")]
use std::fmt::{Display, Formatter};

use core::ops::{Index, IndexMut};


/// The cursor to store the position within the Wave RAM to be read by the wave channel.
/// Since each byte of the Wave RAM contains two values, the range of the cursor would be
/// from 0 to 31.
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WaveRamPositionCursor {
    position: u8,
}


/// The Wave RAM storing a 16 byte array containing the wave data to be played by channel 3.
#[derive(Default, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WaveRam {
    /// The wave RAM's actual data.
    data: [u8; 16],
}


/// An object storing a cursor, where a sample will be read from the Wave RAM.
///
impl WaveRamPositionCursor {
    /// Reset the cursor position to zero.
    pub fn reset(&mut self) {
        self.position = 0;
    }


    /// Advance the cursor to it's next position.
    pub fn advance(&mut self) {
        // increment the position value and keep it in the 0..32 range
        self.position = self.position.wrapping_add(1) & 0x1f;
    }


    /// Get the index where to read the wave RAM.
    pub fn get_index(&self) -> u8 {
        (self.position >> 1) & 0x0f
    }


    /// Get whether to read the high or low nibble of the byte read from the Wave RAM.
    pub fn get_high_or_low(&self) -> bool {
        (self.position & 0x01) == 0
    }
}


impl WaveRam {
    /// Read a sample from the Wave RAM on the position of the Wave RAM position cursor.
    pub fn get_sample(&self, position: &WaveRamPositionCursor) -> u8 {
        // since wave ram contains two samples per byte,
        // the index within the wave ram is wave_step / 2.
        let index = position.get_index();

        // read the byte from wave RAM
        let value = self.data[index as usize];

        // depending on bit 1, either take the high or low nibble
        let amp = match position.get_high_or_low() {
            true  => (value >> 4) & 0x0f,
            false => (value >> 0) & 0x0f,
        };

        amp
    }


    /// Performs the Wave RAM corruption when triggering the wave channel during a read operation
    /// by the sound generator.
    pub fn do_wave_ram_corruption(&mut self, position: &WaveRamPositionCursor) {
        let index = position.get_index() as usize;

        if index < 4 {
            // when one of the first four bytes was read, it's value
            // will be copied into the first byte of wave memory
            self.data[0] = self.data[index];
        }
        else {
            // above the first four bytes, a set of four bytes will be
            // copied into the first four bytes of wave memory

            // get the 4-byte-aligned start address where to copy from
            let address_from = index & !0b_0000_0011;

            // copy the 4 byte range
            for i in 0..4 {
                self.data[i] = self.data[address_from + i];
            }
        }
    }
}


impl Index<u8> for WaveRam {
    type Output = u8;

    fn index(&self, index: u8) -> &Self::Output {
        &self.data[(index & 0x0f) as usize]
    }
}


impl IndexMut<u8> for WaveRam {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.data[(index & 0x0f) as usize]
    }
}


#[cfg(feature = "std")]
impl Display for WaveRam {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.data
            .iter()
            .fold(
                Ok(()),
                |result, item| result.and_then(
                    |_| write!(f, " {:02x}", item)
                )
            )
    }
}

