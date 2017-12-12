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
extern crate jpeg2000;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use jpeg2000::decode::{Codec, ColorSpace, DecodeConfig};

use std::fs::File;
use slog::Drain;

fn get_logger() -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(drain, o!())
}

fn main() {
    let images = vec![
        (
            include_bytes!("./images/rust_logo.jp2").to_vec(),
            "rust_logo",
            Codec::JP2,
        ),
        (
            include_bytes!("./images/opensim_texture.jp2").to_vec(),
            "opensim_texture",
            Codec::J2K,
        ),
    ];

    let logger = get_logger();
    for (mut data, basename, codec) in images {
        let img = jpeg2000::decode::from_memory(
            &mut data[..],
            codec,
            DecodeConfig {
                default_colorspace: Some(ColorSpace::SRGB),
                discard_level: 0,
            },
            Some(logger.clone()),
        ).unwrap();

        let mut output = File::create(format!("output/{}.png", basename)).unwrap();
        let _ = img.save(&mut output, image::ImageFormat::PNG);
    }
}
