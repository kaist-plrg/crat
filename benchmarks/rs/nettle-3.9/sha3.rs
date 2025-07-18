use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn nettle_sha3_permute(state: *mut sha3_state);
    fn nettle_memxor(
        dst: *mut libc::c_void,
        src: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha3_state {
    pub a: [uint64_t; 25],
}
unsafe extern "C" fn sha3_absorb(
    mut state: *mut sha3_state,
    mut length: libc::c_uint,
    mut data: *const uint8_t,
) {
    if length & 7 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"(length & 7) == 0\0" as *const u8 as *const libc::c_char,
            b"sha3.c\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"void sha3_absorb(struct sha3_state *, unsigned int, const uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_665: {
        if length & 7 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"(length & 7) == 0\0" as *const u8 as *const libc::c_char,
                b"sha3.c\0" as *const u8 as *const libc::c_char,
                50 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"void sha3_absorb(struct sha3_state *, unsigned int, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    nettle_memxor(
        ((*state).a).as_mut_ptr() as *mut libc::c_void,
        data as *const libc::c_void,
        length as size_t,
    );
    nettle_sha3_permute(state);
}
pub unsafe extern "C" fn _nettle_sha3_update(
    mut state: *mut sha3_state,
    mut block_size: libc::c_uint,
    mut block: *mut uint8_t,
    mut pos: libc::c_uint,
    mut length: size_t,
    mut data: *const uint8_t,
) -> libc::c_uint {
    if pos > 0 as libc::c_int as libc::c_uint {
        let mut left: libc::c_uint = block_size.wrapping_sub(pos);
        if length < left as libc::c_ulong {
            memcpy(
                block.offset(pos as isize) as *mut libc::c_void,
                data as *const libc::c_void,
                length,
            );
            return (pos as libc::c_ulong).wrapping_add(length) as libc::c_uint;
        } else {
            memcpy(
                block.offset(pos as isize) as *mut libc::c_void,
                data as *const libc::c_void,
                left as libc::c_ulong,
            );
            data = data.offset(left as isize);
            length = (length as libc::c_ulong).wrapping_sub(left as libc::c_ulong)
                as size_t as size_t;
            sha3_absorb(state, block_size, block);
        }
    }
    while length >= block_size as libc::c_ulong {
        sha3_absorb(state, block_size, data);
        length = (length as libc::c_ulong).wrapping_sub(block_size as libc::c_ulong)
            as size_t as size_t;
        data = data.offset(block_size as isize);
    }
    memcpy(block as *mut libc::c_void, data as *const libc::c_void, length);
    return length as libc::c_uint;
}
pub unsafe extern "C" fn _nettle_sha3_pad(
    mut state: *mut sha3_state,
    mut block_size: libc::c_uint,
    mut block: *mut uint8_t,
    mut pos: libc::c_uint,
    mut magic: uint8_t,
) {
    if pos < block_size {} else {
        __assert_fail(
            b"pos < block_size\0" as *const u8 as *const libc::c_char,
            b"sha3.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 91],
                &[libc::c_char; 91],
            >(
                b"void _nettle_sha3_pad(struct sha3_state *, unsigned int, uint8_t *, unsigned int, uint8_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_871: {
        if pos < block_size {} else {
            __assert_fail(
                b"pos < block_size\0" as *const u8 as *const libc::c_char,
                b"sha3.c\0" as *const u8 as *const libc::c_char,
                97 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 91],
                    &[libc::c_char; 91],
                >(
                    b"void _nettle_sha3_pad(struct sha3_state *, unsigned int, uint8_t *, unsigned int, uint8_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let fresh0 = pos;
    pos = pos.wrapping_add(1);
    *block.offset(fresh0 as isize) = magic;
    memset(
        block.offset(pos as isize) as *mut libc::c_void,
        0 as libc::c_int,
        block_size.wrapping_sub(pos) as libc::c_ulong,
    );
    let ref mut fresh1 = *block
        .offset(block_size.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
    *fresh1 = (*fresh1 as libc::c_int | 0x80 as libc::c_int) as uint8_t;
    sha3_absorb(state, block_size, block);
}
