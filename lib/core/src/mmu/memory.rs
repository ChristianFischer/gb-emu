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
use core::cmp::max;

#[cfg(feature = "file_io")]
use std::io;

use crate::boot_rom::BootRom;
use crate::cartridge::Cartridge;
use crate::gameboy::{DeviceConfig, EmulationType};
use crate::mmu::locations::*;
use crate::mmu::mbc::{create_mbc, Mbc, MbcImpl, MemoryBankController};
use crate::mmu::memory_bus::{memory_map, MemoryBusConnection};
use crate::mmu::memory_data::{MemoryData, MemoryDataFixedSize};


/// Stores the information of an active OAM DMA transfer
/// The DMA transfer copies data from the given address
/// into OAM memory.
/// In total, 160 bytes will be transferred, so it takes
/// 160 cycles to transfer for the transfer to be completed.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DmaTransferInfo {
    /// The address where to start copying the memory from.
    pub start_address: u16,

    /// The next byte to be copied.
    pub next_byte: u16,
}


/// State of the OAM DMA transfer, whether it be disabled or
/// in progress, including the time remaining.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DmaTransferState {
    /// No transfer is active.
    Disabled,

    /// A transfer is currently in progress.
    /// The attached struct stores information where
    /// the data should be taken from and how much data
    /// was already transferred.
    Transferring(DmaTransferInfo),
}


pub type WRamBank = MemoryDataFixedSize<4096>;
pub type HRamBank = MemoryDataFixedSize<127>;


/// The memory object is the owner of the emulator's memory.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Memory {
    /// The configuration of the running device
    device_config: DeviceConfig,

    /// Work RAM banks (DMG = 2 * 4kiB, GBC = 8 * 4kiB)
    wram_banks: Vec<WRamBank>,

    /// Active Work RAM banks.
    /// Bank 0 is fixed, Bank 1 can be switched between 1-7 on GBC.
    wram_active_bank_0: u8,
    wram_active_bank_1: u8,

    /// High RAM
    hram: HRamBank,

    /// MemoryBankController implementation.
    mbc: Mbc,

    boot_rom:   Option<BootRom>,
    cartridge:  Option<Cartridge>,
}


impl Memory {
    /// Create a new Memory object.
    pub fn new(device_config: DeviceConfig) -> Self {
        let num_wram_banks = match device_config.emulation {
            EmulationType::DMG => 2,
            EmulationType::GBC => 8,
        };

        Self {
            device_config,

            wram_banks: core::iter::repeat_with(|| WRamBank::new()).take(num_wram_banks).collect(),
            wram_active_bank_0: 0,
            wram_active_bank_1: 1,

            hram: HRamBank::new(),

            mbc: create_mbc(&MemoryBankController::None),

            boot_rom:   None,
            cartridge:  None,
        }
    }


    /// Checks whether a boot rom is active or not.
    pub fn has_boot_rom(&self) -> bool {
        match self.boot_rom {
            None    => false,
            Some(_) => true,
        }
    }

    /// Load a boot ROM into the memory.
    pub fn set_boot_rom(&mut self, boot_rom: BootRom) {
        self.boot_rom = Some(boot_rom)
    }

    /// Load ROM data from a cartridge into the memory.
    pub fn set_cartridge(&mut self, cartridge: Cartridge) {
        self.mbc       = create_mbc(cartridge.get_mbc());
        self.cartridge = Some(cartridge);
    }

    /// Get a reference to the currently assigned cartridge, if any.
    pub fn get_cartridge(&self) -> Option<&Cartridge> {
        self.cartridge.as_ref()
    }

    /// Save the cartridge RAM, if any.
    #[cfg(feature = "file_io")]
    pub fn save_cartridge_ram_to_file_if_any(&self) -> io::Result<()> {
        if let Some(cartridge) = &self.cartridge {
            cartridge.save_ram_to_file_if_any()?;
        }

        Ok(())
    }
}


impl Memory {
    /// Reads data from the boot rom, if any, otherwise from the cartridge.
    fn read_boot_rom_or_cartridge(&self, address: u16) -> u8 {
        if let Some(boot_rom) = &self.boot_rom {
            return boot_rom.read(address);
        }

        self.read_from_cartridge(address)
    }


