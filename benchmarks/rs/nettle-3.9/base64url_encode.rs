use ::libc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_encode_ctx {
    pub alphabet: *const libc::c_char,
    pub word: libc::c_ushort,
    pub bits: libc::c_uchar,
}
pub unsafe extern "C" fn nettle_base64url_encode_init(mut ctx: *mut base64_encode_ctx) {
    static mut base64url_encode_table: [libc::c_char; 64] = unsafe {
        *::std::mem::transmute::<
            &[u8; 64],
            &[libc::c_char; 64],
        >(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_")
    };
    (*ctx).bits = 0 as libc::c_int as libc::c_uchar;
    (*ctx).word = (*ctx).bits as libc::c_ushort;
    (*ctx).alphabet = base64url_encode_table.as_ptr();
}
