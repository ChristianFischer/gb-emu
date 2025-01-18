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

use eframe::epaint::{Color32, Stroke};
use egui::scroll_area::ScrollBarVisibility;
use egui::{pos2, vec2, Grid, Pos2, Rect, ScrollArea, Sense, Ui, Widget};

use libgemi::core::mmu::locations::MEMORY_LOCATION_VRAM_BEGIN;
use libgemi::core::ppu::flags::LcdControlFlag;
use libgemi::core::ppu::graphic_data::{TileMap, TileSet};
use libgemi::core::ppu::ppu::{TILE_ATTR_BIT_H_FLIP, TILE_ATTR_BIT_VRAM_BANK, TILE_ATTR_BIT_V_FLIP};
use libgemi::core::utils::get_bit;
use libgemi::GameBoy;

use crate::event::UiEvent;
use crate::highlight::test_selection;
use crate::selection::{Kind, Selected};
use crate::state::{EmulatorState, UiStates};
use crate::ui::draw_tile::DrawTile;
use crate::ui::style::GemiStyle;
use crate::views::View;

const TILE_ROWS: usize      = 32;
const TILE_COLS: usize      = 32;
const TILE_WIDTH: usize     =  8;
const TILE_HEIGHT: usize    =  8;
const GAP: usize            =  1;
const DEFAULT_SCALE: usize  =  5;


#[derive(serde::Serialize, serde::Deserialize)]
pub struct TileMapView {
    tilemap: TileMap,

    tile_selected: Option<(bool, usize)>,
}


impl TileMapView {
    pub fn new(tilemap: TileMap) -> Self {
        Self {
            tilemap,
            tile_selected: None,
        }
    }
}


impl View for TileMapView {
    fn title(&self, _state: &mut EmulatorState) -> &str {
        match self.tilemap {
            TileMap::H9800 => "TileMap #0",
            TileMap::H9C00 => "TileMap #1",
        }
    }


    fn ui(&mut self, state: &mut EmulatorState, ui: &mut Ui) {
        match state.emu.get_gameboy() {
            None => {}

            Some(gb) => {
                self.render_tilemap(ui, gb, &mut state.ui);
            }
        }
    }


    fn get_current_selection(&self) -> Option<Selected> {
        self.tile_selected.map(|(tilemap_bit, index)| Selected::Tile(tilemap_bit, index))
    }


    fn handle_ui_event(&mut self, event: &UiEvent) {
        match event {
            UiEvent::SelectionChanged(Kind::Focus, Some(Selected::Tile(tilemap_bit, tile_index))) => {
                self.tile_selected = Some((*tilemap_bit, *tile_index));
            },

            _ => { }
        }
    }
}


impl TileMapView {
    fn render_tilemap(&self, ui: &mut Ui, emu: &GameBoy, ui_states: &mut UiStates) {
        let scale = DEFAULT_SCALE as f32;

        // compute the size of a tile how it will be displayed
        let tile_display_size = vec2(
            (TILE_WIDTH  as f32) * scale,
            (TILE_HEIGHT as f32) * scale
        );

        ScrollArea::new([true, true])
                .auto_shrink([false, false])
                .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
                .drag_to_scroll(false)
                .show(ui, |ui| {
                    Grid::new("tilemap_grid")
                            .num_columns(TILE_COLS)
                            .spacing([GAP as f32, GAP as f32])
                            .max_col_width(tile_display_size.x)
                            .min_row_height(tile_display_size.y)
                            .show(ui, |ui| {
                                let origin = ui.cursor().left_top();
                                self.render_tilemap_grid(ui, emu, ui_states);
                                self.render_tilemap_highlights(ui, origin, emu, ui_states);
                            })
                    ;
                })
        ;
    }


