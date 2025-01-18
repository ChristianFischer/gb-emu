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
use crate::cpu::opcode::{opcode, OpCodeContext, OpCodeResult};
use crate::emulator_core::EmulatorCore;
use crate::utils::{carrying_add_u16, carrying_add_u8, carrying_sub_u8};


////////////////////////////////////////////////
//// Flag types

enum ShiftOp {
    ShiftLogical,
    ShiftArithmetic,
    Rotate,
    RotateThroughCarry,
}

enum NullCheck {
    Check,
    ClearFlag
}


////////////////////////////////////////////////
//// INC opcodes
pub mod inc {
    use super::*;

    /// Increments a 8bit value.
    fn increment_u8v(gb: &mut EmulatorCore, value: u8) -> u8 {
        let result = value.wrapping_add(1);

        gb.cpu.set_flag(CpuFlag::Zero,      result == 0);
        gb.cpu.set_flag(CpuFlag::Negative,  false);
        gb.cpu.set_flag(CpuFlag::HalfCarry, (result & 0x0f) == 0);

        result
    }

    /// Increments a 16bit value.
    fn increment_u16v(_gb: &mut EmulatorCore, value: u16) -> u16 {
        let result = value.wrapping_add(1);
        result
    }

    /// Increments a value
    /// r8 <- r8 + 1
    fn increment_r8(gb: &mut EmulatorCore, r8: RegisterR8) {
        let value  = gb.cpu.get_r8(r8);
        let result = increment_u8v(gb, value);
        gb.cpu.set_r8(r8, result);
    }

    /// Increments a value
    /// r16 <- r16 + 1
    fn increment_r16(gb: &mut EmulatorCore, r16: RegisterR16) {
        let value  = gb.cpu.get_r16(r16);
        let result = increment_u16v(gb, value);
        gb.cpu.set_r16(r16, result);
    }

    /// Increments a value.
    /// (r16) <- (r16) + 1
    fn increment_r16ptr(gb: &mut EmulatorCore, ctx: &mut OpCodeContext, r16_ptr: RegisterR16) -> OpCodeResult {
        match ctx.get_stage() {
            0 => {
                let address = gb.cpu.get_r16(r16_ptr);
                let value   = gb.get_mmu().read_u8(address);
                gb.cpu.set_intermediate_value(value);

                OpCodeResult::StageDone(4)
            }

            1 => {
                let address = gb.cpu.get_r16(r16_ptr);
                let value   = gb.cpu.get_intermediate_value();
                let result  = increment_u8v(gb, value);
                gb.get_mmu_mut().write_u8(address, result);

                OpCodeResult::Done
            }

            _ => unreachable!()
        }
    }


    // INC r8
    opcode!(inc_a, [gb] increment_r8(gb, RegisterR8::A));
    opcode!(inc_b, [gb] increment_r8(gb, RegisterR8::B));
    opcode!(inc_c, [gb] increment_r8(gb, RegisterR8::C));
    opcode!(inc_d, [gb] increment_r8(gb, RegisterR8::D));
    opcode!(inc_e, [gb] increment_r8(gb, RegisterR8::E));
    opcode!(inc_h, [gb] increment_r8(gb, RegisterR8::H));
    opcode!(inc_l, [gb] increment_r8(gb, RegisterR8::L));

    // INC r16
    opcode!(inc_bc, [gb] increment_r16(gb, RegisterR16::BC));
    opcode!(inc_de, [gb] increment_r16(gb, RegisterR16::DE));
    opcode!(inc_hl, [gb] increment_r16(gb, RegisterR16::HL));

    // INC (r16)
    opcode!(inc_hlptr, [gb, ctx] increment_r16ptr(gb, ctx, RegisterR16::HL));

    // INC SP
    opcode!(inc_sp, [gb] {
        let sp_old = gb.cpu.get_stack_pointer();
        let sp_new = sp_old.wrapping_add(1);
        gb.cpu.set_stack_pointer(sp_new);
    });
}


////////////////////////////////////////////////
//// DEC opcodes
pub mod dec {
    use super::*;

    /// Decrements a 8bit value.
    fn decrement_u8v(gb: &mut EmulatorCore, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
    
        gb.cpu.set_flag(CpuFlag::Zero,      result == 0);
        gb.cpu.set_flag(CpuFlag::Negative,  true);
        gb.cpu.set_flag(CpuFlag::HalfCarry, (result & 0x0f) == 0x0f);
    
        result
    }
    
    /// Decrements a 16bit value.
    fn decrement_u16v(_gb: &mut EmulatorCore, value: u16) -> u16 {
        let result = value.wrapping_sub(1);
        result
    }
    
    /// Decrements a value
    /// r8 <- r8 - 1
    fn decrement_r8(gb: &mut EmulatorCore, r8: RegisterR8) {
        let value  = gb.cpu.get_r8(r8);
        let result = decrement_u8v(gb, value);
        gb.cpu.set_r8(r8, result);
    }
    
    /// Decrements a value
    /// r16 <- r16 - 1
    fn decrement_r16(gb: &mut EmulatorCore, r16: RegisterR16) {
        let value  = gb.cpu.get_r16(r16);
        let result = decrement_u16v(gb, value);
        gb.cpu.set_r16(r16, result);
    }
    
    /// Decrements a value.
    /// (r16) <- (r16) - 1
    fn decrement_r16ptr(gb: &mut EmulatorCore, ctx: &mut OpCodeContext, r16_ptr: RegisterR16) -> OpCodeResult {
        match ctx.get_stage() {
            0 => {
                let address = gb.cpu.get_r16(r16_ptr);
                let value   = gb.get_mmu().read_u8(address);
                gb.cpu.set_intermediate_value(value);

                OpCodeResult::StageDone(4)
            },

            1 => {
                let address = gb.cpu.get_r16(r16_ptr);
                let value = gb.cpu.get_intermediate_value();
                let result  = decrement_u8v(gb, value);
                gb.get_mmu_mut().write_u8(address, result);

                OpCodeResult::Done
            },

            _ => unreachable!()
        }
    }
    

    // DEC r8
    opcode!(dec_a, [gb] decrement_r8(gb, RegisterR8::A));    
    opcode!(dec_b, [gb] decrement_r8(gb, RegisterR8::B));    
    opcode!(dec_c, [gb] decrement_r8(gb, RegisterR8::C));    
    opcode!(dec_d, [gb] decrement_r8(gb, RegisterR8::D));    
    opcode!(dec_e, [gb] decrement_r8(gb, RegisterR8::E));    
    opcode!(dec_h, [gb] decrement_r8(gb, RegisterR8::H));    
    opcode!(dec_l, [gb] decrement_r8(gb, RegisterR8::L));

    // DEC r16
    opcode!(dec_bc, [gb] decrement_r16(gb, RegisterR16::BC));    
    opcode!(dec_de, [gb] decrement_r16(gb, RegisterR16::DE));    
    opcode!(dec_hl, [gb] decrement_r16(gb, RegisterR16::HL));

    // DEC (r16)
    opcode!(dec_hlptr, [gb, ctx] decrement_r16ptr(gb, ctx, RegisterR16::HL));

    // DEC SP
    opcode!(dec_sp, [gb] {
        let sp_old = gb.cpu.get_stack_pointer();
        let sp_new = sp_old.wrapping_sub(1);
        gb.cpu.set_stack_pointer(sp_new);
    });
}


////////////////////////////////////////////////
//// ADD / ADC opcodes
pub mod add {
    use super::*;

    /// Adds two values and stores it into a 8bit register.
    /// r8 <- r8 + value + (carry flag, if add_carry)
    fn add_r8_u8v(gb: &mut EmulatorCore, r8: RegisterR8, value: u8, add_carry: bool) {
        let current_carry = add_carry && gb.cpu.is_flag_set(CpuFlag::Carry);
        let current_value = gb.cpu.get_r8(r8);
        let (result, half_carry, carry) = carrying_add_u8(current_value, value, current_carry);

        gb.cpu.set_flag(CpuFlag::Zero,      result == 0);
        gb.cpu.set_flag(CpuFlag::Negative,  false);
        gb.cpu.set_flag(CpuFlag::HalfCarry, half_carry);
        gb.cpu.set_flag(CpuFlag::Carry,     carry);
        gb.cpu.set_r8(r8, result as u8);
    }

    /// Adds two values and stores it into a 8bit register.
    /// dst <- dst + u8 + (carry flag, if add_carry)
    fn add_r8_u8(gb: &mut EmulatorCore, dst: RegisterR8, add_carry: bool) {
        let value = gb.cpu.fetch_u8();
        add_r8_u8v(gb, dst, value, add_carry);
    }

    /// Adds two values and stores it into a 8bit register.
    /// dst <- dst + src + (carry flag, if add_carry)
    fn add_r8_r8(gb: &mut EmulatorCore, dst: RegisterR8, src: RegisterR8, add_carry: bool) {
        let value = gb.cpu.get_r8(src);
        add_r8_u8v(gb, dst, value, add_carry);
    }

    /// Adds two values and stores it into a 8bit register.
    /// dst <- dst + (src_ptr) + (carry flag, if add_carry)
    fn add_r8_r16ptr(gb: &mut EmulatorCore, dst: RegisterR8, src_ptr: RegisterR16, add_carry: bool) {
        let address = gb.cpu.get_r16(src_ptr);
        let value   = gb.get_mmu().read_u8(address);
        add_r8_u8v(gb, dst, value, add_carry);
    }

