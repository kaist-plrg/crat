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
    fn _nettle_umac_poly64(
        kh: uint32_t,
        kl: uint32_t,
        y: uint64_t,
        m: uint64_t,
    ) -> uint64_t;
    fn _nettle_umac_poly128(
        k: *const uint32_t,
        y: *mut uint64_t,
        mh: uint64_t,
        ml: uint64_t,
    );
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub unsafe extern "C" fn _nettle_umac_l2_init(
    mut size: libc::c_uint,
    mut k: *mut uint32_t,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < size {
        let mut w: uint32_t = *k.offset(i as isize);
        w = ((w << 8 as libc::c_int | w >> (-(8 as libc::c_int) & 31 as libc::c_int))
            as libc::c_ulong & 0xff00ff as libc::c_ulong
            | (w << 24 as libc::c_int | w >> (-(24 as libc::c_int) & 31 as libc::c_int))
                as libc::c_ulong & 0xff00ff00 as libc::c_ulong) as uint32_t;
        *k
            .offset(
                i as isize,
            ) = (w as libc::c_ulong & 0x1ffffff as libc::c_ulong) as uint32_t;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn _nettle_umac_l2(
    mut key: *const uint32_t,
    mut state: *mut uint64_t,
    mut n: libc::c_uint,
    mut count: uint64_t,
    mut m: *const uint64_t,
) {
    let mut prev: *mut uint64_t = state
        .offset((2 as libc::c_int as libc::c_uint).wrapping_mul(n) as isize);
    let mut i: libc::c_uint = 0;
    if count == 0 as libc::c_int as libc::c_ulong {
        memcpy(
            prev as *mut libc::c_void,
            m as *const libc::c_void,
            (n as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong),
        );
    } else if count == 1 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int as libc::c_uint;
        while i < n {
            let mut y: uint64_t = _nettle_umac_poly64(
                *key.offset(0 as libc::c_int as isize),
                *key.offset(1 as libc::c_int as isize),
                1 as libc::c_int as uint64_t,
                *prev.offset(i as isize),
            );
            *state
                .offset(
                    (2 as libc::c_int as libc::c_uint)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                ) = _nettle_umac_poly64(
                *key.offset(0 as libc::c_int as isize),
                *key.offset(1 as libc::c_int as isize),
                y,
                *m.offset(i as isize),
            );
            i = i.wrapping_add(1);
            i;
            key = key.offset(6 as libc::c_int as isize);
        }
    } else if count < 16384 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int as libc::c_uint;
        while i < n {
            *state
                .offset(
                    (2 as libc::c_int as libc::c_uint)
                        .wrapping_mul(i)
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                ) = _nettle_umac_poly64(
                *key.offset(0 as libc::c_int as isize),
                *key.offset(1 as libc::c_int as isize),
                *state
                    .offset(
                        (2 as libc::c_int as libc::c_uint)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                    ),
                *m.offset(i as isize),
            );
            i = i.wrapping_add(1);
            i;
            key = key.offset(6 as libc::c_int as isize);
        }
    } else if count.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {
        if count == 16384 as libc::c_int as libc::c_ulong {
            i = 0 as libc::c_int as libc::c_uint;
            key = key.offset(2 as libc::c_int as isize);
            while i < n {
                let mut y_0: uint64_t = *state
                    .offset(
                        (2 as libc::c_int as libc::c_uint)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                    );
                if y_0 >= (59 as libc::c_int as uint64_t).wrapping_neg() {
                    y_0 = (y_0 as libc::c_ulong)
                        .wrapping_sub((59 as libc::c_int as uint64_t).wrapping_neg())
                        as uint64_t as uint64_t;
                }
                *state
                    .offset(
                        (2 as libc::c_int as libc::c_uint).wrapping_mul(i) as isize,
                    ) = 0 as libc::c_int as uint64_t;
                *state
                    .offset(
                        (2 as libc::c_int as libc::c_uint)
                            .wrapping_mul(i)
                            .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                    ) = 1 as libc::c_int as uint64_t;
                _nettle_umac_poly128(
                    key,
                    state
                        .offset(
                            (2 as libc::c_int as libc::c_uint).wrapping_mul(i) as isize,
                        ),
                    0 as libc::c_int as uint64_t,
                    y_0,
                );
                i = i.wrapping_add(1);
                i;
                key = key.offset(6 as libc::c_int as isize);
            }
        }
        memcpy(
            prev as *mut libc::c_void,
            m as *const libc::c_void,
            (n as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong),
        );
    } else {
        i = 0 as libc::c_int as libc::c_uint;
        key = key.offset(2 as libc::c_int as isize);
        while i < n {
            _nettle_umac_poly128(
                key,
                state
                    .offset((2 as libc::c_int as libc::c_uint).wrapping_mul(i) as isize),
                *prev.offset(i as isize),
                *m.offset(i as isize),
            );
            i = i.wrapping_add(1);
            i;
            key = key.offset(6 as libc::c_int as isize);
        }
    };
}
pub unsafe extern "C" fn _nettle_umac_l2_final(
    mut key: *const uint32_t,
    mut state: *mut uint64_t,
    mut n: libc::c_uint,
    mut count: uint64_t,
) {
    let mut prev: *mut uint64_t = state
        .offset((2 as libc::c_int as libc::c_uint).wrapping_mul(n) as isize);
    let mut i: libc::c_uint = 0;
    if count > 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"count > 0\0" as *const u8 as *const libc::c_char,
            b"umac-l2.c\0" as *const u8 as *const libc::c_char,
            112 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 81],
                &[libc::c_char; 81],
            >(
                b"void _nettle_umac_l2_final(const uint32_t *, uint64_t *, unsigned int, uint64_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1575: {
        if count > 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"count > 0\0" as *const u8 as *const libc::c_char,
                b"umac-l2.c\0" as *const u8 as *const libc::c_char,
                112 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 81],
                    &[libc::c_char; 81],
                >(
                    b"void _nettle_umac_l2_final(const uint32_t *, uint64_t *, unsigned int, uint64_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if count == 1 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int as libc::c_uint;
        while i < n {
            let fresh0 = state;
            state = state.offset(1);
            *fresh0 = 0 as libc::c_int as uint64_t;
            let fresh1 = prev;
            prev = prev.offset(1);
            let fresh2 = state;
            state = state.offset(1);
            *fresh2 = *fresh1;
            i = i.wrapping_add(1);
            i;
        }
    } else if count <= 16384 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int as libc::c_uint;
        while i < n {
            let mut y: uint64_t = 0;
            let fresh3 = state;
            state = state.offset(1);
            *fresh3 = 0 as libc::c_int as uint64_t;
            y = *state;
            if y >= (59 as libc::c_int as uint64_t).wrapping_neg() {
                y = (y as libc::c_ulong)
                    .wrapping_sub((59 as libc::c_int as uint64_t).wrapping_neg())
                    as uint64_t as uint64_t;
            }
            let fresh4 = state;
            state = state.offset(1);
            *fresh4 = y;
            i = i.wrapping_add(1);
            i;
        }
    } else {
        let mut pad: uint64_t = (1 as libc::c_int as uint64_t) << 63 as libc::c_int;
        if count.wrapping_rem(2 as libc::c_int as libc::c_ulong)
            == 1 as libc::c_int as libc::c_ulong
        {
            i = 0 as libc::c_int as libc::c_uint;
            key = key.offset(2 as libc::c_int as isize);
            while i < n {
                _nettle_umac_poly128(
                    key,
                    state
                        .offset(
                            (2 as libc::c_int as libc::c_uint).wrapping_mul(i) as isize,
                        ),
                    *prev.offset(i as isize),
                    pad,
                );
                i = i.wrapping_add(1);
                i;
                key = key.offset(6 as libc::c_int as isize);
            }
        } else {
            i = 0 as libc::c_int as libc::c_uint;
            key = key.offset(2 as libc::c_int as isize);
            while i < n {
                _nettle_umac_poly128(
                    key,
                    state
                        .offset(
                            (2 as libc::c_int as libc::c_uint).wrapping_mul(i) as isize,
                        ),
                    pad,
                    0 as libc::c_int as uint64_t,
                );
                i = i.wrapping_add(1);
                i;
                key = key.offset(6 as libc::c_int as isize);
            }
        }
        i = 0 as libc::c_int as libc::c_uint;
        while i < n {
            let mut yh: uint64_t = 0;
            let mut yl: uint64_t = 0;
            yh = *state.offset(0 as libc::c_int as isize);
            yl = *state.offset(1 as libc::c_int as isize);
            if yh == !(0 as libc::c_int as uint64_t)
                && yl >= (159 as libc::c_int as uint64_t).wrapping_neg()
            {
                *state.offset(0 as libc::c_int as isize) = 0 as libc::c_int as uint64_t;
                yl = (yl as libc::c_ulong)
                    .wrapping_sub((159 as libc::c_int as uint64_t).wrapping_neg())
                    as uint64_t as uint64_t;
                *state.offset(1 as libc::c_int as isize) = yl;
            }
            i = i.wrapping_add(1);
            i;
            state = state.offset(2 as libc::c_int as isize);
        }
    };
}
