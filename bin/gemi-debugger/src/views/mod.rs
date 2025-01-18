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

use egui::Ui;
use libgemi::core::ppu::graphic_data::TileMap;

use crate::event::UiEvent;
use crate::selection::Selected;
use crate::state::EmulatorState;
use crate::views::cartridge_info::CartridgeInfoView;
use crate::views::cpu::CpuView;
use crate::views::disassembly::DisassemblyView;
use crate::views::display::EmulatorDisplayView;
use crate::views::file_browser::FileBrowserView;
use crate::views::memory::MemoryView;
use crate::views::oam::OamView;
use crate::views::palettes::PaletteView;
use crate::views::snapshots::SnapshotsView;
use crate::views::sprites::SpritesView;
use crate::views::tilemap::TileMapView;

mod cartridge_info;
mod cpu;
mod disassembly;
mod display;
mod file_browser;
mod memory;
mod oam;
mod palettes;
mod snapshots;
mod sprites;
mod tilemap;


/// A trait to be implemented by view objects of which each of them display
/// their unique aspect of information about the currently running emulator.
pub trait View: serde::Serialize + serde::de::DeserializeOwned {
    /// Get the title of the view.
    fn title(&self, state: &mut EmulatorState) -> &str;

    /// Render the view UI.
    fn ui(&mut self, state: &mut EmulatorState, ui: &mut Ui);

    /// Get the currently selected item.
    fn get_current_selection(&self) -> Option<Selected> {
        None
    }

    /// Invoked when an UI Event occurred to be handled by views.
    fn handle_ui_event(&mut self, event: &UiEvent) {
        _ = event;
    }

    /// Invoked when a new instance of the emulator was created.
    fn on_emulator_loaded(&mut self, state: &mut EmulatorState) {
        _ = state;
    }
}


/// An enum to store the different view classes.
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ViewClass {
    FileBrowser(FileBrowserView),
    Display(EmulatorDisplayView),
    CartridgeInfo(CartridgeInfoView),
    Cpu(CpuView),
    Memory(MemoryView),
    Disassembly(DisassemblyView),
    TileMap(TileMapView),
    Sprites(SpritesView),
    Palettes(PaletteView),
    Oam(OamView),
    Snapshots(SnapshotsView),
}


pub type Instantiate = fn() -> ViewClass;


impl ViewClass {
    pub const ALL : [(&'static str, Instantiate); 13] = [
        ("Display",         ViewClass::new_display_view),
        ("CartridgeInfo",   ViewClass::new_cartridge_info),
        ("Cpu",             ViewClass::new_cpu),
        ("Disassembly",     ViewClass::new_disassembly),
        ("Memory",          ViewClass::new_memory),
        ("Oam",             ViewClass::new_oam),
        ("Palettes",        ViewClass::new_palettes),
        ("Sprites Bank #0", || ViewClass::new_sprites(0)),
        ("Sprites Bank #1", || ViewClass::new_sprites(1)),
        ("TileMap #9800",   || ViewClass::new_tile_map(TileMap::H9800)),
        ("TileMap #9C00",   || ViewClass::new_tile_map(TileMap::H9C00)),
        ("FileBrowser",     ViewClass::new_file_browser),
        ("Snapshots",       ViewClass::new_snapshots),
    ];
    
    
    /// Creates a new [`FileBrowserView`] object.
    pub fn new_file_browser() -> ViewClass {
        ViewClass::FileBrowser(FileBrowserView::new())
    }

    
    /// Creates a new [`EmulatorDisplayView`] object.
    pub fn new_display_view() -> ViewClass {
        ViewClass::Display(EmulatorDisplayView::new())
    }


    /// Creates a new [`CartridgeInfoView`] object.
    pub fn new_cartridge_info() -> ViewClass {
        ViewClass::CartridgeInfo(CartridgeInfoView::new())
    }


    /// Creates a new [`CpuView`] object.
    pub fn new_cpu() -> ViewClass {
        ViewClass::Cpu(CpuView::new())
    }


    /// Creates a new [`MemoryView`] object.
    pub fn new_memory() -> ViewClass {
        ViewClass::Memory(MemoryView::new())
    }


    /// Creates a new [`DisassemblyView`] object.
    pub fn new_disassembly() -> ViewClass {
        ViewClass::Disassembly(DisassemblyView::new())
    }


    /// Creates a new [`TileMapView`] object.
    pub fn new_tile_map(tilemap: TileMap) -> ViewClass {
        ViewClass::TileMap(TileMapView::new(tilemap))
    }


    /// Creates a new [`SpritesView`] object.
    pub fn new_sprites(bank_index: u8) -> ViewClass {
        ViewClass::Sprites(SpritesView::new(bank_index))
    }


    /// Creates a new [`PaletteView`] object.
    pub fn new_palettes() -> ViewClass {
        ViewClass::Palettes(PaletteView::new())
    }


    /// Creates a new [`OamView`] object.
    pub fn new_oam() -> ViewClass {
        ViewClass::Oam(OamView::new())
    }
    
    
    /// Creates a new [`SnapshotsView`] object.
    pub fn new_snapshots() -> ViewClass {
        ViewClass::Snapshots(SnapshotsView::new())
    }
}


impl View for ViewClass {
    fn title(&self, state: &mut EmulatorState) -> &str {
        match self {
            ViewClass::FileBrowser(v)   => v.title(state),
            ViewClass::Display(v)       => v.title(state),
            ViewClass::CartridgeInfo(v) => v.title(state),
            ViewClass::Cpu(v)           => v.title(state),
            ViewClass::Memory(v)        => v.title(state),
            ViewClass::Disassembly(v)   => v.title(state),
            ViewClass::TileMap(v)       => v.title(state),
            ViewClass::Sprites(v)       => v.title(state),
            ViewClass::Palettes(v)      => v.title(state),
            ViewClass::Oam(v)           => v.title(state),
            ViewClass::Snapshots(v)     => v.title(state),
        }
    }