    /// Adds two values and stores it into a 16bit register.
    /// r16 <- r16 + value
    fn add_r16_u16v(gb: &mut EmulatorCore, r16: RegisterR16, value: u16) {
        let current_value = gb.cpu.get_r16(r16);
        let (result, half_carry, carry) = carrying_add_u16(current_value, value, false);

        gb.cpu.set_flag(CpuFlag::Negative,  false);
        gb.cpu.set_flag(CpuFlag::HalfCarry, half_carry);
        gb.cpu.set_flag(CpuFlag::Carry,     carry);
        gb.cpu.set_r16(r16, result as u16);
    }

    /// Adds two values and stores it into a 16bit register.
    /// dst <- dst + src
    fn add_r16_r16(gb: &mut EmulatorCore, dst: RegisterR16, src: RegisterR16) {
        let value = gb.cpu.get_r16(src);
        add_r16_u16v(gb, dst, value);
    }


    // ADD r8, ?
    opcode!(add_a_u8,    [gb] add_r8_u8(gb, RegisterR8::A, false));
    opcode!(add_a_a,     [gb] add_r8_r8(gb, RegisterR8::A, RegisterR8::A, false));
    opcode!(add_a_b,     [gb] add_r8_r8(gb, RegisterR8::A, RegisterR8::B, false));
    opcode!(add_a_c,     [gb] add_r8_r8(gb, RegisterR8::A, RegisterR8::C, false));
    opcode!(add_a_d,     [gb] add_r8_r8(gb, RegisterR8::A, RegisterR8::D, false));
    opcode!(add_a_e,     [gb] add_r8_r8(gb, RegisterR8::A, RegisterR8::E, false));
    opcode!(add_a_h,     [gb] add_r8_r8(gb, RegisterR8::A, RegisterR8::H, false));
    opcode!(add_a_l,     [gb] add_r8_r8(gb, RegisterR8::A, RegisterR8::L, false));
    opcode!(add_a_hlptr, [gb] add_r8_r16ptr(gb, RegisterR8::A, RegisterR16::HL, false));

    // add with carry flag
    // ADC r8, ?
    opcode!(adc_a_u8,    [gb] add_r8_u8(gb, RegisterR8::A, true));
    opcode!(adc_a_a,     [gb] add_r8_r8(gb, RegisterR8::A, RegisterR8::A, true));
    opcode!(adc_a_b,     [gb] add_r8_r8(gb, RegisterR8::A, RegisterR8::B, true));
    opcode!(adc_a_c,     [gb] add_r8_r8(gb, RegisterR8::A, RegisterR8::C, true));
    opcode!(adc_a_d,     [gb] add_r8_r8(gb, RegisterR8::A, RegisterR8::D, true));
    opcode!(adc_a_e,     [gb] add_r8_r8(gb, RegisterR8::A, RegisterR8::E, true));
    opcode!(adc_a_h,     [gb] add_r8_r8(gb, RegisterR8::A, RegisterR8::H, true));
    opcode!(adc_a_l,     [gb] add_r8_r8(gb, RegisterR8::A, RegisterR8::L, true));
    opcode!(adc_a_hlptr, [gb] add_r8_r16ptr(gb, RegisterR8::A, RegisterR16::HL, true));

    // ADD r16, r16
    opcode!(add_hl_bc, [gb] add_r16_r16(gb, RegisterR16::HL, RegisterR16::BC));
    opcode!(add_hl_de, [gb] add_r16_r16(gb, RegisterR16::HL, RegisterR16::DE));
    opcode!(add_hl_hl, [gb] add_r16_r16(gb, RegisterR16::HL, RegisterR16::HL));
    opcode!(add_hl_sp, [gb] add_r16_u16v(gb, RegisterR16::HL, gb.cpu.get_stack_pointer()));
}


////////////////////////////////////////////////
//// SUB / SBC opcodes
pub mod sub {
    use super::*;

    /// Subtracts a value from another one and stores the result into a 8bit register.
    /// r8 <- r8 - value - (carry flag, if sub_carry)
    fn sub_r8_u8v(gb: &mut EmulatorCore, r8: RegisterR8, value: u8, sub_carry: bool) {
        let current_carry = sub_carry && gb.cpu.is_flag_set(CpuFlag::Carry);
        let current_value = gb.cpu.get_r8(r8);
        let (result, half_carry, carry) = carrying_sub_u8(current_value, value, current_carry);

        gb.cpu.set_flag(CpuFlag::Zero,      result == 0);
        gb.cpu.set_flag(CpuFlag::Negative,  true);
        gb.cpu.set_flag(CpuFlag::HalfCarry, half_carry);
        gb.cpu.set_flag(CpuFlag::Carry,     carry);
        gb.cpu.set_r8(r8, result as u8);
    }

    /// Adds two values and stores it into a 8bit register.
    /// dst <- dst + u8 + (carry flag, if add_carry)
    fn sub_r8_u8(gb: &mut EmulatorCore, dst: RegisterR8, add_carry: bool) {
        let value = gb.cpu.fetch_u8();
        sub_r8_u8v(gb, dst, value, add_carry);
    }

    /// Subtracts a value from another one and stores the result into a 8bit register.
    /// dst <- dst - src - (carry flag, if sub_carry)
    fn sub_r8_r8(gb: &mut EmulatorCore, dst: RegisterR8, src: RegisterR8, sub_carry: bool) {
        let value = gb.cpu.get_r8(src);
        sub_r8_u8v(gb, dst, value, sub_carry);
    }

    /// Subtracts a value from another one and stores the result into a 8bit register.
    /// dst <- dst - (src_ptr) - (carry flag, if sub_carry)
    fn sub_r8_r16ptr(gb: &mut EmulatorCore, dst: RegisterR8, src_ptr: RegisterR16, sub_carry: bool) {
        let address = gb.cpu.get_r16(src_ptr);
        let value   = gb.get_mmu().read_u8(address);
        sub_r8_u8v(gb, dst, value, sub_carry);
    }


    // SUB r8, ?
    opcode!(sub_a_u8,    [gb] sub_r8_u8(gb, RegisterR8::A, false));
    opcode!(sub_a_a,     [gb] sub_r8_r8(gb, RegisterR8::A, RegisterR8::A, false));
    opcode!(sub_a_b,     [gb] sub_r8_r8(gb, RegisterR8::A, RegisterR8::B, false));
    opcode!(sub_a_c,     [gb] sub_r8_r8(gb, RegisterR8::A, RegisterR8::C, false));
    opcode!(sub_a_d,     [gb] sub_r8_r8(gb, RegisterR8::A, RegisterR8::D, false));
    opcode!(sub_a_e,     [gb] sub_r8_r8(gb, RegisterR8::A, RegisterR8::E, false));
    opcode!(sub_a_h,     [gb] sub_r8_r8(gb, RegisterR8::A, RegisterR8::H, false));
    opcode!(sub_a_l,     [gb] sub_r8_r8(gb, RegisterR8::A, RegisterR8::L, false));
    opcode!(sub_a_hlptr, [gb] sub_r8_r16ptr(gb, RegisterR8::A, RegisterR16::HL, false));

    // subtract with carry flag
    // SUB r8, ?
    opcode!(sbc_a_u8,    [gb] sub_r8_u8(gb, RegisterR8::A, true));
    opcode!(sbc_a_a,     [gb] sub_r8_r8(gb, RegisterR8::A, RegisterR8::A, true));
    opcode!(sbc_a_b,     [gb] sub_r8_r8(gb, RegisterR8::A, RegisterR8::B, true));
    opcode!(sbc_a_c,     [gb] sub_r8_r8(gb, RegisterR8::A, RegisterR8::C, true));
    opcode!(sbc_a_d,     [gb] sub_r8_r8(gb, RegisterR8::A, RegisterR8::D, true));
    opcode!(sbc_a_e,     [gb] sub_r8_r8(gb, RegisterR8::A, RegisterR8::E, true));
    opcode!(sbc_a_h,     [gb] sub_r8_r8(gb, RegisterR8::A, RegisterR8::H, true));
    opcode!(sbc_a_l,     [gb] sub_r8_r8(gb, RegisterR8::A, RegisterR8::L, true));
    opcode!(sbc_a_hlptr, [gb] sub_r8_r16ptr(gb, RegisterR8::A, RegisterR16::HL, true));
}


////////////////////////////////////////////////
//// RL / RLC opcodes
pub mod rl {
    use super::*;

    /// Shifts or rotates a value to the left.
    fn shift_left_u8v(gb: &mut EmulatorCore, value: u8, op: ShiftOp) -> u8 {
        shift_left_u8v_nc(gb, value, op, NullCheck::Check)
    }

    /// Shifts or rotates a value to the left.
    fn shift_left_u8v_nc(gb: &mut EmulatorCore, value: u8, op: ShiftOp, nullcheck: NullCheck) -> u8 {
        let carry    = gb.cpu.is_flag_set(CpuFlag::Carry) as u8;
        let left_bit = (value >> 7) & 1;

        let result = match op {
            ShiftOp::ShiftLogical       => (value << 1) | 0x0000,
            ShiftOp::ShiftArithmetic    => (value << 1) | 0x0000,
            ShiftOp::Rotate             => (value << 1) | left_bit,
            ShiftOp::RotateThroughCarry => (value << 1) | carry,
        };

        let null_bit = match nullcheck {
            NullCheck::Check     => result == 0,
            NullCheck::ClearFlag => false,
        };

        gb.cpu.set_flag(CpuFlag::Zero,      null_bit);
        gb.cpu.set_flag(CpuFlag::Negative,  false);
        gb.cpu.set_flag(CpuFlag::HalfCarry, false);
        gb.cpu.set_flag(CpuFlag::Carry,     left_bit != 0);

        result
    }

