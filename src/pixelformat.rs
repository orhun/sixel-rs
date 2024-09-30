pub use sixel_sys::PixelFormat;
use std::os::raw::c_int;

pub trait PixelFormatChan {
    fn channels_per_pixel(self) -> c_int;
}

impl PixelFormatChan for PixelFormat {
    fn channels_per_pixel(self) -> c_int {
        unsafe { sixel_sys::sixel_helper_compute_depth(self) }
    }
}

// Piston Image library used for reference for this bit. Thanks
pub trait Pixel {
    fn num_channels() -> u8;

    fn channels(&self) -> &[u8];

    fn channels_mut(&mut self) -> &mut [u8];

    fn from_slice(_: &[u8]) -> &Self;

    fn from_slice_mut(_: &mut [u8]) -> &mut Self;
}

macro_rules! define_colors {
    {$(
            $ident: ident,
            $channels: expr;
      )*} => {
        $(
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
            #[repr(C)]
            pub struct $ident {
                pub data: [u8; $channels]
            }

            impl Pixel for $ident {
                fn num_channels() -> u8 {
                    $channels
                }

                fn channels(&self) -> &[u8] {
                    &self.data
                }

                fn channels_mut(&mut self) -> &mut [u8] {
                    &mut self.data
                }

                fn from_slice(slice: &[u8]) -> &$ident {
                    assert_eq!(slice.len(), $channels);
                    unsafe {
                        &mut *(slice.as_ptr() as *mut $ident)
                    }
                }

                fn from_slice_mut<'a>(slice: &'a mut [u8]) -> &'a mut $ident {
                    assert_eq!(slice.len(), $channels);
                    unsafe {
                        &mut *(slice.as_ptr() as *mut $ident)
                    }
                }

            }
         )*
    }
}

define_colors! {
    Color1, 1;
    Color2, 2;
    Color3, 3;
    Color4, 4;
}
