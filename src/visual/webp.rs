use crate::visual::generated::webp;
use crate::visual::generated::webp::size_t;
use std::os::raw::c_int;

pub struct WebP {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl WebP {
    pub fn load_rgb(src: &[u8]) -> Option<WebP> {
        let mut width: i32 = 0;
        let mut height: i32 = 0;

        let ret = unsafe {
            webp::WebPGetInfo(
                src.as_ptr(),
                src.len() as size_t,
                &mut width as *mut _,
                &mut height as *mut _,
            )
        };
        if ret == 0 {
            return None;
        }

        let mut data = vec![0; (width * height * 3) as usize];
        let ret = unsafe {
            webp::WebPDecodeRGBInto(
                src.as_ptr(),
                src.len() as size_t,
                data.as_mut_ptr(),
                data.len() as size_t,
                width as c_int * 3,
            )
        };
        if ret.is_null() {
            return None;
        }

        Some(WebP {
            data,
            width: width as u32,
            height: height as u32,
        })
    }
}
