use ::libc;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_complex {
    pub dat: [libc::c_double; 2],
}
pub unsafe extern "C" fn gsl_poly_dd_eval(
    mut dd: *const libc::c_double,
    mut xa: *const libc::c_double,
    size: size_t,
    x: libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut y: libc::c_double = *dd
        .offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    i = size.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    loop {
        let fresh0 = i;
        i = i.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        y = *dd.offset(i as isize) + (x - *xa.offset(i as isize)) * y;
    }
    return y;
}
pub unsafe extern "C" fn gsl_complex_poly_complex_eval(
    mut c: *const gsl_complex,
    len: libc::c_int,
    z: gsl_complex,
) -> gsl_complex {
    let mut i: libc::c_int = 0;
    let mut ans: gsl_complex = *c.offset((len - 1 as libc::c_int) as isize);
    i = len - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        let mut tmp: libc::c_double = (*c.offset((i - 1 as libc::c_int) as isize))
            .dat[0 as libc::c_int as usize]
            + z.dat[0 as libc::c_int as usize] * ans.dat[0 as libc::c_int as usize]
            - z.dat[1 as libc::c_int as usize] * ans.dat[1 as libc::c_int as usize];
        ans
            .dat[1 as libc::c_int
            as usize] = (*c.offset((i - 1 as libc::c_int) as isize))
            .dat[1 as libc::c_int as usize]
            + z.dat[1 as libc::c_int as usize] * ans.dat[0 as libc::c_int as usize]
            + z.dat[0 as libc::c_int as usize] * ans.dat[1 as libc::c_int as usize];
        ans.dat[0 as libc::c_int as usize] = tmp;
        i -= 1;
        i;
    }
    return ans;
}
pub unsafe extern "C" fn gsl_poly_complex_eval(
    mut c: *const libc::c_double,
    len: libc::c_int,
    z: gsl_complex,
) -> gsl_complex {
    let mut i: libc::c_int = 0;
    let mut ans: gsl_complex = gsl_complex { dat: [0.; 2] };
    ans.dat[0 as libc::c_int as usize] = *c.offset((len - 1 as libc::c_int) as isize);
    ans.dat[1 as libc::c_int as usize] = 0.0f64;
    i = len - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        let mut tmp: libc::c_double = *c.offset((i - 1 as libc::c_int) as isize)
            + z.dat[0 as libc::c_int as usize] * ans.dat[0 as libc::c_int as usize]
            - z.dat[1 as libc::c_int as usize] * ans.dat[1 as libc::c_int as usize];
        ans
            .dat[1 as libc::c_int
            as usize] = z.dat[1 as libc::c_int as usize]
            * ans.dat[0 as libc::c_int as usize]
            + z.dat[0 as libc::c_int as usize] * ans.dat[1 as libc::c_int as usize];
        ans.dat[0 as libc::c_int as usize] = tmp;
        i -= 1;
        i;
    }
    return ans;
}
pub unsafe extern "C" fn gsl_poly_eval(
    mut c: *const libc::c_double,
    len: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut ans: libc::c_double = *c.offset((len - 1 as libc::c_int) as isize);
    i = len - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        ans = *c.offset((i - 1 as libc::c_int) as isize) + x * ans;
        i -= 1;
        i;
    }
    return ans;
}
