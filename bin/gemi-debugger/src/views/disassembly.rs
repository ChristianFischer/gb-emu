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

use std::ops::Range;

use egui::{Grid, RichText, ScrollArea, Sense, TextStyle, Ui, vec2};

use gemi_core::cpu::opcode::{Instruction, Token};
use gemi_core::gameboy::GameBoy;

use crate::state::EmulatorState;
use crate::ui::style::GemiStyle;
use crate::views::View;

const ADDITIONAL_LINES_BEYOND_VIEW : usize = 10;


#[derive(serde::Serialize, serde::Deserialize)]
pub struct DisassemblyView {
    #[serde(skip)]
    rt: RuntimeData,
}


/// Internal data of the [DisassemblyView], which does not need to be serialized.
#[derive(Default)]
struct RuntimeData {
    /// The position of the program counter in the last frame.
    last_pc: u16,

    /// The cache of the disassembled code around the current instruction pointer.
    disassembly_cache: DisassemblyCache,

    /// When set, checks whether the given line is within the current viewport.
    /// If not, this will set [RuntimeData::scroll_to_line] to bring it into focus.
    check_if_line_is_visible: Option<usize>,

    /// When set, cause the [ScrollArea] to bring the requested line into
    /// the center of its viewport.
    scroll_to_line: Option<usize>,
}


/// Contains a set of disassembled instructions in a format ready to be rendered
/// with the least effort as possible.
#[derive(Default)]
struct DisassemblyCache {
    /// The list of instructions disassembled.
    instruction_entries: Vec<InstructionDisplayEntry>,

    /// The memory range of the instructions disassembled.
    /// Usually from the address of the first instruction to the address of the
    /// first byte after the last instruction.
    /// When reached the end of the address range `0xffff`, this will be the end
    /// of the range instead.
    address_range: Range<u16>,
}


/// The data of a single instruction, already prepared to be rendered with
/// the least effort as possible.
struct InstructionDisplayEntry {
    instruction: Instruction,

    label_address: RichText,
    label_opcode_bytes: RichText,
    label_opcode_desc: Vec<RichText>,
}



impl View for DisassemblyView {
    fn title(&self, _state: &mut EmulatorState) -> &str {
        "Disassembly"
    }


    fn ui(&mut self, state: &mut EmulatorState, ui: &mut Ui) {
        if let Some(emu) = state.emu.get_emulator() {
            self.update_disassembly(ui, emu);
            self.render_disassembly_list(ui, emu);
        }
    }
}


impl DisassemblyView {
    pub fn new() -> Self {
        Self {
            rt: RuntimeData::default(),
        }
    }


    /// Updates the currently cached disassembly as needed.
    fn update_disassembly(&mut self, ui: &mut Ui, emu: &GameBoy) {
        let current_pc = emu.cpu.get_instruction_pointer();

        // when the instruction pointer did change, we want to focus the new active line.
        if self.rt.last_pc != current_pc {
            // check if the requested address is already in the cache
            if let Some(line) = self.rt.disassembly_cache.find_line_of_address(current_pc) {
                // if so, scroll to the line we found
                self.scroll_to_line(line);
            }
            else {
                // otherwise clear the cache. It will be regenerated from the current
                // address on. After the CPU did jump out of the current disassembly,
                // we cant be sure whether it's still valid or not
                self.rt.disassembly_cache = Default::default();
            }
        }

        // when no disassembly is ready yet (or was cleared before)
        // disassemble entries from the current instruction pointer onwards.
        if self.rt.disassembly_cache.is_empty() {
            let visible_lines = Self::compute_visible_lines(ui);

            self.rt.disassembly_cache = DisassemblyCache::disassemble_entries_from_pc(
                emu,
                visible_lines + ADDITIONAL_LINES_BEYOND_VIEW
            );

            // on reset the view, scroll back to top
            self.rt.scroll_to_line = Some(0);
        }

        self.rt.last_pc = current_pc;
    }


