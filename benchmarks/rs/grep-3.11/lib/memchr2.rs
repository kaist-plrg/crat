use ::libc;
extern "C" {
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type longword = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;
pub unsafe extern "C" fn memchr2(
    mut s: *const libc::c_void,
    mut c1_in: libc::c_int,
    mut c2_in: libc::c_int,
    mut n: size_t,
) -> *mut libc::c_void {
    let mut char_ptr: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut void_ptr: *const libc::c_void = 0 as *const libc::c_void;
    let mut longword_ptr: *const longword = 0 as *const longword;
    let mut repeated_one: longword = 0;
    let mut repeated_c1: longword = 0;
    let mut repeated_c2: longword = 0;
    let mut c1: libc::c_uchar = 0;
    let mut c2: libc::c_uchar = 0;
    c1 = c1_in as libc::c_uchar;
    c2 = c2_in as libc::c_uchar;
    if c1 as libc::c_int == c2 as libc::c_int {
        return memchr(s, c1 as libc::c_int, n);
    }
    void_ptr = s;
    while n > 0 as libc::c_int as libc::c_ulong
        && (void_ptr as uintptr_t)
            .wrapping_rem(::std::mem::size_of::<longword>() as libc::c_ulong)
            != 0 as libc::c_int as libc::c_ulong
    {
        char_ptr = void_ptr as *const libc::c_uchar;
        if *char_ptr as libc::c_int == c1 as libc::c_int
            || *char_ptr as libc::c_int == c2 as libc::c_int
        {
            return void_ptr as *mut libc::c_void;
        }
        void_ptr = char_ptr.offset(1 as libc::c_int as isize) as *const libc::c_void;
        n = n.wrapping_sub(1);
        n;
    }
    longword_ptr = void_ptr as *const longword;
    repeated_one = 0x1010101 as libc::c_int as longword;
    repeated_c1 = (c1 as libc::c_int | (c1 as libc::c_int) << 8 as libc::c_int)
        as longword;
    repeated_c2 = (c2 as libc::c_int | (c2 as libc::c_int) << 8 as libc::c_int)
        as longword;
    repeated_c1 |= repeated_c1 << 16 as libc::c_int;
    repeated_c2 |= repeated_c2 << 16 as libc::c_int;
    if (0xffffffff as libc::c_uint as libc::c_ulong) < -(1 as libc::c_int) as longword {
        repeated_one |= (repeated_one << 31 as libc::c_int) << 1 as libc::c_int;
        repeated_c1 |= (repeated_c1 << 31 as libc::c_int) << 1 as libc::c_int;
        repeated_c2 |= (repeated_c2 << 31 as libc::c_int) << 1 as libc::c_int;
        if (8 as libc::c_int as libc::c_ulong)
            < ::std::mem::size_of::<longword>() as libc::c_ulong
        {
            let mut i: size_t = 0;
            i = 64 as libc::c_int as size_t;
            while i
                < (::std::mem::size_of::<longword>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            {
                repeated_one |= repeated_one << i;
                repeated_c1 |= repeated_c1 << i;
                repeated_c2 |= repeated_c2 << i;
                i = (i as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
            }
        }
    }
    while n >= ::std::mem::size_of::<longword>() as libc::c_ulong {
        let mut longword1: longword = *longword_ptr ^ repeated_c1;
        let mut longword2: longword = *longword_ptr ^ repeated_c2;
        if (longword1.wrapping_sub(repeated_one) & !longword1
            | longword2.wrapping_sub(repeated_one) & !longword2)
            & repeated_one << 7 as libc::c_int != 0 as libc::c_int as libc::c_ulong
        {
            break;
        }
        longword_ptr = longword_ptr.offset(1);
        longword_ptr;
        n = (n as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<longword>() as libc::c_ulong) as size_t
            as size_t;
    }
    char_ptr = longword_ptr as *const libc::c_uchar;
    while n > 0 as libc::c_int as libc::c_ulong {
        if *char_ptr as libc::c_int == c1 as libc::c_int
            || *char_ptr as libc::c_int == c2 as libc::c_int
        {
            return char_ptr as *mut libc::c_void;
        }
        n = n.wrapping_sub(1);
        n;
        char_ptr = char_ptr.offset(1);
        char_ptr;
    }
    return 0 as *mut libc::c_void;
}