    /// Shifts or rotates a value on a 16bit pointer to the left.
    fn shift_left_r16ptr(gb: &mut EmulatorCore, ctx: &mut OpCodeContext, r16ptr: RegisterR16, op: ShiftOp) -> OpCodeResult {
        match ctx.get_stage() {
            0 => {
                let address = gb.cpu.get_r16(r16ptr);
                let value   = gb.get_mmu().read_u8(address);
                gb.cpu.set_intermediate_value(value);

                OpCodeResult::StageDone(4)
            }

            1 => {
                let address = gb.cpu.get_r16(r16ptr);
                let value   = gb.cpu.get_intermediate_value();
                let result  = shift_left_u8v(gb, value, op);
                gb.get_mmu_mut().write_u8(address, result);

                OpCodeResult::Done
            }

            _ => unreachable!()
        }
    }

    /// Performs an arithmetic shift left of the value of a register.
    fn sla_r8(gb: &mut EmulatorCore, r8: RegisterR8) {
        let value  = gb.cpu.get_r8(r8);
        let result = shift_left_u8v(gb, value, ShiftOp::ShiftArithmetic);
        gb.cpu.set_r8(r8, result);
    }

    /// Performs an arithmetic shift left of the value on a memory location.
    fn sla_r16ptr(gb: &mut EmulatorCore, ctx: &mut OpCodeContext, r16ptr: RegisterR16) -> OpCodeResult {
        shift_left_r16ptr(gb, ctx, r16ptr, ShiftOp::ShiftArithmetic)
    }

    /// Rotates the value of a register to the left through the carry flag.
    fn rl_r8(gb: &mut EmulatorCore, r8: RegisterR8) {
        rl_r8_nc(gb, r8, NullCheck::Check);
    }

    /// Rotates the value of a register to the left through the carry flag.
    fn rl_r8_nc(gb: &mut EmulatorCore, r8: RegisterR8, nullcheck: NullCheck) {
        let value  = gb.cpu.get_r8(r8);
        let result = shift_left_u8v_nc(gb, value, ShiftOp::RotateThroughCarry, nullcheck);
        gb.cpu.set_r8(r8, result);
    }

    /// Rotates the value on a memory location to the left through the carry flag.
    fn rl_r16ptr(gb: &mut EmulatorCore, ctx: &mut OpCodeContext, r16ptr: RegisterR16) -> OpCodeResult {
        shift_left_r16ptr(gb, ctx, r16ptr, ShiftOp::RotateThroughCarry)
    }

    /// Rotates the value of a register to the left.
    fn rlc_r8(gb: &mut EmulatorCore, r8: RegisterR8) {
        rlc_r8_nc(gb, r8, NullCheck::Check);
    }

    /// Rotates the value of a register to the left.
    fn rlc_r8_nc(gb: &mut EmulatorCore, r8: RegisterR8, nullcheck: NullCheck) {
        let value  = gb.cpu.get_r8(r8);
        let result = shift_left_u8v_nc(gb, value, ShiftOp::Rotate, nullcheck);
        gb.cpu.set_r8(r8, result);
    }

    /// Rotates the value on a memory location to the left.
    fn rlc_r16ptr(gb: &mut EmulatorCore, ctx: &mut OpCodeContext, r16ptr: RegisterR16) -> OpCodeResult {
        shift_left_r16ptr(gb, ctx, r16ptr, ShiftOp::Rotate)
    }


    // arithmetic shift left
    opcode!(sla_a,     [gb] sla_r8(gb, RegisterR8::A));
    opcode!(sla_b,     [gb] sla_r8(gb, RegisterR8::B));
    opcode!(sla_c,     [gb] sla_r8(gb, RegisterR8::C));
    opcode!(sla_d,     [gb] sla_r8(gb, RegisterR8::D));
    opcode!(sla_e,     [gb] sla_r8(gb, RegisterR8::E));
    opcode!(sla_h,     [gb] sla_r8(gb, RegisterR8::H));
    opcode!(sla_l,     [gb] sla_r8(gb, RegisterR8::L));

    // rotate left through carry flag
    opcode!(rla,      [gb] rl_r8_nc(gb, RegisterR8::A, NullCheck::ClearFlag));
    opcode!(rl_a,     [gb] rl_r8(gb, RegisterR8::A));
    opcode!(rl_b,     [gb] rl_r8(gb, RegisterR8::B));
    opcode!(rl_c,     [gb] rl_r8(gb, RegisterR8::C));
    opcode!(rl_d,     [gb] rl_r8(gb, RegisterR8::D));
    opcode!(rl_e,     [gb] rl_r8(gb, RegisterR8::E));
    opcode!(rl_h,     [gb] rl_r8(gb, RegisterR8::H));
    opcode!(rl_l,     [gb] rl_r8(gb, RegisterR8::L));

    // rotate left (carry flag just set)
    opcode!(rlca,      [gb] rlc_r8_nc(gb, RegisterR8::A, NullCheck::ClearFlag));
    opcode!(rlc_a,     [gb] rlc_r8(gb, RegisterR8::A));
    opcode!(rlc_b,     [gb] rlc_r8(gb, RegisterR8::B));
    opcode!(rlc_c,     [gb] rlc_r8(gb, RegisterR8::C));
    opcode!(rlc_d,     [gb] rlc_r8(gb, RegisterR8::D));
    opcode!(rlc_e,     [gb] rlc_r8(gb, RegisterR8::E));
    opcode!(rlc_h,     [gb] rlc_r8(gb, RegisterR8::H));
    opcode!(rlc_l,     [gb] rlc_r8(gb, RegisterR8::L));

    opcode!(sla_hlptr, [gb, ctx] sla_r16ptr(gb, ctx, RegisterR16::HL));
    opcode!(rl_hlptr,  [gb, ctx] rl_r16ptr (gb, ctx, RegisterR16::HL));
    opcode!(rlc_hlptr, [gb, ctx] rlc_r16ptr(gb, ctx, RegisterR16::HL));
}

////////////////////////////////////////////////
//// RR / RRC opcodes
pub mod rr {
    use super::*;

    /// Shifts or rotates a value to the right.
    fn shift_right_u8v(gb: &mut EmulatorCore, value: u8, op: ShiftOp) -> u8 {
        shift_right_u8v_nc(gb, value, op, NullCheck::Check)
    }

    /// Shifts or rotates a value to the right.
    fn shift_right_u8v_nc(gb: &mut EmulatorCore, value: u8, op: ShiftOp, nullcheck: NullCheck) -> u8 {
        let carry    = gb.cpu.is_flag_set(CpuFlag::Carry) as u8;
        let left_bit = (value >> 7) & 1;
        let right_bit= value & 1;

        let result = match op {
            ShiftOp::ShiftLogical       => (value >> 1) | 0x0000,
            ShiftOp::ShiftArithmetic    => (value >> 1) | (left_bit << 7),
            ShiftOp::Rotate             => (value >> 1) | (right_bit << 7),
            ShiftOp::RotateThroughCarry => (value >> 1) | (carry << 7),
        };

        let null_bit = match nullcheck {
            NullCheck::Check     => result == 0,
            NullCheck::ClearFlag => false,
        };

        gb.cpu.set_flag(CpuFlag::Zero,      null_bit);
        gb.cpu.set_flag(CpuFlag::Negative,  false);
        gb.cpu.set_flag(CpuFlag::HalfCarry, false);
        gb.cpu.set_flag(CpuFlag::Carry,     right_bit != 0);

        result
    }

    /// Shifts or rotates a value on a 16bit pointer to the right.
    fn shift_right_r16ptr(gb: &mut EmulatorCore, ctx: &mut OpCodeContext, r16ptr: RegisterR16, op: ShiftOp) -> OpCodeResult {
        match ctx.get_stage() {
            0 => {
                let address = gb.cpu.get_r16(r16ptr);
                let value   = gb.get_mmu().read_u8(address);
                gb.cpu.set_intermediate_value(value);

                OpCodeResult::StageDone(4)
            }

            1 => {
                let address = gb.cpu.get_r16(r16ptr);
                let value   = gb.cpu.get_intermediate_value();
                let result  = shift_right_u8v(gb, value, op);
                gb.get_mmu_mut().write_u8(address, result);

                OpCodeResult::Done
            }

            _ => unreachable!()
        }
    }


    /// Performs an arithmetic shift right of the value of a register.
    fn sra_r8(gb: &mut EmulatorCore, r8: RegisterR8) {
        let value  = gb.cpu.get_r8(r8);
        let result = shift_right_u8v(gb, value, ShiftOp::ShiftArithmetic);
        gb.cpu.set_r8(r8, result);
    }

    /// Performs an arithmetic shift right of the value on a memory location.
    fn sra_r16ptr(gb: &mut EmulatorCore, ctx: &mut OpCodeContext, r16ptr: RegisterR16) -> OpCodeResult {
        shift_right_r16ptr(gb, ctx, r16ptr, ShiftOp::ShiftArithmetic)
    }

