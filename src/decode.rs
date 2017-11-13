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

extern "C" fn quiet_callback(_: *const c_char, _: *mut c_void) {}

unsafe fn get_default_decoder_parameters() -> ffi::opj_dparameters {
    let mut jp2_dparams = ffi::opj_dparameters {
        cp_reduce: 0,
        cp_layer: 0,
        infile: [0; 4096],
        outfile: [0; 4096],
        decod_format: 0,
        cod_format: 0,
        DA_x0: 0,
        DA_x1: 0,
        DA_y0: 0,
        DA_y1: 0,
        m_verbose: 0,
        tile_index: 0,
        nb_tile_to_decode: 0,
        jpwl_correct: 0,
        jpwl_exp_comps: 0,
        jpwl_max_tiles: 0,
        flags: 0,
    };
    ffi::opj_set_default_decoder_parameters(&mut jp2_dparams);
    jp2_dparams
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
        let mut jp2_dparams = get_default_decoder_parameters();
        if ffi::opj_setup_decoder(jp2_codec, &mut jp2_dparams) != 1 {
            ffi::opj_stream_destroy(jp2_stream);
            ffi::opj_destroy_codec(jp2_codec);
            return Err(Error::DecoderSetup);
        }

        // Set quiet callbacks.
        ffi::opj_set_info_handler(jp2_codec, Some(quiet_callback), null_mut());
        ffi::opj_set_warning_handler(jp2_codec, Some(quiet_callback), null_mut());
        ffi::opj_set_error_handler(jp2_codec, Some(quiet_callback), null_mut());

        // Read header.
        //let mut jp2_image = libc::malloc(mem::size_of::<ffi::opj_image>()) as *mut ffi::opj_image;
        //let mut jp2_image: *mut ffi::opj_image = null_mut();
        /*let mut jp2_image: *mut ffi::opj_image = &mut ffi::opj_image {
            x0: 0,
            y0: 0,
            x1: 0,
            y1: 0,
            numcomps: 0,
            color_space: 0,
            comps: null_mut(),
            icc_profile_buf: null_mut(),
            icc_profile_len: 0,
        };*/

        let mut jp2_image: *mut *mut ffi::opj_image = null_mut();

        if ffi::opj_read_header(jp2_stream, jp2_codec, jp2_image) != 1 {
            ffi::opj_stream_destroy(jp2_stream);
            ffi::opj_destroy_codec(jp2_codec);
            //ffi::opj_image_destroy(jp2_image);
            //libc::free(jp2_image as *mut libc::c_void);
            return Err(Error::ReadHeader);
        }
        //ffi::opj_image_destroy(jp2_image);
        //libc::free(jp2_image as *mut libc::c_void);

        Ok(())
    }
}
