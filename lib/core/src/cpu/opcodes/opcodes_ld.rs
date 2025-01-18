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

use crate::cpu::cpu::{CpuFlag, RegisterR16, RegisterR8};
use crate::cpu::opcode::{opcode, OpCodeContext};
use crate::emulator_core::EmulatorCore;
use crate::utils::signed_overflow_add_u16;

/// Loads the content of a 8bit register into another one.
fn ld_r8_r8(gb: &mut EmulatorCore, dst: RegisterR8, src: RegisterR8) {
    let value = gb.cpu.get_r8(src);
    gb.cpu.set_r8(dst, value);
}

/// Loads a constant 8bit value from the current instruction pointer into a 8bit register.
fn ld_r8_u8(gb: &mut EmulatorCore, dst: RegisterR8) {
    let value = gb.cpu.fetch_u8();
    gb.cpu.set_r8(dst, value);
}

/// Loads the content of a 8bit register into the device memory.
fn ld_addr_r8(gb: &mut EmulatorCore, dst_address: u16, src: RegisterR8) {
    let value = gb.cpu.get_r8(src);
    gb.get_mmu_mut().write_u8(dst_address, value);
}

/// Writes a 8bit value to a given address in the device memory.
fn ld_addr_u8(gb: &mut EmulatorCore, dst_address: u16, value: u8) {
    gb.get_mmu_mut().write_u8(dst_address, value);
}

/// Writes a 8bit value to a given address in the device memory.
fn ld_addr_u16(gb: &mut EmulatorCore, dst_address: u16, value: u16) {
    gb.get_mmu_mut().write_u16(dst_address, value);
}

/// Loads the value on a given address into a 8bit register.
fn ld_r8_addr(gb: &mut EmulatorCore, dst: RegisterR8, src_address: u16) {
    let value = gb.get_mmu().read_u8(src_address);
    gb.cpu.set_r8(dst, value);
}

/// Loads the content of a 8bit register into the address stored in the target R16 register.
fn ld_r16ptr_r8(gb: &mut EmulatorCore, dst: RegisterR16, src: RegisterR8) {
    let address = gb.cpu.get_r16(dst);
    ld_addr_r8(gb, address, src);
}

/// Loads the content of a 8bit register into a constant address of the device memory.
fn ld_u16ptr_r8(gb: &mut EmulatorCore, src: RegisterR8) {
    let address = gb.cpu.fetch_u16();
    ld_addr_r8(gb, address, src);
}

/// Loads a constant 8bit value from the current instruction pointer
/// into the address stored in the target R16 register.
fn ld_r16ptr_u8(gb: &mut EmulatorCore, dst: RegisterR16) {
    let address = gb.cpu.get_r16(dst);
    let value   = gb.cpu.fetch_u8();
    ld_addr_u8(gb, address, value);
}

/// Loads a 16bit value into a constant address of the device memory.
fn ld_u16ptr_u16v(gb: &mut EmulatorCore, value: u16) {
    let address = gb.cpu.fetch_u16();
    ld_addr_u16(gb, address, value);
}

/// Loads the value at the address stored in a 16bit register
/// into a 8bit register.
fn ld_r8_r16ptr(gb: &mut EmulatorCore, dst: RegisterR8, src: RegisterR16) {
    let address = gb.cpu.get_r16(src);
    ld_r8_addr(gb, dst, address);
}

/// Loads the value at the address stored in a 16bit constant
/// into a 8bit register.
fn ld_r8_u16ptr(gb: &mut EmulatorCore, dst: RegisterR8) {
    let address = gb.cpu.fetch_u16();
    ld_r8_addr(gb, dst, address);
}

/// Loads a constant 16bit value from the current instruction pointer into a 16bit register.
fn ld_r16_u16(gb: &mut EmulatorCore, dst: RegisterR16) {
    let value = gb.cpu.fetch_u16();
    gb.cpu.set_r16(dst, value);
}

