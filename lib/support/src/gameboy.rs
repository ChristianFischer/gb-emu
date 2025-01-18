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

use gemi_core::boot_rom::BootRom;
use gemi_core::cartridge::{Cartridge, GameBoyColorSupport};

use gemi_core::apu::Apu;
use gemi_core::cpu::cpu::Cpu;
use gemi_core::device_type::{DeviceConfig, DeviceType, EmulationType};
use gemi_core::emulator_core::{Clock, EmulatorCore, EmulatorUpdateResults};
use gemi_core::input::Input;
use gemi_core::mmu::memory::Memory;
use gemi_core::mmu::mmu::Mmu;
use gemi_core::ppu::ppu::Ppu;
use gemi_core::serial::SerialPort;
use std::fmt::{Display, Formatter};

/// A factory class to construct a GameBoy device object.
/// Usually created via GameBoy::build()
pub struct Builder {
    boot_rom:      Option<BootRom>,
    cartridge:     Option<Cartridge>,
    device_type:   Option<DeviceType>,
    print_opcodes: bool,
}


/// Error codes occurred during creating an emulator instance.
#[derive(Debug)]
pub enum BuilderErrorCode {
    GameBoyColorNotSupported,
}


/// The GameBoy object providing access to all it's emulated components.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GameBoy {
    device_config: DeviceConfig,
    emulator: Box<EmulatorCore>,
}


impl Builder {
    /// Creates a new empty GameBoy builder
    pub fn new() -> Self {
        Self {
            boot_rom:      None,
            cartridge:     None,
            device_type:   None,
            print_opcodes: false,
        }
    }


    /// Set the boot ROM, which will be executed before the actual ROM.
    pub fn set_boot_rom(&mut self, boot_rom: BootRom) {
        self.boot_rom = Some(boot_rom);
    }


    /// Set the cartridge, which ROM will be executed.
    pub fn set_cartridge(&mut self, cartridge: Cartridge) {
        self.cartridge = Some(cartridge);
    }


    /// Override the preferred device type.
    /// If not specified, the device type will be determined by the cartridge type.
    pub fn set_device_type(&mut self, device_type: DeviceType) {
        self.device_type = Some(device_type);
    }


    /// Configures whether the emulator should print all opcodes being executed or not.
    pub fn set_print_opcodes(&mut self, print: bool) {
        self.print_opcodes = print;
    }


    /// Get the preferred device type, which is either specified explicitly
    /// or selected by the cartridge properties.
    pub fn select_preferred_device_type(&self) -> DeviceType {
        // explicit type will be preferred
        if let Some(device_type) = &self.device_type {
            return *device_type;
        }

        // determine the preferred device type by the cartridge properties
        if let Some(cartridge) = &self.cartridge {
            return match cartridge.get_cgb_support() {
                GameBoyColorSupport::None      => DeviceType::GameBoyDmg,
                GameBoyColorSupport::Supported => DeviceType::GameBoyColor,
                GameBoyColorSupport::Required  => DeviceType::GameBoyColor,
            };
        }

        // default to classic GameBoy
        DeviceType::GameBoyDmg
    }


    /// Check the emulation type based on the selected device and GameBoyColor
    /// support of the selected cartridge.
    pub fn select_emulation_type(&self, device_type: &DeviceType) -> EmulationType {
        match device_type {
            DeviceType::GameBoyDmg => {}
            _ => {
                if let Some(cartridge) = &self.cartridge {
                    if cartridge.supports_cgb() {
                        return EmulationType::GBC;
                    }
                }
            }
        }

        EmulationType::DMG
    }


    /// Build the GameBoy device emulator based on the properties specified with this builder.
    pub fn finish(mut self) -> Result<GameBoy, BuilderErrorCode> {
        // select the preferred device type based on the current config and cartridge
        let device_type    = self.select_preferred_device_type();
        let emulation_type = self.select_emulation_type(&device_type);

        // setup device config based on the current configuration
        let device_config = DeviceConfig {
            device: device_type,
            emulation: emulation_type,
            print_opcodes: self.print_opcodes
        };

        // construct the GameBoy object
        let mut emulator = Box::new(EmulatorCore::new(device_config));

        // set boot ROM, if any
        if let Some(boot_rom) = self.boot_rom.take() {
            emulator.get_peripherals_mut().mem.set_boot_rom(boot_rom);
        }

        // insert cartridge, if any
        if let Some(cartridge) = self.cartridge.take() {
            emulator.get_peripherals_mut().mem.set_cartridge(cartridge);
        }

        Ok(GameBoy {
            device_config,
            emulator,
        })
    }
}


