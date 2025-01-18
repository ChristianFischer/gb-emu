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



mod cgb_sound {
    use super::*;


    mod rom_singles {
        use super::*;


        #[test]
        fn rom_singles_01_registers_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/01-registers.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_01_registers_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/01-registers.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_02_len_ctr_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/02-len ctr.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_02_len_ctr_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/02-len ctr.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_03_trigger_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/03-trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_03_trigger_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/03-trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_04_sweep_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/04-sweep.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_04_sweep_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/04-sweep.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_05_sweep_details_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/05-sweep details.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_05_sweep_details_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/05-sweep details.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_06_overflow_on_trigger_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/06-overflow on trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_06_overflow_on_trigger_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/06-overflow on trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_07_len_sweep_period_sync_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/07-len sweep period sync.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_07_len_sweep_period_sync_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/07-len sweep period sync.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_08_len_ctr_during_power_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/08-len ctr during power.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_08_len_ctr_during_power_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/08-len ctr during power.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_09_wave_read_while_on_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/09-wave read while on.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_09_wave_read_while_on_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/09-wave read while on.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_10_wave_trigger_while_on_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/10-wave trigger while on.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_10_wave_trigger_while_on_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/10-wave trigger while on.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_11_regs_after_power_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/11-regs after power.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_11_regs_after_power_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/11-regs after power.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_12_wave_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/12-wave.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_12_wave_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/rom_singles/12-wave.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }


    mod cgb_sound_other {
        use super::*;


        #[test]
        fn cgb_sound_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/cgb_sound.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn cgb_sound_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cgb_sound/cgb_sound.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }
}


mod cpu_instrs {
    use super::*;


    mod individual {
        use super::*;


        #[test]
        fn individual_01_special_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/01-special.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_01_special_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/01-special.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_01_special_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/01-special.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_01_special_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/01-special.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_01_special_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/01-special.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_01_special_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/01-special.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_02_interrupts_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/02-interrupts.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_02_interrupts_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/02-interrupts.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_02_interrupts_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/02-interrupts.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_02_interrupts_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/02-interrupts.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_02_interrupts_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/02-interrupts.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_02_interrupts_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/02-interrupts.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_03_op_sp_hl_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/03-op sp,hl.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_03_op_sp_hl_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/03-op sp,hl.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_03_op_sp_hl_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/03-op sp,hl.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_03_op_sp_hl_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/03-op sp,hl.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_03_op_sp_hl_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/03-op sp,hl.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_03_op_sp_hl_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/03-op sp,hl.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_04_op_r_imm_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/04-op r,imm.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_04_op_r_imm_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/04-op r,imm.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_04_op_r_imm_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/04-op r,imm.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_04_op_r_imm_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/04-op r,imm.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_04_op_r_imm_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/04-op r,imm.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_04_op_r_imm_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/04-op r,imm.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_05_op_rp_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/05-op rp.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_05_op_rp_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/05-op rp.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_05_op_rp_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/05-op rp.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_05_op_rp_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/05-op rp.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_05_op_rp_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/05-op rp.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_05_op_rp_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/05-op rp.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_06_ld_r_r_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/06-ld r,r.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_06_ld_r_r_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/06-ld r,r.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_06_ld_r_r_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/06-ld r,r.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_06_ld_r_r_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/06-ld r,r.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_06_ld_r_r_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/06-ld r,r.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_06_ld_r_r_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/06-ld r,r.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_07_jr_jp_call_ret_rst_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_07_jr_jp_call_ret_rst_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_07_jr_jp_call_ret_rst_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_07_jr_jp_call_ret_rst_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_07_jr_jp_call_ret_rst_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_07_jr_jp_call_ret_rst_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_08_misc_instrs_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/08-misc instrs.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_08_misc_instrs_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/08-misc instrs.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_08_misc_instrs_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/08-misc instrs.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_08_misc_instrs_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/08-misc instrs.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_08_misc_instrs_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/08-misc instrs.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_08_misc_instrs_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/08-misc instrs.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_09_op_r_r_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/09-op r,r.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_09_op_r_r_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/09-op r,r.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_09_op_r_r_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/09-op r,r.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_09_op_r_r_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/09-op r,r.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_09_op_r_r_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/09-op r,r.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_09_op_r_r_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/09-op r,r.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_10_bit_ops_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/10-bit ops.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_10_bit_ops_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/10-bit ops.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_10_bit_ops_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/10-bit ops.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_10_bit_ops_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/10-bit ops.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_10_bit_ops_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/10-bit ops.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_10_bit_ops_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/10-bit ops.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_11_op_a_hl_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/11-op a,(hl).gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_11_op_a_hl_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/11-op a,(hl).gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_11_op_a_hl_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/11-op a,(hl).gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_11_op_a_hl_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/11-op a,(hl).gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_11_op_a_hl_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/11-op a,(hl).gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_11_op_a_hl_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/individual/11-op a,(hl).gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }


