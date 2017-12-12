/// jpeg2000: Rust bindings to the OpenJPEG library.
/// Copyright (C) 2017 Leonardo Schwarz <mail@leoschwarz.com>
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU General Public License for more details.
///
/// You should have received a copy of the GNU General Public License
/// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DecodeError {
    /// Weird FFI errors that should never happen
    /// (i.e. if you get this with a published version it's a bug.)
    FfiError(&'static str),

    /// Reading the header failed for some reason.
    ReadHeader,

    /// There was a null byte in the string.
    NullInString,

    /// There were too many components in the supplied file.
    /// If it was a valid file this is a bug in the crate too.
    TooManyComponents(usize),

    UnspecifiedColorSpace,
    UnknownColorSpace,
}

impl From<::std::ffi::NulError> for DecodeError {
    fn from(_: ::std::ffi::NulError) -> Self {
        DecodeError::NullInString
    }
}

impl Error for DecodeError {
    fn description(&self) -> &str {
        match *self {
            DecodeError::FfiError(e) => e,
            DecodeError::ReadHeader => "reading the header failed",
            DecodeError::NullInString => "there was a null byte in the string",
            DecodeError::TooManyComponents(_) => {
                "there were too many components in the supplied file."
            }
            DecodeError::UnspecifiedColorSpace => "Color space was not specified.",
            DecodeError::UnknownColorSpace => "Color space is unknown.",
        }
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
