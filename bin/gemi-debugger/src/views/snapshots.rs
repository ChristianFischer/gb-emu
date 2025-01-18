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

use crate::state::EmulatorState;
use crate::strings::{BUTTON_LABEL_ADD, BUTTON_LABEL_DELETE};
use crate::views::View;
use chrono::{DateTime, Utc};
use eframe::epaint::textures::TextureOptions;
use eframe::epaint::{ColorImage, Vec2};
use egui::{Image, Sense, TextStyle, TextureHandle, Ui, Widget};
use egui_extras::{Column, TableBuilder, TableRow};
use libgemi::core::device_type::DeviceType;
use libgemi::core::emulator_core::Clock;
use libgemi::core::mmu::memory_data::MemoryData;
use libgemi::core::ppu::ppu::{SCREEN_H, SCREEN_W};
use libgemi::snapshots::Snapshot;
use std::io;


#[derive(serde::Serialize, serde::Deserialize)]
pub struct SnapshotsView {
    /// The list of snapshots stored in the application.
    snapshots: Vec<SnapshotEntry>,

    /// The index of the currently selected entry.
    selected_entry: Option<usize>,
}


#[derive(serde::Serialize, serde::Deserialize)]
struct SnapshotEntry {
    /// The title of the ROM, from which the snapshot was taken.
    rom_title: String,

    /// The actual snapshot image.
    snapshot: Snapshot,

    /// Time when the snapshot was taken.
    created_at: DateTime<Utc>,
    
    /// The device type the snapshot was created on.
    created_on_device: DeviceType,

    /// The time in seconds, the emulator was running until the snapshot was taken.
    runtime_seconds: f32,

    /// The number of cycles processed until the snapshot was taken.
    runtime_cycles: Clock,

    /// Image data of the emulator's screen, to be used as a thumbnail.
    thumbnail_data: ColorImage,

    /// The image data of [Self::thumbnail_data] as a drawable texture.
    #[serde(skip)]
    thumbnail_texture: Option<TextureHandle>,
}


// aspect ratio of the gameboy screen
const SCREEN_ASPECT : f32 = (SCREEN_W as f32) / (SCREEN_H as f32);


impl View for SnapshotsView {
    fn title(&self, _state: &mut EmulatorState) -> &str {
        "Snapshots"
    }


    fn ui(&mut self, state: &mut EmulatorState, ui: &mut Ui) {
        let text_height = ui.text_style_height(&TextStyle::Monospace);
        let item_height = text_height;

        ui.vertical(|ui| {
            egui::TopBottomPanel::top("snapshots_menu_bar").show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    self.update_toolbar(ui, state);
                });
            });

            TableBuilder::new(ui)
                    .column(Column::exact(item_height * SCREEN_ASPECT).resizable(false)) // thumbnail
                    .column(Column::auto().resizable(true))     // rom title
                    .column(Column::auto().resizable(true))     // device type
                    .column(Column::auto().resizable(true))     // runtime
                    .column(Column::auto().resizable(true))     // creation time
                    .column(Column::auto().resizable(true))     // size
                    .column(Column::remainder())                // empty remainder

                    .vscroll(true)
                    .striped(true)
                    .sense(egui::Sense::click())

                    .header(text_height, |mut header| {
                        header.col(|_|  { /* thumbnail */               } );
                        header.col(|ui| { ui.heading("ROM Title");      } );
                        header.col(|ui| { ui.heading("Device");         } );
                        header.col(|ui| { ui.heading("Runtime");        } );
                        header.col(|ui| { ui.heading("Creation Time");  } );
                        header.col(|ui| { ui.heading("Size");           } );
                        header.col(|_|  {                           } );
                    })

                    .body(|body| {
                        body.rows(
                            item_height,
                            self.snapshots.len(),
                            |row| {
                                self.display_entry(
                                    state,
                                    row,
                                    item_height
                                );
                            }
                        )
                    })
            ;
        });
    }
}


