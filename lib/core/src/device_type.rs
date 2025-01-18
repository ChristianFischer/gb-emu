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


pub mod abbreviations {
    pub const DMG:  &str = "dmg";
    pub const MGB:  &str = "mgb";
    pub const GBC:  &str = "gbc";
    pub const GBA:  &str = "gba";
    pub const AGS:  &str = "ags";
    pub const SGB:  &str = "sgb";
    pub const SGB2: &str = "sgb2";
}


/// The type of GameBoy device to be emulated.
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DeviceType {
    /// The original GameBoy with monochrome 4 color display.
    /// DMG = Dot Matrix Game
    GameBoyDmg,

    /// GameBoy Pocket / MGB; also includes GameBoy Light
    GameBoyPocket,

    /// GameBoy Color with slightly more RAM and color support.
    GameBoyColor,

    /// GameBoy Advance
    GameBoyAdvance,

    /// GameBoy Advance SP
    GameBoyAdvanceSP,

    /// Super GameBoy
    SuperGameBoy,

    /// Super GameBoy 2
    SuperGameBoy2,
}


/// Depending on the device and ROM being emulated, the type of
/// emulation running. For example, the GameBoy Color hardware may
/// run in DMG compatibility mode when a ROM without GBC support
/// is played.
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EmulationType {
    /// Classic GameBoy or compatibility mode.
    DMG,

    /// GameBoy Color support enabled.
    GBC,
}


/// A struct containing the setup information of the running device.
#[derive(Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceConfig {
    /// The current device type being running.
    pub device: DeviceType,

    /// The current emulation mode (DMG compatibility or Color support)
    pub emulation: EmulationType,

    /// Flag if opcodes should be printed
    pub print_opcodes: bool,
}


impl DeviceType {

    /// List of all valid devices.
    pub const ALL_DEVICES: [DeviceType; 6] = [
        DeviceType::GameBoyDmg,
        DeviceType::GameBoyPocket,
        DeviceType::GameBoyColor,
        DeviceType::GameBoyAdvance,
        DeviceType::SuperGameBoy,
        DeviceType::SuperGameBoy2,
    ];

    /// A list of all devices with GameBoy Color compatibility.
    pub const GBC_DEVICES: [DeviceType; 3] = [
        DeviceType::GameBoyColor,
        DeviceType::GameBoyAdvance,
        DeviceType::GameBoyAdvanceSP,
    ];


    /// Get the device type matching the given abbreviation.
    /// Returns `None` if the abbreviation is invalid.
    pub fn from_abbreviation(abbreviation: &str) -> Option<DeviceType> {
        match abbreviation {
            abbreviations::DMG  => Some(DeviceType::GameBoyDmg),
            abbreviations::MGB  => Some(DeviceType::GameBoyPocket),
            abbreviations::GBC  => Some(DeviceType::GameBoyColor),
            abbreviations::GBA  => Some(DeviceType::GameBoyAdvance),
            abbreviations::AGS  => Some(DeviceType::GameBoyAdvanceSP),
            abbreviations::SGB  => Some(DeviceType::SuperGameBoy),
            abbreviations::SGB2 => Some(DeviceType::SuperGameBoy2),
            _                   => None,
        }
    }


    /// Get an abbreviation for the device type.
    pub fn get_abbreviation(&self) -> &'static str {
        match self {
            DeviceType::GameBoyDmg       => abbreviations::DMG,
            DeviceType::GameBoyPocket    => abbreviations::MGB,
            DeviceType::GameBoyColor     => abbreviations::GBC,
            DeviceType::GameBoyAdvance   => abbreviations::GBA,
            DeviceType::GameBoyAdvanceSP => abbreviations::AGS,
            DeviceType::SuperGameBoy     => abbreviations::SGB,
            DeviceType::SuperGameBoy2    => abbreviations::SGB2,
        }
    }


    /// Checks whether this device type has GameBoy Color support.
    pub fn has_gbc_support(&self) -> bool {
        match self {
            DeviceType::GameBoyDmg       => false,
            DeviceType::GameBoyPocket    => false,
            DeviceType::GameBoyColor     => true,
            DeviceType::GameBoyAdvance   => true,
            DeviceType::GameBoyAdvanceSP => true,
            DeviceType::SuperGameBoy     => false,
            DeviceType::SuperGameBoy2    => false,
        }
    }


    /// Get a string representation of the device type.
    pub fn to_string(&self) -> &'static str {
        match self {
            DeviceType::GameBoyDmg       => "GameBoyDmg",
            DeviceType::GameBoyPocket    => "GameBoyPocket",
            DeviceType::GameBoyColor     => "GameBoyColor",
            DeviceType::GameBoyAdvance   => "GameBoyAdvance",
            DeviceType::GameBoyAdvanceSP => "GameBoyAdvance",
            DeviceType::SuperGameBoy     => "SuperGameBoy",
            DeviceType::SuperGameBoy2    => "SuperGameBoy2",
        }
    }
}


impl DeviceConfig {
    /// Preconfigured [DeviceType] for classic GameBoy emulation.
    pub const DEFAULT_DMG: DeviceConfig = DeviceConfig {
        device: DeviceType::GameBoyDmg,
        emulation: EmulationType::DMG,
        print_opcodes: false,
    };


    /// Preconfigured [DeviceType] for GameBoy Color emulation.
    pub const DEFAULT_GBC: DeviceConfig = DeviceConfig {
        device: DeviceType::GameBoyColor,
        emulation: EmulationType::GBC,
        print_opcodes: false,
    };


    /// Checks whether the current device is running with GameBoyColor support enabled.
    /// The running device needs to be a GBC or GBA *and* running a cartridge
    /// with GameBoy Color support.
    pub fn is_gbc_enabled(&self) -> bool {
        match self.emulation {
            EmulationType::DMG => false,
            EmulationType::GBC => true,
        }
    }
}
