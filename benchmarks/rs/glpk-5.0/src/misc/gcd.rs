use ::libc;
extern "C" {
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
}
pub unsafe extern "C" fn _glp_gcd(
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    (x > 0 as libc::c_int && y > 0 as libc::c_int
        || {
            glp_assert_(
                b"x > 0 && y > 0\0" as *const u8 as *const libc::c_char,
                b"misc/gcd.c\0" as *const u8 as *const libc::c_char,
                52 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    while y > 0 as libc::c_int {
        r = x % y;
        x = y;
        y = r;
    }
    return x;
}
pub unsafe extern "C" fn _glp_gcdn(
    mut n: libc::c_int,
    mut x: *mut libc::c_int,
) -> libc::c_int {
    let mut d: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    (n > 0 as libc::c_int
        || {
            glp_assert_(
                b"n > 0\0" as *const u8 as *const libc::c_char,
                b"misc/gcd.c\0" as *const u8 as *const libc::c_char,
                87 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    j = 1 as libc::c_int;
    while j <= n {
        (*x.offset(j as isize) > 0 as libc::c_int
            || {
                glp_assert_(
                    b"x[j] > 0\0" as *const u8 as *const libc::c_char,
                    b"misc/gcd.c\0" as *const u8 as *const libc::c_char,
                    89 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if j == 1 as libc::c_int {
            d = *x.offset(1 as libc::c_int as isize);
        } else {
            d = _glp_gcd(d, *x.offset(j as isize));
        }
        if d == 1 as libc::c_int {
            break;
        }
        j += 1;
        j;
    }
    return d;
}
