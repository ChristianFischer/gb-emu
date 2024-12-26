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

use crate::cartridge::Cartridge;
use crate::mmu::mbc::mbc1::Mbc1;
use crate::mmu::mbc::mbc2::Mbc2;
use crate::mmu::mbc::mbc5::Mbc5;
use crate::mmu::mbc::mbc_none::MbcNone;


/// Type of memory bank controller to be used
#[derive(Copy, Clone)]
pub enum MemoryBankController {
    None,
    MBC1,
    MBC1M,
    MBC2,
    MBC3,
    MBC5,
    MBC6,
    MBC7,
}


/// The actual implementation of [MemoryBankImplementation]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Mbc {
    None(MbcNone),
    MBC1(Mbc1),
    MBC1M(Mbc1),
    MBC2(Mbc2),
    MBC3(MbcNone),
    MBC5(Mbc5),
    MBC6(MbcNone),
    MBC7(MbcNone),
}


/// Trait for objects acting as memory bank controller.
pub trait MbcImpl {
    /// Read a single byte from the device memory.
    fn read_byte(&self, cartridge: &Cartridge, address: u16) -> u8;

    /// Write a single byte into the device memory.
    fn write_byte(&mut self, cartridge: &mut Cartridge, address: u16, value: u8);
}


/// Creates a memory bank controller object based on the type given.
pub fn create_mbc(kind: &MemoryBankController) -> Mbc {
    match kind {
        MemoryBankController::None  => Mbc::None(MbcNone::new()),
        MemoryBankController::MBC1  => Mbc::MBC1(Mbc1::new()),
        MemoryBankController::MBC1M => Mbc::MBC1(Mbc1::new_multicart()),
        MemoryBankController::MBC2  => Mbc::MBC2(Mbc2::new()),
        MemoryBankController::MBC5  => Mbc::MBC5(Mbc5::new()),

        _ => {
            #[cfg(feature = "std")]
            unimplemented!("Not implemented: {}", kind);

            #[cfg(not(feature = "std"))]
            unimplemented!("Not implemented")
        }
    }
}


#[cfg(feature = "std")]
impl Display for MemoryBankController {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            MemoryBankController::None  => "None",
            MemoryBankController::MBC1  => "MBC1",
            MemoryBankController::MBC1M => "MBC1M",
            MemoryBankController::MBC2  => "MBC2",
            MemoryBankController::MBC3  => "MBC3",
            MemoryBankController::MBC5  => "MBC5",
            MemoryBankController::MBC6  => "MBC6",
            MemoryBankController::MBC7  => "MBC7",
        };

        write!(f, "{:}", name)
    }
}


impl MbcImpl for Mbc {
    fn read_byte(&self, cartridge: &Cartridge, address: u16) -> u8 {
        match self {
            Mbc::None(mbc_impl)  => mbc_impl.read_byte(cartridge, address),
            Mbc::MBC1(mbc_impl)  => mbc_impl.read_byte(cartridge, address),
            Mbc::MBC1M(mbc_impl) => mbc_impl.read_byte(cartridge, address),
            Mbc::MBC2(mbc_impl)  => mbc_impl.read_byte(cartridge, address),
            Mbc::MBC3(mbc_impl)  => mbc_impl.read_byte(cartridge, address),
            Mbc::MBC5(mbc_impl)  => mbc_impl.read_byte(cartridge, address),
            Mbc::MBC6(mbc_impl)  => mbc_impl.read_byte(cartridge, address),
            Mbc::MBC7(mbc_impl)  => mbc_impl.read_byte(cartridge, address),
        }
    }


    fn write_byte(&mut self, cartridge: &mut Cartridge, address: u16, value: u8) {
        match self {
            Mbc::None(mbc_impl)  => mbc_impl.write_byte(cartridge, address, value),
            Mbc::MBC1(mbc_impl)  => mbc_impl.write_byte(cartridge, address, value),
            Mbc::MBC1M(mbc_impl) => mbc_impl.write_byte(cartridge, address, value),
            Mbc::MBC2(mbc_impl)  => mbc_impl.write_byte(cartridge, address, value),
            Mbc::MBC3(mbc_impl)  => mbc_impl.write_byte(cartridge, address, value),
            Mbc::MBC5(mbc_impl)  => mbc_impl.write_byte(cartridge, address, value),
            Mbc::MBC6(mbc_impl)  => mbc_impl.write_byte(cartridge, address, value),
            Mbc::MBC7(mbc_impl)  => mbc_impl.write_byte(cartridge, address, value),
        }
    }
}


