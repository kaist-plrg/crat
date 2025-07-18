use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
}
pub unsafe extern "C" fn _glp_fp2rat(
    mut x: libc::c_double,
    mut eps: libc::c_double,
    mut p: *mut libc::c_double,
    mut q: *mut libc::c_double,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut xk: libc::c_double = 0.;
    let mut Akm1: libc::c_double = 0.;
    let mut Ak: libc::c_double = 0.;
    let mut Bkm1: libc::c_double = 0.;
    let mut Bk: libc::c_double = 0.;
    let mut ak: libc::c_double = 0.;
    let mut bk: libc::c_double = 0.;
    let mut fk: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    (0.0f64 <= x && x < 1.0f64
        || {
            glp_assert_(
                b"0.0 <= x && x < 1.0\0" as *const u8 as *const libc::c_char,
                b"misc/fp2rat.c\0" as *const u8 as *const libc::c_char,
                117 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = 0 as libc::c_int;
    loop {
        (k <= 100 as libc::c_int
            || {
                glp_assert_(
                    b"k <= 100\0" as *const u8 as *const libc::c_char,
                    b"misc/fp2rat.c\0" as *const u8 as *const libc::c_char,
                    119 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if k == 0 as libc::c_int {
            xk = x;
            Akm1 = 1.0f64;
            Ak = 0.0f64;
            Bkm1 = 0.0f64;
            Bk = 1.0f64;
        } else {
            temp = xk - floor(xk);
            (temp != 0.0f64
                || {
                    glp_assert_(
                        b"temp != 0.0\0" as *const u8 as *const libc::c_char,
                        b"misc/fp2rat.c\0" as *const u8 as *const libc::c_char,
                        135 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            xk = 1.0f64 / temp;
            ak = 1.0f64;
            bk = floor(xk);
            temp = bk * Ak + ak * Akm1;
            Akm1 = Ak;
            Ak = temp;
            temp = bk * Bk + ak * Bkm1;
            Bkm1 = Bk;
            Bk = temp;
        }
        fk = Ak / Bk;
        if fabs(x - fk) <= eps {
            break;
        }
        k += 1;
        k;
    }
    *p = Ak;
    *q = Bk;
    return k;
}
