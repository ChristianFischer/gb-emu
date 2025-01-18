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
use libgemi::GameBoy;

const MOONEYE_RESULT_SEQ_PASS : &[u8] = &[3, 5, 8, 13, 21, 34];
const MOONEYE_RESULT_SEQ_FAIL : &[u8] = &[0x42, 0x42, 0x42, 0x42, 0x42, 0x42];


/// Takes a reference to an emulator running a mooneye test rom and checks whether it's passed or not.
/// Mooneye test roms send a specific sequence of bytes to the serial port to indicate whether they passed or not.
pub fn check_mooneye_test_passed(gb: &GameBoy) -> Result<(), TestCaseError> {
    let test_result_message = gb.get_serial_port().get_output();

    // Fail if the test rom sent the fail sequence
    if test_result_message == MOONEYE_RESULT_SEQ_FAIL {
        Err(TestCaseError::Failed(format!("ROM sent FAILED sequence.")))
    }
    // Fail if the test rom didn't send the pass sequence
    else if test_result_message != MOONEYE_RESULT_SEQ_PASS {
        Err(TestCaseError::Failed(format!("ROM sent unknown sequence: {test_result_message:?}")))
    }
    else {
        Ok(())
    }
}
