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
use crate::gameboy::DeviceConfig;
use crate::mmu::memory_bus::MemoryBusConnection;


/// This is a placeholder implementation of the APU, which has no effect at all.
/// The [DummyApu] will be used as a replacement, when the APU feature is turned off.
/// This allows the rest of the code being agnostic of the actual APU implementation. 
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DummyApu {
}


impl DummyApu {
    /// Creates a new APU object.
    pub fn new(_device_config: DeviceConfig) -> Self {
        Self {
        }
    }
}



impl MemoryBusConnection for DummyApu {
    fn on_read(&self, address: u16) -> u8 {
        _ = address;
        0xff
    }


    fn on_write(&mut self, address: u16, value: u8) {
        _ = (address, value);
    }
}