pub mod mbc_none {
    use super::*;


    /// A default MBC handling ROMs which do not need bank switching.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct MbcNone {
    }


    impl MbcNone {
        /// Creates a default MBC object.
        pub fn new() -> MbcNone {
            MbcNone {
            }
        }
    }


    impl MbcImpl for MbcNone {
        fn read_byte(&self, cartridge: &Cartridge, address: u16) -> u8 {
            match address {
                // read from ROM address space
                0x0000 ..= 0x7fff => {
                    cartridge.get_rom().get_at(address as usize)
                },

                // RAM address space (not available on non-MBC)
                0xa000 ..= 0xbfff => {
                    0xff
                }

                _ => unreachable!("Unexpected read from address {}", address),
            }
        }


        fn write_byte(&mut self, _cartridge: &mut Cartridge, _address: u16, _value: u8) {
            // not writing any data
        }
    }
}


mod mbc1 {
    use super::*;
    use crate::mmu::memory_data::MemoryData;


    /// Type 1 Memory Bank Controller:
    /// Supports up to 2MiB ROMs or up to 32kiB RAM.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Mbc1 {
        /// Flag whether to handle a multi cart ROM.
        is_multicart: bool,

        /// Mode switch between Mode 0 = 128 ROM banks 1 RAM bank and Mode 1 = 32 ROM banks, 4 RAM banks.
        mode: u8,

        /// The value written into the first bank selection register.
        /// Contains the lower 5 bits of the selected ROM bank.
        bank_selection_0: u8,

        /// The value written into the second bank selection register.
        /// Contains either the upper two bits of the ROM bank number
        /// or 2 bits for RAM bank selection, depending on the selected mode.
        bank_selection_1: u8,

        /// The selected ROM bank slot #0 number.
        rom_bank_0_selected: u32,

        /// The offset added to the address the game wants to read from,
        /// to get the real address within the ROM file.
        rom_bank_0_offset: usize,

        /// The selected ROM bank slot #1 number.
        rom_bank_1_selected: u32,

        /// The offset added to the address the game wants to read from,
        /// to get the real address within the ROM file.
        rom_bank_1_offset: usize,

        /// The selected RAM bank number.
        ram_bank_selected: u32,

        /// The offset added to the address the game wants to read/write,
        /// to get the real address within the RAM image.
        ram_bank_offset: usize,

