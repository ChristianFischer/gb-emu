/*
 * Copyright (C) 2022-2023 by Christian Fischer
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

use egui::Ui;
use libgemi::core::cartridge::GameBoyColorSupport;
use crate::state::EmulatorState;
use crate::ui::data_list::DataList;
use crate::views::View;


/// A view to display information about the currently loaded cartridge.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct CartridgeInfoView {
    /// A [DataList] storing the current data to be displayed.
    /// Since the cartridge's attributes wont change during runtime,
    /// the list will be filled only once when a cartridge was loaded.
    #[serde(skip)]
    #[serde(default = "create_data_list")]
    data_list: DataList,
}


/// Helper function for serde to create a new, empty [DataList] object.
fn create_data_list() -> DataList {
    DataList::new("cartridge_info".to_string())
}


impl CartridgeInfoView {
    /// Creates a new [`CartridgeInfoView`] object.
    pub fn new() -> Self {
        Self {
            data_list: create_data_list(),
        }
    }
}


impl View for CartridgeInfoView {
    fn title(&self, _state: &mut EmulatorState) -> &str {
        "Cartridge Info"
    }


    fn ui(&mut self, _state: &mut EmulatorState, ui: &mut Ui) {
        self.data_list.ui(ui);
    }


    fn on_emulator_loaded(&mut self, state: &mut EmulatorState) {
        // expect a cartridge to be present after emulator loading
        let cart = state.emu.get_cartridge().unwrap();

        let rom_size_str = format!("{} kiB", cart.get_rom_size() / 1024);
        let ram_size_str = format!("{} kiB", cart.get_ram_size() / 1024);
        let requires_cgb = matches!(cart.get_cgb_support(), GameBoyColorSupport::Required);

        self.data_list.clear();
        self.data_list.add_text("Title",                    cart.get_title());
        self.data_list.add_text("Manufacturer",             cart.get_manufacturer_code());
        self.data_list.add_text("Licensee",                 cart.get_licensee_code().to_string());
        self.data_list.add_text("MBC",                      cart.get_mbc().to_string());
        self.data_list.add_text("ROM size",                 rom_size_str);
        self.data_list.add_text("RAM size",                 ram_size_str);
        self.data_list.add_bool("Battery",                  cart.has_battery());
        self.data_list.add_bool("Timer",                    cart.has_timer());
        self.data_list.add_bool("Rumble",                   cart.has_rumble());
        self.data_list.add_bool("Super GameBoy Support",    cart.supports_sgb());
        self.data_list.add_bool("GameBoy Color Support",    cart.supports_cgb());
        self.data_list.add_bool("GameBoy Color Required",   requires_cgb);
    }
}

