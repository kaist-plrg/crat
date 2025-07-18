use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type md5_byte_t = libc::c_uchar;
pub type md5_word_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md5_state_s {
    pub count: [md5_word_t; 2],
    pub abcd: [md5_word_t; 4],
    pub buf: [md5_byte_t; 64],
}
pub type md5_state_t = md5_state_s;
pub type uintptr_t = libc::c_ulong;
unsafe extern "C" fn md5_process(
    mut pms: *mut md5_state_t,
    mut data: *const md5_byte_t,
) {
    let mut a: md5_word_t = (*pms).abcd[0 as libc::c_int as usize];
    let mut b: md5_word_t = (*pms).abcd[1 as libc::c_int as usize];
    let mut c: md5_word_t = (*pms).abcd[2 as libc::c_int as usize];
    let mut d: md5_word_t = (*pms).abcd[3 as libc::c_int as usize];
    let mut t: md5_word_t = 0;
    let mut xbuf: [md5_word_t; 16] = [0; 16];
    let mut X: *const md5_word_t = 0 as *const md5_word_t;
    static mut w: libc::c_int = 1 as libc::c_int;
    if *(&w as *const libc::c_int as *const md5_byte_t) != 0 {
        if data as uintptr_t & (4 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            == 0
        {
            X = data as *const md5_word_t;
        } else {
            memcpy(
                xbuf.as_mut_ptr() as *mut libc::c_void,
                data as *const libc::c_void,
                64 as libc::c_int as libc::c_ulong,
            );
            X = xbuf.as_mut_ptr();
        }
    } else {
        let mut xp: *const md5_byte_t = data;
        let mut i: libc::c_int = 0;
        X = xbuf.as_mut_ptr();
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            xbuf[i
                as usize] = (*xp.offset(0 as libc::c_int as isize) as libc::c_int
                + ((*xp.offset(1 as libc::c_int as isize) as libc::c_int)
                    << 8 as libc::c_int)
                + ((*xp.offset(2 as libc::c_int as isize) as libc::c_int)
                    << 16 as libc::c_int)
                + ((*xp.offset(3 as libc::c_int as isize) as libc::c_int)
                    << 24 as libc::c_int)) as md5_word_t;
            i += 1;
            i;
            xp = xp.offset(4 as libc::c_int as isize);
        }
    }
    t = a
        .wrapping_add(b & c | !b & d)
        .wrapping_add(*X.offset(0 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x28955b87 as libc::c_int as libc::c_uint,
        );
    a = (t << 7 as libc::c_int | t >> 32 as libc::c_int - 7 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a & b | !a & c)
        .wrapping_add(*X.offset(1 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x173848a9 as libc::c_int as libc::c_uint,
        );
    d = (t << 12 as libc::c_int | t >> 32 as libc::c_int - 12 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d & a | !d & b)
        .wrapping_add(*X.offset(2 as libc::c_int as isize))
        .wrapping_add(0x242070db as libc::c_int as libc::c_uint);
    c = (t << 17 as libc::c_int | t >> 32 as libc::c_int - 17 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c & d | !c & a)
        .wrapping_add(*X.offset(3 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x3e423111 as libc::c_int as libc::c_uint,
        );
    b = (t << 22 as libc::c_int | t >> 32 as libc::c_int - 22 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(b & c | !b & d)
        .wrapping_add(*X.offset(4 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0xa83f050 as libc::c_int as libc::c_uint,
        );
    a = (t << 7 as libc::c_int | t >> 32 as libc::c_int - 7 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a & b | !a & c)
        .wrapping_add(*X.offset(5 as libc::c_int as isize))
        .wrapping_add(0x4787c62a as libc::c_int as libc::c_uint);
    d = (t << 12 as libc::c_int | t >> 32 as libc::c_int - 12 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d & a | !d & b)
        .wrapping_add(*X.offset(6 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x57cfb9ec as libc::c_int as libc::c_uint,
        );
    c = (t << 17 as libc::c_int | t >> 32 as libc::c_int - 17 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c & d | !c & a)
        .wrapping_add(*X.offset(7 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x2b96afe as libc::c_int as libc::c_uint,
        );
    b = (t << 22 as libc::c_int | t >> 32 as libc::c_int - 22 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(b & c | !b & d)
        .wrapping_add(*X.offset(8 as libc::c_int as isize))
        .wrapping_add(0x698098d8 as libc::c_int as libc::c_uint);
    a = (t << 7 as libc::c_int | t >> 32 as libc::c_int - 7 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a & b | !a & c)
        .wrapping_add(*X.offset(9 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x74bb0850 as libc::c_int as libc::c_uint,
        );
    d = (t << 12 as libc::c_int | t >> 32 as libc::c_int - 12 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d & a | !d & b)
        .wrapping_add(*X.offset(10 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0xa44e as libc::c_int as libc::c_uint,
        );
    c = (t << 17 as libc::c_int | t >> 32 as libc::c_int - 17 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c & d | !c & a)
        .wrapping_add(*X.offset(11 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x76a32841 as libc::c_int as libc::c_uint,
        );
    b = (t << 22 as libc::c_int | t >> 32 as libc::c_int - 22 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(b & c | !b & d)
        .wrapping_add(*X.offset(12 as libc::c_int as isize))
        .wrapping_add(0x6b901122 as libc::c_int as libc::c_uint);
    a = (t << 7 as libc::c_int | t >> 32 as libc::c_int - 7 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a & b | !a & c)
        .wrapping_add(*X.offset(13 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x2678e6c as libc::c_int as libc::c_uint,
        );
    d = (t << 12 as libc::c_int | t >> 32 as libc::c_int - 12 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d & a | !d & b)
        .wrapping_add(*X.offset(14 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x5986bc71 as libc::c_int as libc::c_uint,
        );
    c = (t << 17 as libc::c_int | t >> 32 as libc::c_int - 17 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c & d | !c & a)
        .wrapping_add(*X.offset(15 as libc::c_int as isize))
        .wrapping_add(0x49b40821 as libc::c_int as libc::c_uint);
    b = (t << 22 as libc::c_int | t >> 32 as libc::c_int - 22 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(b & d | c & !d)
        .wrapping_add(*X.offset(1 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x9e1da9d as libc::c_int as libc::c_uint,
        );
    a = (t << 5 as libc::c_int | t >> 32 as libc::c_int - 5 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a & c | b & !c)
        .wrapping_add(*X.offset(6 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x3fbf4cbf as libc::c_int as libc::c_uint,
        );
    d = (t << 9 as libc::c_int | t >> 32 as libc::c_int - 9 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d & b | a & !b)
        .wrapping_add(*X.offset(11 as libc::c_int as isize))
        .wrapping_add(0x265e5a51 as libc::c_int as libc::c_uint);
    c = (t << 14 as libc::c_int | t >> 32 as libc::c_int - 14 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c & a | d & !a)
        .wrapping_add(*X.offset(0 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x16493855 as libc::c_int as libc::c_uint,
        );
    b = (t << 20 as libc::c_int | t >> 32 as libc::c_int - 20 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(b & d | c & !d)
        .wrapping_add(*X.offset(5 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x29d0efa2 as libc::c_int as libc::c_uint,
        );
    a = (t << 5 as libc::c_int | t >> 32 as libc::c_int - 5 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a & c | b & !c)
        .wrapping_add(*X.offset(10 as libc::c_int as isize))
        .wrapping_add(0x2441453 as libc::c_int as libc::c_uint);
    d = (t << 9 as libc::c_int | t >> 32 as libc::c_int - 9 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d & b | a & !b)
        .wrapping_add(*X.offset(15 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x275e197e as libc::c_int as libc::c_uint,
        );
    c = (t << 14 as libc::c_int | t >> 32 as libc::c_int - 14 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c & a | d & !a)
        .wrapping_add(*X.offset(4 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x182c0437 as libc::c_int as libc::c_uint,
        );
    b = (t << 20 as libc::c_int | t >> 32 as libc::c_int - 20 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(b & d | c & !d)
        .wrapping_add(*X.offset(9 as libc::c_int as isize))
        .wrapping_add(0x21e1cde6 as libc::c_int as libc::c_uint);
    a = (t << 5 as libc::c_int | t >> 32 as libc::c_int - 5 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a & c | b & !c)
        .wrapping_add(*X.offset(14 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x3cc8f829 as libc::c_int as libc::c_uint,
        );
    d = (t << 9 as libc::c_int | t >> 32 as libc::c_int - 9 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d & b | a & !b)
        .wrapping_add(*X.offset(3 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0xb2af278 as libc::c_int as libc::c_uint,
        );
    c = (t << 14 as libc::c_int | t >> 32 as libc::c_int - 14 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c & a | d & !a)
        .wrapping_add(*X.offset(8 as libc::c_int as isize))
        .wrapping_add(0x455a14ed as libc::c_int as libc::c_uint);
    b = (t << 20 as libc::c_int | t >> 32 as libc::c_int - 20 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(b & d | c & !d)
        .wrapping_add(*X.offset(13 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x561c16fa as libc::c_int as libc::c_uint,
        );
    a = (t << 5 as libc::c_int | t >> 32 as libc::c_int - 5 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a & c | b & !c)
        .wrapping_add(*X.offset(2 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x3105c07 as libc::c_int as libc::c_uint,
        );
    d = (t << 9 as libc::c_int | t >> 32 as libc::c_int - 9 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d & b | a & !b)
        .wrapping_add(*X.offset(7 as libc::c_int as isize))
        .wrapping_add(0x676f02d9 as libc::c_int as libc::c_uint);
    c = (t << 14 as libc::c_int | t >> 32 as libc::c_int - 14 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c & a | d & !a)
        .wrapping_add(*X.offset(12 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x72d5b375 as libc::c_int as libc::c_uint,
        );
    b = (t << 20 as libc::c_int | t >> 32 as libc::c_int - 20 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(b ^ c ^ d)
        .wrapping_add(*X.offset(5 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x5c6bd as libc::c_int as libc::c_uint,
        );
    a = (t << 4 as libc::c_int | t >> 32 as libc::c_int - 4 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a ^ b ^ c)
        .wrapping_add(*X.offset(8 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x788e097e as libc::c_int as libc::c_uint,
        );
    d = (t << 11 as libc::c_int | t >> 32 as libc::c_int - 11 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d ^ a ^ b)
        .wrapping_add(*X.offset(11 as libc::c_int as isize))
        .wrapping_add(0x6d9d6122 as libc::c_int as libc::c_uint);
    c = (t << 16 as libc::c_int | t >> 32 as libc::c_int - 16 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c ^ d ^ a)
        .wrapping_add(*X.offset(14 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x21ac7f3 as libc::c_int as libc::c_uint,
        );
    b = (t << 23 as libc::c_int | t >> 32 as libc::c_int - 23 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(b ^ c ^ d)
        .wrapping_add(*X.offset(1 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x5b4115bb as libc::c_int as libc::c_uint,
        );
    a = (t << 4 as libc::c_int | t >> 32 as libc::c_int - 4 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a ^ b ^ c)
        .wrapping_add(*X.offset(4 as libc::c_int as isize))
        .wrapping_add(0x4bdecfa9 as libc::c_int as libc::c_uint);
    d = (t << 11 as libc::c_int | t >> 32 as libc::c_int - 11 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d ^ a ^ b)
        .wrapping_add(*X.offset(7 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x944b49f as libc::c_int as libc::c_uint,
        );
    c = (t << 16 as libc::c_int | t >> 32 as libc::c_int - 16 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c ^ d ^ a)
        .wrapping_add(*X.offset(10 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x4140438f as libc::c_int as libc::c_uint,
        );
    b = (t << 23 as libc::c_int | t >> 32 as libc::c_int - 23 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(b ^ c ^ d)
        .wrapping_add(*X.offset(13 as libc::c_int as isize))
        .wrapping_add(0x289b7ec6 as libc::c_int as libc::c_uint);
    a = (t << 4 as libc::c_int | t >> 32 as libc::c_int - 4 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a ^ b ^ c)
        .wrapping_add(*X.offset(0 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x155ed805 as libc::c_int as libc::c_uint,
        );
    d = (t << 11 as libc::c_int | t >> 32 as libc::c_int - 11 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d ^ a ^ b)
        .wrapping_add(*X.offset(3 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x2b10cf7a as libc::c_int as libc::c_uint,
        );
    c = (t << 16 as libc::c_int | t >> 32 as libc::c_int - 16 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c ^ d ^ a)
        .wrapping_add(*X.offset(6 as libc::c_int as isize))
        .wrapping_add(0x4881d05 as libc::c_int as libc::c_uint);
    b = (t << 23 as libc::c_int | t >> 32 as libc::c_int - 23 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(b ^ c ^ d)
        .wrapping_add(*X.offset(9 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x262b2fc6 as libc::c_int as libc::c_uint,
        );
    a = (t << 4 as libc::c_int | t >> 32 as libc::c_int - 4 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a ^ b ^ c)
        .wrapping_add(*X.offset(12 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x1924661a as libc::c_int as libc::c_uint,
        );
    d = (t << 11 as libc::c_int | t >> 32 as libc::c_int - 11 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d ^ a ^ b)
        .wrapping_add(*X.offset(15 as libc::c_int as isize))
        .wrapping_add(0x1fa27cf8 as libc::c_int as libc::c_uint);
    c = (t << 16 as libc::c_int | t >> 32 as libc::c_int - 16 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c ^ d ^ a)
        .wrapping_add(*X.offset(2 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x3b53a99a as libc::c_int as libc::c_uint,
        );
    b = (t << 23 as libc::c_int | t >> 32 as libc::c_int - 23 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(c ^ (b | !d))
        .wrapping_add(*X.offset(0 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0xbd6ddbb as libc::c_int as libc::c_uint,
        );
    a = (t << 6 as libc::c_int | t >> 32 as libc::c_int - 6 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(b ^ (a | !c))
        .wrapping_add(*X.offset(7 as libc::c_int as isize))
        .wrapping_add(0x432aff97 as libc::c_int as libc::c_uint);
    d = (t << 10 as libc::c_int | t >> 32 as libc::c_int - 10 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(a ^ (d | !b))
        .wrapping_add(*X.offset(14 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x546bdc58 as libc::c_int as libc::c_uint,
        );
    c = (t << 15 as libc::c_int | t >> 32 as libc::c_int - 15 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(d ^ (c | !a))
        .wrapping_add(*X.offset(5 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x36c5fc6 as libc::c_int as libc::c_uint,
        );
    b = (t << 21 as libc::c_int | t >> 32 as libc::c_int - 21 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(c ^ (b | !d))
        .wrapping_add(*X.offset(12 as libc::c_int as isize))
        .wrapping_add(0x655b59c3 as libc::c_int as libc::c_uint);
    a = (t << 6 as libc::c_int | t >> 32 as libc::c_int - 6 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(b ^ (a | !c))
        .wrapping_add(*X.offset(3 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x70f3336d as libc::c_int as libc::c_uint,
        );
    d = (t << 10 as libc::c_int | t >> 32 as libc::c_int - 10 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(a ^ (d | !b))
        .wrapping_add(*X.offset(10 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x100b82 as libc::c_int as libc::c_uint,
        );
    c = (t << 15 as libc::c_int | t >> 32 as libc::c_int - 15 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(d ^ (c | !a))
        .wrapping_add(*X.offset(1 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x7a7ba22e as libc::c_int as libc::c_uint,
        );
    b = (t << 21 as libc::c_int | t >> 32 as libc::c_int - 21 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(c ^ (b | !d))
        .wrapping_add(*X.offset(8 as libc::c_int as isize))
        .wrapping_add(0x6fa87e4f as libc::c_int as libc::c_uint);
    a = (t << 6 as libc::c_int | t >> 32 as libc::c_int - 6 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(b ^ (a | !c))
        .wrapping_add(*X.offset(15 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x1d3191f as libc::c_int as libc::c_uint,
        );
    d = (t << 10 as libc::c_int | t >> 32 as libc::c_int - 10 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(a ^ (d | !b))
        .wrapping_add(*X.offset(6 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x5cfebceb as libc::c_int as libc::c_uint,
        );
    c = (t << 15 as libc::c_int | t >> 32 as libc::c_int - 15 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(d ^ (c | !a))
        .wrapping_add(*X.offset(13 as libc::c_int as isize))
        .wrapping_add(0x4e0811a1 as libc::c_int as libc::c_uint);
    b = (t << 21 as libc::c_int | t >> 32 as libc::c_int - 21 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(c ^ (b | !d))
        .wrapping_add(*X.offset(4 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x8ac817d as libc::c_int as libc::c_uint,
        );
    a = (t << 6 as libc::c_int | t >> 32 as libc::c_int - 6 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(b ^ (a | !c))
        .wrapping_add(*X.offset(11 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x42c50dca as libc::c_int as libc::c_uint,
        );
    d = (t << 10 as libc::c_int | t >> 32 as libc::c_int - 10 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(a ^ (d | !b))
        .wrapping_add(*X.offset(2 as libc::c_int as isize))
        .wrapping_add(0x2ad7d2bb as libc::c_int as libc::c_uint);
    c = (t << 15 as libc::c_int | t >> 32 as libc::c_int - 15 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(d ^ (c | !a))
        .wrapping_add(*X.offset(9 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x14792c6e as libc::c_int as libc::c_uint,
        );
    b = (t << 21 as libc::c_int | t >> 32 as libc::c_int - 21 as libc::c_int)
        .wrapping_add(c);
    (*pms)
        .abcd[0 as libc::c_int
        as usize] = ((*pms).abcd[0 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(a) as md5_word_t as md5_word_t;
    (*pms)
        .abcd[1 as libc::c_int
        as usize] = ((*pms).abcd[1 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(b) as md5_word_t as md5_word_t;
    (*pms)
        .abcd[2 as libc::c_int
        as usize] = ((*pms).abcd[2 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(c) as md5_word_t as md5_word_t;
    (*pms)
        .abcd[3 as libc::c_int
        as usize] = ((*pms).abcd[3 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(d) as md5_word_t as md5_word_t;
}
pub unsafe extern "C" fn md5_init(mut pms: *mut md5_state_t) {
    (*pms).count[1 as libc::c_int as usize] = 0 as libc::c_int as md5_word_t;
    (*pms).count[0 as libc::c_int as usize] = (*pms).count[1 as libc::c_int as usize];
    (*pms).abcd[0 as libc::c_int as usize] = 0x67452301 as libc::c_int as md5_word_t;
    (*pms)
        .abcd[1 as libc::c_int
        as usize] = !(0 as libc::c_int) as md5_word_t
        ^ 0x10325476 as libc::c_int as libc::c_uint;
    (*pms)
        .abcd[2 as libc::c_int
        as usize] = !(0 as libc::c_int) as md5_word_t
        ^ 0x67452301 as libc::c_int as libc::c_uint;
    (*pms).abcd[3 as libc::c_int as usize] = 0x10325476 as libc::c_int as md5_word_t;
}
pub unsafe extern "C" fn md5_append(
    mut pms: *mut md5_state_t,
    mut data: *const md5_byte_t,
    mut nbytes: libc::c_int,
) {
    let mut p: *const md5_byte_t = data;
    let mut left: libc::c_int = nbytes;
    let mut offset: libc::c_int = ((*pms).count[0 as libc::c_int as usize]
        >> 3 as libc::c_int & 63 as libc::c_int as libc::c_uint) as libc::c_int;
    let mut nbits: md5_word_t = (nbytes << 3 as libc::c_int) as md5_word_t;
    if nbytes <= 0 as libc::c_int {
        return;
    }
    (*pms)
        .count[1 as libc::c_int
        as usize] = ((*pms).count[1 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add((nbytes >> 29 as libc::c_int) as libc::c_uint) as md5_word_t
        as md5_word_t;
    (*pms)
        .count[0 as libc::c_int
        as usize] = ((*pms).count[0 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(nbits) as md5_word_t as md5_word_t;
    if (*pms).count[0 as libc::c_int as usize] < nbits {
        (*pms)
            .count[1 as libc::c_int
            as usize] = ((*pms).count[1 as libc::c_int as usize]).wrapping_add(1);
        (*pms).count[1 as libc::c_int as usize];
    }
    if offset != 0 {
        let mut copy: libc::c_int = if offset + nbytes > 64 as libc::c_int {
            64 as libc::c_int - offset
        } else {
            nbytes
        };
        memcpy(
            ((*pms).buf).as_mut_ptr().offset(offset as isize) as *mut libc::c_void,
            p as *const libc::c_void,
            copy as libc::c_ulong,
        );
        if offset + copy < 64 as libc::c_int {
            return;
        }
        p = p.offset(copy as isize);
        left -= copy;
        md5_process(pms, ((*pms).buf).as_mut_ptr());
    }
    while left >= 64 as libc::c_int {
        md5_process(pms, p);
        p = p.offset(64 as libc::c_int as isize);
        left -= 64 as libc::c_int;
    }
    if left != 0 {
        memcpy(
            ((*pms).buf).as_mut_ptr() as *mut libc::c_void,
            p as *const libc::c_void,
            left as libc::c_ulong,
        );
    }
}
pub unsafe extern "C" fn md5_finish(
    mut pms: *mut md5_state_t,
    mut digest: *mut md5_byte_t,
) {
    static mut pad: [md5_byte_t; 64] = [
        0x80 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
    ];
    let mut data: [md5_byte_t; 8] = [0; 8];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        data[i
            as usize] = ((*pms).count[(i >> 2 as libc::c_int) as usize]
            >> ((i & 3 as libc::c_int) << 3 as libc::c_int)) as md5_byte_t;
        i += 1;
        i;
    }
    md5_append(
        pms,
        pad.as_ptr(),
        ((55 as libc::c_int as libc::c_uint)
            .wrapping_sub((*pms).count[0 as libc::c_int as usize] >> 3 as libc::c_int)
            & 63 as libc::c_int as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int,
    );
    md5_append(pms, data.as_mut_ptr(), 8 as libc::c_int);
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        *digest
            .offset(
                i as isize,
            ) = ((*pms).abcd[(i >> 2 as libc::c_int) as usize]
            >> ((i & 3 as libc::c_int) << 3 as libc::c_int)) as md5_byte_t;
        i += 1;
        i;
    }
}
