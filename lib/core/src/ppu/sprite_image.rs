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

use crate::ppu::graphic_data::{Color, SpritePixelValue};

/// A struct representing the image data of a single sprite
/// as it is stored inside the video memory.
/// Each sprite has a 8x8 pixel size with 2 bits per pixel,
/// which results in a total of 16 bytes per sprite.
#[derive(Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpriteImage {
    /// The actual, packed data of the sprite.
    image_data: [u8; 16],
}


impl SpriteImage {
    /// Creates a new sprite image from the given data.
    pub fn new(image_data: [u8; 16]) -> Self {
        Self {
            image_data,
        }
    }


    /// An utility function to iterate over all pixels of the sprite.
    /// This will invoke a callback for each pixel, allowing them to
    /// be stored in any format.
    pub fn read_pixels(&self, mut cb: impl FnMut(usize, usize, u8)) {
        for y in 0..8 {
            // the first and second byte containing the pixel data of
            // the current line.
            let byte0 = self.image_data[y*2 + 0];
            let byte1 = self.image_data[y*2 + 1];

            for x in 0..8 {
                // the mask to extract the data for the current pixel.
                let pixel_mask = 1u8 << (7 - x);

                // combine both bits from the first and second byte
                // to form a single 4-bit-color value.
                let pixel =
                        (if (byte0 & pixel_mask) != 0 { 0x01 } else { 0x00 })
                    |   (if (byte1 & pixel_mask) != 0 { 0x02 } else { 0x00 })
                ;

                cb(x, y, pixel);
            }
        }
    }


    /// Create a new byte array containing a single value for each pixel of this sprite.
    /// Each pixel still has a value from 0x00 to 0x03 and needs to be mapped to an
    /// actual color value to be displayed.
    pub fn to_pixels(&self) -> [u8; 64] {
        let mut image_data = [0x00; 64];

        self.read_pixels(&mut |x, y, pixel| {
            image_data[y*8 + x] = pixel;
        });

        image_data
    }


    /// Creates a new byte array containing RGBA values for each pixel of this sprite.
    /// The data is stored in a byte array, with 4 bytes for each pixel containing
    /// a red, green, blue and alpha value.
    pub fn to_rgba(&self, to_color: impl Fn(&SpritePixelValue) -> Color) -> [u8; 256] {
        let mut image_data = [0x00; 256];

        self.read_pixels(&mut |x, y, pixel| {
            let sprite_pixel = SpritePixelValue::new(pixel);
            let color        = to_color(&sprite_pixel);

            let index = (y*8 + x) * 4;
            image_data[index + 0] = color.r;
            image_data[index + 1] = color.g;
            image_data[index + 2] = color.b;
            image_data[index + 3] = color.a;
        });

        image_data
    }


    /// Creates a new byte array containing RGBA values for each pixel of this sprite.
    /// The data is stored in a byte array, with 4 bytes for each pixel containing
    /// a red, green, blue and alpha value.
    pub fn to_rgba_default_gray(&self) -> [u8; 256] {
        self.to_rgba(|pixel| {
            let gray = match (*pixel).into() {
                0x03 => 0xFF,
                0x02 => 0xAA,
                0x01 => 0x55,
                _    => 0x00,
            };

            Color {
                r: gray,
                g: gray,
                b: gray,
                a: 0xff,
            }
        })
    }


    /// Creates a new string containing the hex representation of the sprite data.
    /// This is unique for each combination of sprite data and can be used as a
    /// unique identifier for the sprite.
    #[cfg(feature = "std")]
    pub fn to_hex_string(&self) -> String {
        format!(
                "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                self.image_data[0x00], self.image_data[0x01], self.image_data[0x02], self.image_data[0x03],
                self.image_data[0x04], self.image_data[0x05], self.image_data[0x06], self.image_data[0x07],
                self.image_data[0x08], self.image_data[0x09], self.image_data[0x0a], self.image_data[0x0b],
                self.image_data[0x0c], self.image_data[0x0d], self.image_data[0x0e], self.image_data[0x0f],
        )
    }
}
