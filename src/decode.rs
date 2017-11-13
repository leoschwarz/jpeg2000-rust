use openjp2_sys as ffi;
use std::os::raw::{c_char, c_void};

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Codec {
    J2K,
    JP2,
    JPP,
    JPT,
    JPX
}

impl Codec {
    fn to_i32(&self) -> i32 {
        match *self {
            Codec::J2K => ffi::CODEC_FORMAT_OPJ_CODEC_J2K,
            Codec::JP2 => ffi::CODEC_FORMAT_OPJ_CODEC_JP2,
            Codec::JPP => ffi::CODEC_FORMAT_OPJ_CODEC_JPP,
            Codec::JPT => ffi::CODEC_FORMAT_OPJ_CODEC_JPT,
            Codec::JPX => ffi::CODEC_FORMAT_OPJ_CODEC_JPX,
        }
    }
}

//#[link(name="openjp2")]
pub fn load_from_memory(buf: &mut [u8], codec: Codec) {
    unsafe {
        // Setup the stream.
        let c_stream = ffi::opj_stream_create(buf.len(), 1);


        //let output: Vec<u8> = Vec::new(); // TODO how will this work!?
        //ffi::wrapper_read_buffer(buffer.as_mut_ptr() as *mut c_char, buffer.len() as i32, codec.to_i32());

        /*
        let stream = ffi::opj_stream_create(buffer.len(), 1);
        ffi::opj_stream_set_user_data(stream, buffer.as_mut_ptr() as *mut c_void, None);
        */
    }
}
