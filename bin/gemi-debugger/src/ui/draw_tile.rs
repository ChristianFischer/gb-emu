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


use std::mem::swap;

use egui::{Image, pos2, Rect, Response, Sense, Ui, Vec2, Widget};

use libgemi::core::ppu::graphic_data::{DmgPalette, GbcPaletteData, Sprite};
use libgemi::core::ppu::ppu::{TILE_ATTR_BIT_H_FLIP, TILE_ATTR_BIT_V_FLIP};
use libgemi::core::ppu::sprite_image::SpriteImage;
use libgemi::core::utils::get_bit;

use crate::ui::sprite_cache;
use crate::ui::sprite_cache::Palette;

/// A utility to draw tiles on the screen.
pub struct DrawTile {
    image: SpriteImage,
    palette: Palette,

    sense: Sense,

    scale: ScaleMode,

    flip_x: bool,
    flip_y: bool,
}


/// Private switch to determine how to scale an image.
enum ScaleMode {
    /// Scale the image's original size by a fixed factor.
    Scale(f32),

    /// Scale the image to fit a fixed size.
    Size(Vec2),
}


impl DrawTile {
    /// Scale the image by a fixed factor.
    /// The factor is applied to both axis keeping the aspect ratio.
    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = ScaleMode::Scale(scale);
        self
    }


    /// Scales the image to a fixed size.
    /// This may change the aspect ratio of the image.
    pub fn fit_to_exact_size(mut self, size: Vec2) -> Self {
        self.scale = ScaleMode::Size(size);
        self
    }


    /// Set whether the image should be flipped on X-axis.
    pub fn flip_x(mut self, flip: bool) -> Self {
        self.flip_x = flip;
        self
    }


    /// Set whether the image should be flipped on Y-axis.
    pub fn flip_y(mut self, flip: bool) -> Self {
        self.flip_y = flip;
        self
    }


    /// Applies the configuration stored in an OAM entry to this tile.
    pub fn apply_oam(self, oam: &Sprite) -> Self {
        self
                .flip_x(oam.is_flip_x())
                .flip_y(oam.is_flip_y())
    }


    /// Applies the configuration stored in the tile attributes of a tilemap.
    pub fn set_tilemap_field_attributes(self, attributes: u8) -> Self {
        self
                .flip_x(get_bit(attributes, TILE_ATTR_BIT_H_FLIP))
                .flip_y(get_bit(attributes, TILE_ATTR_BIT_V_FLIP))
    }


    /// Set the palette used for rendering to a DMG Palette.
    pub fn set_palette_dmg(mut self, palette: DmgPalette) -> Self {
        self.palette = Palette::Dmg(palette);
        self
    }


    /// Set the palette used for rendering to a GameBoy Color Palette.
    pub fn set_palette_gbc(mut self, palette: GbcPaletteData) -> Self {
        self.palette = Palette::Gbc(palette);
        self
    }


    /// Set the [Sense] of this widget in order to respond to clicks and/or drags.
    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = sense;
        self
    }

}


impl Widget for DrawTile {
    fn ui(self, ui: &mut Ui) -> Response {
        let texture = sprite_cache::get_texture_for(ui, &self.image, self.palette);
        let texture_size = texture.size();

        // configure scaling
        let display_size = match self.scale {
            ScaleMode::Scale(scale) => Vec2::new(
                texture_size[0] as f32 * scale,
                texture_size[1] as f32 * scale
            ),

            ScaleMode::Size(size) => size,
        };

        // creating the actual widget
        let mut image = Image::new(&texture)
                .fit_to_exact_size(display_size)
                .sense(self.sense)
        ;

        // configure image flipping
        if self.flip_x || self.flip_y {
            let mut uv = Rect::from_min_max(
                pos2(0.0, 0.0),
                pos2(1.0, 1.0)
            );

            if self.flip_x {
                swap(&mut uv.min.x, &mut uv.max.x)
            }

            if self.flip_y {
                swap(&mut uv.min.y, &mut uv.max.y)
            }

            image = image
                    // rotate is used to force the image being rendered via Mesh
                    .rotate(0.0, Vec2::splat(0.5))
                    .uv(uv)
            ;
        }

        image.ui(ui)
    }
}


impl From<SpriteImage> for DrawTile {
    fn from(value: SpriteImage) -> Self {
        Self {
            image:      value,
            palette:    Palette::NoPalette,
            sense:      Sense::hover(),
            scale:      ScaleMode::Scale(1.0),
            flip_x:     false,
            flip_y:     false,
        }
    }
}
