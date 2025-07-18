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
    fn _nettle_write_be32(length: size_t, dst: *mut uint8_t, src: *const uint32_t);
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
pub struct sm3_ctx {
    pub state: [uint32_t; 8],
    pub count: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 64],
}
static mut K: [uint32_t; 64] = [
    0x79cc4519 as libc::c_ulong as uint32_t,
    0xf3988a32 as libc::c_ulong as uint32_t,
    0xe7311465 as libc::c_ulong as uint32_t,
    0xce6228cb as libc::c_ulong as uint32_t,
    0x9cc45197 as libc::c_ulong as uint32_t,
    0x3988a32f as libc::c_ulong as uint32_t,
    0x7311465e as libc::c_ulong as uint32_t,
    0xe6228cbc as libc::c_ulong as uint32_t,
    0xcc451979 as libc::c_ulong as uint32_t,
    0x988a32f3 as libc::c_ulong as uint32_t,
    0x311465e7 as libc::c_ulong as uint32_t,
    0x6228cbce as libc::c_ulong as uint32_t,
    0xc451979c as libc::c_ulong as uint32_t,
    0x88a32f39 as libc::c_ulong as uint32_t,
    0x11465e73 as libc::c_ulong as uint32_t,
    0x228cbce6 as libc::c_ulong as uint32_t,
    0x9d8a7a87 as libc::c_ulong as uint32_t,
    0x3b14f50f as libc::c_ulong as uint32_t,
    0x7629ea1e as libc::c_ulong as uint32_t,
    0xec53d43c as libc::c_ulong as uint32_t,
    0xd8a7a879 as libc::c_ulong as uint32_t,
    0xb14f50f3 as libc::c_ulong as uint32_t,
    0x629ea1e7 as libc::c_ulong as uint32_t,
    0xc53d43ce as libc::c_ulong as uint32_t,
    0x8a7a879d as libc::c_ulong as uint32_t,
    0x14f50f3b as libc::c_ulong as uint32_t,
    0x29ea1e76 as libc::c_ulong as uint32_t,
    0x53d43cec as libc::c_ulong as uint32_t,
    0xa7a879d8 as libc::c_ulong as uint32_t,
    0x4f50f3b1 as libc::c_ulong as uint32_t,
    0x9ea1e762 as libc::c_ulong as uint32_t,
    0x3d43cec5 as libc::c_ulong as uint32_t,
    0x7a879d8a as libc::c_ulong as uint32_t,
    0xf50f3b14 as libc::c_ulong as uint32_t,
    0xea1e7629 as libc::c_ulong as uint32_t,
    0xd43cec53 as libc::c_ulong as uint32_t,
    0xa879d8a7 as libc::c_ulong as uint32_t,
    0x50f3b14f as libc::c_ulong as uint32_t,
    0xa1e7629e as libc::c_ulong as uint32_t,
    0x43cec53d as libc::c_ulong as uint32_t,
    0x879d8a7a as libc::c_ulong as uint32_t,
    0xf3b14f5 as libc::c_ulong as uint32_t,
    0x1e7629ea as libc::c_ulong as uint32_t,
    0x3cec53d4 as libc::c_ulong as uint32_t,
    0x79d8a7a8 as libc::c_ulong as uint32_t,
    0xf3b14f50 as libc::c_ulong as uint32_t,
    0xe7629ea1 as libc::c_ulong as uint32_t,
    0xcec53d43 as libc::c_ulong as uint32_t,
    0x9d8a7a87 as libc::c_ulong as uint32_t,
    0x3b14f50f as libc::c_ulong as uint32_t,
    0x7629ea1e as libc::c_ulong as uint32_t,
    0xec53d43c as libc::c_ulong as uint32_t,
    0xd8a7a879 as libc::c_ulong as uint32_t,
    0xb14f50f3 as libc::c_ulong as uint32_t,
    0x629ea1e7 as libc::c_ulong as uint32_t,
    0xc53d43ce as libc::c_ulong as uint32_t,
    0x8a7a879d as libc::c_ulong as uint32_t,
    0x14f50f3b as libc::c_ulong as uint32_t,
    0x29ea1e76 as libc::c_ulong as uint32_t,
    0x53d43cec as libc::c_ulong as uint32_t,
    0xa7a879d8 as libc::c_ulong as uint32_t,
    0x4f50f3b1 as libc::c_ulong as uint32_t,
    0x9ea1e762 as libc::c_ulong as uint32_t,
    0x3d43cec5 as libc::c_ulong as uint32_t,
];
unsafe extern "C" fn sm3_compress(mut state: *mut uint32_t, mut input: *const uint8_t) {
    let mut a: uint32_t = 0;
    let mut b: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut d: uint32_t = 0;
    let mut e: uint32_t = 0;
    let mut f: uint32_t = 0;
    let mut g: uint32_t = 0;
    let mut h: uint32_t = 0;
    let mut ss1: uint32_t = 0;
    let mut ss2: uint32_t = 0;
    let mut w: [uint32_t; 16] = [0; 16];
    a = *state.offset(0 as libc::c_int as isize);
    b = *state.offset(1 as libc::c_int as isize);
    c = *state.offset(2 as libc::c_int as isize);
    d = *state.offset(3 as libc::c_int as isize);
    e = *state.offset(4 as libc::c_int as isize);
    f = *state.offset(5 as libc::c_int as isize);
    g = *state.offset(6 as libc::c_int as isize);
    h = *state.offset(7 as libc::c_int as isize);
    ss1 = (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e)
        .wrapping_add(K[0 as libc::c_int as usize]) << 7 as libc::c_int
        | (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(K[0 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[0 as libc::c_int
        as usize] = (*input
        .offset((0 as libc::c_int * 4 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*input
            .offset((0 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*input
            .offset((0 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | *input
            .offset((0 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(3 as libc::c_int as isize) as uint32_t;
    w[4 as libc::c_int
        as usize] = (*input
        .offset((4 as libc::c_int * 4 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*input
            .offset((4 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*input
            .offset((4 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | *input
            .offset((4 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(3 as libc::c_int as isize) as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[0 as libc::c_int as usize] ^ w[4 as libc::c_int as usize],
                ),
        ) as uint32_t as uint32_t;
    w[0 as libc::c_int
        as usize] = (*input
        .offset((0 as libc::c_int * 4 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*input
            .offset((0 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*input
            .offset((0 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | *input
            .offset((0 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(3 as libc::c_int as isize) as uint32_t;
    h = (h as libc::c_uint)
        .wrapping_add(
            (e ^ f ^ g).wrapping_add(ss1).wrapping_add(w[0 as libc::c_int as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as libc::c_int | b >> (-(9 as libc::c_int) & 31 as libc::c_int);
    f = f << 19 as libc::c_int | f >> (-(19 as libc::c_int) & 31 as libc::c_int);
    h = h ^ (h << 9 as libc::c_int | h >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (h << 17 as libc::c_int | h >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(h)
        .wrapping_add(K[1 as libc::c_int as usize]) << 7 as libc::c_int
        | (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(K[1 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[1 as libc::c_int
        as usize] = (*input
        .offset((1 as libc::c_int * 4 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*input
            .offset((1 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*input
            .offset((1 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | *input
            .offset((1 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(3 as libc::c_int as isize) as uint32_t;
    w[5 as libc::c_int
        as usize] = (*input
        .offset((5 as libc::c_int * 4 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*input
            .offset((5 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*input
            .offset((5 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | *input
            .offset((5 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(3 as libc::c_int as isize) as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[1 as libc::c_int as usize] ^ w[5 as libc::c_int as usize],
                ),
        ) as uint32_t as uint32_t;
    w[1 as libc::c_int
        as usize] = (*input
        .offset((1 as libc::c_int * 4 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*input
            .offset((1 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*input
            .offset((1 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | *input
            .offset((1 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(3 as libc::c_int as isize) as uint32_t;
    g = (g as libc::c_uint)
        .wrapping_add(
            (h ^ e ^ f).wrapping_add(ss1).wrapping_add(w[1 as libc::c_int as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as libc::c_int | a >> (-(9 as libc::c_int) & 31 as libc::c_int);
    e = e << 19 as libc::c_int | e >> (-(19 as libc::c_int) & 31 as libc::c_int);
    g = g ^ (g << 9 as libc::c_int | g >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (g << 17 as libc::c_int | g >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(g)
        .wrapping_add(K[2 as libc::c_int as usize]) << 7 as libc::c_int
        | (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(K[2 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[2 as libc::c_int
        as usize] = (*input
        .offset((2 as libc::c_int * 4 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*input
            .offset((2 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*input
            .offset((2 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | *input
            .offset((2 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(3 as libc::c_int as isize) as uint32_t;
    w[6 as libc::c_int
        as usize] = (*input
        .offset((6 as libc::c_int * 4 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*input
            .offset((6 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*input
            .offset((6 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | *input
            .offset((6 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(3 as libc::c_int as isize) as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[2 as libc::c_int as usize] ^ w[6 as libc::c_int as usize],
                ),
        ) as uint32_t as uint32_t;
    w[2 as libc::c_int
        as usize] = (*input
        .offset((2 as libc::c_int * 4 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*input
            .offset((2 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*input
            .offset((2 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | *input
            .offset((2 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(3 as libc::c_int as isize) as uint32_t;
    f = (f as libc::c_uint)
        .wrapping_add(
            (g ^ h ^ e).wrapping_add(ss1).wrapping_add(w[2 as libc::c_int as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> (-(9 as libc::c_int) & 31 as libc::c_int);
    h = h << 19 as libc::c_int | h >> (-(19 as libc::c_int) & 31 as libc::c_int);
    f = f ^ (f << 9 as libc::c_int | f >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (f << 17 as libc::c_int | f >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(f)
        .wrapping_add(K[3 as libc::c_int as usize]) << 7 as libc::c_int
        | (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(K[3 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[3 as libc::c_int
        as usize] = (*input
        .offset((3 as libc::c_int * 4 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*input
            .offset((3 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*input
            .offset((3 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | *input
            .offset((3 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(3 as libc::c_int as isize) as uint32_t;
    w[7 as libc::c_int
        as usize] = (*input
        .offset((7 as libc::c_int * 4 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*input
            .offset((7 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*input
            .offset((7 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | *input
            .offset((7 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(3 as libc::c_int as isize) as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[3 as libc::c_int as usize] ^ w[7 as libc::c_int as usize],
                ),
        ) as uint32_t as uint32_t;
    w[3 as libc::c_int
        as usize] = (*input
        .offset((3 as libc::c_int * 4 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*input
            .offset((3 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*input
            .offset((3 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | *input
            .offset((3 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(3 as libc::c_int as isize) as uint32_t;
    e = (e as libc::c_uint)
        .wrapping_add(
            (f ^ g ^ h).wrapping_add(ss1).wrapping_add(w[3 as libc::c_int as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as libc::c_int | c >> (-(9 as libc::c_int) & 31 as libc::c_int);
    g = g << 19 as libc::c_int | g >> (-(19 as libc::c_int) & 31 as libc::c_int);
    e = e ^ (e << 9 as libc::c_int | e >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (e << 17 as libc::c_int | e >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e)
        .wrapping_add(K[4 as libc::c_int as usize]) << 7 as libc::c_int
        | (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(K[4 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[8 as libc::c_int
        as usize] = (*input
        .offset((8 as libc::c_int * 4 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*input
            .offset((8 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*input
            .offset((8 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | *input
            .offset((8 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(3 as libc::c_int as isize) as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(4 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[8 as libc::c_int as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as libc::c_uint)
        .wrapping_add(
            (e ^ f ^ g)
                .wrapping_add(ss1)
                .wrapping_add(w[(4 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as libc::c_int | b >> (-(9 as libc::c_int) & 31 as libc::c_int);
    f = f << 19 as libc::c_int | f >> (-(19 as libc::c_int) & 31 as libc::c_int);
    h = h ^ (h << 9 as libc::c_int | h >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (h << 17 as libc::c_int | h >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(h)
        .wrapping_add(K[5 as libc::c_int as usize]) << 7 as libc::c_int
        | (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(K[5 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[9 as libc::c_int
        as usize] = (*input
        .offset((9 as libc::c_int * 4 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*input
            .offset((9 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*input
            .offset((9 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | *input
            .offset((9 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(3 as libc::c_int as isize) as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(5 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[9 as libc::c_int as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as libc::c_uint)
        .wrapping_add(
            (h ^ e ^ f)
                .wrapping_add(ss1)
                .wrapping_add(w[(5 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as libc::c_int | a >> (-(9 as libc::c_int) & 31 as libc::c_int);
    e = e << 19 as libc::c_int | e >> (-(19 as libc::c_int) & 31 as libc::c_int);
    g = g ^ (g << 9 as libc::c_int | g >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (g << 17 as libc::c_int | g >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(g)
        .wrapping_add(K[6 as libc::c_int as usize]) << 7 as libc::c_int
        | (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(K[6 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[10 as libc::c_int
        as usize] = (*input
        .offset((10 as libc::c_int * 4 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*input
            .offset((10 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*input
            .offset((10 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | *input
            .offset((10 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(3 as libc::c_int as isize) as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(6 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[10 as libc::c_int as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as libc::c_uint)
        .wrapping_add(
            (g ^ h ^ e)
                .wrapping_add(ss1)
                .wrapping_add(w[(6 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> (-(9 as libc::c_int) & 31 as libc::c_int);
    h = h << 19 as libc::c_int | h >> (-(19 as libc::c_int) & 31 as libc::c_int);
    f = f ^ (f << 9 as libc::c_int | f >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (f << 17 as libc::c_int | f >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(f)
        .wrapping_add(K[7 as libc::c_int as usize]) << 7 as libc::c_int
        | (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(K[7 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[11 as libc::c_int
        as usize] = (*input
        .offset((11 as libc::c_int * 4 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*input
            .offset((11 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*input
            .offset((11 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | *input
            .offset((11 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(3 as libc::c_int as isize) as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(7 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[11 as libc::c_int as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as libc::c_uint)
        .wrapping_add(
            (f ^ g ^ h)
                .wrapping_add(ss1)
                .wrapping_add(w[(7 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as libc::c_int | c >> (-(9 as libc::c_int) & 31 as libc::c_int);
    g = g << 19 as libc::c_int | g >> (-(19 as libc::c_int) & 31 as libc::c_int);
    e = e ^ (e << 9 as libc::c_int | e >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (e << 17 as libc::c_int | e >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e)
        .wrapping_add(K[8 as libc::c_int as usize]) << 7 as libc::c_int
        | (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(K[8 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[12 as libc::c_int
        as usize] = (*input
        .offset((12 as libc::c_int * 4 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*input
            .offset((12 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*input
            .offset((12 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | *input
            .offset((12 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(3 as libc::c_int as isize) as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(8 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[12 as libc::c_int as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as libc::c_uint)
        .wrapping_add(
            (e ^ f ^ g)
                .wrapping_add(ss1)
                .wrapping_add(w[(8 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as libc::c_int | b >> (-(9 as libc::c_int) & 31 as libc::c_int);
    f = f << 19 as libc::c_int | f >> (-(19 as libc::c_int) & 31 as libc::c_int);
    h = h ^ (h << 9 as libc::c_int | h >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (h << 17 as libc::c_int | h >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(h)
        .wrapping_add(K[9 as libc::c_int as usize]) << 7 as libc::c_int
        | (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(K[9 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[13 as libc::c_int
        as usize] = (*input
        .offset((13 as libc::c_int * 4 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*input
            .offset((13 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*input
            .offset((13 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | *input
            .offset((13 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(3 as libc::c_int as isize) as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(9 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[13 as libc::c_int as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as libc::c_uint)
        .wrapping_add(
            (h ^ e ^ f)
                .wrapping_add(ss1)
                .wrapping_add(w[(9 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as libc::c_int | a >> (-(9 as libc::c_int) & 31 as libc::c_int);
    e = e << 19 as libc::c_int | e >> (-(19 as libc::c_int) & 31 as libc::c_int);
    g = g ^ (g << 9 as libc::c_int | g >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (g << 17 as libc::c_int | g >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(g)
        .wrapping_add(K[10 as libc::c_int as usize]) << 7 as libc::c_int
        | (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(K[10 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[14 as libc::c_int
        as usize] = (*input
        .offset((14 as libc::c_int * 4 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*input
            .offset((14 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*input
            .offset((14 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | *input
            .offset((14 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(3 as libc::c_int as isize) as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(10 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[14 as libc::c_int as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as libc::c_uint)
        .wrapping_add(
            (g ^ h ^ e)
                .wrapping_add(ss1)
                .wrapping_add(w[(10 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> (-(9 as libc::c_int) & 31 as libc::c_int);
    h = h << 19 as libc::c_int | h >> (-(19 as libc::c_int) & 31 as libc::c_int);
    f = f ^ (f << 9 as libc::c_int | f >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (f << 17 as libc::c_int | f >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(f)
        .wrapping_add(K[11 as libc::c_int as usize]) << 7 as libc::c_int
        | (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(K[11 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[15 as libc::c_int
        as usize] = (*input
        .offset((15 as libc::c_int * 4 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
        | (*input
            .offset((15 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*input
            .offset((15 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | *input
            .offset((15 as libc::c_int * 4 as libc::c_int) as isize)
            .offset(3 as libc::c_int as isize) as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(11 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[15 as libc::c_int as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as libc::c_uint)
        .wrapping_add(
            (f ^ g ^ h)
                .wrapping_add(ss1)
                .wrapping_add(w[(11 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as libc::c_int | c >> (-(9 as libc::c_int) & 31 as libc::c_int);
    g = g << 19 as libc::c_int | g >> (-(19 as libc::c_int) & 31 as libc::c_int);
    e = e ^ (e << 9 as libc::c_int | e >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (e << 17 as libc::c_int | e >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e)
        .wrapping_add(K[12 as libc::c_int as usize]) << 7 as libc::c_int
        | (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(K[12 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(16 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(16 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(16 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(16 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(16 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(16 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(16 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(16 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(16 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(16 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(16 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(16 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(16 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(16 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(16 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(16 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(16 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(16 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(16 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(16 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(16 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(16 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(16 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(16 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(12 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(16 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as libc::c_uint)
        .wrapping_add(
            (e ^ f ^ g)
                .wrapping_add(ss1)
                .wrapping_add(w[(12 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as libc::c_int | b >> (-(9 as libc::c_int) & 31 as libc::c_int);
    f = f << 19 as libc::c_int | f >> (-(19 as libc::c_int) & 31 as libc::c_int);
    h = h ^ (h << 9 as libc::c_int | h >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (h << 17 as libc::c_int | h >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(h)
        .wrapping_add(K[13 as libc::c_int as usize]) << 7 as libc::c_int
        | (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(K[13 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(17 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(17 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(17 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(17 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(17 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(17 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(17 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(17 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(17 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(17 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(17 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(17 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(17 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(17 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(17 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(17 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(17 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(17 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(17 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(17 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(17 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(17 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(17 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(17 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    c = (c as libc::c_uint)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(13 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(17 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as libc::c_uint)
        .wrapping_add(
            (h ^ e ^ f)
                .wrapping_add(ss1)
                .wrapping_add(w[(13 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as libc::c_int | a >> (-(9 as libc::c_int) & 31 as libc::c_int);
    e = e << 19 as libc::c_int | e >> (-(19 as libc::c_int) & 31 as libc::c_int);
    g = g ^ (g << 9 as libc::c_int | g >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (g << 17 as libc::c_int | g >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(g)
        .wrapping_add(K[14 as libc::c_int as usize]) << 7 as libc::c_int
        | (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(K[14 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(18 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(18 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(18 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(18 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(18 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(18 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(18 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(18 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(18 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(18 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(18 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(18 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(18 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(18 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(18 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(18 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(18 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(18 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(18 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(18 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(18 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(18 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(18 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(18 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    b = (b as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(14 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(18 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as libc::c_uint)
        .wrapping_add(
            (g ^ h ^ e)
                .wrapping_add(ss1)
                .wrapping_add(w[(14 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> (-(9 as libc::c_int) & 31 as libc::c_int);
    h = h << 19 as libc::c_int | h >> (-(19 as libc::c_int) & 31 as libc::c_int);
    f = f ^ (f << 9 as libc::c_int | f >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (f << 17 as libc::c_int | f >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(f)
        .wrapping_add(K[15 as libc::c_int as usize]) << 7 as libc::c_int
        | (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(K[15 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(19 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(19 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(19 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(19 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(19 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(19 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(19 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(19 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(19 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(19 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(19 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(19 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(19 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(19 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(19 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(19 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(19 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(19 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(19 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(19 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(19 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(19 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(19 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(19 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    a = (a as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(15 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(19 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as libc::c_uint)
        .wrapping_add(
            (f ^ g ^ h)
                .wrapping_add(ss1)
                .wrapping_add(w[(15 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as libc::c_int | c >> (-(9 as libc::c_int) & 31 as libc::c_int);
    g = g << 19 as libc::c_int | g >> (-(19 as libc::c_int) & 31 as libc::c_int);
    e = e ^ (e << 9 as libc::c_int | e >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (e << 17 as libc::c_int | e >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e)
        .wrapping_add(K[16 as libc::c_int as usize]) << 7 as libc::c_int
        | (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(K[16 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(20 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(20 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(20 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(20 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(20 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(20 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(20 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(20 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(20 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(20 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(20 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(20 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(20 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(20 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(20 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(20 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(20 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(20 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(20 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(20 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(20 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(20 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(20 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(20 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(16 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(20 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as libc::c_uint)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(16 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as libc::c_int | b >> (-(9 as libc::c_int) & 31 as libc::c_int);
    f = f << 19 as libc::c_int | f >> (-(19 as libc::c_int) & 31 as libc::c_int);
    h = h ^ (h << 9 as libc::c_int | h >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (h << 17 as libc::c_int | h >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(h)
        .wrapping_add(K[17 as libc::c_int as usize]) << 7 as libc::c_int
        | (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(K[17 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(21 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(21 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(21 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(21 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(21 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(21 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(21 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(21 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(21 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(21 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(21 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(21 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(21 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(21 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(21 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(21 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(21 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(21 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(21 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(21 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(21 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(21 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(21 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(21 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    c = (c as libc::c_uint)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(17 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(21 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as libc::c_uint)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(17 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as libc::c_int | a >> (-(9 as libc::c_int) & 31 as libc::c_int);
    e = e << 19 as libc::c_int | e >> (-(19 as libc::c_int) & 31 as libc::c_int);
    g = g ^ (g << 9 as libc::c_int | g >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (g << 17 as libc::c_int | g >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(g)
        .wrapping_add(K[18 as libc::c_int as usize]) << 7 as libc::c_int
        | (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(K[18 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(22 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(22 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(22 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(22 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(22 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(22 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(22 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(22 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(22 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(22 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(22 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(22 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(22 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(22 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(22 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(22 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(22 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(22 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(22 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(22 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(22 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(22 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(22 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(22 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    b = (b as libc::c_uint)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(18 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(22 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as libc::c_uint)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(18 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> (-(9 as libc::c_int) & 31 as libc::c_int);
    h = h << 19 as libc::c_int | h >> (-(19 as libc::c_int) & 31 as libc::c_int);
    f = f ^ (f << 9 as libc::c_int | f >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (f << 17 as libc::c_int | f >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(f)
        .wrapping_add(K[19 as libc::c_int as usize]) << 7 as libc::c_int
        | (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(K[19 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(23 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(23 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(23 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(23 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(23 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(23 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(23 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(23 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(23 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(23 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(23 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(23 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(23 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(23 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(23 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(23 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(23 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(23 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(23 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(23 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(23 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(23 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(23 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(23 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    a = (a as libc::c_uint)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(19 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(23 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as libc::c_uint)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(19 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as libc::c_int | c >> (-(9 as libc::c_int) & 31 as libc::c_int);
    g = g << 19 as libc::c_int | g >> (-(19 as libc::c_int) & 31 as libc::c_int);
    e = e ^ (e << 9 as libc::c_int | e >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (e << 17 as libc::c_int | e >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e)
        .wrapping_add(K[20 as libc::c_int as usize]) << 7 as libc::c_int
        | (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(K[20 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(24 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(24 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(24 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(24 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(24 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(24 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(24 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(24 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(24 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(24 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(24 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(24 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(24 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(24 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(24 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(24 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(24 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(24 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(24 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(24 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(24 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(24 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(24 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(24 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(20 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(24 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as libc::c_uint)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(20 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as libc::c_int | b >> (-(9 as libc::c_int) & 31 as libc::c_int);
    f = f << 19 as libc::c_int | f >> (-(19 as libc::c_int) & 31 as libc::c_int);
    h = h ^ (h << 9 as libc::c_int | h >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (h << 17 as libc::c_int | h >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(h)
        .wrapping_add(K[21 as libc::c_int as usize]) << 7 as libc::c_int
        | (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(K[21 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(25 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(25 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(25 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(25 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(25 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(25 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(25 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(25 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(25 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(25 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(25 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(25 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(25 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(25 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(25 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(25 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(25 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(25 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(25 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(25 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(25 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(25 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(25 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(25 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    c = (c as libc::c_uint)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(21 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(25 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as libc::c_uint)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(21 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as libc::c_int | a >> (-(9 as libc::c_int) & 31 as libc::c_int);
    e = e << 19 as libc::c_int | e >> (-(19 as libc::c_int) & 31 as libc::c_int);
    g = g ^ (g << 9 as libc::c_int | g >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (g << 17 as libc::c_int | g >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(g)
        .wrapping_add(K[22 as libc::c_int as usize]) << 7 as libc::c_int
        | (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(K[22 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(26 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(26 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(26 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(26 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(26 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(26 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(26 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(26 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(26 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(26 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(26 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(26 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(26 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(26 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(26 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(26 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(26 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(26 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(26 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(26 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(26 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(26 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(26 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(26 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    b = (b as libc::c_uint)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(22 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(26 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as libc::c_uint)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(22 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> (-(9 as libc::c_int) & 31 as libc::c_int);
    h = h << 19 as libc::c_int | h >> (-(19 as libc::c_int) & 31 as libc::c_int);
    f = f ^ (f << 9 as libc::c_int | f >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (f << 17 as libc::c_int | f >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(f)
        .wrapping_add(K[23 as libc::c_int as usize]) << 7 as libc::c_int
        | (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(K[23 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(27 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(27 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(27 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(27 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(27 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(27 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(27 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(27 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(27 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(27 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(27 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(27 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(27 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(27 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(27 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(27 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(27 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(27 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(27 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(27 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(27 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(27 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(27 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(27 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    a = (a as libc::c_uint)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(23 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(27 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as libc::c_uint)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(23 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as libc::c_int | c >> (-(9 as libc::c_int) & 31 as libc::c_int);
    g = g << 19 as libc::c_int | g >> (-(19 as libc::c_int) & 31 as libc::c_int);
    e = e ^ (e << 9 as libc::c_int | e >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (e << 17 as libc::c_int | e >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e)
        .wrapping_add(K[24 as libc::c_int as usize]) << 7 as libc::c_int
        | (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(K[24 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(28 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(28 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(28 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(28 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(28 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(28 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(28 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(28 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(28 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(28 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(28 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(28 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(28 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(28 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(28 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(28 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(28 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(28 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(28 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(28 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(28 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(28 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(28 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(28 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(24 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(28 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as libc::c_uint)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(24 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as libc::c_int | b >> (-(9 as libc::c_int) & 31 as libc::c_int);
    f = f << 19 as libc::c_int | f >> (-(19 as libc::c_int) & 31 as libc::c_int);
    h = h ^ (h << 9 as libc::c_int | h >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (h << 17 as libc::c_int | h >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(h)
        .wrapping_add(K[25 as libc::c_int as usize]) << 7 as libc::c_int
        | (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(K[25 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(29 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(29 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(29 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(29 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(29 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(29 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(29 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(29 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(29 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(29 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(29 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(29 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(29 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(29 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(29 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(29 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(29 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(29 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(29 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(29 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(29 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(29 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(29 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(29 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    c = (c as libc::c_uint)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(25 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(29 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as libc::c_uint)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(25 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as libc::c_int | a >> (-(9 as libc::c_int) & 31 as libc::c_int);
    e = e << 19 as libc::c_int | e >> (-(19 as libc::c_int) & 31 as libc::c_int);
    g = g ^ (g << 9 as libc::c_int | g >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (g << 17 as libc::c_int | g >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(g)
        .wrapping_add(K[26 as libc::c_int as usize]) << 7 as libc::c_int
        | (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(K[26 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(30 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(30 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(30 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(30 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(30 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(30 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(30 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(30 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(30 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(30 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(30 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(30 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(30 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(30 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(30 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(30 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(30 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(30 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(30 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(30 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(30 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(30 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(30 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(30 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    b = (b as libc::c_uint)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(26 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(30 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as libc::c_uint)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(26 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> (-(9 as libc::c_int) & 31 as libc::c_int);
    h = h << 19 as libc::c_int | h >> (-(19 as libc::c_int) & 31 as libc::c_int);
    f = f ^ (f << 9 as libc::c_int | f >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (f << 17 as libc::c_int | f >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(f)
        .wrapping_add(K[27 as libc::c_int as usize]) << 7 as libc::c_int
        | (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(K[27 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(31 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(31 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(31 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(31 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(31 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(31 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(31 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(31 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(31 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(31 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(31 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(31 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(31 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(31 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(31 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(31 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(31 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(31 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(31 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(31 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(31 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(31 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(31 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(31 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    a = (a as libc::c_uint)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(27 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(31 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as libc::c_uint)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(27 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as libc::c_int | c >> (-(9 as libc::c_int) & 31 as libc::c_int);
    g = g << 19 as libc::c_int | g >> (-(19 as libc::c_int) & 31 as libc::c_int);
    e = e ^ (e << 9 as libc::c_int | e >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (e << 17 as libc::c_int | e >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e)
        .wrapping_add(K[28 as libc::c_int as usize]) << 7 as libc::c_int
        | (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(K[28 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(32 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(32 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(32 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(32 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(32 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(32 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(32 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(32 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(32 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(32 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(32 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(32 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(32 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(32 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(32 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(32 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(32 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(32 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(32 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(32 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(32 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(32 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(32 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(32 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(28 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(32 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as libc::c_uint)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(28 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as libc::c_int | b >> (-(9 as libc::c_int) & 31 as libc::c_int);
    f = f << 19 as libc::c_int | f >> (-(19 as libc::c_int) & 31 as libc::c_int);
    h = h ^ (h << 9 as libc::c_int | h >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (h << 17 as libc::c_int | h >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(h)
        .wrapping_add(K[29 as libc::c_int as usize]) << 7 as libc::c_int
        | (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(K[29 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(33 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(33 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(33 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(33 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(33 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(33 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(33 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(33 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(33 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(33 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(33 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(33 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(33 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(33 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(33 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(33 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(33 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(33 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(33 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(33 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(33 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(33 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(33 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(33 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    c = (c as libc::c_uint)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(29 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(33 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as libc::c_uint)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(29 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as libc::c_int | a >> (-(9 as libc::c_int) & 31 as libc::c_int);
    e = e << 19 as libc::c_int | e >> (-(19 as libc::c_int) & 31 as libc::c_int);
    g = g ^ (g << 9 as libc::c_int | g >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (g << 17 as libc::c_int | g >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(g)
        .wrapping_add(K[30 as libc::c_int as usize]) << 7 as libc::c_int
        | (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(K[30 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(34 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(34 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(34 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(34 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(34 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(34 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(34 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(34 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(34 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(34 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(34 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(34 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(34 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(34 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(34 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(34 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(34 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(34 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(34 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(34 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(34 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(34 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(34 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(34 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    b = (b as libc::c_uint)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(30 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(34 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as libc::c_uint)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(30 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> (-(9 as libc::c_int) & 31 as libc::c_int);
    h = h << 19 as libc::c_int | h >> (-(19 as libc::c_int) & 31 as libc::c_int);
    f = f ^ (f << 9 as libc::c_int | f >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (f << 17 as libc::c_int | f >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(f)
        .wrapping_add(K[31 as libc::c_int as usize]) << 7 as libc::c_int
        | (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(K[31 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(35 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(35 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(35 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(35 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(35 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(35 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(35 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(35 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(35 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(35 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(35 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(35 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(35 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(35 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(35 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(35 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(35 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(35 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(35 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(35 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(35 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(35 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(35 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(35 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    a = (a as libc::c_uint)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(31 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(35 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as libc::c_uint)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(31 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as libc::c_int | c >> (-(9 as libc::c_int) & 31 as libc::c_int);
    g = g << 19 as libc::c_int | g >> (-(19 as libc::c_int) & 31 as libc::c_int);
    e = e ^ (e << 9 as libc::c_int | e >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (e << 17 as libc::c_int | e >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e)
        .wrapping_add(K[32 as libc::c_int as usize]) << 7 as libc::c_int
        | (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(K[32 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(36 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(36 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(36 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(36 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(36 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(36 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(36 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(36 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(36 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(36 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(36 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(36 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(36 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(36 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(36 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(36 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(36 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(36 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(36 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(36 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(36 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(36 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(36 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(36 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(32 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(36 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as libc::c_uint)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(32 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as libc::c_int | b >> (-(9 as libc::c_int) & 31 as libc::c_int);
    f = f << 19 as libc::c_int | f >> (-(19 as libc::c_int) & 31 as libc::c_int);
    h = h ^ (h << 9 as libc::c_int | h >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (h << 17 as libc::c_int | h >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(h)
        .wrapping_add(K[33 as libc::c_int as usize]) << 7 as libc::c_int
        | (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(K[33 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(37 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(37 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(37 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(37 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(37 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(37 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(37 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(37 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(37 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(37 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(37 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(37 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(37 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(37 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(37 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(37 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(37 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(37 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(37 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(37 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(37 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(37 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(37 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(37 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    c = (c as libc::c_uint)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(33 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(37 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as libc::c_uint)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(33 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as libc::c_int | a >> (-(9 as libc::c_int) & 31 as libc::c_int);
    e = e << 19 as libc::c_int | e >> (-(19 as libc::c_int) & 31 as libc::c_int);
    g = g ^ (g << 9 as libc::c_int | g >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (g << 17 as libc::c_int | g >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(g)
        .wrapping_add(K[34 as libc::c_int as usize]) << 7 as libc::c_int
        | (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(K[34 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(38 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(38 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(38 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(38 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(38 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(38 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(38 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(38 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(38 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(38 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(38 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(38 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(38 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(38 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(38 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(38 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(38 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(38 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(38 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(38 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(38 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(38 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(38 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(38 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    b = (b as libc::c_uint)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(34 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(38 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as libc::c_uint)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(34 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> (-(9 as libc::c_int) & 31 as libc::c_int);
    h = h << 19 as libc::c_int | h >> (-(19 as libc::c_int) & 31 as libc::c_int);
    f = f ^ (f << 9 as libc::c_int | f >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (f << 17 as libc::c_int | f >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(f)
        .wrapping_add(K[35 as libc::c_int as usize]) << 7 as libc::c_int
        | (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(K[35 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(39 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(39 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(39 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(39 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(39 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(39 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(39 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(39 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(39 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(39 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(39 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(39 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(39 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(39 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(39 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(39 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(39 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(39 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(39 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(39 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(39 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(39 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(39 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(39 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    a = (a as libc::c_uint)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(35 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(39 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as libc::c_uint)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(35 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as libc::c_int | c >> (-(9 as libc::c_int) & 31 as libc::c_int);
    g = g << 19 as libc::c_int | g >> (-(19 as libc::c_int) & 31 as libc::c_int);
    e = e ^ (e << 9 as libc::c_int | e >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (e << 17 as libc::c_int | e >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e)
        .wrapping_add(K[36 as libc::c_int as usize]) << 7 as libc::c_int
        | (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(K[36 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(40 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(40 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(40 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(40 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(40 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(40 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(40 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(40 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(40 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(40 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(40 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(40 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(40 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(40 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(40 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(40 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(40 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(40 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(40 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(40 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(40 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(40 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(40 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(40 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(36 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(40 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as libc::c_uint)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(36 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as libc::c_int | b >> (-(9 as libc::c_int) & 31 as libc::c_int);
    f = f << 19 as libc::c_int | f >> (-(19 as libc::c_int) & 31 as libc::c_int);
    h = h ^ (h << 9 as libc::c_int | h >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (h << 17 as libc::c_int | h >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(h)
        .wrapping_add(K[37 as libc::c_int as usize]) << 7 as libc::c_int
        | (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(K[37 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(41 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(41 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(41 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(41 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(41 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(41 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(41 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(41 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(41 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(41 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(41 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(41 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(41 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(41 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(41 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(41 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(41 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(41 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(41 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(41 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(41 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(41 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(41 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(41 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    c = (c as libc::c_uint)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(37 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(41 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as libc::c_uint)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(37 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as libc::c_int | a >> (-(9 as libc::c_int) & 31 as libc::c_int);
    e = e << 19 as libc::c_int | e >> (-(19 as libc::c_int) & 31 as libc::c_int);
    g = g ^ (g << 9 as libc::c_int | g >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (g << 17 as libc::c_int | g >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(g)
        .wrapping_add(K[38 as libc::c_int as usize]) << 7 as libc::c_int
        | (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(K[38 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(42 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(42 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(42 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(42 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(42 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(42 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(42 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(42 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(42 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(42 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(42 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(42 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(42 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(42 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(42 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(42 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(42 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(42 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(42 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(42 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(42 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(42 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(42 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(42 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    b = (b as libc::c_uint)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(38 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(42 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as libc::c_uint)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(38 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> (-(9 as libc::c_int) & 31 as libc::c_int);
    h = h << 19 as libc::c_int | h >> (-(19 as libc::c_int) & 31 as libc::c_int);
    f = f ^ (f << 9 as libc::c_int | f >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (f << 17 as libc::c_int | f >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(f)
        .wrapping_add(K[39 as libc::c_int as usize]) << 7 as libc::c_int
        | (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(K[39 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(43 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(43 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(43 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(43 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(43 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(43 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(43 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(43 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(43 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(43 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(43 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(43 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(43 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(43 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(43 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(43 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(43 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(43 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(43 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(43 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(43 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(43 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(43 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(43 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    a = (a as libc::c_uint)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(39 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(43 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as libc::c_uint)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(39 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as libc::c_int | c >> (-(9 as libc::c_int) & 31 as libc::c_int);
    g = g << 19 as libc::c_int | g >> (-(19 as libc::c_int) & 31 as libc::c_int);
    e = e ^ (e << 9 as libc::c_int | e >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (e << 17 as libc::c_int | e >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e)
        .wrapping_add(K[40 as libc::c_int as usize]) << 7 as libc::c_int
        | (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(K[40 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(44 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(44 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(44 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(44 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(44 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(44 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(44 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(44 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(44 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(44 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(44 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(44 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(44 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(44 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(44 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(44 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(44 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(44 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(44 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(44 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(44 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(44 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(44 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(44 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(40 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(44 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as libc::c_uint)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(40 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as libc::c_int | b >> (-(9 as libc::c_int) & 31 as libc::c_int);
    f = f << 19 as libc::c_int | f >> (-(19 as libc::c_int) & 31 as libc::c_int);
    h = h ^ (h << 9 as libc::c_int | h >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (h << 17 as libc::c_int | h >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(h)
        .wrapping_add(K[41 as libc::c_int as usize]) << 7 as libc::c_int
        | (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(K[41 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(45 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(45 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(45 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(45 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(45 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(45 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(45 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(45 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(45 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(45 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(45 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(45 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(45 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(45 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(45 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(45 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(45 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(45 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(45 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(45 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(45 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(45 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(45 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(45 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    c = (c as libc::c_uint)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(41 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(45 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as libc::c_uint)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(41 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as libc::c_int | a >> (-(9 as libc::c_int) & 31 as libc::c_int);
    e = e << 19 as libc::c_int | e >> (-(19 as libc::c_int) & 31 as libc::c_int);
    g = g ^ (g << 9 as libc::c_int | g >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (g << 17 as libc::c_int | g >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(g)
        .wrapping_add(K[42 as libc::c_int as usize]) << 7 as libc::c_int
        | (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(K[42 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(46 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(46 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(46 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(46 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(46 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(46 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(46 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(46 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(46 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(46 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(46 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(46 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(46 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(46 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(46 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(46 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(46 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(46 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(46 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(46 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(46 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(46 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(46 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(46 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    b = (b as libc::c_uint)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(42 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(46 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as libc::c_uint)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(42 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> (-(9 as libc::c_int) & 31 as libc::c_int);
    h = h << 19 as libc::c_int | h >> (-(19 as libc::c_int) & 31 as libc::c_int);
    f = f ^ (f << 9 as libc::c_int | f >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (f << 17 as libc::c_int | f >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(f)
        .wrapping_add(K[43 as libc::c_int as usize]) << 7 as libc::c_int
        | (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(K[43 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(47 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(47 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(47 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(47 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(47 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(47 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(47 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(47 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(47 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(47 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(47 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(47 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(47 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(47 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(47 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(47 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(47 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(47 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(47 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(47 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(47 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(47 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(47 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(47 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    a = (a as libc::c_uint)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(43 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(47 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as libc::c_uint)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(43 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as libc::c_int | c >> (-(9 as libc::c_int) & 31 as libc::c_int);
    g = g << 19 as libc::c_int | g >> (-(19 as libc::c_int) & 31 as libc::c_int);
    e = e ^ (e << 9 as libc::c_int | e >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (e << 17 as libc::c_int | e >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e)
        .wrapping_add(K[44 as libc::c_int as usize]) << 7 as libc::c_int
        | (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(K[44 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(48 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(48 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(48 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(48 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(48 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(48 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(48 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(48 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(48 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(48 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(48 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(48 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(48 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(48 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(48 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(48 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(48 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(48 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(48 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(48 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(48 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(48 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(48 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(48 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(44 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(48 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as libc::c_uint)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(44 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as libc::c_int | b >> (-(9 as libc::c_int) & 31 as libc::c_int);
    f = f << 19 as libc::c_int | f >> (-(19 as libc::c_int) & 31 as libc::c_int);
    h = h ^ (h << 9 as libc::c_int | h >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (h << 17 as libc::c_int | h >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(h)
        .wrapping_add(K[45 as libc::c_int as usize]) << 7 as libc::c_int
        | (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(K[45 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(49 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(49 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(49 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(49 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(49 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(49 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(49 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(49 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(49 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(49 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(49 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(49 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(49 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(49 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(49 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(49 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(49 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(49 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(49 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(49 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(49 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(49 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(49 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(49 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    c = (c as libc::c_uint)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(45 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(49 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as libc::c_uint)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(45 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as libc::c_int | a >> (-(9 as libc::c_int) & 31 as libc::c_int);
    e = e << 19 as libc::c_int | e >> (-(19 as libc::c_int) & 31 as libc::c_int);
    g = g ^ (g << 9 as libc::c_int | g >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (g << 17 as libc::c_int | g >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(g)
        .wrapping_add(K[46 as libc::c_int as usize]) << 7 as libc::c_int
        | (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(K[46 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(50 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(50 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(50 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(50 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(50 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(50 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(50 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(50 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(50 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(50 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(50 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(50 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(50 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(50 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(50 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(50 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(50 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(50 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(50 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(50 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(50 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(50 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(50 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(50 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    b = (b as libc::c_uint)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(46 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(50 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as libc::c_uint)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(46 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> (-(9 as libc::c_int) & 31 as libc::c_int);
    h = h << 19 as libc::c_int | h >> (-(19 as libc::c_int) & 31 as libc::c_int);
    f = f ^ (f << 9 as libc::c_int | f >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (f << 17 as libc::c_int | f >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(f)
        .wrapping_add(K[47 as libc::c_int as usize]) << 7 as libc::c_int
        | (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(K[47 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(51 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(51 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(51 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(51 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(51 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(51 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(51 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(51 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(51 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(51 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(51 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(51 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(51 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(51 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(51 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(51 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(51 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(51 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(51 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(51 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(51 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(51 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(51 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(51 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    a = (a as libc::c_uint)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(47 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(51 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as libc::c_uint)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(47 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as libc::c_int | c >> (-(9 as libc::c_int) & 31 as libc::c_int);
    g = g << 19 as libc::c_int | g >> (-(19 as libc::c_int) & 31 as libc::c_int);
    e = e ^ (e << 9 as libc::c_int | e >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (e << 17 as libc::c_int | e >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e)
        .wrapping_add(K[48 as libc::c_int as usize]) << 7 as libc::c_int
        | (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(K[48 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(52 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(52 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(52 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(52 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(52 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(52 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(52 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(52 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(52 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(52 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(52 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(52 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(52 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(52 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(52 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(52 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(52 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(52 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(52 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(52 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(52 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(52 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(52 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(52 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(48 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(52 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as libc::c_uint)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(48 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as libc::c_int | b >> (-(9 as libc::c_int) & 31 as libc::c_int);
    f = f << 19 as libc::c_int | f >> (-(19 as libc::c_int) & 31 as libc::c_int);
    h = h ^ (h << 9 as libc::c_int | h >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (h << 17 as libc::c_int | h >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(h)
        .wrapping_add(K[49 as libc::c_int as usize]) << 7 as libc::c_int
        | (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(K[49 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(53 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(53 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(53 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(53 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(53 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(53 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(53 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(53 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(53 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(53 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(53 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(53 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(53 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(53 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(53 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(53 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(53 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(53 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(53 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(53 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(53 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(53 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(53 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(53 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    c = (c as libc::c_uint)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(49 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(53 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as libc::c_uint)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(49 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as libc::c_int | a >> (-(9 as libc::c_int) & 31 as libc::c_int);
    e = e << 19 as libc::c_int | e >> (-(19 as libc::c_int) & 31 as libc::c_int);
    g = g ^ (g << 9 as libc::c_int | g >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (g << 17 as libc::c_int | g >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(g)
        .wrapping_add(K[50 as libc::c_int as usize]) << 7 as libc::c_int
        | (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(K[50 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(54 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(54 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(54 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(54 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(54 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(54 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(54 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(54 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(54 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(54 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(54 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(54 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(54 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(54 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(54 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(54 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(54 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(54 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(54 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(54 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(54 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(54 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(54 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(54 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    b = (b as libc::c_uint)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(50 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(54 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as libc::c_uint)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(50 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> (-(9 as libc::c_int) & 31 as libc::c_int);
    h = h << 19 as libc::c_int | h >> (-(19 as libc::c_int) & 31 as libc::c_int);
    f = f ^ (f << 9 as libc::c_int | f >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (f << 17 as libc::c_int | f >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(f)
        .wrapping_add(K[51 as libc::c_int as usize]) << 7 as libc::c_int
        | (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(K[51 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(55 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(55 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(55 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(55 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(55 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(55 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(55 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(55 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(55 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(55 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(55 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(55 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(55 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(55 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(55 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(55 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(55 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(55 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(55 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(55 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(55 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(55 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(55 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(55 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    a = (a as libc::c_uint)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(51 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(55 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as libc::c_uint)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(51 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as libc::c_int | c >> (-(9 as libc::c_int) & 31 as libc::c_int);
    g = g << 19 as libc::c_int | g >> (-(19 as libc::c_int) & 31 as libc::c_int);
    e = e ^ (e << 9 as libc::c_int | e >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (e << 17 as libc::c_int | e >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e)
        .wrapping_add(K[52 as libc::c_int as usize]) << 7 as libc::c_int
        | (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(K[52 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(56 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(56 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(56 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(56 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(56 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(56 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(56 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(56 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(56 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(56 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(56 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(56 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(56 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(56 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(56 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(56 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(56 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(56 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(56 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(56 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(56 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(56 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(56 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(56 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(52 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(56 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as libc::c_uint)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(52 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as libc::c_int | b >> (-(9 as libc::c_int) & 31 as libc::c_int);
    f = f << 19 as libc::c_int | f >> (-(19 as libc::c_int) & 31 as libc::c_int);
    h = h ^ (h << 9 as libc::c_int | h >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (h << 17 as libc::c_int | h >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(h)
        .wrapping_add(K[53 as libc::c_int as usize]) << 7 as libc::c_int
        | (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(K[53 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(57 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(57 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(57 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(57 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(57 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(57 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(57 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(57 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(57 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(57 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(57 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(57 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(57 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(57 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(57 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(57 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(57 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(57 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(57 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(57 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(57 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(57 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(57 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(57 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    c = (c as libc::c_uint)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(53 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(57 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as libc::c_uint)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(53 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as libc::c_int | a >> (-(9 as libc::c_int) & 31 as libc::c_int);
    e = e << 19 as libc::c_int | e >> (-(19 as libc::c_int) & 31 as libc::c_int);
    g = g ^ (g << 9 as libc::c_int | g >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (g << 17 as libc::c_int | g >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(g)
        .wrapping_add(K[54 as libc::c_int as usize]) << 7 as libc::c_int
        | (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(K[54 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(58 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(58 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(58 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(58 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(58 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(58 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(58 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(58 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(58 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(58 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(58 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(58 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(58 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(58 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(58 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(58 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(58 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(58 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(58 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(58 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(58 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(58 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(58 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(58 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    b = (b as libc::c_uint)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(54 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(58 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as libc::c_uint)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(54 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> (-(9 as libc::c_int) & 31 as libc::c_int);
    h = h << 19 as libc::c_int | h >> (-(19 as libc::c_int) & 31 as libc::c_int);
    f = f ^ (f << 9 as libc::c_int | f >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (f << 17 as libc::c_int | f >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(f)
        .wrapping_add(K[55 as libc::c_int as usize]) << 7 as libc::c_int
        | (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(K[55 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(59 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(59 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(59 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(59 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(59 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(59 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(59 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(59 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(59 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(59 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(59 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(59 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(59 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(59 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(59 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(59 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(59 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(59 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(59 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(59 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(59 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(59 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(59 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(59 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    a = (a as libc::c_uint)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(55 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(59 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as libc::c_uint)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(55 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as libc::c_int | c >> (-(9 as libc::c_int) & 31 as libc::c_int);
    g = g << 19 as libc::c_int | g >> (-(19 as libc::c_int) & 31 as libc::c_int);
    e = e ^ (e << 9 as libc::c_int | e >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (e << 17 as libc::c_int | e >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e)
        .wrapping_add(K[56 as libc::c_int as usize]) << 7 as libc::c_int
        | (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(K[56 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(60 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(60 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(60 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(60 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(60 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(60 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(60 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(60 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(60 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(60 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(60 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(60 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(60 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(60 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(60 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(60 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(60 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(60 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(60 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(60 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(60 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(60 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(60 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(60 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(56 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(60 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as libc::c_uint)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(56 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as libc::c_int | b >> (-(9 as libc::c_int) & 31 as libc::c_int);
    f = f << 19 as libc::c_int | f >> (-(19 as libc::c_int) & 31 as libc::c_int);
    h = h ^ (h << 9 as libc::c_int | h >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (h << 17 as libc::c_int | h >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(h)
        .wrapping_add(K[57 as libc::c_int as usize]) << 7 as libc::c_int
        | (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(K[57 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(61 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(61 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(61 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(61 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(61 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(61 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(61 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(61 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(61 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(61 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(61 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(61 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(61 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(61 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(61 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(61 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(61 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(61 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(61 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(61 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(61 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(61 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(61 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(61 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    c = (c as libc::c_uint)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(57 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(61 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as libc::c_uint)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(57 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as libc::c_int | a >> (-(9 as libc::c_int) & 31 as libc::c_int);
    e = e << 19 as libc::c_int | e >> (-(19 as libc::c_int) & 31 as libc::c_int);
    g = g ^ (g << 9 as libc::c_int | g >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (g << 17 as libc::c_int | g >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(g)
        .wrapping_add(K[58 as libc::c_int as usize]) << 7 as libc::c_int
        | (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(K[58 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(62 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(62 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(62 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(62 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(62 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(62 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(62 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(62 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(62 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(62 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(62 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(62 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(62 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(62 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(62 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(62 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(62 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(62 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(62 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(62 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(62 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(62 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(62 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(62 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    b = (b as libc::c_uint)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(58 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(62 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as libc::c_uint)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(58 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> (-(9 as libc::c_int) & 31 as libc::c_int);
    h = h << 19 as libc::c_int | h >> (-(19 as libc::c_int) & 31 as libc::c_int);
    f = f ^ (f << 9 as libc::c_int | f >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (f << 17 as libc::c_int | f >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(f)
        .wrapping_add(K[59 as libc::c_int as usize]) << 7 as libc::c_int
        | (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(K[59 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(63 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(63 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(63 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(63 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(63 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(63 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(63 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(63 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(63 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(63 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(63 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(63 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(63 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(63 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(63 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(63 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(63 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(63 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(63 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(63 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(63 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(63 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(63 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(63 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    a = (a as libc::c_uint)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(59 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(63 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as libc::c_uint)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(59 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as libc::c_int | c >> (-(9 as libc::c_int) & 31 as libc::c_int);
    g = g << 19 as libc::c_int | g >> (-(19 as libc::c_int) & 31 as libc::c_int);
    e = e ^ (e << 9 as libc::c_int | e >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (e << 17 as libc::c_int | e >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e)
        .wrapping_add(K[60 as libc::c_int as usize]) << 7 as libc::c_int
        | (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(e)
            .wrapping_add(K[60 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (a << 12 as libc::c_int | a >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(64 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(64 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(64 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(64 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(64 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(64 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(64 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(64 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(64 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(64 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(64 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(64 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(64 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(64 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(64 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(64 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(64 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(64 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(64 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(64 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(64 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(64 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(64 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(64 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    d = (d as libc::c_uint)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(60 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(64 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as libc::c_uint)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(60 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as libc::c_int | b >> (-(9 as libc::c_int) & 31 as libc::c_int);
    f = f << 19 as libc::c_int | f >> (-(19 as libc::c_int) & 31 as libc::c_int);
    h = h ^ (h << 9 as libc::c_int | h >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (h << 17 as libc::c_int | h >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(h)
        .wrapping_add(K[61 as libc::c_int as usize]) << 7 as libc::c_int
        | (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(h)
            .wrapping_add(K[61 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (d << 12 as libc::c_int | d >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(65 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(65 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(65 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(65 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(65 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(65 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(65 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(65 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(65 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(65 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(65 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(65 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(65 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(65 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(65 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(65 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(65 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(65 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(65 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(65 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(65 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(65 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(65 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(65 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    c = (c as libc::c_uint)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(61 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(65 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as libc::c_uint)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(61 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as libc::c_int | a >> (-(9 as libc::c_int) & 31 as libc::c_int);
    e = e << 19 as libc::c_int | e >> (-(19 as libc::c_int) & 31 as libc::c_int);
    g = g ^ (g << 9 as libc::c_int | g >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (g << 17 as libc::c_int | g >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(g)
        .wrapping_add(K[62 as libc::c_int as usize]) << 7 as libc::c_int
        | (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(g)
            .wrapping_add(K[62 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (c << 12 as libc::c_int | c >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(66 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(66 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(66 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(66 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(66 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(66 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(66 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(66 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(66 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(66 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(66 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(66 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(66 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(66 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(66 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(66 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(66 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(66 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(66 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(66 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(66 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(66 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(66 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(66 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    b = (b as libc::c_uint)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(62 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(66 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as libc::c_uint)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(62 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> (-(9 as libc::c_int) & 31 as libc::c_int);
    h = h << 19 as libc::c_int | h >> (-(19 as libc::c_int) & 31 as libc::c_int);
    f = f ^ (f << 9 as libc::c_int | f >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (f << 17 as libc::c_int | f >> (-(17 as libc::c_int) & 31 as libc::c_int));
    ss1 = (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(f)
        .wrapping_add(K[63 as libc::c_int as usize]) << 7 as libc::c_int
        | (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int))
            .wrapping_add(f)
            .wrapping_add(K[63 as libc::c_int as usize])
            >> (-(7 as libc::c_int) & 31 as libc::c_int);
    ss2 = ss1
        ^ (b << 12 as libc::c_int | b >> (-(12 as libc::c_int) & 31 as libc::c_int));
    w[(67 as libc::c_int & 0xf as libc::c_int)
        as usize] = w[(67 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ w[(67 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
        ^ (w[(67 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
            << 15 as libc::c_int
            | w[(67 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(67 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(67 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(67 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(67 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 15 as libc::c_int
            | (w[(67 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(67 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(67 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(67 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(15 as libc::c_int) & 31 as libc::c_int))
        ^ ((w[(67 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ w[(67 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
            ^ (w[(67 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                << 15 as libc::c_int
                | w[(67 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int) as usize]
                    >> (-(15 as libc::c_int) & 31 as libc::c_int))) << 23 as libc::c_int
            | (w[(67 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ w[(67 as libc::c_int - 9 as libc::c_int & 0xf as libc::c_int) as usize]
                ^ (w[(67 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                    as usize] << 15 as libc::c_int
                    | w[(67 as libc::c_int - 3 as libc::c_int & 0xf as libc::c_int)
                        as usize] >> (-(15 as libc::c_int) & 31 as libc::c_int)))
                >> (-(23 as libc::c_int) & 31 as libc::c_int))
        ^ (w[(67 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
            << 7 as libc::c_int
            | w[(67 as libc::c_int - 13 as libc::c_int & 0xf as libc::c_int) as usize]
                >> (-(7 as libc::c_int) & 31 as libc::c_int))
        ^ w[(67 as libc::c_int - 6 as libc::c_int & 0xf as libc::c_int) as usize];
    a = (a as libc::c_uint)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(63 as libc::c_int & 0xf as libc::c_int) as usize]
                        ^ w[(67 as libc::c_int & 0xf as libc::c_int) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as libc::c_uint)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(63 as libc::c_int & 0xf as libc::c_int) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as libc::c_int | c >> (-(9 as libc::c_int) & 31 as libc::c_int);
    g = g << 19 as libc::c_int | g >> (-(19 as libc::c_int) & 31 as libc::c_int);
    e = e ^ (e << 9 as libc::c_int | e >> (-(9 as libc::c_int) & 31 as libc::c_int))
        ^ (e << 17 as libc::c_int | e >> (-(17 as libc::c_int) & 31 as libc::c_int));
    let ref mut fresh0 = *state.offset(0 as libc::c_int as isize);
    *fresh0 ^= a;
    let ref mut fresh1 = *state.offset(1 as libc::c_int as isize);
    *fresh1 ^= b;
    let ref mut fresh2 = *state.offset(2 as libc::c_int as isize);
    *fresh2 ^= c;
    let ref mut fresh3 = *state.offset(3 as libc::c_int as isize);
    *fresh3 ^= d;
    let ref mut fresh4 = *state.offset(4 as libc::c_int as isize);
    *fresh4 ^= e;
    let ref mut fresh5 = *state.offset(5 as libc::c_int as isize);
    *fresh5 ^= f;
    let ref mut fresh6 = *state.offset(6 as libc::c_int as isize);
    *fresh6 ^= g;
    let ref mut fresh7 = *state.offset(7 as libc::c_int as isize);
    *fresh7 ^= h;
}
pub unsafe extern "C" fn nettle_sm3_init(mut ctx: *mut sm3_ctx) {
    static mut H0: [uint32_t; 8] = [
        0x7380166f as libc::c_ulong as uint32_t,
        0x4914b2b9 as libc::c_ulong as uint32_t,
        0x172442d7 as libc::c_ulong as uint32_t,
        0xda8a0600 as libc::c_ulong as uint32_t,
        0xa96f30bc as libc::c_ulong as uint32_t,
        0x163138aa as libc::c_ulong as uint32_t,
        0xe38dee4d as libc::c_ulong as uint32_t,
        0xb0fb0e4e as libc::c_ulong as uint32_t,
    ];
    memcpy(
        ((*ctx).state).as_mut_ptr() as *mut libc::c_void,
        H0.as_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint32_t; 8]>() as libc::c_ulong,
    );
    (*ctx).count = 0 as libc::c_int as uint64_t;
    (*ctx).index = 0 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn nettle_sm3_update(
    mut ctx: *mut sm3_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    let mut current_block_16: u64;
    if (*ctx).index != 0 {
        let mut __md_left: libc::c_uint = (::std::mem::size_of::<[uint8_t; 64]>()
            as libc::c_ulong)
            .wrapping_sub((*ctx).index as libc::c_ulong) as libc::c_uint;
        if length < __md_left as libc::c_ulong {
            memcpy(
                ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                    as *mut libc::c_void,
                data as *const libc::c_void,
                length,
            );
            (*ctx)
                .index = ((*ctx).index as libc::c_ulong).wrapping_add(length)
                as libc::c_uint as libc::c_uint;
            current_block_16 = 13056961889198038528;
        } else {
            memcpy(
                ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                    as *mut libc::c_void,
                data as *const libc::c_void,
                __md_left as libc::c_ulong,
            );
            sm3_compress(((*ctx).state).as_mut_ptr(), ((*ctx).block).as_mut_ptr());
            (*ctx).count = ((*ctx).count).wrapping_add(1);
            (*ctx).count;
            data = data.offset(__md_left as isize);
            length = (length as libc::c_ulong).wrapping_sub(__md_left as libc::c_ulong)
                as size_t as size_t;
            current_block_16 = 5720623009719927633;
        }
    } else {
        current_block_16 = 5720623009719927633;
    }
    match current_block_16 {
        5720623009719927633 => {
            while length >= ::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong {
                sm3_compress(((*ctx).state).as_mut_ptr(), data);
                (*ctx).count = ((*ctx).count).wrapping_add(1);
                (*ctx).count;
                data = data
                    .offset(
                        ::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong as isize,
                    );
                length = (length as libc::c_ulong)
                    .wrapping_sub(
                        ::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong,
                    ) as size_t as size_t;
            }
            memcpy(
                ((*ctx).block).as_mut_ptr() as *mut libc::c_void,
                data as *const libc::c_void,
                length,
            );
            (*ctx).index = length as libc::c_uint;
        }
        _ => {}
    };
}
unsafe extern "C" fn sm3_write_digest(
    mut ctx: *mut sm3_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    let mut bit_count: uint64_t = 0;
    if length <= 32 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length <= SM3_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
            b"sm3.c\0" as *const u8 as *const libc::c_char,
            228 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"void sm3_write_digest(struct sm3_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_41727: {
        if length <= 32 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length <= SM3_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
                b"sm3.c\0" as *const u8 as *const libc::c_char,
                228 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"void sm3_write_digest(struct sm3_ctx *, size_t, uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut __md_i: libc::c_uint = 0;
    __md_i = (*ctx).index;
    if (__md_i as libc::c_ulong)
        < ::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong
    {} else {
        __assert_fail(
            b"__md_i < sizeof((ctx)->block)\0" as *const u8 as *const libc::c_char,
            b"sm3.c\0" as *const u8 as *const libc::c_char,
            230 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"void sm3_write_digest(struct sm3_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_41667: {
        if (__md_i as libc::c_ulong)
            < ::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong
        {} else {
            __assert_fail(
                b"__md_i < sizeof((ctx)->block)\0" as *const u8 as *const libc::c_char,
                b"sm3.c\0" as *const u8 as *const libc::c_char,
                230 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"void sm3_write_digest(struct sm3_ctx *, size_t, uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    let fresh8 = __md_i;
    __md_i = __md_i.wrapping_add(1);
    (*ctx).block[fresh8 as usize] = 0x80 as libc::c_int as uint8_t;
    if __md_i as libc::c_ulong
        > (::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong)
            .wrapping_sub(8 as libc::c_int as libc::c_ulong)
    {
        memset(
            ((*ctx).block).as_mut_ptr().offset(__md_i as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong)
                .wrapping_sub(__md_i as libc::c_ulong),
        );
        sm3_compress(((*ctx).state).as_mut_ptr(), ((*ctx).block).as_mut_ptr());
        __md_i = 0 as libc::c_int as libc::c_uint;
    }
    memset(
        ((*ctx).block).as_mut_ptr().offset(__md_i as isize) as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong)
            .wrapping_sub(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(__md_i as libc::c_ulong),
    );
    bit_count = (*ctx).count << 9 as libc::c_int
        | ((*ctx).index << 3 as libc::c_int) as libc::c_ulong;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            0 as libc::c_int as isize,
        ) = (bit_count >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            1 as libc::c_int as isize,
        ) = (bit_count >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            2 as libc::c_int as isize,
        ) = (bit_count >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            3 as libc::c_int as isize,
        ) = (bit_count >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            4 as libc::c_int as isize,
        ) = (bit_count >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            5 as libc::c_int as isize,
        ) = (bit_count >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            6 as libc::c_int as isize,
        ) = (bit_count >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            7 as libc::c_int as isize,
        ) = (bit_count & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    sm3_compress(((*ctx).state).as_mut_ptr(), ((*ctx).block).as_mut_ptr());
    _nettle_write_be32(length, digest, ((*ctx).state).as_mut_ptr());
}
pub unsafe extern "C" fn nettle_sm3_digest(
    mut ctx: *mut sm3_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    sm3_write_digest(ctx, length, digest);
    nettle_sm3_init(ctx);
}