impl SnapshotsView {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            selected_entry: None,
        }
    }
    
    
    fn update_toolbar(&mut self, ui: &mut Ui, state: &mut EmulatorState) {
        // Button "new"
        {
            let has_emu = state.emu.is_emulator_loaded();
            ui.add_enabled_ui(has_emu, |ui| {
                if ui.button(BUTTON_LABEL_ADD).clicked() {
                    if let Ok(entry) = SnapshotEntry::create_from(state) {
                        self.snapshots.push(entry);
                    }
                }
            });
        }

        // Button "delete"
        {
            let has_selected_line = self.selected_entry.is_some();
            ui.add_enabled_ui(has_selected_line, |ui| {
                if ui.button(BUTTON_LABEL_DELETE).clicked() {
                    if let Some(index) = self.selected_entry {
                        self.snapshots.remove(index);
                        self.selected_entry = None;
                    }
                }
            });
        }
    }


    fn display_entry(&mut self, _state: &mut EmulatorState, mut row: TableRow, item_height: f32) {
        let index = row.index();
        let entry = &mut self.snapshots[index];
        
        // is selected?
        row.set_selected(self.selected_entry == Some(index));

        // thumbnail
        row.col(|ui| {
            let texture : &TextureHandle = entry.thumbnail_texture.get_or_insert_with(
                || ui.ctx().load_texture("thumbnail", entry.thumbnail_data.clone(), TextureOptions::NEAREST)
            );

            let image_response = Image::new(texture)
                    .fit_to_exact_size(Vec2::new(item_height * SCREEN_ASPECT, item_height))
                    .sense(Sense::hover())
                    .ui(ui)
            ;

            image_response.on_hover_ui(|ui| {
                Image::new(texture)
                        .ui(ui);
            });
        });

        // ROM title
        row.col(|ui| {
            ui.label(&entry.rom_title);
        });
        
        // Device type
        row.col(|ui| {
            ui.label(entry.created_on_device.get_abbreviation());
        });

        // runtime
        row.col(|ui| {
            ui.label(format!("{:.1}s", entry.runtime_seconds));
        });

        // creation time
        row.col(|ui| {
            let current_time = entry.created_at;
            let local_time   = current_time.with_timezone(&chrono::Local);
            ui.label(local_time.format("%c").to_string());
        });

        // data size
        row.col(|ui| {
            let kib = (entry.snapshot.get_data().len() + 1023) / 1024;
            ui.label(format!("{kib} kiB"));
        });

        // Remainder
        row.col(|_| { });
        
        // handle interactions
        if row.response().double_clicked() {
        }
        else if row.response().clicked() {
            self.selected_entry = Some(index);
        }
    }
}


impl SnapshotEntry {
    fn create_from(state: &EmulatorState) -> io::Result<Self> {
        let gb = state.emu.get_gameboy().ok_or_else(
            || io::Error::new(io::ErrorKind::NotFound, "Emulator not found")
        )?;

        // create the snapshot itself
        let snapshot = Snapshot::create_from(gb)?;

        // get ROM title
        let rom_title = match gb.get_memory().get_cartridge() {
            Some(cartridge) => cartridge.get_title().to_string(),
            None => String::from("---"),
        };

        // create image data of the screen to generate a thumbnail
        let lcd  = gb.get_ppu().get_lcd();
        let image = ColorImage::from_rgba_unmultiplied(
            [lcd.get_width() as usize, lcd.get_height() as usize],
            lcd.get_pixels().as_slice()
        );

        Ok(SnapshotEntry {
            rom_title,
            snapshot,
            created_at:         Utc::now(),
            created_on_device:  gb.get_config().device,
            runtime_seconds:    gb.get_total_seconds_processed(),
            runtime_cycles:     gb.get_total_cycles_processed(),
            thumbnail_data:     image,
            thumbnail_texture:  None,
        })
    }
}
