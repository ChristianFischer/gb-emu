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

use egui::{Grid, Label, PointerButton, Sense, Ui, Vec2, Widget};

use libgemi::core::cpu::cpu::{CpuFlag, RegisterR8};
use libgemi::core::emulator_core::EmulatorCore;
use libgemi::core::utils::to_u8;

use crate::state::EmulatorState;
use crate::ui::style::GemiStyle;
use crate::views::View;


/// A view to display runtime information about the CPU.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CpuView {
    #[serde(skip)]
    rt: CpuViewRuntimeData,
}


/// Runtime data for the [`CpuView`] object.
/// This contains the data which is used to store the state of
/// various UI elements and does not need to be serialized.
struct CpuViewRuntimeData {
    /// Stores which element is currently being edited.
    edit_mode: EditMode,

    /// Stores the current value of the edit box.
    edit_string: String,

    /// Stores the width of the edit box, which corresponds
    /// to the width of the original label.
    edit_label_width: f32,

    /// Stores whether the edit box should request focus,
    /// which is only done once after entering edit mode.
    edit_label_request_focus: bool,
}


/// Stores which element is currently being edited.
#[derive(PartialEq, Eq)]
enum EditMode {
    /// No element is being edited.
    None,

    /// One of the 8bit registers is being edited.
    Register(RegisterR8),

    /// The stack pointer is being edited.
    RegisterSp,

    /// The instruction pointer is being edited.
    RegisterPc,
}


impl CpuView {
    /// Constructs a new [`CpuView`] object.
    pub fn new() -> Self {
        Self {
            rt: CpuViewRuntimeData::default(),
        }
    }
}


impl View for CpuView {
    fn title(&self, _state: &mut EmulatorState) -> &str {
        "CPU"
    }

    fn ui(&mut self, state: &mut EmulatorState, ui: &mut Ui) {
        self.display_registers(ui, state);
        self.display_cpu_flags(ui, state);
    }
}


impl CpuView {
    /// Displays the part of the view showing the CPU registers.
    fn display_registers(&mut self, ui: &mut Ui, state: &mut EmulatorState) {
        Grid::new("cpu_registers")
            .num_columns(2)
            .spacing([20.0, 2.0])
            .show(ui, |ui| {
                let address_style = &GemiStyle::ADDRESS;

                // A+F registers
                ui.label(address_style.rich_text("AF"));
                ui.horizontal(|ui| {
                    self.display_register_u8(ui, state, RegisterR8::A);
                    self.display_register_u8(ui, state, RegisterR8::F);
                });
                ui.end_row();

                // B+C registers
                ui.label(address_style.rich_text("BC"));
                ui.horizontal(|ui| {
                    self.display_register_u8(ui, state, RegisterR8::B);
                    self.display_register_u8(ui, state, RegisterR8::C);
                });
                ui.end_row();

                // D+E registers
                ui.label(address_style.rich_text("DE"));
                ui.horizontal(|ui| {
                    self.display_register_u8(ui, state, RegisterR8::D);
                    self.display_register_u8(ui, state, RegisterR8::E);
                });
                ui.end_row();

                // H+L registers
                ui.label(address_style.rich_text("HL"));
                ui.horizontal(|ui| {
                    self.display_register_u8(ui, state, RegisterR8::H);
                    self.display_register_u8(ui, state, RegisterR8::L);
                });
                ui.end_row();

                // stack-pointer register
                ui.label(address_style.rich_text("SP"));
                self.display_register_sp(ui, state);
                ui.end_row();

                // instruction-pointer register
                ui.label(address_style.rich_text("PC"));
                self.display_register_pc(ui, state);
                ui.end_row();
            })
        ;
    }


    /// Displays the part of the view showing the CPU flags.
    fn display_cpu_flags(&mut self, ui: &mut Ui, state: &mut EmulatorState) {
        ui.separator();

        // CPU flags
        self.display_cpu_flag(ui, state, CpuFlag::Zero);
        self.display_cpu_flag(ui, state, CpuFlag::Negative);
        self.display_cpu_flag(ui, state, CpuFlag::HalfCarry);
        self.display_cpu_flag(ui, state, CpuFlag::Carry);

        ui.separator();

        // Interrupts and HALT flags (readonly)
        if let Some(emu) = state.emu.get_emulator_mut() {
            let mut is_ime  = emu.cpu.is_interrupts_enabled();
            let mut is_halt = emu.cpu.is_running() == false;
            ui.checkbox(&mut is_ime,  "Interrupts Enabled");
            ui.checkbox(&mut is_halt, "HALT");
        }
        else {
            ui.checkbox(&mut false, "Interrupts Enabled");
            ui.checkbox(&mut false, "HALT");
        }
    }


    /// Display or edit a single 8bit register.
    fn display_register_u8(&mut self, ui: &mut Ui, state: &mut EmulatorState, register: RegisterR8) {
        self.display_value(
            ui,
            state,
            EditMode::Register(register),
            |emu| {
                let value= emu.cpu.get_r8(register);
                format!("{:02X}", value)
            },
            |emu, value_str|{
                if let Ok(value) = u8::from_str_radix(value_str, 16) {
                    emu.cpu.set_r8(register, value);
                }
            }
        );
    }


