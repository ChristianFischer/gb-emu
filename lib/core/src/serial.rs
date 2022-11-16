/*
 * Copyright (C) 2022 by Christian Fischer
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

use crate::cpu::Interrupt;
use crate::gameboy::{clock_t, DeviceConfig};
use crate::memory::{MEMORY_LOCATION_SB, MEMORY_LOCATION_SC, MemoryRead, MemoryReadWriteHandle, MemoryWrite};


const UPDATE_TIME_SERIAL_TRANSFER:      clock_t = 4096;


/// An implementation of the GameBoy's serial port.
/// This is a simplified implementation with the main
/// purpose of receiving data from test ROMs like Blargg's.
/// It currently does not provide precise timing, receiving
/// input or double speed mode.
///
/// The output queue is disabled by default and needs to be
/// enabled in order to store data sent.
pub struct SerialPort {
    /// The SerialPort's clock to measure time between the transfer of each byte.
    clock: clock_t,

    /// Access to the device memory.
    mem: MemoryReadWriteHandle,

    /// A queue of all bytes sent by the device.
    output_queue: Vec<u8>,

    /// A flag to enable or disable the output queue.
    output_queue_enabled: bool,
}


impl SerialPort {
    /// Constructs a new instance of the SerialPort.
    pub fn new(mem: MemoryReadWriteHandle) -> SerialPort {
        SerialPort {
            clock: 0,
            mem,
            output_queue: vec![],
            output_queue_enabled: false,
        }
    }


    /// Updates the SerialPort, perform data transfer if any data is pending.
    pub fn update(&mut self, cycles: clock_t) {
        self.clock += cycles;

        if self.clock >= UPDATE_TIME_SERIAL_TRANSFER {
            let transfer_enabled = self.mem.get_bit(MEMORY_LOCATION_SC, 7);

            if transfer_enabled {
                let transfer_byte = self.mem.read_byte(MEMORY_LOCATION_SB);

                // store the data only if the output queue is enabled
                if self.output_queue_enabled {
                    self.output_queue.push(transfer_byte);
                }

                // after transfer completion, disable the transfer status bit
                self.mem.clear_bit(MEMORY_LOCATION_SC, 7);

                // ..  and raise serial transfer interrupt
                self.mem.request_interrupt(Interrupt::Serial);
            }


            self.clock -= UPDATE_TIME_SERIAL_TRANSFER;
        }
    }


    /// Enables the output queue to store data sent by the program.
    pub fn enable_output_queue(&mut self, enabled: bool) {
        self.output_queue_enabled = enabled;
    }


    /// Takes the data currently in the output queue.
    /// The data will then be removed from the current output queue.
    pub fn take_output(&mut self) -> Vec<u8> {
        let result = self.output_queue.clone();
        self.output_queue.clear();

        result
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