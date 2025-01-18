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

#[cfg(feature = "snapshots")]
pub use with_snapshots::Snapshot as Snapshot;

#[cfg(feature = "snapshots")]
mod with_snapshots {
    use crate::GameBoy;
    use gemi_core::utils::SerializableBuffer;
    use std::fs::File;
    use std::io;
    use std::io::{Read, Write};
    use std::path::Path;

    /// Contains a full snapshot of an emulator instance,
    /// including the whole ROM and RAM.
    /// The snapshot can be used to restore the emulator
    /// instance.
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Snapshot {
        /// Binary form of the serialized emulator state.
        data: SerializableBuffer<u8>,
    }


    impl Snapshot {
        /// Creates a new snapshot from an existing emulator instance.
        pub fn create_from(gb: &GameBoy) -> io::Result<Self> {
            let data = bincode::serde::encode_to_vec(
                gb,
                bincode::config::standard()
            ).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            Ok(Self { data: data.into() })
        }


        /// Restores a previously serialized snapshot into a new [GameBoy] instance.
        pub fn restore(&self) -> io::Result<GameBoy> {
            let (result, _) = bincode::serde::decode_from_slice::<GameBoy, _>(
                &self.data,
                bincode::config::standard()
            ).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            Ok(result)
        }


        /// Reads a snapshot from a file path.
        pub fn read_from_file(filepath: &Path) -> io::Result<Self> {
            let mut file = File::open(filepath)?;
            let mut data = Vec::new();
            file.read_to_end(&mut data)?;

            Ok(Self { data: data.into() })
        }


        /// Saves a snapshot into a file path.
        pub fn save_to_file(&self, filepath: &Path) -> io::Result<()> {
            let mut file = File::create(filepath)?;
            file.write_all(&self.data)?;

            Ok(())
        }


        /// Get the serialized data of this snapshot.
        pub fn get_data(&self) -> &[u8] {
            &self.data
        }
    }
}
