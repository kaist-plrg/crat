use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
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
static mut transport2_data: [libc::c_double; 18] = [
    1.671760446434538503f64,
    -0.147735359946794490f64,
    0.148213819946936338e-01f64,
    -0.14195330326305613e-02f64,
    0.1306541324415708e-03f64,
    -0.117155795867579e-04f64,
    0.10333498445756e-05f64,
    -0.901911304223e-07f64,
    0.78177169833e-08f64,
    -0.6744565684e-09f64,
    0.579946394e-10f64,
    -0.49747619e-11f64,
    0.425961e-12f64,
    -0.36422e-13f64,
    0.3111e-14f64,
    -0.265e-15f64,
    0.23e-16f64,
    -0.19e-17f64,
];
static mut transport2_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: transport2_data.as_ptr() as *mut _,
            order: 17 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 9 as libc::c_int,
        };
        init
    }
};
static mut transport3_data: [libc::c_double; 18] = [
    0.762012543243872007f64,
    -0.105674387705058533f64,
    0.119778084819657810e-01f64,
    -0.12144015203698307e-02f64,
    0.1155099769392855e-03f64,
    -0.105815992124423e-04f64,
    0.9474663385302e-06f64,
    -0.836221212858e-07f64,
    0.73109099278e-08f64,
    -0.6350594779e-09f64,
    0.549118282e-10f64,
    -0.47321395e-11f64,
    0.4067695e-12f64,
    -0.348971e-13f64,
    0.29892e-14f64,
    -0.256e-15f64,
    0.219e-16f64,
    -0.19e-17f64,
];
static mut transport3_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: transport3_data.as_ptr() as *mut _,
            order: 17 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 9 as libc::c_int,
        };
        init
    }
};
static mut transport4_data: [libc::c_double; 18] = [
    0.4807570994615110579f64,
    -0.8175378810321083956e-01f64,
    0.1002700665975162973e-01f64,
    -0.10599339359820151e-02f64,
    0.1034506245030405e-03f64,
    -0.96442705485899e-05f64,
    0.8745544408515e-06f64,
    -0.779321207981e-07f64,
    0.68649886141e-08f64,
    -0.5999571076e-09f64,
    0.521366241e-10f64,
    -0.45118382e-11f64,
    0.3892159e-12f64,
    -0.334936e-13f64,
    0.28767e-14f64,
    -0.2467e-15f64,
    0.211e-16f64,
    -0.18e-17f64,
];
static mut transport4_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: transport4_data.as_ptr() as *mut _,
            order: 17 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 9 as libc::c_int,
        };
        init
    }
};
static mut transport5_data: [libc::c_double; 18] = [
    0.347777777133910789f64,
    -0.66456988976050428e-01f64,
    0.8611072656883309e-02f64,
    -0.9396682223755538e-03f64,
    0.936324806081513e-04f64,
    -0.88571319340833e-05f64,
    0.811914989145e-06f64,
    -0.72957654233e-07f64,
    0.646971455e-08f64,
    -0.568490283e-09f64,
    0.49625598e-10f64,
    -0.4310940e-11f64,
    0.373100e-12f64,
    -0.32198e-13f64,
    0.2772e-14f64,
    -0.238e-15f64,
    0.21e-16f64,
    -0.18e-17f64,
];
static mut transport5_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: transport5_data.as_ptr() as *mut _,
            order: 17 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 9 as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn transport_sumexp(
    numexp: libc::c_int,
    order: libc::c_int,
    t: libc::c_double,
    mut x: libc::c_double,
) -> libc::c_double {
    let mut rk: libc::c_double = numexp as libc::c_double;
    let mut sumexp: libc::c_double = 0.0f64;
    let mut k: libc::c_int = 0;
    k = 1 as libc::c_int;
    while k <= numexp {
        let mut sum2: libc::c_double = 1.0f64;
        let mut xk: libc::c_double = 1.0f64 / (rk * x);
        let mut xk1: libc::c_double = 1.0f64;
        let mut j: libc::c_int = 0;
        j = 1 as libc::c_int;
        while j <= order {
            sum2 = sum2 * xk1 * xk + 1.0f64;
            xk1 += 1.0f64;
            j += 1;
            j;
        }
        sumexp *= t;
        sumexp += sum2;
        rk -= 1.0f64;
        k += 1;
        k;
    }
    return sumexp;
}
pub unsafe extern "C" fn gsl_sf_transport_2_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let val_infinity: libc::c_double = 3.289868133696452873f64;
    if x < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"transport.c\0" as *const u8 as *const libc::c_char,
            178 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x < 3.0f64 * 1.4901161193847656e-08f64 {
        (*result).val = x;
        (*result).err = 2.2204460492503131e-16f64 * fabs(x) + x * x / 2.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x <= 4.0f64 {
        let mut t: libc::c_double = x * x / 8.0f64 - 0.5f64 - 0.5f64;
        let mut result_c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut transport2_cs, t, &mut result_c);
        (*result).val = x * result_c.val;
        (*result).err = x * result_c.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < --3.6043653389117154e+01f64 {
        let numexp: libc::c_int = (--3.6043653389117154e+01f64 / x) as libc::c_int
            + 1 as libc::c_int;
        let sumexp: libc::c_double = transport_sumexp(
            numexp,
            2 as libc::c_int,
            exp(-x),
            x,
        );
        let t_0: libc::c_double = 2.0f64 * log(x) - x + log(sumexp);
        if t_0 < -3.6043653389117154e+01f64 {
            (*result).val = val_infinity;
            (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * val_infinity;
        } else {
            let et: libc::c_double = exp(t_0);
            (*result).val = val_infinity - et;
            (*result)
                .err = 2.0f64 * 2.2204460492503131e-16f64
                * (val_infinity + fabs(t_0) * et);
        }
        return GSL_SUCCESS as libc::c_int;
    } else if x < 2.0f64 / 2.2204460492503131e-16f64 {
        let numexp_0: libc::c_int = 1 as libc::c_int;
        let sumexp_0: libc::c_double = transport_sumexp(
            numexp_0,
            2 as libc::c_int,
            1.0f64,
            x,
        );
        let t_1: libc::c_double = 2.0f64 * log(x) - x + log(sumexp_0);
        if t_1 < -3.6043653389117154e+01f64 {
            (*result).val = val_infinity;
            (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * val_infinity;
        } else {
            let et_0: libc::c_double = exp(t_1);
            (*result).val = val_infinity - et_0;
            (*result)
                .err = 2.0f64 * 2.2204460492503131e-16f64
                * (val_infinity + (fabs(t_1) + 1.0f64) * et_0);
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let t_2: libc::c_double = 2.0f64 * log(x) - x;
        if t_2 < -3.6043653389117154e+01f64 {
            (*result).val = val_infinity;
            (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * val_infinity;
        } else {
            let et_1: libc::c_double = exp(t_2);
            (*result).val = val_infinity - et_1;
            (*result)
                .err = 2.0f64 * 2.2204460492503131e-16f64
                * (val_infinity + (fabs(t_2) + 1.0f64) * et_1);
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_transport_3_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let val_infinity: libc::c_double = 7.212341418957565712f64;
    if x < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"transport.c\0" as *const u8 as *const libc::c_char,
            248 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x < 3.0f64 * 1.4901161193847656e-08f64 {
        (*result).val = 0.5f64 * x * x;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * (*result).val;
        if fabs((*result).val) < 2.2250738585072014e-308f64 {
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"transport.c\0" as *const u8 as *const libc::c_char,
                258 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    } else if x <= 4.0f64 {
        let x2: libc::c_double = x * x;
        let t: libc::c_double = x2 / 8.0f64 - 0.5f64 - 0.5f64;
        let mut result_c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut transport3_cs, t, &mut result_c);
        (*result).val = x2 * result_c.val;
        (*result).err = x2 * result_c.err;
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < --3.6043653389117154e+01f64 {
        let numexp: libc::c_int = (--3.6043653389117154e+01f64 / x) as libc::c_int
            + 1 as libc::c_int;
        let sumexp: libc::c_double = transport_sumexp(
            numexp,
            3 as libc::c_int,
            exp(-x),
            x,
        );
        let t_0: libc::c_double = 3.0f64 * log(x) - x + log(sumexp);
        if t_0 < -3.6043653389117154e+01f64 {
            (*result).val = val_infinity;
            (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * val_infinity;
        } else {
            let et: libc::c_double = exp(t_0);
            (*result).val = val_infinity - et;
            (*result)
                .err = 2.0f64 * 2.2204460492503131e-16f64
                * (val_infinity + fabs(t_0) * et);
        }
        return GSL_SUCCESS as libc::c_int;
    } else if x < 3.0f64 / 2.2204460492503131e-16f64 {
        let numexp_0: libc::c_int = 1 as libc::c_int;
        let sumexp_0: libc::c_double = transport_sumexp(
            numexp_0,
            3 as libc::c_int,
            1.0f64,
            x,
        );
        let t_1: libc::c_double = 3.0f64 * log(x) - x + log(sumexp_0);
        if t_1 < -3.6043653389117154e+01f64 {
            (*result).val = val_infinity;
            (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * val_infinity;
        } else {
            let et_0: libc::c_double = exp(t_1);
            (*result).val = val_infinity - et_0;
            (*result)
                .err = 2.0f64 * 2.2204460492503131e-16f64
                * (val_infinity + (fabs(t_1) + 1.0f64) * et_0);
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let t_2: libc::c_double = 3.0f64 * log(x) - x;
        if t_2 < -3.6043653389117154e+01f64 {
            (*result).val = val_infinity;
            (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * val_infinity;
        } else {
            let et_1: libc::c_double = exp(t_2);
            (*result).val = val_infinity - et_1;
            (*result)
                .err = 2.0f64 * 2.2204460492503131e-16f64
                * (val_infinity + (fabs(t_2) + 1.0f64) * et_1);
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_transport_4_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let val_infinity: libc::c_double = 25.97575760906731660f64;
    if x < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"transport.c\0" as *const u8 as *const libc::c_char,
            325 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x < 3.0f64 * 1.4901161193847656e-08f64 {
        (*result).val = x * x * x / 3.0f64;
        (*result).err = 3.0f64 * 2.2204460492503131e-16f64 * (*result).val;
        if fabs((*result).val) < 2.2250738585072014e-308f64 {
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"transport.c\0" as *const u8 as *const libc::c_char,
                335 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    } else if x <= 4.0f64 {
        let x2: libc::c_double = x * x;
        let t: libc::c_double = x2 / 8.0f64 - 0.5f64 - 0.5f64;
        let mut result_c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut transport4_cs, t, &mut result_c);
        (*result).val = x2 * x * result_c.val;
        (*result).err = x2 * x * result_c.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < --3.6043653389117154e+01f64 {
        let numexp: libc::c_int = (--3.6043653389117154e+01f64 / x) as libc::c_int
            + 1 as libc::c_int;
        let sumexp: libc::c_double = transport_sumexp(
            numexp,
            4 as libc::c_int,
            exp(-x),
            x,
        );
        let t_0: libc::c_double = 4.0f64 * log(x) - x + log(sumexp);
        if t_0 < -3.6043653389117154e+01f64 {
            (*result).val = val_infinity;
            (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * val_infinity;
        } else {
            let et: libc::c_double = exp(t_0);
            (*result).val = val_infinity - et;
            (*result)
                .err = 2.0f64 * 2.2204460492503131e-16f64
                * (val_infinity + (fabs(t_0) + 1.0f64) * et);
        }
        return GSL_SUCCESS as libc::c_int;
    } else if x < 3.0f64 / 2.2204460492503131e-16f64 {
        let numexp_0: libc::c_int = 1 as libc::c_int;
        let sumexp_0: libc::c_double = transport_sumexp(
            numexp_0,
            4 as libc::c_int,
            1.0f64,
            x,
        );
        let t_1: libc::c_double = 4.0f64 * log(x) - x + log(sumexp_0);
        if t_1 < -3.6043653389117154e+01f64 {
            (*result).val = val_infinity;
            (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * val_infinity;
        } else {
            let et_0: libc::c_double = exp(t_1);
            (*result).val = val_infinity - et_0;
            (*result)
                .err = 2.0f64 * 2.2204460492503131e-16f64
                * (val_infinity + (fabs(t_1) + 1.0f64) * et_0);
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let t_2: libc::c_double = 4.0f64 * log(x) - x;
        if t_2 < -3.6043653389117154e+01f64 {
            (*result).val = val_infinity;
            (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * val_infinity;
        } else {
            let et_1: libc::c_double = exp(t_2);
            (*result).val = val_infinity - et_1;
            (*result)
                .err = 2.0f64 * 2.2204460492503131e-16f64
                * (val_infinity + (fabs(t_2) + 1.0f64) * et_1);
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_transport_5_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let val_infinity: libc::c_double = 124.4313306172043912f64;
    if x < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"transport.c\0" as *const u8 as *const libc::c_char,
            402 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x < 3.0f64 * 1.4901161193847656e-08f64 {
        (*result).val = x * x * x * x / 4.0f64;
        (*result).err = 4.0f64 * 2.2204460492503131e-16f64 * (*result).val;
        if fabs((*result).val) < 2.2250738585072014e-308f64 {
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"transport.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    } else if x <= 4.0f64 {
        let x2: libc::c_double = x * x;
        let t: libc::c_double = x2 / 8.0f64 - 0.5f64 - 0.5f64;
        let mut result_c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut transport5_cs, t, &mut result_c);
        (*result).val = x2 * x2 * result_c.val;
        (*result).err = x2 * x2 * result_c.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < --3.6043653389117154e+01f64 {
        let numexp: libc::c_int = (--3.6043653389117154e+01f64 / x) as libc::c_int
            + 1 as libc::c_int;
        let sumexp: libc::c_double = transport_sumexp(
            numexp,
            5 as libc::c_int,
            exp(-x),
            x,
        );
        let t_0: libc::c_double = 5.0f64 * log(x) - x + log(sumexp);
        if t_0 < -3.6043653389117154e+01f64 {
            (*result).val = val_infinity;
            (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * val_infinity;
        } else {
            let et: libc::c_double = exp(t_0);
            (*result).val = val_infinity - et;
            (*result)
                .err = 2.0f64 * 2.2204460492503131e-16f64
                * (val_infinity + (fabs(t_0) + 1.0f64) * et);
        }
        return GSL_SUCCESS as libc::c_int;
    } else if x < 3.0f64 / 2.2204460492503131e-16f64 {
        let numexp_0: libc::c_int = 1 as libc::c_int;
        let sumexp_0: libc::c_double = transport_sumexp(
            numexp_0,
            5 as libc::c_int,
            1.0f64,
            x,
        );
        let t_1: libc::c_double = 5.0f64 * log(x) - x + log(sumexp_0);
        if t_1 < -3.6043653389117154e+01f64 {
            (*result).val = val_infinity;
            (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * val_infinity;
        } else {
            let et_0: libc::c_double = exp(t_1);
            (*result).val = val_infinity - et_0;
            (*result)
                .err = 2.0f64 * 2.2204460492503131e-16f64
                * (val_infinity + (fabs(t_1) + 1.0f64) * et_0);
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let t_2: libc::c_double = 5.0f64 * log(x) - x;
        if t_2 < -3.6043653389117154e+01f64 {
            (*result).val = val_infinity;
            (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * val_infinity;
        } else {
            let et_1: libc::c_double = exp(t_2);
            (*result).val = val_infinity - et_1;
            (*result)
                .err = 2.0f64 * 2.2204460492503131e-16f64
                * (val_infinity + (fabs(t_2) + 1.0f64) * et_1);
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_transport_2(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_transport_2_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_transport_2_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"transport.c\0" as *const u8 as *const libc::c_char,
            476 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_transport_3(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_transport_3_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_transport_3_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"transport.c\0" as *const u8 as *const libc::c_char,
            481 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_transport_4(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_transport_4_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_transport_4_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"transport.c\0" as *const u8 as *const libc::c_char,
            486 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_transport_5(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_transport_5_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_transport_5_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"transport.c\0" as *const u8 as *const libc::c_char,
            491 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
