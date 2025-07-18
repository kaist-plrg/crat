use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
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
unsafe extern "C" fn hash(
    mut ctx: *mut libc::c_void,
    mut update: Option::<nettle_hash_update_func>,
    mut digest: Option::<nettle_hash_digest_func>,
    mut digest_size: size_t,
    mut cnt: uint64_t,
    mut a_len: size_t,
    mut a: *const uint8_t,
    mut b_len: size_t,
    mut b: *const uint8_t,
    mut dst: *mut uint8_t,
) {
    let mut tmp: [uint8_t; 8] = [0; 8];
    tmp[7 as libc::c_int
        as usize] = (cnt >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    tmp[6 as libc::c_int
        as usize] = (cnt >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    tmp[5 as libc::c_int
        as usize] = (cnt >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    tmp[4 as libc::c_int
        as usize] = (cnt >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    tmp[3 as libc::c_int
        as usize] = (cnt >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    tmp[2 as libc::c_int
        as usize] = (cnt >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    tmp[1 as libc::c_int
        as usize] = (cnt >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    tmp[0 as libc::c_int
        as usize] = (cnt & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    update
        .unwrap()(
        ctx,
        ::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong,
        tmp.as_mut_ptr(),
    );
    if !a.is_null() && a_len != 0 {
        update.unwrap()(ctx, a_len, a);
    }
    if !b.is_null() && b_len != 0 {
        update.unwrap()(ctx, b_len, b);
    }
    digest.unwrap()(ctx, digest_size, dst);
}
unsafe extern "C" fn hash_ints(
    mut ctx: *mut libc::c_void,
    mut update: Option::<nettle_hash_update_func>,
    mut digest: Option::<nettle_hash_digest_func>,
    mut digest_size: size_t,
    mut i: uint64_t,
    mut j: uint64_t,
    mut k: uint64_t,
    mut dst: *mut uint8_t,
) {
    let mut tmp: [uint8_t; 24] = [0; 24];
    tmp[7 as libc::c_int
        as usize] = (i >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    tmp[6 as libc::c_int
        as usize] = (i >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    tmp[5 as libc::c_int
        as usize] = (i >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    tmp[4 as libc::c_int
        as usize] = (i >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    tmp[3 as libc::c_int
        as usize] = (i >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    tmp[2 as libc::c_int
        as usize] = (i >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    tmp[1 as libc::c_int
        as usize] = (i >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    tmp[0 as libc::c_int
        as usize] = (i & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *tmp
        .as_mut_ptr()
        .offset(8 as libc::c_int as isize)
        .offset(
            7 as libc::c_int as isize,
        ) = (j >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *tmp
        .as_mut_ptr()
        .offset(8 as libc::c_int as isize)
        .offset(
            6 as libc::c_int as isize,
        ) = (j >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *tmp
        .as_mut_ptr()
        .offset(8 as libc::c_int as isize)
        .offset(
            5 as libc::c_int as isize,
        ) = (j >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *tmp
        .as_mut_ptr()
        .offset(8 as libc::c_int as isize)
        .offset(
            4 as libc::c_int as isize,
        ) = (j >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *tmp
        .as_mut_ptr()
        .offset(8 as libc::c_int as isize)
        .offset(
            3 as libc::c_int as isize,
        ) = (j >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *tmp
        .as_mut_ptr()
        .offset(8 as libc::c_int as isize)
        .offset(
            2 as libc::c_int as isize,
        ) = (j >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *tmp
        .as_mut_ptr()
        .offset(8 as libc::c_int as isize)
        .offset(
            1 as libc::c_int as isize,
        ) = (j >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *tmp
        .as_mut_ptr()
        .offset(8 as libc::c_int as isize)
        .offset(
            0 as libc::c_int as isize,
        ) = (j & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *tmp
        .as_mut_ptr()
        .offset(16 as libc::c_int as isize)
        .offset(
            7 as libc::c_int as isize,
        ) = (k >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *tmp
        .as_mut_ptr()
        .offset(16 as libc::c_int as isize)
        .offset(
            6 as libc::c_int as isize,
        ) = (k >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *tmp
        .as_mut_ptr()
        .offset(16 as libc::c_int as isize)
        .offset(
            5 as libc::c_int as isize,
        ) = (k >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *tmp
        .as_mut_ptr()
        .offset(16 as libc::c_int as isize)
        .offset(
            4 as libc::c_int as isize,
        ) = (k >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *tmp
        .as_mut_ptr()
        .offset(16 as libc::c_int as isize)
        .offset(
            3 as libc::c_int as isize,
        ) = (k >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *tmp
        .as_mut_ptr()
        .offset(16 as libc::c_int as isize)
        .offset(
            2 as libc::c_int as isize,
        ) = (k >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *tmp
        .as_mut_ptr()
        .offset(16 as libc::c_int as isize)
        .offset(
            1 as libc::c_int as isize,
        ) = (k >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    *tmp
        .as_mut_ptr()
        .offset(16 as libc::c_int as isize)
        .offset(
            0 as libc::c_int as isize,
        ) = (k & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    update
        .unwrap()(
        ctx,
        ::std::mem::size_of::<[uint8_t; 24]>() as libc::c_ulong,
        tmp.as_mut_ptr(),
    );
    digest.unwrap()(ctx, digest_size, dst);
}
unsafe extern "C" fn block_to_int(
    mut length: size_t,
    mut block: *const uint8_t,
    mut mod_0: size_t,
) -> size_t {
    let mut i: size_t = length;
    let mut r: size_t = 0 as libc::c_int as size_t;
    loop {
        let fresh0 = i;
        i = i.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        r = (r << 8 as libc::c_int)
            .wrapping_add(*block.offset(i as isize) as libc::c_ulong);
        r = (r as libc::c_ulong).wrapping_rem(mod_0) as size_t as size_t;
    }
    return r;
}
pub unsafe extern "C" fn nettle_balloon(
    mut hash_ctx: *mut libc::c_void,
    mut update: Option::<nettle_hash_update_func>,
    mut digest: Option::<nettle_hash_digest_func>,
    mut digest_size: size_t,
    mut s_cost: size_t,
    mut t_cost: size_t,
    mut passwd_length: size_t,
    mut passwd: *const uint8_t,
    mut salt_length: size_t,
    mut salt: *const uint8_t,
    mut scratch: *mut uint8_t,
    mut dst: *mut uint8_t,
) {
    let BS: size_t = digest_size;
    let mut block: *mut uint8_t = scratch;
    let mut buf: *mut uint8_t = scratch.offset(BS as isize);
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut cnt: size_t = 0 as libc::c_int as size_t;
    let fresh1 = cnt;
    cnt = cnt.wrapping_add(1);
    hash(
        hash_ctx,
        update,
        digest,
        digest_size,
        fresh1,
        passwd_length,
        passwd,
        salt_length,
        salt,
        buf,
    );
    i = 1 as libc::c_int as size_t;
    while i < s_cost {
        let fresh2 = cnt;
        cnt = cnt.wrapping_add(1);
        hash(
            hash_ctx,
            update,
            digest,
            digest_size,
            fresh2,
            BS,
            buf
                .offset(
                    i.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(BS)
                        as isize,
                ),
            0 as libc::c_int as size_t,
            0 as *const uint8_t,
            buf.offset(i.wrapping_mul(BS) as isize),
        );
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < t_cost {
        j = 0 as libc::c_int as size_t;
        while j < s_cost {
            let fresh3 = cnt;
            cnt = cnt.wrapping_add(1);
            hash(
                hash_ctx,
                update,
                digest,
                digest_size,
                fresh3,
                BS,
                buf
                    .offset(
                        (if j != 0 {
                            j.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            s_cost.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_mul(BS) as isize,
                    ),
                BS,
                buf.offset(j.wrapping_mul(BS) as isize),
                buf.offset(j.wrapping_mul(BS) as isize),
            );
            k = 0 as libc::c_int as size_t;
            while k < 3 as libc::c_int as libc::c_ulong {
                hash_ints(hash_ctx, update, digest, digest_size, i, j, k, block);
                let fresh4 = cnt;
                cnt = cnt.wrapping_add(1);
                hash(
                    hash_ctx,
                    update,
                    digest,
                    digest_size,
                    fresh4,
                    salt_length,
                    salt,
                    BS,
                    block,
                    block,
                );
                let fresh5 = cnt;
                cnt = cnt.wrapping_add(1);
                hash(
                    hash_ctx,
                    update,
                    digest,
                    digest_size,
                    fresh5,
                    BS,
                    buf.offset(j.wrapping_mul(BS) as isize),
                    BS,
                    buf
                        .offset(
                            (block_to_int(BS, block, s_cost)).wrapping_mul(BS) as isize,
                        ),
                    buf.offset(j.wrapping_mul(BS) as isize),
                );
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    memcpy(
        dst as *mut libc::c_void,
        buf
            .offset(
                s_cost.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(BS)
                    as isize,
            ) as *const libc::c_void,
        BS,
    );
}
pub unsafe extern "C" fn nettle_balloon_itch(
    mut digest_size: size_t,
    mut s_cost: size_t,
) -> size_t {
    return s_cost
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(digest_size);
}