/// Loads the value of a 8bit register into the device memory at the address (0xff00 + u8).
fn ldh_u8_r8(gb: &mut EmulatorCore, src: RegisterR8) {
    let value     = gb.cpu.get_r8(src);
    let address_h = gb.cpu.fetch_u8();
    let address   = 0xff00 | (address_h as u16);
    gb.get_mmu_mut().write_u8(address, value);
}

/// Loads a value from the device memory at the address (0xff00 + u8) into a 8bit register.
fn ldh_r8_u8(gb: &mut EmulatorCore, dst: RegisterR8) {
    let address_h = gb.cpu.fetch_u8();
    let address   = 0xff00 | (address_h as u16);
    let value     = gb.get_mmu().read_u8(address);
    gb.cpu.set_r8(dst, value);
}

/// Loads a value from a 8bit register into the device memory at the address (0xff00 + r8)
fn ldh_r8ptr_r8(gb: &mut EmulatorCore, dst_ptr: RegisterR8, src: RegisterR8) {
    let address_h = gb.cpu.get_r8(dst_ptr);
    let address   = 0xff00 | (address_h as u16);
    let value     = gb.cpu.get_r8(src);
    gb.get_mmu_mut().write_u8(address, value);
}

/// Loads a value from the device memory at the address (0xff00 + r8) into a 8bit register.
fn ldh_r8_r8ptr(gb: &mut EmulatorCore, dst: RegisterR8, src_ptr: RegisterR8) {
    let address_h = gb.cpu.get_r8(src_ptr);
    let address   = 0xff00 | (address_h as u16);
    let value     = gb.get_mmu().read_u8(address);
    gb.cpu.set_r8(dst, value);
}

/// Pushes the value of a 16bit register on the stack.
fn push_r16(gb: &mut EmulatorCore, r16: RegisterR16) {
    let value = gb.cpu.get_r16(r16);
    gb.cpu.push_u16(value);
}

/// Pops a 16bit value from the stack into a 16bit register.
fn pop_r16(gb: &mut EmulatorCore, r16: RegisterR16) {
    let value = gb.cpu.pop_u16();
    gb.cpu.set_r16(r16, value);
}

/// Pops a 16bit value from the stack into a 16bit register.
/// Applies a bitmask to the value before writing into the register.
fn pop_r16_mask(gb: &mut EmulatorCore, r16: RegisterR16, mask: u16) {
    let value = gb.cpu.pop_u16();
    let value_masked = value & mask;
    gb.cpu.set_r16(r16, value_masked);
}


// LD r8, r8
opcode!(ld_a_a, [gb] ld_r8_r8(gb, RegisterR8::A, RegisterR8::A));
opcode!(ld_a_b, [gb] ld_r8_r8(gb, RegisterR8::A, RegisterR8::B));
opcode!(ld_a_c, [gb] ld_r8_r8(gb, RegisterR8::A, RegisterR8::C));
opcode!(ld_a_d, [gb] ld_r8_r8(gb, RegisterR8::A, RegisterR8::D));
opcode!(ld_a_e, [gb] ld_r8_r8(gb, RegisterR8::A, RegisterR8::E));
opcode!(ld_a_l, [gb] ld_r8_r8(gb, RegisterR8::A, RegisterR8::L));
opcode!(ld_a_h, [gb] ld_r8_r8(gb, RegisterR8::A, RegisterR8::H));

// LD r8, r8
opcode!(ld_b_a, [gb] ld_r8_r8(gb, RegisterR8::B, RegisterR8::A));
opcode!(ld_b_b, [gb] ld_r8_r8(gb, RegisterR8::B, RegisterR8::B));
opcode!(ld_b_c, [gb] ld_r8_r8(gb, RegisterR8::B, RegisterR8::C));
opcode!(ld_b_d, [gb] ld_r8_r8(gb, RegisterR8::B, RegisterR8::D));
opcode!(ld_b_e, [gb] ld_r8_r8(gb, RegisterR8::B, RegisterR8::E));
opcode!(ld_b_l, [gb] ld_r8_r8(gb, RegisterR8::B, RegisterR8::L));
opcode!(ld_b_h, [gb] ld_r8_r8(gb, RegisterR8::B, RegisterR8::H));

