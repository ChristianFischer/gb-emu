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

#[cfg(feature = "std")]
use std::fmt::{Display, Formatter};

use crate::cpu::opcode::{Instruction, OpCode};
use crate::cpu::opcodes::{OPCODE_TABLE, OPCODE_TABLE_EXTENDED};
use crate::gameboy::Clock;
use crate::mmu::mmu::Mmu;
use crate::utils::{change_bit, get_bit, to_u16, to_u8};


/// Number of cycles per second.
pub const CPU_CLOCK_SPEED: Clock = 4_194_304;


/// Definition for each supported 8 bit Register.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RegisterR8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

/// Definition for each supported 16 bit Register.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RegisterR16 {
    AF,
    BC,
    DE,
    HL,
}

/// A list of CPU flags.
#[derive(Copy, Clone)]
pub enum CpuFlag {
    Zero,
    Negative,
    HalfCarry,
    Carry,
}

/// State of the interrupts enabled flag.
#[derive(Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImeState {
    /// Interrupts are globally disabled.
    Disabled,

    /// Interrupts are globally enabled.
    Enabled,

    /// Interrupts are disabled, but will be enabled after some CPU cycles.
    EnabledInCycles(Clock),
}

/// Determines the CPU's state, when suspended by the HALT instruction.
#[derive(Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HaltState {
    /// The CPU is running normally.
    Running,

    /// The CPU was suspended by the HALT command.
    Halt,
}

/// An object representing the gameboy's CPU
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cpu {
    /// Interface to the device memory.
    mmu: Mmu,

    /// All CPU registers as 8 bit value each.
    /// To access 16 bit registers there is a set of functions.
    registers: [u8; 8],

    /// Offset where to read the next instruction.
    instruction_pointer: u16,

    /// Offset where to read the next value from the stack.
    stack_pointer: u16,

    /// Temporary storage to store intermediate results for
    /// opcodes, which need to be processed in multiple stages.
    intermediate_value: u8,

    /// The state whether interrupts are enabled or not.
    ime: ImeState,

    /// The state whether the CPU was suspended by the HALT command.
    halt: HaltState,
}


impl RegisterR16 {
    /// Get the 8 bit registers which contains the high and low bytes of a 16 bit register.
    pub const fn to_r8(self) -> (RegisterR8, RegisterR8) {
        match self {
            RegisterR16::AF => (RegisterR8::A, RegisterR8::F),
            RegisterR16::BC => (RegisterR8::B, RegisterR8::C),
            RegisterR16::DE => (RegisterR8::D, RegisterR8::E),
            RegisterR16::HL => (RegisterR8::H, RegisterR8::L),
        }
    }

    /// Get the 8 bit register which contains the high byte of a 16 bit register.
    pub const fn get_high(self) -> RegisterR8 {
        match self {
            RegisterR16::AF => RegisterR8::A,
            RegisterR16::BC => RegisterR8::B,
            RegisterR16::DE => RegisterR8::D,
            RegisterR16::HL => RegisterR8::H,
        }
    }

    /// Get the 8 bit register which contains the low byte of a 16 bit register.
    pub const fn get_low(self) -> RegisterR8 {
        match self {
            RegisterR16::AF => RegisterR8::F,
            RegisterR16::BC => RegisterR8::C,
            RegisterR16::DE => RegisterR8::E,
            RegisterR16::HL => RegisterR8::L,
        }
    }
}


#[cfg(feature = "std")]
impl Display for RegisterR8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            RegisterR8::A => "A",
            RegisterR8::F => "F",
            RegisterR8::B => "B",
            RegisterR8::C => "C",
            RegisterR8::D => "D",
            RegisterR8::E => "E",
            RegisterR8::H => "H",
            RegisterR8::L => "L",
        })
    }
}


#[cfg(feature = "std")]
impl Display for RegisterR16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            RegisterR16::AF => "AF",
            RegisterR16::BC => "BC",
            RegisterR16::DE => "DE",
            RegisterR16::HL => "HL",
        })
    }
}


impl CpuFlag {
    /// Get the bit inside the flags register which stores
    /// the according CPU flag value.
    pub fn bit(&self) -> u8 {
        match self {
            CpuFlag::Zero      => 7,
            CpuFlag::Negative  => 6,
            CpuFlag::HalfCarry => 5,
            CpuFlag::Carry     => 4,
        }
    }
}


