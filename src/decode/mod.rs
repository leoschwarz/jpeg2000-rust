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

use error::DecodeError;
use image::{DynamicImage, GenericImage};
use openjpeg2_sys as ffi;
use slog::{self, Logger};
use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr::null_mut;

mod color_convert;
use self::color_convert::ColorSpaceValue;
pub use self::color_convert::ColorSpace;

mod support;

pub struct DecodeConfig {
    /// Default color space to be used in the case of unspecified values.
    pub default_colorspace: Option<ColorSpace>,
    /// The image resolution is effectively divided by 2 to the power of
    /// the number of discarded levels.
    pub discard_level: u32,
}

impl Default for DecodeConfig {
    fn default() -> Self {
        DecodeConfig {
            default_colorspace: None,
            discard_level: 0,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Codec {
    /// JPEG-2000 codestream.
    J2K,
    /// JP2 file format.
    JP2,
    /// JPP-stream (JPEG 2000, JPIP)
    JPP,
    /// JPT-stream (JPEG 2000, JPIP)
    JPT,
    /// JPX file format (JPEG 2000 Part-2)
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


impl ColorSpaceValue {
    fn from_i32(val: i32) -> Self {
        match val {
            ffi::COLOR_SPACE_OPJ_CLRSPC_CMYK => ColorSpaceValue::CMYK,
            ffi::COLOR_SPACE_OPJ_CLRSPC_EYCC => ColorSpaceValue::EYCC,
            ffi::COLOR_SPACE_OPJ_CLRSPC_GRAY => ColorSpaceValue::GRAY,
            ffi::COLOR_SPACE_OPJ_CLRSPC_SRGB => ColorSpaceValue::SRGB,
            ffi::COLOR_SPACE_OPJ_CLRSPC_SYCC => ColorSpaceValue::SYCC,
            ffi::COLOR_SPACE_OPJ_CLRSPC_UNSPECIFIED => ColorSpaceValue::Unspecified,
            ffi::COLOR_SPACE_OPJ_CLRSPC_UNKNOWN | _ => ColorSpaceValue::Unknown(val),
        }
    }
}

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

/// Divide by 2 to the power of b and round upwards.
#[inline]
fn ceil_div_pow2(a: u32, b: u32) -> u32 {
    (a + (1 << b) - 1) >> b
}

// jp2_stream: this function will take care of deleting this at the end.
unsafe fn load_from_stream(
    jp2_stream: *mut *mut c_void,
    codec: Codec,
    config: DecodeConfig,
    logger: Logger,
) -> Result<DynamicImage, DecodeError> {
    // Setup the codec.
    let jp2_codec = ffi::opj_create_decompress(codec.to_i32());
    if jp2_codec.is_null() {
        ffi::opj_stream_destroy(jp2_stream);
        return Err(DecodeError::FfiError("Codec instantiation failed."));
    }
    let mut data = support::LogHandlerData::new(logger.clone());
    let data_ptr: *mut support::LogHandlerData = &mut data;
    let data_ptr = data_ptr as *mut c_void;
    ffi::opj_set_info_handler(jp2_codec, Some(support::info_handler), data_ptr);
    ffi::opj_set_warning_handler(jp2_codec, Some(support::warning_handler), data_ptr);
    ffi::opj_set_error_handler(jp2_codec, Some(support::error_handler), data_ptr);

    // Setup decoder.
    let mut jp2_dparams = get_default_decoder_parameters();
    jp2_dparams.cp_reduce = config.discard_level;
    if ffi::opj_setup_decoder(jp2_codec, &mut jp2_dparams) != 1 {
        ffi::opj_stream_destroy(jp2_stream);
        ffi::opj_destroy_codec(jp2_codec);
        return Err(DecodeError::FfiError("Setting up the decoder failed."));
    }

    // Read header.
    let mut jp2_image: *mut ffi::opj_image = null_mut();
    if ffi::opj_read_header(jp2_stream, jp2_codec, &mut jp2_image) != 1 {
        ffi::opj_stream_destroy(jp2_stream);
        ffi::opj_destroy_codec(jp2_codec);
        return Err(DecodeError::ReadHeader);
    }

    // Decode the image.
    ffi::opj_decode(jp2_codec, jp2_stream, jp2_image);
    ffi::opj_stream_destroy(jp2_stream);

    let color_space_raw = ColorSpaceValue::from_i32((*jp2_image).color_space);
    let color_space = color_space_raw.determined();
    let color_space: ColorSpace = if color_space.is_none() {
        if color_space_raw == ColorSpaceValue::Unspecified {
            match config.default_colorspace {
                Some(cspace) => cspace,
                None => return Err(DecodeError::UnspecifiedColorSpace),
            }
        } else {
            ffi::opj_destroy_codec(jp2_codec);
            ffi::opj_image_destroy(jp2_image);
            return Err(DecodeError::UnknownColorSpace);
        }
    } else {
        color_space.unwrap()
    };
    info!(logger, "color space: {:?}", color_space);
    info!(logger, "icc_profile_len: {}", (*jp2_image).icc_profile_len);

    let width = (*jp2_image).x1 - (*jp2_image).x0;
    let height = (*jp2_image).y1 - (*jp2_image).y0;
    info!(logger, "width: {}, height: {}", width, height);

    let mut comps: Vec<*mut ffi::opj_image_comp> = Vec::new();
    let comps_len = (*jp2_image).numcomps;
    for i in 0..comps_len {
        comps.push((*jp2_image).comps.offset(i as isize));
    }

    if comps.len() > color_convert::MAX_COMPONENTS {
        ffi::opj_destroy_codec(jp2_codec);
        ffi::opj_image_destroy(jp2_image);
        return Err(DecodeError::TooManyComponents(comps.len()));
    }
    let mut image = DynamicImage::new_rgba8(width, height);
    info!(logger, "number of components: {}", comps.len());

    // Copy the pixels.
    let comp_width = (*comps[0]).w;
    let factor = (*comps[0]).factor;
    let width = ceil_div_pow2(width, factor);
    let height = ceil_div_pow2(height, factor);

    for y in (0..height).rev() {
        for x in 0..width {
            //let index = (x + y * width) as isize;
            let index = (y * comp_width + x) as isize;

            // Note: Initialize the last component value to 255,
            //       since this will be the alpha channel value in case
            //       there is actually no transparency.
            let mut values = [0u8, 0, 0, 255];
            for i in 0..comps.len() {
                let data = (*comps[i]).data;
                //assert!((*comps[i]).sgnd == 0); // TODO signed numbers?!
                let ivalue: u8 = *data.offset(index) as u8;
                values[i] = ivalue;
            }

            image.unsafe_put_pixel(x, y, color_space.convert_to_rgba(values))
        }
    }

    ffi::opj_destroy_codec(jp2_codec);
    ffi::opj_image_destroy(jp2_image);

    Ok(image)
}

pub fn from_memory(
    buf: &[u8],
    codec: Codec,
    config: DecodeConfig,
    logger: Option<Logger>,
) -> Result<DynamicImage, DecodeError> {
    // TODO: In the future this should not copy the data into a vec but instead take a slice and
    // store a slice in the NdUserdata with appropriate lifetime information.
    let mut userdata = support::NdUserdata::new_input(buf);

    let logger = logger.unwrap_or_else(|| Logger::root(slog::Discard, o!()));

    unsafe {
        let stream = ffi::opj_stream_default_create(1);
        ffi::opj_stream_set_read_function(stream, Some(support::nd_opj_stream_read_fn));
        ffi::opj_stream_set_write_function(stream, Some(support::nd_opj_stream_write_fn));
        ffi::opj_stream_set_skip_function(stream, Some(support::nd_opj_stream_skip_fn));
        ffi::opj_stream_set_seek_function(stream, Some(support::nd_opj_stream_seek_fn));

        let userdata_ptr: *mut support::NdUserdata = &mut userdata;
        ffi::opj_stream_set_user_data_length(stream, buf.len() as u64);
        ffi::opj_stream_set_user_data(stream, userdata_ptr as *mut c_void, None);
        load_from_stream(stream, codec, config, logger)
    }
}

// TODO: docs
pub fn from_file<S: Into<String>>(
    file_name: S,
    codec: Codec,
    config: DecodeConfig,
    logger: Option<Logger>,
) -> Result<DynamicImage, DecodeError> {
    let logger = logger.unwrap_or_else(|| Logger::root(slog::Discard, o!()));

    unsafe {
        let f = CString::new(file_name.into())?;
        let jp2_stream = ffi::opj_stream_create_default_file_stream(f.as_ptr(), 1);
        load_from_stream(jp2_stream, codec, config, logger)
    }
}