// LD r8, r8
opcode!(ld_c_a, [gb] ld_r8_r8(gb, RegisterR8::C, RegisterR8::A));
opcode!(ld_c_b, [gb] ld_r8_r8(gb, RegisterR8::C, RegisterR8::B));
opcode!(ld_c_c, [gb] ld_r8_r8(gb, RegisterR8::C, RegisterR8::C));
opcode!(ld_c_d, [gb] ld_r8_r8(gb, RegisterR8::C, RegisterR8::D));
opcode!(ld_c_e, [gb] ld_r8_r8(gb, RegisterR8::C, RegisterR8::E));
opcode!(ld_c_l, [gb] ld_r8_r8(gb, RegisterR8::C, RegisterR8::L));
opcode!(ld_c_h, [gb] ld_r8_r8(gb, RegisterR8::C, RegisterR8::H));

// LD r8, r8
opcode!(ld_d_a, [gb] ld_r8_r8(gb, RegisterR8::D, RegisterR8::A));
opcode!(ld_d_b, [gb] ld_r8_r8(gb, RegisterR8::D, RegisterR8::B));
opcode!(ld_d_c, [gb] ld_r8_r8(gb, RegisterR8::D, RegisterR8::C));
opcode!(ld_d_d, [gb] ld_r8_r8(gb, RegisterR8::D, RegisterR8::D));
opcode!(ld_d_e, [gb] ld_r8_r8(gb, RegisterR8::D, RegisterR8::E));
opcode!(ld_d_l, [gb] ld_r8_r8(gb, RegisterR8::D, RegisterR8::L));
opcode!(ld_d_h, [gb] ld_r8_r8(gb, RegisterR8::D, RegisterR8::H));

// LD r8, r8
opcode!(ld_e_a, [gb] ld_r8_r8(gb, RegisterR8::E, RegisterR8::A));
opcode!(ld_e_b, [gb] ld_r8_r8(gb, RegisterR8::E, RegisterR8::B));
opcode!(ld_e_c, [gb] ld_r8_r8(gb, RegisterR8::E, RegisterR8::C));
opcode!(ld_e_d, [gb] ld_r8_r8(gb, RegisterR8::E, RegisterR8::D));
opcode!(ld_e_e, [gb] ld_r8_r8(gb, RegisterR8::E, RegisterR8::E));
opcode!(ld_e_l, [gb] ld_r8_r8(gb, RegisterR8::E, RegisterR8::L));
opcode!(ld_e_h, [gb] ld_r8_r8(gb, RegisterR8::E, RegisterR8::H));

// LD r8, r8
opcode!(ld_l_a, [gb] ld_r8_r8(gb, RegisterR8::L, RegisterR8::A));
opcode!(ld_l_b, [gb] ld_r8_r8(gb, RegisterR8::L, RegisterR8::B));
opcode!(ld_l_c, [gb] ld_r8_r8(gb, RegisterR8::L, RegisterR8::C));
opcode!(ld_l_d, [gb] ld_r8_r8(gb, RegisterR8::L, RegisterR8::D));
opcode!(ld_l_e, [gb] ld_r8_r8(gb, RegisterR8::L, RegisterR8::E));
opcode!(ld_l_l, [gb] ld_r8_r8(gb, RegisterR8::L, RegisterR8::L));
opcode!(ld_l_h, [gb] ld_r8_r8(gb, RegisterR8::L, RegisterR8::H));

// LD r8, r8
opcode!(ld_h_a, [gb] ld_r8_r8(gb, RegisterR8::H, RegisterR8::A));
opcode!(ld_h_b, [gb] ld_r8_r8(gb, RegisterR8::H, RegisterR8::B));
opcode!(ld_h_c, [gb] ld_r8_r8(gb, RegisterR8::H, RegisterR8::C));
opcode!(ld_h_d, [gb] ld_r8_r8(gb, RegisterR8::H, RegisterR8::D));
opcode!(ld_h_e, [gb] ld_r8_r8(gb, RegisterR8::H, RegisterR8::E));
opcode!(ld_h_l, [gb] ld_r8_r8(gb, RegisterR8::H, RegisterR8::L));
opcode!(ld_h_h, [gb] ld_r8_r8(gb, RegisterR8::H, RegisterR8::H));

