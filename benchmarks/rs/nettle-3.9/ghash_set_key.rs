use ::libc;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [libc::c_ulong; 2],
    pub u64_0: [uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gcm_key {
    pub h: [nettle_block16; 256],
}
#[inline]
unsafe extern "C" fn block16_set(
    mut r: *mut nettle_block16,
    mut x: *const nettle_block16,
) {
    (*r).u64_0[0 as libc::c_int as usize] = (*x).u64_0[0 as libc::c_int as usize];
    (*r).u64_0[1 as libc::c_int as usize] = (*x).u64_0[1 as libc::c_int as usize];
}
#[inline]
unsafe extern "C" fn block16_mulx_ghash(
    mut r: *mut nettle_block16,
    mut x: *const nettle_block16,
) {
    let mut mask: uint64_t = 0;
    mask = ((*x).u64_0[1 as libc::c_int as usize] >> 56 as libc::c_int
        & 1 as libc::c_int as libc::c_ulong)
        .wrapping_neg();
    (*r)
        .u64_0[1 as libc::c_int
        as usize] = ((*x).u64_0[1 as libc::c_int as usize]
        & 0xfefefefefefefefe as libc::c_ulong) >> 1 as libc::c_int
        | ((*x).u64_0[1 as libc::c_int as usize] & 0x1010101010101 as libc::c_ulong)
            << 15 as libc::c_int
        | (*x).u64_0[0 as libc::c_int as usize] >> 49 as libc::c_int
            & 0x80 as libc::c_int as libc::c_ulong;
    (*r)
        .u64_0[0 as libc::c_int
        as usize] = (((*x).u64_0[0 as libc::c_int as usize]
        & 0xfefefefefefefefe as libc::c_ulong) >> 1 as libc::c_int
        | ((*x).u64_0[0 as libc::c_int as usize] & 0x1010101010101 as libc::c_ulong)
            << 15 as libc::c_int) ^ mask & 0xe1 as libc::c_ulong;
}
pub unsafe extern "C" fn _nettle_ghash_set_key_c(
    mut ctx: *mut gcm_key,
    mut key: *const nettle_block16,
) {
    let mut i: libc::c_uint = 0;
    block16_set(
        &mut *((*ctx).h)
            .as_mut_ptr()
            .offset((2 as libc::c_int * 7 as libc::c_int) as isize),
        key,
    );
    i = 1 as libc::c_int as libc::c_uint;
    while i < 64 as libc::c_int as libc::c_uint {
        block16_mulx_ghash(
            &mut *((*ctx).h)
                .as_mut_ptr()
                .offset(
                    (2 as libc::c_int as libc::c_uint)
                        .wrapping_mul(i ^ 7 as libc::c_int as libc::c_uint) as isize,
                ),
            &mut *((*ctx).h)
                .as_mut_ptr()
                .offset(
                    (2 as libc::c_int as libc::c_uint)
                        .wrapping_mul(
                            i.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                ^ 7 as libc::c_int as libc::c_uint,
                        ) as isize,
                ),
        );
        i = i.wrapping_add(1);
        i;
    }
    block16_mulx_ghash(
        &mut *((*ctx).h)
            .as_mut_ptr()
            .offset((2 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int) as isize),
        &mut *((*ctx).h)
            .as_mut_ptr()
            .offset((2 as libc::c_int * (63 as libc::c_int ^ 7 as libc::c_int)) as isize),
    );
    i = 1 as libc::c_int as libc::c_uint;
    while i < 64 as libc::c_int as libc::c_uint {
        block16_mulx_ghash(
            &mut *((*ctx).h)
                .as_mut_ptr()
                .offset(
                    (2 as libc::c_int as libc::c_uint)
                        .wrapping_mul(i ^ 7 as libc::c_int as libc::c_uint)
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                ),
            &mut *((*ctx).h)
                .as_mut_ptr()
                .offset(
                    (2 as libc::c_int as libc::c_uint)
                        .wrapping_mul(
                            i.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                ^ 7 as libc::c_int as libc::c_uint,
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                ),
        );
        i = i.wrapping_add(1);
        i;
    }
}