    /// Performs an arithmetic shift right of the value of a register.
    fn srl_r8(gb: &mut EmulatorCore, r8: RegisterR8) {
        let value  = gb.cpu.get_r8(r8);
        let result = shift_right_u8v(gb, value, ShiftOp::ShiftLogical);
        gb.cpu.set_r8(r8, result);
    }

    /// Performs an arithmetic shift right of the value on a memory location.
    fn srl_r16ptr(gb: &mut EmulatorCore, ctx: &mut OpCodeContext, r16ptr: RegisterR16) -> OpCodeResult {
        shift_right_r16ptr(gb, ctx, r16ptr, ShiftOp::ShiftLogical)
    }

    /// Rotates the value of a register to the right through the carry flag.
    fn rr_r8(gb: &mut EmulatorCore, r8: RegisterR8) {
        rr_r8_nc(gb, r8, NullCheck::Check);
    }

    /// Rotates the value of a register to the right through the carry flag.
    fn rr_r8_nc(gb: &mut EmulatorCore, r8: RegisterR8, nullcheck: NullCheck) {
        let value  = gb.cpu.get_r8(r8);
        let result = shift_right_u8v_nc(gb, value, ShiftOp::RotateThroughCarry, nullcheck);
        gb.cpu.set_r8(r8, result);
    }

    /// Rotates the value on a memory location to the right through the carry flag.
    fn rr_r16ptr(gb: &mut EmulatorCore, ctx: &mut OpCodeContext, r16ptr: RegisterR16) -> OpCodeResult {
        shift_right_r16ptr(gb, ctx, r16ptr, ShiftOp::RotateThroughCarry)
    }

    /// Rotates the value of a register to the right.
    fn rrc_r8(gb: &mut EmulatorCore, r8: RegisterR8) {
        rrc_r8_nc(gb, r8, NullCheck::Check);
    }

    /// Rotates the value of a register to the right.
    fn rrc_r8_nc(gb: &mut EmulatorCore, r8: RegisterR8, nullcheck: NullCheck) {
        let value  = gb.cpu.get_r8(r8);
        let result = shift_right_u8v_nc(gb, value, ShiftOp::Rotate, nullcheck);
        gb.cpu.set_r8(r8, result);
    }

    /// Rotates the value on a memory location to the right.
    fn rrc_r16ptr(gb: &mut EmulatorCore, ctx: &mut OpCodeContext, r16ptr: RegisterR16) -> OpCodeResult {
        shift_right_r16ptr(gb, ctx, r16ptr, ShiftOp::Rotate)
    }


    // arithmetic shift right
    opcode!(sra_a,     [gb] sra_r8(gb, RegisterR8::A));
    opcode!(sra_b,     [gb] sra_r8(gb, RegisterR8::B));
    opcode!(sra_c,     [gb] sra_r8(gb, RegisterR8::C));
    opcode!(sra_d,     [gb] sra_r8(gb, RegisterR8::D));
    opcode!(sra_e,     [gb] sra_r8(gb, RegisterR8::E));
    opcode!(sra_h,     [gb] sra_r8(gb, RegisterR8::H));
    opcode!(sra_l,     [gb] sra_r8(gb, RegisterR8::L));

    // logical shift right
    opcode!(srl_a,     [gb] srl_r8(gb, RegisterR8::A));
    opcode!(srl_b,     [gb] srl_r8(gb, RegisterR8::B));
    opcode!(srl_c,     [gb] srl_r8(gb, RegisterR8::C));
    opcode!(srl_d,     [gb] srl_r8(gb, RegisterR8::D));
    opcode!(srl_e,     [gb] srl_r8(gb, RegisterR8::E));
    opcode!(srl_h,     [gb] srl_r8(gb, RegisterR8::H));
    opcode!(srl_l,     [gb] srl_r8(gb, RegisterR8::L));

    // rotate right through carry flag
    opcode!(rra,      [gb] rr_r8_nc(gb, RegisterR8::A, NullCheck::ClearFlag));
    opcode!(rr_a,     [gb] rr_r8(gb, RegisterR8::A));
    opcode!(rr_b,     [gb] rr_r8(gb, RegisterR8::B));
    opcode!(rr_c,     [gb] rr_r8(gb, RegisterR8::C));
    opcode!(rr_d,     [gb] rr_r8(gb, RegisterR8::D));
    opcode!(rr_e,     [gb] rr_r8(gb, RegisterR8::E));
    opcode!(rr_h,     [gb] rr_r8(gb, RegisterR8::H));
    opcode!(rr_l,     [gb] rr_r8(gb, RegisterR8::L));

    // rotate right (carry flag just set)
    opcode!(rrca,      [gb] rrc_r8_nc(gb, RegisterR8::A, NullCheck::ClearFlag));
    opcode!(rrc_a,     [gb] rrc_r8(gb, RegisterR8::A));
    opcode!(rrc_b,     [gb] rrc_r8(gb, RegisterR8::B));
    opcode!(rrc_c,     [gb] rrc_r8(gb, RegisterR8::C));
    opcode!(rrc_d,     [gb] rrc_r8(gb, RegisterR8::D));
    opcode!(rrc_e,     [gb] rrc_r8(gb, RegisterR8::E));
    opcode!(rrc_h,     [gb] rrc_r8(gb, RegisterR8::H));
    opcode!(rrc_l,     [gb] rrc_r8(gb, RegisterR8::L));

    opcode!(sra_hlptr, [gb, ctx] sra_r16ptr(gb, ctx, RegisterR16::HL));
    opcode!(srl_hlptr, [gb, ctx] srl_r16ptr(gb, ctx, RegisterR16::HL));
    opcode!(rr_hlptr,  [gb, ctx] rr_r16ptr (gb, ctx, RegisterR16::HL));
    opcode!(rrc_hlptr, [gb, ctx] rrc_r16ptr(gb, ctx, RegisterR16::HL));
}

////////////////////////////////////////////////
//// SWAP opcodes
pub mod swap {
    use super::*;

    /// Swaps the low and high nibble of a byte.
    fn swap_nibbles_u8v(gb: &mut EmulatorCore, value: u8) -> u8 {
        let low   = (value >> 0) & 0x0f;
        let high  = (value >> 4) & 0x0f;
        let result = (low << 4) | (high);

        gb.cpu.clear_flags();
        gb.cpu.set_flag(CpuFlag::Zero, result == 0);

        result
    }

    /// Swaps the low and high nibble of a 8bit register.
    fn swap_r8(gb: &mut EmulatorCore, r8: RegisterR8) {
        let value  = gb.cpu.get_r8(r8);
        let result = swap_nibbles_u8v(gb, value);
        gb.cpu.set_r8(r8, result);
    }

    /// Swaps the low and high nibble of a byte at the address of a 16bit register pointer.
    fn swap_r16ptr(gb: &mut EmulatorCore, ctx: &mut OpCodeContext, r16_ptr: RegisterR16) -> OpCodeResult {
        match ctx.get_stage() {
            0 => {
                let address = gb.cpu.get_r16(r16_ptr);
                let value   = gb.get_mmu().read_u8(address);
                gb.cpu.set_intermediate_value(value);

                OpCodeResult::StageDone(4)
            }

            1 => {
                let address = gb.cpu.get_r16(r16_ptr);
                let value   = gb.cpu.get_intermediate_value();
                let result  = swap_nibbles_u8v(gb, value);
                gb.get_mmu_mut().write_u8(address, result);

                OpCodeResult::Done
            }

            _ => unreachable!()
        }
    }


    // swap low and high nibbles of registers
    opcode!(swap_a,     [gb] swap_r8(gb, RegisterR8::A));
    opcode!(swap_b,     [gb] swap_r8(gb, RegisterR8::B));
    opcode!(swap_c,     [gb] swap_r8(gb, RegisterR8::C));
    opcode!(swap_d,     [gb] swap_r8(gb, RegisterR8::D));
    opcode!(swap_e,     [gb] swap_r8(gb, RegisterR8::E));
    opcode!(swap_h,     [gb] swap_r8(gb, RegisterR8::H));
    opcode!(swap_l,     [gb] swap_r8(gb, RegisterR8::L));

    opcode!(swap_hlptr, [gb, ctx] swap_r16ptr(gb, ctx, RegisterR16::HL));
}

////////////////////////////////////////////////
//// Set Bit opcodes
pub mod set_bit {
    use super::*;

    /// Set bit n of a given 8bit value.
    /// value | (1 << bit)
    fn set_bit_u8v(_gb: &mut EmulatorCore, value: u8, bit: u8) -> u8 {
        let result = value | (1 << bit);
        result
    }

    /// Set bit n of in the given register.
    /// r8 <- r8 | (1 << bit)
    fn set_bit_r8(gb: &mut EmulatorCore, r8: RegisterR8, bit: u8) {
        let value  = gb.cpu.get_r8(r8);
        let result = set_bit_u8v(gb, value, bit);
        gb.cpu.set_r8(r8, result);
    }

    /// Set bit n on a memory address.
    /// (r16) <- (r16) | (1 << bit)
    fn set_bit_r16ptr(gb: &mut EmulatorCore, ctx: &mut OpCodeContext, r16_ptr: RegisterR16, bit: u8) -> OpCodeResult {
        match ctx.get_stage() {
            0 => {
                let address = gb.cpu.get_r16(r16_ptr);
                let value   = gb.get_mmu().read_u8(address);
                gb.cpu.set_intermediate_value(value);

                OpCodeResult::StageDone(4)
            }

            1 => {
                let address = gb.cpu.get_r16(r16_ptr);
                let value   = gb.cpu.get_intermediate_value();
                let result  = set_bit_u8v(gb, value, bit);
                gb.get_mmu_mut().write_u8(address, result);

                OpCodeResult::Done
            }

            _ => unreachable!()
        }
    }


