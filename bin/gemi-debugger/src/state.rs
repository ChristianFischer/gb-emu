/*
 * Copyright (C) 2022-2024 by Christian Fischer
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

use gemi_core::cartridge::Cartridge;
use gemi_core::debug::DebugEvent;
use gemi_core::gameboy::{Clock, DeviceType, EmulatorUpdateResults, GameBoy};
use gemi_core::input::InputButton;
use gemi_core::mmu::memory_data::MemoryData;
use gemi_core::ppu::ppu::CPU_CYCLES_PER_FRAME;
use gemi_utils::keybindings::KeyBindings;
use serde::{Deserialize, Deserializer};
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use crate::selection::{Kind, Selection};

/// An enum to store the device type to be emulated
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone)]
pub enum EmulatorDevice {
    GameBoyDmg,
    GameBoyPocket,
    GameBoyColor,
    GameBoyAdvance,
    GameBoyAdvanceSP,
    SuperGameBoy,
    SuperGameBoy2,
}


/// An enum to store the different update modes of the emulator
/// inside of this debugger application.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug)]
pub enum UpdateMode {
    /// The execution of the emulator is currently paused.
    Paused,

    /// The emulator is currently running in continuous mode.
    /// This will run the emulator in real time and continuously spawn new frames.
    Continuous,

    /// Run the emulator for a single step and the switch into pause mode.
    /// The type of step is determined by the [SingleStepMode] parameter.
    Step,
}


#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Copy, Clone)]
/// an enum to define which step to be done when running the
/// emulator in single step mode.
/// See [UpdateMode::Step].
pub enum UpdateStepMode {
    /// Runs the emulator until the next frame was completed.
    Frame,

    /// Runs the emulator until the next line was completed.
    Line,
    
    /// Runs the emulator for a single instruction only.
    Instruction,
}



/// An object handling the current state of the emulator.
/// This provides functionality to load ROMs and serialize the emulator state.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EmulatorState {
    /// Stores the last file path opened.
    pub last_rom_file: Option<PathBuf>,

    /// The actual state of the emulator, which also
    /// contains the emulator instance itself.
    pub emu: EmulatorInstance,

    /// Various states of the UI, which contains the state of buttons
    /// or any active selection.
    pub ui: UiStates,
}


/// An object to store the instance of a running emulator
/// and to provide functionality to run the emulation.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EmulatorInstance {
    /// The actual instance of the emulator.
    /// Will be [None] if no ROM is loaded.
    #[serde(deserialize_with = "deserialize_emulator_instance")]
    gb: Option<GameBoy>,
}


#[derive(serde::Serialize, serde::Deserialize)]
pub struct UiStates {
    /// The key bindings used to control the emulator.
    #[serde(skip)]
    key_bindings: KeyBindings<egui::Key>,

    /// The device being emulated.
    device_type: EmulatorDevice,

    /// The current update mode of the emulator.
    update_mode: UpdateMode,

    /// When [update_mode] is [UpdateMode::Step], this determines
    /// which kind of step to perform.
    update_step_mode: UpdateStepMode,

    /// Describes the currently selected focus item within the UI.
    pub focus: Selection,
    
    /// While moving the mouse cursor over the UI, this will contain the
    /// currently hovered item.
    pub hover: Selection,
}


/// Helper function to deserialize the actual emulator instance.
/// Failing to deserialize the emulator does not lead into fail the whole deserialization.
fn deserialize_emulator_instance<'de, D>(deserializer: D) -> Result<Option<GameBoy>, D::Error>
where
        D: Deserializer<'de>,
{
    let maybe_emu = Option::<GameBoy>::deserialize(deserializer);

    match maybe_emu {
        Ok(emu) => {
            Ok(emu)
        }

        Err(e) => {
            eprintln!("failed to deserialize emulator instance: {e:?}");

            Ok(None)
        }
    }
}


impl Default for EmulatorDevice {
    fn default() -> Self {
        Self::GameBoyColor
    }
}


impl Default for UpdateMode {
    fn default() -> Self {
        Self::Paused
    }
}


impl Default for UpdateStepMode {
    fn default() -> Self {
        Self::Frame
    }
}


impl Display for EmulatorDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EmulatorDevice::GameBoyDmg       => write!(f, "GameBoy DMG"),
            EmulatorDevice::GameBoyPocket    => write!(f, "GameBoy Pocket"),
            EmulatorDevice::GameBoyColor     => write!(f, "GameBoy Color"),
            EmulatorDevice::GameBoyAdvance   => write!(f, "GameBoy Advance"),
            EmulatorDevice::GameBoyAdvanceSP => write!(f, "GameBoy Advance SP"),
            EmulatorDevice::SuperGameBoy     => write!(f, "Super GameBoy"),
            EmulatorDevice::SuperGameBoy2    => write!(f, "Super GameBoy 2"),
        }
    }
}


impl Display for UpdateStepMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdateStepMode::Frame       => write!(f, "Frame"),
            UpdateStepMode::Line        => write!(f, "Line"),
            UpdateStepMode::Instruction => write!(f, "Instruction"),
        }
    }
}


impl Into<DeviceType> for EmulatorDevice {
    fn into(self) -> DeviceType {
        match self {
            EmulatorDevice::GameBoyDmg       => DeviceType::GameBoyDmg,
            EmulatorDevice::GameBoyPocket    => DeviceType::GameBoyPocket,
            EmulatorDevice::GameBoyColor     => DeviceType::GameBoyColor,
            EmulatorDevice::GameBoyAdvance   => DeviceType::GameBoyAdvance,
            EmulatorDevice::GameBoyAdvanceSP => DeviceType::GameBoyAdvanceSP,
            EmulatorDevice::SuperGameBoy     => DeviceType::SuperGameBoy,
            EmulatorDevice::SuperGameBoy2    => DeviceType::SuperGameBoy2,
        }
    }
}


impl EmulatorState {
    /// Open a ROM file and load it into the emulator.
    /// If there's already a running instance of the emulator, this will be closed.
    pub fn open_rom(&mut self, path: &Path) -> Result<(), String> {
        // clear last rom path
        self.last_rom_file = None;

        // load the cartridge from the given path
        let cartridge = Cartridge::load_files_with_default_ram(path)
                .map_err(|e| format!("Failed to load ROM: {}", e))
                ?
        ;

        self.load_cartridge(cartridge)?;

        // store the path to the rom file opened
        self.last_rom_file = Some(path.to_path_buf());

        // success!
        Ok(())
    }


    /// Takes an existing Cartridge object and load it into the emulator.
    /// If there's already a running instance of the emulator, this will be closed.
    pub fn load_cartridge(&mut self, cartridge: Cartridge) -> Result<(), String> {
        // no path known
        self.last_rom_file = None;

        self.instantiate_emulator_with_cartridge(cartridge)
    }


    pub fn reload(&mut self) -> Result<(), String> {
        let cartridge = self.emu.get_cartridge().ok_or("No Cartridge loaded")?;

        // copy the ROM and RAM data from the existing cartridge
        let rom = cartridge.get_rom().as_slice().to_vec();
        let ram = cartridge.get_ram().as_slice().to_vec();

        // create a new cartridge with the existing data
        let new_cartridge = Cartridge::load_from_bytes(rom, Some(ram))
                .map_err(|e| e.to_string())?
        ;

        self.instantiate_emulator_with_cartridge(new_cartridge)
    }


    /// Internal function to create a new emulator instance with an existing cartridge
    /// without changing any other configuration.
    fn instantiate_emulator_with_cartridge(&mut self, cartridge: Cartridge) -> Result<(), String> {
        // on success build the new emulator instance
        let mut builder = GameBoy::build();
        builder.set_device_type(self.ui.get_device_type().clone().into());
        builder.set_cartridge(cartridge);

        // finish & initialize
        let mut gb = builder.finish().map_err(|e| e.to_string())?;
        gb.initialize();

        // reset key states after emulator loading
        self.ui.key_bindings.reset_key_states(&mut gb);

        // store the new emulator instance
        self.emu.gb = Some(gb);

        // success!
        Ok(())
    }


    /// Checks whether the emulator is currently running or not.
    /// This will be the case if the emulator is loaded and not paused.
    pub fn is_running(&self) -> bool {
        self.emu.is_emulator_loaded() && !self.ui.is_paused()
    }


    /// Update the emulator, if any.
    pub fn update(&mut self) {
        self.ui.update_mode = match self.ui.update_mode {
            // stay in pause state
            UpdateMode::Paused => {
                UpdateMode::Paused
            }

            // process the next frame and stay in continuous mode
            UpdateMode::Continuous => {
                self.emu.run_frame();
                UpdateMode::Continuous
            }

            // process the next step and switch into pause mode
            UpdateMode::Step => {
                match self.ui.update_step_mode {
                    UpdateStepMode::Frame       => self.emu.run_frame(),
                    UpdateStepMode::Line        => self.emu.run_line(),
                    UpdateStepMode::Instruction => self.emu.run_single_step(),
                }

                UpdateMode::Paused
            }
        }
    }


    /// Forward key events into the emulator.
    pub fn set_key_pressed(&mut self, key: egui::Key, pressed: bool) {
        if let Some(gb) = &mut self.emu.gb {
            self.ui.key_bindings.set_key_pressed_and_fwd(key, pressed, gb);
        }
    }
}


impl EmulatorInstance {
    /// Get the currently running emulator instance.
    pub fn get_emulator(&self) -> Option<&GameBoy> {
        self.gb.as_ref()
    }


    /// Get the currently running emulator instance.
    pub fn get_emulator_mut(&mut self) -> Option<&mut GameBoy> {
        self.gb.as_mut()
    }


    /// Check if an emulator instance is currently loaded.
    pub fn is_emulator_loaded(&self) -> bool {
        self.gb.is_some()
    }


    /// Get the cartridge of the currently running emulator instance, if any.
    pub fn get_cartridge(&self) -> Option<&Cartridge> {
        self.get_emulator()
            .and_then(|emu| emu.get_peripherals().mem.get_cartridge())
    }


    /// Process a single frame of the emulator, if any.
    pub fn run_frame(&mut self) {
        self.run_until(|_emu, cycles, result|
                result.events.contains(DebugEvent::PpuFrameCompleted)
            ||  cycles >= CPU_CYCLES_PER_FRAME
        );
    }


    /// Run the emulator until the next scanline was completed drawing.
    pub fn run_line(&mut self) {
        self.run_until(|_emu, cycles, result|
                result.events.contains(DebugEvent::PpuLineCompleted)
            ||  cycles >= CPU_CYCLES_PER_FRAME
        );
    }


    /// Run the emulator for a single instruction.
    pub fn run_single_step(&mut self) {
        if let Some(emu) = self.get_emulator_mut() {
            emu.run_single_step();
        }
    }


    /// Run the emulator until a certain condition is met. 
    pub fn run_until<F>(&mut self, condition: F)
        where F: Fn(&GameBoy, Clock, EmulatorUpdateResults) -> bool
    {
        if let Some(emu) = self.get_emulator_mut() {
            let mut cycles = 0;

            loop {
                let result = emu.run_single_step();
                cycles += result.cycles;

                if condition(emu, cycles, result) {
                    break;
                }
            }
        }
    }
}


impl UiStates {
    /// Get the device type to be emulated.
    /// This is not necessarily the device type of the current emulator instance,
    /// but will be used next type starting an emulator
    pub fn get_device_type(&self) -> &EmulatorDevice {
        &self.device_type
    }

    /// Set the device type of the emulator.
    /// This will not affect the currently running emulator,
    /// but takes effect next time when loading a new emulator instance.
    pub fn set_device_type(&mut self, device_type: EmulatorDevice) {
        self.device_type = device_type;
    }

    /// Get the current update mode of the emulator.
    pub fn get_update_mode(&self) -> &UpdateMode {
        &self.update_mode
    }


    /// Checks whether the emulator is currently paused or not.
    pub fn is_paused(&self) -> bool {
        match self.update_mode {
            UpdateMode::Paused => true,
            _ => false,
        }
    }


    /// Change the emulator's update mode.
    pub fn set_update_mode(&mut self, mode: UpdateMode) {
        self.update_mode = mode;
    }


    /// Get what will be performed a single step.
    pub fn get_update_step_mode(&self) -> &UpdateStepMode {
        &self.update_step_mode
    }


    /// Set what will be performed a single step.
    pub fn set_update_step_mode(&mut self, mode: UpdateStepMode) {
        self.update_step_mode = mode;
    }
}



fn make_default_key_bindings() -> KeyBindings<egui::Key> {
    KeyBindings::with_mapping(
        vec![
            (InputButton::DPadRight,    vec![egui::Key::D,      egui::Key::ArrowRight   ]),
            (InputButton::DPadLeft,     vec![egui::Key::A,      egui::Key::ArrowLeft    ]),
            (InputButton::DPadUp,       vec![egui::Key::W,      egui::Key::ArrowUp      ]),
            (InputButton::DPadDown,     vec![egui::Key::S,      egui::Key::ArrowDown    ]),
            (InputButton::A,            vec![egui::Key::E,      egui::Key::X            ]),
            (InputButton::B,            vec![egui::Key::Q,      egui::Key::Y            ]),
            (InputButton::Select,       vec![egui::Key::Num1,   egui::Key::Backspace    ]),
            (InputButton::Start,        vec![egui::Key::Num2,   egui::Key::Enter        ]),
        ]
    )
}


impl Default for EmulatorState {
    fn default() -> Self {
        Self {
            last_rom_file: None,

            emu: EmulatorInstance {
                gb: None,
            },

            ui: UiStates {
                key_bindings:       make_default_key_bindings(),
                device_type:        EmulatorDevice::GameBoyColor,
                update_mode:        UpdateMode::Paused,
                update_step_mode:   UpdateStepMode::Frame,
                focus:              Selection::new(Kind::Focus),
                hover:              Selection::new(Kind::Hover),
            },
        }
    }
}
