use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
unsafe extern "C" fn read32(mut p: *const libc::c_char) -> uint32_t {
    let mut result: uint32_t = 0;
    memcpy(
        &mut result as *mut uint32_t as *mut libc::c_void,
        p as *const libc::c_void,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
    );
    return result;
}
static mut c1: uint32_t = 0xcc9e2d51 as libc::c_uint;
static mut c2: uint32_t = 0x1b873593 as libc::c_int as uint32_t;
unsafe extern "C" fn fmix(mut h: uint32_t) -> uint32_t {
    h ^= h >> 16 as libc::c_int;
    h = (h as libc::c_uint).wrapping_mul(0x85ebca6b as libc::c_uint) as uint32_t
        as uint32_t;
    h ^= h >> 13 as libc::c_int;
    h = (h as libc::c_uint).wrapping_mul(0xc2b2ae35 as libc::c_uint) as uint32_t
        as uint32_t;
    h ^= h >> 16 as libc::c_int;
    return h;
}
unsafe extern "C" fn ror32(mut val: uint32_t, mut shift: libc::c_int) -> uint32_t {
    return if shift == 0 as libc::c_int {
        val
    } else {
        val >> shift | val << 32 as libc::c_int - shift
    };
}
unsafe extern "C" fn mur(mut a: uint32_t, mut h: uint32_t) -> uint32_t {
    a = (a as libc::c_uint).wrapping_mul(c1) as uint32_t as uint32_t;
    a = ror32(a, 17 as libc::c_int);
    a = (a as libc::c_uint).wrapping_mul(c2) as uint32_t as uint32_t;
    h ^= a;
    h = ror32(h, 19 as libc::c_int);
    return h
        .wrapping_mul(5 as libc::c_int as libc::c_uint)
        .wrapping_add(0xe6546b64 as libc::c_uint);
}
unsafe extern "C" fn Hash32Len13to24(
    mut s: *const libc::c_char,
    mut len: size_t,
) -> uint32_t {
    let mut a: uint32_t = read32(
        s.offset(-(4 as libc::c_int as isize)).offset((len >> 1 as libc::c_int) as isize),
    );
    let mut b: uint32_t = read32(s.offset(4 as libc::c_int as isize));
    let mut c: uint32_t = read32(
        s.offset(len as isize).offset(-(8 as libc::c_int as isize)),
    );
    let mut d: uint32_t = read32(s.offset((len >> 1 as libc::c_int) as isize));
    let mut e: uint32_t = read32(s);
    let mut f: uint32_t = read32(
        s.offset(len as isize).offset(-(4 as libc::c_int as isize)),
    );
    let mut h: uint32_t = len as uint32_t;
    return fmix(mur(f, mur(e, mur(d, mur(c, mur(b, mur(a, h)))))));
}
unsafe extern "C" fn Hash32Len0to4(
    mut s: *const libc::c_char,
    mut len: size_t,
) -> uint32_t {
    let mut b: uint32_t = 0 as libc::c_int as uint32_t;
    let mut c: uint32_t = 9 as libc::c_int as uint32_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < len {
        b = b.wrapping_mul(c1).wrapping_add(*s.offset(i as isize) as libc::c_uint);
        c ^= b;
        i += 1;
        i;
    }
    return fmix(mur(b, mur(len as uint32_t, c)));
}
unsafe extern "C" fn Hash32Len5to12(
    mut s: *const libc::c_char,
    mut len: size_t,
) -> uint32_t {
    let mut a: uint32_t = len as uint32_t;
    let mut b: uint32_t = len.wrapping_mul(5 as libc::c_int as libc::c_ulong)
        as uint32_t;
    let mut c: uint32_t = 9 as libc::c_int as uint32_t;
    let mut d: uint32_t = b;
    a = (a as libc::c_uint).wrapping_add(read32(s)) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            read32(s.offset(len as isize).offset(-(4 as libc::c_int as isize))),
        ) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            read32(
                s
                    .offset(
                        (len >> 1 as libc::c_int & 4 as libc::c_int as libc::c_ulong)
                            as isize,
                    ),
            ),
        ) as uint32_t as uint32_t;
    return fmix(mur(c, mur(b, mur(a, d))));
}
pub unsafe extern "C" fn CityHash32(
    mut s: *const libc::c_char,
    mut len: size_t,
) -> uint32_t {
    let mut iters: size_t = 0;
    let mut a0: uint32_t = 0;
    let mut a1: uint32_t = 0;
    let mut a2: uint32_t = 0;
    let mut a3: uint32_t = 0;
    let mut a4: uint32_t = 0;
    let mut h: uint32_t = 0;
    let mut g: uint32_t = 0;
    let mut f: uint32_t = 0;
    if len <= 24 as libc::c_int as libc::c_ulong {
        return if len <= 12 as libc::c_int as libc::c_ulong {
            if len <= 4 as libc::c_int as libc::c_ulong {
                Hash32Len0to4(s, len)
            } else {
                Hash32Len5to12(s, len)
            }
        } else {
            Hash32Len13to24(s, len)
        };
    }
    h = len as uint32_t;
    g = (c1 as libc::c_ulong).wrapping_mul(len) as uint32_t;
    f = g;
    a0 = (ror32(
        (read32(s.offset(len as isize).offset(-(4 as libc::c_int as isize))))
            .wrapping_mul(c1),
        17 as libc::c_int,
    ))
        .wrapping_mul(c2);
    a1 = (ror32(
        (read32(s.offset(len as isize).offset(-(8 as libc::c_int as isize))))
            .wrapping_mul(c1),
        17 as libc::c_int,
    ))
        .wrapping_mul(c2);
    a2 = (ror32(
        (read32(s.offset(len as isize).offset(-(16 as libc::c_int as isize))))
            .wrapping_mul(c1),
        17 as libc::c_int,
    ))
        .wrapping_mul(c2);
    a3 = (ror32(
        (read32(s.offset(len as isize).offset(-(12 as libc::c_int as isize))))
            .wrapping_mul(c1),
        17 as libc::c_int,
    ))
        .wrapping_mul(c2);
    a4 = (ror32(
        (read32(s.offset(len as isize).offset(-(20 as libc::c_int as isize))))
            .wrapping_mul(c1),
        17 as libc::c_int,
    ))
        .wrapping_mul(c2);
    h ^= a0;
    h = ror32(h, 19 as libc::c_int);
    h = h
        .wrapping_mul(5 as libc::c_int as libc::c_uint)
        .wrapping_add(0xe6546b64 as libc::c_uint);
    h ^= a2;
    h = ror32(h, 19 as libc::c_int);
    h = h
        .wrapping_mul(5 as libc::c_int as libc::c_uint)
        .wrapping_add(0xe6546b64 as libc::c_uint);
    g ^= a1;
    g = ror32(g, 19 as libc::c_int);
    g = g
        .wrapping_mul(5 as libc::c_int as libc::c_uint)
        .wrapping_add(0xe6546b64 as libc::c_uint);
    g ^= a3;
    g = ror32(g, 19 as libc::c_int);
    g = g
        .wrapping_mul(5 as libc::c_int as libc::c_uint)
        .wrapping_add(0xe6546b64 as libc::c_uint);
    f = (f as libc::c_uint).wrapping_add(a4) as uint32_t as uint32_t;
    f = ror32(f, 19 as libc::c_int);
    f = f
        .wrapping_mul(5 as libc::c_int as libc::c_uint)
        .wrapping_add(0xe6546b64 as libc::c_uint);
    iters = len
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(20 as libc::c_int as libc::c_ulong);
    loop {
        let mut a0_0: uint32_t = (ror32((read32(s)).wrapping_mul(c1), 17 as libc::c_int))
            .wrapping_mul(c2);
        let mut a1_0: uint32_t = read32(s.offset(4 as libc::c_int as isize));
        let mut a2_0: uint32_t = (ror32(
            (read32(s.offset(8 as libc::c_int as isize))).wrapping_mul(c1),
            17 as libc::c_int,
        ))
            .wrapping_mul(c2);
        let mut a3_0: uint32_t = (ror32(
            (read32(s.offset(12 as libc::c_int as isize))).wrapping_mul(c1),
            17 as libc::c_int,
        ))
            .wrapping_mul(c2);
        let mut a4_0: uint32_t = read32(s.offset(16 as libc::c_int as isize));
        h ^= a0_0;
        h = ror32(h, 18 as libc::c_int);
        h = h
            .wrapping_mul(5 as libc::c_int as libc::c_uint)
            .wrapping_add(0xe6546b64 as libc::c_uint);
        f = (f as libc::c_uint).wrapping_add(a1_0) as uint32_t as uint32_t;
        f = ror32(f, 19 as libc::c_int);
        f = f.wrapping_mul(c1);
        g = (g as libc::c_uint).wrapping_add(a2_0) as uint32_t as uint32_t;
        g = ror32(g, 18 as libc::c_int);
        g = g
            .wrapping_mul(5 as libc::c_int as libc::c_uint)
            .wrapping_add(0xe6546b64 as libc::c_uint);
        h ^= a3_0.wrapping_add(a1_0);
        h = ror32(h, 19 as libc::c_int);
        h = h
            .wrapping_mul(5 as libc::c_int as libc::c_uint)
            .wrapping_add(0xe6546b64 as libc::c_uint);
        g ^= a4_0;
        g = (__bswap_32(g)).wrapping_mul(5 as libc::c_int as libc::c_uint);
        h = (h as libc::c_uint)
            .wrapping_add(a4_0.wrapping_mul(5 as libc::c_int as libc::c_uint))
            as uint32_t as uint32_t;
        h = __bswap_32(h);
        f = (f as libc::c_uint).wrapping_add(a0_0) as uint32_t as uint32_t;
        let mut aux: uint32_t = f;
        f = h;
        h = aux;
        let mut aux_0: uint32_t = f;
        f = g;
        g = aux_0;
        s = s.offset(20 as libc::c_int as isize);
        iters = iters.wrapping_sub(1);
        if !(iters != 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    g = (ror32(g, 11 as libc::c_int)).wrapping_mul(c1);
    g = (ror32(g, 17 as libc::c_int)).wrapping_mul(c1);
    f = (ror32(f, 11 as libc::c_int)).wrapping_mul(c1);
    f = (ror32(f, 17 as libc::c_int)).wrapping_mul(c1);
    h = ror32(h.wrapping_add(g), 19 as libc::c_int);
    h = h
        .wrapping_mul(5 as libc::c_int as libc::c_uint)
        .wrapping_add(0xe6546b64 as libc::c_uint);
    h = (ror32(h, 17 as libc::c_int)).wrapping_mul(c1);
    h = ror32(h.wrapping_add(f), 19 as libc::c_int);
    h = h
        .wrapping_mul(5 as libc::c_int as libc::c_uint)
        .wrapping_add(0xe6546b64 as libc::c_uint);
    h = (ror32(h, 17 as libc::c_int)).wrapping_mul(c1);
    return h;
}