    /// Renders the whole tilemap as a 32x32 grid.
    fn render_tilemap_grid(&self, ui: &mut Ui, emu: &GameBoy, ui_states: &mut UiStates) {
        let ppu     = emu.get_ppu();
        let vram0   = ppu.get_vram(0);
        let tileset = TileSet::by_select_bit(ppu.check_lcdc(LcdControlFlag::TileDataSelect));

        for tile_row in 0..TILE_ROWS {
            for tile_column in 0..TILE_COLS {
                let tilemap_field_index       = tile_row * TILE_COLS + tile_column;
                let tilemap_field_address     = self.tilemap.base_address() as usize + tilemap_field_index;
                let tilemap_field_vram_offset = tilemap_field_address - MEMORY_LOCATION_VRAM_BEGIN as usize;
                let tile_number               = vram0[tilemap_field_vram_offset];
                let tile_image_index          = tileset.get_tile_image_index(tile_number);

                let draw_tile = if emu.get_config().is_gbc_enabled() {
                    let vram1            = ppu.get_vram(1);
                    let tile_attributes  = vram1[tilemap_field_vram_offset];
                    let tile_image_bank  = get_bit(tile_attributes, TILE_ATTR_BIT_VRAM_BANK) as u8;
                    let tile_image       = ppu.get_sprite_image(tile_image_index, tile_image_bank);
                    let palette_index    = (tile_attributes & 0b0000_0111) as usize;
                    let palette          = ppu.get_palettes().gbc_background_palette.get()[palette_index];

                    DrawTile::from(tile_image)
                            .set_tilemap_field_attributes(tile_attributes)
                            .set_palette_gbc(palette)
                }
                else {
                    let tile_image       = ppu.get_sprite_image(tile_image_index, 0);
                    let palette          = &ppu.get_palettes().bgp;

                    DrawTile::from(tile_image)
                            .set_palette_dmg(*palette)
                };

                let response = draw_tile
                        .scale(DEFAULT_SCALE as f32)
                        .sense(Sense::click())
                        .ui(ui)
                ;

                // handle hover
                ui_states.hover.set(Selected::Tile(self.tilemap.to_select_bit(), tilemap_field_index), response.hovered());

                // handle click
                if response.clicked() {
                    ui_states.focus.toggle(Selected::Tile(self.tilemap.to_select_bit(), tilemap_field_index));
                }

                // tooltip
                response.on_hover_ui(|ui| {
                    Grid::new("tooltip")
                            .num_columns(2)
                            .show(ui, |ui| {
                                let image_address = MEMORY_LOCATION_VRAM_BEGIN + (tile_image_index as u16 * 16);

                                // TileMap field
                                ui.label(GemiStyle::CAPTION.rich_text("TileMap field"));
                                ui.end_row();

                                ui.label("position");
                                ui.label(GemiStyle::MONOSPACE.rich_text(format!("{tile_column} : {tile_row}")));
                                ui.end_row();

                                ui.label("index");
                                ui.label(tilemap_field_index.to_string());
                                ui.end_row();

                                ui.label("address");
                                ui.label(GemiStyle::ADDRESS.rich_text(format!("0x{tilemap_field_address:x}")));
                                ui.end_row();

                                ui.end_row();

                                // Image
                                ui.label(GemiStyle::CAPTION.rich_text("Image")); // bold
                                ui.end_row();

                                ui.label("number");
                                ui.label(format!("{tile_number}"));
                                ui.end_row();

                                ui.label("tileset");
                                ui.label(GemiStyle::ADDRESS.rich_text(match tileset {
                                    TileSet::H8000 => "0x8000",
                                    TileSet::H8800 => "0x8800",
                                }));
                                ui.end_row();

                                ui.label("address");
                                ui.label(GemiStyle::ADDRESS.rich_text(format!("0x{image_address:x}")));
                                ui.end_row();

                                // display tooltip
                                if emu.get_config().is_gbc_enabled() {
                                    let vram1            = ppu.get_vram(1);
                                    let tile_attributes  = vram1[tilemap_field_vram_offset];
                                    let tile_image_bank  = get_bit(tile_attributes, TILE_ATTR_BIT_VRAM_BANK) as u8;
                                    let palette_index    = (tile_attributes & 0b0000_0111) as usize;
                                    let mut flip_x       = get_bit(tile_attributes, TILE_ATTR_BIT_H_FLIP);
                                    let mut flip_y       = get_bit(tile_attributes, TILE_ATTR_BIT_V_FLIP);

                                    ui.label("bank");
                                    ui.label(format!("#{tile_image_bank}"));
                                    ui.end_row();

                                    ui.end_row();

                                    // GBC Attribtues
                                    ui.label(GemiStyle::CAPTION.rich_text("Attributes")); // bold
                                    ui.end_row();

                                    ui.label("palette");
                                    ui.label(palette_index.to_string());
                                    ui.end_row();

                                    ui.label("flip x");
                                    ui.checkbox(&mut flip_x, "");
                                    ui.end_row();

                                    ui.label("flip y");
                                    ui.checkbox(&mut flip_y, "");
                                    ui.end_row();
                                }
                            })
                    ;
                });
            }

            ui.end_row();
        }
    }


    /// Renders an overlay for each selected tile on the tilemap.
    fn render_tilemap_highlights(&self, ui: &mut Ui, origin: Pos2, emu: &GameBoy, ui_states: &mut UiStates) {
        let scale = DEFAULT_SCALE as f32;

        // compute the size of a tile how it will be displayed
        let tile_display_size = vec2(
            (TILE_WIDTH  as f32) * scale,
            (TILE_HEIGHT as f32) * scale
        );

        for tile_row in 0..TILE_ROWS {
            for tile_column in 0..TILE_COLS {
                let tile_index = tile_row * TILE_COLS + tile_column;

                let highlight_state = test_selection(Selected::Tile(
                        self.tilemap.to_select_bit(),
                        tile_index
                ))
                        .of_view(self)
                        .compare_with_ui_states(ui_states, emu)
                ;

                let tile_bounds = Rect::from_min_size(
                    pos2(
                        origin.x + (tile_column * (tile_display_size.x as usize + GAP)) as f32,
                        origin.y + (tile_row    * (tile_display_size.y as usize + GAP)) as f32
                    ),
                    tile_display_size
                );

                // draw background if selected
                if let Some(highlight_state) = highlight_state {
                    let highlight_color = highlight_state.get_color(ui);
                    Self::draw_highlight(ui, tile_bounds, highlight_color);
                }
            }
        }
    }


    /// Draw a highlight for the current sprite.
    fn draw_highlight(ui: &mut Ui, tile_bounds: Rect, color: Color32) {
        ui.painter().rect_stroke(
                tile_bounds.expand(1.0),
                3.0,
                Stroke::new(2.0, color)
        );
    }
}
