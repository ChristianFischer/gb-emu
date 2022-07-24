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

use crate::gameboy::GameBoy;

pub fn nop(gb: &mut GameBoy) {}

pub fn stop(gb: &mut GameBoy) {
}

pub fn halt(gb: &mut GameBoy) {
}

pub fn disable_interrupts(gb: &mut GameBoy) {
}

pub fn enable_interrupts(gb: &mut GameBoy) {
}
