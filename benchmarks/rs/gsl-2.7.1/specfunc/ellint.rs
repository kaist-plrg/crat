use ::libc;
extern "C" {
    fn sin(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    static gsl_prec_eps: [libc::c_double; 0];
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
}
pub type gsl_prec_t = libc::c_uint;
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
pub type gsl_mode_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_sf_result_struct {
    pub val: libc::c_double,
    pub err: libc::c_double,
}
pub type gsl_sf_result = gsl_sf_result_struct;
#[inline]
unsafe extern "C" fn GSL_MODE_PREC(mut mt: gsl_mode_t) -> libc::c_uint {
    return mt & 7 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn locMAX3(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut z: libc::c_double,
) -> libc::c_double {
    let mut xy: libc::c_double = if x > y { x } else { y };
    return if xy > z { xy } else { z };
}
#[inline]
unsafe extern "C" fn locMAX4(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut z: libc::c_double,
    mut w: libc::c_double,
) -> libc::c_double {
    let mut xy: libc::c_double = if x > y { x } else { y };
    let mut xyz: libc::c_double = if xy > z { xy } else { z };
    return if xyz > w { xyz } else { w };
}
pub unsafe extern "C" fn gsl_sf_ellint_RC_e(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut mode: gsl_mode_t,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let lolim: libc::c_double = 5.0f64 * 2.2250738585072014e-308f64;
    let uplim: libc::c_double = 0.2f64 * 1.7976931348623157e+308f64;
    let goal: gsl_prec_t = GSL_MODE_PREC(mode);
    let errtol: libc::c_double = if goal == 0 as libc::c_int as libc::c_uint {
        0.001f64
    } else {
        0.03f64
    };
    let prec: libc::c_double = *gsl_prec_eps.as_ptr().offset(goal as isize);
    let nmax: libc::c_int = 10000 as libc::c_int;
    if x < 0.0f64 || y < 0.0f64 || x + y < lolim {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if (if x > y { x } else { y }) < uplim {
        let c1: libc::c_double = 1.0f64 / 7.0f64;
        let c2: libc::c_double = 9.0f64 / 22.0f64;
        let mut xn: libc::c_double = x;
        let mut yn: libc::c_double = y;
        let mut mu: libc::c_double = 0.;
        let mut sn: libc::c_double = 0.;
        let mut lamda: libc::c_double = 0.;
        let mut s: libc::c_double = 0.;
        let mut n: libc::c_int = 0 as libc::c_int;
        loop {
            mu = (xn + yn + yn) / 3.0f64;
            sn = (yn + mu) / mu - 2.0f64;
            if fabs(sn) < errtol {
                break;
            }
            lamda = 2.0f64 * sqrt(xn) * sqrt(yn) + yn;
            xn = (xn + lamda) * 0.25f64;
            yn = (yn + lamda) * 0.25f64;
            n += 1;
            n;
            if n == nmax {
                (*result).val = ::std::f32::NAN as libc::c_double;
                (*result).err = ::std::f32::NAN as libc::c_double;
                gsl_error(
                    b"too many iterations error\0" as *const u8 as *const libc::c_char,
                    b"ellint.c\0" as *const u8 as *const libc::c_char,
                    102 as libc::c_int,
                    GSL_EMAXITER as libc::c_int,
                );
                return GSL_EMAXITER as libc::c_int;
            }
        }
        s = sn * sn * (0.3f64 + sn * (c1 + sn * (0.375f64 + sn * c2)));
        (*result).val = (1.0f64 + s) / sqrt(mu);
        (*result).err = prec * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_ellint_RD_e(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut z: libc::c_double,
    mut mode: gsl_mode_t,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let goal: gsl_prec_t = GSL_MODE_PREC(mode);
    let errtol: libc::c_double = if goal == 0 as libc::c_int as libc::c_uint {
        0.001f64
    } else {
        0.03f64
    };
    let prec: libc::c_double = *gsl_prec_eps.as_ptr().offset(goal as isize);
    let lolim: libc::c_double = 2.0f64
        / pow(1.7976931348623157e+308f64, 2.0f64 / 3.0f64);
    let uplim: libc::c_double = pow(
        0.1f64 * errtol / 2.2250738585072014e-308f64,
        2.0f64 / 3.0f64,
    );
    let nmax: libc::c_int = 10000 as libc::c_int;
    if (if x < y { x } else { y }) < 0.0f64
        || (if x + y < z { x + y } else { z }) < lolim
    {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if locMAX3(x, y, z) < uplim {
        let c1: libc::c_double = 3.0f64 / 14.0f64;
        let c2: libc::c_double = 1.0f64 / 6.0f64;
        let c3: libc::c_double = 9.0f64 / 22.0f64;
        let c4: libc::c_double = 3.0f64 / 26.0f64;
        let mut xn: libc::c_double = x;
        let mut yn: libc::c_double = y;
        let mut zn: libc::c_double = z;
        let mut sigma: libc::c_double = 0.0f64;
        let mut power4: libc::c_double = 1.0f64;
        let mut ea: libc::c_double = 0.;
        let mut eb: libc::c_double = 0.;
        let mut ec: libc::c_double = 0.;
        let mut ed: libc::c_double = 0.;
        let mut ef: libc::c_double = 0.;
        let mut s1: libc::c_double = 0.;
        let mut s2: libc::c_double = 0.;
        let mut mu: libc::c_double = 0.;
        let mut xndev: libc::c_double = 0.;
        let mut yndev: libc::c_double = 0.;
        let mut zndev: libc::c_double = 0.;
        let mut n: libc::c_int = 0 as libc::c_int;
        loop {
            let mut xnroot: libc::c_double = 0.;
            let mut ynroot: libc::c_double = 0.;
            let mut znroot: libc::c_double = 0.;
            let mut lamda: libc::c_double = 0.;
            let mut epslon: libc::c_double = 0.;
            mu = (xn + yn + 3.0f64 * zn) * 0.2f64;
            xndev = (mu - xn) / mu;
            yndev = (mu - yn) / mu;
            zndev = (mu - zn) / mu;
            epslon = locMAX3(fabs(xndev), fabs(yndev), fabs(zndev));
            if epslon < errtol {
                break;
            }
            xnroot = sqrt(xn);
            ynroot = sqrt(yn);
            znroot = sqrt(zn);
            lamda = xnroot * (ynroot + znroot) + ynroot * znroot;
            sigma += power4 / (znroot * (zn + lamda));
            power4 *= 0.25f64;
            xn = (xn + lamda) * 0.25f64;
            yn = (yn + lamda) * 0.25f64;
            zn = (zn + lamda) * 0.25f64;
            n += 1;
            n;
            if n == nmax {
                (*result).val = ::std::f32::NAN as libc::c_double;
                (*result).err = ::std::f32::NAN as libc::c_double;
                gsl_error(
                    b"too many iterations error\0" as *const u8 as *const libc::c_char,
                    b"ellint.c\0" as *const u8 as *const libc::c_char,
                    162 as libc::c_int,
                    GSL_EMAXITER as libc::c_int,
                );
                return GSL_EMAXITER as libc::c_int;
            }
        }
        ea = xndev * yndev;
        eb = zndev * zndev;
        ec = ea - eb;
        ed = ea - 6.0f64 * eb;
        ef = ed + ec + ec;
        s1 = ed * (-c1 + 0.25f64 * c3 * ed - 1.5f64 * c4 * zndev * ef);
        s2 = zndev * (c2 * ef + zndev * (-c3 * ec + zndev * c4 * ea));
        (*result).val = 3.0f64 * sigma + power4 * (1.0f64 + s1 + s2) / (mu * sqrt(mu));
        (*result).err = prec * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            177 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_ellint_RF_e(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut z: libc::c_double,
    mut mode: gsl_mode_t,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let lolim: libc::c_double = 5.0f64 * 2.2250738585072014e-308f64;
    let uplim: libc::c_double = 0.2f64 * 1.7976931348623157e+308f64;
    let goal: gsl_prec_t = GSL_MODE_PREC(mode);
    let errtol: libc::c_double = if goal == 0 as libc::c_int as libc::c_uint {
        0.001f64
    } else {
        0.03f64
    };
    let prec: libc::c_double = *gsl_prec_eps.as_ptr().offset(goal as isize);
    let nmax: libc::c_int = 10000 as libc::c_int;
    if x < 0.0f64 || y < 0.0f64 || z < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            193 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x + y < lolim || x + z < lolim || y + z < lolim {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            196 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if locMAX3(x, y, z) < uplim {
        let c1: libc::c_double = 1.0f64 / 24.0f64;
        let c2: libc::c_double = 3.0f64 / 44.0f64;
        let c3: libc::c_double = 1.0f64 / 14.0f64;
        let mut xn: libc::c_double = x;
        let mut yn: libc::c_double = y;
        let mut zn: libc::c_double = z;
        let mut mu: libc::c_double = 0.;
        let mut xndev: libc::c_double = 0.;
        let mut yndev: libc::c_double = 0.;
        let mut zndev: libc::c_double = 0.;
        let mut e2: libc::c_double = 0.;
        let mut e3: libc::c_double = 0.;
        let mut s: libc::c_double = 0.;
        let mut n: libc::c_int = 0 as libc::c_int;
        loop {
            let mut epslon: libc::c_double = 0.;
            let mut lamda: libc::c_double = 0.;
            let mut xnroot: libc::c_double = 0.;
            let mut ynroot: libc::c_double = 0.;
            let mut znroot: libc::c_double = 0.;
            mu = (xn + yn + zn) / 3.0f64;
            xndev = 2.0f64 - (mu + xn) / mu;
            yndev = 2.0f64 - (mu + yn) / mu;
            zndev = 2.0f64 - (mu + zn) / mu;
            epslon = locMAX3(fabs(xndev), fabs(yndev), fabs(zndev));
            if epslon < errtol {
                break;
            }
            xnroot = sqrt(xn);
            ynroot = sqrt(yn);
            znroot = sqrt(zn);
            lamda = xnroot * (ynroot + znroot) + ynroot * znroot;
            xn = (xn + lamda) * 0.25f64;
            yn = (yn + lamda) * 0.25f64;
            zn = (zn + lamda) * 0.25f64;
            n += 1;
            n;
            if n == nmax {
                (*result).val = ::std::f32::NAN as libc::c_double;
                (*result).err = ::std::f32::NAN as libc::c_double;
                gsl_error(
                    b"too many iterations error\0" as *const u8 as *const libc::c_char,
                    b"ellint.c\0" as *const u8 as *const libc::c_char,
                    225 as libc::c_int,
                    GSL_EMAXITER as libc::c_int,
                );
                return GSL_EMAXITER as libc::c_int;
            }
        }
        e2 = xndev * yndev - zndev * zndev;
        e3 = xndev * yndev * zndev;
        s = 1.0f64 + (c1 * e2 - 0.1f64 - c2 * e3) * e2 + c3 * e3;
        (*result).val = s / sqrt(mu);
        (*result).err = prec * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            236 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_ellint_RJ_e(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut z: libc::c_double,
    mut p: libc::c_double,
    mut mode: gsl_mode_t,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let goal: gsl_prec_t = GSL_MODE_PREC(mode);
    let errtol: libc::c_double = if goal == 0 as libc::c_int as libc::c_uint {
        0.001f64
    } else {
        0.03f64
    };
    let prec: libc::c_double = *gsl_prec_eps.as_ptr().offset(goal as isize);
    let lolim: libc::c_double = pow(
        5.0f64 * 2.2250738585072014e-308f64,
        1.0f64 / 3.0f64,
    );
    let uplim: libc::c_double = 0.3f64
        * pow(0.2f64 * 1.7976931348623157e+308f64, 1.0f64 / 3.0f64);
    let nmax: libc::c_int = 10000 as libc::c_int;
    if x < 0.0f64 || y < 0.0f64 || z < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            252 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x + y < lolim || x + z < lolim || y + z < lolim || p < lolim {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            255 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if locMAX4(x, y, z, p) < uplim {
        let c1: libc::c_double = 3.0f64 / 14.0f64;
        let c2: libc::c_double = 1.0f64 / 3.0f64;
        let c3: libc::c_double = 3.0f64 / 22.0f64;
        let c4: libc::c_double = 3.0f64 / 26.0f64;
        let mut xn: libc::c_double = x;
        let mut yn: libc::c_double = y;
        let mut zn: libc::c_double = z;
        let mut pn: libc::c_double = p;
        let mut sigma: libc::c_double = 0.0f64;
        let mut power4: libc::c_double = 1.0f64;
        let mut mu: libc::c_double = 0.;
        let mut xndev: libc::c_double = 0.;
        let mut yndev: libc::c_double = 0.;
        let mut zndev: libc::c_double = 0.;
        let mut pndev: libc::c_double = 0.;
        let mut ea: libc::c_double = 0.;
        let mut eb: libc::c_double = 0.;
        let mut ec: libc::c_double = 0.;
        let mut e2: libc::c_double = 0.;
        let mut e3: libc::c_double = 0.;
        let mut s1: libc::c_double = 0.;
        let mut s2: libc::c_double = 0.;
        let mut s3: libc::c_double = 0.;
        let mut n: libc::c_int = 0 as libc::c_int;
        loop {
            let mut xnroot: libc::c_double = 0.;
            let mut ynroot: libc::c_double = 0.;
            let mut znroot: libc::c_double = 0.;
            let mut lamda: libc::c_double = 0.;
            let mut alfa: libc::c_double = 0.;
            let mut beta: libc::c_double = 0.;
            let mut epslon: libc::c_double = 0.;
            let mut rcresult: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut rcstatus: libc::c_int = 0;
            mu = (xn + yn + zn + pn + pn) * 0.2f64;
            xndev = (mu - xn) / mu;
            yndev = (mu - yn) / mu;
            zndev = (mu - zn) / mu;
            pndev = (mu - pn) / mu;
            epslon = locMAX4(fabs(xndev), fabs(yndev), fabs(zndev), fabs(pndev));
            if epslon < errtol {
                break;
            }
            xnroot = sqrt(xn);
            ynroot = sqrt(yn);
            znroot = sqrt(zn);
            lamda = xnroot * (ynroot + znroot) + ynroot * znroot;
            alfa = pn * (xnroot + ynroot + znroot) + xnroot * ynroot * znroot;
            alfa = alfa * alfa;
            beta = pn * (pn + lamda) * (pn + lamda);
            rcstatus = gsl_sf_ellint_RC_e(alfa, beta, mode, &mut rcresult);
            if rcstatus != GSL_SUCCESS as libc::c_int {
                (*result).val = 0.0f64;
                (*result).err = 0.0f64;
                return rcstatus;
            }
            sigma += power4 * rcresult.val;
            power4 *= 0.25f64;
            xn = (xn + lamda) * 0.25f64;
            yn = (yn + lamda) * 0.25f64;
            zn = (zn + lamda) * 0.25f64;
            pn = (pn + lamda) * 0.25f64;
            n += 1;
            n;
            if n == nmax {
                (*result).val = ::std::f32::NAN as libc::c_double;
                (*result).err = ::std::f32::NAN as libc::c_double;
                gsl_error(
                    b"too many iterations error\0" as *const u8 as *const libc::c_char,
                    b"ellint.c\0" as *const u8 as *const libc::c_char,
                    305 as libc::c_int,
                    GSL_EMAXITER as libc::c_int,
                );
                return GSL_EMAXITER as libc::c_int;
            }
        }
        ea = xndev * (yndev + zndev) + yndev * zndev;
        eb = xndev * yndev * zndev;
        ec = pndev * pndev;
        e2 = ea - 3.0f64 * ec;
        e3 = eb + 2.0f64 * pndev * (ea - ec);
        s1 = 1.0f64 + e2 * (-c1 + 0.75f64 * c3 * e2 - 1.5f64 * c4 * e3);
        s2 = eb * (0.5f64 * c2 + pndev * (-c3 - c3 + pndev * c4));
        s3 = pndev * ea * (c2 - pndev * c3) - c2 * pndev * ec;
        (*result).val = 3.0f64 * sigma + power4 * (s1 + s2 + s3) / (mu * sqrt(mu));
        (*result).err = prec * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            322 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_ellint_F_e(
    mut phi: libc::c_double,
    mut k: libc::c_double,
    mut mode: gsl_mode_t,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut nc: libc::c_double = floor(phi / 3.14159265358979323846f64 + 0.5f64);
    let mut phi_red: libc::c_double = phi - nc * 3.14159265358979323846f64;
    phi = phi_red;
    let mut sin_phi: libc::c_double = sin(phi);
    let mut sin2_phi: libc::c_double = sin_phi * sin_phi;
    let mut x: libc::c_double = 1.0f64 - sin2_phi;
    let mut y: libc::c_double = 1.0f64 - k * k * sin2_phi;
    let mut rf: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_ellint_RF_e(x, y, 1.0f64, mode, &mut rf);
    (*result).val = sin_phi * rf.val;
    (*result)
        .err = 2.2204460492503131e-16f64 * fabs((*result).val) + fabs(sin_phi * rf.err);
    if nc == 0 as libc::c_int as libc::c_double {
        return status
    } else {
        let mut rk: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let rkstatus: libc::c_int = gsl_sf_ellint_Kcomp_e(k, mode, &mut rk);
        (*result).val += 2 as libc::c_int as libc::c_double * nc * rk.val;
        (*result).err += 2 as libc::c_int as libc::c_double * fabs(nc) * rk.err;
        return if status != GSL_SUCCESS as libc::c_int {
            status
        } else if rkstatus != GSL_SUCCESS as libc::c_int {
            rkstatus
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_ellint_E_e(
    mut phi: libc::c_double,
    mut k: libc::c_double,
    mut mode: gsl_mode_t,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut nc: libc::c_double = floor(phi / 3.14159265358979323846f64 + 0.5f64);
    let mut phi_red: libc::c_double = phi - nc * 3.14159265358979323846f64;
    phi = phi_red;
    let sin_phi: libc::c_double = sin(phi);
    let sin2_phi: libc::c_double = sin_phi * sin_phi;
    let x: libc::c_double = 1.0f64 - sin2_phi;
    let y: libc::c_double = 1.0f64 - k * k * sin2_phi;
    if x < 2.2204460492503131e-16f64 {
        let mut re: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let status: libc::c_int = gsl_sf_ellint_Ecomp_e(k, mode, &mut re);
        (*result)
            .val = 2 as libc::c_int as libc::c_double * nc * re.val
            + (if sin_phi >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
                as libc::c_double * re.val;
        (*result).err = 2 as libc::c_int as libc::c_double * fabs(nc) * re.err + re.err;
        return status;
    } else {
        let mut rf: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut rd: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let sin3_phi: libc::c_double = sin2_phi * sin_phi;
        let rfstatus: libc::c_int = gsl_sf_ellint_RF_e(x, y, 1.0f64, mode, &mut rf);
        let rdstatus: libc::c_int = gsl_sf_ellint_RD_e(x, y, 1.0f64, mode, &mut rd);
        (*result).val = sin_phi * rf.val - k * k / 3.0f64 * sin3_phi * rd.val;
        (*result).err = 2.2204460492503131e-16f64 * fabs(sin_phi * rf.val);
        (*result).err += fabs(sin_phi * rf.err);
        (*result).err
            += k * k / 3.0f64 * 2.2204460492503131e-16f64 * fabs(sin3_phi * rd.val);
        (*result).err += k * k / 3.0f64 * fabs(sin3_phi * rd.err);
        if nc == 0 as libc::c_int as libc::c_double {
            return if rfstatus != GSL_SUCCESS as libc::c_int {
                rfstatus
            } else if rdstatus != GSL_SUCCESS as libc::c_int {
                rdstatus
            } else {
                GSL_SUCCESS as libc::c_int
            }
        } else {
            let mut re_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let restatus: libc::c_int = gsl_sf_ellint_Ecomp_e(k, mode, &mut re_0);
            (*result).val += 2 as libc::c_int as libc::c_double * nc * re_0.val;
            (*result).err += 2 as libc::c_int as libc::c_double * fabs(nc) * re_0.err;
            return if rfstatus != GSL_SUCCESS as libc::c_int {
                rfstatus
            } else if rdstatus != GSL_SUCCESS as libc::c_int {
                rdstatus
            } else if restatus != GSL_SUCCESS as libc::c_int {
                restatus
            } else {
                GSL_SUCCESS as libc::c_int
            };
        }
    };
}
pub unsafe extern "C" fn gsl_sf_ellint_P_e(
    mut phi: libc::c_double,
    mut k: libc::c_double,
    mut n: libc::c_double,
    mut mode: gsl_mode_t,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut nc: libc::c_double = floor(phi / 3.14159265358979323846f64 + 0.5f64);
    let mut phi_red: libc::c_double = phi - nc * 3.14159265358979323846f64;
    phi = phi_red;
    let sin_phi: libc::c_double = sin(phi);
    let sin2_phi: libc::c_double = sin_phi * sin_phi;
    let sin3_phi: libc::c_double = sin2_phi * sin_phi;
    let x: libc::c_double = 1.0f64 - sin2_phi;
    let y: libc::c_double = 1.0f64 - k * k * sin2_phi;
    let mut rf: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut rj: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let rfstatus: libc::c_int = gsl_sf_ellint_RF_e(x, y, 1.0f64, mode, &mut rf);
    let rjstatus: libc::c_int = gsl_sf_ellint_RJ_e(
        x,
        y,
        1.0f64,
        1.0f64 + n * sin2_phi,
        mode,
        &mut rj,
    );
    (*result).val = sin_phi * rf.val - n / 3.0f64 * sin3_phi * rj.val;
    (*result).err = 2.2204460492503131e-16f64 * fabs(sin_phi * rf.val);
    (*result).err += fabs(sin_phi * rf.err);
    (*result).err += n / 3.0f64 * 2.2204460492503131e-16f64 * fabs(sin3_phi * rj.val);
    (*result).err += n / 3.0f64 * fabs(sin3_phi * rj.err);
    if nc == 0 as libc::c_int as libc::c_double {
        return if rfstatus != GSL_SUCCESS as libc::c_int {
            rfstatus
        } else if rjstatus != GSL_SUCCESS as libc::c_int {
            rjstatus
        } else {
            GSL_SUCCESS as libc::c_int
        }
    } else {
        let mut rp: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let rpstatus: libc::c_int = gsl_sf_ellint_Pcomp_e(k, n, mode, &mut rp);
        (*result).val += 2 as libc::c_int as libc::c_double * nc * rp.val;
        (*result).err += 2 as libc::c_int as libc::c_double * fabs(nc) * rp.err;
        return if rfstatus != GSL_SUCCESS as libc::c_int {
            rfstatus
        } else if rjstatus != GSL_SUCCESS as libc::c_int {
            rjstatus
        } else if rpstatus != GSL_SUCCESS as libc::c_int {
            rpstatus
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_ellint_D_e(
    mut phi: libc::c_double,
    mut k: libc::c_double,
    mut mode: gsl_mode_t,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut nc: libc::c_double = floor(phi / 3.14159265358979323846f64 + 0.5f64);
    let mut phi_red: libc::c_double = phi - nc * 3.14159265358979323846f64;
    phi = phi_red;
    let sin_phi: libc::c_double = sin(phi);
    let sin2_phi: libc::c_double = sin_phi * sin_phi;
    let sin3_phi: libc::c_double = sin2_phi * sin_phi;
    let x: libc::c_double = 1.0f64 - sin2_phi;
    let y: libc::c_double = 1.0f64 - k * k * sin2_phi;
    let mut rd: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let status: libc::c_int = gsl_sf_ellint_RD_e(x, y, 1.0f64, mode, &mut rd);
    (*result).val = sin3_phi / 3.0f64 * rd.val;
    (*result)
        .err = 2.2204460492503131e-16f64 * fabs((*result).val)
        + fabs(sin3_phi / 3.0f64 * rd.err);
    if nc == 0 as libc::c_int as libc::c_double {
        return status
    } else {
        let mut rd_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let rdstatus: libc::c_int = gsl_sf_ellint_Dcomp_e(k, mode, &mut rd_0);
        (*result).val += 2 as libc::c_int as libc::c_double * nc * rd_0.val;
        (*result).err += 2 as libc::c_int as libc::c_double * fabs(nc) * rd_0.err;
        return if status != GSL_SUCCESS as libc::c_int {
            status
        } else if rdstatus != GSL_SUCCESS as libc::c_int {
            rdstatus
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_ellint_Dcomp_e(
    mut k: libc::c_double,
    mut mode: gsl_mode_t,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if k * k >= 1.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            488 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let y: libc::c_double = 1.0f64 - k * k;
        let mut rd: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let status: libc::c_int = gsl_sf_ellint_RD_e(0.0f64, y, 1.0f64, mode, &mut rd);
        (*result).val = 1.0f64 / 3.0f64 * rd.val;
        (*result)
            .err = 2.2204460492503131e-16f64 * fabs((*result).val)
            + fabs(1.0f64 / 3.0f64 * rd.err);
        return status;
    };
}
pub unsafe extern "C" fn gsl_sf_ellint_Kcomp_e(
    mut k: libc::c_double,
    mut mode: gsl_mode_t,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if k * k >= 1.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            505 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if k * k >= 1.0f64 - 1.4901161193847656e-08f64 {
        let y: libc::c_double = 1.0f64 - k * k;
        let a: [libc::c_double; 3] = [
            1.38629436112f64,
            0.09666344259f64,
            0.03590092383f64,
        ];
        let b: [libc::c_double; 3] = [0.5f64, 0.12498593597f64, 0.06880248576f64];
        let ta: libc::c_double = a[0 as libc::c_int as usize]
            + y * (a[1 as libc::c_int as usize] + y * a[2 as libc::c_int as usize]);
        let tb: libc::c_double = -log(y)
            * (b[0 as libc::c_int as usize]
                + y * (b[1 as libc::c_int as usize] + y * b[2 as libc::c_int as usize]));
        (*result).val = ta + tb;
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64
            * (fabs((*result).val) + fabs(k / y));
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut y_0: libc::c_double = 1.0f64 - k * k;
        let mut status: libc::c_int = gsl_sf_ellint_RF_e(
            0.0f64,
            y_0,
            1.0f64,
            mode,
            result,
        );
        (*result).err += 0.5f64 * 2.2204460492503131e-16f64 / y_0;
        return status;
    };
}
pub unsafe extern "C" fn gsl_sf_ellint_Ecomp_e(
    mut k: libc::c_double,
    mut mode: gsl_mode_t,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if k * k >= 1.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            542 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if k * k >= 1.0f64 - 1.4901161193847656e-08f64 {
        let y: libc::c_double = 1.0f64 - k * k;
        let a: [libc::c_double; 3] = [
            0.44325141463f64,
            0.06260601220f64,
            0.04757383546f64,
        ];
        let b: [libc::c_double; 3] = [
            0.24998368310f64,
            0.09200180037f64,
            0.04069697526f64,
        ];
        let ta: libc::c_double = 1.0f64
            + y
                * (a[0 as libc::c_int as usize]
                    + y
                        * (a[1 as libc::c_int as usize]
                            + a[2 as libc::c_int as usize] * y));
        let tb: libc::c_double = -y * log(y)
            * (b[0 as libc::c_int as usize]
                + y * (b[1 as libc::c_int as usize] + b[2 as libc::c_int as usize] * y));
        (*result).val = ta + tb;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut rf: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut rd: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let y_0: libc::c_double = 1.0f64 - k * k;
        let rfstatus: libc::c_int = gsl_sf_ellint_RF_e(
            0.0f64,
            y_0,
            1.0f64,
            mode,
            &mut rf,
        );
        let rdstatus: libc::c_int = gsl_sf_ellint_RD_e(
            0.0f64,
            y_0,
            1.0f64,
            mode,
            &mut rd,
        );
        (*result).val = rf.val - k * k / 3.0f64 * rd.val;
        (*result).err = rf.err + k * k / 3.0f64 * rd.err;
        return if rfstatus != GSL_SUCCESS as libc::c_int {
            rfstatus
        } else if rdstatus != GSL_SUCCESS as libc::c_int {
            rdstatus
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_ellint_Pcomp_e(
    mut k: libc::c_double,
    mut n: libc::c_double,
    mut mode: gsl_mode_t,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if k * k >= 1.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            572 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let mut rf: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut rj: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let y: libc::c_double = 1.0f64 - k * k;
        let rfstatus: libc::c_int = gsl_sf_ellint_RF_e(0.0f64, y, 1.0f64, mode, &mut rf);
        let rjstatus: libc::c_int = gsl_sf_ellint_RJ_e(
            0.0f64,
            y,
            1.0f64,
            1.0f64 + n,
            mode,
            &mut rj,
        );
        (*result).val = rf.val - n / 3.0f64 * rj.val;
        (*result).err = rf.err + fabs(n / 3.0f64) * rj.err;
        return if rfstatus != GSL_SUCCESS as libc::c_int {
            rfstatus
        } else if rjstatus != GSL_SUCCESS as libc::c_int {
            rjstatus
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_ellint_Kcomp(
    mut k: libc::c_double,
    mut mode: gsl_mode_t,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_ellint_Kcomp_e(k, mode, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_ellint_Kcomp_e(k, mode, &result)\0" as *const u8
                as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            595 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_ellint_Ecomp(
    mut k: libc::c_double,
    mut mode: gsl_mode_t,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_ellint_Ecomp_e(k, mode, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_ellint_Ecomp_e(k, mode, &result)\0" as *const u8
                as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            600 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_ellint_Pcomp(
    mut k: libc::c_double,
    mut n: libc::c_double,
    mut mode: gsl_mode_t,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_ellint_Pcomp_e(k, n, mode, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_ellint_Pcomp_e(k, n, mode, &result)\0" as *const u8
                as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            605 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_ellint_Dcomp(
    mut k: libc::c_double,
    mut mode: gsl_mode_t,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_ellint_Dcomp_e(k, mode, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_ellint_Dcomp_e(k, mode, &result)\0" as *const u8
                as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            610 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_ellint_F(
    mut phi: libc::c_double,
    mut k: libc::c_double,
    mut mode: gsl_mode_t,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_ellint_F_e(phi, k, mode, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_ellint_F_e(phi, k, mode, &result)\0" as *const u8
                as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            615 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_ellint_E(
    mut phi: libc::c_double,
    mut k: libc::c_double,
    mut mode: gsl_mode_t,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_ellint_E_e(phi, k, mode, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_ellint_E_e(phi, k, mode, &result)\0" as *const u8
                as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            620 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_ellint_P(
    mut phi: libc::c_double,
    mut k: libc::c_double,
    mut n: libc::c_double,
    mut mode: gsl_mode_t,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_ellint_P_e(phi, k, n, mode, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_ellint_P_e(phi, k, n, mode, &result)\0" as *const u8
                as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            625 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_ellint_D(
    mut phi: libc::c_double,
    mut k: libc::c_double,
    mut mode: gsl_mode_t,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_ellint_D_e(phi, k, mode, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_ellint_D_e(phi, k, mode, &result)\0" as *const u8
                as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            630 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_ellint_RC(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut mode: gsl_mode_t,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_ellint_RC_e(x, y, mode, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_ellint_RC_e(x, y, mode, &result)\0" as *const u8
                as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            635 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_ellint_RD(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut z: libc::c_double,
    mut mode: gsl_mode_t,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_ellint_RD_e(x, y, z, mode, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_ellint_RD_e(x, y, z, mode, &result)\0" as *const u8
                as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            640 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_ellint_RF(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut z: libc::c_double,
    mut mode: gsl_mode_t,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_ellint_RF_e(x, y, z, mode, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_ellint_RF_e(x, y, z, mode, &result)\0" as *const u8
                as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            645 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_ellint_RJ(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut z: libc::c_double,
    mut p: libc::c_double,
    mut mode: gsl_mode_t,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_ellint_RJ_e(x, y, z, p, mode, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_ellint_RJ_e(x, y, z, p, mode, &result)\0" as *const u8
                as *const libc::c_char,
            b"ellint.c\0" as *const u8 as *const libc::c_char,
            650 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
