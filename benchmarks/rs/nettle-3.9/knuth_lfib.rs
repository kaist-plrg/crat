use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn abort() -> !;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct knuth_lfib_ctx {
    pub x: [uint32_t; 100],
    pub index: libc::c_uint,
}
pub unsafe extern "C" fn nettle_knuth_lfib_init(
    mut ctx: *mut knuth_lfib_ctx,
    mut seed: uint32_t,
) {
    let mut t: uint32_t = 0;
    let mut j: uint32_t = 0;
    let mut x: [uint32_t; 199] = [0; 199];
    let mut ss: uint32_t = (seed.wrapping_add(2 as libc::c_int as libc::c_uint)
        as libc::c_ulong
        & ((1 as libc::c_ulong) << 30 as libc::c_int)
            .wrapping_sub(2 as libc::c_int as libc::c_ulong)) as uint32_t;
    j = 0 as libc::c_int as uint32_t;
    while j < 100 as libc::c_int as libc::c_uint {
        x[j as usize] = ss;
        ss <<= 1 as libc::c_int;
        if ss as libc::c_ulong >= (1 as libc::c_ulong) << 30 as libc::c_int {
            ss = (ss as libc::c_ulong)
                .wrapping_sub(
                    ((1 as libc::c_ulong) << 30 as libc::c_int)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong),
                ) as uint32_t as uint32_t;
        }
        j = j.wrapping_add(1);
        j;
    }
    while j < (2 as libc::c_int * 100 as libc::c_int - 1 as libc::c_int) as libc::c_uint
    {
        x[j as usize] = 0 as libc::c_int as uint32_t;
        j = j.wrapping_add(1);
        j;
    }
    x[1 as libc::c_int as usize] = (x[1 as libc::c_int as usize]).wrapping_add(1);
    x[1 as libc::c_int as usize];
    ss = (seed as libc::c_ulong
        & ((1 as libc::c_ulong) << 30 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as uint32_t;
    t = (70 as libc::c_int - 1 as libc::c_int) as uint32_t;
    while t != 0 {
        j = (100 as libc::c_int - 1 as libc::c_int) as uint32_t;
        while j > 0 as libc::c_int as libc::c_uint {
            x[j.wrapping_add(j) as usize] = x[j as usize];
            j = j.wrapping_sub(1);
            j;
        }
        j = (2 as libc::c_int * 100 as libc::c_int - 2 as libc::c_int) as uint32_t;
        while j > (100 as libc::c_int - 37 as libc::c_int) as libc::c_uint {
            x[((2 as libc::c_int * 100 as libc::c_int - 1 as libc::c_int)
                as libc::c_uint)
                .wrapping_sub(j)
                as usize] = x[j as usize] & !(1 as libc::c_int) as libc::c_uint;
            j = (j as libc::c_uint).wrapping_sub(2 as libc::c_int as libc::c_uint)
                as uint32_t as uint32_t;
        }
        j = (2 as libc::c_int * 100 as libc::c_int - 2 as libc::c_int) as uint32_t;
        while j >= 100 as libc::c_int as libc::c_uint {
            if x[j as usize] & 1 as libc::c_int as libc::c_uint != 0 {
                x[j
                    .wrapping_sub(
                        (100 as libc::c_int - 37 as libc::c_int) as libc::c_uint,
                    )
                    as usize] = ((x[j
                    .wrapping_sub(
                        (100 as libc::c_int - 37 as libc::c_int) as libc::c_uint,
                    ) as usize])
                    .wrapping_sub(x[j as usize]) as libc::c_ulong
                    & ((1 as libc::c_ulong) << 30 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as uint32_t;
                x[j.wrapping_sub(100 as libc::c_int as libc::c_uint)
                    as usize] = ((x[j.wrapping_sub(100 as libc::c_int as libc::c_uint)
                    as usize])
                    .wrapping_sub(x[j as usize]) as libc::c_ulong
                    & ((1 as libc::c_ulong) << 30 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as uint32_t;
            }
            j = j.wrapping_sub(1);
            j;
        }
        if ss & 1 as libc::c_int as libc::c_uint != 0 {
            j = 100 as libc::c_int as uint32_t;
            while j > 0 as libc::c_int as libc::c_uint {
                x[j
                    as usize] = x[j.wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as usize];
                j = j.wrapping_sub(1);
                j;
            }
            x[0 as libc::c_int as usize] = x[100 as libc::c_int as usize];
            if x[100 as libc::c_int as usize] & 1 as libc::c_int as libc::c_uint != 0 {
                x[37 as libc::c_int
                    as usize] = ((x[37 as libc::c_int as usize])
                    .wrapping_sub(x[100 as libc::c_int as usize]) as libc::c_ulong
                    & ((1 as libc::c_ulong) << 30 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as uint32_t;
            }
        }
        if ss != 0 {
            ss >>= 1 as libc::c_int;
        } else {
            t = t.wrapping_sub(1);
            t;
        }
    }
    j = 0 as libc::c_int as uint32_t;
    while j < 37 as libc::c_int as libc::c_uint {
        (*ctx)
            .x[j
            .wrapping_add(100 as libc::c_int as libc::c_uint)
            .wrapping_sub(37 as libc::c_int as libc::c_uint) as usize] = x[j as usize];
        j = j.wrapping_add(1);
        j;
    }
    while j < 100 as libc::c_int as libc::c_uint {
        (*ctx)
            .x[j.wrapping_sub(37 as libc::c_int as libc::c_uint)
            as usize] = x[j as usize];
        j = j.wrapping_add(1);
        j;
    }
    (*ctx).index = 0 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn nettle_knuth_lfib_get(
    mut ctx: *mut knuth_lfib_ctx,
) -> uint32_t {
    let mut value: uint32_t = 0;
    if (*ctx).index < 100 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"ctx->index < KK\0" as *const u8 as *const libc::c_char,
            b"knuth-lfib.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"uint32_t nettle_knuth_lfib_get(struct knuth_lfib_ctx *)\0"))
                .as_ptr(),
        );
    }
    'c_1877: {
        if (*ctx).index < 100 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"ctx->index < KK\0" as *const u8 as *const libc::c_char,
                b"knuth-lfib.c\0" as *const u8 as *const libc::c_char,
                119 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 56],
                    &[libc::c_char; 56],
                >(b"uint32_t nettle_knuth_lfib_get(struct knuth_lfib_ctx *)\0"))
                    .as_ptr(),
            );
        }
    };
    value = (*ctx).x[(*ctx).index as usize];
    (*ctx)
        .x[(*ctx).index
        as usize] = ((*ctx).x[(*ctx).index as usize] as libc::c_uint)
        .wrapping_sub(
            (*ctx)
                .x[((*ctx).index)
                .wrapping_add(100 as libc::c_int as libc::c_uint)
                .wrapping_sub(37 as libc::c_int as libc::c_uint)
                .wrapping_rem(100 as libc::c_int as libc::c_uint) as usize],
        ) as uint32_t as uint32_t;
    (*ctx)
        .x[(*ctx).index
        as usize] = ((*ctx).x[(*ctx).index as usize] as libc::c_ulong
        & ((1 as libc::c_ulong) << 30 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as uint32_t;
    (*ctx)
        .index = ((*ctx).index)
        .wrapping_add(1 as libc::c_int as libc::c_uint)
        .wrapping_rem(100 as libc::c_int as libc::c_uint);
    return value;
}
pub unsafe extern "C" fn nettle_knuth_lfib_get_array(
    mut ctx: *mut knuth_lfib_ctx,
    mut n: size_t,
    mut a: *mut uint32_t,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < n {
        *a.offset(i as isize) = nettle_knuth_lfib_get(ctx);
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn nettle_knuth_lfib_random(
    mut ctx: *mut knuth_lfib_ctx,
    mut n: size_t,
    mut dst: *mut uint8_t,
) {
    while n >= 3 as libc::c_int as libc::c_ulong {
        let mut value: uint32_t = nettle_knuth_lfib_get(ctx);
        value ^= value >> 24 as libc::c_int;
        *dst
            .offset(
                0 as libc::c_int as isize,
            ) = (value >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        *dst
            .offset(
                1 as libc::c_int as isize,
            ) = (value >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as uint8_t;
        *dst
            .offset(
                2 as libc::c_int as isize,
            ) = (value & 0xff as libc::c_int as libc::c_uint) as uint8_t;
        n = (n as libc::c_ulong).wrapping_sub(3 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        dst = dst.offset(3 as libc::c_int as isize);
    }
    if n != 0 {
        let mut value_0: uint32_t = nettle_knuth_lfib_get(ctx);
        match n {
            1 => {
                let fresh0 = dst;
                dst = dst.offset(1);
                *fresh0 = (value_0 & 0xff as libc::c_int as libc::c_uint) as uint8_t;
            }
            2 => {
                *dst
                    .offset(
                        0 as libc::c_int as isize,
                    ) = (value_0 >> 8 as libc::c_int
                    & 0xff as libc::c_int as libc::c_uint) as uint8_t;
                *dst
                    .offset(
                        1 as libc::c_int as isize,
                    ) = (value_0 & 0xff as libc::c_int as libc::c_uint) as uint8_t;
            }
            _ => {
                abort();
            }
        }
    }
}
