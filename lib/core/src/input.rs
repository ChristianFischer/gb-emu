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

use core::mem::take;

use crate::cpu::interrupts::Interrupt;
use crate::mmu::locations::MEMORY_LOCATION_JOYP;
use crate::mmu::memory_bus::{MemoryBusConnection, MemoryBusSignals};
use crate::utils::{change_bit, get_bit};

/// A list of all buttons available on the GameBoy
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InputButton {
    DPadRight   = 0,
    DPadLeft    = 1,
    DPadUp      = 2,
    DPadDown    = 3,
    A           = 4,
    B           = 5,
    Select      = 6,
    Start       = 7,
}


#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Input {
    /// Pending output to be sent back through the memory bus.
    signals: MemoryBusSignals,

    /// JOYP bits 4 + 5 used to select which button states are returned by reading JOYP:
    /// * 0b_0001_0000 -> direction buttons
    /// * 0b_0010_0000 -> action buttons
    button_selection: u8,

    /// Current state of each button.
    /// bit == 1 means pressed, bit == 0 means released
    button_states: u8,

    /// The pressed state of each button last time;
    /// used to detect changes in the pressed state of each button
    previous_button_states: u8,
}


impl InputButton {
    /// A list of all buttons available on the GameBoy.
    pub const ALL: [InputButton; 8] = [
        InputButton::DPadRight,
        InputButton::DPadLeft,
        InputButton::DPadUp,
        InputButton::DPadDown,
        InputButton::A,
        InputButton::B,
        InputButton::Select,
        InputButton::Start,
    ];
}


impl Input {
    /// Creates a new Input object.
    pub fn new() -> Input {
        Input {
            signals:                MemoryBusSignals::default(),
            button_selection:       0x00,
            button_states:          0x00,
            previous_button_states: 0x00,
        }
    }


    /// Updates the JOYP register and fire the input interrupt depending on the current button states.
    pub fn update(&mut self) {
        let keys_changed = self.button_states != self.previous_button_states;

        if keys_changed {
            let keys_pressed = self.button_states & !self.previous_button_states;

            // when any key was pressed this frame, fire the input interrupt
            if keys_pressed != 0 {
                self.request_interrupt(Interrupt::Input);
            }

            self.previous_button_states = self.button_states;
        }
    }

    /// Changes the 'pressed' state for any button.
    pub fn set_button_pressed(&mut self, button: InputButton, pressed: bool) {
        self.button_states = change_bit(self.button_states, button as u8, pressed);
    }

    /// Checks whether a particular button is currently pressed.
    pub fn is_button_pressed(&self, button: InputButton) -> bool {
        get_bit(self.button_states, button as u8)
    }

    /// Requests an interrupt to be fired.
    fn request_interrupt(&mut self, interrupt: Interrupt) {
        self.signals.interrupts |= interrupt;
    }
}


impl MemoryBusConnection for Input {
    fn on_read(&self, address: u16) -> u8 {
        match address {
            MEMORY_LOCATION_JOYP => {
                let states = match self.button_selection {
                    0x00 => 0x00,
                    0x10 => (!self.button_states >> 4) & 0x0f,
                    0x20 => (!self.button_states >> 0) & 0x0f,
                    _    => 0x0f,
                };

                0b_1100_0000 | states | self.button_selection
            },

            _ => 0xff
        }
    }


    fn on_write(&mut self, address: u16, value: u8) {
        match address {
            MEMORY_LOCATION_JOYP => {
                self.button_selection = value & 0x30;
            },

            _ => { }
        };
    }


    fn take_signals(&mut self) -> MemoryBusSignals {
        take(&mut self.signals)
    }
}
