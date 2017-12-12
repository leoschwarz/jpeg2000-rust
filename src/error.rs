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

#[derive(Debug)]
pub enum DecodeError {
    /// Weird FFI errors that should never happen
    /// (i.e. if you get this with a published version it's a bug.)
    FfiError(&'static str),

    /// Reading the header failed for some reason.
    ReadHeader,

    /// There were too many components in the supplied file.
    /// If it was a valid file this is a bug in the crate too.
    TooManyComponents(usize),

    // TODO: This should not be a problem in the future.
    UnspecifiedColorSpace,
    UnknownColorSpace,
}
