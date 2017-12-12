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
extern crate image;
extern crate libc;
extern crate openjpeg2_sys;
#[macro_use]
pub extern crate slog;

pub use self::image::DynamicImage;

pub mod decode;
pub mod error;
