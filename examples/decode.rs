extern crate jpeg2000;

use jpeg2000::decode::Codec;

fn main() {
    let mut buffer = include_bytes!("./rust-logo-512x512-blk.jp2").to_vec();

    jpeg2000::decode::load_from_memory(&mut buffer[..], Codec::JP2).unwrap();
}
