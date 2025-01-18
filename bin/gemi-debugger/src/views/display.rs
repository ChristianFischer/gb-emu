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

use std::cmp::min;
use std::ops::{Add, Div, Mul, Sub};

use eframe::emath::Rect;
use eframe::epaint::textures::TextureOptions;
use eframe::epaint::{ColorImage, Stroke};
use egui::{vec2, Color32, Context, Grid, Image, Pos2, Sense, TextureHandle, Ui, Vec2, Widget};

use libgemi::core::emulator_core::Clock;
use libgemi::core::ppu::flags::LcdControlFlag;
use libgemi::core::ppu::graphic_data::Sprite;
use libgemi::core::ppu::ppu::{SCREEN_H, SCREEN_W};
use libgemi::GameBoy;

use crate::highlight::test_selection;
use crate::selection::Selected;
use crate::state::{EmulatorState, UiStates, UpdateMode};
use crate::views::View;


/// The main view to show the emulator's display.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EmulatorDisplayView {
    #[serde(skip)]
    rt: RuntimeData,
}


#[derive(Default)]
struct RuntimeData {
    display_image: Option<TextureHandle>,
    display_image_timestamp: Clock,

    current_draw_pos: ScreenPos,
    last_draw_pos: ScreenPos,
}


#[derive(Default, Copy, Clone, Eq, PartialEq)]
struct ScreenPos {
    pub x: usize,
    pub y: usize,
}


impl View for EmulatorDisplayView {
    fn title(&self, _state: &mut EmulatorState) -> &str {
        "Display"
    }


    fn ui(&mut self, state: &mut EmulatorState, ui: &mut Ui) {
        match state.emu.get_gameboy() {
            None => {}

            Some(gb) => {
                self.update_display_image(ui.ctx(), gb, &mut state.ui);
                self.render_display_image(ui, gb, &mut state.ui);
            }
        }
    }
}


impl EmulatorDisplayView {
    pub fn new() -> Self {
        Self {
            rt: Default::default(),
        }
    }


    /// Checks whether the cached display image is outdated and updates the image, if necessary.
    fn update_display_image(&mut self, ctx: &Context, gb: &GameBoy, ui_states: &mut UiStates) {
        if self.rt.display_image.is_none() || self.rt.display_image_timestamp != gb.get_total_cycles_processed() {
            let ppu    = &gb.get_ppu();
            let lcd    = ppu.get_lcd();
            let size   = [lcd.get_width() as _, lcd.get_height() as _];
            let pixels = lcd.get_pixels_as_slice();

            // get the position on the screen which was drawn recently
            let current_pos = ScreenPos {
                x: min(lcd.get_width() as usize,  ppu.get_current_line_pixel() as usize),
                y: min(lcd.get_height() as usize, ppu.get_current_line() as usize),
            };

            // check whether the draw cursor has moved
            if self.rt.current_draw_pos != current_pos {
                self.rt.last_draw_pos = self.rt.current_draw_pos;
                self.rt.current_draw_pos = current_pos;
            }

            // create a texture from the pixel data
            let mut image = ColorImage::from_rgba_unmultiplied(size, pixels);

            // when the emulator is paused or in stepped-mode, highlight
            // the pixels drawn since the last step
            if
                    Self::display_highlight_in_mode(&ui_states.get_update_mode())
                &&  self.rt.last_draw_pos != self.rt.current_draw_pos
            {
                self.apply_highlight(&mut image);
            }

            let texture = ctx.load_texture("display", image, TextureOptions::NEAREST);

            self.rt.display_image           = Some(texture);
            self.rt.display_image_timestamp = gb.get_total_cycles_processed();
        }
    }


    /// Render the display image of the currently running emulator.
    fn render_display_image(&mut self, ui: &mut Ui, emu: &GameBoy, ui_states: &mut UiStates) {
        if let Some(texture) = &self.rt.display_image {
            let texture_size = texture.size_vec2();
            let available_size = ui.available_size();

            // compute the scale factor to fit the image into the available space
            // (but only whole numbers and not smaller than 1)
            let scale = f32::max(
                1.0,
                f32::min(
                    available_size.x / texture_size.x,
                    available_size.y / texture_size.y
                )
            ).floor();

            // store the origin of the draw area
            let origin = ui.cursor().left_top();

            // render the texture
            Image::new(texture)
                    .fit_to_exact_size(texture_size * scale)
                    .ui(ui)
            ;

            self.handle_interactions(ui, ui_states, emu, origin, scale);

            self.render_selection_overlays(ui, ui_states, emu, origin, scale);
        }
    }


    /// Tells whether to highlight the recent drawn pixels in a certain [UpdateMode] or not.
    fn display_highlight_in_mode(mode: &UpdateMode) -> bool {
        match mode {
            UpdateMode::Paused | UpdateMode::Step => true,
            UpdateMode::Continuous => false,
        }
    }