    opcode!(set_bit_0_a, [gb] set_bit_r8(gb, RegisterR8::A, 0));
    opcode!(set_bit_0_b, [gb] set_bit_r8(gb, RegisterR8::B, 0));
    opcode!(set_bit_0_c, [gb] set_bit_r8(gb, RegisterR8::C, 0));
    opcode!(set_bit_0_d, [gb] set_bit_r8(gb, RegisterR8::D, 0));
    opcode!(set_bit_0_e, [gb] set_bit_r8(gb, RegisterR8::E, 0));
    opcode!(set_bit_0_h, [gb] set_bit_r8(gb, RegisterR8::H, 0));
    opcode!(set_bit_0_l, [gb] set_bit_r8(gb, RegisterR8::L, 0));

    opcode!(set_bit_1_a, [gb] set_bit_r8(gb, RegisterR8::A, 1));
    opcode!(set_bit_1_b, [gb] set_bit_r8(gb, RegisterR8::B, 1));
    opcode!(set_bit_1_c, [gb] set_bit_r8(gb, RegisterR8::C, 1));
    opcode!(set_bit_1_d, [gb] set_bit_r8(gb, RegisterR8::D, 1));
    opcode!(set_bit_1_e, [gb] set_bit_r8(gb, RegisterR8::E, 1));
    opcode!(set_bit_1_h, [gb] set_bit_r8(gb, RegisterR8::H, 1));
    opcode!(set_bit_1_l, [gb] set_bit_r8(gb, RegisterR8::L, 1));

    opcode!(set_bit_2_a, [gb] set_bit_r8(gb, RegisterR8::A, 2));
    opcode!(set_bit_2_b, [gb] set_bit_r8(gb, RegisterR8::B, 2));
    opcode!(set_bit_2_c, [gb] set_bit_r8(gb, RegisterR8::C, 2));
    opcode!(set_bit_2_d, [gb] set_bit_r8(gb, RegisterR8::D, 2));
    opcode!(set_bit_2_e, [gb] set_bit_r8(gb, RegisterR8::E, 2));
    opcode!(set_bit_2_h, [gb] set_bit_r8(gb, RegisterR8::H, 2));
    opcode!(set_bit_2_l, [gb] set_bit_r8(gb, RegisterR8::L, 2));

    opcode!(set_bit_3_a, [gb] set_bit_r8(gb, RegisterR8::A, 3));
    opcode!(set_bit_3_b, [gb] set_bit_r8(gb, RegisterR8::B, 3));
    opcode!(set_bit_3_c, [gb] set_bit_r8(gb, RegisterR8::C, 3));
    opcode!(set_bit_3_d, [gb] set_bit_r8(gb, RegisterR8::D, 3));
    opcode!(set_bit_3_e, [gb] set_bit_r8(gb, RegisterR8::E, 3));
    opcode!(set_bit_3_h, [gb] set_bit_r8(gb, RegisterR8::H, 3));
    opcode!(set_bit_3_l, [gb] set_bit_r8(gb, RegisterR8::L, 3));

    opcode!(set_bit_4_a, [gb] set_bit_r8(gb, RegisterR8::A, 4));
    opcode!(set_bit_4_b, [gb] set_bit_r8(gb, RegisterR8::B, 4));
    opcode!(set_bit_4_c, [gb] set_bit_r8(gb, RegisterR8::C, 4));
    opcode!(set_bit_4_d, [gb] set_bit_r8(gb, RegisterR8::D, 4));
    opcode!(set_bit_4_e, [gb] set_bit_r8(gb, RegisterR8::E, 4));
    opcode!(set_bit_4_h, [gb] set_bit_r8(gb, RegisterR8::H, 4));
    opcode!(set_bit_4_l, [gb] set_bit_r8(gb, RegisterR8::L, 4));

    opcode!(set_bit_5_a, [gb] set_bit_r8(gb, RegisterR8::A, 5));
    opcode!(set_bit_5_b, [gb] set_bit_r8(gb, RegisterR8::B, 5));
    opcode!(set_bit_5_c, [gb] set_bit_r8(gb, RegisterR8::C, 5));
    opcode!(set_bit_5_d, [gb] set_bit_r8(gb, RegisterR8::D, 5));
    opcode!(set_bit_5_e, [gb] set_bit_r8(gb, RegisterR8::E, 5));
    opcode!(set_bit_5_h, [gb] set_bit_r8(gb, RegisterR8::H, 5));
    opcode!(set_bit_5_l, [gb] set_bit_r8(gb, RegisterR8::L, 5));

    opcode!(set_bit_6_a, [gb] set_bit_r8(gb, RegisterR8::A, 6));
    opcode!(set_bit_6_b, [gb] set_bit_r8(gb, RegisterR8::B, 6));
    opcode!(set_bit_6_c, [gb] set_bit_r8(gb, RegisterR8::C, 6));
    opcode!(set_bit_6_d, [gb] set_bit_r8(gb, RegisterR8::D, 6));
    opcode!(set_bit_6_e, [gb] set_bit_r8(gb, RegisterR8::E, 6));
    opcode!(set_bit_6_h, [gb] set_bit_r8(gb, RegisterR8::H, 6));
    opcode!(set_bit_6_l, [gb] set_bit_r8(gb, RegisterR8::L, 6));

    opcode!(set_bit_7_a, [gb] set_bit_r8(gb, RegisterR8::A, 7));
    opcode!(set_bit_7_b, [gb] set_bit_r8(gb, RegisterR8::B, 7));
    opcode!(set_bit_7_c, [gb] set_bit_r8(gb, RegisterR8::C, 7));
    opcode!(set_bit_7_d, [gb] set_bit_r8(gb, RegisterR8::D, 7));
    opcode!(set_bit_7_e, [gb] set_bit_r8(gb, RegisterR8::E, 7));
    opcode!(set_bit_7_h, [gb] set_bit_r8(gb, RegisterR8::H, 7));
    opcode!(set_bit_7_l, [gb] set_bit_r8(gb, RegisterR8::L, 7));

    opcode!(set_bit_0_hlptr, [gb, ctx] set_bit_r16ptr(gb, ctx, RegisterR16::HL, 0));
    opcode!(set_bit_1_hlptr, [gb, ctx] set_bit_r16ptr(gb, ctx, RegisterR16::HL, 1));
    opcode!(set_bit_2_hlptr, [gb, ctx] set_bit_r16ptr(gb, ctx, RegisterR16::HL, 2));
    opcode!(set_bit_3_hlptr, [gb, ctx] set_bit_r16ptr(gb, ctx, RegisterR16::HL, 3));
    opcode!(set_bit_4_hlptr, [gb, ctx] set_bit_r16ptr(gb, ctx, RegisterR16::HL, 4));
    opcode!(set_bit_5_hlptr, [gb, ctx] set_bit_r16ptr(gb, ctx, RegisterR16::HL, 5));
    opcode!(set_bit_6_hlptr, [gb, ctx] set_bit_r16ptr(gb, ctx, RegisterR16::HL, 6));
    opcode!(set_bit_7_hlptr, [gb, ctx] set_bit_r16ptr(gb, ctx, RegisterR16::HL, 7));
}

////////////////////////////////////////////////
//// Reset Bit opcodes
pub mod res_bit {
    use super::*;

    /// Resets bit n of a given 8bit value.
    /// value & !(1 << bit)
    fn res_bit_u8v(_gb: &mut EmulatorCore, value: u8, bit: u8) -> u8 {
        let result = value & !(1 << bit);
        result
    }

    /// Resets bit n of in the given register.
    /// r8 <- r8 & !(1 << bit)
    fn res_bit_r8(gb: &mut EmulatorCore, r8: RegisterR8, bit: u8) {
        let value  = gb.cpu.get_r8(r8);
        let result = res_bit_u8v(gb, value, bit);
        gb.cpu.set_r8(r8, result);
    }

    /// Resets bit n on a memory address.
    /// (r16) <- (r16) & !(1 << bit)
    fn res_bit_r16ptr(gb: &mut EmulatorCore, ctx: &mut OpCodeContext, r16_ptr: RegisterR16, bit: u8) -> OpCodeResult {
        match ctx.get_stage() {
            0 => {
                let address = gb.cpu.get_r16(r16_ptr);
                let value   = gb.get_mmu().read_u8(address);
                gb.cpu.set_intermediate_value(value);

                OpCodeResult::StageDone(4)
            }

            1 => {
                let address = gb.cpu.get_r16(r16_ptr);
                let value   = gb.cpu.get_intermediate_value();
                let result  = res_bit_u8v(gb, value, bit);
                gb.get_mmu_mut().write_u8(address, result);

                OpCodeResult::Done
            }

            _ => unreachable!()
        }
    }


    opcode!(res_bit_0_a, [gb] res_bit_r8(gb, RegisterR8::A, 0));
    opcode!(res_bit_0_b, [gb] res_bit_r8(gb, RegisterR8::B, 0));
    opcode!(res_bit_0_c, [gb] res_bit_r8(gb, RegisterR8::C, 0));
    opcode!(res_bit_0_d, [gb] res_bit_r8(gb, RegisterR8::D, 0));
    opcode!(res_bit_0_e, [gb] res_bit_r8(gb, RegisterR8::E, 0));
    opcode!(res_bit_0_h, [gb] res_bit_r8(gb, RegisterR8::H, 0));
    opcode!(res_bit_0_l, [gb] res_bit_r8(gb, RegisterR8::L, 0));

