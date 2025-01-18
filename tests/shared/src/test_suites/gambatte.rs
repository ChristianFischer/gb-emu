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

use crate::io_utils::{filename_to_symbol, get_plain_filename, recursive_visit_directory, FindRomCallbacks, HandleDirectory, TestConfigVisitorRef, Workspace};
use crate::rom_utils::file_is_rom;
use crate::test_config::{CheckResultConfig, EmulatorTestConfig, LcdColorMod, RunConfig, SetUpConfig};
use libgemi::core::device_type::DeviceType;
use libgemi::core::ppu::graphic_data::{Color, DmgDisplayPalette};
use std::collections::HashSet;
use std::path::PathBuf;


/// Lookup table to check for device types within gambatte test rom file names.
const GAMBATTE_DEVICE_CODE_TABLE : &[(&str, &[DeviceType])] = &[
    ("dmg08",  &[DeviceType::GameBoyDmg]),
    ("xdmg08", &[DeviceType::GameBoyDmg]),
    ("cgb04c", &[DeviceType::GameBoyColor]),
    ("xcgb",   &[DeviceType::GameBoyColor]),
];


/// Creates a test config for a particular file with any given settings.
fn make_config_for_file(
    file: &PathBuf,
    name: &str,
    devices: HashSet<DeviceType>,
    checks: &CheckResultConfig
) -> EmulatorTestConfig {
    let mut test_name = name.to_string().to_lowercase();

    // add test code to test name, because there are some similar test configs
    // with just a different result code
    if let Some(display_code) = &checks.gambatte_display_result_code {
        test_name = format!("{}_out{}", test_name, display_code.to_lowercase());
    }

    EmulatorTestConfig {
        name: test_name,
        devices,
        setup: SetUpConfig::with_rom_file(&file.to_str().unwrap()),
        run_config: RunConfig {
            run_frames: Some(16),
            .. RunConfig::default()
        },
        result: checks.clone(),
    }
}


/// Finds test configurations for a gambatte test rom.
fn find_gambatte_checks(workspace: &Workspace, file: &PathBuf) -> Vec<EmulatorTestConfig> {
    if let Some(_) = file_is_rom(file) {
        let mut configs : Vec<EmulatorTestConfig> = Vec::new();
        let filename = get_plain_filename(file);

        // check for comparison images
        {
            let path = workspace.get_root_path().join(file).parent().unwrap().to_path_buf();

            for entry in GAMBATTE_DEVICE_CODE_TABLE {
                let ref_image_name = format!("{}_{}.png", filename, entry.0);
                let ref_image_path = path.join(ref_image_name);
                let device_types   = HashSet::from_iter(entry.1.into_iter().map(|d| *d));

                // look for any comparison image matching the current device type
                if ref_image_path.exists() {
                    let mut config = make_config_for_file(
                        file,
                        &filename,
                        device_types.clone(),
                        &Default::default()
                    );

                    config.result.compare_lcd_with_image = Some(
                        workspace.to_relative_path(&ref_image_path).to_str().unwrap().to_string().replace('\\', "/")
                    );

                    if device_types.contains(&DeviceType::GameBoyDmg) {
                        // for DMG set the expected color palette
                        config.setup.dmg_display_palette = Some(DmgDisplayPalette::new([
                            Color::from_rgba32(0x000000ff),
                            Color::from_rgba32(0x555555ff),
                            Color::from_rgba32(0xaaaaaaff),
                            Color::from_rgba32(0xffffffff),
                        ]));
                    }
                    else {
                        // for GameBoyColor define how colors are expected in the comparison image
                        config.result.color_mod = LcdColorMod::Gambatte;
                    }

                    configs.push(config);
                }
            }

            if !configs.is_empty() {
                return configs;
            }
        }

        // check for test config in the file name
        {
            let mut filename_elements: Vec<&str> = filename.split("_").collect();
            let mut test_name = String::new();

            // skip elements until we find a device name
            while !filename_elements.is_empty() {
                if let Some(_) = filepart_is_device(filename_elements[0]) {
                    break;
                }

                // from the skipped file parts build up the test name
                if !test_name.is_empty() {
                    test_name.push('_');
                }

                test_name.push_str(filename_elements[0]);

                filename_elements.remove(0);
            }

            // test name should never be empty
            if test_name.is_empty() {
                test_name = filename.clone();
            }

            // ensure the test name won't include special characters
            test_name = filename_to_symbol(&test_name);

            // parse the file name
            while !filename_elements.is_empty() {
                let mut devices : HashSet<DeviceType> = HashSet::new();
                let mut checks = CheckResultConfig::default();

                // collect all device types found at the current position in the file name
                while let Some(device_types) = filepart_is_device(filename_elements[0]) {
                    for device_type in device_types {
                        devices.insert(*device_type);
                    }

                    filename_elements.remove(0);

                    if filename_elements.is_empty() {
                        break;
                    }
                }

                // when no devices were found, stop here
                if devices.is_empty() {
                    break;
                }

                // check for known checks
                while !filename_elements.is_empty() {
                    if let Some(code) = filepart_is_display_check_code(filename_elements[0]) {
                        checks.gambatte_display_result_code = Some(code);
                        filename_elements.remove(0);
                        continue;
                    }

                    break;
                }

                // when found any checks, create a test config for each device type pending
                if checks.has_any_checks() {
                    let test_config = make_config_for_file(
                        file,
                        &test_name,
                        devices,
                        &checks
                    );

                    configs.push(test_config);
                }
            }

            if !configs.is_empty() {
                return configs;
            }
        }
    }

    vec![]
}


/// Checks whether the given part of a file name matches a device type.
fn filepart_is_device(part: &str) -> Option<&[DeviceType]> {
    for entry in GAMBATTE_DEVICE_CODE_TABLE {
        if part == entry.0 {
            return Some(entry.1);
        }
    }

    None
}


/// Checks whether the given part of a file name matches a display code.
/// This has the expected format of 'outXX' where XX is a 1-2 digit code
/// expected to be displayed on the emulator screen.
fn filepart_is_display_check_code(part: &str) -> Option<String> {
    if part.starts_with("out") && part.len() >= 4 && part.len() <= 5 {
        return Some(part[3 ..].to_string());
    }

    None
}



/// Create tests for Gambatte test roms.
pub fn visit_tests_gambatte(workspace: &Workspace, subdir: &str, visitor: TestConfigVisitorRef) {
    recursive_visit_directory(
        workspace,
        workspace.get_path(subdir),
        &FindRomCallbacks {
            // open module for new directories
            on_handle_dir: Box::new(|_, _| {
                HandleDirectory::CreateModule
            }),

            // create tests for ROM files
            on_file_found: Box::new(|f, _| {
                find_gambatte_checks(workspace, f)
            }),
        },
        visitor
    );
}
