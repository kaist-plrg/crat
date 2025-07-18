use ::libc;
#[inline]
unsafe extern "C" fn c_tolower(mut c: libc::c_int) -> libc::c_int {
    match c {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
        | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => {
            return c - 'A' as i32 + 'a' as i32;
        }
        _ => return c,
    };
}
pub unsafe extern "C" fn c_strcasecmp(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
) -> libc::c_int {
    let mut p1: *const libc::c_uchar = s1 as *const libc::c_uchar;
    let mut p2: *const libc::c_uchar = s2 as *const libc::c_uchar;
    let mut c1: libc::c_uchar = 0;
    let mut c2: libc::c_uchar = 0;
    if p1 == p2 {
        return 0 as libc::c_int;
    }
    loop {
        c1 = c_tolower(*p1 as libc::c_int) as libc::c_uchar;
        c2 = c_tolower(*p2 as libc::c_int) as libc::c_uchar;
        if c1 as libc::c_int == '\0' as i32 {
            break;
        }
        p1 = p1.offset(1);
        p1;
        p2 = p2.offset(1);
        p2;
        if !(c1 as libc::c_int == c2 as libc::c_int) {
            break;
        }
    }
    if 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
        <= 2147483647 as libc::c_int
    {
        return c1 as libc::c_int - c2 as libc::c_int
    } else {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int
            - ((c1 as libc::c_int) < c2 as libc::c_int) as libc::c_int
    };
}