    /// Updates the image to highlight the recently drawn pixels, as defined
    /// via `last_draw_pos` and `current_draw_pos`.
    fn apply_highlight(&self, image: &mut ColorImage) {
        let current_index = self.rt.current_draw_pos.index_on(image);
        let last_index    = self.rt.last_draw_pos.index_on(image);
        let end_index     = image.width() * image.height();

        // helper function to fade the image on a given position
        let mut fade_image_at = |index: usize| {
            let color = image.pixels[index];

            image.pixels[index] = Color32::from_rgba_unmultiplied(
                color.r(), color.g(), color.b(), 64
            );
        };

        if current_index >= last_index {
            for index in current_index..end_index {
                fade_image_at(index);
            }

            for index in 0..last_index {
                fade_image_at(index);
            }
        }
        else {
            for index in last_index..current_index {
                fade_image_at(index);
            }
        }
    }


    /// Handles interactions of the user with the UI.
    fn handle_interactions(&self, ui: &mut Ui, ui_states: &mut UiStates, emu: &GameBoy, origin: Pos2, scale: f32) {
        let mut oam_hit : Option<(usize, Sprite, Rect)> = None;

        let display_bounds = Rect::from_min_size(
                origin,
                Vec2::new(
                        SCREEN_W as f32, 
                        SCREEN_H as f32
                ) * scale
        );

        // listen for click and hover interactions
        let response = ui.interact(
                display_bounds, 
                ui.id().with(1), 
                Sense::click()
        );
        
        if response.hovered() {
            if let Some(hover_pos) = ui.input(|input| input.pointer.hover_pos()) {
                let ppu           = emu.get_ppu();
                let oam           = ppu.get_oam();
                let large_sprites = ppu.check_lcdc(LcdControlFlag::SpritesSize);
                let sprite_size   = if large_sprites { 16 } else { 8 };

                // transform the position of the mouse cursor into OAM position space
                let oam_pos = hover_pos
                        .sub(display_bounds.left_top())
                        .div(scale)
                        .add(vec2(8.0, 16.0))
                        .to_pos2()
                ;

                // test for all OAM entries
                for oam_index in 0..40 {
                    let oam_entry  = &oam[oam_index];
                    let oam_bounds = Rect::from_min_size(
                            Pos2::new(oam_entry.pos_x as f32, oam_entry.pos_y as f32),
                            Vec2::new(8.0, sprite_size as f32)
                    );

                    let hit = oam_bounds.contains(oam_pos);
                    ui_states.hover.set(Selected::OamEntry(oam_index), hit);

                    if hit {
                        if response.clicked() {
                            ui_states.focus.toggle(Selected::OamEntry(oam_index));
                        }

                        oam_hit = Some((oam_index, oam_entry.clone(), oam_bounds));
                    }
                }
            }

            // tooltip
            if let Some((oam_index, sprite, bounds)) = oam_hit {
                // interact on the OAM bounds itself, to get the tooltip
                // displayed close on the image itself
                ui.interact(
                    bounds
                            .translate(vec2(-8.0, -16.0))
                            .mul(scale)
                            .translate(origin.to_vec2()),
                    ui.id().with(2),
                    Sense::hover()
                )
                    .on_hover_ui(|ui| {
                        Grid::new("tooltip")
                                .num_columns(2)
                                .show(ui, |ui| {
                                    ui.label("position");
                                    ui.label(format!("{} : {}", sprite.pos_x, sprite.pos_y));
                                    ui.end_row();

                                    ui.label("oam");
                                    ui.label(oam_index.to_string());
                                    ui.end_row();

                                    ui.label("tile");
                                    ui.label(sprite.tile.to_string());
                                    ui.end_row();
                                })
                        ;
                    })
                ;
            }
        }
    }


    /// Render overlays on the display of the currently running emulator to
    /// highlight any currently selected sprites and tiles.
    fn render_selection_overlays(&self, ui: &mut Ui, ui_states: &mut UiStates, emu: &GameBoy, origin: Pos2, scale: f32) {
        let ppu           = emu.get_ppu();
        let oam           = ppu.get_oam();
        let large_sprites = ppu.check_lcdc(LcdControlFlag::SpritesSize);

        for oam_index in 0..40 {
            let entry = &oam[oam_index];

            // check whether this item is highlighted or not
            let highlight_state = test_selection(Selected::OamEntry(oam_index))
                    .of_view(self)
                    .compare_with_ui_states(ui_states, emu)
            ;

            // render highlight if selected
            if let Some(highlight_state) = highlight_state {
                let highlight_color = highlight_state.get_color(ui);

                Self::render_sprite_outline(
                        ui,
                        &entry,
                        origin,
                        highlight_color,
                        large_sprites,
                        scale,
                );
            }
        }
    }


    /// Draws an outline on the location where an entry from the OAM will
    /// be displayed.
    fn render_sprite_outline(ui: &mut Ui, sprite: &Sprite, origin: Pos2, color: Color32, large: bool, scale: f32) {
        let sprite_size = if large { 16 } else { 8 };

        let sprite_bounds = Rect::from_min_size(
            Pos2::new(
                origin.x + ((sprite.pos_x as f32 -  8.0) * scale),
                origin.y + ((sprite.pos_y as f32 - 16.0) * scale)
            ),
            Vec2::new(
                8.0 * scale,
                sprite_size as f32 * scale
            )
        );

        ui.painter().rect_stroke(
            sprite_bounds,
            2.0,
            Stroke::new(2.0, color)
        );
    }
}


impl ScreenPos {
    fn index_on(&self, image: &ColorImage) -> usize {
        let w = image.width();

        self.y * w + self.x
    }
}
