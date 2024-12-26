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

use gemi_core::mmu::locations::*;


const APU_REGISTER_READABLE_BITS : [u8; 48] = [
    /* CH1 */   0b_0111_1111, 0b_1100_0000, 0b_1111_1111, 0b_0000_0000, 0b_0100_0000,
    /* CH2 */   0b_0000_0000, 0b_1100_0000, 0b_1111_1111, 0b_0000_0000, 0b_0100_0000,
    /* CH3 */   0b_1000_0000, 0b_0000_0000, 0b_0110_0000, 0b_0000_0000, 0b_0100_0000,
    /* CH4 */   0b_0000_0000, 0b_0000_0000, 0b_1111_1111, 0b_1111_1111, 0b_0100_0000,
    /* NR5x */  0b_1111_1111, 0b_1111_1111, 0b_1000_1111,

    /* off */   0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,

    /* Wave */  0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];


/// Get the bitmask for readable bits for a specific APU register.
fn get_readable_bits_for(address: u16) -> u8 {
    APU_REGISTER_READABLE_BITS[(address - 0xff10) as usize]
}


/// Checks a given register by writing 0xff and 0x00 into it and compares the input value with
/// the value returned from the register. Checks if unused bits are always set to '1'.
fn test_apu_register(name: &str, address: u16) {
    let readable_bits     = get_readable_bits_for(address);
    let non_readable_bits = !readable_bits;

    let mut gb = gemi_core::gameboy::Builder::new()
        .finish()
        .unwrap()
    ;

    gb.cpu.get_mmu_mut().write_u8(address, 0xff);
    let result1 = gb.cpu.get_mmu().read_u8(address);

    // all bits are expected to be '1'
    assert_eq!(0xff, result1, "register {name} expected value: '{:08b}' got '{:08b}'", 0xff, result1);

    gb.cpu.get_mmu_mut().write_u8(address, 0x00);
    let result2 = gb.cpu.get_mmu().read_u8(address);

    // only non-readable bits are expected to be '1'
    assert_eq!(non_readable_bits, result2, "register {name} expected value: '{:08b}' got '{:08b}'", non_readable_bits, result2);

    // all readable bits are expected to be '0'
    assert_eq!(0x00, result2 & readable_bits);
}


macro_rules! test_apu_register {
    ($name:ident : address=$address:expr) => {
        #[test]
        fn $name() {
            test_apu_register(stringify!($name), $address);

        }
    };
}


mod channel1 {
    use super::*;

    test_apu_register!(nr10: address=MEMORY_LOCATION_APU_NR10);
    test_apu_register!(nr11: address=MEMORY_LOCATION_APU_NR11);
    test_apu_register!(nr12: address=MEMORY_LOCATION_APU_NR12);
    test_apu_register!(nr13: address=MEMORY_LOCATION_APU_NR13);
    test_apu_register!(nr14: address=MEMORY_LOCATION_APU_NR14);
}

mod channel2 {
    use super::*;

    test_apu_register!(nr20: address=MEMORY_LOCATION_APU_NR20);
    test_apu_register!(nr21: address=MEMORY_LOCATION_APU_NR21);
    test_apu_register!(nr22: address=MEMORY_LOCATION_APU_NR22);
    test_apu_register!(nr23: address=MEMORY_LOCATION_APU_NR23);
    test_apu_register!(nr24: address=MEMORY_LOCATION_APU_NR24);
}

mod channel3 {
    use super::*;

    test_apu_register!(nr30: address=MEMORY_LOCATION_APU_NR30);
    test_apu_register!(nr31: address=MEMORY_LOCATION_APU_NR31);
    test_apu_register!(nr32: address=MEMORY_LOCATION_APU_NR32);
    test_apu_register!(nr33: address=MEMORY_LOCATION_APU_NR33);
    test_apu_register!(nr34: address=MEMORY_LOCATION_APU_NR34);
}

mod channel4 {
    use super::*;

    test_apu_register!(nr40: address=MEMORY_LOCATION_APU_NR40);
    test_apu_register!(nr41: address=MEMORY_LOCATION_APU_NR41);
    test_apu_register!(nr42: address=MEMORY_LOCATION_APU_NR42);
    test_apu_register!(nr43: address=MEMORY_LOCATION_APU_NR43);
    test_apu_register!(nr44: address=MEMORY_LOCATION_APU_NR44);
}

mod apu_control {
    use super::*;

    test_apu_register!(nr50: address=MEMORY_LOCATION_APU_NR50);
    test_apu_register!(nr51: address=MEMORY_LOCATION_APU_NR51);
}


#[test]
fn test_registers_after_reset() {
    let mut gb = gemi_core::gameboy::Builder::new()
        .finish()
        .unwrap()
    ;

    // turn apu on
    gb.get_mmu_mut().write_u8(MEMORY_LOCATION_APU_NR52, 0x80);

    // set all registers to 0xff
    for register in MEMORY_LOCATION_APU_NR10 ..= MEMORY_LOCATION_APU_NR51 {
        gb.get_mmu_mut().write_u8(register, 0xff);
    }

    // check all registers if they return 0xff
    for register in MEMORY_LOCATION_APU_NR10 ..= MEMORY_LOCATION_APU_NR51 {
        let value = gb.get_mmu_mut().read_u8(register);
        assert_eq!(0xff, value);
    }

    // turn apu off and on (reset)
    gb.get_mmu_mut().write_u8(MEMORY_LOCATION_APU_NR52, 0x00);
    gb.get_mmu_mut().write_u8(MEMORY_LOCATION_APU_NR52, 0x80);

    // check all registers after reset
    for register in MEMORY_LOCATION_APU_NR10 ..= MEMORY_LOCATION_APU_NR51 {
        let value             = gb.get_mmu_mut().read_u8(register);
        let readable_bits     = get_readable_bits_for(register);
        let non_readable_bits = !readable_bits;

        assert_eq!(
            non_readable_bits, value,
            "Register {:04x} should have only non readable bits set (expected: '{:08b}', got '{:08b}",
            register, non_readable_bits, value
        );
    }
}
