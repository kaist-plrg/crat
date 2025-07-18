use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
static mut base64: [libc::c_char; 65] = unsafe {
    *::std::mem::transmute::<
        &[u8; 65],
        &mut [libc::c_char; 65],
    >(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\0")
};
unsafe extern "C" fn pos(mut c: libc::c_char) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = base64.as_mut_ptr();
    while *p != 0 {
        if *p as libc::c_int == c as libc::c_int {
            return p.offset_from(base64.as_mut_ptr()) as libc::c_long as libc::c_int;
        }
        p = p.offset(1);
        p;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn base64_encode(
    mut data: *const libc::c_void,
    mut size: libc::c_int,
    mut str: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut q: *const libc::c_uchar = 0 as *const libc::c_uchar;
    s = malloc(
        (size * 4 as libc::c_int / 3 as libc::c_int + 4 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_char;
    p = s;
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    q = data as *const libc::c_uchar;
    i = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < size {
        let fresh0 = i;
        i = i + 1;
        c = *q.offset(fresh0 as isize) as libc::c_int;
        c *= 256 as libc::c_int;
        if i < size {
            c += *q.offset(i as isize) as libc::c_int;
        }
        i += 1;
        i;
        c *= 256 as libc::c_int;
        if i < size {
            c += *q.offset(i as isize) as libc::c_int;
        }
        i += 1;
        i;
        *p
            .offset(
                0 as libc::c_int as isize,
            ) = base64[((c & 0xfc0000 as libc::c_int) >> 18 as libc::c_int) as usize];
        *p
            .offset(
                1 as libc::c_int as isize,
            ) = base64[((c & 0x3f000 as libc::c_int) >> 12 as libc::c_int) as usize];
        *p
            .offset(
                2 as libc::c_int as isize,
            ) = base64[((c & 0xfc0 as libc::c_int) >> 6 as libc::c_int) as usize];
        *p
            .offset(
                3 as libc::c_int as isize,
            ) = base64[((c & 0x3f as libc::c_int) >> 0 as libc::c_int) as usize];
        if i > size {
            *p.offset(3 as libc::c_int as isize) = '=' as i32 as libc::c_char;
        }
        if i > size + 1 as libc::c_int {
            *p.offset(2 as libc::c_int as isize) = '=' as i32 as libc::c_char;
        }
        p = p.offset(4 as libc::c_int as isize);
    }
    *p = 0 as libc::c_int as libc::c_char;
    *str = s;
    return strlen(s) as libc::c_int;
}
pub unsafe extern "C" fn base64_decode(
    mut str: *const libc::c_char,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut c: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut done: libc::c_int = 0 as libc::c_int;
    q = data as *mut libc::c_uchar;
    p = str;
    while *p as libc::c_int != 0 && done == 0 {
        x = pos(*p.offset(0 as libc::c_int as isize));
        if x >= 0 as libc::c_int {
            c = x;
            c *= 64 as libc::c_int;
            x = pos(*p.offset(1 as libc::c_int as isize));
            if x >= 0 as libc::c_int {
                c += x;
            } else {
                return -(1 as libc::c_int)
            }
            c *= 64 as libc::c_int;
            if *p.offset(2 as libc::c_int as isize) as libc::c_int == '=' as i32 {
                done += 1;
                done;
            } else {
                x = pos(*p.offset(2 as libc::c_int as isize));
                if x >= 0 as libc::c_int {
                    c += x;
                } else {
                    return -(1 as libc::c_int)
                }
            }
            c *= 64 as libc::c_int;
            if *p.offset(3 as libc::c_int as isize) as libc::c_int == '=' as i32 {
                done += 1;
                done;
            } else {
                if done != 0 {
                    return -(1 as libc::c_int);
                }
                x = pos(*p.offset(3 as libc::c_int as isize));
                if x >= 0 as libc::c_int {
                    c += x;
                } else {
                    return -(1 as libc::c_int)
                }
            }
            if done < 3 as libc::c_int {
                let fresh1 = q;
                q = q.offset(1);
                *fresh1 = ((c & 0xff0000 as libc::c_int) >> 16 as libc::c_int)
                    as libc::c_uchar;
            }
            if done < 2 as libc::c_int {
                let fresh2 = q;
                q = q.offset(1);
                *fresh2 = ((c & 0xff00 as libc::c_int) >> 8 as libc::c_int)
                    as libc::c_uchar;
            }
            if done < 1 as libc::c_int {
                let fresh3 = q;
                q = q.offset(1);
                *fresh3 = ((c & 0xff as libc::c_int) >> 0 as libc::c_int)
                    as libc::c_uchar;
            }
            p = p.offset(4 as libc::c_int as isize);
        } else {
            done = 3 as libc::c_int;
            break;
        }
    }
    return q.offset_from(data as *mut libc::c_uchar) as libc::c_long as libc::c_int;
}
