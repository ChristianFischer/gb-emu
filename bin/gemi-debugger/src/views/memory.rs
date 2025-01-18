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

use std::ops::Range;

use egui::Ui;

use libgemi::core::mmu::locations::{MEMORY_LOCATION_OAM_BEGIN, MEMORY_LOCATION_SPRITES_BEGIN};
use libgemi::core::ppu::graphic_data::TileMap;
use libgemi::GameBoy;

use crate::event::UiEvent;
use crate::event::UiEvent::SelectionChanged;
use crate::selection::{Kind, Selected};
use crate::state::EmulatorState;
use crate::ui::memory_editor::MemoryEditor;
use crate::ui::style::GemiStyle;
use crate::views::View;


/// A view to display the emulator's memory.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MemoryView {
    memory_editor: MemoryEditor<GameBoy>,
}


impl MemoryView {
    /// Creates a new [`MemoryView`] object.
    pub fn new() -> Self {
        let mut memory_editor = MemoryEditor::new();
        
        // prepare two types of highlights, one for selections one for hover
        memory_editor.add_highlight_range(GemiStyle::BACKGROUND_HIGHLIGHT_SELECTION);
        memory_editor.add_highlight_range(GemiStyle::BACKGROUND_HIGHLIGHT_HOVER);

        Self {
            memory_editor,
        }
    }
}


impl View for MemoryView {
    fn title(&self, _state: &mut EmulatorState) -> &str {
        "Memory"
    }


    fn ui(&mut self, state: &mut EmulatorState, ui: &mut Ui) {
        self.display_memory_editor(state, ui);
    }


    fn handle_ui_event(&mut self, event: &UiEvent) {
        self.on_ui_event(event);
    }


    fn on_emulator_loaded(&mut self, state: &mut EmulatorState) {
        self.refresh_memory_map(state);
    }
}


impl MemoryView {
    fn on_ui_event(&mut self, event: &UiEvent) {
        use Selected::*;
        use Kind::*;

        let get_highlight_index = |kind: &Kind| -> usize {
            match kind {
                Focus => 0,
                Hover => 1,
            }
        };

        // compute the address range of a selected sprite
        let get_sprite_address_range = |sprite_index: usize| -> Range<usize> {
            let address_begin = (sprite_index * 16) + MEMORY_LOCATION_SPRITES_BEGIN as usize;
            let address_end   = address_begin + 16;
            address_begin .. address_end
        };

        // compute the address range of a selection within the OAM list
        let get_oam_address_range = |oam_index: usize| -> Range<usize> {
            let address_begin = (oam_index * 4) + MEMORY_LOCATION_OAM_BEGIN as usize;
            let address_end   = address_begin + 4;
            address_begin .. address_end
        };

        // compute the address range of a selected tile on a tilemap
        let get_tile_address_range = |tilemap_bit: bool, tile_index: usize| -> Range<usize> {
            let tilemap = TileMap::by_select_bit(tilemap_bit);
            let address = tilemap.base_address() as usize + tile_index;

            address .. (address + 1)
        };

        match event {
            SelectionChanged(kind, Some(Sprite(_, sprite_index))) => {
                self.memory_editor.set_highlighted_range(
                        get_highlight_index(kind),
                        get_sprite_address_range(*sprite_index)
                );
            }

            SelectionChanged(kind, Some(OamEntry(oam_index))) => {
                self.memory_editor.set_highlighted_range(
                        get_highlight_index(kind),
                        get_oam_address_range(*oam_index)
                );
            }

            SelectionChanged(kind, Some(Tile(tilemap_bit, tile_index))) => {
                self.memory_editor.set_highlighted_range(
                        get_highlight_index(kind),
                        get_tile_address_range(*tilemap_bit, *tile_index)
                );
            }

            SelectionChanged(kind, Some(Instruction(range))) => {
                self.memory_editor.set_highlighted_range(
                        get_highlight_index(kind),
                        range.start as usize .. range.end as usize,
                );
            }

            SelectionChanged(kind, None) => {
                self.memory_editor.clear_highlight(
                        get_highlight_index(kind)
                );
            }
        }
    }


    /// Refreshes the memory map of the editor.
    fn refresh_memory_map(&mut self, state: &mut EmulatorState) {
        let has_cartridge_ram = if let Some(cart) = state.emu.get_cartridge() {
            cart.has_ram()
        }
        else {
            false
        };

        self.memory_editor.clear_memory_areas();
        self.memory_editor.add_memory_area("ROM Bank #0",      0x0000..=0x3fff, false);
        self.memory_editor.add_memory_area("ROM Bank #1",      0x4000..=0x7fff, false);
        self.memory_editor.add_memory_area("VRAM",             0x8000..=0x9fff, true);
        self.memory_editor.add_memory_area("Cartridge RAM",    0xa000..=0xbfff, has_cartridge_ram);
        self.memory_editor.add_memory_area("WRAM Bank #0",     0xc000..=0xcfff, true);
        self.memory_editor.add_memory_area("WRAM Bank #1",     0xd000..=0xdfff, true);
        self.memory_editor.add_memory_area("Mirror RAM",       0xe000..=0xfdff, true);
        self.memory_editor.add_memory_area("OAM",              0xfe00..=0xfe9f, true);
        self.memory_editor.add_memory_area("<unusable>",       0xfea0..=0xfeff, false);
        self.memory_editor.add_memory_area("IO",               0xff00..=0xff79, false);
        self.memory_editor.add_memory_area("HRAM",             0xff80..=0xfffe, true);
    }


    /// Display the memory editor.
    fn display_memory_editor(&mut self, state: &mut EmulatorState, ui: &mut Ui) {
        let is_paused = state.ui.is_paused();

        if let Some(gb) = state.emu.get_gameboy_mut() {
            // allow editing while paused
            self.memory_editor.set_editable(is_paused);

            self.memory_editor.show(
                ui,
                gb,

                // reading memory
                |emu, address| {
                    match address {
                        // we can only read an address, if the address is in a valid 16 bit range
                        0x0000 ..= 0xffff => {
                            let value = emu.get_mmu().read_u8(address as u16);
                            Some(value)
                        }

                        _ => None,
                    }
                },

                // writing memory
                |emu, address, value| {
                    // only some address ranges are writable
                    match address {
                        /* VRAM",             */    0x8000..=0x9FFF
                        /* Cartridge RAM",    */  | 0xA000..=0xBFFF
                        /* WRAM Bank #0",     */  | 0xC000..=0xCFFF
                        /* WRAM Bank #1",     */  | 0xD000..=0xDFFF
                        /* OAM",              */  | 0xFE00..=0xFE9F
                        /* HRAM",             */  | 0xFF80..=0xFFFE => {
                            emu.get_mmu_mut().write_u8(address as u16, value);
                        }

                        _ => { }
                    }
                }
            );
        }
    }
}
