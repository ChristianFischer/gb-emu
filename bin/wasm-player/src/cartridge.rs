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

use gemi_core::cartridge::Cartridge as NativeCartridge;
use gemi_core::mmu::memory_data::MemoryData;
use wasm_bindgen::prelude::wasm_bindgen;


/// A wrapper around the internal cartridge type to expose it to the JS side.
#[wasm_bindgen]
pub struct Cartridge {
    cartridge: NativeCartridge,
}


#[wasm_bindgen]
impl Cartridge {
    /// Load a cartridge from a byte array.
    #[wasm_bindgen]
    pub fn load_from_bytes(bytes: Vec<u8>) -> Result<Cartridge, String> {
        let cartridge = NativeCartridge::load_from_bytes(bytes, NativeCartridge::NO_RAM)
            .map_err(|e| format!("Failed to load cartridge: {}", e))
            ?;

        Ok(
            Cartridge {
                cartridge
            }
        )
    }


    /// Load the cartridge RAM from a byte array.
    pub fn load_ram_from_bytes(&mut self, bytes: Vec<u8>) -> Result<(), String> {
        self.cartridge
            .get_ram_mut()
            .read_from_bytes(bytes.as_slice())
            .map_err(|e| format!("Failed to load cartridge RAM: {}", e))
    }


    /// Get the title of the cartridge.
    #[wasm_bindgen]
    pub fn get_title(&self) -> String {
        self.cartridge.get_title().clone()
    }


    /// Checks whether this cartridge supports GameBoy Color features or not.
    #[wasm_bindgen]
    pub fn is_gbc(&self) -> bool {
        self.cartridge.supports_cgb()
    }
}


impl Into<NativeCartridge> for Cartridge {
    fn into(self) -> NativeCartridge {
        self.cartridge
    }
}
