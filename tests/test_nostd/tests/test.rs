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

#![no_std]

use gemi_core::device_type::{DeviceConfig, DeviceType, EmulationType};
use gemi_core::emulator_core::EmulatorCore;


#[test]
fn test_nostd() {
    // setup device type
    let device_config = DeviceConfig {
        device: DeviceType::GameBoyDmg,
        emulation: EmulationType::DMG,
        print_opcodes: false,
    };

    // create the emulator instance
    let mut emulator = EmulatorCore::new(device_config);

    // run one single frame
    emulator.run_frame();
}
