use ::libc;
extern "C" {
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_sf_result_struct {
    pub val: libc::c_double,
    pub err: libc::c_double,
}
pub type gsl_sf_result = gsl_sf_result_struct;
pub type cheb_series = cheb_series_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cheb_series_struct {
    pub c: *mut libc::c_double,
    pub order: libc::c_int,
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub order_sp: libc::c_int,
}
#[inline]
unsafe extern "C" fn cheb_eval_e(
    mut cs: *const cheb_series,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut d: libc::c_double = 0.0f64;
    let mut dd: libc::c_double = 0.0f64;
    let mut y: libc::c_double = (2.0f64 * x - (*cs).a - (*cs).b) / ((*cs).b - (*cs).a);
    let mut y2: libc::c_double = 2.0f64 * y;
    let mut e: libc::c_double = 0.0f64;
    j = (*cs).order;
    while j >= 1 as libc::c_int {
        let mut temp: libc::c_double = d;
        d = y2 * d - dd + *((*cs).c).offset(j as isize);
        e += fabs(y2 * temp) + fabs(dd) + fabs(*((*cs).c).offset(j as isize));
        dd = temp;
        j -= 1;
        j;
    }
    let mut temp_0: libc::c_double = d;
    d = y * d - dd + 0.5f64 * *((*cs).c).offset(0 as libc::c_int as isize);
    e
        += fabs(y * temp_0) + fabs(dd)
            + 0.5f64 * fabs(*((*cs).c).offset(0 as libc::c_int as isize));
    (*result).val = d;
    (*result)
        .err = 2.2204460492503131e-16f64 * e
        + fabs(*((*cs).c).offset((*cs).order as isize));
    return GSL_SUCCESS as libc::c_int;
}
static mut lopx_data: [libc::c_double; 21] = [
    2.16647910664395270521272590407f64,
    -0.28565398551049742084877469679f64,
    0.01517767255690553732382488171f64,
    -0.00200215904941415466274422081f64,
    0.00019211375164056698287947962f64,
    -0.00002553258886105542567601400f64,
    2.9004512660400621301999384544e-06f64,
    -3.8873813517057343800270917900e-07f64,
    4.7743678729400456026672697926e-08f64,
    -6.4501969776090319441714445454e-09f64,
    8.2751976628812389601561347296e-10f64,
    -1.1260499376492049411710290413e-10f64,
    1.4844576692270934446023686322e-11f64,
    -2.0328515972462118942821556033e-12f64,
    2.7291231220549214896095654769e-13f64,
    -3.7581977830387938294437434651e-14f64,
    5.1107345870861673561462339876e-15f64,
    -7.0722150011433276578323272272e-16f64,
    9.7089758328248469219003866867e-17f64,
    -1.3492637457521938883731579510e-17f64,
    1.8657327910677296608121390705e-18f64,
];
static mut lopx_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: lopx_data.as_ptr() as *mut _,
            order: 20 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 10 as libc::c_int,
        };
        init
    }
};
static mut lopxmx_data: [libc::c_double; 20] = [
    -1.12100231323744103373737274541f64,
    0.19553462773379386241549597019f64,
    -0.01467470453808083971825344956f64,
    0.00166678250474365477643629067f64,
    -0.00018543356147700369785746902f64,
    0.00002280154021771635036301071f64,
    -2.8031253116633521699214134172e-06f64,
    3.5936568872522162983669541401e-07f64,
    -4.6241857041062060284381167925e-08f64,
    6.0822637459403991012451054971e-09f64,
    -8.0339824424815790302621320732e-10f64,
    1.0751718277499375044851551587e-10f64,
    -1.4445310914224613448759230882e-11f64,
    1.9573912180610336168921438426e-12f64,
    -2.6614436796793061741564104510e-13f64,
    3.6402634315269586532158344584e-14f64,
    -4.9937495922755006545809120531e-15f64,
    6.8802890218846809524646902703e-16f64,
    -9.5034129794804273611403251480e-17f64,
    1.3170135013050997157326965813e-17f64,
];
static mut lopxmx_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: lopxmx_data.as_ptr() as *mut _,
            order: 19 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 9 as libc::c_int,
        };
        init
    }
};
pub unsafe extern "C" fn gsl_sf_log_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"log.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        (*result).val = log(x);
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_log_abs_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x == 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"log.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        (*result).val = log(fabs(x));
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_complex_log_e(
    zr: libc::c_double,
    zi: libc::c_double,
    mut lnr: *mut gsl_sf_result,
    mut theta: *mut gsl_sf_result,
) -> libc::c_int {
    if zr != 0.0f64 || zi != 0.0f64 {
        let ax: libc::c_double = fabs(zr);
        let ay: libc::c_double = fabs(zi);
        let min: libc::c_double = if ax < ay { ax } else { ay };
        let max: libc::c_double = if ax > ay { ax } else { ay };
        (*lnr).val = log(max) + 0.5f64 * log(1.0f64 + min / max * (min / max));
        (*lnr).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*lnr).val);
        (*theta).val = atan2(zi, zr);
        (*theta).err = 2.2204460492503131e-16f64 * fabs((*lnr).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*lnr).val = ::std::f32::NAN as libc::c_double;
        (*lnr).err = ::std::f32::NAN as libc::c_double;
        (*theta).val = ::std::f32::NAN as libc::c_double;
        (*theta).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"log.c\0" as *const u8 as *const libc::c_char,
            159 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_log_1plusx_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= -1.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"log.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if fabs(x) < 2.4607833005759251e-03f64 {
        let c1: libc::c_double = -0.5f64;
        let c2: libc::c_double = 1.0f64 / 3.0f64;
        let c3: libc::c_double = -1.0f64 / 4.0f64;
        let c4: libc::c_double = 1.0f64 / 5.0f64;
        let c5: libc::c_double = -1.0f64 / 6.0f64;
        let c6: libc::c_double = 1.0f64 / 7.0f64;
        let c7: libc::c_double = -1.0f64 / 8.0f64;
        let c8: libc::c_double = 1.0f64 / 9.0f64;
        let c9: libc::c_double = -1.0f64 / 10.0f64;
        let t: libc::c_double = c5 + x * (c6 + x * (c7 + x * (c8 + x * c9)));
        (*result).val = x * (1.0f64 + x * (c1 + x * (c2 + x * (c3 + x * (c4 + x * t)))));
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if fabs(x) < 0.5f64 {
        let mut t_0: libc::c_double = 0.5f64 * (8.0f64 * x + 1.0f64) / (x + 2.0f64);
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut lopx_cs, t_0, &mut c);
        (*result).val = x * c.val;
        (*result).err = fabs(x * c.err);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = log(1.0f64 + x);
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_log_1plusx_mx_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= -1.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"log.c\0" as *const u8 as *const libc::c_char,
            209 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if fabs(x) < 7.4009597974140505e-04f64 {
        let c1: libc::c_double = -0.5f64;
        let c2: libc::c_double = 1.0f64 / 3.0f64;
        let c3: libc::c_double = -1.0f64 / 4.0f64;
        let c4: libc::c_double = 1.0f64 / 5.0f64;
        let c5: libc::c_double = -1.0f64 / 6.0f64;
        let c6: libc::c_double = 1.0f64 / 7.0f64;
        let c7: libc::c_double = -1.0f64 / 8.0f64;
        let c8: libc::c_double = 1.0f64 / 9.0f64;
        let c9: libc::c_double = -1.0f64 / 10.0f64;
        let t: libc::c_double = c5 + x * (c6 + x * (c7 + x * (c8 + x * c9)));
        (*result).val = x * x * (c1 + x * (c2 + x * (c3 + x * (c4 + x * t))));
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if fabs(x) < 0.5f64 {
        let mut t_0: libc::c_double = 0.5f64 * (8.0f64 * x + 1.0f64) / (x + 2.0f64);
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut lopxmx_cs, t_0, &mut c);
        (*result).val = x * x * c.val;
        (*result).err = x * x * c.err;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let lterm: libc::c_double = log(1.0f64 + x);
        (*result).val = lterm - x;
        (*result).err = 2.2204460492503131e-16f64 * (fabs(lterm) + fabs(x));
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_log(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_log_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_log_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"log.c\0" as *const u8 as *const libc::c_char,
            250 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_log_abs(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_log_abs_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_log_abs_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"log.c\0" as *const u8 as *const libc::c_char,
            255 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_log_1plusx(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_log_1plusx_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_log_1plusx_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"log.c\0" as *const u8 as *const libc::c_char,
            260 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_log_1plusx_mx(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_log_1plusx_mx_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_log_1plusx_mx_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"log.c\0" as *const u8 as *const libc::c_char,
            265 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
