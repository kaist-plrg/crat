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
unsafe extern "C" fn poly64_mul(
    mut kh: uint32_t,
    mut kl: uint32_t,
    mut y: uint64_t,
) -> uint64_t {
    let mut yl: uint64_t = 0;
    let mut yh: uint64_t = 0;
    let mut pl: uint64_t = 0;
    let mut ph: uint64_t = 0;
    let mut ml: uint64_t = 0;
    let mut mh: uint64_t = 0;
    yl = y & 0xffffffff as libc::c_uint as libc::c_ulong;
    yh = y >> 32 as libc::c_int;
    pl = yl.wrapping_mul(kl as libc::c_ulong);
    ph = yh.wrapping_mul(kh as libc::c_ulong);
    ml = yh
        .wrapping_mul(kl as libc::c_ulong)
        .wrapping_add(yl.wrapping_mul(kh as libc::c_ulong));
    mh = ml >> 32 as libc::c_int;
    ml <<= 32 as libc::c_int;
    pl = (pl as libc::c_ulong).wrapping_add(ml) as uint64_t as uint64_t;
    ph = (ph as libc::c_ulong)
        .wrapping_add(mh.wrapping_add((pl < ml) as libc::c_int as libc::c_ulong))
        as uint64_t as uint64_t;
    if ph < (1 as libc::c_int as uint64_t) << 57 as libc::c_int {} else {
        __assert_fail(
            b"ph < ((uint64_t) 1 << 57)\0" as *const u8 as *const libc::c_char,
            b"umac-poly64.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"uint64_t poly64_mul(uint32_t, uint32_t, uint64_t)\0"))
                .as_ptr(),
        );
    }
    'c_622: {
        if ph < (1 as libc::c_int as uint64_t) << 57 as libc::c_int {} else {
            __assert_fail(
                b"ph < ((uint64_t) 1 << 57)\0" as *const u8 as *const libc::c_char,
                b"umac-poly64.c\0" as *const u8 as *const libc::c_char,
                56 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"uint64_t poly64_mul(uint32_t, uint32_t, uint64_t)\0"))
                    .as_ptr(),
            );
        }
    };
    ph = (ph as libc::c_ulong).wrapping_mul(59 as libc::c_int as libc::c_ulong)
        as uint64_t as uint64_t;
    pl = (pl as libc::c_ulong).wrapping_add(ph) as uint64_t as uint64_t;
    if pl < ph {
        pl = (pl as libc::c_ulong).wrapping_add(59 as libc::c_int as libc::c_ulong)
            as uint64_t as uint64_t;
    }
    return pl;
}
pub unsafe extern "C" fn _nettle_umac_poly64(
    mut kh: uint32_t,
    mut kl: uint32_t,
    mut y: uint64_t,
    mut m: uint64_t,
) -> uint64_t {
    if m >> 32 as libc::c_int == 0xffffffff as libc::c_uint as libc::c_ulong {
        y = poly64_mul(kh, kl, y);
        if y == 0 as libc::c_int as libc::c_ulong {
            y = (59 as libc::c_int as uint64_t)
                .wrapping_neg()
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        } else {
            y = y.wrapping_sub(1);
            y;
        }
        m = (m as libc::c_ulong).wrapping_sub(59 as libc::c_int as libc::c_ulong)
            as uint64_t as uint64_t;
    }
    y = poly64_mul(kh, kl, y);
    y = (y as libc::c_ulong).wrapping_add(m) as uint64_t as uint64_t;
    if y < m {
        y = (y as libc::c_ulong).wrapping_add(59 as libc::c_int as libc::c_ulong)
            as uint64_t as uint64_t;
    }
    return y;
}
