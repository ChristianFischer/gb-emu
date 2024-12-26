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

#[cfg(feature = "std")]
use std::fmt::{Debug, Display, Formatter, LowerHex, UpperHex};

use crate::utils::get_bit;


/// An RGBA color value containing a RGB value with additional alpha channel
/// ready to be displayed on modern screens.
#[derive(Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color {
    /// 8 bit red channel.
    pub r: u8,

    /// 8 bit green channel.
    pub g: u8,

    /// 8 bit blue channel.
    pub b: u8,

    /// 8 bit alpha channel.
    pub a: u8,
}

/// The pixel value read from a sprite.
/// This value needs to be transformed into a color value using a color palette.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpritePixelValue(u8);

/// A color palette of the classic GameBoy.
/// This contains 4 Colors packed into one byte. Each color in the palette
/// is meant as the brightness of a pixel on the LCD (0-3). This value
/// may be transformed into a RGB color by a DmgDisplayPalette.
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DmgPalette(u8);

/// A pixel value to be displayed on the LCD of a DMG GameBoy.
/// The value represents the brightness of a LCD pixel and may be
/// transformed into a RGB value using a palette to be displayed on modern screens.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DmgLcdPixel(u8);

/// A palette to convert DmgLcdPixel values into RGB colors
/// which can be represented on modern screens.
#[derive(Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DmgDisplayPalette {
    palette: [Color; 4],
}

/// The palette data stored in a dedicated memory area of the GameBoy Color.
/// This is meant to be used to translate sprite pixels into RGB colors.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GbcPaletteData {
    palette: [u16; 4]
}


/// A list of possible tilesets the gameboy can handle.
#[derive(Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TileSet {
    /// The tileset is based on the 0x8000 address plus tile index as unsigned integer.
    H8000,

    /// The tileset is based on the 0x8800 address plus tile index as signed integer.
    H8800,
}

/// A list of possible TileMaps the gameboy can handle.
#[derive(Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TileMap {
    /// This tilemap is stored in the video memory at 0x9800 - 0x9bff
    H9800,

    /// This tilemap is stored in the video memory at 0x9c00 - 0x9fff
    H9C00,
}


/// Stores the data of a single sprite entry, how
/// it's stored in the OAM memory.
#[derive(Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sprite {
    /// The sprites position on Y axis.
    pub pos_y: u8,

    /// The sprites position on X axis.
    pub pos_x: u8,

    /// The tile number containing the sprites image data to be displayed.
    pub tile: u8,

    /// Flags to control the sprites behaviour.
    pub flags: u8,
}



impl Color {
    /// Creates a color object representing white color.
    pub fn white() -> Self {
        Self {
            r: 0xff, g: 0xff, b: 0xff, a: 0xff
        }
    }

    /// Creates a color object from a 32bit uint.
    pub fn from_rgba32(rgba: u32) -> Self {
        Self {
            r: ((rgba >> 24) & 0xff) as u8,
            g: ((rgba >> 16) & 0xff) as u8,
            b: ((rgba >>  8) & 0xff) as u8,
            a: ((rgba >>  0) & 0xff) as u8,
        }
    }

    /// Creates a color object from a 16 bit uint like read from GBC palettes.
    pub fn from_rgb_555(color: u16) -> Self {
        Self {
            r: Self::u5_to_u8(((color >>  0) & 0x1f) as u8),
            g: Self::u5_to_u8(((color >>  5) & 0x1f) as u8),
            b: Self::u5_to_u8(((color >> 10) & 0x1f) as u8),
            a: 0xff,
        }
    }

    /// Get the 32bit uint representation of this color.
    pub fn to_u32(&self) -> u32 {
            ((self.r as u32) << 24)
         |  ((self.g as u32) << 16)
         |  ((self.b as u32) <<  8)
         |  ((self.a as u32) <<  0)
    }

    /// Converts a 5 bit color channel value from an GameBoyColor color palette
    /// into a 8 bit value to be used on 32bit RGB displays
    /// using the conversion as expected by GBC Acid 2 test.
    fn u5_to_u8(v: u8) -> u8 {
        v << 3 | v >> 2
    }
}

#[cfg(feature = "std")]
impl LowerHex for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08x}", self.to_u32())
    }
}

#[cfg(feature = "std")]
impl UpperHex for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08X}", self.to_u32())
    }
}

#[cfg(feature = "std")]
impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02x},{:02x},{:02x},{:02x}", self.r, self.g, self.b, self.a)
    }
}


impl SpritePixelValue {
    /// Creates a sprite pixel with a given value.
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    /// Creates a transparent pixel value.
    pub fn none() -> Self {
        Self(0)
    }

