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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "dyn_alloc")]
extern crate alloc;
extern crate core;

pub mod apu;
pub mod boot_rom;
pub mod cartridge;
pub mod cpu;
pub mod debug;
pub mod device_type;
pub mod gameboy;
pub mod input;
pub mod mmu;
pub mod ppu;
pub mod serial;
pub mod snapshots;
pub mod timer;
pub mod utils;