    opcode!(res_bit_1_a, [gb] res_bit_r8(gb, RegisterR8::A, 1));
    opcode!(res_bit_1_b, [gb] res_bit_r8(gb, RegisterR8::B, 1));
    opcode!(res_bit_1_c, [gb] res_bit_r8(gb, RegisterR8::C, 1));
    opcode!(res_bit_1_d, [gb] res_bit_r8(gb, RegisterR8::D, 1));
    opcode!(res_bit_1_e, [gb] res_bit_r8(gb, RegisterR8::E, 1));
    opcode!(res_bit_1_h, [gb] res_bit_r8(gb, RegisterR8::H, 1));
    opcode!(res_bit_1_l, [gb] res_bit_r8(gb, RegisterR8::L, 1));

    opcode!(res_bit_2_a, [gb] res_bit_r8(gb, RegisterR8::A, 2));
    opcode!(res_bit_2_b, [gb] res_bit_r8(gb, RegisterR8::B, 2));
    opcode!(res_bit_2_c, [gb] res_bit_r8(gb, RegisterR8::C, 2));
    opcode!(res_bit_2_d, [gb] res_bit_r8(gb, RegisterR8::D, 2));
    opcode!(res_bit_2_e, [gb] res_bit_r8(gb, RegisterR8::E, 2));
    opcode!(res_bit_2_h, [gb] res_bit_r8(gb, RegisterR8::H, 2));
    opcode!(res_bit_2_l, [gb] res_bit_r8(gb, RegisterR8::L, 2));

    opcode!(res_bit_3_a, [gb] res_bit_r8(gb, RegisterR8::A, 3));
    opcode!(res_bit_3_b, [gb] res_bit_r8(gb, RegisterR8::B, 3));
    opcode!(res_bit_3_c, [gb] res_bit_r8(gb, RegisterR8::C, 3));
    opcode!(res_bit_3_d, [gb] res_bit_r8(gb, RegisterR8::D, 3));
    opcode!(res_bit_3_e, [gb] res_bit_r8(gb, RegisterR8::E, 3));
    opcode!(res_bit_3_h, [gb] res_bit_r8(gb, RegisterR8::H, 3));
    opcode!(res_bit_3_l, [gb] res_bit_r8(gb, RegisterR8::L, 3));

    opcode!(res_bit_4_a, [gb] res_bit_r8(gb, RegisterR8::A, 4));
    opcode!(res_bit_4_b, [gb] res_bit_r8(gb, RegisterR8::B, 4));
    opcode!(res_bit_4_c, [gb] res_bit_r8(gb, RegisterR8::C, 4));
    opcode!(res_bit_4_d, [gb] res_bit_r8(gb, RegisterR8::D, 4));
    opcode!(res_bit_4_e, [gb] res_bit_r8(gb, RegisterR8::E, 4));
    opcode!(res_bit_4_h, [gb] res_bit_r8(gb, RegisterR8::H, 4));
    opcode!(res_bit_4_l, [gb] res_bit_r8(gb, RegisterR8::L, 4));

    opcode!(res_bit_5_a, [gb] res_bit_r8(gb, RegisterR8::A, 5));
    opcode!(res_bit_5_b, [gb] res_bit_r8(gb, RegisterR8::B, 5));
    opcode!(res_bit_5_c, [gb] res_bit_r8(gb, RegisterR8::C, 5));
    opcode!(res_bit_5_d, [gb] res_bit_r8(gb, RegisterR8::D, 5));
    opcode!(res_bit_5_e, [gb] res_bit_r8(gb, RegisterR8::E, 5));
    opcode!(res_bit_5_h, [gb] res_bit_r8(gb, RegisterR8::H, 5));
    opcode!(res_bit_5_l, [gb] res_bit_r8(gb, RegisterR8::L, 5));

    opcode!(res_bit_6_a, [gb] res_bit_r8(gb, RegisterR8::A, 6));
    opcode!(res_bit_6_b, [gb] res_bit_r8(gb, RegisterR8::B, 6));
    opcode!(res_bit_6_c, [gb] res_bit_r8(gb, RegisterR8::C, 6));
    opcode!(res_bit_6_d, [gb] res_bit_r8(gb, RegisterR8::D, 6));
    opcode!(res_bit_6_e, [gb] res_bit_r8(gb, RegisterR8::E, 6));
    opcode!(res_bit_6_h, [gb] res_bit_r8(gb, RegisterR8::H, 6));
    opcode!(res_bit_6_l, [gb] res_bit_r8(gb, RegisterR8::L, 6));

    opcode!(res_bit_7_a, [gb] res_bit_r8(gb, RegisterR8::A, 7));
    opcode!(res_bit_7_b, [gb] res_bit_r8(gb, RegisterR8::B, 7));
    opcode!(res_bit_7_c, [gb] res_bit_r8(gb, RegisterR8::C, 7));
    opcode!(res_bit_7_d, [gb] res_bit_r8(gb, RegisterR8::D, 7));
    opcode!(res_bit_7_e, [gb] res_bit_r8(gb, RegisterR8::E, 7));
    opcode!(res_bit_7_h, [gb] res_bit_r8(gb, RegisterR8::H, 7));
    opcode!(res_bit_7_l, [gb] res_bit_r8(gb, RegisterR8::L, 7));

    opcode!(res_bit_0_hlptr, [gb, ctx] res_bit_r16ptr(gb, ctx, RegisterR16::HL, 0));
    opcode!(res_bit_1_hlptr, [gb, ctx] res_bit_r16ptr(gb, ctx, RegisterR16::HL, 1));
    opcode!(res_bit_2_hlptr, [gb, ctx] res_bit_r16ptr(gb, ctx, RegisterR16::HL, 2));
    opcode!(res_bit_3_hlptr, [gb, ctx] res_bit_r16ptr(gb, ctx, RegisterR16::HL, 3));
    opcode!(res_bit_4_hlptr, [gb, ctx] res_bit_r16ptr(gb, ctx, RegisterR16::HL, 4));
    opcode!(res_bit_5_hlptr, [gb, ctx] res_bit_r16ptr(gb, ctx, RegisterR16::HL, 5));
    opcode!(res_bit_6_hlptr, [gb, ctx] res_bit_r16ptr(gb, ctx, RegisterR16::HL, 6));
    opcode!(res_bit_7_hlptr, [gb, ctx] res_bit_r16ptr(gb, ctx, RegisterR16::HL, 7));
}

////////////////////////////////////////////////
//// Check Bit opcodes
pub mod chk_bit {
    use super::*;

    /// Checks if bit n of a value is set.
    /// Set the Zero flag, if the bit was 0.
    fn check_bit_u8v(gb: &mut EmulatorCore, value: u8, bit: u8) {
        let result = value & (1 << bit);
        gb.cpu.set_flag(CpuFlag::Zero,      result == 0);
        gb.cpu.set_flag(CpuFlag::Negative,  false);
        gb.cpu.set_flag(CpuFlag::HalfCarry, true);
    }

    /// Checks if bit n of a register is set.
    /// Set the Zero flag, if the bit was 0.
    fn check_bit_r8(gb: &mut EmulatorCore, r8: RegisterR8, bit: u8) {
        let value  = gb.cpu.get_r8(r8);
        check_bit_u8v(gb, value, bit);
    }

    /// Checks if bit n on a memory address is set.
    /// Set the Zero flag, if the bit was 0.
    fn check_bit_r16ptr(gb: &mut EmulatorCore, r16_ptr: RegisterR16, bit: u8) {
        let address = gb.cpu.get_r16(r16_ptr);
        let value   = gb.get_mmu().read_u8(address);
        check_bit_u8v(gb, value, bit);
    }


    opcode!(check_bit_0_a,     [gb] check_bit_r8(gb, RegisterR8::A, 0));
    opcode!(check_bit_0_b,     [gb] check_bit_r8(gb, RegisterR8::B, 0));
    opcode!(check_bit_0_c,     [gb] check_bit_r8(gb, RegisterR8::C, 0));
    opcode!(check_bit_0_d,     [gb] check_bit_r8(gb, RegisterR8::D, 0));
    opcode!(check_bit_0_e,     [gb] check_bit_r8(gb, RegisterR8::E, 0));
    opcode!(check_bit_0_h,     [gb] check_bit_r8(gb, RegisterR8::H, 0));
    opcode!(check_bit_0_l,     [gb] check_bit_r8(gb, RegisterR8::L, 0));
    opcode!(check_bit_0_hlptr, [gb] check_bit_r16ptr(gb, RegisterR16::HL, 0));

    opcode!(check_bit_1_a,     [gb] check_bit_r8(gb, RegisterR8::A, 1));
    opcode!(check_bit_1_b,     [gb] check_bit_r8(gb, RegisterR8::B, 1));
    opcode!(check_bit_1_c,     [gb] check_bit_r8(gb, RegisterR8::C, 1));
    opcode!(check_bit_1_d,     [gb] check_bit_r8(gb, RegisterR8::D, 1));
    opcode!(check_bit_1_e,     [gb] check_bit_r8(gb, RegisterR8::E, 1));
    opcode!(check_bit_1_h,     [gb] check_bit_r8(gb, RegisterR8::H, 1));
    opcode!(check_bit_1_l,     [gb] check_bit_r8(gb, RegisterR8::L, 1));
    opcode!(check_bit_1_hlptr, [gb] check_bit_r16ptr(gb, RegisterR16::HL, 1));