    mod cpu_instrs_other {
        use super::*;


        #[test]
        fn cpu_instrs_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/cpu_instrs.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn cpu_instrs_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/cpu_instrs.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn cpu_instrs_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/cpu_instrs.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn cpu_instrs_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/cpu_instrs.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn cpu_instrs_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/cpu_instrs.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn cpu_instrs_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/cpu_instrs/cpu_instrs.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }
}


mod dmg_sound {
    use super::*;


    mod rom_singles {
        use super::*;


        #[test]
        fn rom_singles_01_registers_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/01-registers.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_01_registers_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/01-registers.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_01_registers_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/01-registers.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_01_registers_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/01-registers.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_01_registers_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/01-registers.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_01_registers_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/01-registers.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_02_len_ctr_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/02-len ctr.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_02_len_ctr_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/02-len ctr.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_02_len_ctr_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/02-len ctr.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_02_len_ctr_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/02-len ctr.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_02_len_ctr_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/02-len ctr.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_02_len_ctr_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/02-len ctr.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_03_trigger_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/03-trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_03_trigger_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/03-trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_03_trigger_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/03-trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_03_trigger_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/03-trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_03_trigger_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/03-trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_03_trigger_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/03-trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_04_sweep_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/04-sweep.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_04_sweep_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/04-sweep.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_04_sweep_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/04-sweep.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_04_sweep_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/04-sweep.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_04_sweep_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/04-sweep.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_04_sweep_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/04-sweep.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_05_sweep_details_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/05-sweep details.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_05_sweep_details_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/05-sweep details.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_05_sweep_details_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/05-sweep details.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_05_sweep_details_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/05-sweep details.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_05_sweep_details_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/05-sweep details.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_05_sweep_details_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/05-sweep details.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_06_overflow_on_trigger_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/06-overflow on trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_06_overflow_on_trigger_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/06-overflow on trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_06_overflow_on_trigger_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/06-overflow on trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_06_overflow_on_trigger_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/06-overflow on trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_06_overflow_on_trigger_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/06-overflow on trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_06_overflow_on_trigger_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/06-overflow on trigger.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_07_len_sweep_period_sync_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/07-len sweep period sync.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_07_len_sweep_period_sync_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/07-len sweep period sync.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_07_len_sweep_period_sync_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/07-len sweep period sync.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_07_len_sweep_period_sync_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/07-len sweep period sync.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_07_len_sweep_period_sync_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/07-len sweep period sync.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_07_len_sweep_period_sync_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/07-len sweep period sync.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_08_len_ctr_during_power_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/08-len ctr during power.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_08_len_ctr_during_power_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/08-len ctr during power.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_08_len_ctr_during_power_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/08-len ctr during power.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_08_len_ctr_during_power_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/08-len ctr during power.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_08_len_ctr_during_power_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/08-len ctr during power.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_08_len_ctr_during_power_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/08-len ctr during power.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_09_wave_read_while_on_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/09-wave read while on.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_09_wave_read_while_on_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/09-wave read while on.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_09_wave_read_while_on_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/09-wave read while on.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_09_wave_read_while_on_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/09-wave read while on.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_09_wave_read_while_on_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/09-wave read while on.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_09_wave_read_while_on_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/09-wave read while on.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_10_wave_trigger_while_on_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/10-wave trigger while on.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_10_wave_trigger_while_on_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/10-wave trigger while on.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_10_wave_trigger_while_on_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/10-wave trigger while on.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_10_wave_trigger_while_on_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/10-wave trigger while on.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_10_wave_trigger_while_on_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/10-wave trigger while on.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_10_wave_trigger_while_on_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/10-wave trigger while on.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_11_regs_after_power_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/11-regs after power.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_11_regs_after_power_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/11-regs after power.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_11_regs_after_power_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/11-regs after power.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_11_regs_after_power_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/11-regs after power.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_11_regs_after_power_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/11-regs after power.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_11_regs_after_power_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/11-regs after power.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_12_wave_write_while_on_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/12-wave write while on.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_12_wave_write_while_on_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/12-wave write while on.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_12_wave_write_while_on_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/12-wave write while on.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_12_wave_write_while_on_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/12-wave write while on.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_12_wave_write_while_on_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/12-wave write while on.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_12_wave_write_while_on_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/rom_singles/12-wave write while on.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }


