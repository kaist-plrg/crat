use ::libc;
extern "C" {
    fn nettle_des_set_key(ctx: *mut des_ctx, key: *const uint8_t) -> libc::c_int;
    fn nettle_des_encrypt(
        ctx: *const des_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_des_decrypt(
        ctx: *const des_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct des_ctx {
    pub key: [uint32_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct des3_ctx {
    pub des: [des_ctx; 3],
}
pub unsafe extern "C" fn nettle_des3_set_key(
    mut ctx: *mut des3_ctx,
    mut key: *const uint8_t,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut is_good: libc::c_int = 1 as libc::c_int;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 3 as libc::c_int as libc::c_uint {
        if nettle_des_set_key(&mut *((*ctx).des).as_mut_ptr().offset(i as isize), key)
            == 0
        {
            is_good = 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
        key = key.offset(8 as libc::c_int as isize);
    }
    return is_good;
}
pub unsafe extern "C" fn nettle_des3_encrypt(
    mut ctx: *const des3_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    nettle_des_encrypt(
        &*((*ctx).des).as_ptr().offset(0 as libc::c_int as isize),
        length,
        dst,
        src,
    );
    nettle_des_decrypt(
        &*((*ctx).des).as_ptr().offset(1 as libc::c_int as isize),
        length,
        dst,
        dst,
    );
    nettle_des_encrypt(
        &*((*ctx).des).as_ptr().offset(2 as libc::c_int as isize),
        length,
        dst,
        dst,
    );
}
pub unsafe extern "C" fn nettle_des3_decrypt(
    mut ctx: *const des3_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    nettle_des_decrypt(
        &*((*ctx).des).as_ptr().offset(2 as libc::c_int as isize),
        length,
        dst,
        src,
    );
    nettle_des_encrypt(
        &*((*ctx).des).as_ptr().offset(1 as libc::c_int as isize),
        length,
        dst,
        dst,
    );
    nettle_des_decrypt(
        &*((*ctx).des).as_ptr().offset(0 as libc::c_int as isize),
        length,
        dst,
        dst,
    );
}
