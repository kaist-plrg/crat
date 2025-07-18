use ::libc;
extern "C" {
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn ldexp(_: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_complex_log_e(
        zr: libc::c_double,
        zi: libc::c_double,
        lnr: *mut gsl_sf_result,
        theta: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_log_1plusx_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
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
unsafe extern "C" fn GSL_MIN_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn GSL_MAX_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a > b { a } else { b };
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
#[inline]
unsafe extern "C" fn sinh_series(
    x: libc::c_double,
    mut result: *mut libc::c_double,
) -> libc::c_int {
    let y: libc::c_double = x * x;
    let c0: libc::c_double = 1.0f64 / 6.0f64;
    let c1: libc::c_double = 1.0f64 / 120.0f64;
    let c2: libc::c_double = 1.0f64 / 5040.0f64;
    let c3: libc::c_double = 1.0f64 / 362880.0f64;
    let c4: libc::c_double = 1.0f64 / 39916800.0f64;
    let c5: libc::c_double = 1.0f64 / 6227020800.0f64;
    let c6: libc::c_double = 1.0f64 / 1307674368000.0f64;
    let c7: libc::c_double = 1.0f64 / 355687428096000.0f64;
    *result = x
        * (1.0f64
            + y
                * (c0
                    + y
                        * (c1
                            + y
                                * (c2
                                    + y * (c3 + y * (c4 + y * (c5 + y * (c6 + y * c7))))))));
    return GSL_SUCCESS as libc::c_int;
}
#[inline]
unsafe extern "C" fn cosh_m1_series(
    x: libc::c_double,
    mut result: *mut libc::c_double,
) -> libc::c_int {
    let y: libc::c_double = x * x;
    let c0: libc::c_double = 0.5f64;
    let c1: libc::c_double = 1.0f64 / 24.0f64;
    let c2: libc::c_double = 1.0f64 / 720.0f64;
    let c3: libc::c_double = 1.0f64 / 40320.0f64;
    let c4: libc::c_double = 1.0f64 / 3628800.0f64;
    let c5: libc::c_double = 1.0f64 / 479001600.0f64;
    let c6: libc::c_double = 1.0f64 / 87178291200.0f64;
    let c7: libc::c_double = 1.0f64 / 20922789888000.0f64;
    let c8: libc::c_double = 1.0f64 / 6402373705728000.0f64;
    *result = y
        * (c0
            + y
                * (c1
                    + y
                        * (c2
                            + y
                                * (c3
                                    + y * (c4 + y * (c5 + y * (c6 + y * (c7 + y * c8))))))));
    return GSL_SUCCESS as libc::c_int;
}
static mut sinc_data: [libc::c_double; 17] = [
    1.133648177811747875422f64,
    -0.532677564732557348781f64,
    -0.068293048346633177859f64,
    0.033403684226353715020f64,
    0.001485679893925747818f64,
    -0.000734421305768455295f64,
    -0.000016837282388837229f64,
    0.000008359950146618018f64,
    0.000000117382095601192f64,
    -0.000000058413665922724f64,
    -0.000000000554763755743f64,
    0.000000000276434190426f64,
    0.000000000001895374892f64,
    -0.000000000000945237101f64,
    -0.000000000000004900690f64,
    0.000000000000002445383f64,
    0.000000000000000009925f64,
];
static mut sinc_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: sinc_data.as_ptr() as *mut _,
            order: 16 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 10 as libc::c_int,
        };
        init
    }
};
static mut sin_data: [libc::c_double; 12] = [
    -0.3295190160663511504173f64,
    0.0025374284671667991990f64,
    0.0006261928782647355874f64,
    -4.6495547521854042157541e-06f64,
    -5.6917531549379706526677e-07f64,
    3.7283335140973803627866e-09f64,
    3.0267376484747473727186e-10f64,
    -1.7400875016436622322022e-12f64,
    -1.0554678305790849834462e-13f64,
    5.3701981409132410797062e-16f64,
    2.5984137983099020336115e-17f64,
    -1.1821555255364833468288e-19f64,
];
static mut sin_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: sin_data.as_ptr() as *mut _,
            order: 11 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 11 as libc::c_int,
        };
        init
    }
};
static mut cos_data: [libc::c_double; 11] = [
    0.165391825637921473505668118136f64,
    -0.00084852883845000173671196530195f64,
    -0.000210086507222940730213625768083f64,
    1.16582269619760204299639757584e-6f64,
    1.43319375856259870334412701165e-7f64,
    -7.4770883429007141617951330184e-10f64,
    -6.0969994944584252706997438007e-11f64,
    2.90748249201909353949854872638e-13f64,
    1.77126739876261435667156490461e-14f64,
    -7.6896421502815579078577263149e-17f64,
    -3.7363121133079412079201377318e-18f64,
];
static mut cos_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: cos_data.as_ptr() as *mut _,
            order: 10 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 10 as libc::c_int,
        };
        init
    }
};
pub unsafe extern "C" fn gsl_sf_sin_e(
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let P1: libc::c_double = 7.85398125648498535156e-1f64;
    let P2: libc::c_double = 3.77489470793079817668e-8f64;
    let P3: libc::c_double = 2.69515142907905952645e-15f64;
    let sgn_x: libc::c_double = (if x >= 0.0f64 {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    }) as libc::c_double;
    let abs_x: libc::c_double = fabs(x);
    if abs_x < 1.2207031250000000e-04f64 {
        let x2: libc::c_double = x * x;
        (*result).val = x * (1.0f64 - x2 / 6.0f64);
        (*result).err = fabs(x * x2 * x2 / 100.0f64);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut sgn_result: libc::c_double = sgn_x;
        let mut y: libc::c_double = floor(abs_x / (0.25f64 * 3.14159265358979323846f64));
        let mut octant: libc::c_int = (y
            - ldexp(floor(ldexp(y, -(3 as libc::c_int))), 3 as libc::c_int))
            as libc::c_int;
        let mut stat_cs: libc::c_int = 0;
        let mut z: libc::c_double = 0.;
        if octant & 1 as libc::c_int != 0 {
            octant += 1 as libc::c_int;
            octant &= 0o7 as libc::c_int;
            y += 1.0f64;
        }
        if octant > 3 as libc::c_int {
            octant -= 4 as libc::c_int;
            sgn_result = -sgn_result;
        }
        z = abs_x - y * P1 - y * P2 - y * P3;
        if octant == 0 as libc::c_int {
            let mut sin_cs_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let t: libc::c_double = 8.0f64 * fabs(z) / 3.14159265358979323846f64
                - 1.0f64;
            stat_cs = cheb_eval_e(&mut sin_cs, t, &mut sin_cs_result);
            (*result).val = z * (1.0f64 + z * z * sin_cs_result.val);
        } else {
            let mut cos_cs_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let t_0: libc::c_double = 8.0f64 * fabs(z) / 3.14159265358979323846f64
                - 1.0f64;
            stat_cs = cheb_eval_e(&mut cos_cs, t_0, &mut cos_cs_result);
            (*result)
                .val = 1.0f64 - 0.5f64 * z * z * (1.0f64 - z * z * cos_cs_result.val);
        }
        (*result).val *= sgn_result;
        if abs_x > 1.0f64 / 2.2204460492503131e-16f64 {
            (*result).err = fabs((*result).val);
        } else if abs_x > 100.0f64 / 1.4901161193847656e-08f64 {
            (*result)
                .err = 2.0f64 * abs_x * 2.2204460492503131e-16f64 * fabs((*result).val);
        } else if abs_x > 0.1f64 / 1.4901161193847656e-08f64 {
            (*result).err = 2.0f64 * 1.4901161193847656e-08f64 * fabs((*result).val);
        } else {
            (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        }
        return stat_cs;
    };
}
pub unsafe extern "C" fn gsl_sf_cos_e(
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let P1: libc::c_double = 7.85398125648498535156e-1f64;
    let P2: libc::c_double = 3.77489470793079817668e-8f64;
    let P3: libc::c_double = 2.69515142907905952645e-15f64;
    let abs_x: libc::c_double = fabs(x);
    if abs_x < 1.2207031250000000e-04f64 {
        let x2: libc::c_double = x * x;
        (*result).val = 1.0f64 - 0.5f64 * x2;
        (*result).err = fabs(x2 * x2 / 12.0f64);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut sgn_result: libc::c_double = 1.0f64;
        let mut y: libc::c_double = floor(abs_x / (0.25f64 * 3.14159265358979323846f64));
        let mut octant: libc::c_int = (y
            - ldexp(floor(ldexp(y, -(3 as libc::c_int))), 3 as libc::c_int))
            as libc::c_int;
        let mut stat_cs: libc::c_int = 0;
        let mut z: libc::c_double = 0.;
        if octant & 1 as libc::c_int != 0 {
            octant += 1 as libc::c_int;
            octant &= 0o7 as libc::c_int;
            y += 1.0f64;
        }
        if octant > 3 as libc::c_int {
            octant -= 4 as libc::c_int;
            sgn_result = -sgn_result;
        }
        if octant > 1 as libc::c_int {
            sgn_result = -sgn_result;
        }
        z = abs_x - y * P1 - y * P2 - y * P3;
        if octant == 0 as libc::c_int {
            let mut cos_cs_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let t: libc::c_double = 8.0f64 * fabs(z) / 3.14159265358979323846f64
                - 1.0f64;
            stat_cs = cheb_eval_e(&mut cos_cs, t, &mut cos_cs_result);
            (*result)
                .val = 1.0f64 - 0.5f64 * z * z * (1.0f64 - z * z * cos_cs_result.val);
        } else {
            let mut sin_cs_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let t_0: libc::c_double = 8.0f64 * fabs(z) / 3.14159265358979323846f64
                - 1.0f64;
            stat_cs = cheb_eval_e(&mut sin_cs, t_0, &mut sin_cs_result);
            (*result).val = z * (1.0f64 + z * z * sin_cs_result.val);
        }
        (*result).val *= sgn_result;
        if abs_x > 1.0f64 / 2.2204460492503131e-16f64 {
            (*result).err = fabs((*result).val);
        } else if abs_x > 100.0f64 / 1.4901161193847656e-08f64 {
            (*result)
                .err = 2.0f64 * abs_x * 2.2204460492503131e-16f64 * fabs((*result).val);
        } else if abs_x > 0.1f64 / 1.4901161193847656e-08f64 {
            (*result).err = 2.0f64 * 1.4901161193847656e-08f64 * fabs((*result).val);
        } else {
            (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        }
        return stat_cs;
    };
}
pub unsafe extern "C" fn gsl_sf_hypot_e(
    x: libc::c_double,
    y: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x == 0.0f64 && y == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let a: libc::c_double = fabs(x);
        let b: libc::c_double = fabs(y);
        let min: libc::c_double = GSL_MIN_DBL(a, b);
        let max: libc::c_double = GSL_MAX_DBL(a, b);
        let rat: libc::c_double = min / max;
        let root_term: libc::c_double = sqrt(1.0f64 + rat * rat);
        if max < 1.7976931348623157e+308f64 / root_term {
            (*result).val = max * root_term;
            (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            return GSL_SUCCESS as libc::c_int;
        } else {
            (*result).val = ::std::f32::INFINITY as libc::c_double;
            (*result).err = ::std::f32::INFINITY as libc::c_double;
            gsl_error(
                b"overflow\0" as *const u8 as *const libc::c_char,
                b"trig.c\0" as *const u8 as *const libc::c_char,
                335 as libc::c_int,
                GSL_EOVRFLW as libc::c_int,
            );
            return GSL_EOVRFLW as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn gsl_sf_complex_sin_e(
    zr: libc::c_double,
    zi: libc::c_double,
    mut szr: *mut gsl_sf_result,
    mut szi: *mut gsl_sf_result,
) -> libc::c_int {
    if fabs(zi) < 1.0f64 {
        let mut ch_m1: libc::c_double = 0.;
        let mut sh: libc::c_double = 0.;
        sinh_series(zi, &mut sh);
        cosh_m1_series(zi, &mut ch_m1);
        (*szr).val = sin(zr) * (ch_m1 + 1.0f64);
        (*szi).val = cos(zr) * sh;
        (*szr).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*szr).val);
        (*szi).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*szi).val);
        return GSL_SUCCESS as libc::c_int;
    } else if fabs(zi) < 7.0978271289338397e+02f64 {
        let mut ex: libc::c_double = exp(zi);
        let mut ch: libc::c_double = 0.5f64 * (ex + 1.0f64 / ex);
        let mut sh_0: libc::c_double = 0.5f64 * (ex - 1.0f64 / ex);
        (*szr).val = sin(zr) * ch;
        (*szi).val = cos(zr) * sh_0;
        (*szr).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*szr).val);
        (*szi).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*szi).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*szr).val = ::std::f32::INFINITY as libc::c_double;
        (*szr).err = ::std::f32::INFINITY as libc::c_double;
        (*szi).val = ::std::f32::INFINITY as libc::c_double;
        (*szi).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"trig.c\0" as *const u8 as *const libc::c_char,
            369 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_complex_cos_e(
    zr: libc::c_double,
    zi: libc::c_double,
    mut czr: *mut gsl_sf_result,
    mut czi: *mut gsl_sf_result,
) -> libc::c_int {
    if fabs(zi) < 1.0f64 {
        let mut ch_m1: libc::c_double = 0.;
        let mut sh: libc::c_double = 0.;
        sinh_series(zi, &mut sh);
        cosh_m1_series(zi, &mut ch_m1);
        (*czr).val = cos(zr) * (ch_m1 + 1.0f64);
        (*czi).val = -sin(zr) * sh;
        (*czr).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*czr).val);
        (*czi).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*czi).val);
        return GSL_SUCCESS as libc::c_int;
    } else if fabs(zi) < 7.0978271289338397e+02f64 {
        let mut ex: libc::c_double = exp(zi);
        let mut ch: libc::c_double = 0.5f64 * (ex + 1.0f64 / ex);
        let mut sh_0: libc::c_double = 0.5f64 * (ex - 1.0f64 / ex);
        (*czr).val = cos(zr) * ch;
        (*czi).val = -sin(zr) * sh_0;
        (*czr).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*czr).val);
        (*czi).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*czi).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*czr).val = ::std::f32::INFINITY as libc::c_double;
        (*czr).err = ::std::f32::INFINITY as libc::c_double;
        (*czi).val = ::std::f32::INFINITY as libc::c_double;
        (*czi).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"trig.c\0" as *const u8 as *const libc::c_char,
            402 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_complex_logsin_e(
    zr: libc::c_double,
    zi: libc::c_double,
    mut lszr: *mut gsl_sf_result,
    mut lszi: *mut gsl_sf_result,
) -> libc::c_int {
    if zi > 60.0f64 {
        (*lszr).val = -0.69314718055994530942f64 + zi;
        (*lszi).val = 0.5f64 * 3.14159265358979323846f64 - zr;
        (*lszr).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*lszr).val);
        (*lszi).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*lszi).val);
    } else if zi < -60.0f64 {
        (*lszr).val = -0.69314718055994530942f64 - zi;
        (*lszi).val = -0.5f64 * 3.14159265358979323846f64 + zr;
        (*lszr).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*lszr).val);
        (*lszi).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*lszi).val);
    } else {
        let mut sin_r: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sin_i: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut status: libc::c_int = 0;
        gsl_sf_complex_sin_e(zr, zi, &mut sin_r, &mut sin_i);
        status = gsl_sf_complex_log_e(sin_r.val, sin_i.val, lszr, lszi);
        if status == GSL_EDOM as libc::c_int {
            (*lszr).val = ::std::f32::NAN as libc::c_double;
            (*lszr).err = ::std::f32::NAN as libc::c_double;
            (*lszi).val = ::std::f32::NAN as libc::c_double;
            (*lszi).err = ::std::f32::NAN as libc::c_double;
            gsl_error(
                b"domain error\0" as *const u8 as *const libc::c_char,
                b"trig.c\0" as *const u8 as *const libc::c_char,
                432 as libc::c_int,
                GSL_EDOM as libc::c_int,
            );
            return GSL_EDOM as libc::c_int;
        }
    }
    return gsl_sf_angle_restrict_symm_e(&mut (*lszi).val);
}
pub unsafe extern "C" fn gsl_sf_lnsinh_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"trig.c\0" as *const u8 as *const libc::c_char,
            445 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if fabs(x) < 1.0f64 {
        let mut eps: libc::c_double = 0.;
        sinh_series(x, &mut eps);
        (*result).val = log(eps);
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < -0.5f64 * -3.6043653389117154e+01f64 {
        (*result).val = x + log(0.5f64 * (1.0f64 - exp(-2.0f64 * x)));
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = -0.69314718055994530942f64 + x;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_lncosh_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if fabs(x) < 1.0f64 {
        let mut eps: libc::c_double = 0.;
        cosh_m1_series(x, &mut eps);
        return gsl_sf_log_1plusx_e(eps, result);
    } else if fabs(x) < -0.5f64 * -3.6043653389117154e+01f64 {
        (*result).val = fabs(x) + log(0.5f64 * (1.0f64 + exp(-2.0f64 * fabs(x))));
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = -0.69314718055994530942f64 + fabs(x);
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_polar_to_rect(
    r: libc::c_double,
    theta: libc::c_double,
    mut x: *mut gsl_sf_result,
    mut y: *mut gsl_sf_result,
) -> libc::c_int {
    let mut t: libc::c_double = theta;
    let mut status: libc::c_int = gsl_sf_angle_restrict_symm_e(&mut t);
    let mut c: libc::c_double = cos(t);
    let mut s: libc::c_double = sin(t);
    (*x).val = r * cos(t);
    (*y).val = r * sin(t);
    (*x).err = r * fabs(s * 2.2204460492503131e-16f64 * t);
    (*x).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*x).val);
    (*y).err = r * fabs(c * 2.2204460492503131e-16f64 * t);
    (*y).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*y).val);
    return status;
}
pub unsafe extern "C" fn gsl_sf_rect_to_polar(
    x: libc::c_double,
    y: libc::c_double,
    mut r: *mut gsl_sf_result,
    mut theta: *mut gsl_sf_result,
) -> libc::c_int {
    let mut stat_h: libc::c_int = gsl_sf_hypot_e(x, y, r);
    if (*r).val > 0.0f64 {
        (*theta).val = atan2(y, x);
        (*theta).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*theta).val);
        return stat_h;
    } else {
        (*theta).val = ::std::f32::NAN as libc::c_double;
        (*theta).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"trig.c\0" as *const u8 as *const libc::c_char,
            528 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_angle_restrict_symm_err_e(
    theta: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let P1: libc::c_double = 4 as libc::c_int as libc::c_double
        * 7.8539812564849853515625e-01f64;
    let P2: libc::c_double = 4 as libc::c_int as libc::c_double
        * 3.7748947079307981766760e-08f64;
    let P3: libc::c_double = 4 as libc::c_int as libc::c_double
        * 2.6951514290790594840552e-15f64;
    let TwoPi: libc::c_double = 2 as libc::c_int as libc::c_double * (P1 + P2 + P3);
    let y: libc::c_double = ((if theta >= 0.0f64 {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    }) * 2 as libc::c_int) as libc::c_double * floor(fabs(theta) / TwoPi);
    let mut r: libc::c_double = theta - y * P1 - y * P2 - y * P3;
    if r > 3.14159265358979323846f64 {
        r = r - 2 as libc::c_int as libc::c_double * P1
            - 2 as libc::c_int as libc::c_double * P2
            - 2 as libc::c_int as libc::c_double * P3;
    } else if r < -3.14159265358979323846f64 {
        r = r + 2 as libc::c_int as libc::c_double * P1
            + 2 as libc::c_int as libc::c_double * P2
            + 2 as libc::c_int as libc::c_double * P3;
    }
    (*result).val = r;
    if fabs(theta) > 0.0625f64 / 2.2204460492503131e-16f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"trig.c\0" as *const u8 as *const libc::c_char,
            552 as libc::c_int,
            GSL_ELOSS as libc::c_int,
        );
        return GSL_ELOSS as libc::c_int;
    } else if fabs(theta) > 0.0625f64 / 1.4901161193847656e-08f64 {
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val - theta);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut delta: libc::c_double = fabs((*result).val - theta);
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64
            * (if delta < 3.14159265358979323846f64 {
                delta
            } else {
                3.14159265358979323846f64
            });
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_angle_restrict_pos_err_e(
    theta: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let P1: libc::c_double = 4 as libc::c_int as libc::c_double
        * 7.85398125648498535156e-01f64;
    let P2: libc::c_double = 4 as libc::c_int as libc::c_double
        * 3.77489470793079817668e-08f64;
    let P3: libc::c_double = 4 as libc::c_int as libc::c_double
        * 2.69515142907905952645e-15f64;
    let TwoPi: libc::c_double = 2 as libc::c_int as libc::c_double * (P1 + P2 + P3);
    let y: libc::c_double = 2 as libc::c_int as libc::c_double * floor(theta / TwoPi);
    let mut r: libc::c_double = theta - y * P1 - y * P2 - y * P3;
    if r > TwoPi {
        r = r - 2 as libc::c_int as libc::c_double * P1
            - 2 as libc::c_int as libc::c_double * P2
            - 2 as libc::c_int as libc::c_double * P3;
    } else if r < 0 as libc::c_int as libc::c_double {
        r = r + 2 as libc::c_int as libc::c_double * P1
            + 2 as libc::c_int as libc::c_double * P2
            + 2 as libc::c_int as libc::c_double * P3;
    }
    (*result).val = r;
    if fabs(theta) > 0.0625f64 / 2.2204460492503131e-16f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = fabs((*result).val);
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"trig.c\0" as *const u8 as *const libc::c_char,
            588 as libc::c_int,
            GSL_ELOSS as libc::c_int,
        );
        return GSL_ELOSS as libc::c_int;
    } else if fabs(theta) > 0.0625f64 / 1.4901161193847656e-08f64 {
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val - theta);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut delta: libc::c_double = fabs((*result).val - theta);
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64
            * (if delta < 3.14159265358979323846f64 {
                delta
            } else {
                3.14159265358979323846f64
            });
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_angle_restrict_symm_e(
    mut theta: *mut libc::c_double,
) -> libc::c_int {
    let mut r: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat: libc::c_int = gsl_sf_angle_restrict_symm_err_e(*theta, &mut r);
    *theta = r.val;
    return stat;
}
pub unsafe extern "C" fn gsl_sf_angle_restrict_pos_e(
    mut theta: *mut libc::c_double,
) -> libc::c_int {
    let mut r: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat: libc::c_int = gsl_sf_angle_restrict_pos_err_e(*theta, &mut r);
    *theta = r.val;
    return stat;
}
pub unsafe extern "C" fn gsl_sf_sin_err_e(
    x: libc::c_double,
    dx: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut stat_s: libc::c_int = gsl_sf_sin_e(x, result);
    (*result).err += fabs(cos(x) * dx);
    (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
    return stat_s;
}
pub unsafe extern "C" fn gsl_sf_cos_err_e(
    x: libc::c_double,
    dx: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut stat_c: libc::c_int = gsl_sf_cos_e(x, result);
    (*result).err += fabs(sin(x) * dx);
    (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
    return stat_c;
}
pub unsafe extern "C" fn gsl_sf_sinc_e(
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let ax: libc::c_double = fabs(x);
    if ax < 0.8f64 {
        return cheb_eval_e(&mut sinc_cs, 2.0f64 * ax - 1.0f64, result)
    } else if ax < 100.0f64 {
        (*result)
            .val = sin(3.14159265358979323846f64 * ax)
            / (3.14159265358979323846f64 * ax);
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let r: libc::c_double = 3.14159265358979323846f64 * ax;
        let mut s: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_s: libc::c_int = gsl_sf_sin_e(r, &mut s);
        (*result).val = s.val / r;
        (*result)
            .err = s.err / r + 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat_s;
    };
}
pub unsafe extern "C" fn gsl_sf_sin(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_sin_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_sin_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"trig.c\0" as *const u8 as *const libc::c_char,
            726 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_cos(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_cos_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_cos_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"trig.c\0" as *const u8 as *const libc::c_char,
            731 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_hypot(
    x: libc::c_double,
    y: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hypot_e(x, y, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hypot_e(x, y, &result)\0" as *const u8 as *const libc::c_char,
            b"trig.c\0" as *const u8 as *const libc::c_char,
            736 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_lnsinh(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_lnsinh_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_lnsinh_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"trig.c\0" as *const u8 as *const libc::c_char,
            741 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_lncosh(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_lncosh_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_lncosh_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"trig.c\0" as *const u8 as *const libc::c_char,
            746 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_angle_restrict_symm(
    theta: libc::c_double,
) -> libc::c_double {
    let mut result: libc::c_double = theta;
    let mut status: libc::c_int = gsl_sf_angle_restrict_symm_e(&mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_angle_restrict_symm_e(&result)\0" as *const u8
                as *const libc::c_char,
            b"trig.c\0" as *const u8 as *const libc::c_char,
            752 as libc::c_int,
            status,
        );
        return result;
    }
    return result;
}
pub unsafe extern "C" fn gsl_sf_angle_restrict_pos(
    theta: libc::c_double,
) -> libc::c_double {
    let mut result: libc::c_double = theta;
    let mut status: libc::c_int = gsl_sf_angle_restrict_pos_e(&mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_angle_restrict_pos_e(&result)\0" as *const u8
                as *const libc::c_char,
            b"trig.c\0" as *const u8 as *const libc::c_char,
            758 as libc::c_int,
            status,
        );
        return result;
    }
    return result;
}
pub unsafe extern "C" fn gsl_sf_sinc(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_sinc_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_sinc_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"trig.c\0" as *const u8 as *const libc::c_char,
            770 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
