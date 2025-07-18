use ::libc;
pub static mut version_etc_copyright: [libc::c_char; 47] = unsafe {
    *::std::mem::transmute::<
        &[u8; 47],
        &[libc::c_char; 47],
    >(b"Copyright %s %d Free Software Foundation, Inc.\0")
};
