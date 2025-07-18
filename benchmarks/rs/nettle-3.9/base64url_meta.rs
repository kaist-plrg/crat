use ::libc;
extern "C" {
    fn nettle_base64url_encode_init(ctx: *mut base64_encode_ctx);
    fn nettle_base64_encode_update(
        ctx: *mut base64_encode_ctx,
        dst: *mut libc::c_char,
        length: size_t,
        src: *const uint8_t,
    ) -> size_t;
    fn nettle_base64_encode_final(
        ctx: *mut base64_encode_ctx,
        dst: *mut libc::c_char,
    ) -> size_t;
    fn nettle_base64url_decode_init(ctx: *mut base64_decode_ctx);
    fn nettle_base64_decode_update(
        ctx: *mut base64_decode_ctx,
        dst_length: *mut size_t,
        dst: *mut uint8_t,
        src_length: size_t,
        src: *const libc::c_char,
    ) -> libc::c_int;
    fn nettle_base64_decode_final(ctx: *mut base64_decode_ctx) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type nettle_armor_length_func = unsafe extern "C" fn(size_t) -> size_t;
pub type nettle_armor_init_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type nettle_armor_encode_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_char,
    size_t,
    *const uint8_t,
) -> size_t;
pub type nettle_armor_encode_final_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_char,
) -> size_t;
pub type nettle_armor_decode_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut size_t,
    *mut uint8_t,
    size_t,
    *const libc::c_char,
) -> libc::c_int;
pub type nettle_armor_decode_final_func = unsafe extern "C" fn(
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_armor {
    pub name: *const libc::c_char,
    pub encode_context_size: libc::c_uint,
    pub decode_context_size: libc::c_uint,
    pub encode_final_length: libc::c_uint,
    pub encode_init: Option::<nettle_armor_init_func>,
    pub encode_length: Option::<nettle_armor_length_func>,
    pub encode_update: Option::<nettle_armor_encode_update_func>,
    pub encode_final: Option::<nettle_armor_encode_final_func>,
    pub decode_init: Option::<nettle_armor_init_func>,
    pub decode_length: Option::<nettle_armor_length_func>,
    pub decode_update: Option::<nettle_armor_decode_update_func>,
    pub decode_final: Option::<nettle_armor_decode_final_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_decode_ctx {
    pub table: *const libc::c_schar,
    pub word: libc::c_ushort,
    pub bits: libc::c_uchar,
    pub padding: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_encode_ctx {
    pub alphabet: *const libc::c_char,
    pub word: libc::c_ushort,
    pub bits: libc::c_uchar,
}
unsafe extern "C" fn base64url_encode_length(mut length: size_t) -> size_t {
    return length
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_add(4 as libc::c_int as libc::c_ulong)
        .wrapping_div(6 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn base64url_decode_length(mut length: size_t) -> size_t {
    return length
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(6 as libc::c_int as libc::c_ulong)
        .wrapping_div(8 as libc::c_int as libc::c_ulong);
}
pub static mut nettle_base64url: nettle_armor = unsafe {
    {
        let mut init = nettle_armor {
            name: b"base64url\0" as *const u8 as *const libc::c_char,
            encode_context_size: ::std::mem::size_of::<base64_encode_ctx>()
                as libc::c_ulong as libc::c_uint,
            decode_context_size: ::std::mem::size_of::<base64_decode_ctx>()
                as libc::c_ulong as libc::c_uint,
            encode_final_length: 3 as libc::c_int as libc::c_uint,
            encode_init: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut base64_encode_ctx) -> ()>,
                Option::<nettle_armor_init_func>,
            >(
                Some(
                    nettle_base64url_encode_init
                        as unsafe extern "C" fn(*mut base64_encode_ctx) -> (),
                ),
            ),
            encode_length: Some(base64url_encode_length as nettle_armor_length_func),
            encode_update: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut base64_encode_ctx,
                        *mut libc::c_char,
                        size_t,
                        *const uint8_t,
                    ) -> size_t,
                >,
                Option::<nettle_armor_encode_update_func>,
            >(
                Some(
                    nettle_base64_encode_update
                        as unsafe extern "C" fn(
                            *mut base64_encode_ctx,
                            *mut libc::c_char,
                            size_t,
                            *const uint8_t,
                        ) -> size_t,
                ),
            ),
            encode_final: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut base64_encode_ctx,
                        *mut libc::c_char,
                    ) -> size_t,
                >,
                Option::<nettle_armor_encode_final_func>,
            >(
                Some(
                    nettle_base64_encode_final
                        as unsafe extern "C" fn(
                            *mut base64_encode_ctx,
                            *mut libc::c_char,
                        ) -> size_t,
                ),
            ),
            decode_init: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut base64_decode_ctx) -> ()>,
                Option::<nettle_armor_init_func>,
            >(
                Some(
                    nettle_base64url_decode_init
                        as unsafe extern "C" fn(*mut base64_decode_ctx) -> (),
                ),
            ),
            decode_length: Some(base64url_decode_length as nettle_armor_length_func),
            decode_update: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut base64_decode_ctx,
                        *mut size_t,
                        *mut uint8_t,
                        size_t,
                        *const libc::c_char,
                    ) -> libc::c_int,
                >,
                Option::<nettle_armor_decode_update_func>,
            >(
                Some(
                    nettle_base64_decode_update
                        as unsafe extern "C" fn(
                            *mut base64_decode_ctx,
                            *mut size_t,
                            *mut uint8_t,
                            size_t,
                            *const libc::c_char,
                        ) -> libc::c_int,
                ),
            ),
            decode_final: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut base64_decode_ctx) -> libc::c_int>,
                Option::<nettle_armor_decode_final_func>,
            >(
                Some(
                    nettle_base64_decode_final
                        as unsafe extern "C" fn(*mut base64_decode_ctx) -> libc::c_int,
                ),
            ),
        };
        init
    }
};
