use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn nettle_aes128_set_encrypt_key(ctx: *mut aes128_ctx, key: *const uint8_t);
    fn nettle_aes128_encrypt(
        ctx: *const aes128_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn _nettle_umac_l2_init(size: libc::c_uint, k: *mut uint32_t);
    fn _nettle_umac_l3_init(size: libc::c_uint, k: *mut uint64_t);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes128_ctx {
    pub keys: [uint32_t; 44],
}
unsafe extern "C" fn umac_kdf(
    mut aes: *mut aes128_ctx,
    mut index: libc::c_uint,
    mut length: libc::c_uint,
    mut dst: *mut uint8_t,
) {
    let mut block: [uint8_t; 16] = [0; 16];
    let mut count: uint64_t = 0;
    block[0 as libc::c_int
        as usize] = (index as uint64_t >> 56 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    block[1 as libc::c_int
        as usize] = (index as uint64_t >> 48 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    block[2 as libc::c_int
        as usize] = (index as uint64_t >> 40 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    block[3 as libc::c_int
        as usize] = (index as uint64_t >> 32 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    block[4 as libc::c_int
        as usize] = (index as uint64_t >> 24 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    block[5 as libc::c_int
        as usize] = (index as uint64_t >> 16 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    block[6 as libc::c_int
        as usize] = (index as uint64_t >> 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    block[7 as libc::c_int
        as usize] = (index as uint64_t & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    count = 1 as libc::c_int as uint64_t;
    while length >= 16 as libc::c_int as libc::c_uint {
        *block
            .as_mut_ptr()
            .offset(8 as libc::c_int as isize)
            .offset(
                0 as libc::c_int as isize,
            ) = (count >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        *block
            .as_mut_ptr()
            .offset(8 as libc::c_int as isize)
            .offset(
                1 as libc::c_int as isize,
            ) = (count >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        *block
            .as_mut_ptr()
            .offset(8 as libc::c_int as isize)
            .offset(
                2 as libc::c_int as isize,
            ) = (count >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        *block
            .as_mut_ptr()
            .offset(8 as libc::c_int as isize)
            .offset(
                3 as libc::c_int as isize,
            ) = (count >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        *block
            .as_mut_ptr()
            .offset(8 as libc::c_int as isize)
            .offset(
                4 as libc::c_int as isize,
            ) = (count >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        *block
            .as_mut_ptr()
            .offset(8 as libc::c_int as isize)
            .offset(
                5 as libc::c_int as isize,
            ) = (count >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        *block
            .as_mut_ptr()
            .offset(8 as libc::c_int as isize)
            .offset(
                6 as libc::c_int as isize,
            ) = (count >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        *block
            .as_mut_ptr()
            .offset(8 as libc::c_int as isize)
            .offset(
                7 as libc::c_int as isize,
            ) = (count & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
        nettle_aes128_encrypt(aes, 16 as libc::c_int as size_t, dst, block.as_mut_ptr());
        length = length.wrapping_sub(16 as libc::c_int as libc::c_uint);
        dst = dst.offset(16 as libc::c_int as isize);
        count = count.wrapping_add(1);
        count;
    }
    if length > 0 as libc::c_int as libc::c_uint {
        *block
            .as_mut_ptr()
            .offset(8 as libc::c_int as isize)
            .offset(
                0 as libc::c_int as isize,
            ) = (count >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        *block
            .as_mut_ptr()
            .offset(8 as libc::c_int as isize)
            .offset(
                1 as libc::c_int as isize,
            ) = (count >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        *block
            .as_mut_ptr()
            .offset(8 as libc::c_int as isize)
            .offset(
                2 as libc::c_int as isize,
            ) = (count >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        *block
            .as_mut_ptr()
            .offset(8 as libc::c_int as isize)
            .offset(
                3 as libc::c_int as isize,
            ) = (count >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        *block
            .as_mut_ptr()
            .offset(8 as libc::c_int as isize)
            .offset(
                4 as libc::c_int as isize,
            ) = (count >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        *block
            .as_mut_ptr()
            .offset(8 as libc::c_int as isize)
            .offset(
                5 as libc::c_int as isize,
            ) = (count >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        *block
            .as_mut_ptr()
            .offset(8 as libc::c_int as isize)
            .offset(
                6 as libc::c_int as isize,
            ) = (count >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uint8_t;
        *block
            .as_mut_ptr()
            .offset(8 as libc::c_int as isize)
            .offset(
                7 as libc::c_int as isize,
            ) = (count & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
        nettle_aes128_encrypt(
            aes,
            16 as libc::c_int as size_t,
            block.as_mut_ptr(),
            block.as_mut_ptr(),
        );
        memcpy(
            dst as *mut libc::c_void,
            block.as_mut_ptr() as *const libc::c_void,
            length as libc::c_ulong,
        );
    }
}
pub unsafe extern "C" fn _nettle_umac_set_key(
    mut l1_key: *mut uint32_t,
    mut l2_key: *mut uint32_t,
    mut l3_key1: *mut uint64_t,
    mut l3_key2: *mut uint32_t,
    mut aes: *mut aes128_ctx,
    mut key: *const uint8_t,
    mut n: libc::c_uint,
) {
    let mut size: libc::c_uint = 0;
    let mut buffer: [uint8_t; 16] = [0; 16];
    nettle_aes128_set_encrypt_key(aes, key);
    size = ((1024 as libc::c_int / 4 as libc::c_int) as libc::c_uint)
        .wrapping_add(
            (4 as libc::c_int as libc::c_uint)
                .wrapping_mul(n.wrapping_sub(1 as libc::c_int as libc::c_uint)),
        );
    umac_kdf(
        aes,
        1 as libc::c_int as libc::c_uint,
        (size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
            as libc::c_uint,
        l1_key as *mut uint8_t,
    );
    let mut be_i: libc::c_uint = 0;
    be_i = 0 as libc::c_int as libc::c_uint;
    while be_i < size {
        let mut be_x: uint32_t = *l1_key.offset(be_i as isize);
        *l1_key
            .offset(
                be_i as isize,
            ) = ((be_x << 8 as libc::c_int
            | be_x >> (-(8 as libc::c_int) & 31 as libc::c_int)) as libc::c_ulong
            & 0xff00ff as libc::c_ulong
            | (be_x << 24 as libc::c_int
                | be_x >> (-(24 as libc::c_int) & 31 as libc::c_int)) as libc::c_ulong
                & 0xff00ff00 as libc::c_ulong) as uint32_t;
        be_i = be_i.wrapping_add(1);
        be_i;
    }
    size = (6 as libc::c_int as libc::c_uint).wrapping_mul(n);
    umac_kdf(
        aes,
        2 as libc::c_int as libc::c_uint,
        (size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
            as libc::c_uint,
        l2_key as *mut uint8_t,
    );
    _nettle_umac_l2_init(size, l2_key);
    size = (8 as libc::c_int as libc::c_uint).wrapping_mul(n);
    umac_kdf(
        aes,
        3 as libc::c_int as libc::c_uint,
        (size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong)
            as libc::c_uint,
        l3_key1 as *mut uint8_t,
    );
    _nettle_umac_l3_init(size, l3_key1);
    umac_kdf(
        aes,
        4 as libc::c_int as libc::c_uint,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
            as libc::c_uint,
        l3_key2 as *mut uint8_t,
    );
    umac_kdf(
        aes,
        0 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        buffer.as_mut_ptr(),
    );
    nettle_aes128_set_encrypt_key(aes, buffer.as_mut_ptr());
}
