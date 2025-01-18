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

use std::collections::HashMap;
use std::sync::Mutex;

use egui::{ColorImage, TextureOptions, Ui};
use lazy_static::lazy_static;

use libgemi::core::ppu::graphic_data::{DmgDisplayPalette, DmgPalette, GbcPaletteData};
use libgemi::core::ppu::sprite_image::SpriteImage;

/// The maximum number of sprites to cache.
/// If this number is exceeded, the oldest entries will be removed.
pub const MAX_SPRITE_CACHE_SIZE : usize = 4096;


lazy_static! {
    static ref INSTANCE:            Mutex<SpriteCache>       = Mutex::new(SpriteCache::new());
    static ref DMG_DISPLAY_PALETTE: Mutex<DmgDisplayPalette> = Mutex::new(DmgDisplayPalette::new_green());
}


/// The data struct holding all cached data.
pub struct SpriteCache {
    map: HashMap<SpriteImage, SpriteWithPaletteCacheEntry>,
}


/// A single entry in the sprite cache to store data of a single image,
/// which includes variations due to different palettes.
struct SpriteWithPaletteCacheEntry {
    /// A map holding variations using various palettes.
    map: HashMap<Palette, SpriteCacheEntry>,

    /// This counts the time since the last usage of this entry.
    /// The value is incremented each frame and reset to zero on each usage.
    time_since_used: usize,
}


/// A single entry in the sprite cache to store the data assigned
/// to a specific image and palette.
struct SpriteCacheEntry {
    /// The texture being cached.
    texture: egui::TextureHandle,
}


/// An enum to switch between DMG and GBC palette types.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Palette {
    NoPalette,

    /// Classic GameBoy palette.
    Dmg(DmgPalette),

    /// GameBoy Color palette.
    Gbc(GbcPaletteData),
}


/// Returns a texture for the given sprite image.
/// The texture will be created once and cached, so any subsequent calls
/// will return the same texture.
pub fn get_texture_for(ui: &mut Ui, sprite_image: &SpriteImage, palette: Palette) -> egui::TextureHandle {
    let mut instance = INSTANCE.lock().unwrap();

    // get the image entry
    let image_entry = if let Some(entry) = instance.map.get_mut(sprite_image) {
        // reset the age of the entry, once it is used
        entry.time_since_used = 0;
        entry
    }
    else {
        let entry = instance.map.entry(sprite_image.clone()).or_insert_with(
            || SpriteWithPaletteCacheEntry {
                map: HashMap::new(),
                time_since_used: 0,
            }
        );

        entry
    };

    // get the entry for the sprite palette
    if let Some(entry) = image_entry.map.get(&palette) {
        entry.texture.clone()
    }
    else {
        let id      = sprite_image.to_hex_string();
        let pixels  = palette.to_rgba(sprite_image);
        let image   = ColorImage::from_rgba_unmultiplied([8, 8], &pixels);
        let texture = ui.ctx().load_texture(id, image, TextureOptions::NEAREST);

        image_entry.map.insert(
            palette.clone(),
            SpriteCacheEntry {
                texture: texture.clone(),
            }
        );

        texture
    }
}


/// To be called once per frame to update the age of each entry
/// and to remove old entries, if the cache size exceeds the maximum.
pub fn on_frame() {
    let mut instance = INSTANCE.lock().unwrap();

    // if we did exceed the maximum cache size, remove all entries, which
    // have not been used since some time.
    if instance.map.len() >= MAX_SPRITE_CACHE_SIZE {
        instance.map.retain(|_, entry| entry.time_since_used == 0);
    }

    // increase the age of all entries
    for entry in instance.map.values_mut() {
        entry.time_since_used += 1;
    }
}


impl SpriteCache {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}


impl Palette {
    /// Converts an image into RGBA data, using the current palette to convert
    /// pixel values into RGBA.
    fn to_rgba(&self, image: &SpriteImage) -> [u8; 256] {
        match self {
            Palette::NoPalette => {
                image.to_rgba_default_gray()
            }

            Palette::Dmg(palette) => {
                let dmg_display_palette = DMG_DISPLAY_PALETTE.lock().unwrap();
                image.to_rgba(|pixel| *dmg_display_palette.get_color(&palette.get_color(pixel)))
            }

            Palette::Gbc(palette) => {
                image.to_rgba(|pixel| palette.get_color(pixel))
            }
        }
    }
}
