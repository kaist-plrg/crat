use ::libc;
static mut version: [libc::c_char; 5] = unsafe {
    *::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"4.16\0")
};
pub unsafe extern "C" fn proxychains_get_version() -> *const libc::c_char {
    return version.as_ptr();
}