    fn ui(&mut self, state: &mut EmulatorState, ui: &mut Ui) {
        match self {
            ViewClass::FileBrowser(v)   => v.ui(state, ui),
            ViewClass::Display(v)       => v.ui(state, ui),
            ViewClass::CartridgeInfo(v) => v.ui(state, ui),
            ViewClass::Cpu(v)           => v.ui(state, ui),
            ViewClass::Memory(v)        => v.ui(state, ui),
            ViewClass::Disassembly(v)   => v.ui(state, ui),
            ViewClass::TileMap(v)       => v.ui(state, ui),
            ViewClass::Sprites(v)       => v.ui(state, ui),
            ViewClass::Palettes(v)      => v.ui(state, ui),
            ViewClass::Oam(v)           => v.ui(state, ui),
            ViewClass::Snapshots(v)     => v.ui(state, ui),
        }
    }


    fn get_current_selection(&self) -> Option<Selected> {
        match self {
            ViewClass::FileBrowser(v)   => v.get_current_selection(),
            ViewClass::Display(v)       => v.get_current_selection(),
            ViewClass::CartridgeInfo(v) => v.get_current_selection(),
            ViewClass::Cpu(v)           => v.get_current_selection(),
            ViewClass::Memory(v)        => v.get_current_selection(),
            ViewClass::Disassembly(v)   => v.get_current_selection(),
            ViewClass::TileMap(v)       => v.get_current_selection(),
            ViewClass::Sprites(v)       => v.get_current_selection(),
            ViewClass::Palettes(v)      => v.get_current_selection(),
            ViewClass::Oam(v)           => v.get_current_selection(),
            ViewClass::Snapshots(v)     => v.get_current_selection(),
        }
    }


    fn handle_ui_event(&mut self, event: &UiEvent) {
        match self {
            ViewClass::FileBrowser(v)   => v.handle_ui_event(event),
            ViewClass::Display(v)       => v.handle_ui_event(event),
            ViewClass::CartridgeInfo(v) => v.handle_ui_event(event),
            ViewClass::Cpu(v)           => v.handle_ui_event(event),
            ViewClass::Memory(v)        => v.handle_ui_event(event),
            ViewClass::Disassembly(v)   => v.handle_ui_event(event),
            ViewClass::TileMap(v)       => v.handle_ui_event(event),
            ViewClass::Sprites(v)       => v.handle_ui_event(event),
            ViewClass::Palettes(v)      => v.handle_ui_event(event),
            ViewClass::Oam(v)           => v.handle_ui_event(event),
            ViewClass::Snapshots(v)     => v.handle_ui_event(event),
        }
    }


    fn on_emulator_loaded(&mut self, state: &mut EmulatorState) {
        match self {
            ViewClass::FileBrowser(v)   => v.on_emulator_loaded(state),
            ViewClass::Display(v)       => v.on_emulator_loaded(state),
            ViewClass::CartridgeInfo(v) => v.on_emulator_loaded(state),
            ViewClass::Cpu(v)           => v.on_emulator_loaded(state),
            ViewClass::Memory(v)        => v.on_emulator_loaded(state),
            ViewClass::Disassembly(v)   => v.on_emulator_loaded(state),
            ViewClass::TileMap(v)       => v.on_emulator_loaded(state),
            ViewClass::Sprites(v)       => v.on_emulator_loaded(state),
            ViewClass::Palettes(v)      => v.on_emulator_loaded(state),
            ViewClass::Oam(v)           => v.on_emulator_loaded(state),
            ViewClass::Snapshots(v)     => v.on_emulator_loaded(state),
        }
    }
}
