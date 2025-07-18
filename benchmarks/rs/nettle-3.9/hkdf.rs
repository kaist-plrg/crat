use ::libc;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type nettle_hash_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *const uint8_t,
) -> ();
pub type nettle_hash_digest_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
) -> ();
pub unsafe extern "C" fn nettle_hkdf_extract(
    mut mac_ctx: *mut libc::c_void,
    mut update: Option::<nettle_hash_update_func>,
    mut digest: Option::<nettle_hash_digest_func>,
    mut digest_size: size_t,
    mut secret_size: size_t,
    mut secret: *const uint8_t,
    mut dst: *mut uint8_t,
) {
    update.unwrap()(mac_ctx, secret_size, secret);
    digest.unwrap()(mac_ctx, digest_size, dst);
}
pub unsafe extern "C" fn nettle_hkdf_expand(
    mut mac_ctx: *mut libc::c_void,
    mut update: Option::<nettle_hash_update_func>,
    mut digest: Option::<nettle_hash_digest_func>,
    mut digest_size: size_t,
    mut info_size: size_t,
    mut info: *const uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
) {
    let mut i: uint8_t = 1 as libc::c_int as uint8_t;
    if length == 0 {
        return;
    }
    loop {
        update.unwrap()(mac_ctx, info_size, info);
        update.unwrap()(mac_ctx, 1 as libc::c_int as size_t, &mut i);
        if length <= digest_size {
            break;
        }
        digest.unwrap()(mac_ctx, digest_size, dst);
        update.unwrap()(mac_ctx, digest_size, dst);
        dst = dst.offset(digest_size as isize);
        length = (length as libc::c_ulong).wrapping_sub(digest_size) as size_t as size_t;
        i = i.wrapping_add(1);
        i;
    }
    digest.unwrap()(mac_ctx, length, dst);
}