    /// Display or edit the instruction pointer register.
    fn display_register_pc(&mut self, ui: &mut Ui, state: &mut EmulatorState) {
        self.display_value(
            ui,
            state,
            EditMode::RegisterPc,
            |emu| {
                let (h, l) = to_u8(emu.cpu.get_instruction_pointer());
                format!("{:02X} {:02X}", h, l)
            },
            |emu, value_str|{
                if let Ok(value) = u16::from_str_radix(value_str, 16) {
                    emu.cpu.set_instruction_pointer(value);
                }
            }
        );
    }


    /// Display or edit the stack pointer register.
    fn display_register_sp(&mut self, ui: &mut Ui, state: &mut EmulatorState) {
        self.display_value(
            ui,
            state,
            EditMode::RegisterSp,
            |emu| {
                let (h, l) = to_u8(emu.cpu.get_stack_pointer());
                format!("{:02X} {:02X}", h, l)
            },
            |emu, value_str|{
                if let Ok(value) = u16::from_str_radix(value_str, 16) {
                    emu.cpu.set_stack_pointer(value);
                }
            }
        );
    }


    /// Utility function to display any value or allows to edit on click.
    /// The value is read using the `on_read_value` callback and the
    /// written by using the `on_write_value` callback.
    fn display_value(
        &mut self,
        ui: &mut Ui, state: &mut EmulatorState,
        expected_edit_mode: EditMode,
        on_read_value: impl FnOnce(&EmulatorCore) -> String,
        on_write_value: impl FnOnce(&mut EmulatorCore, &String)
    ) {
        let is_paused = state.ui.is_paused();

        if let Some(emu) = state.emu.get_emulator_mut() {
            let style = &GemiStyle::VALUE_HIGHLIGHTED;
            let is_in_edit_mode = self.rt.edit_mode == expected_edit_mode;
            let value_str = on_read_value(emu);

            // cancel edit mode when the emulator is running
            if is_in_edit_mode && !is_paused {
                self.rt.edit_mode = EditMode::None;
            }

            // check whether we're editing or just displaying the value
            if is_in_edit_mode {
                // size of the edit box depends whether we're editing a 8 or 16 bit value
                let max_characters = match expected_edit_mode {
                    EditMode::RegisterSp | EditMode::RegisterPc => 4,
                    EditMode::Register(_) => 2,
                    _ => 0,
                };

                // display the edit box
                let response = style.text_edit_singleline(&mut self.rt.edit_string)
                    .desired_width(self.rt.edit_label_width)
                    .char_limit(max_characters)
                    .clip_text(false)
                    .margin(Vec2::new(0.0, 0.0))
                    .horizontal_align(egui::Align::Center)
                    .frame(false)
                    .font(style.style.clone())
                    .ui(ui)
                ;

                // request focus the first time after entering edit mode
                if self.rt.edit_label_request_focus {
                    self.rt.edit_label_request_focus = false;
                    response.request_focus();
                }

                // on change use the on_write callback to apply the value
                if response.changed() {
                    on_write_value(emu, &self.rt.edit_string);
                }

                // leave edit mode when out of focus
                if response.lost_focus() {
                    self.rt.edit_mode = EditMode::None;
                }
            }
            else {
                let style = &GemiStyle::VALUE_WRITABLE;

                // fetch the rich text element for the value
                let text = style.rich_text(&value_str);

                // display the value and listen for click events
                let response = Label::new(text).sense(Sense::click()).ui(ui);

                // on click, enter edit mode
                if is_paused && response.clicked_by(PointerButton::Primary) {
                    self.rt.edit_mode                = expected_edit_mode;
                    self.rt.edit_string              = value_str.replace(" ", "");
                    self.rt.edit_label_width         = response.rect.width();
                    self.rt.edit_label_request_focus = true;
                }
            }
        }
    }


    /// Display or edit a single CPU flag.
    fn display_cpu_flag(&mut self, ui: &mut Ui, state: &mut EmulatorState, flag: CpuFlag) {
        let name = match flag {
            CpuFlag::Zero      => "Zero",
            CpuFlag::Negative  => "Negative",
            CpuFlag::HalfCarry => "HalfCarry",
            CpuFlag::Carry     => "Carry",
        };

        if let Some(emu) = state.emu.get_emulator_mut() {
            let mut flag_value = emu.cpu.is_flag_set(flag);

            if ui.checkbox(&mut flag_value, name).clicked_by(PointerButton::Primary) {
                emu.cpu.set_flag(flag, flag_value);
            }
        }
        else {
            ui.checkbox(&mut false, name);
        }
    }
}


impl Default for CpuViewRuntimeData {
    fn default() -> Self {
        Self {
            edit_mode: EditMode::None,
            edit_string: String::new(),
            edit_label_width: 0.0,
            edit_label_request_focus: false,
        }
    }
}