    /// Reads data from the cartridge.
    fn read_from_cartridge(&self, address: u16) -> u8 {
        if let Some(cartridge) = &self.cartridge {
            return self.mbc.read_byte(cartridge, address);
        }

        0xff
    }


    /// Writes data to the cartridge.
    fn write_to_cartridge(&mut self, address: u16, value: u8) {
        if let Some(cartridge) = &mut self.cartridge {
            self.mbc.write_byte(cartridge, address, value);
        }
    }
}


impl MemoryBusConnection for Memory {
    fn on_read(&self, address: u16) -> u8 {
        memory_map!(
            address => {
                0x0000 ..= 0x00ff => [] self.read_boot_rom_or_cartridge(address),
                0x0100 ..= 0x7fff => [] self.read_from_cartridge(address),
                0xa000 ..= 0xbfff => [] self.read_from_cartridge(address),

                0xc000 ..= 0xcfff => [mapped_address] {
                    let bank = &self.wram_banks[self.wram_active_bank_0 as usize];
                    bank.get_at(mapped_address)
                },

                0xd000 ..= 0xdfff => [mapped_address] {
                    let bank = &self.wram_banks[self.wram_active_bank_1 as usize];
                    bank.get_at(mapped_address)
                },

                0xe000 ..= 0xfdff => [mapped_address] {
                    // echo RAM; mapped into WRAM (0xc000 - 0xddff)
                    self.on_read((mapped_address + 0xc000) as u16)
                },

                0xfea0 ..= 0xfeff => [] {
                    // unusable ram area
                    0xff
                },

                0xff80 ..= 0xfffe => [mapped_address] {
                    self.hram.get_at(mapped_address)
                },

                // io registers
                0xff00 ..= 0xff7f => [] {
                    match address {
                        MEMORY_LOCATION_BOOT_ROM_DISABLE => {
                            match self.boot_rom {
                                Some(_) => 0x00,
                                None    => 0xff,
                            }
                        },

                        MEMORY_LOCATION_SVBK => {
                            // on GBC: WRAM bank #1
                            if let EmulationType::GBC = self.device_config.emulation {
                                self.wram_active_bank_1 | 0b_1111_1000
                            }
                            else {
                                0xff
                            }
                        },

                        _ => 0xff
                    }
                }
            }
        )
    }


    fn on_write(&mut self, address: u16, value: u8) {
        memory_map!(
            address => {
                0x0000 ..= 0x7fff => [] self.write_to_cartridge(address, value),
                0xa000 ..= 0xbfff => [] self.write_to_cartridge(address, value),

                0xc000 ..= 0xcfff => [mapped_address] {
                    let bank = &mut self.wram_banks[self.wram_active_bank_0 as usize];
                    bank.set_at(mapped_address, value)
                },

                0xd000 ..= 0xdfff => [mapped_address] {
                    let bank = &mut self.wram_banks[self.wram_active_bank_1 as usize];
                    bank.set_at(mapped_address, value)
                },

                0xe000 ..= 0xfdff => [mapped_address] {
                    // echo RAM; mapped into WRAM (0xc000 - 0xddff)
                    self.on_write((mapped_address + 0xc000) as u16, value)
                },

                0xfea0 ..= 0xfeff => [] { /* unusable ram area */ },

                0xff80 ..= 0xfffe => [mapped_address] {
                    self.hram.set_at(mapped_address, value)
                },

                // io registers
                0xff00 ..= 0xff7f => [] {
                    match address {
                        MEMORY_LOCATION_BOOT_ROM_DISABLE => {
                            if (value & 0x01) != 0 {
                                self.boot_rom = None;
                            }
                        },

                        MEMORY_LOCATION_SVBK => {
                            // on GBC: switch WRAM bank #1
                            if let EmulationType::GBC = self.device_config.emulation {
                                let bank = value & 0x07;
                                self.wram_active_bank_1 = max(1, bank);
                            }
                        },

                        _ => { }
                    }
                }
            }
        )
    }
}