// LD r8, u8
opcode!(ld_a_u8, [gb] ld_r8_u8(gb, RegisterR8::A));
opcode!(ld_b_u8, [gb] ld_r8_u8(gb, RegisterR8::B));
opcode!(ld_c_u8, [gb] ld_r8_u8(gb, RegisterR8::C));
opcode!(ld_d_u8, [gb] ld_r8_u8(gb, RegisterR8::D));
opcode!(ld_e_u8, [gb] ld_r8_u8(gb, RegisterR8::E));
opcode!(ld_h_u8, [gb] ld_r8_u8(gb, RegisterR8::H));
opcode!(ld_l_u8, [gb] ld_r8_u8(gb, RegisterR8::L));

// LD r16, u16
opcode!(ld_bc_u16, [gb] ld_r16_u16(gb, RegisterR16::BC));
opcode!(ld_de_u16, [gb] ld_r16_u16(gb, RegisterR16::DE));
opcode!(ld_hl_u16, [gb] ld_r16_u16(gb, RegisterR16::HL));

// LD r8, (u16)
opcode!(ld_a_u16ptr, [gb] ld_r8_u16ptr(gb, RegisterR8::A));

// LD r8, (r16)
opcode!(ld_a_bcptr, [gb] ld_r8_r16ptr(gb, RegisterR8::A, RegisterR16::BC));
opcode!(ld_a_deptr, [gb] ld_r8_r16ptr(gb, RegisterR8::A, RegisterR16::DE));
opcode!(ld_a_hlptr, [gb] ld_r8_r16ptr(gb, RegisterR8::A, RegisterR16::HL));
opcode!(ld_b_hlptr, [gb] ld_r8_r16ptr(gb, RegisterR8::B, RegisterR16::HL));
opcode!(ld_c_hlptr, [gb] ld_r8_r16ptr(gb, RegisterR8::C, RegisterR16::HL));
opcode!(ld_d_hlptr, [gb] ld_r8_r16ptr(gb, RegisterR8::D, RegisterR16::HL));
opcode!(ld_e_hlptr, [gb] ld_r8_r16ptr(gb, RegisterR8::E, RegisterR16::HL));
opcode!(ld_h_hlptr, [gb] ld_r8_r16ptr(gb, RegisterR8::H, RegisterR16::HL));
opcode!(ld_l_hlptr, [gb] ld_r8_r16ptr(gb, RegisterR8::L, RegisterR16::HL));

// LD (r16), r8
opcode!(ld_bcptr_a, [gb] ld_r16ptr_r8(gb, RegisterR16::BC, RegisterR8::A));
opcode!(ld_deptr_a, [gb] ld_r16ptr_r8(gb, RegisterR16::DE, RegisterR8::A));
opcode!(ld_hlptr_a, [gb] ld_r16ptr_r8(gb, RegisterR16::HL, RegisterR8::A));
opcode!(ld_hlptr_b, [gb] ld_r16ptr_r8(gb, RegisterR16::HL, RegisterR8::B));
opcode!(ld_hlptr_c, [gb] ld_r16ptr_r8(gb, RegisterR16::HL, RegisterR8::C));
opcode!(ld_hlptr_d, [gb] ld_r16ptr_r8(gb, RegisterR16::HL, RegisterR8::D));
opcode!(ld_hlptr_e, [gb] ld_r16ptr_r8(gb, RegisterR16::HL, RegisterR8::E));
opcode!(ld_hlptr_h, [gb] ld_r16ptr_r8(gb, RegisterR16::HL, RegisterR8::H));
opcode!(ld_hlptr_l, [gb] ld_r16ptr_r8(gb, RegisterR16::HL, RegisterR8::L));

// LD A, (HL+)
opcode!(ld_a_hlptri, [gb] {
    let hl = gb.cpu.get_r16(RegisterR16::HL);
    ld_r8_addr(gb, RegisterR8::A, hl);
    gb.cpu.set_r16(RegisterR16::HL, hl + 1);
});

