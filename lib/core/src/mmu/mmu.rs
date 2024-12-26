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

use core::cmp::min;

use crate::gameboy::{Clock, Peripherals};
use crate::mmu::locations::*;
use crate::mmu::memory::{DmaTransferInfo, DmaTransferState};
use crate::mmu::memory_bus::{impl_memory_mapper, MemoryBus, MemoryBusConnection, MemoryMapper};
use crate::mmu::memory_data::MemoryData;
use crate::utils::{to_u16, to_u8};

/// The memory management unit, which provides an interface to read and write the device memory.
/// IO operations are performed via memory bus, which maps memory addresses to their according
/// device.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mmu {
    internal: MmuInternal,
}


/// Private part of the MMU object. This implements the actual memory bus trait to forward
/// IO operations to their actual components.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MmuInternal {
    peripherals: Peripherals,

    dma: DmaTransferState,

    /// Stores the value written into the DMA register.
    dma_register_value: u8,
}


impl Mmu {
    pub fn new(peripherals: Peripherals) -> Self {
        Self {
            internal: MmuInternal {
                peripherals,

                dma: DmaTransferState::Disabled,
                dma_register_value: 0xff,
            }
        }
    }


    /// Get the peripherals connected to the memory bus.
    pub fn get_peripherals(&self) -> &Peripherals {
        &self.internal.peripherals
    }


    /// Get the peripherals connected to the memory bus.
    pub fn get_peripherals_mut(&mut self) -> &mut Peripherals {
        &mut self.internal.peripherals
    }


    /// Reads a single byte value from the memory bus on a given address.
    pub fn read_u8(&self, address: u16) -> u8 {
        self.internal.read(address)
    }


    /// Reads two bytes into a 16 bit integer from the memory bus on a given address.
    pub fn read_u16(&self, address: u16) -> u16 {
        let l = self.read_u8(address.wrapping_add(0));
        let h = self.read_u8(address.wrapping_add(1));
        to_u16(h, l)
    }


    /// Writes a single byte value to the memory bus on a given address.
    pub fn write_u8(&mut self, address: u16, value: u8) {
        self.internal.write(address, value);
    }


    /// Writes two bytes from a 16 bit integer to the memory bus on a given address.
    pub fn write_u16(&mut self, address: u16, value: u16) {
        let (h, l) = to_u8(value);
        self.write_u8(address.wrapping_add(0), l);
        self.write_u8(address.wrapping_add(1), h);
    }


    /// Let the memory controller handle it's tasks.
    /// 'cycles' gives the number of ticks passed since
    /// the last call.
    pub fn update(&mut self, cycles: Clock) {
        self.internal.handle_dma_transfer(cycles);
    }
}


impl MmuInternal {
    /// Handles an OAM DMA transfer, if any active.
    fn handle_dma_transfer(&mut self, cycles: Clock) {
        match self.dma {
            DmaTransferState::Disabled => {}

            DmaTransferState::Transferring(ref transfer) => {
                let oam_size       = MEMORY_LOCATION_OAM_END - MEMORY_LOCATION_OAM_BEGIN + 1;
                let transfer_begin = transfer.next_byte;
                let transfer_end   = min(transfer_begin.saturating_add(cycles as u16), oam_size);

                // Copy the amount of data the memory controller was able to handle
                for b in transfer_begin .. transfer_end {
                    let src = transfer.start_address.saturating_add(b);
                    let dst = b as usize;

                    let val = self.read(src);
                    self.peripherals.ppu.get_oam_bank_mut().set_at(dst, val);
                }

                // store the current state or set the transfer state to 'Disabled'
                if transfer_end == oam_size {
                    self.dma = DmaTransferState::Disabled;
                }
                else {
                    self.dma = DmaTransferState::Transferring(DmaTransferInfo {
                        start_address: transfer.start_address,
                        next_byte:     transfer_end,
                    });
                }
            }
        }
    }
}


impl MemoryBus<MmuInternal, MmuInternal> for MmuInternal {
    fn get_root(&self) -> &MmuInternal {
        self
    }

    fn get_root_mut(&mut self) -> &mut MmuInternal {
        self
    }
}


impl MemoryBusConnection for MmuInternal {
    fn on_read(&self, address: u16) -> u8 {
        match address {
            MEMORY_LOCATION_DMA_ADDRESS => self.dma_register_value,
            _ => 0xff,
        }
    }

    fn on_write(&mut self, address: u16, value: u8) {
        match address {
            MEMORY_LOCATION_DMA_ADDRESS => {
                self.dma_register_value = value;
                let start_address = (value as u16) << 8;

                self.dma = DmaTransferState::Transferring(DmaTransferInfo {
                    start_address,
                    next_byte: 0,
                });
            },

            _ => { }
        }
    }
}


impl_memory_mapper!(
    MemoryMapper(root: MmuInternal) for MmuInternal {
        // Cartridge ROM
        0x0000 ..= 0x7fff => *root.peripherals.mem,

        // Video RAM
        0x8000 ..= 0x9fff => *root.peripherals.ppu,

        // External RAM
        0xa000 ..= 0xbfff => *root.peripherals.mem,

        // WRAM, Mirror RAM
        0xc000 ..= 0xfdff => *root.peripherals.mem,

        // OAM
        0xfe00 ..= 0xfe9f => *root.peripherals.ppu,

        // Restricted RAM area
        0xfea0 ..= 0xfeff => *root.peripherals.mem,

        // input registers
        0xff00 ..= 0xff00 => *root.peripherals.input,

        // serial data transfer registers
        0xff01 ..= 0xff02 => *root.peripherals.serial,

        // timer registers
        0xff04 ..= 0xff07 => *root.peripherals.timer,

        // APU registers
        0xff10 ..= 0xff3f => *root.peripherals.apu,

        // PPU registers
        0xff40 ..= 0xff45 => *root.peripherals.ppu,
        0xff47 ..= 0xff4f => *root.peripherals.ppu,
        0xff68 ..= 0xff6b => *root.peripherals.ppu,

        MEMORY_LOCATION_DMA_ADDRESS => *root,

        MEMORY_LOCATION_INTERRUPTS_FLAGGED => *root.peripherals.interrupts,
        MEMORY_LOCATION_INTERRUPTS_ENABLED => *root.peripherals.interrupts,

        // IO Registers
        0xff00 ..= 0xff7f => *root.peripherals.mem,

        // HRAM
        0xff80 ..= 0xfffe => *root.peripherals.mem
    }
);

