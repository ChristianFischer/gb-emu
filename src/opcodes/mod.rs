/*
 * Copyright (C) 2022 by Christian Fischer
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

mod opcodes_arithmetic;
mod opcodes_control;
mod opcodes_jump;
mod opcodes_ld;

use crate::opcode::OpCode;

use crate::opcodes::opcodes_arithmetic::*;
use crate::opcodes::opcodes_control::*;
use crate::opcodes::opcodes_jump::*;
use crate::opcodes::opcodes_ld::*;


/// Represents an invalid opcode.
pub static OPCODE_INVALID: OpCode = OpCode {
    name: "[INVALID]",
    bytes: 1,
    cycles: 0,
    proc: |gb| {
        panic!();
    }
};


/// The table of all supported opcodes.
/// The array's index is the opcodes numerical value.
pub static OPCODE_TABLE: [OpCode; 256] = [
    /* 0x00*/ OpCode { name: "NOP",              bytes: 1, cycles:  0, proc: nop                 },
    /* 0x01*/ OpCode { name: "LD BC, ${x16}",    bytes: 3, cycles: 12, proc: ld_bc_u16           },
    /* 0x02*/ OpCode { name: "LD (BC), A",       bytes: 1, cycles:  8, proc: ld_bcptr_a          },
    /* 0x03*/ OpCode { name: "INC BC",           bytes: 1, cycles:  8, proc: inc_bc              },
    /* 0x04*/ OpCode { name: "INC B",            bytes: 1, cycles:  4, proc: inc_b               },
    /* 0x05*/ OpCode { name: "DEC B",            bytes: 1, cycles:  4, proc: dec_b               },
    /* 0x06*/ OpCode { name: "LD B, ${x8}",      bytes: 2, cycles:  8, proc: ld_b_u8             },
    /* 0x07*/ OpCode { name: "RLC A",            bytes: 1, cycles:  4, proc: rlc_a               },
    /* 0x08*/ OpCode { name: "LD (${x16}), SP",  bytes: 3, cycles: 20, proc: ld_u16ptr_sp        },
    /* 0x09*/ OpCode { name: "ADD HL, BC",       bytes: 1, cycles:  8, proc: add_hl_bc           },
    /* 0x0A*/ OpCode { name: "LD A, (BC)",       bytes: 1, cycles:  8, proc: ld_a_bcptr          },
    /* 0x0B*/ OpCode { name: "DEC BC",           bytes: 1, cycles:  8, proc: dec_bc              },
    /* 0x0C*/ OpCode { name: "INC C",            bytes: 1, cycles:  4, proc: inc_c               },
    /* 0x0D*/ OpCode { name: "DEC C",            bytes: 1, cycles:  4, proc: dec_c               },
    /* 0x0E*/ OpCode { name: "LD C, ${x8}",      bytes: 2, cycles:  8, proc: ld_c_u8             },
    /* 0x0F*/ OpCode { name: "RRC A",            bytes: 1, cycles:  4, proc: rrc_a               },

    /* 0x10*/ OpCode { name: "STOP",             bytes: 1, cycles:  4, proc: stop                },
    /* 0x11*/ OpCode { name: "LD DE, ${x16}",    bytes: 3, cycles: 12, proc: ld_de_u16           },
    /* 0x12*/ OpCode { name: "LD (DE), A",       bytes: 1, cycles:  8, proc: ld_deptr_a          },
    /* 0x13*/ OpCode { name: "INC DE",           bytes: 1, cycles:  8, proc: inc_de              },
    /* 0x14*/ OpCode { name: "INC D",            bytes: 1, cycles:  4, proc: inc_d               },
    /* 0x15*/ OpCode { name: "DEC D",            bytes: 1, cycles:  4, proc: dec_d               },
    /* 0x16*/ OpCode { name: "LD D, ${x8}",      bytes: 2, cycles:  8, proc: ld_d_u8             },
    /* 0x17*/ OpCode { name: "RL A",             bytes: 1, cycles:  4, proc: rl_a                },
    /* 0x18*/ OpCode { name: "JR {i8}",          bytes: 2, cycles: 12, proc: jr_i8               },
    /* 0x19*/ OpCode { name: "ADD HL, DE",       bytes: 1, cycles:  8, proc: add_hl_de           },
    /* 0x1A*/ OpCode { name: "LD A, (DE)",       bytes: 1, cycles:  8, proc: ld_a_deptr          },
    /* 0x1B*/ OpCode { name: "DEC DE",           bytes: 1, cycles:  8, proc: dec_de              },
    /* 0x1C*/ OpCode { name: "INC E",            bytes: 1, cycles:  4, proc: inc_e               },
    /* 0x1D*/ OpCode { name: "DEC E",            bytes: 1, cycles:  4, proc: dec_e               },
    /* 0x1E*/ OpCode { name: "LD E, ${x8}",      bytes: 2, cycles:  8, proc: ld_e_u8             },
    /* 0x1F*/ OpCode { name: "RR A",             bytes: 1, cycles:  4, proc: rr_a                },

    /* 0x20*/ OpCode { name: "JR NZ, {i8}",      bytes: 2, cycles:  8, proc: jr_nz_i8            },
    /* 0x21*/ OpCode { name: "LD HL, ${x16}",    bytes: 3, cycles: 12, proc: ld_hl_u16           },
    /* 0x22*/ OpCode { name: "LD (HL+), A",      bytes: 1, cycles:  8, proc: ld_hlptri_a         },
    /* 0x23*/ OpCode { name: "INC HL",           bytes: 1, cycles:  8, proc: inc_hl              },
    /* 0x24*/ OpCode { name: "INC H",            bytes: 1, cycles:  4, proc: inc_h               },
    /* 0x25*/ OpCode { name: "DEC H",            bytes: 1, cycles:  4, proc: dec_h               },
    /* 0x26*/ OpCode { name: "LD H, ${x8}",      bytes: 2, cycles:  8, proc: ld_h_u8             },
    /* 0x27*/ OpCode { name: "DAA",              bytes: 1, cycles:  4, proc: daa                 },
    /* 0x28*/ OpCode { name: "JR Z, {i8}",       bytes: 2, cycles:  8, proc: jr_z_i8             },
    /* 0x29*/ OpCode { name: "ADD HL, HL",       bytes: 1, cycles:  8, proc: add_hl_hl           },
    /* 0x2A*/ OpCode { name: "LD A, (HL+)",      bytes: 1, cycles:  8, proc: ld_a_hlptri         },
    /* 0x2B*/ OpCode { name: "DEC HL",           bytes: 1, cycles:  8, proc: dec_hl              },
    /* 0x2C*/ OpCode { name: "INC L",            bytes: 1, cycles:  4, proc: inc_l               },
    /* 0x2D*/ OpCode { name: "DEC L",            bytes: 1, cycles:  4, proc: dec_l               },
    /* 0x2E*/ OpCode { name: "LD L, ${x8}",      bytes: 2, cycles:  8, proc: ld_l_u8             },
    /* 0x2F*/ OpCode { name: "CPL",              bytes: 1, cycles:  4, proc: cpl_a               },

    /* 0x30*/ OpCode { name: "JR NC, {i8}",      bytes: 2, cycles:  8, proc: jr_nc_i8            },
    /* 0x31*/ OpCode { name: "LD SP, ${x16}",    bytes: 3, cycles: 12, proc: ld_sp_u16           },
    /* 0x32*/ OpCode { name: "LD (HL-), A",      bytes: 1, cycles:  8, proc: ld_hlptrd_a         },
    /* 0x33*/ OpCode { name: "INC SP",           bytes: 1, cycles:  8, proc: inc_sp              },
    /* 0x34*/ OpCode { name: "INC (HL)",         bytes: 1, cycles: 12, proc: inc_hlptr           },
    /* 0x35*/ OpCode { name: "DEC (HL)",         bytes: 1, cycles: 12, proc: dec_hlptr           },
    /* 0x36*/ OpCode { name: "LD (HL), ${x8}",   bytes: 2, cycles:  8, proc: ld_hlptr_u8         },
    /* 0x37*/ OpCode { name: "SCF",              bytes: 1, cycles:  4, proc: scf                 },
    /* 0x38*/ OpCode { name: "JR C, {i8}",       bytes: 2, cycles:  8, proc: jr_c_i8             },
    /* 0x39*/ OpCode { name: "ADD HL, SP",       bytes: 1, cycles:  8, proc: add_hl_sp           },
    /* 0x3A*/ OpCode { name: "LD A, (HL-)",      bytes: 1, cycles:  8, proc: ld_a_hlptrd         },
    /* 0x3B*/ OpCode { name: "DEC SP",           bytes: 1, cycles:  8, proc: dec_sp              },
    /* 0x3C*/ OpCode { name: "INC A",            bytes: 1, cycles:  4, proc: inc_a               },
    /* 0x3D*/ OpCode { name: "DEC A",            bytes: 1, cycles:  4, proc: dec_a               },
    /* 0x3E*/ OpCode { name: "LD A, ${x8}",      bytes: 2, cycles:  8, proc: ld_l_u8             },
    /* 0x3F*/ OpCode { name: "CCF",              bytes: 1, cycles:  4, proc: ccf                 },

    /* 0x40*/ OpCode { name: "LD B, B",          bytes: 1, cycles:  4, proc: ld_b_b              },
    /* 0x41*/ OpCode { name: "LD B, C",          bytes: 1, cycles:  4, proc: ld_b_c              },
    /* 0x42*/ OpCode { name: "LD B, D",          bytes: 1, cycles:  4, proc: ld_b_d              },
    /* 0x43*/ OpCode { name: "LD B, E",          bytes: 1, cycles:  4, proc: ld_b_e              },
    /* 0x44*/ OpCode { name: "LD B, H",          bytes: 1, cycles:  4, proc: ld_b_h              },
    /* 0x45*/ OpCode { name: "LD B, L",          bytes: 1, cycles:  4, proc: ld_b_l              },
    /* 0x46*/ OpCode { name: "LD B, (HL)",       bytes: 1, cycles:  8, proc: ld_b_hlptr          },
    /* 0x47*/ OpCode { name: "LD B, A",          bytes: 1, cycles:  4, proc: ld_b_a              },
    /* 0x48*/ OpCode { name: "LD C, B",          bytes: 1, cycles:  4, proc: ld_c_b              },
    /* 0x49*/ OpCode { name: "LD C, C",          bytes: 1, cycles:  4, proc: ld_c_c              },
    /* 0x4A*/ OpCode { name: "LD C, D",          bytes: 1, cycles:  4, proc: ld_c_d              },
    /* 0x4B*/ OpCode { name: "LD C, E",          bytes: 1, cycles:  4, proc: ld_c_e              },
    /* 0x4C*/ OpCode { name: "LD C, H",          bytes: 1, cycles:  4, proc: ld_c_h              },
    /* 0x4D*/ OpCode { name: "LD C, L",          bytes: 1, cycles:  4, proc: ld_c_l              },
    /* 0x4E*/ OpCode { name: "LD C, (HL)",       bytes: 1, cycles:  8, proc: ld_c_hlptr          },
    /* 0x4F*/ OpCode { name: "LD C, A",          bytes: 1, cycles:  4, proc: ld_c_a              },

    /* 0x50*/ OpCode { name: "LD D, B",          bytes: 1, cycles:  4, proc: ld_d_b              },
    /* 0x51*/ OpCode { name: "LD D, C",          bytes: 1, cycles:  4, proc: ld_d_c              },
    /* 0x52*/ OpCode { name: "LD D, D",          bytes: 1, cycles:  4, proc: ld_d_d              },
    /* 0x53*/ OpCode { name: "LD D, E",          bytes: 1, cycles:  4, proc: ld_d_e              },
    /* 0x54*/ OpCode { name: "LD D, H",          bytes: 1, cycles:  4, proc: ld_d_h              },
    /* 0x55*/ OpCode { name: "LD D, L",          bytes: 1, cycles:  4, proc: ld_d_l              },
    /* 0x56*/ OpCode { name: "LD D, (HL)",       bytes: 1, cycles:  8, proc: ld_d_hlptr          },
    /* 0x57*/ OpCode { name: "LD D, A",          bytes: 1, cycles:  4, proc: ld_d_a              },
    /* 0x58*/ OpCode { name: "LD E, B",          bytes: 1, cycles:  4, proc: ld_e_b              },
    /* 0x59*/ OpCode { name: "LD E, C",          bytes: 1, cycles:  4, proc: ld_e_c              },
    /* 0x5A*/ OpCode { name: "LD E, D",          bytes: 1, cycles:  4, proc: ld_e_d              },
    /* 0x5B*/ OpCode { name: "LD E, E",          bytes: 1, cycles:  4, proc: ld_e_e              },
    /* 0x5C*/ OpCode { name: "LD E, H",          bytes: 1, cycles:  4, proc: ld_e_h              },
    /* 0x5D*/ OpCode { name: "LD E, L",          bytes: 1, cycles:  4, proc: ld_e_l              },
    /* 0x5E*/ OpCode { name: "LD E, (HL)",       bytes: 1, cycles:  8, proc: ld_e_hlptr          },
    /* 0x5F*/ OpCode { name: "LD E, A",          bytes: 1, cycles:  4, proc: ld_e_a              },

    /* 0x60*/ OpCode { name: "LD H, B",          bytes: 1, cycles:  4, proc: ld_h_b              },
    /* 0x61*/ OpCode { name: "LD H, C",          bytes: 1, cycles:  4, proc: ld_h_c              },
    /* 0x62*/ OpCode { name: "LD H, D",          bytes: 1, cycles:  4, proc: ld_h_d              },
    /* 0x63*/ OpCode { name: "LD H, E",          bytes: 1, cycles:  4, proc: ld_h_e              },
    /* 0x64*/ OpCode { name: "LD H, H",          bytes: 1, cycles:  4, proc: ld_h_h              },
    /* 0x65*/ OpCode { name: "LD H, L",          bytes: 1, cycles:  4, proc: ld_h_l              },
    /* 0x66*/ OpCode { name: "LD H, (HL)",       bytes: 1, cycles:  8, proc: ld_h_hlptr          },
    /* 0x67*/ OpCode { name: "LD H, A",          bytes: 1, cycles:  4, proc: ld_h_a              },
    /* 0x68*/ OpCode { name: "LD L, B",          bytes: 1, cycles:  4, proc: ld_l_b              },
    /* 0x69*/ OpCode { name: "LD L, C",          bytes: 1, cycles:  4, proc: ld_l_c              },
    /* 0x6A*/ OpCode { name: "LD L, D",          bytes: 1, cycles:  4, proc: ld_l_d              },
    /* 0x6B*/ OpCode { name: "LD L, E",          bytes: 1, cycles:  4, proc: ld_l_e              },
    /* 0x6C*/ OpCode { name: "LD L, H",          bytes: 1, cycles:  4, proc: ld_l_h              },
    /* 0x6D*/ OpCode { name: "LD L, L",          bytes: 1, cycles:  4, proc: ld_l_l              },
    /* 0x6E*/ OpCode { name: "LD L, (HL)",       bytes: 1, cycles:  8, proc: ld_l_hlptr          },
    /* 0x6F*/ OpCode { name: "LD L, A",          bytes: 1, cycles:  4, proc: ld_l_a              },

    /* 0x70*/ OpCode { name: "LD (HL), B",       bytes: 1, cycles:  8, proc: ld_hlptr_b          },
    /* 0x71*/ OpCode { name: "LD (HL), C",       bytes: 1, cycles:  8, proc: ld_hlptr_c          },
    /* 0x72*/ OpCode { name: "LD (HL), D",       bytes: 1, cycles:  8, proc: ld_hlptr_d          },
    /* 0x73*/ OpCode { name: "LD (HL), E",       bytes: 1, cycles:  8, proc: ld_hlptr_e          },
    /* 0x74*/ OpCode { name: "LD (HL), H",       bytes: 1, cycles:  8, proc: ld_hlptr_h          },
    /* 0x75*/ OpCode { name: "LD (HL), L",       bytes: 1, cycles:  8, proc: ld_hlptr_l          },
    /* 0x76*/ OpCode { name: "HALT",             bytes: 1, cycles:  4, proc: halt                },
    /* 0x77*/ OpCode { name: "LD (HL), A",       bytes: 1, cycles:  8, proc: ld_hlptr_a          },
    /* 0x78*/ OpCode { name: "LD A, B",          bytes: 1, cycles:  4, proc: ld_a_b              },
    /* 0x79*/ OpCode { name: "LD A, C",          bytes: 1, cycles:  4, proc: ld_a_c              },
    /* 0x7A*/ OpCode { name: "LD A, D",          bytes: 1, cycles:  4, proc: ld_a_d              },
    /* 0x7B*/ OpCode { name: "LD A, E",          bytes: 1, cycles:  4, proc: ld_a_e              },
    /* 0x7C*/ OpCode { name: "LD A, H",          bytes: 1, cycles:  4, proc: ld_a_h              },
    /* 0x7D*/ OpCode { name: "LD A, L",          bytes: 1, cycles:  4, proc: ld_a_l              },
    /* 0x7E*/ OpCode { name: "LD A, (HL)",       bytes: 1, cycles:  8, proc: ld_a_hlptr          },
    /* 0x7F*/ OpCode { name: "LD A, A",          bytes: 1, cycles:  4, proc: ld_a_a              },

    /* 0x80*/ OpCode { name: "ADD A, B",         bytes: 1, cycles:  4, proc: add_a_b             },
    /* 0x81*/ OpCode { name: "ADD A, C",         bytes: 1, cycles:  4, proc: add_a_c             },
    /* 0x82*/ OpCode { name: "ADD A, D",         bytes: 1, cycles:  4, proc: add_a_d             },
    /* 0x83*/ OpCode { name: "ADD A, E",         bytes: 1, cycles:  4, proc: add_a_e             },
    /* 0x84*/ OpCode { name: "ADD A, H",         bytes: 1, cycles:  4, proc: add_a_h             },
    /* 0x85*/ OpCode { name: "ADD A, L",         bytes: 1, cycles:  4, proc: add_a_l             },
    /* 0x86*/ OpCode { name: "ADD A, (HL)",      bytes: 1, cycles:  8, proc: add_a_hlptr         },
    /* 0x87*/ OpCode { name: "ADD A, A",         bytes: 1, cycles:  4, proc: add_a_a             },
    /* 0x88*/ OpCode { name: "ADC A, B",         bytes: 1, cycles:  4, proc: adc_a_b             },
    /* 0x89*/ OpCode { name: "ADC A, C",         bytes: 1, cycles:  4, proc: adc_a_c             },
    /* 0x8A*/ OpCode { name: "ADC A, D",         bytes: 1, cycles:  4, proc: adc_a_d             },
    /* 0x8B*/ OpCode { name: "ADC A, E",         bytes: 1, cycles:  4, proc: adc_a_e             },
    /* 0x8C*/ OpCode { name: "ADC A, H",         bytes: 1, cycles:  4, proc: adc_a_h             },
    /* 0x8D*/ OpCode { name: "ADC A, L",         bytes: 1, cycles:  4, proc: adc_a_l             },
    /* 0x8E*/ OpCode { name: "ADC A, (HL)",      bytes: 1, cycles:  8, proc: adc_a_hlptr         },
    /* 0x8F*/ OpCode { name: "ADC A, A",         bytes: 1, cycles:  4, proc: adc_a_a             },

    /* 0x90*/ OpCode { name: "SUB A, B",         bytes: 1, cycles:  4, proc: sub_a_b             },
    /* 0x91*/ OpCode { name: "SUB A, C",         bytes: 1, cycles:  4, proc: sub_a_c             },
    /* 0x92*/ OpCode { name: "SUB A, D",         bytes: 1, cycles:  4, proc: sub_a_d             },
    /* 0x93*/ OpCode { name: "SUB A, E",         bytes: 1, cycles:  4, proc: sub_a_e             },
    /* 0x94*/ OpCode { name: "SUB A, H",         bytes: 1, cycles:  4, proc: sub_a_h             },
    /* 0x95*/ OpCode { name: "SUB A, L",         bytes: 1, cycles:  4, proc: sub_a_l             },
    /* 0x96*/ OpCode { name: "SUB A, (HL)",      bytes: 1, cycles:  8, proc: sub_a_hlptr         },
    /* 0x97*/ OpCode { name: "SUB A, A",         bytes: 1, cycles:  4, proc: sub_a_a             },
    /* 0x98*/ OpCode { name: "SBC A, B",         bytes: 1, cycles:  4, proc: sbc_a_b             },
    /* 0x99*/ OpCode { name: "SBC A, C",         bytes: 1, cycles:  4, proc: sbc_a_c             },
    /* 0x9A*/ OpCode { name: "SBC A, D",         bytes: 1, cycles:  4, proc: sbc_a_d             },
    /* 0x9B*/ OpCode { name: "SBC A, E",         bytes: 1, cycles:  4, proc: sbc_a_e             },
    /* 0x9C*/ OpCode { name: "SBC A, H",         bytes: 1, cycles:  4, proc: sbc_a_h             },
    /* 0x9D*/ OpCode { name: "SBC A, L",         bytes: 1, cycles:  4, proc: sbc_a_l             },
    /* 0x9E*/ OpCode { name: "SBC A, (HL)",      bytes: 1, cycles:  8, proc: sbc_a_hlptr         },
    /* 0x9F*/ OpCode { name: "SBC A, A",         bytes: 1, cycles:  4, proc: sbc_a_a             },

    /* 0xA0*/ OpCode { name: "AND A, B",         bytes: 1, cycles:  4, proc: and_a_b             },
    /* 0xA1*/ OpCode { name: "AND A, C",         bytes: 1, cycles:  4, proc: and_a_c             },
    /* 0xA2*/ OpCode { name: "AND A, D",         bytes: 1, cycles:  4, proc: and_a_d             },
    /* 0xA3*/ OpCode { name: "AND A, E",         bytes: 1, cycles:  4, proc: and_a_e             },
    /* 0xA4*/ OpCode { name: "AND A, H",         bytes: 1, cycles:  4, proc: and_a_h             },
    /* 0xA5*/ OpCode { name: "AND A, L",         bytes: 1, cycles:  4, proc: and_a_l             },
    /* 0xA6*/ OpCode { name: "AND A, (HL)",      bytes: 1, cycles:  8, proc: and_a_hlptr         },
    /* 0xA7*/ OpCode { name: "AND A, A",         bytes: 1, cycles:  4, proc: and_a_a             },
    /* 0xA8*/ OpCode { name: "XOR A, B",         bytes: 1, cycles:  4, proc: xor_a_b             },
    /* 0xA9*/ OpCode { name: "XOR A, C",         bytes: 1, cycles:  4, proc: xor_a_c             },
    /* 0xAA*/ OpCode { name: "XOR A, D",         bytes: 1, cycles:  4, proc: xor_a_d             },
    /* 0xAB*/ OpCode { name: "XOR A, E",         bytes: 1, cycles:  4, proc: xor_a_e             },
    /* 0xAC*/ OpCode { name: "XOR A, H",         bytes: 1, cycles:  4, proc: xor_a_h             },
    /* 0xAD*/ OpCode { name: "XOR A, L",         bytes: 1, cycles:  4, proc: xor_a_l             },
    /* 0xAE*/ OpCode { name: "XOR A, (HL)",      bytes: 1, cycles:  8, proc: xor_a_hlptr         },
    /* 0xAF*/ OpCode { name: "XOR A, A",         bytes: 1, cycles:  4, proc: xor_a_a             },

    /* 0xB0*/ OpCode { name: "OR A, B",          bytes: 1, cycles:  4, proc: or_a_b              },
    /* 0xB1*/ OpCode { name: "OR A, C",          bytes: 1, cycles:  4, proc: or_a_c              },
    /* 0xB2*/ OpCode { name: "OR A, D",          bytes: 1, cycles:  4, proc: or_a_d              },
    /* 0xB3*/ OpCode { name: "OR A, E",          bytes: 1, cycles:  4, proc: or_a_e              },
    /* 0xB4*/ OpCode { name: "OR A, H",          bytes: 1, cycles:  4, proc: or_a_h              },
    /* 0xB5*/ OpCode { name: "OR A, L",          bytes: 1, cycles:  4, proc: or_a_l              },
    /* 0xB6*/ OpCode { name: "OR A, (HL)",       bytes: 1, cycles:  8, proc: or_a_hlptr          },
    /* 0xB7*/ OpCode { name: "OR A, A",          bytes: 1, cycles:  4, proc: or_a_a              },
    /* 0xB8*/ OpCode { name: "CP A, B",          bytes: 1, cycles:  4, proc: cp_a_b              },
    /* 0xB9*/ OpCode { name: "CP A, C",          bytes: 1, cycles:  4, proc: cp_a_c              },
    /* 0xBA*/ OpCode { name: "CP A, D",          bytes: 1, cycles:  4, proc: cp_a_d              },
    /* 0xBB*/ OpCode { name: "CP A, E",          bytes: 1, cycles:  4, proc: cp_a_e              },
    /* 0xBC*/ OpCode { name: "CP A, H",          bytes: 1, cycles:  4, proc: cp_a_h              },
    /* 0xBD*/ OpCode { name: "CP A, L",          bytes: 1, cycles:  4, proc: cp_a_l              },
    /* 0xBE*/ OpCode { name: "CP A, (HL)",       bytes: 1, cycles:  8, proc: cp_a_hlptr          },
    /* 0xBF*/ OpCode { name: "CP A, A",          bytes: 1, cycles:  4, proc: cp_a_a              },

    /* 0xC0*/ OPCODE_INVALID,
    /* 0xC1*/ OPCODE_INVALID,
    /* 0xC2*/ OpCode { name: "JP NZ, 0x{x16}",   bytes: 3, cycles: 12, proc: jp_nz_u16           },
    /* 0xC3*/ OpCode { name: "JP 0x{x16}",       bytes: 3, cycles: 12, proc: jp_u16              },
    /* 0xC4*/ OPCODE_INVALID,
    /* 0xC5*/ OPCODE_INVALID,
    /* 0xC6*/ OpCode { name: "ADD A, {u8}",      bytes: 2, cycles:  8, proc: add_a_u8            },
    /* 0xC7*/ OPCODE_INVALID,
    /* 0xC8*/ OPCODE_INVALID,
    /* 0xC9*/ OPCODE_INVALID,
    /* 0xCA*/ OpCode { name: "JP Z, 0x{x16}",    bytes: 3, cycles: 12, proc: jp_z_u16            },
    /* 0xCB*/ OPCODE_INVALID,
    /* 0xCC*/ OPCODE_INVALID,
    /* 0xCD*/ OPCODE_INVALID,
    /* 0xCE*/ OpCode { name: "ADC A, {u8}",      bytes: 2, cycles:  8, proc: adc_a_u8            },
    /* 0xCF*/ OPCODE_INVALID,

    /* 0xD0*/ OPCODE_INVALID,
    /* 0xD1*/ OPCODE_INVALID,
    /* 0xD2*/ OpCode { name: "JP NC, 0x{x16}",   bytes: 3, cycles: 12, proc: jp_nc_u16           },
    /* 0xD3*/ OPCODE_INVALID,
    /* 0xD4*/ OPCODE_INVALID,
    /* 0xD5*/ OPCODE_INVALID,
    /* 0xD6*/ OpCode { name: "SUB A, {u8}",      bytes: 2, cycles:  8, proc: sub_a_u8            },
    /* 0xD7*/ OPCODE_INVALID,
    /* 0xD8*/ OPCODE_INVALID,
    /* 0xD9*/ OPCODE_INVALID,
    /* 0xDA*/ OpCode { name: "JP C, 0x{x16}",    bytes: 3, cycles: 12, proc: jp_c_u16            },
    /* 0xDB*/ OPCODE_INVALID,
    /* 0xDC*/ OPCODE_INVALID,
    /* 0xDD*/ OPCODE_INVALID,
    /* 0xDE*/ OpCode { name: "SBC A, {u8}",      bytes: 2, cycles:  8, proc: sbc_a_u8            },
    /* 0xDF*/ OPCODE_INVALID,

    /* 0xE0*/ OpCode { name: "LDH $ff{x8}, A",   bytes: 2, cycles: 12, proc: ldh_u8_a            },
    /* 0xE1*/ OPCODE_INVALID,
    /* 0xE2*/ OPCODE_INVALID,
    /* 0xE3*/ OPCODE_INVALID,
    /* 0xE4*/ OPCODE_INVALID,
    /* 0xE5*/ OPCODE_INVALID,
    /* 0xE6*/ OpCode { name: "AND A, ${x8}",     bytes: 2, cycles:  8, proc: and_a_u8            },
    /* 0xE7*/ OPCODE_INVALID,
    /* 0xE8*/ OPCODE_INVALID,
    /* 0xE9*/ OPCODE_INVALID,
    /* 0xEA*/ OpCode { name: "LD (${x16}), A",   bytes: 3, cycles: 16, proc: ld_u16ptr_a         },
    /* 0xEB*/ OPCODE_INVALID,
    /* 0xEC*/ OPCODE_INVALID,
    /* 0xED*/ OPCODE_INVALID,
    /* 0xEE*/ OpCode { name: "XOR A, ${x8}",     bytes: 2, cycles:  8, proc: xor_a_u8            },
    /* 0xEF*/ OPCODE_INVALID,

    /* 0xF0*/ OpCode { name: "LDH A, $ff{x8}",   bytes: 2, cycles: 12, proc: ldh_a_u8            },
    /* 0xF1*/ OPCODE_INVALID,
    /* 0xF2*/ OPCODE_INVALID,
    /* 0xF3*/ OpCode { name: "DI",               bytes: 1, cycles:  4, proc: disable_interrupts  },
    /* 0xF4*/ OPCODE_INVALID,
    /* 0xF5*/ OPCODE_INVALID,
    /* 0xF6*/ OpCode { name: "OR A, ${x8}",      bytes: 2, cycles:  8, proc: or_a_u8             },
    /* 0xF7*/ OPCODE_INVALID,
    /* 0xF8*/ OPCODE_INVALID,
    /* 0xF9*/ OPCODE_INVALID,
    /* 0xFA*/ OpCode { name: "LD A, (${x16})",   bytes: 3, cycles: 16, proc: ld_a_u16ptr         },
    /* 0xFB*/ OpCode { name: "EI",               bytes: 1, cycles:  4, proc: enable_interrupts   },
    /* 0xFC*/ OPCODE_INVALID,
    /* 0xFD*/ OPCODE_INVALID,
    /* 0xFE*/ OpCode { name: "CP A, ${x8}",      bytes: 2, cycles:  8, proc: cp_a_u8             },
    /* 0xFF*/ OPCODE_INVALID,
];