// LD A, (HL-)
opcode!(ld_a_hlptrd, [gb] {
    let hl = gb.cpu.get_r16(RegisterR16::HL);
    ld_r8_addr(gb, RegisterR8::A, hl);
    gb.cpu.set_r16(RegisterR16::HL, hl - 1);
});

// LD (HL+), A
opcode!(ld_hlptri_a, [gb] {
    let hl = gb.cpu.get_r16(RegisterR16::HL);
    ld_addr_r8(gb, hl, RegisterR8::A);
    gb.cpu.set_r16(RegisterR16::HL, hl + 1);
});

// LD (HL-), A
opcode!(ld_hlptrd_a, [gb] {
    let hl = gb.cpu.get_r16(RegisterR16::HL);
    ld_addr_r8(gb, hl, RegisterR8::A);
    gb.cpu.set_r16(RegisterR16::HL, hl - 1);
});

// LD (r16), u8
opcode!(ld_hlptr_u8, [gb] ld_r16ptr_u8(gb, RegisterR16::HL));

// LD (u16), A
opcode!(ld_u16ptr_a,  [gb] ld_u16ptr_r8(gb, RegisterR8::A));

// LD (u16), SP
opcode!(ld_u16ptr_sp, [gb] ld_u16ptr_u16v(gb, gb.cpu.get_stack_pointer()));

// LDH (0xff00 + u8), A // LDH A, (0xff00 + u8)
opcode!(ldh_u8_a, [gb] ldh_u8_r8(gb, RegisterR8::A));
opcode!(ldh_a_u8, [gb] ldh_r8_u8(gb, RegisterR8::A));

// LDH (0xff00 + C), A // LDH A, (0xff00 + C)
opcode!(ldh_cptr_a, [gb] ldh_r8ptr_r8(gb, RegisterR8::C, RegisterR8::A));
opcode!(ldh_a_cptr, [gb] ldh_r8_r8ptr(gb, RegisterR8::A, RegisterR8::C));

// LD HL, SP+i8
opcode!(ld_hl_sp_i8, [gb] {
    let offset = gb.cpu.fetch_i8();
    let sp     = gb.cpu.get_stack_pointer();
    let (sp_new, _, _) = signed_overflow_add_u16(sp, offset as i16);

    let carry_bits = sp ^ sp_new ^ (offset as u16);
    let half_carry = (carry_bits & 0x0010) != 0;
    let carry      = (carry_bits & 0x0100) != 0;

    gb.cpu.set_flag(CpuFlag::Zero,      false);
    gb.cpu.set_flag(CpuFlag::Negative,  false);
    gb.cpu.set_flag(CpuFlag::HalfCarry, half_carry);
    gb.cpu.set_flag(CpuFlag::Carry,     carry);
    gb.cpu.set_r16(RegisterR16::HL, sp_new);
});

// LD SP, u16
opcode!(ld_sp_u16, [gb] {
    let value = gb.cpu.fetch_u16();
    gb.cpu.set_stack_pointer(value);
});

// LD SP, HL
opcode!(ld_sp_hl, [gb] {
    let value = gb.cpu.get_r16(RegisterR16::HL);
    gb.cpu.set_stack_pointer(value);
});

// PUSH r16
opcode!(push_af, [gb] push_r16(gb, RegisterR16::AF));
opcode!(push_bc, [gb] push_r16(gb, RegisterR16::BC));
opcode!(push_de, [gb] push_r16(gb, RegisterR16::DE));
opcode!(push_hl, [gb] push_r16(gb, RegisterR16::HL));

// POP r16
opcode!(pop_af, [gb] pop_r16_mask(gb, RegisterR16::AF, 0xfff0));
opcode!(pop_bc, [gb] pop_r16(gb, RegisterR16::BC));
opcode!(pop_de, [gb] pop_r16(gb, RegisterR16::DE));
opcode!(pop_hl, [gb] pop_r16(gb, RegisterR16::HL));
