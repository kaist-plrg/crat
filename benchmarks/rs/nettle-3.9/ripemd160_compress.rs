use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub unsafe extern "C" fn _nettle_ripemd160_compress(
    mut state: *mut uint32_t,
    mut data: *const uint8_t,
) {
    let mut a: uint32_t = 0;
    let mut b: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut d: uint32_t = 0;
    let mut e: uint32_t = 0;
    let mut aa: uint32_t = 0;
    let mut bb: uint32_t = 0;
    let mut cc: uint32_t = 0;
    let mut dd: uint32_t = 0;
    let mut ee: uint32_t = 0;
    let mut t: uint32_t = 0;
    let mut x: [uint32_t; 16] = [0; 16];
    memcpy(
        x.as_mut_ptr() as *mut libc::c_void,
        data as *const libc::c_void,
        ::std::mem::size_of::<[uint32_t; 16]>() as libc::c_ulong,
    );
    a = *state.offset(0 as libc::c_int as isize);
    b = *state.offset(1 as libc::c_int as isize);
    c = *state.offset(2 as libc::c_int as isize);
    d = *state.offset(3 as libc::c_int as isize);
    e = *state.offset(4 as libc::c_int as isize);
    t = a
        .wrapping_add(b ^ c ^ d)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[0 as libc::c_int as usize]);
    a = (t << 11 as libc::c_int | t >> (-(11 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a ^ b ^ c)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[1 as libc::c_int as usize]);
    e = (t << 14 as libc::c_int | t >> (-(14 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e ^ a ^ b)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[2 as libc::c_int as usize]);
    d = (t << 15 as libc::c_int | t >> (-(15 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d ^ e ^ a)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[3 as libc::c_int as usize]);
    c = (t << 12 as libc::c_int | t >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c ^ d ^ e)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[4 as libc::c_int as usize]);
    b = (t << 5 as libc::c_int | t >> (-(5 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b ^ c ^ d)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[5 as libc::c_int as usize]);
    a = (t << 8 as libc::c_int | t >> (-(8 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a ^ b ^ c)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[6 as libc::c_int as usize]);
    e = (t << 7 as libc::c_int | t >> (-(7 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e ^ a ^ b)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[7 as libc::c_int as usize]);
    d = (t << 9 as libc::c_int | t >> (-(9 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d ^ e ^ a)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[8 as libc::c_int as usize]);
    c = (t << 11 as libc::c_int | t >> (-(11 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c ^ d ^ e)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[9 as libc::c_int as usize]);
    b = (t << 13 as libc::c_int | t >> (-(13 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b ^ c ^ d)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[10 as libc::c_int as usize]);
    a = (t << 14 as libc::c_int | t >> (-(14 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a ^ b ^ c)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[11 as libc::c_int as usize]);
    e = (t << 15 as libc::c_int | t >> (-(15 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e ^ a ^ b)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[12 as libc::c_int as usize]);
    d = (t << 6 as libc::c_int | t >> (-(6 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d ^ e ^ a)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[13 as libc::c_int as usize]);
    c = (t << 7 as libc::c_int | t >> (-(7 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c ^ d ^ e)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[14 as libc::c_int as usize]);
    b = (t << 9 as libc::c_int | t >> (-(9 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b ^ c ^ d)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[15 as libc::c_int as usize]);
    a = (t << 8 as libc::c_int | t >> (-(8 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a & b | !a & c)
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        .wrapping_add(x[7 as libc::c_int as usize]);
    e = (t << 7 as libc::c_int | t >> (-(7 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e & a | !e & b)
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        .wrapping_add(x[4 as libc::c_int as usize]);
    d = (t << 6 as libc::c_int | t >> (-(6 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d & e | !d & a)
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        .wrapping_add(x[13 as libc::c_int as usize]);
    c = (t << 8 as libc::c_int | t >> (-(8 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c & d | !c & e)
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        .wrapping_add(x[1 as libc::c_int as usize]);
    b = (t << 13 as libc::c_int | t >> (-(13 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b & c | !b & d)
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        .wrapping_add(x[10 as libc::c_int as usize]);
    a = (t << 11 as libc::c_int | t >> (-(11 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a & b | !a & c)
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        .wrapping_add(x[6 as libc::c_int as usize]);
    e = (t << 9 as libc::c_int | t >> (-(9 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e & a | !e & b)
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        .wrapping_add(x[15 as libc::c_int as usize]);
    d = (t << 7 as libc::c_int | t >> (-(7 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d & e | !d & a)
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        .wrapping_add(x[3 as libc::c_int as usize]);
    c = (t << 15 as libc::c_int | t >> (-(15 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c & d | !c & e)
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        .wrapping_add(x[12 as libc::c_int as usize]);
    b = (t << 7 as libc::c_int | t >> (-(7 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b & c | !b & d)
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        .wrapping_add(x[0 as libc::c_int as usize]);
    a = (t << 12 as libc::c_int | t >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a & b | !a & c)
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        .wrapping_add(x[9 as libc::c_int as usize]);
    e = (t << 15 as libc::c_int | t >> (-(15 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e & a | !e & b)
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        .wrapping_add(x[5 as libc::c_int as usize]);
    d = (t << 9 as libc::c_int | t >> (-(9 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d & e | !d & a)
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        .wrapping_add(x[2 as libc::c_int as usize]);
    c = (t << 11 as libc::c_int | t >> (-(11 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c & d | !c & e)
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        .wrapping_add(x[14 as libc::c_int as usize]);
    b = (t << 7 as libc::c_int | t >> (-(7 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b & c | !b & d)
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        .wrapping_add(x[11 as libc::c_int as usize]);
    a = (t << 13 as libc::c_int | t >> (-(13 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a & b | !a & c)
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        .wrapping_add(x[8 as libc::c_int as usize]);
    e = (t << 12 as libc::c_int | t >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add((e | !a) ^ b)
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        .wrapping_add(x[3 as libc::c_int as usize]);
    d = (t << 11 as libc::c_int | t >> (-(11 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add((d | !e) ^ a)
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        .wrapping_add(x[10 as libc::c_int as usize]);
    c = (t << 13 as libc::c_int | t >> (-(13 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add((c | !d) ^ e)
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        .wrapping_add(x[14 as libc::c_int as usize]);
    b = (t << 6 as libc::c_int | t >> (-(6 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add((b | !c) ^ d)
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        .wrapping_add(x[4 as libc::c_int as usize]);
    a = (t << 7 as libc::c_int | t >> (-(7 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add((a | !b) ^ c)
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        .wrapping_add(x[9 as libc::c_int as usize]);
    e = (t << 14 as libc::c_int | t >> (-(14 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add((e | !a) ^ b)
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        .wrapping_add(x[15 as libc::c_int as usize]);
    d = (t << 9 as libc::c_int | t >> (-(9 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add((d | !e) ^ a)
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        .wrapping_add(x[8 as libc::c_int as usize]);
    c = (t << 13 as libc::c_int | t >> (-(13 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add((c | !d) ^ e)
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        .wrapping_add(x[1 as libc::c_int as usize]);
    b = (t << 15 as libc::c_int | t >> (-(15 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add((b | !c) ^ d)
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        .wrapping_add(x[2 as libc::c_int as usize]);
    a = (t << 14 as libc::c_int | t >> (-(14 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add((a | !b) ^ c)
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        .wrapping_add(x[7 as libc::c_int as usize]);
    e = (t << 8 as libc::c_int | t >> (-(8 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add((e | !a) ^ b)
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        .wrapping_add(x[0 as libc::c_int as usize]);
    d = (t << 13 as libc::c_int | t >> (-(13 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add((d | !e) ^ a)
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        .wrapping_add(x[6 as libc::c_int as usize]);
    c = (t << 6 as libc::c_int | t >> (-(6 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add((c | !d) ^ e)
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        .wrapping_add(x[13 as libc::c_int as usize]);
    b = (t << 5 as libc::c_int | t >> (-(5 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add((b | !c) ^ d)
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        .wrapping_add(x[11 as libc::c_int as usize]);
    a = (t << 12 as libc::c_int | t >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add((a | !b) ^ c)
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        .wrapping_add(x[5 as libc::c_int as usize]);
    e = (t << 7 as libc::c_int | t >> (-(7 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add((e | !a) ^ b)
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        .wrapping_add(x[12 as libc::c_int as usize]);
    d = (t << 5 as libc::c_int | t >> (-(5 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d & a | e & !a)
        .wrapping_add(0x8f1bbcdc as libc::c_uint)
        .wrapping_add(x[1 as libc::c_int as usize]);
    c = (t << 11 as libc::c_int | t >> (-(11 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c & e | d & !e)
        .wrapping_add(0x8f1bbcdc as libc::c_uint)
        .wrapping_add(x[9 as libc::c_int as usize]);
    b = (t << 12 as libc::c_int | t >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b & d | c & !d)
        .wrapping_add(0x8f1bbcdc as libc::c_uint)
        .wrapping_add(x[11 as libc::c_int as usize]);
    a = (t << 14 as libc::c_int | t >> (-(14 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a & c | b & !c)
        .wrapping_add(0x8f1bbcdc as libc::c_uint)
        .wrapping_add(x[10 as libc::c_int as usize]);
    e = (t << 15 as libc::c_int | t >> (-(15 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e & b | a & !b)
        .wrapping_add(0x8f1bbcdc as libc::c_uint)
        .wrapping_add(x[0 as libc::c_int as usize]);
    d = (t << 14 as libc::c_int | t >> (-(14 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d & a | e & !a)
        .wrapping_add(0x8f1bbcdc as libc::c_uint)
        .wrapping_add(x[8 as libc::c_int as usize]);
    c = (t << 15 as libc::c_int | t >> (-(15 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c & e | d & !e)
        .wrapping_add(0x8f1bbcdc as libc::c_uint)
        .wrapping_add(x[12 as libc::c_int as usize]);
    b = (t << 9 as libc::c_int | t >> (-(9 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b & d | c & !d)
        .wrapping_add(0x8f1bbcdc as libc::c_uint)
        .wrapping_add(x[4 as libc::c_int as usize]);
    a = (t << 8 as libc::c_int | t >> (-(8 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a & c | b & !c)
        .wrapping_add(0x8f1bbcdc as libc::c_uint)
        .wrapping_add(x[13 as libc::c_int as usize]);
    e = (t << 9 as libc::c_int | t >> (-(9 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e & b | a & !b)
        .wrapping_add(0x8f1bbcdc as libc::c_uint)
        .wrapping_add(x[3 as libc::c_int as usize]);
    d = (t << 14 as libc::c_int | t >> (-(14 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d & a | e & !a)
        .wrapping_add(0x8f1bbcdc as libc::c_uint)
        .wrapping_add(x[7 as libc::c_int as usize]);
    c = (t << 5 as libc::c_int | t >> (-(5 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c & e | d & !e)
        .wrapping_add(0x8f1bbcdc as libc::c_uint)
        .wrapping_add(x[15 as libc::c_int as usize]);
    b = (t << 6 as libc::c_int | t >> (-(6 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b & d | c & !d)
        .wrapping_add(0x8f1bbcdc as libc::c_uint)
        .wrapping_add(x[14 as libc::c_int as usize]);
    a = (t << 8 as libc::c_int | t >> (-(8 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a & c | b & !c)
        .wrapping_add(0x8f1bbcdc as libc::c_uint)
        .wrapping_add(x[5 as libc::c_int as usize]);
    e = (t << 6 as libc::c_int | t >> (-(6 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e & b | a & !b)
        .wrapping_add(0x8f1bbcdc as libc::c_uint)
        .wrapping_add(x[6 as libc::c_int as usize]);
    d = (t << 5 as libc::c_int | t >> (-(5 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d & a | e & !a)
        .wrapping_add(0x8f1bbcdc as libc::c_uint)
        .wrapping_add(x[2 as libc::c_int as usize]);
    c = (t << 12 as libc::c_int | t >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c ^ (d | !e))
        .wrapping_add(0xa953fd4e as libc::c_uint)
        .wrapping_add(x[4 as libc::c_int as usize]);
    b = (t << 9 as libc::c_int | t >> (-(9 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b ^ (c | !d))
        .wrapping_add(0xa953fd4e as libc::c_uint)
        .wrapping_add(x[0 as libc::c_int as usize]);
    a = (t << 15 as libc::c_int | t >> (-(15 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a ^ (b | !c))
        .wrapping_add(0xa953fd4e as libc::c_uint)
        .wrapping_add(x[5 as libc::c_int as usize]);
    e = (t << 5 as libc::c_int | t >> (-(5 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e ^ (a | !b))
        .wrapping_add(0xa953fd4e as libc::c_uint)
        .wrapping_add(x[9 as libc::c_int as usize]);
    d = (t << 11 as libc::c_int | t >> (-(11 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d ^ (e | !a))
        .wrapping_add(0xa953fd4e as libc::c_uint)
        .wrapping_add(x[7 as libc::c_int as usize]);
    c = (t << 6 as libc::c_int | t >> (-(6 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c ^ (d | !e))
        .wrapping_add(0xa953fd4e as libc::c_uint)
        .wrapping_add(x[12 as libc::c_int as usize]);
    b = (t << 8 as libc::c_int | t >> (-(8 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b ^ (c | !d))
        .wrapping_add(0xa953fd4e as libc::c_uint)
        .wrapping_add(x[2 as libc::c_int as usize]);
    a = (t << 13 as libc::c_int | t >> (-(13 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a ^ (b | !c))
        .wrapping_add(0xa953fd4e as libc::c_uint)
        .wrapping_add(x[10 as libc::c_int as usize]);
    e = (t << 12 as libc::c_int | t >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e ^ (a | !b))
        .wrapping_add(0xa953fd4e as libc::c_uint)
        .wrapping_add(x[14 as libc::c_int as usize]);
    d = (t << 5 as libc::c_int | t >> (-(5 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d ^ (e | !a))
        .wrapping_add(0xa953fd4e as libc::c_uint)
        .wrapping_add(x[1 as libc::c_int as usize]);
    c = (t << 12 as libc::c_int | t >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c ^ (d | !e))
        .wrapping_add(0xa953fd4e as libc::c_uint)
        .wrapping_add(x[3 as libc::c_int as usize]);
    b = (t << 13 as libc::c_int | t >> (-(13 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b ^ (c | !d))
        .wrapping_add(0xa953fd4e as libc::c_uint)
        .wrapping_add(x[8 as libc::c_int as usize]);
    a = (t << 14 as libc::c_int | t >> (-(14 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a ^ (b | !c))
        .wrapping_add(0xa953fd4e as libc::c_uint)
        .wrapping_add(x[11 as libc::c_int as usize]);
    e = (t << 11 as libc::c_int | t >> (-(11 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e ^ (a | !b))
        .wrapping_add(0xa953fd4e as libc::c_uint)
        .wrapping_add(x[6 as libc::c_int as usize]);
    d = (t << 8 as libc::c_int | t >> (-(8 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d ^ (e | !a))
        .wrapping_add(0xa953fd4e as libc::c_uint)
        .wrapping_add(x[15 as libc::c_int as usize]);
    c = (t << 5 as libc::c_int | t >> (-(5 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c ^ (d | !e))
        .wrapping_add(0xa953fd4e as libc::c_uint)
        .wrapping_add(x[13 as libc::c_int as usize]);
    b = (t << 6 as libc::c_int | t >> (-(6 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    aa = a;
    bb = b;
    cc = c;
    dd = d;
    ee = e;
    a = *state.offset(0 as libc::c_int as isize);
    b = *state.offset(1 as libc::c_int as isize);
    c = *state.offset(2 as libc::c_int as isize);
    d = *state.offset(3 as libc::c_int as isize);
    e = *state.offset(4 as libc::c_int as isize);
    t = a
        .wrapping_add(b ^ (c | !d))
        .wrapping_add(0x50a28be6 as libc::c_int as libc::c_uint)
        .wrapping_add(x[5 as libc::c_int as usize]);
    a = (t << 8 as libc::c_int | t >> (-(8 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a ^ (b | !c))
        .wrapping_add(0x50a28be6 as libc::c_int as libc::c_uint)
        .wrapping_add(x[14 as libc::c_int as usize]);
    e = (t << 9 as libc::c_int | t >> (-(9 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e ^ (a | !b))
        .wrapping_add(0x50a28be6 as libc::c_int as libc::c_uint)
        .wrapping_add(x[7 as libc::c_int as usize]);
    d = (t << 9 as libc::c_int | t >> (-(9 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d ^ (e | !a))
        .wrapping_add(0x50a28be6 as libc::c_int as libc::c_uint)
        .wrapping_add(x[0 as libc::c_int as usize]);
    c = (t << 11 as libc::c_int | t >> (-(11 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c ^ (d | !e))
        .wrapping_add(0x50a28be6 as libc::c_int as libc::c_uint)
        .wrapping_add(x[9 as libc::c_int as usize]);
    b = (t << 13 as libc::c_int | t >> (-(13 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b ^ (c | !d))
        .wrapping_add(0x50a28be6 as libc::c_int as libc::c_uint)
        .wrapping_add(x[2 as libc::c_int as usize]);
    a = (t << 15 as libc::c_int | t >> (-(15 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a ^ (b | !c))
        .wrapping_add(0x50a28be6 as libc::c_int as libc::c_uint)
        .wrapping_add(x[11 as libc::c_int as usize]);
    e = (t << 15 as libc::c_int | t >> (-(15 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e ^ (a | !b))
        .wrapping_add(0x50a28be6 as libc::c_int as libc::c_uint)
        .wrapping_add(x[4 as libc::c_int as usize]);
    d = (t << 5 as libc::c_int | t >> (-(5 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d ^ (e | !a))
        .wrapping_add(0x50a28be6 as libc::c_int as libc::c_uint)
        .wrapping_add(x[13 as libc::c_int as usize]);
    c = (t << 7 as libc::c_int | t >> (-(7 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c ^ (d | !e))
        .wrapping_add(0x50a28be6 as libc::c_int as libc::c_uint)
        .wrapping_add(x[6 as libc::c_int as usize]);
    b = (t << 7 as libc::c_int | t >> (-(7 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b ^ (c | !d))
        .wrapping_add(0x50a28be6 as libc::c_int as libc::c_uint)
        .wrapping_add(x[15 as libc::c_int as usize]);
    a = (t << 8 as libc::c_int | t >> (-(8 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a ^ (b | !c))
        .wrapping_add(0x50a28be6 as libc::c_int as libc::c_uint)
        .wrapping_add(x[8 as libc::c_int as usize]);
    e = (t << 11 as libc::c_int | t >> (-(11 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e ^ (a | !b))
        .wrapping_add(0x50a28be6 as libc::c_int as libc::c_uint)
        .wrapping_add(x[1 as libc::c_int as usize]);
    d = (t << 14 as libc::c_int | t >> (-(14 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d ^ (e | !a))
        .wrapping_add(0x50a28be6 as libc::c_int as libc::c_uint)
        .wrapping_add(x[10 as libc::c_int as usize]);
    c = (t << 14 as libc::c_int | t >> (-(14 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c ^ (d | !e))
        .wrapping_add(0x50a28be6 as libc::c_int as libc::c_uint)
        .wrapping_add(x[3 as libc::c_int as usize]);
    b = (t << 12 as libc::c_int | t >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b ^ (c | !d))
        .wrapping_add(0x50a28be6 as libc::c_int as libc::c_uint)
        .wrapping_add(x[12 as libc::c_int as usize]);
    a = (t << 6 as libc::c_int | t >> (-(6 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a & c | b & !c)
        .wrapping_add(0x5c4dd124 as libc::c_int as libc::c_uint)
        .wrapping_add(x[6 as libc::c_int as usize]);
    e = (t << 9 as libc::c_int | t >> (-(9 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e & b | a & !b)
        .wrapping_add(0x5c4dd124 as libc::c_int as libc::c_uint)
        .wrapping_add(x[11 as libc::c_int as usize]);
    d = (t << 13 as libc::c_int | t >> (-(13 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d & a | e & !a)
        .wrapping_add(0x5c4dd124 as libc::c_int as libc::c_uint)
        .wrapping_add(x[3 as libc::c_int as usize]);
    c = (t << 15 as libc::c_int | t >> (-(15 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c & e | d & !e)
        .wrapping_add(0x5c4dd124 as libc::c_int as libc::c_uint)
        .wrapping_add(x[7 as libc::c_int as usize]);
    b = (t << 7 as libc::c_int | t >> (-(7 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b & d | c & !d)
        .wrapping_add(0x5c4dd124 as libc::c_int as libc::c_uint)
        .wrapping_add(x[0 as libc::c_int as usize]);
    a = (t << 12 as libc::c_int | t >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a & c | b & !c)
        .wrapping_add(0x5c4dd124 as libc::c_int as libc::c_uint)
        .wrapping_add(x[13 as libc::c_int as usize]);
    e = (t << 8 as libc::c_int | t >> (-(8 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e & b | a & !b)
        .wrapping_add(0x5c4dd124 as libc::c_int as libc::c_uint)
        .wrapping_add(x[5 as libc::c_int as usize]);
    d = (t << 9 as libc::c_int | t >> (-(9 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d & a | e & !a)
        .wrapping_add(0x5c4dd124 as libc::c_int as libc::c_uint)
        .wrapping_add(x[10 as libc::c_int as usize]);
    c = (t << 11 as libc::c_int | t >> (-(11 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c & e | d & !e)
        .wrapping_add(0x5c4dd124 as libc::c_int as libc::c_uint)
        .wrapping_add(x[14 as libc::c_int as usize]);
    b = (t << 7 as libc::c_int | t >> (-(7 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b & d | c & !d)
        .wrapping_add(0x5c4dd124 as libc::c_int as libc::c_uint)
        .wrapping_add(x[15 as libc::c_int as usize]);
    a = (t << 7 as libc::c_int | t >> (-(7 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a & c | b & !c)
        .wrapping_add(0x5c4dd124 as libc::c_int as libc::c_uint)
        .wrapping_add(x[8 as libc::c_int as usize]);
    e = (t << 12 as libc::c_int | t >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e & b | a & !b)
        .wrapping_add(0x5c4dd124 as libc::c_int as libc::c_uint)
        .wrapping_add(x[12 as libc::c_int as usize]);
    d = (t << 7 as libc::c_int | t >> (-(7 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d & a | e & !a)
        .wrapping_add(0x5c4dd124 as libc::c_int as libc::c_uint)
        .wrapping_add(x[4 as libc::c_int as usize]);
    c = (t << 6 as libc::c_int | t >> (-(6 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c & e | d & !e)
        .wrapping_add(0x5c4dd124 as libc::c_int as libc::c_uint)
        .wrapping_add(x[9 as libc::c_int as usize]);
    b = (t << 15 as libc::c_int | t >> (-(15 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b & d | c & !d)
        .wrapping_add(0x5c4dd124 as libc::c_int as libc::c_uint)
        .wrapping_add(x[1 as libc::c_int as usize]);
    a = (t << 13 as libc::c_int | t >> (-(13 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a & c | b & !c)
        .wrapping_add(0x5c4dd124 as libc::c_int as libc::c_uint)
        .wrapping_add(x[2 as libc::c_int as usize]);
    e = (t << 11 as libc::c_int | t >> (-(11 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add((e | !a) ^ b)
        .wrapping_add(0x6d703ef3 as libc::c_int as libc::c_uint)
        .wrapping_add(x[15 as libc::c_int as usize]);
    d = (t << 9 as libc::c_int | t >> (-(9 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add((d | !e) ^ a)
        .wrapping_add(0x6d703ef3 as libc::c_int as libc::c_uint)
        .wrapping_add(x[5 as libc::c_int as usize]);
    c = (t << 7 as libc::c_int | t >> (-(7 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add((c | !d) ^ e)
        .wrapping_add(0x6d703ef3 as libc::c_int as libc::c_uint)
        .wrapping_add(x[1 as libc::c_int as usize]);
    b = (t << 15 as libc::c_int | t >> (-(15 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add((b | !c) ^ d)
        .wrapping_add(0x6d703ef3 as libc::c_int as libc::c_uint)
        .wrapping_add(x[3 as libc::c_int as usize]);
    a = (t << 11 as libc::c_int | t >> (-(11 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add((a | !b) ^ c)
        .wrapping_add(0x6d703ef3 as libc::c_int as libc::c_uint)
        .wrapping_add(x[7 as libc::c_int as usize]);
    e = (t << 8 as libc::c_int | t >> (-(8 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add((e | !a) ^ b)
        .wrapping_add(0x6d703ef3 as libc::c_int as libc::c_uint)
        .wrapping_add(x[14 as libc::c_int as usize]);
    d = (t << 6 as libc::c_int | t >> (-(6 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add((d | !e) ^ a)
        .wrapping_add(0x6d703ef3 as libc::c_int as libc::c_uint)
        .wrapping_add(x[6 as libc::c_int as usize]);
    c = (t << 6 as libc::c_int | t >> (-(6 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add((c | !d) ^ e)
        .wrapping_add(0x6d703ef3 as libc::c_int as libc::c_uint)
        .wrapping_add(x[9 as libc::c_int as usize]);
    b = (t << 14 as libc::c_int | t >> (-(14 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add((b | !c) ^ d)
        .wrapping_add(0x6d703ef3 as libc::c_int as libc::c_uint)
        .wrapping_add(x[11 as libc::c_int as usize]);
    a = (t << 12 as libc::c_int | t >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add((a | !b) ^ c)
        .wrapping_add(0x6d703ef3 as libc::c_int as libc::c_uint)
        .wrapping_add(x[8 as libc::c_int as usize]);
    e = (t << 13 as libc::c_int | t >> (-(13 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add((e | !a) ^ b)
        .wrapping_add(0x6d703ef3 as libc::c_int as libc::c_uint)
        .wrapping_add(x[12 as libc::c_int as usize]);
    d = (t << 5 as libc::c_int | t >> (-(5 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add((d | !e) ^ a)
        .wrapping_add(0x6d703ef3 as libc::c_int as libc::c_uint)
        .wrapping_add(x[2 as libc::c_int as usize]);
    c = (t << 14 as libc::c_int | t >> (-(14 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add((c | !d) ^ e)
        .wrapping_add(0x6d703ef3 as libc::c_int as libc::c_uint)
        .wrapping_add(x[10 as libc::c_int as usize]);
    b = (t << 13 as libc::c_int | t >> (-(13 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add((b | !c) ^ d)
        .wrapping_add(0x6d703ef3 as libc::c_int as libc::c_uint)
        .wrapping_add(x[0 as libc::c_int as usize]);
    a = (t << 13 as libc::c_int | t >> (-(13 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add((a | !b) ^ c)
        .wrapping_add(0x6d703ef3 as libc::c_int as libc::c_uint)
        .wrapping_add(x[4 as libc::c_int as usize]);
    e = (t << 7 as libc::c_int | t >> (-(7 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add((e | !a) ^ b)
        .wrapping_add(0x6d703ef3 as libc::c_int as libc::c_uint)
        .wrapping_add(x[13 as libc::c_int as usize]);
    d = (t << 5 as libc::c_int | t >> (-(5 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d & e | !d & a)
        .wrapping_add(0x7a6d76e9 as libc::c_int as libc::c_uint)
        .wrapping_add(x[8 as libc::c_int as usize]);
    c = (t << 15 as libc::c_int | t >> (-(15 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c & d | !c & e)
        .wrapping_add(0x7a6d76e9 as libc::c_int as libc::c_uint)
        .wrapping_add(x[6 as libc::c_int as usize]);
    b = (t << 5 as libc::c_int | t >> (-(5 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b & c | !b & d)
        .wrapping_add(0x7a6d76e9 as libc::c_int as libc::c_uint)
        .wrapping_add(x[4 as libc::c_int as usize]);
    a = (t << 8 as libc::c_int | t >> (-(8 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a & b | !a & c)
        .wrapping_add(0x7a6d76e9 as libc::c_int as libc::c_uint)
        .wrapping_add(x[1 as libc::c_int as usize]);
    e = (t << 11 as libc::c_int | t >> (-(11 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e & a | !e & b)
        .wrapping_add(0x7a6d76e9 as libc::c_int as libc::c_uint)
        .wrapping_add(x[3 as libc::c_int as usize]);
    d = (t << 14 as libc::c_int | t >> (-(14 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d & e | !d & a)
        .wrapping_add(0x7a6d76e9 as libc::c_int as libc::c_uint)
        .wrapping_add(x[11 as libc::c_int as usize]);
    c = (t << 14 as libc::c_int | t >> (-(14 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c & d | !c & e)
        .wrapping_add(0x7a6d76e9 as libc::c_int as libc::c_uint)
        .wrapping_add(x[15 as libc::c_int as usize]);
    b = (t << 6 as libc::c_int | t >> (-(6 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b & c | !b & d)
        .wrapping_add(0x7a6d76e9 as libc::c_int as libc::c_uint)
        .wrapping_add(x[0 as libc::c_int as usize]);
    a = (t << 14 as libc::c_int | t >> (-(14 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a & b | !a & c)
        .wrapping_add(0x7a6d76e9 as libc::c_int as libc::c_uint)
        .wrapping_add(x[5 as libc::c_int as usize]);
    e = (t << 6 as libc::c_int | t >> (-(6 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e & a | !e & b)
        .wrapping_add(0x7a6d76e9 as libc::c_int as libc::c_uint)
        .wrapping_add(x[12 as libc::c_int as usize]);
    d = (t << 9 as libc::c_int | t >> (-(9 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d & e | !d & a)
        .wrapping_add(0x7a6d76e9 as libc::c_int as libc::c_uint)
        .wrapping_add(x[2 as libc::c_int as usize]);
    c = (t << 12 as libc::c_int | t >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c & d | !c & e)
        .wrapping_add(0x7a6d76e9 as libc::c_int as libc::c_uint)
        .wrapping_add(x[13 as libc::c_int as usize]);
    b = (t << 9 as libc::c_int | t >> (-(9 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b & c | !b & d)
        .wrapping_add(0x7a6d76e9 as libc::c_int as libc::c_uint)
        .wrapping_add(x[9 as libc::c_int as usize]);
    a = (t << 12 as libc::c_int | t >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a & b | !a & c)
        .wrapping_add(0x7a6d76e9 as libc::c_int as libc::c_uint)
        .wrapping_add(x[7 as libc::c_int as usize]);
    e = (t << 5 as libc::c_int | t >> (-(5 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e & a | !e & b)
        .wrapping_add(0x7a6d76e9 as libc::c_int as libc::c_uint)
        .wrapping_add(x[10 as libc::c_int as usize]);
    d = (t << 15 as libc::c_int | t >> (-(15 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d & e | !d & a)
        .wrapping_add(0x7a6d76e9 as libc::c_int as libc::c_uint)
        .wrapping_add(x[14 as libc::c_int as usize]);
    c = (t << 8 as libc::c_int | t >> (-(8 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c ^ d ^ e)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[12 as libc::c_int as usize]);
    b = (t << 8 as libc::c_int | t >> (-(8 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b ^ c ^ d)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[15 as libc::c_int as usize]);
    a = (t << 5 as libc::c_int | t >> (-(5 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a ^ b ^ c)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[10 as libc::c_int as usize]);
    e = (t << 12 as libc::c_int | t >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e ^ a ^ b)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[4 as libc::c_int as usize]);
    d = (t << 9 as libc::c_int | t >> (-(9 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d ^ e ^ a)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[1 as libc::c_int as usize]);
    c = (t << 12 as libc::c_int | t >> (-(12 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c ^ d ^ e)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[5 as libc::c_int as usize]);
    b = (t << 5 as libc::c_int | t >> (-(5 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b ^ c ^ d)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[8 as libc::c_int as usize]);
    a = (t << 14 as libc::c_int | t >> (-(14 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a ^ b ^ c)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[7 as libc::c_int as usize]);
    e = (t << 6 as libc::c_int | t >> (-(6 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e ^ a ^ b)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[6 as libc::c_int as usize]);
    d = (t << 8 as libc::c_int | t >> (-(8 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d ^ e ^ a)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[2 as libc::c_int as usize]);
    c = (t << 13 as libc::c_int | t >> (-(13 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c ^ d ^ e)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[13 as libc::c_int as usize]);
    b = (t << 6 as libc::c_int | t >> (-(6 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = a
        .wrapping_add(b ^ c ^ d)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[14 as libc::c_int as usize]);
    a = (t << 5 as libc::c_int | t >> (-(5 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(e);
    c = c << 10 as libc::c_int | c >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = e
        .wrapping_add(a ^ b ^ c)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[0 as libc::c_int as usize]);
    e = (t << 15 as libc::c_int | t >> (-(15 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(d);
    b = b << 10 as libc::c_int | b >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = d
        .wrapping_add(e ^ a ^ b)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[3 as libc::c_int as usize]);
    d = (t << 13 as libc::c_int | t >> (-(13 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(c);
    a = a << 10 as libc::c_int | a >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = c
        .wrapping_add(d ^ e ^ a)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[9 as libc::c_int as usize]);
    c = (t << 11 as libc::c_int | t >> (-(11 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(b);
    e = e << 10 as libc::c_int | e >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = b
        .wrapping_add(c ^ d ^ e)
        .wrapping_add(0 as libc::c_int as libc::c_uint)
        .wrapping_add(x[11 as libc::c_int as usize]);
    b = (t << 11 as libc::c_int | t >> (-(11 as libc::c_int) & 31 as libc::c_int))
        .wrapping_add(a);
    d = d << 10 as libc::c_int | d >> (-(10 as libc::c_int) & 31 as libc::c_int);
    t = (*state.offset(1 as libc::c_int as isize)).wrapping_add(d).wrapping_add(cc);
    *state
        .offset(
            1 as libc::c_int as isize,
        ) = (*state.offset(2 as libc::c_int as isize)).wrapping_add(e).wrapping_add(dd);
    *state
        .offset(
            2 as libc::c_int as isize,
        ) = (*state.offset(3 as libc::c_int as isize)).wrapping_add(a).wrapping_add(ee);
    *state
        .offset(
            3 as libc::c_int as isize,
        ) = (*state.offset(4 as libc::c_int as isize)).wrapping_add(b).wrapping_add(aa);
    *state
        .offset(
            4 as libc::c_int as isize,
        ) = (*state.offset(0 as libc::c_int as isize)).wrapping_add(c).wrapping_add(bb);
    *state.offset(0 as libc::c_int as isize) = t;
}
