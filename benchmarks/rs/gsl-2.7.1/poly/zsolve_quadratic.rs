use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_complex {
    pub dat: [libc::c_double; 2],
}
pub unsafe extern "C" fn gsl_poly_complex_solve_quadratic(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut c: libc::c_double,
    mut z0: *mut gsl_complex,
    mut z1: *mut gsl_complex,
) -> libc::c_int {
    let mut disc: libc::c_double = b * b - 4 as libc::c_int as libc::c_double * a * c;
    if a == 0 as libc::c_int as libc::c_double {
        if b == 0 as libc::c_int as libc::c_double {
            return 0 as libc::c_int
        } else {
            (*z0).dat[0 as libc::c_int as usize] = -c / b;
            (*z0).dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
            return 1 as libc::c_int;
        }
    }
    if disc > 0 as libc::c_int as libc::c_double {
        if b == 0 as libc::c_int as libc::c_double {
            let mut s: libc::c_double = fabs(0.5f64 * sqrt(disc) / a);
            (*z0).dat[0 as libc::c_int as usize] = -s;
            (*z0).dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
            (*z1).dat[0 as libc::c_int as usize] = s;
            (*z1).dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
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
                (*z0).dat[0 as libc::c_int as usize] = r1;
                (*z0)
                    .dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
                (*z1).dat[0 as libc::c_int as usize] = r2;
                (*z1)
                    .dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
            } else {
                (*z0).dat[0 as libc::c_int as usize] = r2;
                (*z0)
                    .dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
                (*z1).dat[0 as libc::c_int as usize] = r1;
                (*z1)
                    .dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
            }
        }
        return 2 as libc::c_int;
    } else if disc == 0 as libc::c_int as libc::c_double {
        (*z0).dat[0 as libc::c_int as usize] = -0.5f64 * b / a;
        (*z0).dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
        (*z1).dat[0 as libc::c_int as usize] = -0.5f64 * b / a;
        (*z1).dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
        return 2 as libc::c_int;
    } else {
        let mut s_0: libc::c_double = fabs(0.5f64 * sqrt(-disc) / a);
        (*z0).dat[0 as libc::c_int as usize] = -0.5f64 * b / a;
        (*z0).dat[1 as libc::c_int as usize] = -s_0;
        (*z1).dat[0 as libc::c_int as usize] = -0.5f64 * b / a;
        (*z1).dat[1 as libc::c_int as usize] = s_0;
        return 2 as libc::c_int;
    };
}