/// The table of all extended opcodes.
/// The array's index is the opcodes numerical value.
pub static OPCODE_TABLE_EXTENDED: [OpCode; 256] = [
    /* 0x00*/ OPCODE_INVALID,
    /* 0x01*/ OPCODE_INVALID,
    /* 0x02*/ OPCODE_INVALID,
    /* 0x03*/ OPCODE_INVALID,
    /* 0x04*/ OPCODE_INVALID,
    /* 0x05*/ OPCODE_INVALID,
    /* 0x06*/ OPCODE_INVALID,
    /* 0x07*/ OPCODE_INVALID,
    /* 0x08*/ OPCODE_INVALID,
    /* 0x09*/ OPCODE_INVALID,
    /* 0x0A*/ OPCODE_INVALID,
    /* 0x0B*/ OPCODE_INVALID,
    /* 0x0C*/ OPCODE_INVALID,
    /* 0x0D*/ OPCODE_INVALID,
    /* 0x0E*/ OPCODE_INVALID,
    /* 0x0F*/ OPCODE_INVALID,

    /* 0x10*/ OPCODE_INVALID,
    /* 0x11*/ OPCODE_INVALID,
    /* 0x12*/ OPCODE_INVALID,
    /* 0x13*/ OPCODE_INVALID,
    /* 0x14*/ OPCODE_INVALID,
    /* 0x15*/ OPCODE_INVALID,
    /* 0x16*/ OPCODE_INVALID,
    /* 0x17*/ OPCODE_INVALID,
    /* 0x18*/ OPCODE_INVALID,
    /* 0x19*/ OPCODE_INVALID,
    /* 0x1A*/ OPCODE_INVALID,
    /* 0x1B*/ OPCODE_INVALID,
    /* 0x1C*/ OPCODE_INVALID,
    /* 0x1D*/ OPCODE_INVALID,
    /* 0x1E*/ OPCODE_INVALID,
    /* 0x1F*/ OPCODE_INVALID,

    /* 0x20*/ OPCODE_INVALID,
    /* 0x21*/ OPCODE_INVALID,
    /* 0x22*/ OPCODE_INVALID,
    /* 0x23*/ OPCODE_INVALID,
    /* 0x24*/ OPCODE_INVALID,
    /* 0x25*/ OPCODE_INVALID,
    /* 0x26*/ OPCODE_INVALID,
    /* 0x27*/ OPCODE_INVALID,
    /* 0x28*/ OPCODE_INVALID,
    /* 0x29*/ OPCODE_INVALID,
    /* 0x2A*/ OPCODE_INVALID,
    /* 0x2B*/ OPCODE_INVALID,
    /* 0x2C*/ OPCODE_INVALID,
    /* 0x2D*/ OPCODE_INVALID,
    /* 0x2E*/ OPCODE_INVALID,
    /* 0x2F*/ OPCODE_INVALID,

    /* 0x30*/ OPCODE_INVALID,
    /* 0x31*/ OPCODE_INVALID,
    /* 0x32*/ OPCODE_INVALID,
    /* 0x33*/ OPCODE_INVALID,
    /* 0x34*/ OPCODE_INVALID,
    /* 0x35*/ OPCODE_INVALID,
    /* 0x36*/ OPCODE_INVALID,
    /* 0x37*/ OPCODE_INVALID,
    /* 0x38*/ OPCODE_INVALID,
    /* 0x39*/ OPCODE_INVALID,
    /* 0x3A*/ OPCODE_INVALID,
    /* 0x3B*/ OPCODE_INVALID,
    /* 0x3C*/ OPCODE_INVALID,
    /* 0x3D*/ OPCODE_INVALID,
    /* 0x3E*/ OPCODE_INVALID,
    /* 0x3F*/ OPCODE_INVALID,

    /* 0x40*/ OPCODE_INVALID,
    /* 0x41*/ OPCODE_INVALID,
    /* 0x42*/ OPCODE_INVALID,
    /* 0x43*/ OPCODE_INVALID,
    /* 0x44*/ OPCODE_INVALID,
    /* 0x45*/ OPCODE_INVALID,
    /* 0x46*/ OPCODE_INVALID,
    /* 0x47*/ OPCODE_INVALID,
    /* 0x48*/ OPCODE_INVALID,
    /* 0x49*/ OPCODE_INVALID,
    /* 0x4A*/ OPCODE_INVALID,
    /* 0x4B*/ OPCODE_INVALID,
    /* 0x4C*/ OPCODE_INVALID,
    /* 0x4D*/ OPCODE_INVALID,
    /* 0x4E*/ OPCODE_INVALID,
    /* 0x4F*/ OPCODE_INVALID,

    /* 0x50*/ OPCODE_INVALID,
    /* 0x51*/ OPCODE_INVALID,
    /* 0x52*/ OPCODE_INVALID,
    /* 0x53*/ OPCODE_INVALID,
    /* 0x54*/ OPCODE_INVALID,
    /* 0x55*/ OPCODE_INVALID,
    /* 0x56*/ OPCODE_INVALID,
    /* 0x57*/ OPCODE_INVALID,
    /* 0x58*/ OPCODE_INVALID,
    /* 0x59*/ OPCODE_INVALID,
    /* 0x5A*/ OPCODE_INVALID,
    /* 0x5B*/ OPCODE_INVALID,
    /* 0x5C*/ OPCODE_INVALID,
    /* 0x5D*/ OPCODE_INVALID,
    /* 0x5E*/ OPCODE_INVALID,
    /* 0x5F*/ OPCODE_INVALID,

    /* 0x60*/ OPCODE_INVALID,
    /* 0x61*/ OPCODE_INVALID,
    /* 0x62*/ OPCODE_INVALID,
    /* 0x63*/ OPCODE_INVALID,
    /* 0x64*/ OPCODE_INVALID,
    /* 0x65*/ OPCODE_INVALID,
    /* 0x66*/ OPCODE_INVALID,
    /* 0x67*/ OPCODE_INVALID,
    /* 0x68*/ OPCODE_INVALID,
    /* 0x69*/ OPCODE_INVALID,
    /* 0x6A*/ OPCODE_INVALID,
    /* 0x6B*/ OPCODE_INVALID,
    /* 0x6C*/ OPCODE_INVALID,
    /* 0x6D*/ OPCODE_INVALID,
    /* 0x6E*/ OPCODE_INVALID,
    /* 0x6F*/ OPCODE_INVALID,

    /* 0x70*/ OPCODE_INVALID,
    /* 0x71*/ OPCODE_INVALID,
    /* 0x72*/ OPCODE_INVALID,
    /* 0x73*/ OPCODE_INVALID,
    /* 0x74*/ OPCODE_INVALID,
    /* 0x75*/ OPCODE_INVALID,
    /* 0x76*/ OPCODE_INVALID,
    /* 0x77*/ OPCODE_INVALID,
    /* 0x78*/ OPCODE_INVALID,
    /* 0x79*/ OPCODE_INVALID,
    /* 0x7A*/ OPCODE_INVALID,
    /* 0x7B*/ OPCODE_INVALID,
    /* 0x7C*/ OPCODE_INVALID,
    /* 0x7D*/ OPCODE_INVALID,
    /* 0x7E*/ OPCODE_INVALID,
    /* 0x7F*/ OPCODE_INVALID,

    /* 0x80*/ OPCODE_INVALID,
    /* 0x81*/ OPCODE_INVALID,
    /* 0x82*/ OPCODE_INVALID,
    /* 0x83*/ OPCODE_INVALID,
    /* 0x84*/ OPCODE_INVALID,
    /* 0x85*/ OPCODE_INVALID,
    /* 0x86*/ OPCODE_INVALID,
    /* 0x87*/ OPCODE_INVALID,
    /* 0x88*/ OPCODE_INVALID,
    /* 0x89*/ OPCODE_INVALID,
    /* 0x8A*/ OPCODE_INVALID,
    /* 0x8B*/ OPCODE_INVALID,
    /* 0x8C*/ OPCODE_INVALID,
    /* 0x8D*/ OPCODE_INVALID,
    /* 0x8E*/ OPCODE_INVALID,
    /* 0x8F*/ OPCODE_INVALID,

    /* 0x90*/ OPCODE_INVALID,
    /* 0x91*/ OPCODE_INVALID,
    /* 0x92*/ OPCODE_INVALID,
    /* 0x93*/ OPCODE_INVALID,
    /* 0x94*/ OPCODE_INVALID,
    /* 0x95*/ OPCODE_INVALID,
    /* 0x96*/ OPCODE_INVALID,
    /* 0x97*/ OPCODE_INVALID,
    /* 0x98*/ OPCODE_INVALID,
    /* 0x99*/ OPCODE_INVALID,
    /* 0x9A*/ OPCODE_INVALID,
    /* 0x9B*/ OPCODE_INVALID,
    /* 0x9C*/ OPCODE_INVALID,
    /* 0x9D*/ OPCODE_INVALID,
    /* 0x9E*/ OPCODE_INVALID,
    /* 0x9F*/ OPCODE_INVALID,

    /* 0xA0*/ OPCODE_INVALID,
    /* 0xA1*/ OPCODE_INVALID,
    /* 0xA2*/ OPCODE_INVALID,
    /* 0xA3*/ OPCODE_INVALID,
    /* 0xA4*/ OPCODE_INVALID,
    /* 0xA5*/ OPCODE_INVALID,
    /* 0xA6*/ OPCODE_INVALID,
    /* 0xA7*/ OPCODE_INVALID,
    /* 0xA8*/ OPCODE_INVALID,
    /* 0xA9*/ OPCODE_INVALID,
    /* 0xAA*/ OPCODE_INVALID,
    /* 0xAB*/ OPCODE_INVALID,
    /* 0xAC*/ OPCODE_INVALID,
    /* 0xAD*/ OPCODE_INVALID,
    /* 0xAE*/ OPCODE_INVALID,
    /* 0xAF*/ OPCODE_INVALID,

    /* 0xB0*/ OPCODE_INVALID,
    /* 0xB1*/ OPCODE_INVALID,
    /* 0xB2*/ OPCODE_INVALID,
    /* 0xB3*/ OPCODE_INVALID,
    /* 0xB4*/ OPCODE_INVALID,
    /* 0xB5*/ OPCODE_INVALID,
    /* 0xB6*/ OPCODE_INVALID,
    /* 0xB7*/ OPCODE_INVALID,
    /* 0xB8*/ OPCODE_INVALID,
    /* 0xB9*/ OPCODE_INVALID,
    /* 0xBA*/ OPCODE_INVALID,
    /* 0xBB*/ OPCODE_INVALID,
    /* 0xBC*/ OPCODE_INVALID,
    /* 0xBD*/ OPCODE_INVALID,
    /* 0xBE*/ OPCODE_INVALID,
    /* 0xBF*/ OPCODE_INVALID,

    /* 0xC0*/ OPCODE_INVALID,
    /* 0xC1*/ OPCODE_INVALID,
    /* 0xC2*/ OPCODE_INVALID,
    /* 0xC3*/ OPCODE_INVALID,
    /* 0xC4*/ OPCODE_INVALID,
    /* 0xC5*/ OPCODE_INVALID,
    /* 0xC6*/ OPCODE_INVALID,
    /* 0xC7*/ OPCODE_INVALID,
    /* 0xC8*/ OPCODE_INVALID,
    /* 0xC9*/ OPCODE_INVALID,
    /* 0xCA*/ OPCODE_INVALID,
    /* 0xCB*/ OPCODE_INVALID,
    /* 0xCC*/ OPCODE_INVALID,
    /* 0xCD*/ OPCODE_INVALID,
    /* 0xCE*/ OPCODE_INVALID,
    /* 0xCF*/ OPCODE_INVALID,

    /* 0xD0*/ OPCODE_INVALID,
    /* 0xD1*/ OPCODE_INVALID,
    /* 0xD2*/ OPCODE_INVALID,
    /* 0xD3*/ OPCODE_INVALID,
    /* 0xD4*/ OPCODE_INVALID,
    /* 0xD5*/ OPCODE_INVALID,
    /* 0xD6*/ OPCODE_INVALID,
    /* 0xD7*/ OPCODE_INVALID,
    /* 0xD8*/ OPCODE_INVALID,
    /* 0xD9*/ OPCODE_INVALID,
    /* 0xDA*/ OPCODE_INVALID,
    /* 0xDB*/ OPCODE_INVALID,
    /* 0xDC*/ OPCODE_INVALID,
    /* 0xDD*/ OPCODE_INVALID,
    /* 0xDE*/ OPCODE_INVALID,
    /* 0xDF*/ OPCODE_INVALID,

    /* 0xE0*/ OPCODE_INVALID,
    /* 0xE1*/ OPCODE_INVALID,
    /* 0xE2*/ OPCODE_INVALID,
    /* 0xE3*/ OPCODE_INVALID,
    /* 0xE4*/ OPCODE_INVALID,
    /* 0xE5*/ OPCODE_INVALID,
    /* 0xE6*/ OPCODE_INVALID,
    /* 0xE7*/ OPCODE_INVALID,
    /* 0xE8*/ OPCODE_INVALID,
    /* 0xE9*/ OPCODE_INVALID,
    /* 0xEA*/ OPCODE_INVALID,
    /* 0xEB*/ OPCODE_INVALID,
    /* 0xEC*/ OPCODE_INVALID,
    /* 0xED*/ OPCODE_INVALID,
    /* 0xEE*/ OPCODE_INVALID,
    /* 0xEF*/ OPCODE_INVALID,

    /* 0xF0*/ OPCODE_INVALID,
    /* 0xF1*/ OPCODE_INVALID,
    /* 0xF2*/ OPCODE_INVALID,
    /* 0xF3*/ OPCODE_INVALID,
    /* 0xF4*/ OPCODE_INVALID,
    /* 0xF5*/ OPCODE_INVALID,
    /* 0xF6*/ OPCODE_INVALID,
    /* 0xF7*/ OPCODE_INVALID,
    /* 0xF8*/ OPCODE_INVALID,
    /* 0xF9*/ OPCODE_INVALID,
    /* 0xFA*/ OPCODE_INVALID,
    /* 0xFB*/ OPCODE_INVALID,
    /* 0xFC*/ OPCODE_INVALID,
    /* 0xFD*/ OPCODE_INVALID,
    /* 0xFE*/ OPCODE_INVALID,
    /* 0xFF*/ OPCODE_INVALID,
];

