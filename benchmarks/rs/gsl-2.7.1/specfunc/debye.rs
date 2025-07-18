use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
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
static mut adeb1_data: [libc::c_double; 17] = [
    2.4006597190381410194f64,
    0.1937213042189360089f64,
    -0.62329124554895770e-02f64,
    0.3511174770206480e-03f64,
    -0.228222466701231e-04f64,
    0.15805467875030e-05f64,
    -0.1135378197072e-06f64,
    0.83583361188e-08f64,
    -0.6264424787e-09f64,
    0.476033489e-10f64,
    -0.36574154e-11f64,
    0.2835431e-12f64,
    -0.221473e-13f64,
    0.17409e-14f64,
    -0.1376e-15f64,
    0.109e-16f64,
    -0.9e-18f64,
];
static mut adeb1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: adeb1_data.as_ptr() as *mut _,
            order: 16 as libc::c_int,
            a: -1.0f64,
            b: 1.0f64,
            order_sp: 9 as libc::c_int,
        };
        init
    }
};
static mut adeb2_data: [libc::c_double; 18] = [
    2.5943810232570770282f64,
    0.2863357204530719834f64,
    -0.102062656158046713e-01f64,
    0.6049109775346844e-03f64,
    -0.405257658950210e-04f64,
    0.28633826328811e-05f64,
    -0.2086394303065e-06f64,
    0.155237875826e-07f64,
    -0.11731280087e-08f64,
    0.897358589e-10f64,
    -0.69317614e-11f64,
    0.5398057e-12f64,
    -0.423241e-13f64,
    0.33378e-14f64,
    -0.2645e-15f64,
    0.211e-16f64,
    -0.17e-17f64,
    0.1e-18f64,
];
static mut adeb2_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: adeb2_data.as_ptr() as *mut _,
            order: 17 as libc::c_int,
            a: -1.0f64,
            b: 1.0f64,
            order_sp: 10 as libc::c_int,
        };
        init
    }
};
static mut adeb3_data: [libc::c_double; 17] = [
    2.707737068327440945f64,
    0.340068135211091751f64,
    -0.12945150184440869e-01f64,
    0.7963755380173816e-03f64,
    -0.546360009590824e-04f64,
    0.39243019598805e-05f64,
    -0.2894032823539e-06f64,
    0.217317613962e-07f64,
    -0.16542099950e-08f64,
    0.1272796189e-09f64,
    -0.987963460e-11f64,
    0.7725074e-12f64,
    -0.607797e-13f64,
    0.48076e-14f64,
    -0.3820e-15f64,
    0.305e-16f64,
    -0.24e-17f64,
];
static mut adeb3_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: adeb3_data.as_ptr() as *mut _,
            order: 16 as libc::c_int,
            a: -1.0f64,
            b: 1.0f64,
            order_sp: 10 as libc::c_int,
        };
        init
    }
};
static mut adeb4_data: [libc::c_double; 17] = [
    2.781869415020523460f64,
    0.374976783526892863f64,
    -0.14940907399031583e-01f64,
    0.945679811437042e-03f64,
    -0.66132916138933e-04f64,
    0.4815632982144e-05f64,
    -0.3588083958759e-06f64,
    0.271601187416e-07f64,
    -0.20807099122e-08f64,
    0.1609383869e-09f64,
    -0.125470979e-10f64,
    0.9847265e-12f64,
    -0.777237e-13f64,
    0.61648e-14f64,
    -0.4911e-15f64,
    0.393e-16f64,
    -0.32e-17f64,
];
static mut adeb4_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: adeb4_data.as_ptr() as *mut _,
            order: 16 as libc::c_int,
            a: -1.0f64,
            b: 1.0f64,
            order_sp: 10 as libc::c_int,
        };
        init
    }
};
static mut adeb5_data: [libc::c_double; 17] = [
    2.8340269546834530149f64,
    0.3994098857106266445f64,
    -0.164566764773099646e-1f64,
    0.10652138340664541e-2f64,
    -0.756730374875418e-4f64,
    0.55745985240273e-5f64,
    -0.4190692330918e-6f64,
    0.319456143678e-7f64,
    -0.24613318171e-8f64,
    0.1912801633e-9f64,
    -0.149720049e-10f64,
    0.11790312e-11f64,
    -0.933329e-13f64,
    0.74218e-14f64,
    -0.5925e-15f64,
    0.475e-16f64,
    -0.39e-17f64,
];
static mut adeb5_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: adeb5_data.as_ptr() as *mut _,
            order: 16 as libc::c_int,
            a: -1.0f64,
            b: 1.0f64,
            order_sp: 10 as libc::c_int,
        };
        init
    }
};
static mut adeb6_data: [libc::c_double; 17] = [
    2.8726727134130122113f64,
    0.4174375352339027746f64,
    -0.176453849354067873e-1f64,
    0.11629852733494556e-2f64,
    -0.837118027357117e-4f64,
    0.62283611596189e-5f64,
    -0.4718644465636e-6f64,
    0.361950397806e-7f64,
    -0.28030368010e-8f64,
    0.2187681983e-9f64,
    -0.171857387e-10f64,
    0.13575809e-11f64,
    -0.1077580e-12f64,
    0.85893e-14f64,
    -0.6872e-15f64,
    0.552e-16f64,
    -0.44e-17f64,
];
static mut adeb6_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: adeb6_data.as_ptr() as *mut _,
            order: 16 as libc::c_int,
            a: -1.0f64,
            b: 1.0f64,
            order_sp: 10 as libc::c_int,
        };
        init
    }
};
pub unsafe extern "C" fn gsl_sf_debye_1_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let val_infinity: libc::c_double = 1.64493406684822644f64;
    let xcut: libc::c_double = --7.0839641853226408e+02f64;
    if x < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"debye.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x < 2.0f64 * 1.4901161193847656e-08f64 {
        (*result).val = 1.0f64 - 0.25f64 * x + x * x / 36.0f64;
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x <= 4.0f64 {
        let t: libc::c_double = x * x / 8.0f64 - 1.0f64;
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut adeb1_cs, t, &mut c);
        (*result).val = c.val - 0.25f64 * x;
        (*result).err = c.err + 0.25f64 * x * 2.2204460492503131e-16f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x < -(0.69314718055994530942f64 + -3.6043653389117154e+01f64) {
        let nexp: libc::c_int = floor(xcut / x) as libc::c_int;
        let ex: libc::c_double = exp(-x);
        let mut sum: libc::c_double = 0.0f64;
        let mut xk: libc::c_double = nexp as libc::c_double * x;
        let mut rk: libc::c_double = nexp as libc::c_double;
        let mut i: libc::c_int = 0;
        i = nexp;
        while i >= 1 as libc::c_int {
            sum *= ex;
            sum += (1.0f64 + 1.0f64 / xk) / rk;
            rk -= 1.0f64;
            xk -= x;
            i -= 1;
            i;
        }
        (*result).val = val_infinity / x - sum * ex;
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < xcut {
        (*result).val = (val_infinity - exp(-x) * (x + 1.0f64)) / x;
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = val_infinity / x;
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_debye_2_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let val_infinity: libc::c_double = 4.80822761263837714f64;
    let xcut: libc::c_double = --7.0839641853226408e+02f64;
    if x < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"debye.c\0" as *const u8 as *const libc::c_char,
            255 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x < 2.0f64 * 1.41421356237309504880f64 * 1.4901161193847656e-08f64 {
        (*result).val = 1.0f64 - x / 3.0f64 + x * x / 24.0f64;
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else if x <= 4.0f64 {
        let t: libc::c_double = x * x / 8.0f64 - 1.0f64;
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut adeb2_cs, t, &mut c);
        (*result).val = c.val - x / 3.0f64;
        (*result).err = c.err + 2.2204460492503131e-16f64 * x / 3.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x < -(0.69314718055994530942f64 + -3.6043653389117154e+01f64) {
        let nexp: libc::c_int = floor(xcut / x) as libc::c_int;
        let ex: libc::c_double = exp(-x);
        let mut xk: libc::c_double = nexp as libc::c_double * x;
        let mut rk: libc::c_double = nexp as libc::c_double;
        let mut sum: libc::c_double = 0.0f64;
        let mut i: libc::c_int = 0;
        i = nexp;
        while i >= 1 as libc::c_int {
            sum *= ex;
            sum += (1.0f64 + 2.0f64 / xk + 2.0f64 / (xk * xk)) / rk;
            rk -= 1.0f64;
            xk -= x;
            i -= 1;
            i;
        }
        (*result).val = val_infinity / (x * x) - 2.0f64 * sum * ex;
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < xcut {
        let x2: libc::c_double = x * x;
        let sum_0: libc::c_double = 2.0f64 + 2.0f64 * x + x2;
        (*result).val = (val_infinity - 2.0f64 * sum_0 * exp(-x)) / x2;
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = val_infinity / x / x;
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        if fabs((*result).val) < 2.2250738585072014e-308f64 {
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"debye.c\0" as *const u8 as *const libc::c_char,
                297 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_debye_3_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let val_infinity: libc::c_double = 19.4818182068004875f64;
    let xcut: libc::c_double = --7.0839641853226408e+02f64;
    if x < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"debye.c\0" as *const u8 as *const libc::c_char,
            311 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x < 2.0f64 * 1.41421356237309504880f64 * 1.4901161193847656e-08f64 {
        (*result).val = 1.0f64 - 3.0f64 * x / 8.0f64 + x * x / 20.0f64;
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else if x <= 4.0f64 {
        let t: libc::c_double = x * x / 8.0f64 - 1.0f64;
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut adeb3_cs, t, &mut c);
        (*result).val = c.val - 0.375f64 * x;
        (*result).err = c.err + 2.2204460492503131e-16f64 * 0.375f64 * x;
        return GSL_SUCCESS as libc::c_int;
    } else if x < -(0.69314718055994530942f64 + -3.6043653389117154e+01f64) {
        let nexp: libc::c_int = floor(xcut / x) as libc::c_int;
        let ex: libc::c_double = exp(-x);
        let mut xk: libc::c_double = nexp as libc::c_double * x;
        let mut rk: libc::c_double = nexp as libc::c_double;
        let mut sum: libc::c_double = 0.0f64;
        let mut i: libc::c_int = 0;
        i = nexp;
        while i >= 1 as libc::c_int {
            let mut xk_inv: libc::c_double = 1.0f64 / xk;
            sum *= ex;
            sum
                += (((6.0f64 * xk_inv + 6.0f64) * xk_inv + 3.0f64) * xk_inv + 1.0f64)
                    / rk;
            rk -= 1.0f64;
            xk -= x;
            i -= 1;
            i;
        }
        (*result).val = val_infinity / (x * x * x) - 3.0f64 * sum * ex;
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else if x < xcut {
        let x3: libc::c_double = x * x * x;
        let sum_0: libc::c_double = 6.0f64 + 6.0f64 * x + 3.0f64 * x * x + x3;
        (*result).val = (val_infinity - 3.0f64 * sum_0 * exp(-x)) / x3;
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = val_infinity / x / x / x;
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        if fabs((*result).val) < 2.2250738585072014e-308f64 {
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"debye.c\0" as *const u8 as *const libc::c_char,
                354 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_debye_4_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let val_infinity: libc::c_double = 99.5450644937635129f64;
    let xcut: libc::c_double = --7.0839641853226408e+02f64;
    if x < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"debye.c\0" as *const u8 as *const libc::c_char,
            368 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x < 2.0f64 * 1.41421356237309504880f64 * 1.4901161193847656e-08f64 {
        (*result).val = 1.0f64 - 2.0f64 * x / 5.0f64 + x * x / 18.0f64;
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else if x <= 4.0f64 {
        let t: libc::c_double = x * x / 8.0f64 - 1.0f64;
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut adeb4_cs, t, &mut c);
        (*result).val = c.val - 2.0f64 * x / 5.0f64;
        (*result).err = c.err + 2.2204460492503131e-16f64 * 2.0f64 * x / 5.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x < -(0.69314718055994530942f64 + -3.6043653389117154e+01f64) {
        let nexp: libc::c_int = floor(xcut / x) as libc::c_int;
        let ex: libc::c_double = exp(-x);
        let mut xk: libc::c_double = nexp as libc::c_double * x;
        let mut rk: libc::c_double = nexp as libc::c_double;
        let mut sum: libc::c_double = 0.0f64;
        let mut i: libc::c_int = 0;
        i = nexp;
        while i >= 1 as libc::c_int {
            let mut xk_inv: libc::c_double = 1.0f64 / xk;
            sum *= ex;
            sum
                += ((((24.0f64 * xk_inv + 24.0f64) * xk_inv + 12.0f64) * xk_inv + 4.0f64)
                    * xk_inv + 1.0f64) / rk;
            rk -= 1.0f64;
            xk -= x;
            i -= 1;
            i;
        }
        (*result).val = val_infinity / (x * x * x * x) - 4.0f64 * sum * ex;
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else if x < xcut {
        let x2: libc::c_double = x * x;
        let x4: libc::c_double = x2 * x2;
        let sum_0: libc::c_double = 24.0f64 + 24.0f64 * x + 12.0f64 * x2
            + 4.0f64 * x2 * x + x4;
        (*result).val = (val_infinity - 4.0f64 * sum_0 * exp(-x)) / x4;
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = val_infinity / x / x / x / x;
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        if fabs((*result).val) < 2.2250738585072014e-308f64 {
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"debye.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_debye_5_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let val_infinity: libc::c_double = 610.405837190669483828710757875f64;
    let xcut: libc::c_double = --7.0839641853226408e+02f64;
    if x < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"debye.c\0" as *const u8 as *const libc::c_char,
            425 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x < 2.0f64 * 1.41421356237309504880f64 * 1.4901161193847656e-08f64 {
        (*result).val = 1.0f64 - 5.0f64 * x / 12.0f64 + 5.0f64 * x * x / 84.0f64;
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else if x <= 4.0f64 {
        let t: libc::c_double = x * x / 8.0f64 - 1.0f64;
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut adeb5_cs, t, &mut c);
        (*result).val = c.val - 5.0f64 * x / 12.0f64;
        (*result).err = c.err + 2.2204460492503131e-16f64 * 5.0f64 * x / 12.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x < -(0.69314718055994530942f64 + -3.6043653389117154e+01f64) {
        let nexp: libc::c_int = floor(xcut / x) as libc::c_int;
        let ex: libc::c_double = exp(-x);
        let mut xk: libc::c_double = nexp as libc::c_double * x;
        let mut rk: libc::c_double = nexp as libc::c_double;
        let mut sum: libc::c_double = 0.0f64;
        let mut i: libc::c_int = 0;
        i = nexp;
        while i >= 1 as libc::c_int {
            let mut xk_inv: libc::c_double = 1.0f64 / xk;
            sum *= ex;
            sum
                += (((((120.0f64 * xk_inv + 120.0f64) * xk_inv + 60.0f64) * xk_inv
                    + 20.0f64) * xk_inv + 5.0f64) * xk_inv + 1.0f64) / rk;
            rk -= 1.0f64;
            xk -= x;
            i -= 1;
            i;
        }
        (*result).val = val_infinity / (x * x * x * x * x) - 5.0f64 * sum * ex;
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else if x < xcut {
        let x2: libc::c_double = x * x;
        let x4: libc::c_double = x2 * x2;
        let x5: libc::c_double = x4 * x;
        let sum_0: libc::c_double = 120.0f64 + 120.0f64 * x + 60.0f64 * x2
            + 20.0f64 * x2 * x + 5.0f64 * x4 + x5;
        (*result).val = (val_infinity - 5.0f64 * sum_0 * exp(-x)) / x5;
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = val_infinity / x / x / x / x / x;
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        if fabs((*result).val) < 2.2250738585072014e-308f64 {
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"debye.c\0" as *const u8 as *const libc::c_char,
                470 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_debye_6_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let val_infinity: libc::c_double = 4356.06887828990661194792541535f64;
    let xcut: libc::c_double = --7.0839641853226408e+02f64;
    if x < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"debye.c\0" as *const u8 as *const libc::c_char,
            483 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x < 2.0f64 * 1.41421356237309504880f64 * 1.4901161193847656e-08f64 {
        (*result).val = 1.0f64 - 3.0f64 * x / 7.0f64 + x * x / 16.0f64;
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else if x <= 4.0f64 {
        let t: libc::c_double = x * x / 8.0f64 - 1.0f64;
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut adeb6_cs, t, &mut c);
        (*result).val = c.val - 3.0f64 * x / 7.0f64;
        (*result).err = c.err + 2.2204460492503131e-16f64 * 3.0f64 * x / 7.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x < -(0.69314718055994530942f64 + -3.6043653389117154e+01f64) {
        let nexp: libc::c_int = floor(xcut / x) as libc::c_int;
        let ex: libc::c_double = exp(-x);
        let mut xk: libc::c_double = nexp as libc::c_double * x;
        let mut rk: libc::c_double = nexp as libc::c_double;
        let mut sum: libc::c_double = 0.0f64;
        let mut i: libc::c_int = 0;
        i = nexp;
        while i >= 1 as libc::c_int {
            let mut xk_inv: libc::c_double = 1.0f64 / xk;
            sum *= ex;
            sum
                += ((((((720.0f64 * xk_inv + 720.0f64) * xk_inv + 360.0f64) * xk_inv
                    + 120.0f64) * xk_inv + 30.0f64) * xk_inv + 6.0f64) * xk_inv + 1.0f64)
                    / rk;
            rk -= 1.0f64;
            xk -= x;
            i -= 1;
            i;
        }
        (*result).val = val_infinity / (x * x * x * x * x * x) - 6.0f64 * sum * ex;
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else if x < xcut {
        let x2: libc::c_double = x * x;
        let x4: libc::c_double = x2 * x2;
        let x6: libc::c_double = x4 * x2;
        let sum_0: libc::c_double = 720.0f64 + 720.0f64 * x + 360.0f64 * x2
            + 120.0f64 * x2 * x + 30.0f64 * x4 + 6.0f64 * x4 * x + x6;
        (*result).val = (val_infinity - 6.0f64 * sum_0 * exp(-x)) / x6;
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = val_infinity / x / x / x / x / x / x;
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        if fabs((*result).val) < 2.2250738585072014e-308f64 {
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"debye.c\0" as *const u8 as *const libc::c_char,
                528 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_debye_1(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_debye_1_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_debye_1_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"debye.c\0" as *const u8 as *const libc::c_char,
            540 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_debye_2(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_debye_2_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_debye_2_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"debye.c\0" as *const u8 as *const libc::c_char,
            545 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_debye_3(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_debye_3_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_debye_3_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"debye.c\0" as *const u8 as *const libc::c_char,
            550 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_debye_4(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_debye_4_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_debye_4_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"debye.c\0" as *const u8 as *const libc::c_char,
            555 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_debye_5(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_debye_5_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_debye_5_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"debye.c\0" as *const u8 as *const libc::c_char,
            560 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_debye_6(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_debye_6_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_debye_6_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"debye.c\0" as *const u8 as *const libc::c_char,
            565 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
