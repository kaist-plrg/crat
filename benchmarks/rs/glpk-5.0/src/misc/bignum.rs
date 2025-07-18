use ::libc;
extern "C" {
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
}
pub unsafe extern "C" fn _glp_bigmul(
    mut n: libc::c_int,
    mut m: libc::c_int,
    mut x: *mut libc::c_ushort,
    mut y: *mut libc::c_ushort,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    (n >= 1 as libc::c_int
        || {
            glp_assert_(
                b"n >= 1\0" as *const u8 as *const libc::c_char,
                b"misc/bignum.c\0" as *const u8 as *const libc::c_char,
                75 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (m >= 1 as libc::c_int
        || {
            glp_assert_(
                b"m >= 1\0" as *const u8 as *const libc::c_char,
                b"misc/bignum.c\0" as *const u8 as *const libc::c_char,
                76 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    j = 0 as libc::c_int;
    while j < m {
        *x.offset(j as isize) = 0 as libc::c_int as libc::c_ushort;
        j += 1;
        j;
    }
    i = 0 as libc::c_int;
    while i < n {
        if *x.offset((i + m) as isize) != 0 {
            t = 0 as libc::c_int as libc::c_uint;
            j = 0 as libc::c_int;
            while j < m {
                t = t
                    .wrapping_add(
                        (*x.offset((i + m) as isize) as libc::c_uint)
                            .wrapping_mul(*y.offset(j as isize) as libc::c_uint)
                            .wrapping_add(*x.offset((i + j) as isize) as libc::c_uint),
                    );
                *x.offset((i + j) as isize) = t as libc::c_ushort;
                t >>= 16 as libc::c_int;
                j += 1;
                j;
            }
            *x.offset((i + m) as isize) = t as libc::c_ushort;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn _glp_bigdiv(
    mut n: libc::c_int,
    mut m: libc::c_int,
    mut x: *mut libc::c_ushort,
    mut y: *mut libc::c_ushort,
) {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut d: libc::c_ushort = 0;
    let mut q: libc::c_ushort = 0;
    let mut r: libc::c_ushort = 0;
    (n >= 0 as libc::c_int
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const libc::c_char,
                b"misc/bignum.c\0" as *const u8 as *const libc::c_char,
                133 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (m >= 1 as libc::c_int
        || {
            glp_assert_(
                b"m >= 1\0" as *const u8 as *const libc::c_char,
                b"misc/bignum.c\0" as *const u8 as *const libc::c_char,
                134 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*y.offset((m - 1 as libc::c_int) as isize) as libc::c_int != 0 as libc::c_int
        || {
            glp_assert_(
                b"y[m-1] != 0\0" as *const u8 as *const libc::c_char,
                b"misc/bignum.c\0" as *const u8 as *const libc::c_char,
                135 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if m == 1 as libc::c_int {
        d = 0 as libc::c_int as libc::c_ushort;
        i = n;
        while i >= 0 as libc::c_int {
            t = ((d as libc::c_uint) << 16 as libc::c_int)
                .wrapping_add(*x.offset(i as isize) as libc::c_uint);
            *x
                .offset(
                    (i + 1 as libc::c_int) as isize,
                ) = t.wrapping_div(*y.offset(0 as libc::c_int as isize) as libc::c_uint)
                as libc::c_ushort;
            d = t.wrapping_rem(*y.offset(0 as libc::c_int as isize) as libc::c_uint)
                as libc::c_ushort;
            i -= 1;
            i;
        }
        *x.offset(0 as libc::c_int as isize) = d;
    } else {
        d = (0x10000 as libc::c_int as libc::c_uint)
            .wrapping_div(
                (*y.offset((m - 1 as libc::c_int) as isize) as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
            ) as libc::c_ushort;
        if d as libc::c_int == 1 as libc::c_int {
            *x.offset((n + m) as isize) = 0 as libc::c_int as libc::c_ushort;
        } else {
            t = 0 as libc::c_int as libc::c_uint;
            i = 0 as libc::c_int;
            while i < n + m {
                t = t
                    .wrapping_add(
                        (*x.offset(i as isize) as libc::c_uint)
                            .wrapping_mul(d as libc::c_uint),
                    );
                *x.offset(i as isize) = t as libc::c_ushort;
                t >>= 16 as libc::c_int;
                i += 1;
                i;
            }
            *x.offset((n + m) as isize) = t as libc::c_ushort;
            t = 0 as libc::c_int as libc::c_uint;
            j = 0 as libc::c_int;
            while j < m {
                t = t
                    .wrapping_add(
                        (*y.offset(j as isize) as libc::c_uint)
                            .wrapping_mul(d as libc::c_uint),
                    );
                *y.offset(j as isize) = t as libc::c_ushort;
                t >>= 16 as libc::c_int;
                j += 1;
                j;
            }
        }
        i = n;
        while i >= 0 as libc::c_int {
            if (*x.offset((i + m) as isize) as libc::c_int)
                < *y.offset((m - 1 as libc::c_int) as isize) as libc::c_int
            {
                t = ((*x.offset((i + m) as isize) as libc::c_uint) << 16 as libc::c_int)
                    .wrapping_add(
                        *x.offset((i + m - 1 as libc::c_int) as isize) as libc::c_uint,
                    );
                q = t
                    .wrapping_div(
                        *y.offset((m - 1 as libc::c_int) as isize) as libc::c_uint,
                    ) as libc::c_ushort;
                r = t
                    .wrapping_rem(
                        *y.offset((m - 1 as libc::c_int) as isize) as libc::c_uint,
                    ) as libc::c_ushort;
                if q as libc::c_int == 0 as libc::c_int {
                    current_block = 14348810298283489471;
                } else {
                    current_block = 1599017216358687849;
                }
            } else {
                q = 0 as libc::c_int as libc::c_ushort;
                r = *x.offset((i + m - 1 as libc::c_int) as isize);
                current_block = 11236950403584799864;
            }
            loop {
                match current_block {
                    14348810298283489471 => {
                        *x.offset((i + m) as isize) = q;
                        break;
                    }
                    11236950403584799864 => {
                        q = q.wrapping_sub(1);
                        q;
                        t = (r as libc::c_uint)
                            .wrapping_add(
                                *y.offset((m - 1 as libc::c_int) as isize) as libc::c_uint,
                            );
                        r = t as libc::c_ushort;
                        if !(t > 0xffff as libc::c_int as libc::c_uint) {
                            current_block = 1599017216358687849;
                            continue;
                        }
                    }
                    _ => {
                        t = (*y.offset((m - 2 as libc::c_int) as isize) as libc::c_uint)
                            .wrapping_mul(q as libc::c_uint);
                        if (t >> 16 as libc::c_int) as libc::c_ushort as libc::c_int
                            > r as libc::c_int
                        {
                            current_block = 11236950403584799864;
                            continue;
                        }
                        if !(((t >> 16 as libc::c_int) as libc::c_ushort as libc::c_int)
                            < r as libc::c_int)
                        {
                            if t as libc::c_ushort as libc::c_int
                                > *x.offset((i + m - 2 as libc::c_int) as isize)
                                    as libc::c_int
                            {
                                current_block = 11236950403584799864;
                                continue;
                            }
                        }
                    }
                }
                if q as libc::c_int == 0 as libc::c_int {
                    current_block = 14348810298283489471;
                    continue;
                }
                t = 0 as libc::c_int as libc::c_uint;
                j = 0 as libc::c_int;
                while j < m {
                    t = t
                        .wrapping_add(
                            (*y.offset(j as isize) as libc::c_uint)
                                .wrapping_mul(q as libc::c_uint),
                        );
                    if (*x.offset((i + j) as isize) as libc::c_int)
                        < t as libc::c_ushort as libc::c_int
                    {
                        t = t.wrapping_add(0x10000 as libc::c_int as libc::c_uint);
                    }
                    let ref mut fresh0 = *x.offset((i + j) as isize);
                    *fresh0 = (*fresh0 as libc::c_int
                        - t as libc::c_ushort as libc::c_int) as libc::c_ushort;
                    t >>= 16 as libc::c_int;
                    j += 1;
                    j;
                }
                if *x.offset((i + m) as isize) as libc::c_int
                    >= t as libc::c_ushort as libc::c_int
                {
                    current_block = 14348810298283489471;
                    continue;
                }
                q = q.wrapping_sub(1);
                q;
                t = 0 as libc::c_int as libc::c_uint;
                j = 0 as libc::c_int;
                while j < m {
                    t = t
                        .wrapping_add(
                            (*x.offset((i + j) as isize) as libc::c_uint)
                                .wrapping_add(*y.offset(j as isize) as libc::c_uint),
                        );
                    *x.offset((i + j) as isize) = t as libc::c_ushort;
                    t >>= 16 as libc::c_int;
                    j += 1;
                    j;
                }
                current_block = 14348810298283489471;
            }
            i -= 1;
            i;
        }
        if d as libc::c_int > 1 as libc::c_int {
            t = 0 as libc::c_int as libc::c_uint;
            i = m - 1 as libc::c_int;
            while i >= 0 as libc::c_int {
                t = (t << 16 as libc::c_int)
                    .wrapping_add(*x.offset(i as isize) as libc::c_uint);
                *x
                    .offset(
                        i as isize,
                    ) = t.wrapping_div(d as libc::c_uint) as libc::c_ushort;
                t = t.wrapping_rem(d as libc::c_uint);
                i -= 1;
                i;
            }
            t = 0 as libc::c_int as libc::c_uint;
            j = m - 1 as libc::c_int;
            while j >= 0 as libc::c_int {
                t = (t << 16 as libc::c_int)
                    .wrapping_add(*y.offset(j as isize) as libc::c_uint);
                *y
                    .offset(
                        j as isize,
                    ) = t.wrapping_div(d as libc::c_uint) as libc::c_ushort;
                t = t.wrapping_rem(d as libc::c_uint);
                j -= 1;
                j;
            }
        }
    };
}
