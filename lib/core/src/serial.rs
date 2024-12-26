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

use std::mem::take;

use crate::cpu::interrupts::Interrupt;
use crate::gameboy::Clock;
use crate::mmu::locations::{MEMORY_LOCATION_SB, MEMORY_LOCATION_SC};
use crate::mmu::memory_bus::{MemoryBusConnection, MemoryBusSignals};
use crate::utils::{as_bit_flag, get_bit};

const UPDATE_TIME_SERIAL_TRANSFER:      Clock = 4096;


/// An implementation of the GameBoy's serial port.
/// This is a simplified implementation with the main
/// purpose of receiving data from test ROMs like Blargg's.
/// It currently does not provide precise timing, receiving
/// input or double speed mode.
///
/// The output queue is disabled by default and needs to be
/// enabled in order to store data sent.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SerialPort {
    /// The SerialPort's clock to measure time between the transfer of each byte.
    clock: Clock,

    /// Pending output to be sent back through the memory bus.
    signals: MemoryBusSignals,

    /// The flag written by SC register to enable or disable serial data transfer.
    transfer_enabled: bool,

    /// The current byte written via SB register to be transferred.
    transfer_byte: u8,

    /// A queue of all bytes sent by the device.
    output_queue: Vec<u8>,

    /// A flag to enable or disable the output queue.
    output_queue_enabled: bool,
}


impl SerialPort {
    /// Constructs a new instance of the SerialPort.
    pub fn new() -> SerialPort {
        SerialPort {
            clock:                  0,
            signals:                MemoryBusSignals::default(),
            transfer_enabled:       false,
            transfer_byte:          0x00,
            output_queue:           vec![],
            output_queue_enabled:   false,
        }
    }


    /// Updates the SerialPort, perform data transfer if any data is pending.
    pub fn update(&mut self, cycles: Clock) {
        self.clock += cycles;

        if self.clock >= UPDATE_TIME_SERIAL_TRANSFER {
            if self.transfer_enabled {
                // store the data only if the output queue is enabled
                if self.output_queue_enabled {
                    self.output_queue.push(self.transfer_byte);
                }

                // after transfer completion, disable the transfer status bit
                self.transfer_enabled = false;

                // ..  and raise serial transfer interrupt
                self.request_interrupt(Interrupt::Serial);
            }


            self.clock -= UPDATE_TIME_SERIAL_TRANSFER;
        }
    }


    /// Requests an interrupt to be fired.
    fn request_interrupt(&mut self, interrupt: Interrupt) {
        self.signals.interrupts |= interrupt;
    }


    /// Enables the output queue to store data sent by the program.
    pub fn enable_output_queue(&mut self, enabled: bool) {
        self.output_queue_enabled = enabled;
    }


    /// Get the data currently in the output queue.
    pub fn get_output(&self) -> Vec<u8> {
        self.output_queue.clone()
    }


    /// Get the data currently in the output queue interpreted as a text string.
    #[cfg(feature = "std")]
    pub fn get_output_as_text(&self) -> String {
        self.get_output()
            .into_iter()
            .map(|b| b as char)
            .collect()
    }


    /// Takes the data currently in the output queue.
    /// The data will then be removed from the current output queue.
    pub fn take_output(&mut self) -> Vec<u8> {
        take(&mut self.output_queue)
    }


    /// Takes the data currently in the output queue interpreted as a text string.
    /// The data will then be removed from the current output queue.
    pub fn take_output_as_text(&mut self) -> String {
        self.take_output()
            .into_iter()
            .map(|b| b as char)
            .collect()
    }


    /// Takes the next byte from the output queue.
    pub fn take_next(&mut self) -> Option<u8> {
        if !self.output_queue.is_empty() {
            let next_byte = self.output_queue.remove(0);
            return Some(next_byte);
        }

        None
    }
}


impl MemoryBusConnection for SerialPort {
    fn on_read(&self, address: u16) -> u8 {
        match address {
            MEMORY_LOCATION_SB => self.transfer_byte,
            MEMORY_LOCATION_SC => 0b_0111_1111 | as_bit_flag(self.transfer_enabled, 7),
            _ => 0xff
        }
    }


    fn on_write(&mut self, address: u16, value: u8) {
        match address {
            MEMORY_LOCATION_SB => self.transfer_byte    = value,
            MEMORY_LOCATION_SC => self.transfer_enabled = get_bit(value, 7),
            _ => { }
        };
    }


    fn take_signals(&mut self) -> MemoryBusSignals {
        take(&mut self.signals)
    }
}
