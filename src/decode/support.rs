/// jpeg2000: Rust bindings to the OpenJPEG library.
///
/// Copyright (C) 2010 Linden Research, Inc.
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

use std::{mem, slice};
use std::os::raw::c_void;

struct NdUserdata {
    input_stream: bool,
    offset: usize,
    length: usize,
    output: Vec<u8>,
    input: Vec<u8>
}

unsafe extern "C"
fn nd_opj_stream_read_fn (p_buffer: *mut c_void,
                       p_nb_bytes: usize,
                       p_user_data: *mut c_void) -> usize
{
    let userdata = p_user_data as *mut NdUserdata;
    assert!((*userdata).input_stream);

    let n_imgsize = (*userdata).input.len();
    let n_byteleft = n_imgsize - (*userdata).offset;

    let mut n_read = p_nb_bytes;

    if n_read > n_byteleft {
        n_read = n_byteleft;
    }

    if (*userdata).input.is_empty() || p_buffer.is_null() || n_read == 0 || n_byteleft == 0 {
        // TODO: The original returned -1 here,
        // but for some reason our signature is usize...
        return 0;
    }

    let target = slice::from_raw_parts_mut(p_buffer as *mut u8, n_read);
    let offset = (*userdata).offset;
    target.copy_from_slice(&(*userdata).input[offset..offset+n_read]);

    (*userdata).offset += n_read;

    n_read
}

unsafe extern "C"
fn nd_opj_stream_write_fn(p_buffer: *mut c_void,
                          p_nb_bytes: usize,
                          p_user_data: *mut c_void) -> usize
{
    let userdata = p_user_data as *mut NdUserdata;
    assert!(! (*userdata).input_stream);

    let buffer = p_buffer as *mut u8;

    (*userdata).output.reserve((*userdata).output.len() + p_nb_bytes);
    (*userdata).output.extend_from_slice(slice::from_raw_parts(buffer, p_nb_bytes));

    p_nb_bytes
}

unsafe extern "C"
fn nd_opj_stream_skip_fn(p_nb_bytes: i64, p_user_data: *mut c_void) -> i64
{
    let userdata = p_user_data as *mut NdUserdata;
    assert!((*userdata).input_stream);

    let n_imgsize = (*userdata).input.len();
    let n_byteleft = (n_imgsize - (*userdata).offset) as i64;

    let mut n_skip = p_nb_bytes;

    if n_skip > n_byteleft {
        n_skip = n_byteleft;
    }

    (*userdata).offset += (n_skip as usize);
    (*userdata).offset as i64
}

unsafe extern "C"
fn nd_opj_stream_seek_fn(p_nb_bytes: i64, p_user_data: *mut c_void) -> bool
{
    let userdata = p_user_data as *mut NdUserdata;
    assert!((*userdata).input_stream);

    let n_imgsize = (*userdata).input.len();
    let n_seek = p_nb_bytes as usize;
    
    if n_seek > n_imgsize {
        false 
    } else {
        (*userdata).offset = n_seek;
        true
    }
}

