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

use alloc::vec::Vec;

use crate::gameboy::{DeviceConfig, EmulationType};
use crate::mmu::memory_data::mapped::MemoryDataMapped;
use crate::mmu::memory_data::{MemoryData, MemoryDataFixedSize};
use crate::ppu::graphic_data::{DmgPalette, GbcPaletteData, Sprite};
use crate::utils::{get_bit, SerializableArray};

pub type OamRam         = SerializableArray<Sprite, 40>;
pub type GbcPaletteRam  = SerializableArray<GbcPaletteData, 8>;

pub type VRamBank       = MemoryDataFixedSize<8192>;
pub type WRamBank       = MemoryDataFixedSize<4096>;
pub type OamRamBank     = MemoryDataMapped<OamRam>;
pub type GbcPaletteBank = MemoryDataMapped<GbcPaletteRam>;


/// Utility to read and write data into GameBoy Color palettes
/// via BCPS/BCPD and OCPS/OCPD registers.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GbcPalettePointer {
    /// The pointer where to access the palette buffer on the next read/write operation.
    pointer: u8,

    /// Whether to auto-increment the pointer on the next write operation.
    auto_increment: bool,
}


/// An helper object to hold all video memory structures used by the PPU.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VideoMemory {
    /// Video RAM (DMG = 1 * 8kiB, GBC = 2 * 8kiB)
    pub vram_banks: Vec<VRamBank>,

    /// Active Video RAM Bank (0-1, CGB only)
    pub vram_active_bank: u8,

    /// OAM memory: 40 sprites, 4 bytes each = 160B
    pub oam: OamRamBank,


    pub palettes: Palettes,
}


/// Stores all palettes used in either GBC or DMG mode.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Palettes {
    /// DMG background palette
    pub bgp: DmgPalette,

    /// DMG object palettes
    pub obp: [DmgPalette; 2],

    /// GameBoy Color only: storage for background palettes
    pub gbc_background_palette: GbcPaletteBank,

    /// GameBoy Color only: storage for object palettes
    pub gbc_object_palette: GbcPaletteBank,

    /// GameBoy Color only: the pointer where to read/write background palette data
    pub gbc_background_palette_pointer: GbcPalettePointer,

    /// GameBoy Color only: the pointer where to read/write object palette data
    pub gbc_object_palette_pointer: GbcPalettePointer,
}


impl GbcPalettePointer {
    /// Creates a new pointer with a null-value.
    pub fn new() -> Self {
        Self {
            pointer: 0x00,
            auto_increment: false,
        }
    }


    /// Set the pointer when writing the BCPS/OCPS register.
    pub fn set(&mut self, value: u8) {
        self.pointer        = value & 0x3f;
        self.auto_increment = get_bit(value, 7);
    }


    /// Get the pointer value
    pub fn get(&self) -> u8 {
        self.pointer | ((self.auto_increment as u8) << 7)
    }


    /// Writes into a palette memory on the pointers current address.
    /// Performs the auto increment when enabled.
    pub fn write(&mut self, palette: &mut GbcPaletteBank, value: u8) {
        palette.set_at(self.pointer as usize, value);

        if self.auto_increment {
            self.pointer = self.pointer.wrapping_add(1) & 0x3f;
        }
    }


    /// Reads data from a palette on the pointers current address.
    pub fn read(&self, palette: &GbcPaletteBank) -> u8 {
        palette.get_at(self.pointer as usize)
    }
}


impl VideoMemory {
    pub fn new(device_config: DeviceConfig) -> Self {
        let num_vram_banks = match device_config.emulation {
            EmulationType::DMG => 1,
            EmulationType::GBC => 2,
        };

        Self {
            vram_banks: core::iter::repeat_with(|| VRamBank::new()).take(num_vram_banks).collect(),
            vram_active_bank: 0,

            oam: OamRamBank::new([Sprite::empty(); 40]),

            palettes: Palettes::new(),
        }
    }
}


impl Palettes {
    pub fn new() -> Self {
        Self {
            bgp: DmgPalette::create_default(),
            obp: [DmgPalette::create_default(); 2],

            gbc_background_palette: GbcPaletteBank::new([GbcPaletteData::new(); 8]),
            gbc_object_palette: GbcPaletteBank::new([GbcPaletteData::new(); 8]),

            gbc_background_palette_pointer: GbcPalettePointer::new(),
            gbc_object_palette_pointer: GbcPalettePointer::new(),
        }
    }
}