impl GameBoy {
    /// Creates a builder to build up the device.
    pub fn build() -> Builder {
        Builder::new()
    }


    /// Boot the device, initializing the Boot ROM program.
    pub fn initialize(&mut self) {
        self.emulator.initialize();
    }


    /// Get the number of cycles processed by the emulator since it started.
    pub fn get_total_cycles_processed(&self) -> Clock {
        self.emulator.get_total_cycles_processed()
    }


    /// Get the time in seconds the emulator did run.
    pub fn get_total_seconds_processed(&self) -> f32 {
        self.emulator.get_total_seconds_processed()
    }


    /// Runs the emulator for a single step, either an instruction
    /// or to process a single HALT cycle.
    pub fn run_single_step(&mut self) -> EmulatorUpdateResults {
        self.emulator.run_single_step()
    }


    /// Continues running the program located on the cartridge,
    /// until the PPU has completed one single frame.
    pub fn run_frame(&mut self) -> EmulatorUpdateResults {
        self.emulator.run_frame()
    }


    /// Get the emulator device configuration.
    pub fn get_config(&self) -> &DeviceConfig {
        &self.device_config
    }


    /// Get the actual emulator instance.
    pub fn get_emulator(&self) -> &EmulatorCore {
        &self.emulator
    }


    /// Get the actual emulator instance.
    pub fn get_emulator_mut(&mut self) -> &mut EmulatorCore {
        &mut self.emulator
    }


    /// Get the device CPU.
    pub fn get_cpu(&self) -> &Cpu {
        &self.emulator.cpu
    }


    /// Get the device CPU.
    pub fn get_cpu_mut(&mut self) -> &mut Cpu {
        &mut self.emulator.cpu
    }


    /// Get the device MMU.
    pub fn get_mmu(&self) -> &Mmu {
        self.emulator.get_mmu()
    }


    /// Get the device MMU.
    pub fn get_mmu_mut(&mut self) -> &mut Mmu {
        self.emulator.get_mmu_mut()
    }


    /// Get the device memory component.
    pub fn get_memory(&self) -> &Memory {
        &self.emulator.get_peripherals().mem
    }


    /// Get the device memory component.
    pub fn get_memory_mut(&mut self) -> &mut Memory {
        &mut self.emulator.get_peripherals_mut().mem
    }


    /// Get the device PPU.
    pub fn get_ppu(&self) -> &Ppu {
        &self.emulator.get_peripherals().ppu
    }


    /// Get the device PPU.
    pub fn get_ppu_mut(&mut self) -> &mut Ppu {
        &mut self.emulator.get_peripherals_mut().ppu
    }


    /// Get the device APU.
    pub fn get_apu(&self) -> &Apu {
        &self.emulator.get_peripherals().apu
    }


    /// Get the device APU.
    pub fn get_apu_mut(&mut self) -> &mut Apu {
        &mut self.emulator.get_peripherals_mut().apu
    }


    /// Get the device input component.
    pub fn get_input(&self) -> &Input {
        &self.emulator.get_peripherals().input
    }


    /// Get the device input component.
    pub fn get_input_mut(&mut self) -> &mut Input {
        &mut self.emulator.get_peripherals_mut().input
    }


    /// Get the device serial port component.
    pub fn get_serial_port(&self) -> &SerialPort {
        &self.emulator.get_peripherals().serial
    }


    /// Get the device serial port component.
    pub fn get_serial_port_mut(&mut self) -> &mut SerialPort {
        &mut self.emulator.get_peripherals_mut().serial
    }
}


impl Display for BuilderErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "failed to build emulator")
    }
}