        /// Sets if the RAM bank is enabled or not.
        ram_enabled: bool,
    }


    impl Mbc1 {
        pub fn new() -> Mbc1 {
            Mbc1 {
                is_multicart: false,
                mode: 0,

                bank_selection_0: 0x00,
                bank_selection_1: 0x00,

                rom_bank_0_selected: 0,
                rom_bank_0_offset:   0x0000,

                rom_bank_1_selected: 1,
                rom_bank_1_offset:   0x4000,

                ram_bank_selected: 0,
                ram_bank_offset:   0x0000,

                ram_enabled: false,
            }
        }


        pub fn new_multicart() -> Mbc1 {
            Mbc1 {
                is_multicart: true,
                .. Self::new()
            }
        }


        /// After writing to one of the bank selection registers,
        /// this function is used to calculate the actual RAM and ROM bank numbers
        /// as well as the offsets to read and write inside the ROM and RAM images.
        fn update_selected_banks(&mut self, cartridge: &Cartridge) {
            let mut rom_bank_0 = 0u32;
            let mut rom_bank_1 = 0u32;
            let mut ram_bank   = 0u32;

            // on MBC1M multi cart ROMs only 4 bits of the first selection register are used
            // and the 2nd register will become bit 4+5 instead of bit 5+6
            let (bank_selection_0_mask, bank_selection_1_offset) = if self.is_multicart {
                (0b_0000_1111, 4)
            }
            else {
                (0b_0001_1111, 5)
            };

            // Setting the first bank selection register to '0' will select bank '1' instead,
            // so bank '0' should only be accessible on the 0x0000 - 0x3fff address range.
            // Because this check is only done on the first register, not the bank number itself,
            // this also causes bank 0x20, 0x40 and 0x60 to be inaccessible as well through the
            // 2nd bank slot.
            // The check for value 0 is always being done on the full 5 bit register, even if
            // it's not fully used, either on MBC1M which is only using 4 bits, or when a cartridge
            // has less than 16 banks, so less than 5 bits are required to encode the bank number.
            if self.bank_selection_0 != 0 {
                rom_bank_1 |= (self.bank_selection_0 & bank_selection_0_mask) as u32;
            }
            else {
                rom_bank_1 |= 1;
            }

            if self.mode == 0 {
                // in mode 0 the 2 bits of the 2nd register will be used as
                // bit 5 and 6 of the rom bank selection
                rom_bank_1 |= (self.bank_selection_1 as u32) << bank_selection_1_offset;
            }
            else {
                // in mode 1, the bits of the 2nd register will be used
                // for ROM bank selection on both ROM banks, and at the
                // same time to select the RAM bank number
                rom_bank_0 |= (self.bank_selection_1 as u32) << bank_selection_1_offset;
                rom_bank_1 |= (self.bank_selection_1 as u32) << bank_selection_1_offset;
                ram_bank    = (self.bank_selection_1 as u32) << 0;
            }

            // store the rom bank and the offset to be added to all requested addresses
            if cartridge.get_rom_bank_count() != 0 {
                rom_bank_0 = rom_bank_0 % cartridge.get_rom_bank_count();
                rom_bank_1 = rom_bank_1 % cartridge.get_rom_bank_count();

                self.rom_bank_0_selected = rom_bank_0;
                self.rom_bank_0_offset   = (rom_bank_0 as usize) * 0x4000;

                self.rom_bank_1_selected = rom_bank_1;
                self.rom_bank_1_offset   = (rom_bank_1 as usize) * 0x4000;
            }

            // store the ram bank and the offset to be added to all addresses
            if cartridge.get_ram_bank_count() != 0 {
                ram_bank = ram_bank % cartridge.get_ram_bank_count();

                self.ram_bank_selected = ram_bank;
                self.ram_bank_offset   = (ram_bank as usize) * 0x2000;
            }
        }
    }


    impl MbcImpl for Mbc1 {
        fn read_byte(&self, cartridge: &Cartridge, address: u16) -> u8 {
            match address {
                // read from fixed ROM bank, which is always bank 0.
                0x0000 ..= 0x3fff => {
                    let rom_address = (address as usize) + self.rom_bank_0_offset;
                    cartridge.get_rom().get_at(rom_address)
                },

                // read from the switchable ROM bank
                0x4000 ..= 0x7fff => {
                    let rom_address = (address as usize) + self.rom_bank_1_offset - 0x4000;
                    cartridge.get_rom().get_at(rom_address)
                },

                // read from switchable RAM bank.
                0xa000 ..= 0xbfff => {
                    if cartridge.has_ram() && self.ram_enabled {
                        let ram_address = (address as usize) - 0xa000 + self.ram_bank_offset;
                        cartridge.get_ram().get_at(ram_address)
                    }
                    else {
                        0xff
                    }
                }

                _ => unreachable!("Unexpected read from address {}", address),
            }
        }


        fn write_byte(&mut self, cartridge: &mut Cartridge, address: u16, value: u8) {
            match (address >> 13) & 0x000f {
                // 0x0000 - 0x1fff: enable or disable RAM
                0x00 => {
                    self.ram_enabled = (value & 0x0f) == 0x0a;
                },

                // 0x2000 - 0x3fff: select ROM bank
                0x01 => {
                    self.bank_selection_0 = value & 0x1f;
                    self.update_selected_banks(cartridge);
                },

                // 0x4000 - 0x5fff: select RAM bank
                0x02 => {
                    self.bank_selection_1 = value & 0x03;
                    self.update_selected_banks(cartridge);
                },

                // 0x6000 - 0x7fff: switch ROM / RAM mode
                0x03 => {
                    self.mode = value & 0x01;
                    self.update_selected_banks(cartridge);
                },

                // 0xa000 - 0xbfff: Cartridge RAM
                0x05 => {
                    if cartridge.has_ram() && self.ram_enabled {
                        let ram_address = (address as usize) - 0xa000 + self.ram_bank_offset;
                        cartridge.get_ram_mut().set_at(ram_address, value);
                    }
                },

                _ => unreachable!("Unexpected write to address {}", address),
            }
        }
    }
}


mod mbc2 {
    use super::*;
    use crate::mmu::memory_data::{MemoryData, MemoryDataFixedSize};
    use crate::utils::{get_bit, get_high};


