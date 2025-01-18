/*
 * Copyright (C) 2022-2023 by Christian Fischer
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

use libgemi::core::device_type::EmulationType;
use crate::io_utils::{filename_to_symbol, FindRomCallbacks, HandleDirectory, recursive_visit_directory, TestConfigVisitorRef, Workspace};
use crate::rom_utils::file_is_rom;
use crate::test_config::{CheckResultConfig, EmulatorTestConfig, RunConfig, SetUpConfig};


/// Create tests for Blargg's test roms.
pub fn visit_tests_blargg(workspace: &Workspace, subdir: &str, visitor: TestConfigVisitorRef) {
    let cgb_sound_folder = String::from("cgb_sound");

    recursive_visit_directory(
        workspace,
        workspace.get_path(subdir),
        &FindRomCallbacks {
            // open module for new directories
            on_handle_dir: Box::new(|_, _| {
                HandleDirectory::CreateModule
            }),

            // create tests for ROM files
            on_file_found: Box::new(|f, state| {
                match file_is_rom(f) {
                    Some(file_device_type) => {
                        // check whether this file is in a subdir of cgb_sound
                        let is_gbc_subfolder = state.stack.contains(&cgb_sound_folder);
                        let is_gbc_file      = file_device_type == EmulationType::GBC;

                        // when folder is gbc, only allow gbc compatible devices
                        let devices = if is_gbc_subfolder || is_gbc_file {
                            EmulatorTestConfig::for_gbc_devices()
                        }
                        else {
                            EmulatorTestConfig::for_any_device()
                        };

                        let cfg = EmulatorTestConfig {
                            name: filename_to_symbol(f.to_str().unwrap()),
                            devices,
                            setup: SetUpConfig::with_rom_file(&f.to_str().unwrap()),
                            run_config: RunConfig {
                                stop_on_infinite_loop: true,
                                .. RunConfig::default()
                            },
                            result: CheckResultConfig {
                                blargg_check_result_code: true,
                                .. CheckResultConfig::default()
                            },
                        };

                        vec![ cfg ]
                    }

                    _ => vec![]
                }
            }),
        },
        visitor
    );
}
