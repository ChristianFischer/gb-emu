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

use crate::apu::Apu;
use crate::cartridge::LicenseeCode;
use crate::cpu::cpu::{Cpu, CpuFlag, RegisterR8, CPU_CLOCK_SPEED};
use crate::cpu::interrupts::InterruptRegisters;
use crate::cpu::opcode::{OpCodeContext, OpCodeResult};
use crate::debug::{DebugEvent, DebugEvents};
use crate::device_type::{DeviceConfig, DeviceType, EmulationType};
use crate::input::Input;
use crate::mmu::memory::Memory;
use crate::mmu::memory_bus::{MemoryBusConnection, MemoryBusSignals};
use crate::mmu::mmu::Mmu;
use crate::ppu::ppu::{Ppu, CPU_CYCLES_PER_FRAME};
use crate::serial::SerialPort;
use crate::timer::Timer;
use crate::utils::{carrying_add_u8, get_high};


/// Type to measure clock ticks of the device.
/// Alias for unsigned 64bit integer.
pub type Clock = u64;


/// The GameBoy object providing access to all it's emulated components.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmulatorCore {
    device_config: DeviceConfig,

    pub cpu: Cpu,

    total_cycles: Clock,
}


/// A set of components connected together via memory bus.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Peripherals {
    pub apu:        Apu,
    pub ppu:        Ppu,
    pub mem:        Memory,
    pub timer:      Timer,
    pub input:      Input,
    pub serial:     SerialPort,
    pub interrupts: InterruptRegisters,
}


/// An object containing feedback from running the emulator.
#[derive(Default)]
pub struct EmulatorUpdateResults {
    /// The number of cycles being processed.
    pub cycles: Clock,

    /// Any debug events occurred during updating the emulator.
    pub events: DebugEvents,
}


impl EmulatorCore {

    /// Create a new GameBoy device.
    pub fn new(device_config: DeviceConfig) -> Self {
        Self {
            device_config,
            
            cpu: Cpu::new(
                Mmu::new(
                    Peripherals {
                        apu:        Apu::new(device_config),
                        ppu:        Ppu::new(device_config),
                        mem:        Memory::new(device_config),
                        timer:      Timer::new(),
                        input:      Input::new(),
                        serial:     SerialPort::new(),
                        interrupts: InterruptRegisters::new(),
                    }
                )
            ),

            total_cycles: 0,
        }
    }


    /// Boot the device, initializing the Boot ROM program.
    pub fn initialize(&mut self) {
        if self.get_peripherals().mem.has_boot_rom() {
            self.cpu.set_instruction_pointer(0x0000);
        }
        else {
            self.setup_initial_values();
        }
    }

