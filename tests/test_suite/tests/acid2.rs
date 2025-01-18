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

use libgemi::core::device_type::DeviceType;
use libgemi::core::ppu::graphic_data::{Color, DmgDisplayPalette};
use testrunner::run_test_case;
use tests_shared::test_config::{CheckResultConfig, EmulatorTestCase, RunConfig, SetUpConfig};


#[test]
pub fn acid2_dmg() {
    let test_case = EmulatorTestCase {
        device: DeviceType::GameBoyDmg,

        setup: SetUpConfig {
            dmg_display_palette: Some(DmgDisplayPalette::new([
                Color::from_rgba32(0xffffffff),
                Color::from_rgba32(0xaaaaaaff),
                Color::from_rgba32(0x555555ff),
                Color::from_rgba32(0x000000ff),
            ])),
            .. SetUpConfig::with_rom_file("dmg-acid2/dmg-acid2.gb")
        },

        run_config: RunConfig {
            stop_on_halt: true,
            .. Default::default()
        },

        result: CheckResultConfig {
            compare_lcd_with_image: Some("dmg-acid2/dmg-acid2-dmg.png".to_string()),
            .. Default::default()
        },
    };

    run_test_case(test_case);
}


#[test]
pub fn acid2_gbc() {
    let test_case = EmulatorTestCase {
        device: DeviceType::GameBoyColor,

        setup: SetUpConfig {
            .. SetUpConfig::with_rom_file("cgb-acid2/cgb-acid2.gbc")
        },

        run_config: RunConfig {
            stop_on_halt: true,
            .. Default::default()
        },

        result: CheckResultConfig {
            compare_lcd_with_image: Some("cgb-acid2/cgb-acid2.png".to_string()),
            .. Default::default()
        },
    };

    run_test_case(test_case);
}


#[test]
#[ignore]
pub fn acid2_gbc_hell() {
    let test_case = EmulatorTestCase {
        device: DeviceType::GameBoyColor,

        setup: SetUpConfig {
            .. SetUpConfig::with_rom_file("cgb-acid-hell/cgb-acid-hell.gbc")
        },

        run_config: RunConfig {
            stop_on_halt: true,
            .. Default::default()
        },

        result: CheckResultConfig {
            compare_lcd_with_image: Some("cgb-acid-hell/cgb-acid-hell.png".to_string()),
            .. Default::default()
        },
    };

    run_test_case(test_case);
}