    /// Checks whether a pixel can be considered being transparent (pixel value is 0)
    pub fn is_transparent(&self) -> bool {
        self.0 == 0
    }

    /// Checks whether a pixel can be considered being opaque (pixel value is not 0)
    pub fn is_opaque(&self) -> bool {
        self.0 != 0
    }
}


impl From<u8> for SpritePixelValue {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}


impl Into<u8> for SpritePixelValue {
    fn into(self) -> u8 {
        self.0
    }
}


impl DmgPalette {
    /// Creates a default palette which basically just takes the pixel value as color value.
    pub fn create_default() -> Self {
        Self(0b_11_10_01_00)
    }

    /// Get the LCD value for a specific pixel value.
    pub fn get_color(&self, pixel: &SpritePixelValue) -> DmgLcdPixel {
        let palette_data = self.0;
        DmgLcdPixel((palette_data >> (pixel.0 << 1)) & 0x03)
    }
}

impl From<u8> for DmgPalette {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl Into<u8> for DmgPalette {
    fn into(self) -> u8 {
        self.0
    }
}

impl Default for DmgPalette {
    fn default() -> Self {
        Self(0x00)
    }
}

#[cfg(feature = "std")]
impl Display for DmgPalette {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            ((self.0 >> 0) & 0x03),
            ((self.0 >> 2) & 0x03),
            ((self.0 >> 4) & 0x03),
            ((self.0 >> 6) & 0x03)
        )
    }
}



impl DmgDisplayPalette {
    /// Creates a new Palette with a set of four colors.
    pub fn new(palette: [Color; 4]) -> Self {
        Self {
            palette
        }
    }

    /// Creates a default gray palette for DMG LCD .
    pub fn new_gray() -> Self {
        Self::new([
            Color::from_rgba32(0xffffffffu32),
            Color::from_rgba32(0xa9a9a9ffu32),
            Color::from_rgba32(0x545454ffu32),
            Color::from_rgba32(0x000000ffu32),
        ])
    }

    /// Creates a new palette with green tones similar to the classic GameBoy LCD.
    pub fn new_green() -> Self {
        Self::new([
            Color::from_rgba32(0x9bbc0f_ffu32),
            Color::from_rgba32(0x8bac0f_ffu32),
            Color::from_rgba32(0x306230_ffu32),
            Color::from_rgba32(0x0f380f_ffu32),
        ])
    }

    /// Get the list of colors in this palette.
    pub fn get_colors(&self) -> &[Color; 4] {
        &self.palette
    }

    /// Get the RGBA color for a specific pixel value.
    pub fn get_color(&self, pixel: &DmgLcdPixel) -> &Color {
        &self.palette[pixel.0 as usize]
    }
}



impl GbcPaletteData {
    /// Creates a new palette containing just black color.
    pub fn new() -> Self {
        Self {
            palette: [0x0000; 4]
        }
    }

    /// Get the RGBA color for a specific pixel value.
    pub fn get_color(&self, pixel: &SpritePixelValue) -> Color {
        Color::from_rgb_555(self.palette[pixel.0 as usize])
    }
}


impl TileSet {
    /// Selects a TileSet based on the value of a selection bit from the LCD status register.
    pub fn by_select_bit(bit: bool) -> TileSet {
        match bit {
            false => TileSet::H8800,
            true  => TileSet::H8000,
        }
    }


    /// Get the index of a tile within the VRAM bank.
    /// When the VRAM is seen as an array of images, the
    /// index returned by this function can be used to access
    /// the requested tile.
    pub fn get_tile_image_index(&self, tile: u8) -> usize {
        let tile = tile as usize;
        
        match *self {
            TileSet::H8000 => 0x0000 + tile,
            TileSet::H8800 => 0x0100 + tile - ((tile & 0x80) << 1),
        }
    }


    /// Get the address of a tile when this tileset is used.
    /// ````
    /// use gemi_core::ppu::graphic_data::TileSet;
    ///
    /// assert_eq!(0x8000, TileSet::H8000.address_of_tile(0));
    /// assert_eq!(0x8800, TileSet::H8000.address_of_tile(128));
    /// assert_eq!(0x8ff0, TileSet::H8000.address_of_tile(255));
    ///
    /// assert_eq!(0x9000, TileSet::H8800.address_of_tile(0));
    /// assert_eq!(0x8800, TileSet::H8800.address_of_tile(128));
    /// assert_eq!(0x8ff0, TileSet::H8800.address_of_tile(255));
    /// ````
    pub fn address_of_tile(&self, tile: u8) -> u16 {
        let tile_u16 = tile as u16;

        match *self {
            TileSet::H8000 => 0x8000 + (tile_u16 << 4),
            TileSet::H8800 => 0x9000 + (tile_u16 << 4) - ((tile_u16 & 0x80) << 5),
        }
    }


