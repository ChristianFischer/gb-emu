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
use libgemi::core::ppu::graphic_data::TileMap;
use libgemi::GameBoy;


/// sequence of tile numbers representing the text 'Passed'.
const TILE_SEQUENCE_PASSED : [u8; 6] = [ 0x50, 0x61, 0x73, 0x73, 0x65, 0x64 ];


/// Takes a reference to an emulator running a blargg test rom and checks whether it's passed or not.
/// Usually blargg prints a message like 'Passed' or 'Passed all tests' to the serial port.
/// Some tests are missing this output, for them we check if the 'Passed' text was written on the
/// screen by searching for the according tiles on the tile map.
pub fn check_blargg_test_passed(gb: &GameBoy) -> Result<(), TestCaseError> {
    // get any message written to the serial port
    let output_message = gb.get_serial_port().get_output_as_text();

    match output_message.trim().split('\n').into_iter().last() {
        // Passed - return success
        Some("Passed all tests") | Some("Passed") => return Ok(()),

        // no message, continue
        Some("") => { },

        // other message will be taken as error
        _ => return Err(TestCaseError::Failed(format!("Unexpected output message: {output_message}"))),
    };

    // Search for a 'Passed' message in the tile map
    for line in 0..32 {
        let mut line_match = true;

        // compare tiles with expected sequence
        for tile_x in 0..TILE_SEQUENCE_PASSED.len() {
            let tile_index   = (line * 32 + tile_x) as u16;
            let tile_address = TileMap::H9800.base_address() + tile_index;
            let tile         = gb.get_mmu().read_u8(tile_address);

            if tile != TILE_SEQUENCE_PASSED[tile_x] {
                line_match = false;
                break;
            }
        }

        // success, if all tiles match the 'Passed' sequence
        if line_match {
            return Ok(());
        }
    }

    // no success message
    Err(TestCaseError::Failed("No 'Passed' message received from the emulator.".to_string()))
}
