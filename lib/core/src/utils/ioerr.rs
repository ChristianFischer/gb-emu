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

#[cfg(feature = "file_io")]
use std::path::PathBuf;

use std::fmt::{Display, Formatter};
use std::result;


/// Information about an IO error with the error source attached.
pub struct Error {
    /// The source type of where the error is related to.
    pub source: Source,

    /// Optionally: a file which caused the error.
    #[cfg(feature = "file_io")]
    pub source_file: Option<PathBuf>,
    
    /// An error code which describes the actual error.
    pub error_code: ErrorCode,
}


/// A source type which describes the module where an error is related to.
pub enum Source {
    BootRomImage,
    RomImage,
    RamImage,
}


/// An error code describing an actual error.
pub enum ErrorCode {
    /// A file to be loaded had an unexpected size.
    /// This may be the case, for example, when loading a RAM image,
    /// which has a different size than the actual RAM.
    InvalidFileSize(InvalidFileSizeError),
}


/// Additional attributes for [ErrorCode::InvalidFileSize].
pub struct InvalidFileSizeError {
    /// The actual size of the file being loaded.
    pub actual: usize,
    
    /// The expected size of the file.
    pub expected: usize,
}


/// An alias type for [result::Result<T, Error>].
pub type Result<T> = result::Result<T, Error>;


impl Display for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::BootRomImage => write!(f, "Boot ROM"),
            Source::RomImage     => write!(f, "ROM"),
            Source::RamImage     => write!(f, "RAM"),
        }
    }
}


impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.source, self.error_code)
    }
}


impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::InvalidFileSize(err) => {
                write!(f, "Invalid file size: {} (expected: {})", err.actual, err.expected)
            }
        }
    }
}


#[cfg(feature = "file_io")]
impl From<Error> for std::io::Error {
    fn from(e: Error) -> Self {
        match &e.error_code {
            ErrorCode::InvalidFileSize(_) => {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_string()
                )
            },
        }
    }
}
