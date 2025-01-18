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

use crate::cpu::cpu::{CpuFlag, RegisterR16};
use crate::cpu::opcode::{opcode, OpCodeContext};
use crate::emulator_core::{Clock, EmulatorCore};

const BRANCH_CYCLES_JMP:    Clock =  4;
const BRANCH_CYCLES_CALL:   Clock = 12;
const BRANCH_CYCLES_RET:    Clock = 12;


/// Performs a jump to an address if a condition is met.
fn jp_if_u16(gb: &mut EmulatorCore, ctx: &mut OpCodeContext, flag: CpuFlag, value: bool) {
    let address = gb.cpu.fetch_u16();
    if gb.cpu.is_flag_set(flag) == value {
        ctx.add_cycles(BRANCH_CYCLES_JMP);
        gb.cpu.jump_to(address);
    }
}

/// Performs a relative jump if a condition is met.
fn jr_if_i8(gb: &mut EmulatorCore, ctx: &mut OpCodeContext, flag: CpuFlag, value: bool) {
    let offset = gb.cpu.fetch_i8();
    if gb.cpu.is_flag_set(flag) == value {
        ctx.add_cycles(BRANCH_CYCLES_JMP);
        gb.cpu.jump_relative(offset as i16);
    }
}

/// Calls a subroutine by storing the current instruction pointer on the stack
/// and set the instruction pointer to a new address.
fn call_addr(gb: &mut EmulatorCore, address: u16) {
    gb.cpu.call_addr(address);
}

/// Calls a subroutine by storing the current instruction pointer on the stack
/// and set the instruction pointer to a new address.
fn call_addr_u16(gb: &mut EmulatorCore) {
    let address = gb.cpu.fetch_u16();
    call_addr(gb, address);
}

/// Calls a subroutine by storing the current instruction pointer on the stack
/// and set the instruction pointer to a new address, if a condition is met.
fn call_addr_if(gb: &mut EmulatorCore, ctx: &mut OpCodeContext, flag: CpuFlag, value: bool, address: u16) {
    if gb.cpu.is_flag_set(flag) == value {
        ctx.add_cycles(BRANCH_CYCLES_CALL);
        call_addr(gb, address);
    }
}

/// Calls a subroutine by storing the current instruction pointer on the stack
/// and set the instruction pointer to a new address, taken from the current instruction pointer,
/// if a condition is met.
fn call_u16_if(gb: &mut EmulatorCore, ctx: &mut OpCodeContext, flag: CpuFlag, value: bool) {
    let address = gb.cpu.fetch_u16();
    call_addr_if(gb, ctx, flag, value, address);
}

/// Returns from a subroutine by taking the previous instruction pointer address from the stack.
fn ret_from_call(gb: &mut EmulatorCore) {
    gb.cpu.ret_from_call();
}

/// Returns from a subroutine by taking the previous instruction pointer address from the stack,
/// if a condition is met.
fn ret_if(gb: &mut EmulatorCore, ctx: &mut OpCodeContext, flag: CpuFlag, value: bool) {
    if gb.cpu.is_flag_set(flag) == value {
        ctx.add_cycles(BRANCH_CYCLES_RET);
        ret_from_call(gb);
    }
}



opcode!(jr_i8, [gb] {
    let offset = gb.cpu.fetch_i8();
    gb.cpu.jump_relative(offset as i16);
});

opcode!(jp_u16, [gb] {
    let address = gb.cpu.fetch_u16();
    gb.cpu.jump_to(address);
});

opcode!(jp_hl, [gb] {
    let address = gb.cpu.get_r16(RegisterR16::HL);
    gb.cpu.jump_to(address);
});

opcode!(jr_z_i8,  [gb, ctx] jr_if_i8(gb, ctx, CpuFlag::Zero,  true));
opcode!(jr_c_i8,  [gb, ctx] jr_if_i8(gb, ctx, CpuFlag::Carry, true));
opcode!(jr_nz_i8, [gb, ctx] jr_if_i8(gb, ctx, CpuFlag::Zero,  false));
opcode!(jr_nc_i8, [gb, ctx] jr_if_i8(gb, ctx, CpuFlag::Carry, false));

opcode!(jp_z_u16,  [gb, ctx] jp_if_u16(gb, ctx, CpuFlag::Zero,  true));
opcode!(jp_c_u16,  [gb, ctx] jp_if_u16(gb, ctx, CpuFlag::Carry, true));
opcode!(jp_nz_u16, [gb, ctx] jp_if_u16(gb, ctx, CpuFlag::Zero,  false));
opcode!(jp_nc_u16, [gb, ctx] jp_if_u16(gb, ctx, CpuFlag::Carry, false));

opcode!(call_u16, [gb] call_addr_u16(gb));

opcode!(call_z_u16,  [gb, ctx] call_u16_if(gb, ctx, CpuFlag::Zero, true));
opcode!(call_c_u16,  [gb, ctx] call_u16_if(gb, ctx, CpuFlag::Carry, true));
opcode!(call_nz_u16, [gb, ctx] call_u16_if(gb, ctx, CpuFlag::Zero, false));
opcode!(call_nc_u16, [gb, ctx] call_u16_if(gb, ctx, CpuFlag::Carry, false));

opcode!(rst_00h, [gb] call_addr(gb, 0x0000));
opcode!(rst_08h, [gb] call_addr(gb, 0x0008));
opcode!(rst_10h, [gb] call_addr(gb, 0x0010));
opcode!(rst_18h, [gb] call_addr(gb, 0x0018));
opcode!(rst_20h, [gb] call_addr(gb, 0x0020));
opcode!(rst_28h, [gb] call_addr(gb, 0x0028));
opcode!(rst_30h, [gb] call_addr(gb, 0x0030));
opcode!(rst_38h, [gb] call_addr(gb, 0x0038));

opcode!(ret, [gb] ret_from_call(gb));

opcode!(ret_z,  [gb, ctx] ret_if(gb, ctx, CpuFlag::Zero,  true));
opcode!(ret_c,  [gb, ctx] ret_if(gb, ctx, CpuFlag::Carry, true));
opcode!(ret_nz, [gb, ctx] ret_if(gb, ctx, CpuFlag::Zero,  false));
opcode!(ret_nc, [gb, ctx] ret_if(gb, ctx, CpuFlag::Carry, false));

opcode!(reti, [gb] {
    ret_from_call(gb);
    gb.cpu.enable_interrupts();
});