    opcode!(check_bit_2_a,     [gb] check_bit_r8(gb, RegisterR8::A, 2));
    opcode!(check_bit_2_b,     [gb] check_bit_r8(gb, RegisterR8::B, 2));
    opcode!(check_bit_2_c,     [gb] check_bit_r8(gb, RegisterR8::C, 2));
    opcode!(check_bit_2_d,     [gb] check_bit_r8(gb, RegisterR8::D, 2));
    opcode!(check_bit_2_e,     [gb] check_bit_r8(gb, RegisterR8::E, 2));
    opcode!(check_bit_2_h,     [gb] check_bit_r8(gb, RegisterR8::H, 2));
    opcode!(check_bit_2_l,     [gb] check_bit_r8(gb, RegisterR8::L, 2));
    opcode!(check_bit_2_hlptr, [gb] check_bit_r16ptr(gb, RegisterR16::HL, 2));

    opcode!(check_bit_3_a,     [gb] check_bit_r8(gb, RegisterR8::A, 3));
    opcode!(check_bit_3_b,     [gb] check_bit_r8(gb, RegisterR8::B, 3));
    opcode!(check_bit_3_c,     [gb] check_bit_r8(gb, RegisterR8::C, 3));
    opcode!(check_bit_3_d,     [gb] check_bit_r8(gb, RegisterR8::D, 3));
    opcode!(check_bit_3_e,     [gb] check_bit_r8(gb, RegisterR8::E, 3));
    opcode!(check_bit_3_h,     [gb] check_bit_r8(gb, RegisterR8::H, 3));
    opcode!(check_bit_3_l,     [gb] check_bit_r8(gb, RegisterR8::L, 3));
    opcode!(check_bit_3_hlptr, [gb] check_bit_r16ptr(gb, RegisterR16::HL, 3));

    opcode!(check_bit_4_a,     [gb] check_bit_r8(gb, RegisterR8::A, 4));
    opcode!(check_bit_4_b,     [gb] check_bit_r8(gb, RegisterR8::B, 4));
    opcode!(check_bit_4_c,     [gb] check_bit_r8(gb, RegisterR8::C, 4));
    opcode!(check_bit_4_d,     [gb] check_bit_r8(gb, RegisterR8::D, 4));
    opcode!(check_bit_4_e,     [gb] check_bit_r8(gb, RegisterR8::E, 4));
    opcode!(check_bit_4_h,     [gb] check_bit_r8(gb, RegisterR8::H, 4));
    opcode!(check_bit_4_l,     [gb] check_bit_r8(gb, RegisterR8::L, 4));
    opcode!(check_bit_4_hlptr, [gb] check_bit_r16ptr(gb, RegisterR16::HL, 4));

    opcode!(check_bit_5_a,     [gb] check_bit_r8(gb, RegisterR8::A, 5));
    opcode!(check_bit_5_b,     [gb] check_bit_r8(gb, RegisterR8::B, 5));
    opcode!(check_bit_5_c,     [gb] check_bit_r8(gb, RegisterR8::C, 5));
    opcode!(check_bit_5_d,     [gb] check_bit_r8(gb, RegisterR8::D, 5));
    opcode!(check_bit_5_e,     [gb] check_bit_r8(gb, RegisterR8::E, 5));
    opcode!(check_bit_5_h,     [gb] check_bit_r8(gb, RegisterR8::H, 5));
    opcode!(check_bit_5_l,     [gb] check_bit_r8(gb, RegisterR8::L, 5));
    opcode!(check_bit_5_hlptr, [gb] check_bit_r16ptr(gb, RegisterR16::HL, 5));

    opcode!(check_bit_6_a,     [gb] check_bit_r8(gb, RegisterR8::A, 6));
    opcode!(check_bit_6_b,     [gb] check_bit_r8(gb, RegisterR8::B, 6));
    opcode!(check_bit_6_c,     [gb] check_bit_r8(gb, RegisterR8::C, 6));
    opcode!(check_bit_6_d,     [gb] check_bit_r8(gb, RegisterR8::D, 6));
    opcode!(check_bit_6_e,     [gb] check_bit_r8(gb, RegisterR8::E, 6));
    opcode!(check_bit_6_h,     [gb] check_bit_r8(gb, RegisterR8::H, 6));
    opcode!(check_bit_6_l,     [gb] check_bit_r8(gb, RegisterR8::L, 6));
    opcode!(check_bit_6_hlptr, [gb] check_bit_r16ptr(gb, RegisterR16::HL, 6));

    opcode!(check_bit_7_a,     [gb] check_bit_r8(gb, RegisterR8::A, 7));
    opcode!(check_bit_7_b,     [gb] check_bit_r8(gb, RegisterR8::B, 7));
    opcode!(check_bit_7_c,     [gb] check_bit_r8(gb, RegisterR8::C, 7));
    opcode!(check_bit_7_d,     [gb] check_bit_r8(gb, RegisterR8::D, 7));
    opcode!(check_bit_7_e,     [gb] check_bit_r8(gb, RegisterR8::E, 7));
    opcode!(check_bit_7_h,     [gb] check_bit_r8(gb, RegisterR8::H, 7));
    opcode!(check_bit_7_l,     [gb] check_bit_r8(gb, RegisterR8::L, 7));
    opcode!(check_bit_7_hlptr, [gb] check_bit_r16ptr(gb, RegisterR16::HL, 7));
}

////////////////////////////////////////////////
//// CP opcodes
pub mod cp {
    use super::*;

    /// Compares two values.
    fn cp_u8v_u8v(gb: &mut EmulatorCore, value1: u8, value2: u8) {
        let (_, half_carry, carry) = carrying_sub_u8(value1, value2, false);

        gb.cpu.set_flag(CpuFlag::Zero,      value1 == value2);
        gb.cpu.set_flag(CpuFlag::Negative,  true);
        gb.cpu.set_flag(CpuFlag::HalfCarry, half_carry);
        gb.cpu.set_flag(CpuFlag::Carry,     carry);
    }

    /// Compares two values.
    /// cp r8, u8
    fn cp_r8_u8(gb: &mut EmulatorCore, r8: RegisterR8) {
        let value1 = gb.cpu.get_r8(r8);
        let value2 = gb.cpu.fetch_u8();
        cp_u8v_u8v(gb, value1, value2);
    }

    /// Compares two values.
    /// cp dst, src
    fn cp_r8_r8(gb: &mut EmulatorCore, dst: RegisterR8, src: RegisterR8) {
        let value1 = gb.cpu.get_r8(dst);
        let value2 = gb.cpu.get_r8(src);
        cp_u8v_u8v(gb, value1, value2);
    }

    /// Compares two values.
    /// cp dst, (src_ptr)
    fn cp_r8_r16ptr(gb: &mut EmulatorCore, dst: RegisterR8, src_ptr: RegisterR16) {
        let value1  = gb.cpu.get_r8(dst);
        let address = gb.cpu.get_r16(src_ptr);
        let value2  = gb.get_mmu().read_u8(address);
        cp_u8v_u8v(gb, value1, value2);
    }


    opcode!(cp_a_u8,    [gb] cp_r8_u8(gb, RegisterR8::A));
    opcode!(cp_a_a,     [gb] cp_r8_r8(gb, RegisterR8::A, RegisterR8::A));
    opcode!(cp_a_b,     [gb] cp_r8_r8(gb, RegisterR8::A, RegisterR8::B));
    opcode!(cp_a_c,     [gb] cp_r8_r8(gb, RegisterR8::A, RegisterR8::C));
    opcode!(cp_a_d,     [gb] cp_r8_r8(gb, RegisterR8::A, RegisterR8::D));
    opcode!(cp_a_e,     [gb] cp_r8_r8(gb, RegisterR8::A, RegisterR8::E));
    opcode!(cp_a_h,     [gb] cp_r8_r8(gb, RegisterR8::A, RegisterR8::H));
    opcode!(cp_a_l,     [gb] cp_r8_r8(gb, RegisterR8::A, RegisterR8::L));
    opcode!(cp_a_hlptr, [gb] cp_r8_r16ptr(gb, RegisterR8::A, RegisterR16::HL));
}

////////////////////////////////////////////////
//// AND opcodes
pub mod and {
    use super::*;

    /// Computes a bitwise AND.
    /// r8 <- r8 & value
    fn and_r8_u8v(gb: &mut EmulatorCore, r8: RegisterR8, value: u8) {
        let old_value = gb.cpu.get_r8(r8);
        let result    = old_value & value;
        gb.cpu.set_flag(CpuFlag::Zero,      result == 0);
        gb.cpu.set_flag(CpuFlag::Negative,  false);
        gb.cpu.set_flag(CpuFlag::HalfCarry, true);
        gb.cpu.set_flag(CpuFlag::Carry,     false);
        gb.cpu.set_r8(r8, result);
    }

    /// Computes a bitwise AND.
    /// dst <- dst & u8
    fn and_r8_u8(gb: &mut EmulatorCore, dst: RegisterR8) {
        let value = gb.cpu.fetch_u8();
        and_r8_u8v(gb, dst, value);
    }