impl HaltState {
    /// Checks whether the CPU is running in the according state.
    pub fn is_cpu_running(&self) -> bool {
        match self {
            HaltState::Running => true,
            HaltState::Halt => false,
        }
    }
}


impl Cpu {
    /// Creates an empty CPU object.
    pub fn new(mmu: Mmu) -> Cpu {
        Cpu {
            mmu,

            registers: [0; 8],

            instruction_pointer: 0x0100,
            stack_pointer: 0x0000,
            intermediate_value: 0x0000,

            ime:  ImeState::Disabled,
            halt: HaltState::Running,
        }
    }

    /// Checks whether the CPU is currently running or being suspended by HALT state.
    pub fn is_running(&self) -> bool {
        self.halt.is_cpu_running()
    }

    /// Let the CPU process their data.
    /// This function takes the amount of ticks to be processed.
    pub fn update(&mut self, cycles: Clock) {
        self.handle_halt_state();
        self.handle_ime_pending(cycles);
    }

    /// Checks the current HALT state and check
    /// if the state will be left when interrupts are pending.
    fn handle_halt_state(&mut self) {
        match self.halt {
            HaltState::Halt => {
                if self.get_mmu().get_peripherals().interrupts.has_interrupts_pending() {
                    self.halt = HaltState::Running;
                }
            },

            _ => { }
        }
    }

    /// Handles any pending interrupts.
    pub fn handle_interrupts(&mut self) -> Option<Clock> {
        match self.ime {
            ImeState::Enabled => {
                let cpu_state = &mut self.get_mmu_mut().get_peripherals_mut().interrupts;

                if let Some(interrupt) = cpu_state.take_pending_interrupt() {
                    // disable further interrupts when a interrupt is being handled
                    self.ime = ImeState::Disabled;

                    // call the address of the interrupt
                    self.call_addr(interrupt.address());

                    // stop handling other interrupts
                    return Some(20);
                }
            },

            _ => { },
        }

        None
    }

    /// Handles IME state.
    fn handle_ime_pending(&mut self, cycles: Clock) {
        match self.ime {
            ImeState::EnabledInCycles(timeout) => {
                let new_timeout = timeout.saturating_sub(cycles);

                if new_timeout == 0 {
                    self.ime = ImeState::Enabled;
                }
                else {
                    self.ime = ImeState::EnabledInCycles(new_timeout);
                }
            }

            _ => { }
        }
    }

    /// Enables interrupts.
    pub fn enable_interrupts(&mut self) {
        self.ime = ImeState::Enabled;
    }

    /// Enables interrupts after a number of cycles passed.
    pub fn enable_interrupts_in(&mut self, cycles: Clock) {
        match self.ime {
            ImeState::Disabled | ImeState::EnabledInCycles(_) => {
                self.ime = ImeState::EnabledInCycles(cycles);
            }

            _ => { }
        }
    }

    /// Disables interrupts.
    pub fn disable_interrupts(&mut self) {
        self.ime = ImeState::Disabled;
    }

    /// Checks if interrupts are globally enabled.
    pub fn is_interrupts_enabled(&self) -> bool {
        match self.ime {
            ImeState::Enabled => true,
            _ => false,
        }
    }

    /// Enters the HALT mode.
    pub fn enter_halt_mode(&mut self) {
        match self.ime {
            ImeState::EnabledInCycles(_) => {
                // when interrupts are about to be enabled, enable them immediately
                // and revert the program counter to repeat the HALT instruction
                // after interrupts were handled.
                self.ime = ImeState::Enabled;
                self.instruction_pointer -= 1;
            }

            _ => {
                self.halt = HaltState::Halt;
            }
        }
    }


    /// Get access to the memory unit linked to the CPU.
    pub fn get_mmu(&self) -> &Mmu {
        &self.mmu
    }

    /// Get access to the memory unit linked to the CPU.
    pub fn get_mmu_mut(&mut self) -> &mut Mmu {
        &mut self.mmu
    }


