use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
unsafe extern "C" fn poly128_mul(mut k: *const uint32_t, mut y: *mut uint64_t) {
    let mut y0: uint64_t = 0;
    let mut y1: uint64_t = 0;
    let mut y2: uint64_t = 0;
    let mut y3: uint64_t = 0;
    let mut p0: uint64_t = 0;
    let mut p1: uint64_t = 0;
    let mut p2: uint64_t = 0;
    let mut p3: uint64_t = 0;
    let mut m0: uint64_t = 0;
    let mut m1: uint64_t = 0;
    let mut m2: uint64_t = 0;
    y0 = *y.offset(1 as libc::c_int as isize) & 0xffffffff as libc::c_ulong;
    y1 = *y.offset(1 as libc::c_int as isize) >> 32 as libc::c_int;
    y2 = *y.offset(0 as libc::c_int as isize) & 0xffffffff as libc::c_ulong;
    y3 = *y.offset(0 as libc::c_int as isize) >> 32 as libc::c_int;
    p0 = y0.wrapping_mul(*k.offset(3 as libc::c_int as isize) as libc::c_ulong);
    m0 = y0
        .wrapping_mul(*k.offset(2 as libc::c_int as isize) as libc::c_ulong)
        .wrapping_add(
            y1.wrapping_mul(*k.offset(3 as libc::c_int as isize) as libc::c_ulong),
        );
    p1 = y0
        .wrapping_mul(*k.offset(1 as libc::c_int as isize) as libc::c_ulong)
        .wrapping_add(
            y1.wrapping_mul(*k.offset(2 as libc::c_int as isize) as libc::c_ulong),
        )
        .wrapping_add(
            y2.wrapping_mul(*k.offset(3 as libc::c_int as isize) as libc::c_ulong),
        );
    m1 = y0
        .wrapping_mul(*k.offset(0 as libc::c_int as isize) as libc::c_ulong)
        .wrapping_add(
            y1.wrapping_mul(*k.offset(1 as libc::c_int as isize) as libc::c_ulong),
        )
        .wrapping_add(
            y2.wrapping_mul(*k.offset(2 as libc::c_int as isize) as libc::c_ulong),
        )
        .wrapping_add(
            y3.wrapping_mul(*k.offset(3 as libc::c_int as isize) as libc::c_ulong),
        );
    p2 = y1
        .wrapping_mul(*k.offset(0 as libc::c_int as isize) as libc::c_ulong)
        .wrapping_add(
            y2.wrapping_mul(*k.offset(1 as libc::c_int as isize) as libc::c_ulong),
        )
        .wrapping_add(
            y3.wrapping_mul(*k.offset(2 as libc::c_int as isize) as libc::c_ulong),
        );
    m2 = y2
        .wrapping_mul(*k.offset(0 as libc::c_int as isize) as libc::c_ulong)
        .wrapping_add(
            y3.wrapping_mul(*k.offset(1 as libc::c_int as isize) as libc::c_ulong),
        );
    p3 = y3.wrapping_mul(*k.offset(0 as libc::c_int as isize) as libc::c_ulong);
    m1 = (m1 as libc::c_ulong)
        .wrapping_add(
            (159 as libc::c_int as libc::c_ulong).wrapping_mul(p3 >> 32 as libc::c_int),
        ) as uint64_t as uint64_t;
    p1 = (p1 as libc::c_ulong)
        .wrapping_add(
            (159 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    (p3 & 0xffffffff as libc::c_ulong)
                        .wrapping_add(m2 >> 32 as libc::c_int),
                ),
        ) as uint64_t as uint64_t;
    m0 = (m0 as libc::c_ulong)
        .wrapping_add(
            (159 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    (p2 >> 32 as libc::c_int)
                        .wrapping_add(m2 & 0xffffffff as libc::c_ulong),
                ),
        ) as uint64_t as uint64_t;
    p0 = (p0 as libc::c_ulong)
        .wrapping_add(
            (159 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    (p2 & 0xffffffff as libc::c_ulong)
                        .wrapping_add(m1 >> 32 as libc::c_int),
                ),
        ) as uint64_t as uint64_t;
    p1 = (p1 as libc::c_ulong).wrapping_add(m0 >> 32 as libc::c_int) as uint64_t
        as uint64_t;
    m0 <<= 32 as libc::c_int;
    m1 <<= 32 as libc::c_int;
    p0 = (p0 as libc::c_ulong).wrapping_add(m0) as uint64_t as uint64_t;
    p1 = (p1 as libc::c_ulong).wrapping_add((p0 < m0) as libc::c_int as libc::c_ulong)
        as uint64_t as uint64_t;
    p1 = (p1 as libc::c_ulong).wrapping_add(m1) as uint64_t as uint64_t;
    if p1 < m1 {
        p0 = (p0 as libc::c_ulong).wrapping_add(159 as libc::c_int as libc::c_ulong)
            as uint64_t as uint64_t;
        p1 = (p1 as libc::c_ulong)
            .wrapping_add(
                (p0 < 159 as libc::c_int as libc::c_ulong) as libc::c_int
                    as libc::c_ulong,
            ) as uint64_t as uint64_t;
    }
    *y.offset(0 as libc::c_int as isize) = p1;
    *y.offset(1 as libc::c_int as isize) = p0;
}
pub unsafe extern "C" fn _nettle_umac_poly128(
    mut k: *const uint32_t,
    mut y: *mut uint64_t,
    mut mh: uint64_t,
    mut ml: uint64_t,
) {
    let mut yh: uint64_t = 0;
    let mut yl: uint64_t = 0;
    let mut cy: uint64_t = 0;
    if mh >> 32 as libc::c_int == 0xffffffff as libc::c_uint as libc::c_ulong {
        poly128_mul(k, y);
        if *y.offset(1 as libc::c_int as isize) > 0 as libc::c_int as libc::c_ulong {
            let ref mut fresh0 = *y.offset(1 as libc::c_int as isize);
            *fresh0 = (*fresh0).wrapping_sub(1);
            *fresh0;
        } else if *y.offset(0 as libc::c_int as isize)
            > 0 as libc::c_int as libc::c_ulong
        {
            let ref mut fresh1 = *y.offset(0 as libc::c_int as isize);
            *fresh1 = (*fresh1).wrapping_sub(1);
            *fresh1;
            *y.offset(1 as libc::c_int as isize) = !(0 as libc::c_int as uint64_t);
        } else {
            *y.offset(0 as libc::c_int as isize) = !(0 as libc::c_int as uint64_t);
            *y
                .offset(
                    1 as libc::c_int as isize,
                ) = (159 as libc::c_int as uint64_t)
                .wrapping_neg()
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
        mh = (mh as libc::c_ulong)
            .wrapping_sub(
                (ml < 159 as libc::c_int as libc::c_ulong) as libc::c_int
                    as libc::c_ulong,
            ) as uint64_t as uint64_t;
        ml = (ml as libc::c_ulong).wrapping_sub(159 as libc::c_int as libc::c_ulong)
            as uint64_t as uint64_t;
    }
    if mh < !(0 as libc::c_int as uint64_t)
        || ml < (159 as libc::c_int as uint64_t).wrapping_neg()
    {} else {
        __assert_fail(
            b"mh < UMAC_P128_HI || ml < UMAC_P128_LO\0" as *const u8
                as *const libc::c_char,
            b"umac-poly128.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"void _nettle_umac_poly128(const uint32_t *, uint64_t *, uint64_t, uint64_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1077: {
        if mh < !(0 as libc::c_int as uint64_t)
            || ml < (159 as libc::c_int as uint64_t).wrapping_neg()
        {} else {
            __assert_fail(
                b"mh < UMAC_P128_HI || ml < UMAC_P128_LO\0" as *const u8
                    as *const libc::c_char,
                b"umac-poly128.c\0" as *const u8 as *const libc::c_char,
                131 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 76],
                    &[libc::c_char; 76],
                >(
                    b"void _nettle_umac_poly128(const uint32_t *, uint64_t *, uint64_t, uint64_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    poly128_mul(k, y);
    yl = (*y.offset(1 as libc::c_int as isize)).wrapping_add(ml);
    cy = (yl < ml) as libc::c_int as uint64_t;
    yh = (*y.offset(0 as libc::c_int as isize)).wrapping_add(cy);
    cy = (yh < cy) as libc::c_int as uint64_t;
    yh = (yh as libc::c_ulong).wrapping_add(mh) as uint64_t as uint64_t;
    cy = (cy as libc::c_ulong).wrapping_add((yh < mh) as libc::c_int as libc::c_ulong)
        as uint64_t as uint64_t;
    if cy <= 1 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"cy <= 1\0" as *const u8 as *const libc::c_char,
            b"umac-poly128.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"void _nettle_umac_poly128(const uint32_t *, uint64_t *, uint64_t, uint64_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_600: {
        if cy <= 1 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"cy <= 1\0" as *const u8 as *const libc::c_char,
                b"umac-poly128.c\0" as *const u8 as *const libc::c_char,
                140 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 76],
                    &[libc::c_char; 76],
                >(
                    b"void _nettle_umac_poly128(const uint32_t *, uint64_t *, uint64_t, uint64_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if cy != 0 {
        yl = (yl as libc::c_ulong).wrapping_add(159 as libc::c_int as libc::c_ulong)
            as uint64_t as uint64_t;
        yh = (yh as libc::c_ulong)
            .wrapping_add(
                (yl < 159 as libc::c_int as libc::c_ulong) as libc::c_int
                    as libc::c_ulong,
            ) as uint64_t as uint64_t;
    }
    *y.offset(0 as libc::c_int as isize) = yh;
    *y.offset(1 as libc::c_int as isize) = yl;
}