    /// setup values like expected after the boot rom was executed on the original GameBoy.
    fn setup_initial_values(&mut self) {
        let pc = 0x0100;
        let sp = 0xfffe;

        // the title checksum is calculated on GBC and GBA in DMG compatibility mode
        // if licensee code is '1' in either old or new format
        let title_checksum = if let Some(cartridge) = self.get_peripherals().mem.get_cartridge().as_ref() {
            match cartridge.get_licensee_code() {
                LicenseeCode::Old(1) | LicenseeCode::New(1) => {
                    cartridge.compute_title_checksum()
                }

                _ => 0x00
            }
        }
        else {
            // without cartridge, the checksum is 0xff
            0xff
        };

        // read cartridge header checksum
        let header_checksum = self.get_mmu().read_u8(0x14d);

        // select initial values based on device type and emulation mode
        let (a, flag_z, flag_n, flag_h, flag_c, b, c, d, e, h, l) =
            match (self.device_config.device, self.device_config.emulation)
        {
            // classic GameBoy
            (DeviceType::GameBoyDmg, _) => {
                let ch = header_checksum != 0;
                (0x01, true, false, ch, ch, 0x00, 0x13, 0x00, 0xd8, 0x01, 0x4d)
            }

            // classic GameBoy
            (DeviceType::GameBoyPocket, _) => {
                let ch = header_checksum != 0;
                (0x01, true, false, ch, ch, 0x00, 0x13, 0x00, 0xd8, 0x01, 0x4d)
            }

            // GameBoy Color with classic GameBoy cartridge
            (DeviceType::GameBoyColor, EmulationType::DMG) => {
                let b = title_checksum;

                let (h, l) = match b {
                    0x43 | 0x58 => (0x99, 0x1a), _ => (0x00, 0x7c)
                };

                (0x11, true, false, false, false, b, 0x00, 0x00, 0x08, h, l)
            }

            // GameBoy Color with a cartridge using the GBC extensions
            (DeviceType::GameBoyColor, EmulationType::GBC) => {
                (0x11, true, false, false, false, 0x00, 0x00, 0xff, 0x56, 0x00, 0x0d)
            }

            // GameBoy Advance with classic GameBoy cartridge
            (DeviceType::GameBoyAdvance, EmulationType::DMG) |
            (DeviceType::GameBoyAdvanceSP, EmulationType::DMG) => {
                let (b, flag_h, _) = carrying_add_u8(title_checksum, 1, false);
                let flag_z = b == 0;

                let (h, l) = match b {
                    0x44 | 0x59 => (0x99, 0x1a), _ => (0x00, 0x7c)
                };

                (0x11, flag_z, false, flag_h, false, b, 0x00, 0x00, 0x08, h, l)
            }

            // GameBoy Advance with a cartridge using GBC extensions
            (DeviceType::GameBoyAdvance, EmulationType::GBC) |
            (DeviceType::GameBoyAdvanceSP, EmulationType::GBC) => {
                (0x11, false, false, false, false, 0x01, 0x00, 0xff, 0x56, 0x00, 0x0d)
            }

            // SuperGameBoy with any cartridge?
            (DeviceType::SuperGameBoy, _) => {
                (0x01, false, false, false, false, 0x00, 0x14, 0x00, 0x00, 0xc0, 0x60)
            }

            // SuperGameBoy with any cartridge?
            (DeviceType::SuperGameBoy2, _) => {
                (0xff, false, false, false, false, 0x00, 0x14, 0x00, 0x00, 0xc0, 0x60)
            }
        };

        // build flags register
        let f = 0
            |   (if flag_z { 1 << CpuFlag::Zero.bit() }      else { 0 })
            |   (if flag_n { 1 << CpuFlag::Negative.bit() }  else { 0 })
            |   (if flag_h { 1 << CpuFlag::HalfCarry.bit() } else { 0 })
            |   (if flag_c { 1 << CpuFlag::Carry.bit() }     else { 0 })
        ;

        // update registers with the desired values
        self.cpu.set_r8(RegisterR8::A, a);
        self.cpu.set_r8(RegisterR8::F, f);
        self.cpu.set_r8(RegisterR8::B, b);
        self.cpu.set_r8(RegisterR8::C, c);
        self.cpu.set_r8(RegisterR8::D, d);
        self.cpu.set_r8(RegisterR8::E, e);
        self.cpu.set_r8(RegisterR8::H, h);
        self.cpu.set_r8(RegisterR8::L, l);
        self.cpu.set_instruction_pointer(pc);
        self.cpu.set_stack_pointer(sp);

        // initialize IO registers
        {
            // placeholder for unknown/unused entries
            const X : u8 = 0xff;

            let dma = match self.device_config.device {
                DeviceType::GameBoyColor | DeviceType::GameBoyAdvance => 0x00,
                _ => 0xff,
            };
            
            // set the initial vram bank index to 0 on GBC
            let vbk = match self.device_config.emulation {
                EmulationType::DMG => 0xff,
                EmulationType::GBC => 0xfe,
            };

            // GBC prefers object priority by OAM index, DMG by sprite x position
            let opri = match self.device_config.emulation {
                EmulationType::DMG => 0xff,
                EmulationType::GBC => 0xfe,
            };

            // Timer, LCD-STAT and LY depends on how long the boot rom took for execution
            let (timer_counter, tac, lcds, ly) = match self.device_config.device {
                _ => (0xabf0, 0xf8, 0x85, 0x00)
            };

            // div depends on the high byte of the timer counter
            let div = get_high(timer_counter);

            let io_reg_data : [u8; 256] = [
                /*          0     1     2     3     4     5     6     7     8     9     a     b     c     d     e     f */
                /* 00 */ 0xcf, 0x00, 0x7e,    X,  div, 0x00, 0x00,  tac,    X,    X,    X,    X,    X,    X,    X, 0xe1,
                /* 10 */ 0x80, 0xbf, 0xf3, 0xff, 0xbf,    X, 0x3f, 0x00, 0xff, 0xbf, 0x7f, 0xff, 0x9f, 0xff, 0xbf,    X,
                /* 20 */ 0xff, 0x00, 0x00, 0xbf, 0x77, 0xf3, 0xf1,    X,    X,    X,    X,    X,    X,    X,    X,    X,
                /* 30 */    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,
                /* 40 */ 0x91, lcds, 0x00, 0x00,   ly, 0x00,  dma, 0xfc, 0x00, 0x00, 0x00, 0x00,    X, 0xff,    X,  vbk,
                /* 50 */    X, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,    X,    X,    X,    X,    X,    X,    X,    X,    X,
                /* 60 */    X,    X,    X,    X,    X,    X,    X,    X, 0xff, 0xff, 0xff, 0xff, opri,    X,    X,    X,
                /* 70 */ 0xff,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,
                /* 80 */    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,
                /* 90 */    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,
                /* a0 */    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,
                /* b0 */    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,
                /* c0 */    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,
                /* d0 */    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,
                /* e0 */    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,
                /* f0 */    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X,    X, 0x00,
            ];

            // apply selected values
            for i in 0..=255 {
                self.get_mmu_mut().write_u8(0xff00 + i, io_reg_data[i as usize]);
            }

            self.get_peripherals_mut().timer.initialize_counter(timer_counter, tac);
        }
    }


