use openjp2_sys as ffi;
use std::os::raw::{c_char, c_void, c_int};
use std::ptr::{null, null_mut};
use std::mem;
use libc;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Codec {
    J2K,
    JP2,
    JPP,
    JPT,
    JPX,
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

#[derive(Debug)]
pub enum Error {
    /// Invalid codec was selected.
    InvalidCodec,
    /// Something went wrong setting up the decoder.
    DecoderSetup,
    /// Reading the header failed.
    ReadHeader,
}

pub fn load_from_memory(buf: &mut [u8], codec: Codec) -> Result<(), Error> {
    unsafe {
        // Setup the stream.
        println!("buf.len() = {}", buf.len());
        //let jp2_stream = ffi::opj_stream_create(buf.len(), 1);
        let jp2_stream = ffi::opj_stream_default_create(1);
        ffi::opj_stream_set_user_data(jp2_stream, buf.as_mut_ptr() as *mut c_void, None);
        ffi::opj_stream_set_user_data_length(jp2_stream, buf.len() as u64);

        // Setup the decoder.
        let jp2_codec = ffi::opj_create_decompress(codec.to_i32());
        if jp2_codec.is_null() {
            ffi::opj_stream_destroy(jp2_stream);
            return Err(Error::InvalidCodec);
        }
        let jp2_dparams = libc::malloc(mem::size_of::<ffi::opj_dparameters>()) as
            *mut ffi::opj_dparameters;
        ffi::opj_set_default_decoder_parameters(jp2_dparams);
        if ffi::opj_setup_decoder(jp2_codec, jp2_dparams) != 1 {
            ffi::opj_stream_destroy(jp2_stream);
            ffi::opj_destroy_codec(jp2_codec);
            libc::free(jp2_dparams as *mut libc::c_void);
            return Err(Error::DecoderSetup);
        }
        libc::free(jp2_dparams as *mut libc::c_void);

        // Read header.
        //let mut jp2_image = libc::malloc(mem::size_of::<ffi::opj_image>()) as *mut ffi::opj_image;
        let mut jp2_image: *mut ffi::opj_image = null_mut();
        if ffi::opj_read_header(jp2_stream, jp2_codec, &mut jp2_image) != 1 {
            ffi::opj_stream_destroy(jp2_stream);
            ffi::opj_destroy_codec(jp2_codec);
            ffi::opj_image_destroy(jp2_image);
            libc::free(jp2_image as *mut libc::c_void);
            return Err(Error::ReadHeader);
        }
        ffi::opj_image_destroy(jp2_image);
        libc::free(jp2_image as *mut libc::c_void);

        Ok(())
        /*
        let errcode: c_int = ffi::wrapper_read_buffer(
            buf.as_mut_ptr() as *mut c_char,
            buf.len() as i32,
            codec.to_i32(),
        );

        if errcode != 0 {
            panic!("there was an error: {}", errcode);
        } else {
            println!("wrapper_read_buffer completed without error");
        }
        */
    }
}
