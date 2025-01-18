/*
 * Copyright (C) 2022-2023 by Christian Fischer
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

use libgemi::core::device_type::DeviceType;
use testrunner::run_test_case;
use tests_shared::test_config::*;

#[allow(unused_imports)]
use libgemi::core::ppu::graphic_data::{Color, DmgDisplayPalette};



mod acceptance {
    use super::*;


    mod bits {
        use super::*;


        #[test]
        fn mem_oam_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/bits/mem_oam.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn mem_oam_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/bits/mem_oam.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn mem_oam_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/bits/mem_oam.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn mem_oam_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/bits/mem_oam.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn mem_oam_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/bits/mem_oam.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn mem_oam_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/bits/mem_oam.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn reg_f_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/bits/reg_f.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn reg_f_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/bits/reg_f.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn reg_f_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/bits/reg_f.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn reg_f_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/bits/reg_f.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn reg_f_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/bits/reg_f.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn reg_f_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/bits/reg_f.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn unused_hwio_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/bits/unused_hwio-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn unused_hwio_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/bits/unused_hwio-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn unused_hwio_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/bits/unused_hwio-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn unused_hwio_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/bits/unused_hwio-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }


    mod instr {
        use super::*;


        #[test]
        fn daa_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/instr/daa.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn daa_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/instr/daa.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn daa_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/instr/daa.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn daa_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/instr/daa.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn daa_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/instr/daa.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn daa_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/instr/daa.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }


    mod interrupts {
        use super::*;


        #[test]
        #[ignore]
        fn ie_push_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/interrupts/ie_push.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ie_push_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/interrupts/ie_push.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ie_push_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/interrupts/ie_push.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ie_push_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/interrupts/ie_push.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ie_push_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/interrupts/ie_push.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ie_push_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/interrupts/ie_push.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }


    mod oam_dma {
        use super::*;


        #[test]
        #[ignore]
        fn basic_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma/basic.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn basic_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma/basic.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn basic_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma/basic.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn basic_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma/basic.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn basic_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma/basic.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn basic_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma/basic.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn reg_read_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma/reg_read.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn reg_read_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma/reg_read.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn reg_read_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma/reg_read.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn reg_read_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma/reg_read.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn reg_read_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma/reg_read.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn reg_read_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma/reg_read.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn sources_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma/sources-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn sources_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma/sources-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn sources_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma/sources-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn sources_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma/sources-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }


    mod ppu {
        use super::*;


        #[test]
        #[ignore]
        fn hblank_ly_scx_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/hblank_ly_scx_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn hblank_ly_scx_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/hblank_ly_scx_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn hblank_ly_scx_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/hblank_ly_scx_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn hblank_ly_scx_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/hblank_ly_scx_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_1_2_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_1_2_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_1_2_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_1_2_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_1_2_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_1_2_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_1_2_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_1_2_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_0_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_0_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_0_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_0_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_0_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_0_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_0_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_0_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_0_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_0_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_0_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_0_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_mode0_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode0_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_mode0_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode0_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_mode0_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode0_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_mode0_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode0_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_mode0_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode0_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_mode0_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode0_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_mode0_timing_sprites_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode0_timing_sprites.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_mode0_timing_sprites_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode0_timing_sprites.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_mode0_timing_sprites_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode0_timing_sprites.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_mode0_timing_sprites_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode0_timing_sprites.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_mode0_timing_sprites_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode0_timing_sprites.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_mode0_timing_sprites_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode0_timing_sprites.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_mode3_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode3_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_mode3_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode3_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_mode3_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode3_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_mode3_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode3_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_mode3_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode3_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_mode3_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_mode3_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_oam_ok_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_oam_ok_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_oam_ok_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_oam_ok_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_oam_ok_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_oam_ok_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_oam_ok_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_oam_ok_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_oam_ok_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_oam_ok_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_2_oam_ok_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/intr_2_oam_ok_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn lcdon_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/lcdon_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn lcdon_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/lcdon_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn lcdon_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/lcdon_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn lcdon_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/lcdon_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn lcdon_write_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/lcdon_write_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn lcdon_write_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/lcdon_write_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn lcdon_write_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/lcdon_write_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn lcdon_write_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/lcdon_write_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn stat_irq_blocking_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/stat_irq_blocking.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn stat_irq_blocking_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/stat_irq_blocking.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn stat_irq_blocking_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/stat_irq_blocking.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn stat_irq_blocking_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/stat_irq_blocking.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn stat_irq_blocking_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/stat_irq_blocking.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn stat_irq_blocking_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/stat_irq_blocking.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn stat_lyc_onoff_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/stat_lyc_onoff.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn stat_lyc_onoff_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/stat_lyc_onoff.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn stat_lyc_onoff_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/stat_lyc_onoff.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn stat_lyc_onoff_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/stat_lyc_onoff.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn stat_lyc_onoff_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/stat_lyc_onoff.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn stat_lyc_onoff_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/stat_lyc_onoff.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn vblank_stat_intr_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/vblank_stat_intr-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn vblank_stat_intr_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/vblank_stat_intr-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn vblank_stat_intr_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/vblank_stat_intr-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn vblank_stat_intr_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ppu/vblank_stat_intr-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }


    mod serial {
        use super::*;


        #[test]
        #[ignore]
        fn boot_sclk_align_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/serial/boot_sclk_align-dmgABCmgb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn boot_sclk_align_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/serial/boot_sclk_align-dmgABCmgb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }


    mod timer {
        use super::*;


        #[test]
        fn div_write_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/div_write.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn div_write_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/div_write.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn div_write_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/div_write.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn div_write_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/div_write.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn div_write_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/div_write.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn div_write_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/div_write.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rapid_toggle_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/rapid_toggle.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rapid_toggle_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/rapid_toggle.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rapid_toggle_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/rapid_toggle.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rapid_toggle_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/rapid_toggle.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rapid_toggle_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/rapid_toggle.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rapid_toggle_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/rapid_toggle.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim00_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim00.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim00_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim00.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim00_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim00.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim00_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim00.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim00_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim00.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim00_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim00.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim00_div_trigger_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim00_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim00_div_trigger_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim00_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim00_div_trigger_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim00_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim00_div_trigger_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim00_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim00_div_trigger_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim00_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim00_div_trigger_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim00_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim01_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim01.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim01_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim01.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim01_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim01.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim01_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim01.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim01_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim01.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim01_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim01.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim01_div_trigger_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim01_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim01_div_trigger_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim01_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim01_div_trigger_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim01_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim01_div_trigger_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim01_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim01_div_trigger_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim01_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim01_div_trigger_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim01_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim10_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim10.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim10_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim10.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim10_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim10.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim10_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim10.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim10_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim10.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim10_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim10.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim10_div_trigger_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim10_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim10_div_trigger_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim10_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim10_div_trigger_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim10_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim10_div_trigger_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim10_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim10_div_trigger_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim10_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim10_div_trigger_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim10_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim11_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim11.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim11_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim11.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim11_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim11.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim11_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim11.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim11_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim11.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim11_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim11.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim11_div_trigger_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim11_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim11_div_trigger_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim11_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim11_div_trigger_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim11_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim11_div_trigger_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim11_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim11_div_trigger_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim11_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tim11_div_trigger_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tim11_div_trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tima_reload_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tima_reload.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tima_reload_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tima_reload.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tima_reload_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tima_reload.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tima_reload_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tima_reload.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tima_reload_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tima_reload.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tima_reload_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tima_reload.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tima_write_reloading_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tima_write_reloading.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tima_write_reloading_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tima_write_reloading.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tima_write_reloading_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tima_write_reloading.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tima_write_reloading_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tima_write_reloading.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tima_write_reloading_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tima_write_reloading.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tima_write_reloading_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tima_write_reloading.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tma_write_reloading_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tma_write_reloading.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tma_write_reloading_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tma_write_reloading.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tma_write_reloading_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tma_write_reloading.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tma_write_reloading_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tma_write_reloading.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tma_write_reloading_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tma_write_reloading.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn tma_write_reloading_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/timer/tma_write_reloading.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }


    mod acceptance_other {
        use super::*;


        #[test]
        #[ignore]
        fn add_sp_e_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/add_sp_e_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn add_sp_e_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/add_sp_e_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn add_sp_e_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/add_sp_e_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn add_sp_e_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/add_sp_e_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn add_sp_e_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/add_sp_e_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn add_sp_e_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/add_sp_e_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn boot_div_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_div-S.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn boot_div_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_div-S.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn boot_div_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_div-dmgABCmgb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn boot_div_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_div-dmgABCmgb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn boot_div2_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_div2-S.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn boot_div2_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_div2-S.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn boot_hwio_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_hwio-S.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn boot_hwio_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_hwio-S.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn boot_hwio_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_hwio-dmgABCmgb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn boot_hwio_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_hwio-dmgABCmgb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn boot_regs_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_regs-dmgABC.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn boot_regs_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_regs-mgb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn boot_regs_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_regs-sgb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn boot_regs_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/boot_regs-sgb2.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_cc_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_cc_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_cc_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_cc_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_cc_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_cc_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_cc_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_cc_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_cc_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_cc_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_cc_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_cc_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_cc_timing2_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_cc_timing2.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_cc_timing2_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_cc_timing2.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_cc_timing2_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_cc_timing2.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_cc_timing2_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_cc_timing2.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_cc_timing2_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_cc_timing2.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_cc_timing2_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_cc_timing2.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_timing2_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_timing2.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_timing2_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_timing2.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_timing2_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_timing2.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_timing2_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_timing2.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_timing2_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_timing2.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn call_timing2_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/call_timing2.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn di_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/di_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn di_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/di_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn di_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/di_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn di_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/di_timing-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn div_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/div_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn div_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/div_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn div_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/div_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn div_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/div_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn div_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/div_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn div_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/div_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ei_sequence_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ei_sequence.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ei_sequence_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ei_sequence.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ei_sequence_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ei_sequence.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ei_sequence_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ei_sequence.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ei_sequence_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ei_sequence.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ei_sequence_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ei_sequence.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ei_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ei_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ei_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ei_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ei_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ei_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ei_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ei_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ei_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ei_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ei_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ei_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn halt_ime0_ei_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime0_ei.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn halt_ime0_ei_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime0_ei.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn halt_ime0_ei_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime0_ei.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn halt_ime0_ei_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime0_ei.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn halt_ime0_ei_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime0_ei.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn halt_ime0_ei_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime0_ei.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn halt_ime0_nointr_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime0_nointr_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn halt_ime0_nointr_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime0_nointr_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn halt_ime0_nointr_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime0_nointr_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn halt_ime0_nointr_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime0_nointr_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn halt_ime0_nointr_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime0_nointr_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn halt_ime0_nointr_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime0_nointr_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn halt_ime1_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime1_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn halt_ime1_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime1_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn halt_ime1_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime1_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn halt_ime1_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime1_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn halt_ime1_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime1_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn halt_ime1_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime1_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn halt_ime1_timing2_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime1_timing2-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn halt_ime1_timing2_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime1_timing2-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn halt_ime1_timing2_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime1_timing2-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn halt_ime1_timing2_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/halt_ime1_timing2-GS.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn if_ie_registers_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/if_ie_registers.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn if_ie_registers_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/if_ie_registers.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn if_ie_registers_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/if_ie_registers.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn if_ie_registers_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/if_ie_registers.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn if_ie_registers_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/if_ie_registers.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn if_ie_registers_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/if_ie_registers.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/intr_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/intr_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/intr_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/intr_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/intr_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn intr_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/intr_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn jp_cc_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/jp_cc_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn jp_cc_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/jp_cc_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn jp_cc_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/jp_cc_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn jp_cc_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/jp_cc_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn jp_cc_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/jp_cc_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn jp_cc_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/jp_cc_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn jp_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/jp_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn jp_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/jp_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn jp_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/jp_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn jp_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/jp_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn jp_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/jp_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn jp_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/jp_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ld_hl_sp_e_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ld_hl_sp_e_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ld_hl_sp_e_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ld_hl_sp_e_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ld_hl_sp_e_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ld_hl_sp_e_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ld_hl_sp_e_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ld_hl_sp_e_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ld_hl_sp_e_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ld_hl_sp_e_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ld_hl_sp_e_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ld_hl_sp_e_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn oam_dma_restart_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_restart.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn oam_dma_restart_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_restart.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn oam_dma_restart_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_restart.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn oam_dma_restart_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_restart.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn oam_dma_restart_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_restart.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn oam_dma_restart_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_restart.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn oam_dma_start_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_start.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn oam_dma_start_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_start.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn oam_dma_start_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_start.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn oam_dma_start_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_start.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn oam_dma_start_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_start.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn oam_dma_start_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_start.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn oam_dma_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn oam_dma_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn oam_dma_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn oam_dma_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn oam_dma_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn oam_dma_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/oam_dma_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn pop_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/pop_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn pop_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/pop_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn pop_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/pop_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn pop_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/pop_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn pop_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/pop_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn pop_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/pop_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn push_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/push_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn push_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/push_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn push_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/push_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn push_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/push_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn push_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/push_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn push_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/push_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rapid_di_ei_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/rapid_di_ei.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rapid_di_ei_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/rapid_di_ei.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rapid_di_ei_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/rapid_di_ei.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rapid_di_ei_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/rapid_di_ei.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rapid_di_ei_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/rapid_di_ei.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rapid_di_ei_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/rapid_di_ei.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ret_cc_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ret_cc_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ret_cc_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ret_cc_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ret_cc_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ret_cc_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ret_cc_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ret_cc_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ret_cc_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ret_cc_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ret_cc_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ret_cc_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ret_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ret_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ret_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ret_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ret_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ret_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ret_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ret_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ret_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ret_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn ret_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/ret_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn reti_intr_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/reti_intr_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn reti_intr_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/reti_intr_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn reti_intr_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/reti_intr_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn reti_intr_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/reti_intr_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn reti_intr_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/reti_intr_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn reti_intr_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/reti_intr_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn reti_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/reti_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn reti_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/reti_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn reti_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/reti_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn reti_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/reti_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn reti_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/reti_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn reti_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/reti_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rst_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/rst_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rst_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/rst_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rst_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/rst_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rst_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/rst_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rst_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/rst_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rst_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/acceptance/rst_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }
}


mod emulator_only {
    use super::*;


    mod mbc1 {
        use super::*;


        #[test]
        fn bits_bank1_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_bank1.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_bank1_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_bank1.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_bank1_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_bank1.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_bank1_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_bank1.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_bank1_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_bank1.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_bank1_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_bank1.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_bank2_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_bank2.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_bank2_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_bank2.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_bank2_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_bank2.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_bank2_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_bank2.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_bank2_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_bank2.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_bank2_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_bank2.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_mode_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_mode.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_mode_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_mode.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_mode_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_mode.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_mode_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_mode.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_mode_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_mode.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_mode_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_mode.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_ramg_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_ramg.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_ramg_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_ramg.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_ramg_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_ramg.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_ramg_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_ramg.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_ramg_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_ramg.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_ramg_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/bits_ramg.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn multicart_rom_8mb_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/multicart_rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn multicart_rom_8mb_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/multicart_rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn multicart_rom_8mb_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/multicart_rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn multicart_rom_8mb_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/multicart_rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn multicart_rom_8mb_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/multicart_rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn multicart_rom_8mb_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/multicart_rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn ram_256kb_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/ram_256kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn ram_256kb_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/ram_256kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn ram_256kb_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/ram_256kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn ram_256kb_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/ram_256kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn ram_256kb_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/ram_256kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn ram_256kb_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/ram_256kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn ram_64kb_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/ram_64kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn ram_64kb_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/ram_64kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn ram_64kb_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/ram_64kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn ram_64kb_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/ram_64kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn ram_64kb_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/ram_64kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn ram_64kb_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/ram_64kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_16mb_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_16Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_16mb_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_16Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_16mb_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_16Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_16mb_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_16Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_16mb_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_16Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_16mb_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_16Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_1mb_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_1mb_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_1mb_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_1mb_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_1mb_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_1mb_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_2mb_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_2mb_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_2mb_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_2mb_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_2mb_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_2mb_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_4mb_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_4Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_4mb_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_4Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_4mb_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_4Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_4mb_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_4Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_4mb_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_4Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_4mb_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_4Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_512kb_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_512kb_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_512kb_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_512kb_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_512kb_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_512kb_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_8mb_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_8mb_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_8mb_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_8mb_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_8mb_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_8mb_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc1/rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }


    mod mbc2 {
        use super::*;


        #[test]
        fn bits_ramg_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_ramg.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_ramg_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_ramg.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_ramg_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_ramg.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_ramg_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_ramg.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_ramg_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_ramg.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_ramg_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_ramg.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_romb_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_romb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_romb_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_romb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_romb_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_romb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_romb_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_romb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_romb_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_romb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_romb_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_romb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_unused_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_unused.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_unused_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_unused.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_unused_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_unused.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_unused_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_unused.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_unused_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_unused.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn bits_unused_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/bits_unused.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn ram_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/ram.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn ram_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/ram.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn ram_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/ram.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn ram_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/ram.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn ram_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/ram.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn ram_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/ram.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_1mb_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_1mb_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_1mb_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_1mb_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_1mb_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_1mb_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_2mb_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_2mb_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_2mb_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_2mb_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_2mb_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_2mb_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_512kb_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_512kb_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_512kb_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_512kb_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_512kb_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_512kb_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc2/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }


    mod mbc5 {
        use super::*;


        #[test]
        fn rom_16mb_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_16Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_16mb_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_16Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_16mb_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_16Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_16mb_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_16Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_16mb_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_16Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_16mb_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_16Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_1mb_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_1mb_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_1mb_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_1mb_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_1mb_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_1mb_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_1Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_2mb_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_2mb_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_2mb_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_2mb_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_2mb_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_2mb_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_2Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_32mb_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_32Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_32mb_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_32Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_32mb_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_32Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_32mb_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_32Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_32mb_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_32Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_32mb_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_32Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_4mb_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_4Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_4mb_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_4Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_4mb_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_4Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_4mb_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_4Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_4mb_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_4Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_4mb_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_4Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_512kb_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_512kb_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_512kb_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_512kb_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_512kb_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_512kb_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_512kb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_64mb_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_64Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_64mb_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_64Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_64mb_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_64Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_64mb_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_64Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_64mb_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_64Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_64mb_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_64Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_8mb_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_8mb_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_8mb_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_8mb_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_8mb_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_8mb_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/emulator-only/mbc5/rom_8Mb.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }
}


mod misc {
    use super::*;


    mod bits {
        use super::*;


        #[test]
        #[ignore]
        fn unused_hwio_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/misc/bits/unused_hwio-C.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn unused_hwio_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/misc/bits/unused_hwio-C.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }


    mod ppu {
        use super::*;


        #[test]
        #[ignore]
        fn vblank_stat_intr_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/misc/ppu/vblank_stat_intr-C.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn vblank_stat_intr_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/misc/ppu/vblank_stat_intr-C.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }


    mod misc_other {
        use super::*;


        #[test]
        #[ignore]
        fn boot_div_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/misc/boot_div-A.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn boot_hwio_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/misc/boot_hwio-C.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn boot_hwio_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/misc/boot_hwio-C.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn boot_regs_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("mooneye-test-suite/misc/boot_regs-A.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    mooneye_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }
}