    /// Get the number of cycles processed by the emulator since it started.
    pub fn get_total_cycles_processed(&self) -> Clock {
        self.total_cycles
    }


    /// Get the time in seconds the emulator did run.
    pub fn get_total_seconds_processed(&self) -> f32 {
        (self.total_cycles as f32) / (CPU_CLOCK_SPEED as f32)
    }


    /// Get the device MMU.
    pub fn get_mmu(&self) -> &Mmu {
        self.cpu.get_mmu()
    }


    /// Get the device MMU.
    pub fn get_mmu_mut(&mut self) -> &mut Mmu {
        self.cpu.get_mmu_mut()
    }


    /// Get the device peripheral components.
    pub fn get_peripherals(&self) -> &Peripherals {
        self.get_mmu().get_peripherals()
    }


    /// Get the device peripheral components.
    pub fn get_peripherals_mut(&mut self) -> &mut Peripherals {
        self.get_mmu_mut().get_peripherals_mut()
    }


    /// Runs the emulator for a single step, either an instruction
    /// or to process a single HALT cycle.
    pub fn run_single_step(&mut self) -> EmulatorUpdateResults {
        self.process_next()
    }


    /// Continues running the program located on the cartridge,
    /// until the PPU has completed one single frame.
    pub fn run_frame(&mut self) -> EmulatorUpdateResults {
        let mut results = EmulatorUpdateResults::default();

        // update until receiving the 'frame completed' event.
        loop {
            results += self.process_next();

            // stop after completing one frame
            if results.events.contains(DebugEvent::PpuFrameCompleted) {
                break;
            }

            // inc ase the screen was disabled, stop after the time of one frame has passed
            if results.cycles >= CPU_CYCLES_PER_FRAME {
                break;
            }
        }

        results
    }