    /// Get the index of a tile with the current tileset in use.
    /// This may result in [None] when the address is outside the range
    /// of this tileset or if the address is not a multiple of 16.
    /// ````
    /// use gemi_core::ppu::graphic_data::TileSet;
    ///
    /// assert_eq!(Some(0),   TileSet::H8000.get_tile_index_by_address(0x8000));
    /// assert_eq!(Some(128), TileSet::H8000.get_tile_index_by_address(0x8800));
    /// assert_eq!(Some(255), TileSet::H8000.get_tile_index_by_address(0x8ff0));
    /// assert_eq!(None,      TileSet::H8000.get_tile_index_by_address(0x9000));
    ///
    /// assert_eq!(None,      TileSet::H8800.get_tile_index_by_address(0x8000));
    /// assert_eq!(Some(128), TileSet::H8800.get_tile_index_by_address(0x8800));
    /// assert_eq!(Some(255), TileSet::H8800.get_tile_index_by_address(0x8ff0));
    /// assert_eq!(Some(0),   TileSet::H8800.get_tile_index_by_address(0x9000));
    ///
    /// assert_eq!(None,      TileSet::H8000.get_tile_index_by_address(0x8801));
    /// assert_eq!(None,      TileSet::H8800.get_tile_index_by_address(0x8801));
    /// ````
    pub fn get_tile_index_by_address(&self, address: u16) -> Option<u8> {
        // fail when the address is not evenly divisible by 8.
        if (address & 0b1111) != 0 {
            return None;
        }

        match *self {
            // tileset 0: 0x8000 - 0x8ff0
            TileSet::H8000 => {
               if address >= 0x8000 && address <= 0x8ff0 {
                    return Some(((address - 0x8000) >> 4) as u8);
                }
            }

            // tileset 1: 0x8800 - 0x97f0
            TileSet::H8800 => {
                if address >= 0x8800 && address <= 0x8ff0 {
                    return Some(((address - 0x8000) >> 4) as u8);
                }
                
                if address >= 0x9000 && address <= 0x97f0 {
                    return Some(((address - 0x9000) >> 4) as u8);
                }
            }
        }

        None
    }
}


impl TileMap {
    /// Selects a TileMap based on the value of a selection bit from the LCD status register.
    pub fn by_select_bit(bit: bool) -> TileMap {
        match bit {
            false => TileMap::H9800,
            true  => TileMap::H9C00,
        }
    }
    
    
    /// Get the selection bit, which belongs to the current TileMap.
    pub fn to_select_bit(&self) -> bool {
        match *self {
            TileMap::H9800 => false,
            TileMap::H9C00 => true,
        }
    }
    

    /// Get the base address where the tilemap is stored.
    pub fn base_address(&self) -> u16 {
        match *self {
            TileMap::H9800 => 0x9800,
            TileMap::H9C00 => 0x9c00,
        }
    }
}


impl Sprite {
    /// Creates an empty sprite with all values zero.
    pub fn empty() -> Sprite {
        Sprite {
            pos_x: 0,
            pos_y: 0,
            tile:  0,
            flags: 0,
        }
    }

    /// Checks whether the sprite is mirrored on X axis.
    pub fn is_flip_x(&self) -> bool {
        get_bit(self.flags, 5)
    }

    /// Checks whether the sprite is mirrored on Y axis.
    pub fn is_flip_y(&self) -> bool {
        get_bit(self.flags, 6)
    }

    /// Get the number of the VRAM bank, where to read the sprite data from.
    /// Only valid when running a GameBoyColor emulation.
    pub fn get_gbc_vram_bank(&self) -> u8 {
        get_bit(self.flags, 3) as u8
    }

    /// Get the palette used by this sprite when in DMG mode (OBP0 or OBP1).
    pub fn get_dmg_palette(&self) -> u8 {
        get_bit(self.flags, 4) as u8
    }

    /// Get the color palette index used by this sprite, when in GameBoy Color mode.
    pub fn get_color_palette(&self) -> u8 {
        self.flags & 0x07
    }

    /// Checks whether the sprite should always be drawn above background.
    pub fn is_bg_priority(&self) -> bool {
        get_bit(self.flags, 7)
    }
}


#[cfg(feature = "std")]
impl Display for Sprite {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "tile #{} @ {}:{} flipX={} flipY={}",
            self.tile,
            self.pos_x as i32 - 8,
            self.pos_y as i32 - 16,
            self.is_flip_x(),
            self.is_flip_y()
        )
    }
}
