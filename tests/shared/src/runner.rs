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

use std::fmt::{Debug, Formatter};
use std::panic;
use std::path::PathBuf;

use libgemi::core::boot_rom::BootRom;
use libgemi::core::cartridge::Cartridge;
use libgemi::core::device_type::DeviceType;
use libgemi::core::utils::to_u8;
use libgemi::GameBoy;

use crate::checks::blargg_checks::check_blargg_test_passed;
use crate::checks::check_display::compare_display_with_image;
use crate::checks::gambatte_checks::check_gambatte_display_code;
use crate::checks::mooneye_checks::check_mooneye_test_passed;
use crate::io_utils::Workspace;
use crate::test_config::{CheckResultConfig, EmulatorTestCase, RunConfig, SetUpConfig};

/// The maximum number of frames allowed per emulator run,
/// before it's considered as an error.
const MAX_FRAMES: u32 = 10_000;


/// An enum containing an error code when a test failed to run.
pub enum TestCaseError {
    /// The test ran, but without success code.
    Failed(String),

    /// The emulator failed to run for some reason.
    SetUpError(String),

    /// The emulator panicked while running.
    Panic(String),
}

impl Debug for TestCaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TestCaseError::Failed(s)     => f.write_str(&format!("Failed: {}", s)),
            TestCaseError::SetUpError(s) => f.write_str(&format!("Error: {}",  s)),
            TestCaseError::Panic(s)      => f.write_str(&format!("Panic: {}",  s)),
        }
    }
}


/// Print the commandline to re-run this test ROM normally.
pub fn print_run_command(workspace: &Workspace, device_type: &DeviceType, setup: &SetUpConfig) {
    let mut cmd = String::new();

    // command
    cmd.push_str("cargo run --package gemi-player --bin gemi-player --");

    // add argument for specific device type
    {
        let device_type_arg = match device_type {
            DeviceType::GameBoyDmg       => "--dmg",
            DeviceType::GameBoyPocket    => "--mgb",
            DeviceType::GameBoyColor     => "--gbc",
            DeviceType::GameBoyAdvance   => "--gba",
            DeviceType::GameBoyAdvanceSP => "--ags",
            DeviceType::SuperGameBoy     => "--sgb",
            DeviceType::SuperGameBoy2    => "--sgb2",
        };

        cmd.push_str(&format!(" {}", device_type_arg));
    }

    // add absolute file reference to the test rom
    let absolute_cartridge_path = workspace.get_path_to_str(&setup.cartridge_path);
    cmd.push_str(" ");
    cmd.push_str(&absolute_cartridge_path);

    /*
    // get the current working dir
    let working_dir = if let Ok(wd) = env::current_dir() {
        format!("{}/", wd.to_str().unwrap().to_string().replace("\\", "/"))
    }
    else {
        String::new()
    };

    // add file reference to the ROM to be executed
    cmd.push_str(&format!(
        " {}{}",
        working_dir,
        &setup.cartridge_path
    ));
    */

    println!("#> {cmd}");
}


/// Creates the device emulator based on a setup configuration.
pub fn create_device_with_config(workspace: &Workspace, device_type: &DeviceType, setup: &SetUpConfig) -> Result<GameBoy, TestCaseError> {
    let mut builder = GameBoy::build();

    // set the device type to be emulated
    builder.set_device_type(*device_type);

    // load the cartridge file
    let cartridge_path = PathBuf::from(workspace.get_path_to_str(&setup.cartridge_path));
    let cartridge = Cartridge::load_file(&cartridge_path)
        .map_err(|e| TestCaseError::SetUpError(e.to_string()))
        ?;

    builder.set_cartridge(cartridge);

    // load the boot ROM if any
    if let Some(boot_rom_path) = &setup.boot_rom_path {
        let path = PathBuf::from(workspace.get_path_to_str(boot_rom_path));
        let boot_rom = BootRom::load_file(&path)
            .map_err(|e| TestCaseError::SetUpError(e.to_string()))
            ?;

        builder.set_boot_rom(boot_rom);
    }

    // create the device emulator
    let mut gb = builder.finish()
        .map_err(|e| TestCaseError::SetUpError(e.to_string()))
        ?;

    // initialize
    gb.initialize();

    // set the color palette for DMG emulation
    if let Some(palette) = setup.dmg_display_palette {
        gb.get_ppu_mut().set_dmg_display_palette(palette);
    }

    Ok(gb)
}


