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

use std::collections::HashSet;
use std::path::PathBuf;

use libgemi::core::device_type::DeviceType;

use crate::io_utils::{filename_to_symbol, recursive_visit_directory, FindRomCallbacks, HandleDirectory, TestConfigVisitorRef, Workspace};
use crate::test_config::{CheckResultConfig, EmulatorTestConfig, RunConfig, SetUpConfig};


/// Lookup table to extract device types from file names.
/// Each entry contains a mask for the set of device revision covered by this entry
const DEVICE_TYPE_LOOKUP_TABLE : &[(&str, u32, DeviceType)] = &[
    ("dmg",  0b_001110, DeviceType::GameBoyDmg),        // covers revisions ABC
    ("mgb",  0b_000000, DeviceType::GameBoyPocket),
    ("cgb",  0b_111111, DeviceType::GameBoyColor),      // covers revisions 0ABCDE
    ("agb",  0b_100111, DeviceType::GameBoyAdvance),    // covers revisions 0ABE
    ("ags",  0b_000000, DeviceType::GameBoyAdvanceSP),
    ("sgb2", 0b_000000, DeviceType::SuperGameBoy2),
    ("sgb",  0b_000000, DeviceType::SuperGameBoy),
];


/// Lookup table to extract device groups from file names.
const DEVICE_TYPE_GROUP_LOOKUP_TABLE : &[(&str, &[DeviceType])] = &[
    ("G", &[DeviceType::GameBoyDmg, DeviceType::GameBoyPocket]),
    ("S", &[DeviceType::SuperGameBoy, DeviceType::SuperGameBoy2]),
    ("C", &[DeviceType::GameBoyColor, DeviceType::GameBoyAdvance, DeviceType::GameBoyAdvanceSP]),
    ("A", &[DeviceType::GameBoyAdvance, DeviceType::GameBoyAdvanceSP]),
];


/// Checks if a filename is a valid ROM of the mooneye test suite.
/// If so, it returns the test name stripped away the device codes and a list
/// of devices, this test is intended to run on.
fn file_is_mooneye_test(file: &PathBuf) -> Option<(String, HashSet<DeviceType>)> {
    // check if it's a ROM file at all
    let is_gbc_rom = match file.extension().map(|o| o.to_str().unwrap()) {
        Some("gb")  => false,
        Some("gbc") => true,
        _ => return None,
    };

    // get the filename normalized to unix file separators
    let mut filename = file.file_name().unwrap().to_str().unwrap().to_string();
    filename = filename.replace('\\', "/");

    // strip the path name
    if let Some(last_slash) = filename.rfind('/') {
        filename = filename[.. last_slash].to_string();
    }

    // strip file extension if any
    if let Some(dot) = filename.rfind('.') {
        filename = filename[0 .. dot].to_string();
    }

    // check for a dash within the filename which would separate
    // the actual name from the device type suffix
    if let Some(last_dash) = filename.rfind('-') {
        let mut devices : HashSet<DeviceType> = HashSet::new();
        let mut substr = &filename[last_dash+1 ..];
        let mut found_devices = false;

        'parser_loop:
        while substr.len() > 0 {
            // check device codes like 'dmg', 'cgb'
            for (code, supported_revision_mask, device_type) in DEVICE_TYPE_LOOKUP_TABLE {
                let (match_device, found_revision_codes) = lookup_model_code(&mut substr, code);
                if match_device {
                    // since device revisions are currently not supported, skip any tests
                    // for specific revisions not covered by the current lookup entry
                    if found_revision_codes == *supported_revision_mask {
                        devices.insert(*device_type);
                    }

                    // but still found 'something'
                    found_devices = true;

                    continue 'parser_loop;
                }
            }

            // check device group codes like 'G', which do not contain revision numbers
            for (code, device_types) in DEVICE_TYPE_GROUP_LOOKUP_TABLE {
                if lookup_model_group_code(&mut substr, code) {
                    for device_type in *device_types {
                        devices.insert(*device_type);
                    }

                    found_devices = true;

                    continue 'parser_loop;
                }
            }

            // break if no match
            break;
        }

        // done, if any devices were found
        if found_devices {
            let stripped_filename = &filename[.. last_dash];
            return Some((filename_to_symbol(stripped_filename), devices));
        }
    }

    // if no device specifiers were found, just return a default list
    // based on whether it's a GBC or DMG file extension
    let devices = if is_gbc_rom {
        EmulatorTestConfig::for_gbc_devices()
    }
    else {
        EmulatorTestConfig::for_any_device()
    };

    Some((filename_to_symbol(&filename), devices))
}


/// Checks for a single device code within a given string.
/// On success, it forwards the cursor on reading the string
/// and additionally looks for revision codes.
fn lookup_model_code(substr: &mut &str, code: &str) -> (bool, u32) {
    if substr.starts_with(code) {
        *substr = &substr[code.len() ..];

        let revision_mask = eat_up_revision_codes(substr);

        return (true, revision_mask);
    }

    (false, 0x00)
}


/// Checks for a device group code within a given string.
/// On success, it forwards the cursor on reading the string.
fn lookup_model_group_code(substr: &mut &str, code: &str) -> bool {
    if substr.starts_with(code) {
        *substr = &substr[code.len() ..];

        return true;
    }

    false
}


/// Checks for revision codes within a given string, forwarding the reading cursor
/// and generating a mask of which revisions were found.
fn eat_up_revision_codes(substr: &mut &str) -> u32 {
    let mut revision_mask = 0;

    while substr.len() > 0 {
        let part = &substr[0 .. 1];
        match part {
            "0" | "A" | "B" | "C" | "D" | "E" => {
                revision_mask |= match part {
                    "0" => 0b000001,
                    "A" => 0b000010,
                    "B" => 0b000100,
                    "C" => 0b001000,
                    "D" => 0b010000,
                    "E" => 0b100000,
                    _   => 0b000000,
                };

                *substr = &substr[1 ..];
            }

            _ => break
        }
    }

    revision_mask
}



/// Create tests for Mooneye test roms.
pub fn visit_tests_mooneye(workspace: &Workspace, subdir: &str, visitor: TestConfigVisitorRef) {
    recursive_visit_directory(
        workspace,
        workspace.get_path(subdir),
        &FindRomCallbacks {
            // open module for new directories
            on_handle_dir: Box::new(|d, _| {
                match filename_to_symbol(d.file_name().unwrap().to_str().unwrap()).as_ref() {
                    "madness" | "manual_only" | "utils" => HandleDirectory::Ignore,
                    _ => HandleDirectory::CreateModule,
                }
            }),

            // create tests for ROM files
            on_file_found: Box::new(|f, _| {
                if let Some((test_name, devices)) = file_is_mooneye_test(f) {
                    let cfg = EmulatorTestConfig {
                        name: test_name,
                        devices,
                        setup: SetUpConfig::with_rom_file(&f.to_str().unwrap()),
                        run_config: RunConfig {
                            stop_on_infinite_loop: true,
                            .. RunConfig::default()
                        },
                        result: CheckResultConfig {
                            mooneye_check_result_code: true,
                            .. CheckResultConfig::default()
                        },
                    };

                    vec![ cfg ]
                }
                else {
                    vec![]
                }
            }),
        },
        visitor
    );
}
