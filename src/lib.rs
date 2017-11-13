extern crate image;
extern crate libc;
extern crate openjp2_sys;

pub use self::image::{ImageResult, DynamicImage};

pub mod decode;

/*
pub fn load_from_memory(buffer: &[u8], codec: Codec) -> ImageResult<DynamicImage> {
    

    unimplemented!()
}

*/
