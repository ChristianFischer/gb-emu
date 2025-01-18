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

use egui::{Color32, Rgba, Ui};

use libgemi::core::mmu::locations::MEMORY_LOCATION_VRAM_BEGIN;
use libgemi::core::ppu::flags::LcdControlFlag;
use libgemi::core::ppu::graphic_data::{TileMap, TileSet};
use libgemi::core::ppu::ppu::TILE_ATTR_BIT_VRAM_BANK;
use libgemi::core::utils::get_bit;
use libgemi::GameBoy;

use crate::selection::{Kind, Selected};
use crate::state::UiStates;
use crate::ui::style::GemiStyle;
use crate::views::View;

/// The state how an item gets highlighted, based on the selections stored
/// in the applications [UiStates].
pub enum HighlightState {
    /// The item is directly hovered.
    Hovered,

    /// The item is directly selected.
    Selected,

    /// The item gets highlighted by another, related item, which is
    /// currently being hovered.
    FocusHovered,

    /// The item gets highlighted by another, related item, which is
    /// currently being selected as the global focus item.
    Focus,
}


/// A helper object to compute the [HighlightState] of a single 
/// selection, represented by a [Selected] object.
/// To begin the test, see [test_selection].
pub struct HighlightTesting {
    currently_selected: Option<Selected>,
    possibly_selected: Selected,
}


/// Begin testing the [HighlightState] of a [Selected] object.
/// This returns a configured [HighlightTesting] object, which
/// implements the testing operation.
pub fn test_selection(possibly_selected: Selected) -> HighlightTesting {
    HighlightTesting {
        currently_selected: None,
        possibly_selected,
    }
}


impl HighlightTesting {
    /// Compares the [Selected] object in question with the current
    /// selection of the given [View].
    /// When matching, the selection will count as [HighlightState::Selected].
    pub fn of_view(mut self, view: &impl View) -> Self {
        self.currently_selected = view.get_current_selection();
        self
    }


    /// Checks the highlight state of the [Selected] object.
    /// The item can be either selected or hovered directly or indirectly,
    /// by the selection of a related item.
    pub fn compare_with_ui_states(self, ui_states: &UiStates, gb: &GameBoy) -> Option<HighlightState> {
        // directly hovered
        if ui_states.hover.is_selected(&self.possibly_selected) {
            return Some(HighlightState::Hovered);
        }

        // directly selected as the focus item
        if ui_states.focus.is_selected(&self.possibly_selected) {
            return Some(HighlightState::Selected);
        }

        // directly selected within the current view
        if let Some(currently_selected) = self.currently_selected {
            if currently_selected == self.possibly_selected {
                return Some(HighlightState::Selected);
            }
        }

        // checks for any other selections, if they're related to `possibly_selected`
        for selection in [&ui_states.hover, &ui_states.focus] {
            if let Some(currently_selected) = selection.get() {
                if
                        Self::is_related(gb, &self.possibly_selected, currently_selected)
                    ||  Self::is_related(gb, currently_selected, &self.possibly_selected)
                {
                    // return a highlight state depending on whether it is
                    // currently selected or hovered
                    return Some(
                        match *selection.get_kind() {
                            Kind::Focus => HighlightState::Focus,
                            Kind::Hover => HighlightState::FocusHovered,
                        }
                    );
                }
            }
        }

        None
    }


    /// Checks whether two selections are related to each other.
    /// For example, an entry in the OAM table is related to the sprite image,
    /// it is referred to and vice versa.
    fn is_related(gb: &GameBoy, a: &Selected, b: &Selected) -> bool {
        match (a, b) {
            // sprites highlighted by a selected oam entry
            (Selected::Sprite(bank_index, sprite_index), Selected::OamEntry(oam_index)) => {
                let oam   = gb.get_ppu().get_oam();
                let entry = &oam[*oam_index];

                // when on GBC, also check the bank index
                if gb.get_config().is_gbc_enabled() {
                    if entry.get_gbc_vram_bank() != *bank_index {
                        return false;
                    }
                }

                if entry.tile as usize == *sprite_index {
                    return true;
                }
            }

            // sprites highlighted by a selected tile
            (Selected::Sprite(bank_index, sprite_index), Selected::Tile(tilemap_bit, tile_index)) => {
                let ppu     = gb.get_ppu();
                let vram0   = ppu.get_vram(0);
                let tilemap = TileMap::by_select_bit(*tilemap_bit);
                let tileset = TileSet::by_select_bit(ppu.check_lcdc(LcdControlFlag::TileDataSelect));

                let tilemap_field_address     = tilemap.base_address() as usize + tile_index;
                let tilemap_field_vram_offset = tilemap_field_address - MEMORY_LOCATION_VRAM_BEGIN as usize;
                let tile_number               = vram0[tilemap_field_vram_offset];
                let tile_image_index          = tileset.get_tile_image_index(tile_number);

                // when on GBC, also check the bank index
                if gb.get_config().is_gbc_enabled() {
                    let vram1           = ppu.get_vram(1);
                    let tile_attributes = vram1[tilemap_field_vram_offset];
                    let tile_bank_index = get_bit(tile_attributes, TILE_ATTR_BIT_VRAM_BANK) as u8;

                    if tile_bank_index != *bank_index {
                        return false;
                    }
                }

                if tile_image_index == *sprite_index {
                    return true;
                }
            }

            _ => { }
        }

        return false;
    }
}


impl HighlightState {
    /// Receive a color to highlight the current item.
    /// This color is expected to be used as a foreground color for example
    /// to draw a border around an item.
    pub fn get_color(&self, ui: &Ui) -> Color32 {
        let gray = Color32::from_rgba_unmultiplied(128, 128, 128, 255);

        match *self {
            HighlightState::Hovered         => Self::blend_colors(ui.visuals().selection.bg_fill, gray, 0.5),
            HighlightState::Selected        => ui.visuals().selection.bg_fill,
            HighlightState::FocusHovered    => GemiStyle::BACKGROUND_HIGHLIGHT_HOVER,
            HighlightState::Focus           => GemiStyle::BACKGROUND_HIGHLIGHT_SELECTION,
        }
    }


    /// Receive a color to highlight the current item.
    /// This color is expected to be used as a background behind an item.
    pub fn get_background_color(&self, ui: &Ui) -> Color32 {
        match *self {
            HighlightState::Hovered         => ui.visuals().widgets.hovered.bg_fill,
            HighlightState::Selected        => ui.visuals().selection.bg_fill,
            HighlightState::FocusHovered    => GemiStyle::BACKGROUND_HIGHLIGHT_HOVER,
            HighlightState::Focus           => GemiStyle::BACKGROUND_HIGHLIGHT_SELECTION,
        }
    }


    fn blend_colors(a: Color32, b: Color32, f: f32) -> Color32 {
        let rgba1 = Rgba::from(a).multiply(f);
        let rgba2 = Rgba::from(b).multiply(1.0 - f);

        (rgba1 + rgba2).into()
    }
}