    /// Renders the actual UI using the currently stored disassembly cache.
    fn render_disassembly_list(&mut self, ui: &mut Ui, emu: &GameBoy) {
        let (line_content_height, line_height_padded) = Self::compute_line_height(ui);
        let available_rows = self.rt.disassembly_cache.get_lines_count();

        // create the widget for the scroll area
        let mut scroll_area = ScrollArea::vertical()
                .id_source("scroll_area")
                .auto_shrink([false, false])
        ;

        // did we request to scroll to a specific line?
        if let Some(line) = self.rt.scroll_to_line {
            let offset =
                    (line_height_padded * (line as f32))
                -   (ui.available_height() / 2.0)
                +   line_height_padded
            ;

            scroll_area = scroll_area.vertical_scroll_offset(
                    if offset > 0.0 { offset } else { 0.0 }
            );

            self.rt.scroll_to_line = None;
        }

        // render the grid with all disassembled instructions
        scroll_area.show_rows(
            ui,
            line_content_height,
            available_rows,
            |ui, display_rows| {
                let viewport_width = ui.available_width();

                // when getting close to the end of the list, keep adding lines, so there are
                // always at least ´ADDITIONAL_LINES_BEYOND_VIEW` lines at the end of the list
                // to allow the user to keep scrolling
                let preferred_number_of_lines = display_rows.end + ADDITIONAL_LINES_BEYOND_VIEW;
                if
                        self.rt.disassembly_cache.get_lines_count() < preferred_number_of_lines
                    &&  !self.rt.disassembly_cache.is_at_end()
                {
                    let lines_added = self.rt.disassembly_cache.fill_up(emu, preferred_number_of_lines);

                    // request a repaint after changing the number of lines available
                    if lines_added > 0 {
                        ui.ctx().request_repaint();
                    }
                }

                // check whether the requested line is already visible
                // doing so as a first step is easier, because this information
                // is already present by [ScrollArea::show_rows].
                if let Some(line) = self.rt.check_if_line_is_visible {
                    // don't count last lines, probably already partially outside the view
                    let visible_rows = display_rows.start .. display_rows.end.saturating_sub(3);

                    if !visible_rows.contains(&line) {
                        self.rt.scroll_to_line = Some(line);
                        ui.ctx().request_repaint();
                    }

                    self.rt.check_if_line_is_visible = None;
                }

                // render the actual disassembly lines within a grid
                Grid::new("grid")
                        .min_col_width(0.0)
                        .num_columns(4)
                        .show(ui, |ui| {
                            for row in display_rows {
                                // bounding box of the whole line
                                let line_bounds = egui::Rect::from_min_size(
                                    ui.cursor().left_top(),
                                    vec2(viewport_width, line_height_padded)
                                );

                                // mouse interaction with the current row
                                let hover_response  = ui.interact(line_bounds, ui.id().with(1), Sense::hover());
                                if hover_response.hovered() {
                                    ui.painter().rect_filled(
                                        line_bounds,
                                        2.0,
                                        ui.style().visuals.widgets.hovered.bg_fill
                                    );
                                }

                                let entry = &self.rt.disassembly_cache.instruction_entries[row];
                                entry.render_as_row(ui, emu);
                            }
                        })
                ;
            }
        );
    }


    /// Brings a specific line inside the current viewport.
    pub fn scroll_to_line(&mut self, line: usize) {
        self.rt.check_if_line_is_visible = Some(line);
    }


    /// Computes the number of lines fitting into the current viewport.
    fn compute_visible_lines(ui: &Ui) -> usize {
        let available_height = ui.available_height();
        let (_, total_line_height) = Self::compute_line_height(ui);
        let lines = available_height / total_line_height;

        lines.ceil() as usize
    }


    /// Computes the height of a single line.
    /// The two results of this function are the height of a line's content
    /// and the line height including spacing.
    fn compute_line_height(ui: &Ui) -> (f32, f32) {
        let line_height  = ui.text_style_height(&TextStyle::Monospace);
        let line_spacing = ui.spacing().item_spacing.y;

        (line_height, line_height + line_spacing)
    }
}


impl DisassemblyCache {
    /// Creates a new disassembly, starting at the current address of the instruction
    /// pointer, creating a specific number of lines.
    fn disassemble_entries_from_pc(emu: &GameBoy, max_entries: usize) -> Self {
        let current_pc = emu.cpu.get_instruction_pointer();

        // creates an empty disassembly on the address of the current instruction pointer
        let mut disassembly = Self {
            address_range: current_pc .. current_pc,
            .. Default::default()
        };

        // fill the disassembly with 'n' entries
        disassembly.fill_up(emu, max_entries);

        disassembly
    }


