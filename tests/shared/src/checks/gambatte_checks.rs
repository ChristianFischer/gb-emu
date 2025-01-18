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

use libgemi::core::ppu::ppu::LcdBuffer;
use libgemi::GameBoy;

use crate::runner::TestCaseError;

type BitmapDigit      = [u8; 8];
type BitmapDigitEntry = (char, BitmapDigit);

/// A lookup table containing the hex digits 0-9 and a-f
/// as used by gambatte test roms to display result codes.
const GAMBATTE_DIGIT_BITMAPS : &[BitmapDigitEntry] = &[
    ('0', [
        0b_0000_0000,
        0b_0111_1111,
        0b_0100_0001,
        0b_0100_0001,
        0b_0100_0001,
        0b_0100_0001,
        0b_0100_0001,
        0b_0111_1111,
    ]),

    ('1', [
        0b_0000_0000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
        0b_0000_1000,
    ]),

    ('2', [
        0b_0000_0000,
        0b_0111_1111,
        0b_0000_0001,
        0b_0000_0001,
        0b_0111_1111,
        0b_0100_0000,
        0b_0100_0000,
        0b_0111_1111,
    ]),

    ('3', [
        0b_0000_0000,
        0b_0111_1111,
        0b_0000_0001,
        0b_0000_0001,
        0b_0011_1111,
        0b_0000_0001,
        0b_0000_0001,
        0b_0111_1111,
    ]),

    ('4', [
        0b_0000_0000,
        0b_0100_0001,
        0b_0100_0001,
        0b_0100_0001,
        0b_0111_1111,
        0b_0000_0001,
        0b_0000_0001,
        0b_0000_0001,
    ]),

    ('5', [
        0b_0000_0000,
        0b_0111_1111,
        0b_1000_0000,
        0b_1000_0000,
        0b_0111_1110,
        0b_0000_0001,
        0b_0000_0001,
        0b_0111_1110,
    ]),

    ('5', [
        0b_0000_0000,
        0b_0111_1111,
        0b_0100_0000,
        0b_0100_0000,
        0b_0111_1110,
        0b_0000_0001,
        0b_0000_0001,
        0b_0111_1110,
    ]),

    ('6', [
        0b_0000_0000,
        0b_0111_1111,
        0b_0100_0000,
        0b_0100_0000,
        0b_0111_1111,
        0b_0100_0001,
        0b_0100_0001,
        0b_0111_1111,
    ]),

    ('7', [
        0b_0000_0000,
        0b_0111_1111,
        0b_0000_0001,
        0b_0000_0010,
        0b_0000_0100,
        0b_0000_1000,
        0b_0001_0000,
        0b_0001_0000,
    ]),

    ('8', [
        0b_0000_0000,
        0b_0011_1110,
        0b_0100_0001,
        0b_0100_0001,
        0b_0011_1110,
        0b_0100_0001,
        0b_0100_0001,
        0b_0011_1110,
    ]),

    ('9', [
        0b_0000_0000,
        0b_0111_1111,
        0b_0100_0001,
        0b_0100_0001,
        0b_0111_1111,
        0b_0000_0001,
        0b_0000_0001,
        0b_0111_1111,
    ]),

    ('A', [
        0b_0000_0000,
        0b_0000_1000,
        0b_0010_0010,
        0b_0100_0001,
        0b_0111_1111,
        0b_0100_0001,
        0b_0100_0001,
        0b_0100_0001,
    ]),

    ('B', [
        0b_0000_0000,
        0b_0111_1110,
        0b_0100_0001,
        0b_0100_0001,
        0b_0111_1110,
        0b_0100_0001,
        0b_0100_0001,
        0b_0111_1110,
    ]),

    ('C', [
        0b_0000_0000,
        0b_0011_1110,
        0b_0100_0001,
        0b_0100_0000,
        0b_0100_0000,
        0b_0100_0000,
        0b_0100_0001,
        0b_0011_1110,
    ]),

    ('D', [
        0b_0000_0000,
        0b_0111_1110,
        0b_0100_0001,
        0b_0100_0001,
        0b_0100_0001,
        0b_0100_0001,
        0b_0100_0001,
        0b_0111_1110,
    ]),

    ('E', [
        0b_0000_0000,
        0b_0111_1111,
        0b_0100_0000,
        0b_0100_0000,
        0b_0111_1111,
        0b_0100_0000,
        0b_0100_0000,
        0b_0111_1111,
    ]),

    ('F', [
        0b_0000_0000,
        0b_0111_1111,
        0b_0100_0000,
        0b_0100_0000,
        0b_0111_1111,
        0b_0100_0000,
        0b_0100_0000,
        0b_0100_0000,
    ]),
];


/// Checks if the emulator display is showing the expected result code.
pub fn check_gambatte_display_code(gb: &GameBoy, display_code_expected: &str) -> Result<(), TestCaseError> {
    let length = display_code_expected.len();

    match read_characters_from_display(gb.get_ppu().get_lcd(), length) {
        Some(display_code_read) => {
            if display_code_read != display_code_expected {
                Err(TestCaseError::Failed(format!(
                    "Display code does not match. Expected: '{}', got: '{}'",
                    display_code_expected,
                    display_code_read
                )))
            }
            else {
                Ok(())
            }
        }

        None => {
            Err(TestCaseError::SetUpError("Failed to read display code from display".to_string()))
        }
    }
}


/// Read the characters displayed on the emulator display.
/// Returns a sequence of characters to be seen on the display, or 'None'
/// if it was not possible to read the expected amount of characters
fn read_characters_from_display(lcd: &LcdBuffer, count: usize) -> Option<String> {
    let mut characters = String::new();

    for x in 0..count {
        if let Some(character) = read_character_from_display(lcd, x as u32, 0) {
            characters.push(character);
        }
        else {
            return None;
        }
    }

    Some(characters)
}


/// Reads a single character from the emulator display.
/// Returns the character read or 'None' if the displaying image
/// does not match any expected character.
fn read_character_from_display(lcd: &LcdBuffer, x: u32, y: u32) -> Option<char> {
    let x_begin = x * 8;
    let y_begin = y * 8;

    'iterate_character: for entry in GAMBATTE_DIGIT_BITMAPS {
        let bitmap = &entry.1;

        for py in 0..8 {
            for px in 0..8 {
                let pixel_x = x_begin + px;
                let pixel_y = y_begin + py;
                let pixel   = lcd.get_pixel(pixel_x, pixel_y);

                let pixel_value    = pixel.r < 0x40 && pixel.g < 0x40 && pixel.b < 0x40;
                let expected_value = ((bitmap[py as usize] >> (7-px)) & 0x01) != 0;

                if expected_value != pixel_value {
                    continue 'iterate_character;
                }
            }
        }

        // pattern did match
        return Some(entry.0);
    }

    None
}
