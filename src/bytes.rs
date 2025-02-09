// Taken from https://github.com/benjajaja/sixel-bytes/blob/master/src/lib.rs

use std::{
    ffi::{c_uchar, c_void},
    mem, ptr, slice,
};

use sixel_sys::{
    sixel_dither_destroy, sixel_dither_initialize, sixel_dither_new,
    sixel_dither_set_diffusion_type, sixel_dither_set_pixelformat, sixel_encode,
    sixel_output_destroy, sixel_output_new, sixel_output_set_encode_policy, status as sixel_status,
    Dither, EncodePolicy, MethodForLargest, Output,
};

use crate::{
    optflags::DiffusionMethod,
    pixelformat::PixelFormat,
    status::{self, Error, Status},
};

// According to sixel-sys, this is unused/ignored.
const DEPTH_ALWAYS_IGNORED: i32 = 24;

/// Encode image bytes to a [String] containing the sixel data.
///
/// The `bytes` must match the width, height, and "pixelformat".
pub fn sixel_string(
    bytes: &[u8],
    width: i32,
    height: i32,
    pixelformat: PixelFormat,
    method_for_diffuse: DiffusionMethod,
) -> Status<String> {
    let mut sixel_data: Vec<::std::os::raw::c_char> = Vec::new();
    let sixel_data_ptr: *mut c_void = &mut sixel_data as *mut _ as *mut c_void;

    let mut output: *mut Output = ptr::null_mut() as *mut _;
    let output_ptr: *mut *mut Output = &mut output as *mut _;

    let mut dither: *mut Dither = ptr::null_mut() as *mut _;
    let dither_ptr: *mut *mut Dither = &mut dither as *mut _;

    let pixels = bytes.as_ptr() as *mut c_uchar;

    unsafe extern "C" fn callback(
        data: *mut ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
        priv_: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int {
        let sixel_data: &mut Vec<::std::os::raw::c_char> =
            &mut *(priv_ as *mut Vec<::std::os::raw::c_char>);

        let data_slice: &mut [::std::os::raw::c_char] =
            slice::from_raw_parts_mut(if data.is_null() { return 1 } else { data }, size as usize);
        sixel_data.append(&mut data_slice.to_vec());
        sixel_status::OK
    }

    unsafe {
        status::from_libsixel(sixel_output_new(
            output_ptr,
            Some(callback),
            sixel_data_ptr,
            ptr::null_mut(),
        ))?;

        sixel_output_set_encode_policy(output, EncodePolicy::Auto);

        status::from_libsixel(sixel_dither_new(dither_ptr, 256, ptr::null_mut()))?;

        status::from_libsixel(sixel_dither_initialize(
            dither,
            pixels,
            width,
            height,
            pixelformat,
            MethodForLargest::Auto,
            sixel_sys::MethodForRepColor::Auto,
            sixel_sys::QualityMode::Auto,
        ))?;
        sixel_dither_set_pixelformat(dither, pixelformat);
        sixel_dither_set_diffusion_type(dither, method_for_diffuse.to_sixel_diffusion_method());

        status::from_libsixel(sixel_encode(
            pixels,
            width,
            height,
            DEPTH_ALWAYS_IGNORED,
            dither,
            output,
        ))?;

        sixel_output_destroy(output);
        sixel_dither_destroy(dither);

        // TODO: should we just return something like [u8]? Is all sixel data valid utf8?
        String::from_utf8(mem::transmute(sixel_data)).map_err(|_| Error::Utf8)
    }
}
