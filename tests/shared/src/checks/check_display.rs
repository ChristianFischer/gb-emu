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

use crate::runner::TestCaseError;
use crate::test_config::LcdColorMod;
use image::ImageReader;
use image::{Rgba, RgbaImage};
use libgemi::core::ppu::graphic_data::Color;
use libgemi::core::ppu::ppu::LcdBuffer;
use libgemi::GameBoy;
use std::cmp::min;


/// Loads an image from any given file.
/// Panics on Failure.
pub fn load_image_from_file(image_path: &str) -> Result<RgbaImage, TestCaseError> {
    Ok(
        ImageReader::open(image_path)
        .map_err(|e| TestCaseError::SetUpError(format!("Failed to open image file: {}", e)))?
        .decode()
        .map_err(|e| TestCaseError::SetUpError(format!("Failed to decode image file: {}", e)))?
        .to_rgba8()
    )
}


/// Compares an image pixel with a color value from the emulator.
pub fn compare_pixel(a: &Rgba<u8>, b: &Color) -> bool {
        a.0[0] == b.r
    &&  a.0[1] == b.g
    &&  a.0[2] == b.b
    &&  a.0[3] == b.a
}


/// Applies a color modification to a LCD content image.
pub fn apply_color_mod(lcd: &LcdBuffer, color_mod: &LcdColorMod) -> LcdBuffer {
    let mut new_buffer = LcdBuffer::alloc();

    for y in 0..lcd.get_height() {
        for x in 0..lcd.get_width() {
            let original_color = lcd.get_pixel(x, y);

            let modified_color = match color_mod {
                LcdColorMod::None => original_color.clone(),

                LcdColorMod::Gambatte => {
                    let r = (original_color.r >> 3) as u32;
                    let g = (original_color.g >> 3) as u32;
                    let b = (original_color.b >> 3) as u32;

                    let r2 = (r * 13 + g * 2 + b) / 2;
                    let g2 = (g * 3 + b) * 2;
                    let b2 = (r * 3 + g * 2 + b * 11) / 2;

                    Color {
                        r: r2 as u8,
                        g: g2 as u8,
                        b: b2 as u8,
                        a: original_color.a
                    }
                }
            };

            new_buffer.set_pixel(x, y, modified_color);
        }
    }

    new_buffer
}


/// Compares an image with a LCD screen buffer and returns whether they're identical or not.
pub fn compare_images(expected_image: &RgbaImage, lcd_buffer: &LcdBuffer) -> bool {
    let image_w = min(expected_image.width(),  lcd_buffer.get_width());
    let image_h = min(expected_image.height(), lcd_buffer.get_height());

    for y in 0..image_h {
        for x in 0..image_w {
            let expected_pixel = expected_image.get_pixel(x, y);
            let lcd_pixel      = lcd_buffer.get_pixel(x, y);

            if !compare_pixel(&expected_pixel, &lcd_pixel) {
                return false;
            }
        }
    }

    true
}


/// Compares two images and prints a pattern which describes,
/// which areas of the images are different.
pub fn create_comparison_pattern(expected_image: &RgbaImage, lcd_buffer: &LcdBuffer) -> String {
    let image_w    = min(expected_image.width(),  lcd_buffer.get_width());
    let image_h    = min(expected_image.height(), lcd_buffer.get_height());
    let block_size = 16;

    let mut message = String::new();
    message.push_str("     0  1  2  3  4  5  6  7  8  9\n");

    for block_y in 0..(image_h / block_size) {
        message.push_str(&format!(" {:2} ", block_y));

        for block_x in 0..(image_w / block_size) {
            let mut block_equal = true;

            for inner_y in 0..block_size {
                for inner_x in 0..block_size {
                    let x = block_x * block_size + inner_x;
                    let y = block_y * block_size + inner_y;

                    let expected_pixel = expected_image.get_pixel(x, y);
                    let lcd_pixel      = lcd_buffer.get_pixel(x, y);
                    let pixel_equal    = compare_pixel(&expected_pixel, &lcd_pixel);

                    block_equal &= pixel_equal;
                }
            }

            message.push_str(&format!("{} ", if block_equal { ".." } else { "xx" }));
        }

        message.push_str("\n");
    }

    message
}


/// Compares the content of the emulators LCD buffer with a reference image.
pub fn compare_display_with_image(gb: &GameBoy, image_path: &str, color_mod: &LcdColorMod) -> Result<(), TestCaseError> {
    let expected_image = load_image_from_file(&image_path)?;
    let lcd_buffer     = gb.get_ppu().get_lcd();
    let compare_image  = apply_color_mod(&lcd_buffer, &color_mod);

    // check if both images have the same size
    if
            expected_image.width()  != compare_image.get_width()
        ||  expected_image.height() != compare_image.get_height()
    {
        return Err(TestCaseError::Failed(format!(
            "Emulator Display has different size than reference image: {}x{} vs {}x{}",
            compare_image.get_width(), compare_image.get_height(),
            expected_image.width(),    expected_image.height()
        )));
    }

    // check for image equality
    let images_identical = compare_images(&expected_image, &compare_image);

    if !images_identical {
        // raise an error including a comparison pattern
        return Err(TestCaseError::Failed(format!(
            "Emulator Display different to reference image:\n{}",
            create_comparison_pattern(&expected_image, &compare_image)
        )));
    }

    Ok(())
}

