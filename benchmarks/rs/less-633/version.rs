use ::libc;
pub static mut version: [libc::c_char; 4] = unsafe {
    *::std::mem::transmute::<&[u8; 4], &mut [libc::c_char; 4]>(b"633\0")
};
