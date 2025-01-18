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

use gemi_core::input::{Input, InputButton};
use gemi_support::GameBoy;


/// An utility to map key bindings of a frontend system to emulator input buttons.
/// This can bind multiple native keys to a single input button.
/// A template argument [KeyCode] is used to define the native key codes.
pub struct KeyBindings<KeyCode>
    where KeyCode: Eq
{
    /// The list of key bindings stored for this object.
    bindings: Vec<KeyBindEntry<KeyCode>>,
}


/// A single key binding entry mapping a set of native [KeyCode] values to a single [InputButton].
struct KeyBindEntry<KeyCode>
    where KeyCode: Eq
{
    /// The [InputButton] which will receive the pressed state from the native [KeyCode] values.
    button: InputButton,

    /// The list of key entries, which contains the mapped [KeyCode] and their current state.
    keys: Vec<KeyCodeEntry<KeyCode>>,
}


/// A single entry storing a [KeyCode] and its current state.
struct KeyCodeEntry<KeyCode>
    where KeyCode: Eq
{
    /// The native [KeyCode] value.
    key: KeyCode,

    /// The current pressed state of the key.
    pressed: bool,
}


impl<KeyCode> Default for KeyBindings<KeyCode>
    where KeyCode: Eq
{
    fn default() -> Self {
        Self {
            // create an empty entry for each button
            bindings: InputButton::ALL
                .iter()
                .map(|button| KeyBindEntry::new_button(*button))
                .collect()
        }
    }
}


impl<KeyCode> KeyBindings<KeyCode>
    where KeyCode: Eq
{
    /// Creates a new [KeyBindings] object with a given key mapping.
    /// The mapping will consist of a list of tuples containing an [InputButton]
    /// and a list of [KeyCode] values.
    pub fn with_mapping(key_binds: Vec<(InputButton, Vec<impl Into<KeyCode>>)>) -> Self {
        Self {
            bindings: key_binds
                .into_iter()
                .map(|(button, keys)| KeyBindEntry {
                    button,
                    keys: keys
                        .into_iter()
                        .map(|key| key.into())
                        .map(|key| KeyCodeEntry::new_key(key))
                        .collect()
                })
                .collect()
        }
    }


    /// Assign a new key binding to a given button.
    pub fn add_keybinding(&mut self, button: InputButton, key: KeyCode) {
        if let Some(entry) = self.find_entry_mut(button) {
            entry.add_key(key);
        }
    }


    /// Removes a key binding from a given button.
    /// The button itself won't be removed from the list of bindings, after removing all key codes,
    /// but cannot be activated anymore.
    pub fn remove_keybinding(&mut self, button: InputButton, key: &KeyCode) {
        if let Some(entry) = self.find_entry_mut(button) {
            entry.remove_key(key);
        }
    }


    /// Set the pressed state of a single key.
    /// This will update the state of any button key binding which contains the given key.
    pub fn set_key_pressed(&mut self, key: KeyCode, pressed: bool) {
        for entry in self.bindings.iter_mut() {
            if let Some(key_entry) = entry.find_key_code_entry_mut(&key) {
                key_entry.pressed = pressed;
            }
        }
    }


    /// Set the pressed state of a single key.
    /// This will update the state of any button key binding which contains the given key
    /// and then forward the [InputButton] state to the emulator reference.
    pub fn set_key_pressed_and_fwd(&mut self, key: KeyCode, pressed: bool, gb: &mut GameBoy) {
        for entry in self.bindings.iter_mut() {
            if let Some(key_entry) = entry.find_key_code_entry_mut(&key) {
                if key_entry.pressed != pressed {
                    key_entry.pressed = pressed;

                    // update the emulator's input state
                    gb.get_input_mut().set_button_pressed(
                        entry.button,
                        entry.is_any_pressed(),
                    );
                }
            }
        }
    }


    /// Apply the current state of each [InputButton] to the emulator reference.
    pub fn apply_button_states(&self, gb: &mut GameBoy) {
        self.apply_button_states_to_input(gb.get_input_mut());
    }


    /// Apply the current state of each [InputButton] to the reference of the emulator input system.
    pub fn apply_button_states_to_input(&self, input: &mut Input) {
        for entry in self.bindings.iter() {
            input.set_button_pressed(
                entry.button,
                entry.is_any_pressed(),
            );
        }
    }


    /// Reset all key states to 'not pressed'.
    /// Additionally this will reset the state of each assigned [InputButton] to 'not pressed'
    /// and forward this state into the emulator.
    pub fn reset_key_states(&mut self, gb: &mut GameBoy) {
        for entry in self.bindings.iter_mut() {
            // reset the state of each key
            for key_entry in entry.keys.iter_mut() {
                key_entry.pressed = false;
            }

            // update the emulator's input state
            gb.get_input_mut().set_button_pressed(
                entry.button,
                false
            );
        }
    }


    /// Find the key binding entry for any given button.
    fn find_entry_mut(&mut self, button: InputButton) -> Option<&mut KeyBindEntry<KeyCode>> {
        self.bindings
            .iter_mut()
            .find(|entry| entry.button == button)
    }
}


impl<KeyCode> KeyBindEntry<KeyCode>
    where KeyCode: Eq
{
    /// Creates a new [KeyBindEntry] object with a given [InputButton].
    fn new_button(button: InputButton) -> Self {
        Self {
            button,
            keys: Vec::new(),
        }
    }


    /// Add a new [KeyCode] to the list of keys.
    fn add_key(&mut self, key: KeyCode) {
        if self.find_key_code_entry_mut(&key).is_none() {
            self.keys.push(KeyCodeEntry {
                key,
                pressed: false,
            });
        }
    }


    /// Remove a [KeyCode] from the list of keys.
    fn remove_key(&mut self, key: &KeyCode) {
        if let Some(index) = self.keys.iter().position(|entry| entry.key == *key) {
            self.keys.remove(index);
        }
    }


    /// Find the key code entry for any given key.
    pub fn find_key_code_entry_mut(&mut self, key: &KeyCode) -> Option<&mut KeyCodeEntry<KeyCode>> {
        self.keys
            .iter_mut()
            .find(|entry| entry.key == *key)
    }


    /// Checks whether any key is currently pressed and therefor
    /// the assigned [InputButton] should be pressed.
    pub fn is_any_pressed(&self) -> bool {
        self.keys.iter().any(|entry| entry.pressed)
    }
}


impl<KeyCode> KeyCodeEntry<KeyCode>
    where KeyCode: Eq
{
    /// Creates a new [KeyCodeEntry] object with a given [KeyCode].
    pub fn new_key(key: KeyCode) -> Self {
        Self {
            key,
            pressed: false,
        }
    }
}