    /// Continues to disassemble from the last address until [max_entries] lines
    /// are stored in the disassembly cache.
    fn fill_up(&mut self, emu: &GameBoy, max_entries: usize) -> usize {
        let mut pc          = self.address_range.end;
        let mut added_lines = 0;

        // accessor to read from emulator memory
        let read_emu = |address| emu.get_mmu().read_u8(address);

        // keep adding entries until reaching the maximum number
        // or the instruction pointer reaches the end of address range
        while
                self.instruction_entries.len() < max_entries
            &&  pc < 0xffff
        {
            let instruction = Instruction::read_instruction(pc, read_emu);
            let entry       = InstructionDisplayEntry::prepare_instruction_display(instruction, emu);

            let instruction_length = entry.instruction.get_instruction_length();
            self.instruction_entries.push(entry);

            // compute the address of the next instruction
            pc = pc.saturating_add(instruction_length);

            added_lines += 1;
        }

        // update the range
        self.address_range.end = pc;

        added_lines
    }


    /// Checks whether this cache is currently empty or not.
    fn is_empty(&self) -> bool {
        self.instruction_entries.is_empty()
    }


    /// Get the number of lines in this cache.
    fn get_lines_count(&self) -> usize {
        self.instruction_entries.len()
    }


    /// Checks whether the disassembly reached the end of the address space.
    fn is_at_end(&self) -> bool {
        self.address_range.end == 0xffff
    }


    /// Checks whether the instruction on a specific address is stored in the cache or not.
    /// If so, it returns the index of the line the instruction belongs to.
    /// The address will only match, if it is the first byte of the instruction.
    fn find_line_of_address(&self, address: u16) -> Option<usize> {
        // early out if not in range at all
        if !self.address_range.contains(&address) {
            return None;
        }

        for line in 0..self.get_lines_count() {
            let instruction = &self.instruction_entries[line].instruction;

            // return the line if it matches the requested address
            if instruction.opcode_address == address {
                return Some(line);
            }

            // stop if we got beyond the requested address
            if instruction.opcode_address > address {
                break;
            }
        }

        None
    }
}


impl InstructionDisplayEntry {
    /// Creates a new [InstructionDisplayEntry] for a given [Instruction].
    /// This will fetch any data required to display the instruction
    /// and stores it in a format ready to be rendered.
    fn prepare_instruction_display(instruction: Instruction, emu: &GameBoy) -> Self {
        // address
        let label_address = {
            let address_str = format!("{:04x}", instruction.opcode_address);
            GemiStyle::ADDRESS.rich_text(address_str)
        };

        // instruction bytes
        let label_opcode_bytes = {
            // number of bytes for this instruction (+1 for 0xcb opcodes)
            let num_instruction_bytes = instruction.get_instruction_length();

            let bytes_string =
                    (0..num_instruction_bytes)
                    .into_iter()
                    .map(|offset| instruction.opcode_address.wrapping_add(offset))
                    .map(|address| emu.get_mmu().read_u8(address))
                    .fold(
                        String::new(),
                        |str, byte| format!("{str}{byte:02x} ")
                    )
            ;

            // create a padded string with at least 10 characters to ensure all labels have the same size
            let bytes_string_padded = format!("{bytes_string:<10}");

            GemiStyle::VALUE_READ_ONLY.rich_text(bytes_string_padded)
        };

        // format the opcode label
        let label_opcode_desc = {
            instruction.opcode.tokenize()
                    .into_iter()
                    .map(|token| {
                        match token {
                            Token::Command(cmd) => {
                                GemiStyle::KEYWORD.rich_text(cmd)
                            }

                            Token::Text(t) => {
                                GemiStyle::KEYWORD_LOW.rich_text(t)
                            }

                            Token::Argument(arg) => {
                                let str = instruction.resolve_argument(&arg);
                                GemiStyle::KEYWORD_LOW.rich_text(str)
                            }
                        }
                    })
                    .collect()
        };

        Self {
            instruction,
            label_address,
            label_opcode_bytes,
            label_opcode_desc,
        }
    }


    /// Renders a single instruction into a row
    fn render_as_row(&self, ui: &mut Ui, emu: &GameBoy) {
        // is current
        {
            let current_pc = emu.cpu.get_instruction_pointer();
            let is_current = current_pc == self.instruction.opcode_address;

            if is_current {
                ui.label("\u{23f5}");
            }
            else {
                ui.allocate_space(vec2(12.0, 0.0));
            }
        }

        // address
        {
            ui.label(self.label_address.clone());
        }

        // instruction bytes
        {
            ui.label(self.label_opcode_bytes.clone());
        }

        // format the opcode label
        {
            ui.horizontal(|ui|{
                for token in &self.label_opcode_desc {
                    ui.label(token.clone());

                    // reduce item spacing for each following item
                    ui.style_mut().spacing.item_spacing.x = 0.0;
                }
            });
        }

        ui.end_row();
    }
}
