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

use alloc::boxed::Box;

use crate::utils::SerializableArray;


#[cfg(feature = "file_io")]
use std::{
    fs::File,
    io,
    io::Read,
    path::Path,
};


/// A data object containing a 256 byte boot ROM.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BootRom {
    rom: Box<SerializableArray<u8, 256>>,
}


impl BootRom {
    /// Load a boot rom from a file.
    #[cfg(feature = "file_io")]
    pub fn load_file(filepath: &Path) -> Result<BootRom, io::Error> {
        use crate::utils::ioerr;

        let mut file = File::open(filepath)?;
        let metadata  = file.metadata()?;
        let file_size = metadata.len();

        // fail when the boot rom image has an unexpected size
        if file_size != 256 {
            let error = ioerr::Error {
                source: ioerr::Source::BootRomImage,
                source_file: Some(filepath.to_path_buf()),
                error_code: ioerr::ErrorCode::InvalidFileSize(ioerr::InvalidFileSizeError {
                    expected: 256,
                    actual: file_size as usize,
                })
            };

            return Err(error.into());
        }

        let mut buffer = [0u8; 256];
        file.read_exact(&mut buffer)?;

        Ok(BootRom::new(buffer))
    }


    /// Creates a new `BootRom` object from existing data.
    pub fn new(data: [u8; 256]) -> BootRom {
        BootRom {
            rom: Box::new(data.into())
        }
    }

    
    /// Get data from the boot ROM.
    pub fn read(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }
}
