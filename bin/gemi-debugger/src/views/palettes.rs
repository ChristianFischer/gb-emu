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

use egui::{Color32, Grid, Response, Sense, TextStyle, Ui, Vec2};

use libgemi::core::ppu::graphic_data::{Color, DmgDisplayPalette, DmgPalette, GbcPaletteData, SpritePixelValue};
use libgemi::GameBoy;

use crate::state::EmulatorState;
use crate::ui::style::GemiStyle;
use crate::views::View;


#[derive(serde::Serialize, serde::Deserialize)]
pub struct PaletteView {
    #[serde(skip)]
    rt: PalettesRuntimeData,
}


/// Internal data of the [PaletteView], which does not get serialized.
struct PalettesRuntimeData {
    dmg_display_palette: DmgDisplayPalette,
    pixels: [SpritePixelValue; 4],
    is_paused: bool,
}


impl PaletteView {
    /// Creates a new [`PaletteView`] object.
    pub fn new() -> Self {
        Self {
            rt: Default::default(),
        }
    }
}


impl View for PaletteView {
    fn title(&self, _state: &mut EmulatorState) -> &str {
        "Palettes"
    }


    fn ui(&mut self, state: &mut EmulatorState, ui: &mut Ui) {
        self.rt.is_paused = state.ui.is_paused();

        if let Some(gb) = state.emu.get_gameboy_mut() {
            let is_gbc = gb.get_config().is_gbc_enabled();

            // display all palettes within a grid
            Grid::new("palettes")
                    .num_columns(5)
                    .min_col_width(1.0)
                    .striped(true)
                    .show(ui, |ui| {
                        self.display_dmg_palettes(ui, gb);

                        if is_gbc {
                            self.display_gbc_palettes(ui, gb);
                        }
                    }
            );
        }
    }
}


impl PaletteView {
    /// Display a list of all non-color palettes within the current grid.
    fn display_dmg_palettes(&self, ui: &mut Ui, emu: &GameBoy) {
        let ppu      = emu.get_ppu();
        let palettes = ppu.get_palettes();

        self.display_dmg_palette_entry(ui, "BGP",  &palettes.bgp);
        self.display_dmg_palette_entry(ui, "OBP0", &palettes.obp[0]);
        self.display_dmg_palette_entry(ui, "OBP1", &palettes.obp[1]);
    }


    /// Display a single non-color palette together with its name.
    fn display_dmg_palette_entry(&self, ui: &mut Ui, name: &str, palette: &DmgPalette) {
        // The palette name
        ui.label(GemiStyle::ADDRESS.rich_text(name));

        // display the color for each possible pixel value
        for pixel in &self.rt.pixels {
            let palette_color = palette.get_color(pixel);
            let color         = self.rt.dmg_display_palette.get_color(&palette_color);

            self.display_color_box(ui, color);
        }

        ui.end_row();
    }



    /// Display all palettes of the GameBoy Color.
    fn display_gbc_palettes(&self, ui: &mut Ui, emu: &GameBoy) {
        let ppu      = emu.get_ppu();
        let palettes = ppu.get_palettes();
        let bg       = palettes.gbc_background_palette.get();
        let obj      = palettes.gbc_object_palette.get();

        ui.end_row();

        // background palettes 0-7
        for i in 0..8 {
            self.display_gbc_palette_entry(ui, &format!("BCP #{i}"), &bg[i]);
        }

        ui.end_row();

        // object palettes 0-7
        for i in 0..8 {
            self.display_gbc_palette_entry(ui, &format!("OCP #{i}"), &obj[i]);
        }
    }


    /// Display a single color palette together with its name.
    fn display_gbc_palette_entry(&self, ui: &mut Ui, name: &str, palette: &GbcPaletteData) {
        // The palette name
        ui.label(GemiStyle::ADDRESS.rich_text(name));

        // display the color for each possible pixel value
        for pixel in &self.rt.pixels {
            let color = palette.get_color(&pixel);
            self.display_color_box(ui, &color);
        }

        ui.end_row();
    }



    /// Display a single color within a small box.
    fn display_color_box(&self, ui: &mut Ui, color: &Color) -> Response {
        let text_height = ui.text_style_height(&TextStyle::Monospace);

        let color32 = Color32::from_rgba_unmultiplied(
            color.r,
            color.g,
            color.b,
            color.a
        );

        let (rect, response) = ui.allocate_exact_size(Vec2::splat(text_height), Sense::hover());
        ui.painter().rect(rect, 0.0, color32, ui.visuals().window_stroke);

        response
    }
}


impl Default for PalettesRuntimeData {
    fn default() -> Self {
        Self {
            dmg_display_palette: DmgDisplayPalette::new_green(),

            pixels: [
                SpritePixelValue::new(0),
                SpritePixelValue::new(1),
                SpritePixelValue::new(2),
                SpritePixelValue::new(3),
            ],

            is_paused: true,
        }
    }
}