    /// Fetches the next opcode on the current location of the instruction pointer.
    /// The instruction pointer will be forwarded to the next instruction.
    pub fn fetch_next_opcode(&mut self) -> &'static OpCode {
        let opcode_byte = self.fetch_u8();
        if opcode_byte != 0xCB {
            &OPCODE_TABLE[opcode_byte as usize]
        }
        else {
            let opcode_byte_extended = self.fetch_u8();
            &OPCODE_TABLE_EXTENDED[opcode_byte_extended as usize]
        }
    }

    /// Fetches the next instruction on the current location of the instruction pointer.
    /// The instruction pointer will be forwarded to the next instruction.
    pub fn fetch_next_instruction(&mut self) -> Instruction {
        let opcode_address = self.instruction_pointer;
        let opcode_byte    = self.get_next_byte() as u16;
        let opcode_id      = if opcode_byte == 0xCB { self.get_next_u16() } else { opcode_byte };
        let opcode         = self.fetch_next_opcode();

        Instruction {
            opcode,
            opcode_id,
            opcode_address,
            arg: [
                self.mmu.read_u8(self.instruction_pointer),
                self.mmu.read_u8(self.instruction_pointer.wrapping_add(1))
            ]
        }
    }

    /// Get the next byte on the current location of the instruction pointer, without moving it.
    pub fn get_next_byte(&self) -> u8 {
        self.mmu.read_u8(self.instruction_pointer)
    }

    /// Get the next byte relative to the current location of the instruction pointer, without moving it.
    pub fn get_next_byte_at(&self, offset: u16) -> u8 {
        self.mmu.read_u8(self.instruction_pointer + offset)
    }

    /// Get the next i8 value on the current location of the instruction pointer, without moving it.
    pub fn get_next_i8(&self) -> i8 {
        self.get_next_byte() as i8
    }

    /// Get the next u8 value on the current location of the instruction pointer, without moving it.
    pub fn get_next_u8(&self) -> u8 {
        self.get_next_byte()
    }

    /// Get the next u16 value on the current location of the instruction pointer, without moving it.
    pub fn get_next_u16(&self) -> u16 {
        let low  = self.get_next_byte_at(0);
        let high = self.get_next_byte_at(1);
        to_u16(high, low)
    }

    /// Fetches the next u8 value on the current location of the instruction pointer.
    /// The instruction pointer will be forwarded to the next instruction.
    pub fn fetch_u8(&mut self) -> u8 {
        let value = self.get_next_byte();
        self.instruction_pointer += 1;
        value
    }

    /// Fetches the next u16 value on the current location of the instruction pointer.
    /// The instruction pointer will be forwarded to the next instruction.
    pub fn fetch_u16(&mut self) -> u16 {
        let low  = self.fetch_u8();
        let high = self.fetch_u8();
        to_u16(high, low)
    }

    /// Fetches the next u8 value on the current location of the instruction pointer.
    /// The instruction pointer will be forwarded to the next instruction.
    pub fn fetch_i8(&mut self) -> i8 {
        self.fetch_u8() as i8
    }

    /// Fetches the next u8 value on the current location of the instruction pointer.
    /// The instruction pointer will be forwarded to the next instruction.
    pub fn fetch_i16(&mut self) -> i16 {
        self.fetch_u16() as i16
    }

    /// Pushes a 8bit value on the stack, moving the stack pointer.
    pub fn push_u8(&mut self, value: u8) {
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.mmu.write_u8(self.stack_pointer, value);
    }

    /// Pushes a 16bit value on the stack, moving the stack pointer.
    pub fn push_u16(&mut self, value: u16) {
        let (high, low) = to_u8(value);
        self.push_u8(high);
        self.push_u8(low);
    }

    /// Pops a 8bit value from the stack, moving the stack pointer.
    pub fn pop_u8(&mut self) -> u8 {
        let value = self.mmu.read_u8(self.stack_pointer);
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        value
    }

    /// Pops a 8bit value from the stack, moving the stack pointer.
    pub fn pop_u16(&mut self) -> u16 {
        let low  = self.pop_u8();
        let high = self.pop_u8();
        to_u16(high, low)
    }

    /// Get the value of a 8 bit register.
    pub fn get_r8(&self, register: RegisterR8) -> u8 {
        self.registers[register as usize]
    }

    /// Set the value of a 8 bit register.
    pub fn set_r8(&mut self, register: RegisterR8, value: u8) {
        self.registers[register as usize] = value;
    }

    /// Get the value of a 16 bit register.
    pub fn get_r16(&self, register: RegisterR16) -> u16 {
        let (high_r8, low_r8) = register.to_r8();
        let high = self.registers[high_r8 as usize];
        let low  = self.registers[low_r8 as usize];
        to_u16(high, low)
    }

    /// Set the value of a 16 bit register.
    pub fn set_r16(&mut self, register: RegisterR16, value: u16) {
        let (high_r8, low_r8) = register.to_r8();
        let (high, low) = to_u8(value);
        self.registers[high_r8 as usize] = high;
        self.registers[low_r8 as usize]  = low;
    }

    /// Checks whether a specific CPU flag is set.
    pub fn is_flag_set(&self, flag: CpuFlag) -> bool {
        get_bit(self.get_r8(RegisterR8::F), flag.bit())
    }

    /// Set the value of a CPU flag.
    pub fn set_flag(&mut self, flag: CpuFlag, value: bool) {
        let value_old = self.get_r8(RegisterR8::F);
        let value_new = change_bit(value_old, flag.bit(), value);
        self.set_r8(RegisterR8::F, value_new);
    }

    /// Clear all CPU flags.
    pub fn clear_flags(&mut self) {
        self.set_r8(RegisterR8::F, 0x00);
    }

    /// Moves the instruction pointer relative to it's current position.
    pub fn jump_relative(&mut self, offset: i16) {
        if offset < 0 {
            self.instruction_pointer -= (-offset) as u16;
        }
        else {
            self.instruction_pointer += offset as u16;
        }
    }

    /// Moves the instruction pointer to a fixed location.
    pub fn jump_to(&mut self, address: u16) {
        self.set_instruction_pointer(address);
    }

    /// Performs a call to a given address.
    /// Saves the current instruction pointer on the stack and then moves
    /// the instruction pointer to the new address.
    pub fn call_addr(&mut self, address: u16) {
        let instruction_pointer = self.get_instruction_pointer();
        self.push_u16(instruction_pointer);
        self.set_instruction_pointer(address);
    }

    /// Returns from a previous call.
    /// Reads the value of the instruction pointer from the stack.
    pub fn ret_from_call(&mut self) {
        let instruction_pointer = self.pop_u16();
        self.set_instruction_pointer(instruction_pointer);
    }

    /// Get the current address of the instruction pointer.
    pub fn get_instruction_pointer(&self) -> u16 {
        self.instruction_pointer
    }

    /// Set the current address of the instruction pointer.
    pub fn set_instruction_pointer(&mut self, address: u16) {
        self.instruction_pointer = address;
    }

    /// Get the current address of the stack pointer.
    pub fn get_stack_pointer(&self) -> u16 {
        self.stack_pointer
    }

    /// Set the current address of the stack pointer.
    pub fn set_stack_pointer(&mut self, address: u16) {
        self.stack_pointer = address;
    }

    /// Get the intermediate value.
    pub fn get_intermediate_value(&self) -> u8 {
        self.intermediate_value
    }

    /// Set the intermediate value.
    pub fn set_intermediate_value(&mut self, value: u8) {
        self.intermediate_value = value;
    }

    /// Creates a string representation of the current CPU flags.
    #[cfg(feature = "std")]
    pub fn flags_to_string(&self) -> String {
        format!(
            "{}{}{}{}",
            if self.is_flag_set(CpuFlag::Zero)      { 'Z' } else { '-' },
            if self.is_flag_set(CpuFlag::Negative)  { 'N' } else { '-' },
            if self.is_flag_set(CpuFlag::HalfCarry) { 'H' } else { '-' },
            if self.is_flag_set(CpuFlag::Carry)     { 'C' } else { '-' }
        )
    }

    /// Creates a string representation of the current CPU registers.
    #[cfg(feature = "std")]
    pub fn registers_to_string(&self) -> String {
        format!(
            "A={:02x} F={:02x} B={:02x} C={:02x} D={:02x} E={:02x} H={:02x} L={:02x} SP={:04x} IP={:04x}",
            self.get_r8(RegisterR8::A),
            self.get_r8(RegisterR8::F),
            self.get_r8(RegisterR8::B),
            self.get_r8(RegisterR8::C),
            self.get_r8(RegisterR8::D),
            self.get_r8(RegisterR8::E),
            self.get_r8(RegisterR8::H),
            self.get_r8(RegisterR8::L),
            self.get_stack_pointer(),
            self.get_instruction_pointer()
        )
    }
}


#[cfg(feature = "std")]
impl Display for Cpu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.flags_to_string(),
            self.registers_to_string(),
        )
    }
}