    /// Computes a bitwise AND.
    /// dst <- dst & src
    fn and_r8_r8(gb: &mut EmulatorCore, dst: RegisterR8, src: RegisterR8) {
        let value = gb.cpu.get_r8(src);
        and_r8_u8v(gb, dst, value);
    }

    /// Computes a bitwise AND.
    /// dst <- dst & (src_ptr)
    fn and_r8_r16ptr(gb: &mut EmulatorCore, dst: RegisterR8, src_ptr: RegisterR16) {
        let address = gb.cpu.get_r16(src_ptr);
        let value   = gb.get_mmu().read_u8(address);
        and_r8_u8v(gb, dst, value);
    }


    opcode!(and_a_u8,    [gb] and_r8_u8(gb, RegisterR8::A));
    opcode!(and_a_a,     [gb] and_r8_r8(gb, RegisterR8::A, RegisterR8::A));
    opcode!(and_a_b,     [gb] and_r8_r8(gb, RegisterR8::A, RegisterR8::B));
    opcode!(and_a_c,     [gb] and_r8_r8(gb, RegisterR8::A, RegisterR8::C));
    opcode!(and_a_d,     [gb] and_r8_r8(gb, RegisterR8::A, RegisterR8::D));
    opcode!(and_a_e,     [gb] and_r8_r8(gb, RegisterR8::A, RegisterR8::E));
    opcode!(and_a_h,     [gb] and_r8_r8(gb, RegisterR8::A, RegisterR8::H));
    opcode!(and_a_l,     [gb] and_r8_r8(gb, RegisterR8::A, RegisterR8::L));
    opcode!(and_a_hlptr, [gb] and_r8_r16ptr(gb, RegisterR8::A, RegisterR16::HL));
}

////////////////////////////////////////////////
//// OR opcodes
pub mod or {
    use super::*;

    /// Computes a bitwise OR.
    /// r8 <- r8 | value
    fn or_r8_u8v(gb: &mut EmulatorCore, r8: RegisterR8, value: u8) {
        let old_value = gb.cpu.get_r8(r8);
        let result    = old_value | value;
        gb.cpu.set_flag(CpuFlag::Zero,      result == 0);
        gb.cpu.set_flag(CpuFlag::Negative,  false);
        gb.cpu.set_flag(CpuFlag::HalfCarry, false);
        gb.cpu.set_flag(CpuFlag::Carry,     false);
        gb.cpu.set_r8(r8, result);
    }

    /// Computes a bitwise OR.
    /// dst <- dst | u8
    fn or_r8_u8(gb: &mut EmulatorCore, dst: RegisterR8) {
        let value = gb.cpu.fetch_u8();
        or_r8_u8v(gb, dst, value);
    }

    /// Computes a bitwise OR.
    /// dst <- dst | src
    fn or_r8_r8(gb: &mut EmulatorCore, dst: RegisterR8, src: RegisterR8) {
        let value = gb.cpu.get_r8(src);
        or_r8_u8v(gb, dst, value);
    }

    /// Computes a bitwise OR.
    /// dst <- dst | (src_ptr)
    fn or_r8_r16ptr(gb: &mut EmulatorCore, dst: RegisterR8, src_ptr: RegisterR16) {
        let address = gb.cpu.get_r16(src_ptr);
        let value   = gb.get_mmu().read_u8(address);
        or_r8_u8v(gb, dst, value);
    }


    opcode!(or_a_u8,    [gb] or_r8_u8(gb, RegisterR8::A));
    opcode!(or_a_a,     [gb] or_r8_r8(gb, RegisterR8::A, RegisterR8::A));
    opcode!(or_a_b,     [gb] or_r8_r8(gb, RegisterR8::A, RegisterR8::B));
    opcode!(or_a_c,     [gb] or_r8_r8(gb, RegisterR8::A, RegisterR8::C));
    opcode!(or_a_d,     [gb] or_r8_r8(gb, RegisterR8::A, RegisterR8::D));
    opcode!(or_a_e,     [gb] or_r8_r8(gb, RegisterR8::A, RegisterR8::E));
    opcode!(or_a_h,     [gb] or_r8_r8(gb, RegisterR8::A, RegisterR8::H));
    opcode!(or_a_l,     [gb] or_r8_r8(gb, RegisterR8::A, RegisterR8::L));
    opcode!(or_a_hlptr, [gb] or_r8_r16ptr(gb, RegisterR8::A, RegisterR16::HL));
}

////////////////////////////////////////////////
//// XOR opcodes
pub mod xor {
    use super::*;

    /// Computes a bitwise XOR.
    /// r8 <- r8 ^ value
    fn xor_r8_u8v(gb: &mut EmulatorCore, r8: RegisterR8, value: u8) {
        let old_value = gb.cpu.get_r8(r8);
        let result    = old_value ^ value;
        gb.cpu.set_flag(CpuFlag::Zero,      result == 0);
        gb.cpu.set_flag(CpuFlag::Negative,  false);
        gb.cpu.set_flag(CpuFlag::HalfCarry, false);
        gb.cpu.set_flag(CpuFlag::Carry,     false);
        gb.cpu.set_r8(r8, result);
    }

    /// Computes a bitwise XOR.
    /// dst <- dst ^ src
    fn xor_r8_r8(gb: &mut EmulatorCore, dst: RegisterR8, src: RegisterR8) {
        let value = gb.cpu.get_r8(src);
        xor_r8_u8v(gb, dst, value);
    }

    /// Computes a bitwise XOR.
    /// dst <- dst ^ (src_ptr)
    fn xor_r8_r16ptr(gb: &mut EmulatorCore, dst: RegisterR8, src_ptr: RegisterR16) {
        let address = gb.cpu.get_r16(src_ptr);
        let value   = gb.get_mmu().read_u8(address);
        xor_r8_u8v(gb, dst, value);
    }


    fn xor_r8_u8(gb: &mut EmulatorCore, r8: RegisterR8) {
        let value = gb.cpu.fetch_u8();
        xor_r8_u8v(gb, r8, value);
    }


    opcode!(xor_a_u8,    [gb] xor_r8_u8(gb, RegisterR8::A));
    opcode!(xor_a_a,     [gb] xor_r8_r8(gb, RegisterR8::A, RegisterR8::A));
    opcode!(xor_a_b,     [gb] xor_r8_r8(gb, RegisterR8::A, RegisterR8::B));
    opcode!(xor_a_c,     [gb] xor_r8_r8(gb, RegisterR8::A, RegisterR8::C));
    opcode!(xor_a_d,     [gb] xor_r8_r8(gb, RegisterR8::A, RegisterR8::D));
    opcode!(xor_a_e,     [gb] xor_r8_r8(gb, RegisterR8::A, RegisterR8::E));
    opcode!(xor_a_h,     [gb] xor_r8_r8(gb, RegisterR8::A, RegisterR8::H));
    opcode!(xor_a_l,     [gb] xor_r8_r8(gb, RegisterR8::A, RegisterR8::L));
    opcode!(xor_a_hlptr, [gb] xor_r8_r16ptr(gb, RegisterR8::A, RegisterR16::HL));
}

////////////////////////////////////////////////
//// other

// Convert a BCD Number.
opcode!(daa, [gb] {
    let mut a     = gb.cpu.get_r8(RegisterR8::A);
    let mut half  = gb.cpu.is_flag_set(CpuFlag::HalfCarry);
    let mut carry = gb.cpu.is_flag_set(CpuFlag::Carry);
    let mut tmp_a = a as u16;

    if gb.cpu.is_flag_set(CpuFlag::Negative) {
        if half {
            tmp_a = tmp_a.wrapping_sub(0x06);

            if !carry {
                tmp_a &= 0xff;
            }
        }

        if carry {
            tmp_a = tmp_a.wrapping_sub(0x60);
        }
    }
    else {
        if half || ((tmp_a & 0x0f) >= 0x0a) {
            tmp_a = tmp_a.wrapping_add(0x06);
        }

        if carry || (tmp_a >= 0xa0) {
            tmp_a = tmp_a.wrapping_add(0x60);
        }
    }

    a      = (tmp_a & 0x00ff) as u8;
    carry |= (tmp_a & 0x0100) != 0;
    half   = false;

    gb.cpu.set_flag(CpuFlag::Zero,      a == 0);
    gb.cpu.set_flag(CpuFlag::HalfCarry, half);
    gb.cpu.set_flag(CpuFlag::Carry,     carry);

    gb.cpu.set_r8(RegisterR8::A, a);
});

// Complement
opcode!(cpl_a, [gb] {
    let value  = gb.cpu.get_r8(RegisterR8::A);
    let result = !value;
    gb.cpu.set_r8(RegisterR8::A, result);
    gb.cpu.set_flag(CpuFlag::Negative,  true);
    gb.cpu.set_flag(CpuFlag::HalfCarry, true);
});

// Set Carry flag.
opcode!(scf, [gb] {
    gb.cpu.set_flag(CpuFlag::Negative,  false);
    gb.cpu.set_flag(CpuFlag::HalfCarry, false);
    gb.cpu.set_flag(CpuFlag::Carry,     true);
});

// Change carry flag
opcode!(ccf, [gb] {
    gb.cpu.set_flag(CpuFlag::Negative,  false);
    gb.cpu.set_flag(CpuFlag::HalfCarry, false);
    gb.cpu.set_flag(CpuFlag::Carry,     !gb.cpu.is_flag_set(CpuFlag::Carry));
});