    mod dmg_sound_other {
        use super::*;


        #[test]
        fn dmg_sound_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/dmg_sound.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn dmg_sound_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/dmg_sound.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn dmg_sound_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/dmg_sound.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn dmg_sound_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/dmg_sound.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn dmg_sound_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/dmg_sound.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn dmg_sound_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/dmg_sound/dmg_sound.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }
}


mod instr_timing {
    use super::*;


    #[test]
    fn instr_timing_dmg() {
        let test_case = EmulatorTestCase {
            device: DeviceType::GameBoyDmg,
            setup: SetUpConfig {
                .. SetUpConfig::with_rom_file("blargg/instr_timing/instr_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                blargg_check_result_code: true,
                .. CheckResultConfig::default()
            },
        };

        run_test_case(test_case);
    }


    #[test]
    fn instr_timing_mgb() {
        let test_case = EmulatorTestCase {
            device: DeviceType::GameBoyPocket,
            setup: SetUpConfig {
                .. SetUpConfig::with_rom_file("blargg/instr_timing/instr_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                blargg_check_result_code: true,
                .. CheckResultConfig::default()
            },
        };

        run_test_case(test_case);
    }


    #[test]
    fn instr_timing_gbc() {
        let test_case = EmulatorTestCase {
            device: DeviceType::GameBoyColor,
            setup: SetUpConfig {
                .. SetUpConfig::with_rom_file("blargg/instr_timing/instr_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                blargg_check_result_code: true,
                .. CheckResultConfig::default()
            },
        };

        run_test_case(test_case);
    }


    #[test]
    fn instr_timing_gba() {
        let test_case = EmulatorTestCase {
            device: DeviceType::GameBoyAdvance,
            setup: SetUpConfig {
                .. SetUpConfig::with_rom_file("blargg/instr_timing/instr_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                blargg_check_result_code: true,
                .. CheckResultConfig::default()
            },
        };

        run_test_case(test_case);
    }


    #[test]
    fn instr_timing_sgb() {
        let test_case = EmulatorTestCase {
            device: DeviceType::SuperGameBoy,
            setup: SetUpConfig {
                .. SetUpConfig::with_rom_file("blargg/instr_timing/instr_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                blargg_check_result_code: true,
                .. CheckResultConfig::default()
            },
        };

        run_test_case(test_case);
    }


    #[test]
    fn instr_timing_sgb2() {
        let test_case = EmulatorTestCase {
            device: DeviceType::SuperGameBoy2,
            setup: SetUpConfig {
                .. SetUpConfig::with_rom_file("blargg/instr_timing/instr_timing.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                blargg_check_result_code: true,
                .. CheckResultConfig::default()
            },
        };

        run_test_case(test_case);
    }
}


mod interrupt_time {
    use super::*;


    #[test]
    #[ignore]
    fn interrupt_time_dmg() {
        let test_case = EmulatorTestCase {
            device: DeviceType::GameBoyDmg,
            setup: SetUpConfig {
                .. SetUpConfig::with_rom_file("blargg/interrupt_time/interrupt_time.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                blargg_check_result_code: true,
                .. CheckResultConfig::default()
            },
        };

        run_test_case(test_case);
    }


    #[test]
    #[ignore]
    fn interrupt_time_mgb() {
        let test_case = EmulatorTestCase {
            device: DeviceType::GameBoyPocket,
            setup: SetUpConfig {
                .. SetUpConfig::with_rom_file("blargg/interrupt_time/interrupt_time.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                blargg_check_result_code: true,
                .. CheckResultConfig::default()
            },
        };

        run_test_case(test_case);
    }


    #[test]
    #[ignore]
    fn interrupt_time_gbc() {
        let test_case = EmulatorTestCase {
            device: DeviceType::GameBoyColor,
            setup: SetUpConfig {
                .. SetUpConfig::with_rom_file("blargg/interrupt_time/interrupt_time.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                blargg_check_result_code: true,
                .. CheckResultConfig::default()
            },
        };

        run_test_case(test_case);
    }


    #[test]
    #[ignore]
    fn interrupt_time_gba() {
        let test_case = EmulatorTestCase {
            device: DeviceType::GameBoyAdvance,
            setup: SetUpConfig {
                .. SetUpConfig::with_rom_file("blargg/interrupt_time/interrupt_time.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                blargg_check_result_code: true,
                .. CheckResultConfig::default()
            },
        };

        run_test_case(test_case);
    }


    #[test]
    #[ignore]
    fn interrupt_time_sgb() {
        let test_case = EmulatorTestCase {
            device: DeviceType::SuperGameBoy,
            setup: SetUpConfig {
                .. SetUpConfig::with_rom_file("blargg/interrupt_time/interrupt_time.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                blargg_check_result_code: true,
                .. CheckResultConfig::default()
            },
        };

        run_test_case(test_case);
    }


    #[test]
    #[ignore]
    fn interrupt_time_sgb2() {
        let test_case = EmulatorTestCase {
            device: DeviceType::SuperGameBoy2,
            setup: SetUpConfig {
                .. SetUpConfig::with_rom_file("blargg/interrupt_time/interrupt_time.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                blargg_check_result_code: true,
                .. CheckResultConfig::default()
            },
        };

        run_test_case(test_case);
    }
}


mod mem_timing {
    use super::*;


    mod individual {
        use super::*;


        #[test]
        fn individual_01_read_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/individual/01-read_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_01_read_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/individual/01-read_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_01_read_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/individual/01-read_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_01_read_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/individual/01-read_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_01_read_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/individual/01-read_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_01_read_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/individual/01-read_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_02_write_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/individual/02-write_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_02_write_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/individual/02-write_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_02_write_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/individual/02-write_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_02_write_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/individual/02-write_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_02_write_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/individual/02-write_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_02_write_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/individual/02-write_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_03_modify_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/individual/03-modify_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_03_modify_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/individual/03-modify_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_03_modify_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/individual/03-modify_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_03_modify_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/individual/03-modify_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_03_modify_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/individual/03-modify_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn individual_03_modify_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/individual/03-modify_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }


    mod mem_timing_other {
        use super::*;


        #[test]
        fn mem_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/mem_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn mem_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/mem_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn mem_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/mem_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn mem_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/mem_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn mem_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/mem_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn mem_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing/mem_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }
}


mod mem_timing_2 {
    use super::*;


    mod rom_singles {
        use super::*;


        #[test]
        fn rom_singles_01_read_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/rom_singles/01-read_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_01_read_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/rom_singles/01-read_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_01_read_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/rom_singles/01-read_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_01_read_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/rom_singles/01-read_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_01_read_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/rom_singles/01-read_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_01_read_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/rom_singles/01-read_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_02_write_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/rom_singles/02-write_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_02_write_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/rom_singles/02-write_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_02_write_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/rom_singles/02-write_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_02_write_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/rom_singles/02-write_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_02_write_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/rom_singles/02-write_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_02_write_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/rom_singles/02-write_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_03_modify_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/rom_singles/03-modify_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_03_modify_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/rom_singles/03-modify_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_03_modify_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/rom_singles/03-modify_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_03_modify_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/rom_singles/03-modify_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_03_modify_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/rom_singles/03-modify_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn rom_singles_03_modify_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/rom_singles/03-modify_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }


    mod mem_timing_2_other {
        use super::*;


        #[test]
        fn mem_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/mem_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn mem_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/mem_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn mem_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/mem_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn mem_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/mem_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn mem_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/mem_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        fn mem_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/mem_timing-2/mem_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }
}


mod oam_bug {
    use super::*;


    mod rom_singles {
        use super::*;


        #[test]
        #[ignore]
        fn rom_singles_1_lcd_sync_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/1-lcd_sync.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_1_lcd_sync_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/1-lcd_sync.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_1_lcd_sync_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/1-lcd_sync.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_1_lcd_sync_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/1-lcd_sync.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_1_lcd_sync_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/1-lcd_sync.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_1_lcd_sync_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/1-lcd_sync.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_2_causes_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/2-causes.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_2_causes_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/2-causes.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_2_causes_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/2-causes.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_2_causes_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/2-causes.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_2_causes_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/2-causes.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_2_causes_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/2-causes.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_3_non_causes_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/3-non_causes.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_3_non_causes_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/3-non_causes.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_3_non_causes_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/3-non_causes.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_3_non_causes_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/3-non_causes.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_3_non_causes_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/3-non_causes.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_3_non_causes_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/3-non_causes.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_4_scanline_timing_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/4-scanline_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_4_scanline_timing_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/4-scanline_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_4_scanline_timing_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/4-scanline_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_4_scanline_timing_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/4-scanline_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_4_scanline_timing_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/4-scanline_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_4_scanline_timing_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/4-scanline_timing.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_5_timing_bug_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/5-timing_bug.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_5_timing_bug_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/5-timing_bug.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_5_timing_bug_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/5-timing_bug.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_5_timing_bug_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/5-timing_bug.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_5_timing_bug_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/5-timing_bug.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_5_timing_bug_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/5-timing_bug.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_6_timing_no_bug_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/6-timing_no_bug.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_6_timing_no_bug_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/6-timing_no_bug.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_6_timing_no_bug_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/6-timing_no_bug.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_6_timing_no_bug_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/6-timing_no_bug.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_6_timing_no_bug_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/6-timing_no_bug.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_6_timing_no_bug_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/6-timing_no_bug.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_7_timing_effect_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/7-timing_effect.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_7_timing_effect_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/7-timing_effect.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_7_timing_effect_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/7-timing_effect.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_7_timing_effect_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/7-timing_effect.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_7_timing_effect_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/7-timing_effect.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_7_timing_effect_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/7-timing_effect.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_8_instr_effect_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/8-instr_effect.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_8_instr_effect_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/8-instr_effect.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_8_instr_effect_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/8-instr_effect.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_8_instr_effect_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/8-instr_effect.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_8_instr_effect_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/8-instr_effect.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn rom_singles_8_instr_effect_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/rom_singles/8-instr_effect.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }


    mod oam_bug_other {
        use super::*;


        #[test]
        #[ignore]
        fn oam_bug_dmg() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyDmg,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/oam_bug.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn oam_bug_mgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyPocket,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/oam_bug.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn oam_bug_gbc() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyColor,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/oam_bug.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn oam_bug_gba() {
            let test_case = EmulatorTestCase {
                device: DeviceType::GameBoyAdvance,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/oam_bug.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn oam_bug_sgb() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/oam_bug.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }


        #[test]
        #[ignore]
        fn oam_bug_sgb2() {
            let test_case = EmulatorTestCase {
                device: DeviceType::SuperGameBoy2,
                setup: SetUpConfig {
                    .. SetUpConfig::with_rom_file("blargg/oam_bug/oam_bug.gb")
                },
                run_config: RunConfig {
                    stop_on_infinite_loop: true,
                    .. RunConfig::default()
                },
                result: CheckResultConfig {
                    blargg_check_result_code: true,
                    .. CheckResultConfig::default()
                },
            };

            run_test_case(test_case);
        }
    }
}


mod blargg_other {
    use super::*;


    #[test]
    #[ignore]
    fn halt_bug_dmg() {
        let test_case = EmulatorTestCase {
            device: DeviceType::GameBoyDmg,
            setup: SetUpConfig {
                .. SetUpConfig::with_rom_file("blargg/halt_bug.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                blargg_check_result_code: true,
                .. CheckResultConfig::default()
            },
        };

        run_test_case(test_case);
    }


    #[test]
    #[ignore]
    fn halt_bug_mgb() {
        let test_case = EmulatorTestCase {
            device: DeviceType::GameBoyPocket,
            setup: SetUpConfig {
                .. SetUpConfig::with_rom_file("blargg/halt_bug.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                blargg_check_result_code: true,
                .. CheckResultConfig::default()
            },
        };

        run_test_case(test_case);
    }


    #[test]
    #[ignore]
    fn halt_bug_gbc() {
        let test_case = EmulatorTestCase {
            device: DeviceType::GameBoyColor,
            setup: SetUpConfig {
                .. SetUpConfig::with_rom_file("blargg/halt_bug.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                blargg_check_result_code: true,
                .. CheckResultConfig::default()
            },
        };

        run_test_case(test_case);
    }


    #[test]
    #[ignore]
    fn halt_bug_gba() {
        let test_case = EmulatorTestCase {
            device: DeviceType::GameBoyAdvance,
            setup: SetUpConfig {
                .. SetUpConfig::with_rom_file("blargg/halt_bug.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                blargg_check_result_code: true,
                .. CheckResultConfig::default()
            },
        };

        run_test_case(test_case);
    }


    #[test]
    #[ignore]
    fn halt_bug_sgb() {
        let test_case = EmulatorTestCase {
            device: DeviceType::SuperGameBoy,
            setup: SetUpConfig {
                .. SetUpConfig::with_rom_file("blargg/halt_bug.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                blargg_check_result_code: true,
                .. CheckResultConfig::default()
            },
        };

        run_test_case(test_case);
    }


    #[test]
    #[ignore]
    fn halt_bug_sgb2() {
        let test_case = EmulatorTestCase {
            device: DeviceType::SuperGameBoy2,
            setup: SetUpConfig {
                .. SetUpConfig::with_rom_file("blargg/halt_bug.gb")
            },
            run_config: RunConfig {
                stop_on_infinite_loop: true,
                .. RunConfig::default()
            },
            result: CheckResultConfig {
                blargg_check_result_code: true,
                .. CheckResultConfig::default()
            },
        };

        run_test_case(test_case);
    }
}