    /// Continues processing the next pending operation.
    fn process_next(&mut self) -> EmulatorUpdateResults {
        if self.cpu.is_running() {
            if let Some(cycles) = self.cpu.handle_interrupts() {
                let signals = self.update_components(cycles);

                EmulatorUpdateResults {
                    cycles,
                    events: signals.events,
                }
            }
            else {
                self.process_next_opcode()
            }
        }
        else {
            // when in HALT state just pass 4 cycles
            // where the CPU idles
            let halt_cycle = 4;
            let signals    = self.update_components(halt_cycle);

            EmulatorUpdateResults {
                cycles: halt_cycle,
                events: signals.events,
            }
        }
    }


    /// Process the next opcode.
    fn process_next_opcode(&mut self) -> EmulatorUpdateResults {
        let instruction = self.cpu.fetch_next_instruction();
        let mut context = OpCodeContext::for_instruction(&instruction);
        let mut signals = MemoryBusSignals::default();
        let mut total_step_cycles : Clock = 0;

        // process cycles ahead of the actual opcode execution to get read/write operations
        // to be invoked on their expected cycle
        if instruction.opcode.cycles_ahead != 0 {
            let cycles_ahead = instruction.opcode.cycles_ahead;
            total_step_cycles += cycles_ahead;
            signals |= self.update_components(cycles_ahead);
        }

        loop {
            // invoke opcode execution
            let result = (instruction.opcode.proc)(self, &mut context);

            match result {
                // the opcode was partially executed and needs time to pass on other components
                // to update timer or memory operations.
                OpCodeResult::StageDone(step_cycles) => {
                    total_step_cycles += step_cycles;
                    signals |= self.update_components(step_cycles);
                    context.enter_next_stage();
                }

                // the opcode is completed. the remaining time needs to be applied on components.
                OpCodeResult::Done => {
                    // get the total number of cycles consumed by this opcode and subtract the
                    // number of cycles already applied to components
                    let remaining_cycles = context.get_cycles_consumed() - total_step_cycles;
                    signals |= self.update_components(remaining_cycles);

                    break;
                }
            }
        }

        // print opcode and CPU state if enabled
        #[cfg(feature = "std")]
        if self.device_config.print_opcodes {
            println!(
                "/* {:04x} [{:02x}]{} */ {:<16}    ; {}",
                instruction.opcode_address,
                instruction.opcode_id,
                if instruction.opcode_id <= 0xff { "  " } else { "" },
                instruction.to_string(),
                self.cpu
            );
        }

        EmulatorUpdateResults {
            cycles: context.get_cycles_consumed(),
            events: signals.events,
        }
    }


    /// Applies the time passed during CPU execution to other components as well.
    #[must_use]
    fn update_components(&mut self, cycles: Clock) -> MemoryBusSignals {
        self.cpu.update(cycles);
        self.get_mmu_mut().update(cycles);
        #[cfg(feature = "apu")]
        self.get_peripherals_mut().apu.update(cycles);
        self.get_peripherals_mut().ppu.update(cycles);
        self.get_peripherals_mut().timer.update(cycles);
        self.get_peripherals_mut().serial.update(cycles);
        self.get_peripherals_mut().input.update();

        // collects all signals received from components
        let signals =
                self.get_peripherals_mut().apu.take_signals()
            |   self.get_peripherals_mut().ppu.take_signals()
            |   self.get_peripherals_mut().timer.take_signals()
            |   self.get_peripherals_mut().serial.take_signals()
            |   self.get_peripherals_mut().input.take_signals()
        ;

        // forward all requested interrupts into the Interrupts component.
        self.get_peripherals_mut().interrupts.request_interrupts(signals.interrupts);

        // increment clock counters
        self.total_cycles += cycles;

        signals
    }
}


impl core::ops::Add for EmulatorUpdateResults {
    type Output = EmulatorUpdateResults;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            cycles: self.cycles + rhs.cycles,
            events: self.events | rhs.events,
        }
    }
}


impl core::ops::AddAssign for EmulatorUpdateResults {
    fn add_assign(&mut self, rhs: Self) {
        self.cycles += rhs.cycles;
        self.events |= rhs.events;
    }
}
