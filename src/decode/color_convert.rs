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

use image::Rgba;

/// This is the type only describing the actual ColorSpaces and doesn't allow for the `Unknown` and
/// `Unspecified` variant.
#[allow(dead_code)] // TODO: remove
#[derive(Clone, Debug)]
pub enum ColorSpace {
    CMYK,
    EYCC,
    GRAY,
    SRGB,
    SYCC,
}

/// This is a type used for decoding the color space type as provided by the C API.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ColorSpaceValue {
    CMYK,
    EYCC,
    GRAY,
    SRGB,
    SYCC,
    Unknown(i32),
    Unspecified,
}

impl ColorSpaceValue {
    pub fn determined(&self) -> Option<ColorSpace> {
        match *self {
            ColorSpaceValue::CMYK => Some(ColorSpace::CMYK),
            ColorSpaceValue::EYCC => Some(ColorSpace::EYCC),
            ColorSpaceValue::GRAY => Some(ColorSpace::GRAY),
            ColorSpaceValue::SRGB => Some(ColorSpace::SRGB),
            ColorSpaceValue::SYCC => Some(ColorSpace::SYCC),
            ColorSpaceValue::Unknown(_) | ColorSpaceValue::Unspecified => None,
        }
    }
}

/// The maximum number of components used in any pixel encoding.
pub const MAX_COMPONENTS: usize = 4;
type ArrComponents = [u8; MAX_COMPONENTS];

impl ColorSpace {
    pub fn convert_to_rgba(&self, source: ArrComponents) -> Rgba<u8> {
        let result: [u8; 4] = match *self {
            ColorSpace::SRGB => source,
            ColorSpace::GRAY => [source[0], source[0], source[0], 255],
            _ => unimplemented!(),
        };

        Rgba { data: result }
    }
}
