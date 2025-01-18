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

use crate::io_utils::filename_to_symbol;
use libgemi::core::device_type::DeviceType;
use libgemi::core::ppu::graphic_data::DmgDisplayPalette;
use std::collections::HashSet;


/// A definition how to modify the colors of the emulator display
/// to match any comparison images used in test cases.
#[derive(Copy, Clone)]
pub enum LcdColorMod {
    /// No modification, takes colors as they're output by the emulator.
    None,

    /// Color modifications like created by the gambatte emulator.
    Gambatte,
}


/// Configuration parameters on how to setup an emulator instance.
#[derive(Clone)]
pub struct SetUpConfig {
    /// Optional: Boot ROM to be run before the actual cartridge is run.
    pub boot_rom_path: Option<String>,

    /// Path to the ROM file to run.
    pub cartridge_path: String,

    /// Optional: A palette to translate DMG color values into
    /// RGBA color values. Only used in DMG mode.
    pub dmg_display_palette: Option<DmgDisplayPalette>,
}


/// Configuration on how to run the emulator.
/// Also contains stop conditions to stop the emulator after tests were finished.
/// Any stop condition are only checked after a frame was completed.
#[derive(Clone)]
pub struct RunConfig {
    /// The number of frames to be processed.
    pub run_frames: Option<u32>,

    /// Stops the emulator when in HALT mode.
    pub stop_on_halt: bool,

    /// Stops the emulator when an infinite loop was detected,
    /// like when a 'JR -2' instruction is invoked.
    pub stop_on_infinite_loop: bool,
}


/// Configuration how to check whether a test ROM was successful or not.
#[derive(Clone)]
pub struct CheckResultConfig {
    /// Compare the emulator display with a given image.
    /// If failed, the test will print a pattern to detect which areas
    /// of the screen were different to the comparison image.
    pub compare_lcd_with_image: Option<String>,

    /// When comparing the LCD content with a reference image,
    /// the LCD content may be modified first in order to get
    /// the expected color values.
    pub color_mod: LcdColorMod,

    /// For gambatte test ROMs:
    /// A result code which is displayed on the emulator screen.
    /// Checks if the expected result code is the same as displayed
    /// on the emulator screen.
    pub gambatte_display_result_code: Option<String>,

    /// For blargg test ROMs:
    /// If true, check for the expected result code in the emulator's serial output.
    pub blargg_check_result_code: bool,

    /// For mooneye test ROMs:
    /// If true, check for the expected result code in the emulator's serial output.
    pub mooneye_check_result_code: bool,
}


/// Configuration of an emulator test run.
/// This configuration has a set of devices which should be tested,
/// and can spawn an unique test case for each device.
pub struct EmulatorTestConfig {
    pub name:       String,
    pub devices:    HashSet<DeviceType>,
    pub setup:      SetUpConfig,
    pub run_config: RunConfig,
    pub result:     CheckResultConfig,
}


/// Configuration of a single test case.
#[derive(Clone)]
pub struct EmulatorTestCase {
    pub device:     DeviceType,
    pub setup:      SetUpConfig,
    pub run_config: RunConfig,
    pub result:     CheckResultConfig,
}


impl SetUpConfig {
    /// Creates a configuration where a ROM file will be loaded.
    pub fn with_rom_file(cartridge_path: &str) -> Self {
        Self {
            cartridge_path:         cartridge_path.to_string(),
            boot_rom_path:          None,
            dmg_display_palette:    None,
        }
    }
}


impl Default for RunConfig {
    fn default() -> Self {
        Self {
            run_frames: None,
            stop_on_halt: false,
            stop_on_infinite_loop: false,
        }
    }
}


impl CheckResultConfig {
    /// Checks if this CheckResultConfig has set up any checks.
    pub fn has_any_checks(&self) -> bool {
        if let Some(_) = self.compare_lcd_with_image {
            return true;
        }

        if let Some(_) = self.gambatte_display_result_code {
            return true;
        }

        if self.blargg_check_result_code || self.mooneye_check_result_code {
            return true;
        }

        false
    }


    /// Checks if one of the checks requires the serial output to be enabled.
    pub fn requires_serial_output(&self) -> bool {
        if self.blargg_check_result_code || self.mooneye_check_result_code {
            return true;
        }

        return false;
    }
}


impl Default for CheckResultConfig {
    fn default() -> Self {
        Self {
            compare_lcd_with_image: None,
            color_mod: LcdColorMod::None,
            gambatte_display_result_code: None,
            blargg_check_result_code: false,
            mooneye_check_result_code: false,
        }
    }
}


impl EmulatorTestConfig {

    /// Generates a hash set with all valid devices.
    pub fn for_any_device() -> HashSet<DeviceType> {
        HashSet::from(DeviceType::ALL_DEVICES)
    }


    /// Generates a hash set with all devices with GameBoy Color compatibility.
    pub fn for_gbc_devices() -> HashSet<DeviceType> {
        HashSet::from(DeviceType::GBC_DEVICES)
    }


    /// Creates a configuration where a ROM file will be loaded.
    pub fn with_rom_file(cartridge_path: &str) -> Self {
        Self {
            name:       filename_to_symbol(cartridge_path),
            devices:    HashSet::new(),
            setup:      SetUpConfig::with_rom_file(cartridge_path),
            run_config: RunConfig::default(),
            result:     CheckResultConfig::default(),
        }
    }


    /// Get an iterator over all test cases which are defined in this configuration.
    pub fn get_test_cases(&self) -> impl Iterator<Item = EmulatorTestCase> + '_ {
        DeviceType::ALL_DEVICES
            .into_iter()

            // just select devices enabled for this test
            .filter(|device| self.devices.contains(device))

            // generate a test case for each of them
            .map(|device| {
                EmulatorTestCase {
                    device:     device.clone(),
                    setup:      self.setup.clone(),
                    run_config: self.run_config.clone(),
                    result:     self.result.clone(),
                }
            })
    }
}
