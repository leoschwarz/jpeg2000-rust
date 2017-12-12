extern crate image;
extern crate jpeg2000;

use std::fs::File;
use jpeg2000::decode::{Codec, ColorSpace, DecodeConfig};

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

    for (mut data, basename, codec) in images {
        let img = jpeg2000::decode::load_from_memory(
            &mut data[..],
            codec,
            DecodeConfig {
                default_colorspace: Some(ColorSpace::SRGB),
                discard_level: 0,
            },
        ).unwrap();

        let mut output = File::create(format!("{}.png", basename)).unwrap();
        let _ = img.save(&mut output, image::ImageFormat::PNG);
    }
}
