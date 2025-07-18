use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn gsl_poly_solve_quadratic(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut c: libc::c_double,
    mut x0: *mut libc::c_double,
    mut x1: *mut libc::c_double,
) -> libc::c_int {
    if a == 0 as libc::c_int as libc::c_double {
        if b == 0 as libc::c_int as libc::c_double {
            return 0 as libc::c_int
        } else {
            *x0 = -c / b;
            return 1 as libc::c_int;
        }
    }
    let mut disc: libc::c_double = b * b - 4 as libc::c_int as libc::c_double * a * c;
    if disc > 0 as libc::c_int as libc::c_double {
        if b == 0 as libc::c_int as libc::c_double {
            let mut r: libc::c_double = sqrt(-c / a);
            *x0 = -r;
            *x1 = r;
        } else {
            let mut sgnb: libc::c_double = (if b > 0 as libc::c_int as libc::c_double {
                1 as libc::c_int
            } else {
                -(1 as libc::c_int)
            }) as libc::c_double;
            let mut temp: libc::c_double = -0.5f64 * (b + sgnb * sqrt(disc));
            let mut r1: libc::c_double = temp / a;
            let mut r2: libc::c_double = c / temp;
            if r1 < r2 {
                *x0 = r1;
                *x1 = r2;
            } else {
                *x0 = r2;
                *x1 = r1;
            }
        }
        return 2 as libc::c_int;
    } else if disc == 0 as libc::c_int as libc::c_double {
        *x0 = -0.5f64 * b / a;
        *x1 = -0.5f64 * b / a;
        return 2 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