    /// Type 2 Memory Bank Controller:
    /// Supports up to 256kiB ROMs and up to 128kB RAM.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Mbc2 {
        /// The value written into the bank selection register.
        /// Bits 0-3 are used to select the ROM bank number.
        bank_selection_0: u8,

        /// The selected ROM bank number.
        rom_bank_selected: u32,

        /// The offset added to the address the game wants to read from,
        /// to get the real address within the ROM file.
        rom_bank_offset:   usize,

        /// 512 half-bytes Built-in RAM.
        ram: MemoryDataFixedSize<512>,

        /// Sets if the RAM bank is enabled or not.
        ram_enabled: bool,
    }


    impl Mbc2 {
        pub fn new() -> Self {
            Self {
                bank_selection_0: 0x00,

                rom_bank_selected: 1,
                rom_bank_offset:   0x4000,

                ram: MemoryDataFixedSize::new(),

                ram_enabled: false,
            }
        }


        /// After writing to one of the bank selection registers,
        /// this function is used to calculate the actual RAM and ROM bank numbers
        /// as well as the offsets to read and write inside the ROM and RAM images.
        fn update_selected_banks(&mut self, cartridge: &Cartridge) {
            let rom_bank = if self.bank_selection_0 != 0 {
                self.bank_selection_0 as u32
            }
            else {
                1
            };

            // store the rom bank and the offset to be added to all
            // requested addresses, beginning with 0x4000
            if cartridge.get_rom_bank_count() != 0 {
                self.rom_bank_selected = rom_bank % cartridge.get_rom_bank_count();
                self.rom_bank_offset   = (self.rom_bank_selected as usize) * 0x4000;
            }
        }
    }


    impl MbcImpl for Mbc2 {
        fn read_byte(&self, cartridge: &Cartridge, address: u16) -> u8 {
            match address {
                // read from fixed ROM bank, which is always bank 0.
                0x0000 ..= 0x3fff => {
                    let rom_address = address as usize;
                    cartridge.get_rom().get_at(rom_address)
                },

                // read from the switchable ROM bank
                0x4000 ..= 0x7fff => {
                    let rom_address = (address as usize) + self.rom_bank_offset - 0x4000;
                    cartridge.get_rom().get_at(rom_address)
                },

                // read from switchable RAM bank.
                0xa000 ..= 0xbfff => {
                    if self.ram_enabled {
                        // only the lowest 9 bits of the address are used, so
                        // 0xa200 to 0xbfff are mirroring the RAM area 15 times
                        let ram_address = ((address - 0xa000) & 0x1ff) as usize;
                        let ram_value   = self.ram.get_at(ram_address) & 0x0f;
                        0xf0 | ram_value
                    }
                    else {
                        0xff
                    }
                }

                _ => unreachable!("Unexpected read from address {}", address),
            }
        }


        fn write_byte(&mut self, cartridge: &mut Cartridge, address: u16, value: u8) {
            match address {
                // bank selection / RAM enable register
                0x0000 ..= 0x3fff => {
                    // if bit 8 of the address is set..
                    if get_bit(get_high(address), 0) {
                        // .. take the value to select the ROM bank number ..
                        self.bank_selection_0 = value & 0x0f;
                        self.update_selected_banks(cartridge);
                    }
                    else {
                        // .. otherwise take the value to enable or disable RAM
                        self.ram_enabled = (value & 0x0f) == 0x0a;
                    }
                },

                // invalid address
                0x4000 ..= 0x7fff => {
                },

                // Cartridge RAM
                0xa000 ..= 0xbfff => {
                    if self.ram_enabled {
                        // only the lowest 9 bits of the address are used, so
                        // 0xa200 to 0xbfff are mirroring the RAM area 15 times
                        let ram_address = (address & 0x1ff) as usize;

                        // the cartridge RAM only stores half-bytes,
                        // so ignore the upper nibble
                        let ram_value   = value & 0x0f;

                        self.ram.set_at(ram_address, ram_value);
                    }
                },

                _ => unreachable!("Unexpected write to address {}", address),
            }
        }
    }
}


mod mbc5 {
    use super::*;
    use crate::mmu::memory_data::MemoryData;


    /// Type 5 Memory Bank Controller:
    /// Supports up to 8MiB ROMs and up to 128kiB RAM.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Mbc5 {
        /// The value written into the first bank selection register.
        /// Contains the lower 8 bits of the selected ROM bank.
        rom_bank_selection_0: u8,

        /// The value written into the second bank selection register.
        /// Contains the 9th bit of the selected ROM bank.
        rom_bank_selection_1: u8,

