use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub type C2RustUnnamed = libc::c_int;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
unsafe extern "C" fn backward_recurse_c(
    mut aa: libc::c_double,
    mut qq: libc::c_double,
    mut xx: libc::c_double,
    mut ff: *mut libc::c_double,
    mut gx: *mut libc::c_double,
    mut even_odd: libc::c_int,
    mut ni: libc::c_int,
) {
    let mut ii: libc::c_int = 0;
    let mut nn: libc::c_int = 0;
    let mut g1: libc::c_double = 0.;
    g1 = *gx;
    *ff.offset(ni as isize) = xx;
    if even_odd == 0 as libc::c_int {
        ii = 0 as libc::c_int;
        while ii < ni {
            nn = 100 as libc::c_int - ii - 1 as libc::c_int;
            *ff
                .offset(
                    (ni - ii - 1 as libc::c_int) as isize,
                ) = -1.0f64
                / (((4 as libc::c_int * nn * nn) as libc::c_double - aa) / qq
                    + *ff.offset((ni - ii) as isize));
            ii += 1;
            ii;
        }
        if ni == 100 as libc::c_int - 1 as libc::c_int {
            *ff.offset(0 as libc::c_int as isize) *= 2.0f64;
        }
    } else {
        ii = 0 as libc::c_int;
        while ii < ni {
            nn = 100 as libc::c_int - ii - 1 as libc::c_int;
            *ff
                .offset(
                    (ni - ii - 1 as libc::c_int) as isize,
                ) = -1.0f64
                / ((((2 as libc::c_int * nn + 1 as libc::c_int)
                    * (2 as libc::c_int * nn + 1 as libc::c_int)) as libc::c_double - aa)
                    / qq + *ff.offset((ni - ii) as isize));
            ii += 1;
            ii;
        }
    }
    *gx = *ff.offset(0 as libc::c_int as isize) - g1;
}
unsafe extern "C" fn backward_recurse_s(
    mut aa: libc::c_double,
    mut qq: libc::c_double,
    mut xx: libc::c_double,
    mut ff: *mut libc::c_double,
    mut gx: *mut libc::c_double,
    mut even_odd: libc::c_int,
    mut ni: libc::c_int,
) {
    let mut ii: libc::c_int = 0;
    let mut nn: libc::c_int = 0;
    let mut g1: libc::c_double = 0.;
    g1 = *gx;
    *ff.offset(ni as isize) = xx;
    if even_odd == 0 as libc::c_int {
        ii = 0 as libc::c_int;
        while ii < ni {
            nn = 100 as libc::c_int - ii - 1 as libc::c_int;
            *ff
                .offset(
                    (ni - ii - 1 as libc::c_int) as isize,
                ) = -1.0f64
                / (((4 as libc::c_int * (nn + 1 as libc::c_int)
                    * (nn + 1 as libc::c_int)) as libc::c_double - aa) / qq
                    + *ff.offset((ni - ii) as isize));
            ii += 1;
            ii;
        }
    } else {
        ii = 0 as libc::c_int;
        while ii < ni {
            nn = 100 as libc::c_int - ii - 1 as libc::c_int;
            *ff
                .offset(
                    (ni - ii - 1 as libc::c_int) as isize,
                ) = -1.0f64
                / ((((2 as libc::c_int * nn + 1 as libc::c_int)
                    * (2 as libc::c_int * nn + 1 as libc::c_int)) as libc::c_double - aa)
                    / qq + *ff.offset((ni - ii) as isize));
            ii += 1;
            ii;
        }
    }
    *gx = *ff.offset(0 as libc::c_int as isize) - g1;
}
pub unsafe extern "C" fn gsl_sf_mathieu_a_coeff(
    mut order: libc::c_int,
    mut qq: libc::c_double,
    mut aa: libc::c_double,
    mut coeff: *mut libc::c_double,
) -> libc::c_int {
    let mut ni: libc::c_int = 0;
    let mut nn: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut even_odd: libc::c_int = 0;
    let mut eps: libc::c_double = 0.;
    let mut g1: libc::c_double = 0.;
    let mut g2: libc::c_double = 0.;
    let mut x1: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut e1: libc::c_double = 0.;
    let mut e2: libc::c_double = 0.;
    let mut de: libc::c_double = 0.;
    let mut xh: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    let mut ratio: libc::c_double = 0.;
    let mut ff: [libc::c_double; 100] = [0.; 100];
    eps = 1e-14f64;
    *coeff.offset(0 as libc::c_int as isize) = 1.0f64;
    even_odd = 0 as libc::c_int;
    if order % 2 as libc::c_int != 0 as libc::c_int {
        even_odd = 1 as libc::c_int;
    }
    if order > 100 as libc::c_int {
        return GSL_FAILURE as libc::c_int;
    }
    if qq == 0.0f64 {
        ii = 0 as libc::c_int;
        while ii < 100 as libc::c_int {
            *coeff.offset(ii as isize) = 0.0f64;
            ii += 1;
            ii;
        }
        *coeff.offset((order / 2 as libc::c_int) as isize) = 1.0f64;
        return GSL_SUCCESS as libc::c_int;
    }
    if order < 5 as libc::c_int {
        nn = 0 as libc::c_int;
        sum = 0.0f64;
        if even_odd == 0 as libc::c_int {
            ratio = aa / qq;
        } else {
            ratio = (aa - 1 as libc::c_int as libc::c_double - qq) / qq;
        }
    } else {
        if even_odd == 0 as libc::c_int {
            *coeff.offset(1 as libc::c_int as isize) = aa / qq;
            *coeff
                .offset(
                    2 as libc::c_int as isize,
                ) = (aa - 4 as libc::c_int as libc::c_double) / qq
                * *coeff.offset(1 as libc::c_int as isize)
                - 2 as libc::c_int as libc::c_double;
            sum = *coeff.offset(0 as libc::c_int as isize)
                + *coeff.offset(1 as libc::c_int as isize)
                + *coeff.offset(2 as libc::c_int as isize);
            ii = 3 as libc::c_int;
            while ii < order / 2 as libc::c_int + 1 as libc::c_int {
                *coeff
                    .offset(
                        ii as isize,
                    ) = (aa
                    - (4 as libc::c_int * (ii - 1 as libc::c_int)
                        * (ii - 1 as libc::c_int)) as libc::c_double) / qq
                    * *coeff.offset((ii - 1 as libc::c_int) as isize)
                    - *coeff.offset((ii - 2 as libc::c_int) as isize);
                sum += *coeff.offset(ii as isize);
                ii += 1;
                ii;
            }
        } else {
            *coeff
                .offset(
                    1 as libc::c_int as isize,
                ) = (aa - 1 as libc::c_int as libc::c_double) / qq
                - 1 as libc::c_int as libc::c_double;
            sum = *coeff.offset(0 as libc::c_int as isize)
                + *coeff.offset(1 as libc::c_int as isize);
            ii = 2 as libc::c_int;
            while ii < order / 2 as libc::c_int + 1 as libc::c_int {
                *coeff
                    .offset(
                        ii as isize,
                    ) = (aa
                    - ((2 as libc::c_int * ii - 1 as libc::c_int)
                        * (2 as libc::c_int * ii - 1 as libc::c_int)) as libc::c_double)
                    / qq * *coeff.offset((ii - 1 as libc::c_int) as isize)
                    - *coeff.offset((ii - 2 as libc::c_int) as isize);
                sum += *coeff.offset(ii as isize);
                ii += 1;
                ii;
            }
        }
        nn = ii - 1 as libc::c_int;
        ratio = *coeff.offset(nn as isize)
            / *coeff.offset((nn - 1 as libc::c_int) as isize);
    }
    ni = 100 as libc::c_int - nn - 1 as libc::c_int;
    if even_odd == 0 as libc::c_int {
        x1 = -qq
            / (4.0f64 * 100 as libc::c_int as libc::c_double
                * 100 as libc::c_int as libc::c_double);
    } else {
        x1 = -qq
            / ((2.0f64 * 100 as libc::c_int as libc::c_double + 1.0f64)
                * (2.0f64 * 100 as libc::c_int as libc::c_double + 1.0f64));
    }
    g1 = ratio;
    backward_recurse_c(aa, qq, x1, ff.as_mut_ptr(), &mut g1, even_odd, ni);
    x2 = g1;
    g2 = ratio;
    backward_recurse_c(aa, qq, x2, ff.as_mut_ptr(), &mut g2, even_odd, ni);
    loop {
        e1 = g1 - x1;
        e2 = g2 - x2;
        de = e1 - e2;
        if fabs(de) < eps {
            break;
        }
        xh = (e1 * x2 - e2 * x1) / de;
        x1 = x2;
        g1 = g2;
        x2 = xh;
        g2 = ratio;
        backward_recurse_c(aa, qq, x2, ff.as_mut_ptr(), &mut g2, even_odd, ni);
    }
    sum += *coeff.offset(nn as isize);
    ii = nn + 1 as libc::c_int;
    while ii < 100 as libc::c_int {
        *coeff
            .offset(
                ii as isize,
            ) = ff[(ii - nn - 1 as libc::c_int) as usize]
            * *coeff.offset((ii - 1 as libc::c_int) as isize);
        sum += *coeff.offset(ii as isize);
        if fabs(*coeff.offset(ii as isize)) < 1e-20f64 {
            while ii < 100 as libc::c_int {
                let fresh0 = ii;
                ii = ii + 1;
                *coeff.offset(fresh0 as isize) = 0.0f64;
            }
        }
        ii += 1;
        ii;
    }
    ii = 0 as libc::c_int;
    while ii < 100 as libc::c_int {
        *coeff.offset(ii as isize) /= sum;
        ii += 1;
        ii;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_mathieu_b_coeff(
    mut order: libc::c_int,
    mut qq: libc::c_double,
    mut aa: libc::c_double,
    mut coeff: *mut libc::c_double,
) -> libc::c_int {
    let mut ni: libc::c_int = 0;
    let mut nn: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut even_odd: libc::c_int = 0;
    let mut eps: libc::c_double = 0.;
    let mut g1: libc::c_double = 0.;
    let mut g2: libc::c_double = 0.;
    let mut x1: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut e1: libc::c_double = 0.;
    let mut e2: libc::c_double = 0.;
    let mut de: libc::c_double = 0.;
    let mut xh: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    let mut ratio: libc::c_double = 0.;
    let mut ff: [libc::c_double; 100] = [0.; 100];
    eps = 1e-10f64;
    *coeff.offset(0 as libc::c_int as isize) = 1.0f64;
    even_odd = 0 as libc::c_int;
    if order % 2 as libc::c_int != 0 as libc::c_int {
        even_odd = 1 as libc::c_int;
    }
    if order > 100 as libc::c_int {
        return GSL_FAILURE as libc::c_int;
    }
    if qq == 0.0f64 {
        ii = 0 as libc::c_int;
        while ii < 100 as libc::c_int {
            *coeff.offset(ii as isize) = 0.0f64;
            ii += 1;
            ii;
        }
        *coeff.offset(((order - 1 as libc::c_int) / 2 as libc::c_int) as isize) = 1.0f64;
        return GSL_SUCCESS as libc::c_int;
    }
    if order < 5 as libc::c_int {
        nn = 0 as libc::c_int;
        sum = 0.0f64;
        if even_odd == 0 as libc::c_int {
            ratio = (aa - 4 as libc::c_int as libc::c_double) / qq;
        } else {
            ratio = (aa - 1 as libc::c_int as libc::c_double - qq) / qq;
        }
    } else {
        if even_odd == 0 as libc::c_int {
            *coeff
                .offset(
                    1 as libc::c_int as isize,
                ) = (aa - 4 as libc::c_int as libc::c_double) / qq;
            sum = 2 as libc::c_int as libc::c_double
                * *coeff.offset(0 as libc::c_int as isize)
                + 4 as libc::c_int as libc::c_double
                    * *coeff.offset(1 as libc::c_int as isize);
            ii = 2 as libc::c_int;
            while ii < order / 2 as libc::c_int {
                *coeff
                    .offset(
                        ii as isize,
                    ) = (aa - (4 as libc::c_int * ii * ii) as libc::c_double) / qq
                    * *coeff.offset((ii - 1 as libc::c_int) as isize)
                    - *coeff.offset((ii - 2 as libc::c_int) as isize);
                sum
                    += (2 as libc::c_int * (ii + 1 as libc::c_int)) as libc::c_double
                        * *coeff.offset(ii as isize);
                ii += 1;
                ii;
            }
        } else {
            *coeff
                .offset(
                    1 as libc::c_int as isize,
                ) = (aa - 1 as libc::c_int as libc::c_double) / qq
                + 1 as libc::c_int as libc::c_double;
            sum = *coeff.offset(0 as libc::c_int as isize)
                + 3 as libc::c_int as libc::c_double
                    * *coeff.offset(1 as libc::c_int as isize);
            ii = 2 as libc::c_int;
            while ii < order / 2 as libc::c_int + 1 as libc::c_int {
                *coeff
                    .offset(
                        ii as isize,
                    ) = (aa
                    - ((2 as libc::c_int * ii - 1 as libc::c_int)
                        * (2 as libc::c_int * ii - 1 as libc::c_int)) as libc::c_double)
                    / qq * *coeff.offset((ii - 1 as libc::c_int) as isize)
                    - *coeff.offset((ii - 2 as libc::c_int) as isize);
                sum
                    += (2 as libc::c_int * (ii + 1 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_double * *coeff.offset(ii as isize);
                ii += 1;
                ii;
            }
        }
        nn = ii - 1 as libc::c_int;
        ratio = *coeff.offset(nn as isize)
            / *coeff.offset((nn - 1 as libc::c_int) as isize);
    }
    ni = 100 as libc::c_int - nn - 1 as libc::c_int;
    if even_odd == 0 as libc::c_int {
        x1 = -qq
            / (4.0f64 * (100 as libc::c_int as libc::c_double + 1.0f64)
                * (100 as libc::c_int as libc::c_double + 1.0f64));
    } else {
        x1 = -qq
            / ((2.0f64 * 100 as libc::c_int as libc::c_double + 1.0f64)
                * (2.0f64 * 100 as libc::c_int as libc::c_double + 1.0f64));
    }
    g1 = ratio;
    backward_recurse_s(aa, qq, x1, ff.as_mut_ptr(), &mut g1, even_odd, ni);
    x2 = g1;
    g2 = ratio;
    backward_recurse_s(aa, qq, x2, ff.as_mut_ptr(), &mut g2, even_odd, ni);
    loop {
        e1 = g1 - x1;
        e2 = g2 - x2;
        de = e1 - e2;
        if fabs(de) < eps {
            break;
        }
        xh = (e1 * x2 - e2 * x1) / de;
        x1 = x2;
        g1 = g2;
        x2 = xh;
        g2 = ratio;
        backward_recurse_s(aa, qq, x2, ff.as_mut_ptr(), &mut g2, even_odd, ni);
    }
    sum
        += (2 as libc::c_int * (nn + 1 as libc::c_int)) as libc::c_double
            * *coeff.offset(nn as isize);
    ii = nn + 1 as libc::c_int;
    while ii < 100 as libc::c_int {
        *coeff
            .offset(
                ii as isize,
            ) = ff[(ii - nn - 1 as libc::c_int) as usize]
            * *coeff.offset((ii - 1 as libc::c_int) as isize);
        sum
            += (2 as libc::c_int * (ii + 1 as libc::c_int)) as libc::c_double
                * *coeff.offset(ii as isize);
        if fabs(*coeff.offset(ii as isize)) < 1e-20f64 {
            while ii < 100 as libc::c_int {
                let fresh1 = ii;
                ii = ii + 1;
                *coeff.offset(fresh1 as isize) = 0.0f64;
            }
        }
        ii += 1;
        ii;
    }
    ii = 0 as libc::c_int;
    while ii < 100 as libc::c_int {
        *coeff.offset(ii as isize) /= sum;
        ii += 1;
        ii;
    }
    return GSL_SUCCESS as libc::c_int;
}
