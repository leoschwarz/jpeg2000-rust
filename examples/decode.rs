extern crate jpeg2000;
extern crate image;

use std::fs::File;
use jpeg2000::decode::Codec;

fn main() {
    let mut buffer = include_bytes!("./rust-logo-512x512-blk.jp2").to_vec();

    let img = jpeg2000::decode::load_from_memory(&mut buffer[..], Codec::JP2).unwrap();

    let mut output = File::create("result.png").unwrap();
    img.save(&mut output, image::ImageFormat::PNG);
}