/// Run the emulator until any stop condition is met, which is defined in the RunConfig.
pub fn run_to_stop_conditions(gb: &mut GameBoy, config: &RunConfig) -> Result<u32, TestCaseError> {
    let mut stop_next_frame           = false;
    let mut stop_next_frame_countdown = 2u32;
    let mut frames_to_process         = config.run_frames;
    let mut frames                    = 0;

    loop {
        gb.run_frame();
        frames += 1;

        // stop when the flag was set
        // this will be used to ensure the following frame will be completed,
        // after the actual program has been finished.
        if stop_next_frame {
            // well, 'stop_next_frame' does not necessarily mean the VERY next frame...
            // some tests take a bit more time.
            stop_next_frame_countdown = stop_next_frame_countdown.saturating_sub(1);

            if stop_next_frame_countdown == 0 {
                break;
            }
        }

        // stop running after 'n' frames were processed
        if let Some(frames) = &mut frames_to_process {
            *frames = frames.saturating_sub(1);

            if *frames == 0 {
                break;
            }
        }

        // stop running when in HALT state
        if config.stop_on_halt {
            if !gb.get_cpu().is_running() {
                stop_next_frame = true;
            }
        }

        // stop if the emulator is stuck in an infinite loop
        // like JR -2
        if config.stop_on_infinite_loop {
            let current_address = gb.get_cpu().get_instruction_pointer();
            let (addr_high, addr_low) = to_u8(current_address);

            // check for JR -2
            if check_for_opcode_sequence(gb, current_address, &[0x18, 0xfe]) {
                stop_next_frame = true;
            }

            // check for NOP, JR -3
            let seq_nop_jr = [0x00, 0x18, 0xfd];
            if
                    check_for_opcode_sequence(gb, current_address, &seq_nop_jr)
                ||  check_for_opcode_sequence(gb, current_address.saturating_sub(1), &seq_nop_jr)
            {
                stop_next_frame = true;
            }

            // check for jump to self
            if check_for_opcode_sequence(gb, current_address, &[0xc3, addr_low, addr_high]) {
                stop_next_frame = true;
            }
        }

        // if no stop condition was met, stop when 10k frames were computed
        assert!(
            frames < MAX_FRAMES,
            "Emulator did not properly stop before reaching {} frames",
            MAX_FRAMES
        )
    }

    Ok(frames)
}


/// Checks for a sequence of bytes in the memory at the specified address
fn check_for_opcode_sequence(gb: &GameBoy, address: u16, sequence: &[u8]) -> bool {
    for i in 0..sequence.len() {
        let i_addr = address + (i as u16);

        let byte_expected = sequence[i];
        let byte_read     = gb.get_mmu().read_u8(i_addr);

        if byte_read != byte_expected {
            return false;
        }
    }

    true
}


/// After the emulator has been finished, run result checks on the current state.
pub fn check_results(gb: &GameBoy, workspace: &Workspace, result: &CheckResultConfig) -> Result<(), TestCaseError> {
    if let Some(image_path) = &result.compare_lcd_with_image {
        let workspace_image_path = workspace.get_path_to_str(image_path);
        compare_display_with_image(gb, &workspace_image_path, &result.color_mod)?;
    }

    if let Some(gambatte_display_result_code) = &result.gambatte_display_result_code {
        check_gambatte_display_code(gb, &gambatte_display_result_code)?;
    }

    if result.blargg_check_result_code {
        check_blargg_test_passed(gb)?;
    }

    if result.mooneye_check_result_code {
        check_mooneye_test_passed(gb)?;
    }

    Ok(())
}


/// Helper function to run a whole test case
/// Constructs the emulator instance, runs the program and checks for results.
/// On failure, this will return an error with an attached [TestCaseError] item
/// for more detailed information.
pub fn run_test_case_for_result(workspace: &Workspace, test_case: &EmulatorTestCase) -> Result<GameBoy, TestCaseError> {
    let device  = &test_case.device;
    let setup   = &test_case.setup;
    let run_cfg = &test_case.run_config;
    let result  = &test_case.result;

    // Construct
    let mut gb = create_device_with_config(workspace, device, setup)?;

    // enable serial output, if required for any result check
    if result.requires_serial_output() {
        gb.get_serial_port_mut().enable_output_queue(true);
    }

    // Run
    run_to_stop_conditions(&mut gb, &run_cfg)?;

    // Check
    check_results(&gb, workspace, &result)?;

    Ok(gb)
}


/// Safely runs a test case.
/// When the emulator panics, this will lead into a [TestCaseError::Panic] instead of
/// crashing the application.
pub fn run_test_case_safe(workspace: &Workspace, test_case: &EmulatorTestCase) -> Result<GameBoy, TestCaseError> {
    // suppress default error handler to avoid printing the panic message
    let old_panic_hook = panic::take_hook();
    panic::set_hook(Box::new(|_|{}));

    // run the actual test - if it panics, the error will be caught
    let result = panic::catch_unwind(|| {
        run_test_case_for_result(workspace, test_case)
    }).map_err(|e| TestCaseError::Panic(format!("{:?}", e)))?;

    // restore the old panic hook
    panic::set_hook(old_panic_hook);

    // finish
    result
}