        /// The value written into the RAM bank selection register.
        ram_bank_selection_0: u8,

        /// The selected ROM bank number.
        rom_bank_selected: u32,

        /// The offset added to the address the game wants to read from,
        /// to get the real address within the ROM file.
        rom_bank_offset:   usize,

        /// The selected RAM bank number.
        ram_bank_selected: u32,

        /// The offset added to the address the game wants to read/write,
        /// to get the real address within the RAM image.
        ram_bank_offset: usize,

        /// Sets if the RAM bank is enabled or not.
        ram_enabled: bool,
    }


    impl Mbc5 {
        pub fn new() -> Self {
            Self {
                rom_bank_selection_0: 0x00,
                rom_bank_selection_1: 0x00,
                ram_bank_selection_0: 0x00,

                rom_bank_selected: 1,
                rom_bank_offset:   0x4000,

                ram_bank_selected: 0,
                ram_bank_offset:   0x0000,

                ram_enabled: false,
            }
        }


        /// After writing to one of the bank selection registers,
        /// this function is used to calculate the actual RAM and ROM bank numbers
        /// as well as the offsets to read and write inside the ROM and RAM images.
        fn update_selected_banks(&mut self, cartridge: &Cartridge) {
            let rom_bank =
                (self.rom_bank_selection_0 as u32)
              | ((self.rom_bank_selection_1 as u32 & 0x01) << 8)
            ;

            let ram_bank =
                self.ram_bank_selection_0 as u32
            ;

            // store the rom bank and the offset to be added to all
            // requested addresses, beginning with 0x4000
            if cartridge.get_rom_bank_count() != 0 {
                self.rom_bank_selected = rom_bank % cartridge.get_rom_bank_count();
                self.rom_bank_offset   = (self.rom_bank_selected as usize) * 0x4000;
            }

            // store the ram bank and the offset to be added to all addresses
            if cartridge.get_ram_bank_count() != 0 {
                self.ram_bank_selected = ram_bank % cartridge.get_ram_bank_count();
                self.ram_bank_offset   = (self.ram_bank_offset as usize) * 0x2000;
            }
        }
    }


    impl MbcImpl for Mbc5 {
        fn read_byte(&self, cartridge: &Cartridge, address: u16) -> u8 {
            match address {
                // read from fixed ROM bank, which is always bank 0.
                0x0000 ..= 0x3fff => {
                    let rom_address = address as usize;
                    cartridge.get_rom().get_at(rom_address)
                },

                // read from the switchable ROM bank
                0x4000 ..= 0x7fff => {
                    let rom_address = (address as usize) + self.rom_bank_offset - 0x4000;
                    cartridge.get_rom().get_at(rom_address)
                },

                // read from switchable RAM bank.
                0xa000 ..= 0xbfff => {
                    if cartridge.has_ram() && self.ram_enabled {
                        let ram_address = (address as usize) + self.ram_bank_offset - 0xa000;
                        cartridge.get_ram().get_at(ram_address)
                    }
                    else {
                        0xff
                    }
                }

                _ => unreachable!("Unexpected read from address {}", address),
            }
        }


        fn write_byte(&mut self, cartridge: &mut Cartridge, address: u16, value: u8) {
            match address {
                // enable or disable RAM
                0x0000 ..= 0x1fff => {
                    self.ram_enabled = (value & 0x0f) == 0x0a;
                },

                // ROM bank selection #0
                0x2000 ..= 0x2fff => {
                    self.rom_bank_selection_0 = value;
                    self.update_selected_banks(cartridge);
                },

                // ROM bank selection #1
                0x3000 ..= 0x3fff => {
                    self.rom_bank_selection_1 = value;
                    self.update_selected_banks(cartridge);
                },

                // RAM bank selection
                0x4000 ..= 0x5fff => {
                    self.ram_bank_selection_0 = value & 0x0f;
                    self.update_selected_banks(cartridge);
                },

                // invalid address
                0x6000 ..= 0x7fff => {
                },

                // Cartridge RAM
                0xa000 ..= 0xbfff => {
                    if cartridge.has_ram() && self.ram_enabled {
                        let ram_address = (address as usize) - 0xa000 + self.ram_bank_offset;
                        cartridge.get_ram_mut().set_at(ram_address, value);
                    }
                },

                _ => unreachable!("Unexpected write to address {}", address),
            }
        }
    }
}
