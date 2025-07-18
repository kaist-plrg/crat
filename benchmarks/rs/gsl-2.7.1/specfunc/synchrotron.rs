use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_pow_int(x: libc::c_double, n: libc::c_int) -> libc::c_double;
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
static mut synchrotron1_data: [libc::c_double; 13] = [
    30.364682982501076273f64,
    17.079395277408394574f64,
    4.560132133545072889f64,
    0.549281246730419979f64,
    0.372976075069301172e-01f64,
    0.161362430201041242e-02f64,
    0.481916772120371e-04f64,
    0.10512425288938e-05f64,
    0.174638504670e-07f64,
    0.22815486544e-09f64,
    0.240443082e-11f64,
    0.2086588e-13f64,
    0.15167e-15f64,
];
static mut synchrotron1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: synchrotron1_data.as_ptr() as *mut _,
            order: 12 as libc::c_int,
            a: -1.0f64,
            b: 1.0f64,
            order_sp: 9 as libc::c_int,
        };
        init
    }
};
static mut synchrotron2_data: [libc::c_double; 12] = [
    0.4490721623532660844f64,
    0.898353677994187218e-01f64,
    0.81044573772151290e-02f64,
    0.4261716991089162e-03f64,
    0.147609631270746e-04f64,
    0.3628633615300e-06f64,
    0.66634807498e-08f64,
    0.949077166e-10f64,
    0.1079125e-11f64,
    0.10022e-13f64,
    0.77e-16f64,
    0.5e-18f64,
];
static mut synchrotron2_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: synchrotron2_data.as_ptr() as *mut _,
            order: 11 as libc::c_int,
            a: -1.0f64,
            b: 1.0f64,
            order_sp: 7 as libc::c_int,
        };
        init
    }
};
static mut synchrotron1a_data: [libc::c_double; 23] = [
    2.1329305161355000985f64,
    0.741352864954200240e-01f64,
    0.86968099909964198e-02f64,
    0.11703826248775692e-02f64,
    0.1645105798619192e-03f64,
    0.240201021420640e-04f64,
    0.35827756389389e-05f64,
    0.5447747626984e-06f64,
    0.838802856196e-07f64,
    0.13069882684e-07f64,
    0.2053099071e-08f64,
    0.325187537e-09f64,
    0.517914041e-10f64,
    0.83002988e-11f64,
    0.13352728e-11f64,
    0.2159150e-12f64,
    0.349967e-13f64,
    0.56994e-14f64,
    0.9291e-15f64,
    0.152e-15f64,
    0.249e-16f64,
    0.41e-17f64,
    0.7e-18f64,
];
static mut synchrotron1a_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: synchrotron1a_data.as_ptr() as *mut _,
            order: 22 as libc::c_int,
            a: -1.0f64,
            b: 1.0f64,
            order_sp: 11 as libc::c_int,
        };
        init
    }
};
static mut synchrotron21_data: [libc::c_double; 13] = [
    38.617839923843085480f64,
    23.037715594963734597f64,
    5.3802499868335705968f64,
    0.6156793806995710776f64,
    0.406688004668895584e-01f64,
    0.17296274552648414e-02f64,
    0.51061258836577e-04f64,
    0.110459595022e-05f64,
    0.18235530206e-07f64,
    0.2370769803e-09f64,
    0.24887296e-11f64,
    0.21529e-13f64,
    0.156e-15f64,
];
static mut synchrotron21_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: synchrotron21_data.as_ptr() as *mut _,
            order: 12 as libc::c_int,
            a: -1.0f64,
            b: 1.0f64,
            order_sp: 9 as libc::c_int,
        };
        init
    }
};
static mut synchrotron22_data: [libc::c_double; 13] = [
    7.9063148270660804288f64,
    3.1353463612853425684f64,
    0.4854879477453714538f64,
    0.394816675827237234e-01f64,
    0.19661622334808802e-02f64,
    0.659078932293042e-04f64,
    0.15857561349856e-05f64,
    0.286865301123e-07f64,
    0.4041202360e-09f64,
    0.45568444e-11f64,
    0.420459e-13f64,
    0.3232e-15f64,
    0.21e-17f64,
];
static mut synchrotron22_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: synchrotron22_data.as_ptr() as *mut _,
            order: 12 as libc::c_int,
            a: -1.0f64,
            b: 1.0f64,
            order_sp: 8 as libc::c_int,
        };
        init
    }
};
static mut synchrotron2a_data: [libc::c_double; 17] = [
    2.020337094170713600f64,
    0.10956237121807404e-01f64,
    0.8542384730114676e-03f64,
    0.723430242132822e-04f64,
    0.63124427962699e-05f64,
    0.5648193141174e-06f64,
    0.512832480138e-07f64,
    0.47196532914e-08f64,
    0.4380744214e-09f64,
    0.410268149e-10f64,
    0.38623072e-11f64,
    0.3661323e-12f64,
    0.348023e-13f64,
    0.33301e-14f64,
    0.319e-15f64,
    0.307e-16f64,
    0.3e-17f64,
];
static mut synchrotron2a_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: synchrotron2a_data.as_ptr() as *mut _,
            order: 16 as libc::c_int,
            a: -1.0f64,
            b: 1.0f64,
            order_sp: 8 as libc::c_int,
        };
        init
    }
};
pub unsafe extern "C" fn gsl_sf_synchrotron_1_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"synchrotron.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x < 2.0f64 * 1.41421356237309504880f64 * 1.4901161193847656e-08f64 {
        let mut z: libc::c_double = pow(x, 1.0f64 / 3.0f64);
        let mut cf: libc::c_double = 1 as libc::c_int as libc::c_double
            - 8.43812762813205e-01f64 * z * z;
        (*result).val = 2.14952824153447863671f64 * z * cf;
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else if x <= 4.0f64 {
        let c0: libc::c_double = 3.14159265358979323846f64
            / 1.73205080756887729352744634151f64;
        let px: libc::c_double = pow(x, 1.0f64 / 3.0f64);
        let px11: libc::c_double = gsl_sf_pow_int(px, 11 as libc::c_int);
        let t: libc::c_double = x * x / 8.0f64 - 1.0f64;
        let mut result_c1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut result_c2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut synchrotron1_cs, t, &mut result_c1);
        cheb_eval_e(&mut synchrotron2_cs, t, &mut result_c2);
        (*result).val = px * result_c1.val - px11 * result_c2.val - c0 * x;
        (*result)
            .err = px * result_c1.err + px11 * result_c2.err
            + c0 * x * 2.2204460492503131e-16f64;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < -8.0f64 * -7.0839641853226408e+02f64 / 7.0f64 {
        let c0_0: libc::c_double = 0.2257913526447274323630976f64;
        let t_0: libc::c_double = (12.0f64 - x) / (x + 4.0f64);
        let mut result_c1_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut synchrotron1a_cs, t_0, &mut result_c1_0);
        (*result).val = sqrt(x) * result_c1_0.val * exp(c0_0 - x);
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64 * (*result).val
            * (fabs(c0_0 - x) + 1.0f64);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"synchrotron.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_synchrotron_2_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"synchrotron.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x < 2.0f64 * 1.41421356237309504880f64 * 1.4901161193847656e-08f64 {
        let mut z: libc::c_double = pow(x, 1.0f64 / 3.0f64);
        let mut cf: libc::c_double = 1 as libc::c_int as libc::c_double
            - 1.17767156510235e+00f64 * z * x;
        (*result).val = 1.07476412076723931836f64 * z * cf;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else if x <= 4.0f64 {
        let px: libc::c_double = pow(x, 1.0f64 / 3.0f64);
        let px5: libc::c_double = gsl_sf_pow_int(px, 5 as libc::c_int);
        let t: libc::c_double = x * x / 8.0f64 - 1.0f64;
        let mut cheb1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut cheb2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut synchrotron21_cs, t, &mut cheb1);
        cheb_eval_e(&mut synchrotron22_cs, t, &mut cheb2);
        (*result).val = px * cheb1.val - px5 * cheb2.val;
        (*result).err = px * cheb1.err + px5 * cheb2.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < -8.0f64 * -7.0839641853226408e+02f64 / 7.0f64 {
        let c0: libc::c_double = 0.22579135264472743236f64;
        let t_0: libc::c_double = (10.0f64 - x) / (x + 2.0f64);
        let mut cheb1_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut synchrotron2a_cs, t_0, &mut cheb1_0);
        (*result).val = sqrt(x) * exp(c0 - x) * cheb1_0.val;
        (*result)
            .err = 2.2204460492503131e-16f64 * (*result).val * (fabs(c0 - x) + 1.0f64);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"synchrotron.c\0" as *const u8 as *const libc::c_char,
            269 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_synchrotron_1(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_synchrotron_1_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_synchrotron_1_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"synchrotron.c\0" as *const u8 as *const libc::c_char,
            279 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_synchrotron_2(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_synchrotron_2_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_synchrotron_2_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"synchrotron.c\0" as *const u8 as *const libc::c_char,
            284 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
