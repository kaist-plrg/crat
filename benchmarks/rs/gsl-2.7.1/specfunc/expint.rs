use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_gamma_inc_e(
        a: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
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
static mut AE11_data: [libc::c_double; 39] = [
    0.121503239716065790f64,
    -0.065088778513550150f64,
    0.004897651357459670f64,
    -0.000649237843027216f64,
    0.000093840434587471f64,
    0.000000420236380882f64,
    -0.000008113374735904f64,
    0.000002804247688663f64,
    0.000000056487164441f64,
    -0.000000344809174450f64,
    0.000000058209273578f64,
    0.000000038711426349f64,
    -0.000000012453235014f64,
    -0.000000005118504888f64,
    0.000000002148771527f64,
    0.000000000868459898f64,
    -0.000000000343650105f64,
    -0.000000000179796603f64,
    0.000000000047442060f64,
    0.000000000040423282f64,
    -0.000000000003543928f64,
    -0.000000000008853444f64,
    -0.000000000000960151f64,
    0.000000000001692921f64,
    0.000000000000607990f64,
    -0.000000000000224338f64,
    -0.000000000000200327f64,
    -0.000000000000006246f64,
    0.000000000000045571f64,
    0.000000000000016383f64,
    -0.000000000000005561f64,
    -0.000000000000006074f64,
    -0.000000000000000862f64,
    0.000000000000001223f64,
    0.000000000000000716f64,
    -0.000000000000000024f64,
    -0.000000000000000201f64,
    -0.000000000000000082f64,
    0.000000000000000017f64,
];
static mut AE11_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: AE11_data.as_ptr() as *mut _,
            order: 38 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 20 as libc::c_int,
        };
        init
    }
};
static mut AE12_data: [libc::c_double; 25] = [
    0.582417495134726740f64,
    -0.158348850905782750f64,
    -0.006764275590323141f64,
    0.005125843950185725f64,
    0.000435232492169391f64,
    -0.000143613366305483f64,
    -0.000041801320556301f64,
    -0.000002713395758640f64,
    0.000001151381913647f64,
    0.000000420650022012f64,
    0.000000066581901391f64,
    0.000000000662143777f64,
    -0.000000002844104870f64,
    -0.000000000940724197f64,
    -0.000000000177476602f64,
    -0.000000000015830222f64,
    0.000000000002905732f64,
    0.000000000001769356f64,
    0.000000000000492735f64,
    0.000000000000093709f64,
    0.000000000000010707f64,
    -0.000000000000000537f64,
    -0.000000000000000716f64,
    -0.000000000000000244f64,
    -0.000000000000000058f64,
];
static mut AE12_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: AE12_data.as_ptr() as *mut _,
            order: 24 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 15 as libc::c_int,
        };
        init
    }
};
static mut E11_data: [libc::c_double; 19] = [
    -16.11346165557149402600f64,
    7.79407277874268027690f64,
    -1.95540581886314195070f64,
    0.37337293866277945612f64,
    -0.05692503191092901938f64,
    0.00721107776966009185f64,
    -0.00078104901449841593f64,
    0.00007388093356262168f64,
    -0.00000620286187580820f64,
    0.00000046816002303176f64,
    -0.00000003209288853329f64,
    0.00000000201519974874f64,
    -0.00000000011673686816f64,
    0.00000000000627627066f64,
    -0.00000000000031481541f64,
    0.00000000000001479904f64,
    -0.00000000000000065457f64,
    0.00000000000000002733f64,
    -0.00000000000000000108f64,
];
static mut E11_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: E11_data.as_ptr() as *mut _,
            order: 18 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 13 as libc::c_int,
        };
        init
    }
};
static mut E12_data: [libc::c_double; 16] = [
    -0.03739021479220279500f64,
    0.04272398606220957700f64,
    -0.13031820798497005440f64,
    0.01441912402469889073f64,
    -0.00134617078051068022f64,
    0.00010731029253063780f64,
    -0.00000742999951611943f64,
    0.00000045377325690753f64,
    -0.00000002476417211390f64,
    0.00000000122076581374f64,
    -0.00000000005485141480f64,
    0.00000000000226362142f64,
    -0.00000000000008635897f64,
    0.00000000000000306291f64,
    -0.00000000000000010148f64,
    0.00000000000000000315f64,
];
static mut E12_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: E12_data.as_ptr() as *mut _,
            order: 15 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 10 as libc::c_int,
        };
        init
    }
};
static mut AE13_data: [libc::c_double; 25] = [
    -0.605773246640603460f64,
    -0.112535243483660900f64,
    0.013432266247902779f64,
    -0.001926845187381145f64,
    0.000309118337720603f64,
    -0.000053564132129618f64,
    0.000009827812880247f64,
    -0.000001885368984916f64,
    0.000000374943193568f64,
    -0.000000076823455870f64,
    0.000000016143270567f64,
    -0.000000003466802211f64,
    0.000000000758754209f64,
    -0.000000000168864333f64,
    0.000000000038145706f64,
    -0.000000000008733026f64,
    0.000000000002023672f64,
    -0.000000000000474132f64,
    0.000000000000112211f64,
    -0.000000000000026804f64,
    0.000000000000006457f64,
    -0.000000000000001568f64,
    0.000000000000000383f64,
    -0.000000000000000094f64,
    0.000000000000000023f64,
];
static mut AE13_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: AE13_data.as_ptr() as *mut _,
            order: 24 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 15 as libc::c_int,
        };
        init
    }
};
static mut AE14_data: [libc::c_double; 26] = [
    -0.18929180007530170f64,
    -0.08648117855259871f64,
    0.00722410154374659f64,
    -0.00080975594575573f64,
    0.00010999134432661f64,
    -0.00001717332998937f64,
    0.00000298562751447f64,
    -0.00000056596491457f64,
    0.00000011526808397f64,
    -0.00000002495030440f64,
    0.00000000569232420f64,
    -0.00000000135995766f64,
    0.00000000033846628f64,
    -0.00000000008737853f64,
    0.00000000002331588f64,
    -0.00000000000641148f64,
    0.00000000000181224f64,
    -0.00000000000052538f64,
    0.00000000000015592f64,
    -0.00000000000004729f64,
    0.00000000000001463f64,
    -0.00000000000000461f64,
    0.00000000000000148f64,
    -0.00000000000000048f64,
    0.00000000000000016f64,
    -0.00000000000000005f64,
];
static mut AE14_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: AE14_data.as_ptr() as *mut _,
            order: 25 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 13 as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn expint_E1_impl(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
    scale: libc::c_int,
) -> libc::c_int {
    let xmaxt: libc::c_double = --7.0839641853226408e+02f64;
    let xmax: libc::c_double = xmaxt - log(xmaxt);
    if x < -xmax && scale == 0 {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"expint.c\0" as *const u8 as *const libc::c_char,
            298 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    } else if x <= -10.0f64 {
        let s: libc::c_double = 1.0f64 / x * (if scale != 0 { 1.0f64 } else { exp(-x) });
        let mut result_c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut AE11_cs, 20.0f64 / x + 1.0f64, &mut result_c);
        (*result).val = s * (1.0f64 + result_c.val);
        (*result).err = s * result_c.err;
        (*result).err
            += 2.0f64 * 2.2204460492503131e-16f64 * (fabs(x) + 1.0f64)
                * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x <= -4.0f64 {
        let s_0: libc::c_double = 1.0f64 / x
            * (if scale != 0 { 1.0f64 } else { exp(-x) });
        let mut result_c_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut AE12_cs, (40.0f64 / x + 7.0f64) / 3.0f64, &mut result_c_0);
        (*result).val = s_0 * (1.0f64 + result_c_0.val);
        (*result).err = s_0 * result_c_0.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x <= -1.0f64 {
        let ln_term: libc::c_double = -log(fabs(x));
        let scale_factor: libc::c_double = if scale != 0 { exp(x) } else { 1.0f64 };
        let mut result_c_1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut E11_cs, (2.0f64 * x + 5.0f64) / 3.0f64, &mut result_c_1);
        (*result).val = scale_factor * (ln_term + result_c_1.val);
        (*result)
            .err = scale_factor
            * (result_c_1.err + 2.2204460492503131e-16f64 * fabs(ln_term));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x == 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"expint.c\0" as *const u8 as *const libc::c_char,
            329 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x <= 1.0f64 {
        let ln_term_0: libc::c_double = -log(fabs(x));
        let scale_factor_0: libc::c_double = if scale != 0 { exp(x) } else { 1.0f64 };
        let mut result_c_2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut E12_cs, x, &mut result_c_2);
        (*result).val = scale_factor_0 * (ln_term_0 - 0.6875f64 + x + result_c_2.val);
        (*result)
            .err = scale_factor_0
            * (result_c_2.err + 2.2204460492503131e-16f64 * fabs(ln_term_0));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x <= 4.0f64 {
        let s_1: libc::c_double = 1.0f64 / x
            * (if scale != 0 { 1.0f64 } else { exp(-x) });
        let mut result_c_3: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut AE13_cs, (8.0f64 / x - 5.0f64) / 3.0f64, &mut result_c_3);
        (*result).val = s_1 * (1.0f64 + result_c_3.val);
        (*result).err = s_1 * result_c_3.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x <= xmax || scale != 0 {
        let s_2: libc::c_double = 1.0f64 / x
            * (if scale != 0 { 1.0f64 } else { exp(-x) });
        let mut result_c_4: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut AE14_cs, 8.0f64 / x - 1.0f64, &mut result_c_4);
        (*result).val = s_2 * (1.0f64 + result_c_4.val);
        (*result).err = s_2 * (2.2204460492503131e-16f64 + result_c_4.err);
        (*result).err
            += 2.0f64 * (x + 1.0f64) * 2.2204460492503131e-16f64 * fabs((*result).val);
        if (*result).val == 0.0f64 {
            (*result).val = 0.0f64;
            (*result).err = 2.2250738585072014e-308f64;
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"expint.c\0" as *const u8 as *const libc::c_char,
                358 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        } else {
            return GSL_SUCCESS as libc::c_int
        }
    } else {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"expint.c\0" as *const u8 as *const libc::c_char,
            363 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    };
}
unsafe extern "C" fn expint_E2_impl(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
    scale: libc::c_int,
) -> libc::c_int {
    let xmaxt: libc::c_double = --7.0839641853226408e+02f64;
    let xmax: libc::c_double = xmaxt - log(xmaxt);
    if x < -xmax && scale == 0 {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"expint.c\0" as *const u8 as *const libc::c_char,
            377 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    } else if x == 0.0f64 {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x < 100.0f64 {
        let ex: libc::c_double = if scale != 0 { 1.0f64 } else { exp(-x) };
        let mut result_E1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_E1: libc::c_int = expint_E1_impl(x, &mut result_E1, scale);
        (*result).val = ex - x * result_E1.val;
        (*result).err = 2.2204460492503131e-16f64 * ex + fabs(x) * result_E1.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat_E1;
    } else if x < xmax || scale != 0 {
        let s: libc::c_double = if scale != 0 { 1.0f64 } else { exp(-x) };
        let c1: libc::c_double = -2.0f64;
        let c2: libc::c_double = 6.0f64;
        let c3: libc::c_double = -24.0f64;
        let c4: libc::c_double = 120.0f64;
        let c5: libc::c_double = -720.0f64;
        let c6: libc::c_double = 5040.0f64;
        let c7: libc::c_double = -40320.0f64;
        let c8: libc::c_double = 362880.0f64;
        let c9: libc::c_double = -3628800.0f64;
        let c10: libc::c_double = 39916800.0f64;
        let c11: libc::c_double = -479001600.0f64;
        let c12: libc::c_double = 6227020800.0f64;
        let c13: libc::c_double = -87178291200.0f64;
        let y: libc::c_double = 1.0f64 / x;
        let sum6: libc::c_double = c6
            + y
                * (c7
                    + y * (c8 + y * (c9 + y * (c10 + y * (c11 + y * (c12 + y * c13))))));
        let sum: libc::c_double = y
            * (c1 + y * (c2 + y * (c3 + y * (c4 + y * (c5 + y * sum6)))));
        (*result).val = s * (1.0f64 + sum) / x;
        (*result)
            .err = 2.0f64 * (x + 1.0f64) * 2.2204460492503131e-16f64 * (*result).val;
        if (*result).val == 0.0f64 {
            (*result).val = 0.0f64;
            (*result).err = 2.2250738585072014e-308f64;
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"expint.c\0" as *const u8 as *const libc::c_char,
                413 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        } else {
            return GSL_SUCCESS as libc::c_int
        }
    } else {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"expint.c\0" as *const u8 as *const libc::c_char,
            418 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    };
}
unsafe extern "C" fn expint_En_impl(
    n: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
    scale: libc::c_int,
) -> libc::c_int {
    if n < 0 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"expint.c\0" as *const u8 as *const libc::c_char,
            426 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if n == 0 as libc::c_int {
        if x == 0 as libc::c_int as libc::c_double {
            (*result).val = ::std::f32::NAN as libc::c_double;
            (*result).err = ::std::f32::NAN as libc::c_double;
            gsl_error(
                b"domain error\0" as *const u8 as *const libc::c_char,
                b"expint.c\0" as *const u8 as *const libc::c_char,
                429 as libc::c_int,
                GSL_EDOM as libc::c_int,
            );
            return GSL_EDOM as libc::c_int;
        } else {
            (*result).val = (if scale != 0 { 1.0f64 } else { exp(-x) }) / x;
            (*result)
                .err = 2 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64
                * fabs((*result).val);
            if fabs((*result).val) < 2.2250738585072014e-308f64 {
                gsl_error(
                    b"underflow\0" as *const u8 as *const libc::c_char,
                    b"expint.c\0" as *const u8 as *const libc::c_char,
                    433 as libc::c_int,
                    GSL_EUNDRFLW as libc::c_int,
                );
                return GSL_EUNDRFLW as libc::c_int;
            }
            return GSL_SUCCESS as libc::c_int;
        }
    } else if n == 1 as libc::c_int {
        return expint_E1_impl(x, result, scale)
    } else if n == 2 as libc::c_int {
        return expint_E2_impl(x, result, scale)
    } else {
        if x < 0 as libc::c_int as libc::c_double {
            (*result).val = ::std::f32::NAN as libc::c_double;
            (*result).err = ::std::f32::NAN as libc::c_double;
            gsl_error(
                b"domain error\0" as *const u8 as *const libc::c_char,
                b"expint.c\0" as *const u8 as *const libc::c_char,
                442 as libc::c_int,
                GSL_EDOM as libc::c_int,
            );
            return GSL_EDOM as libc::c_int;
        }
        if x == 0 as libc::c_int as libc::c_double {
            (*result)
                .val = (if scale != 0 {
                exp(x)
            } else {
                1 as libc::c_int as libc::c_double
            }) * (1 as libc::c_int as libc::c_double / (n as libc::c_double - 1.0f64));
            (*result)
                .err = 2 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64
                * fabs((*result).val);
            if fabs((*result).val) < 2.2250738585072014e-308f64 {
                gsl_error(
                    b"underflow\0" as *const u8 as *const libc::c_char,
                    b"expint.c\0" as *const u8 as *const libc::c_char,
                    447 as libc::c_int,
                    GSL_EUNDRFLW as libc::c_int,
                );
                return GSL_EUNDRFLW as libc::c_int;
            }
            return GSL_SUCCESS as libc::c_int;
        } else {
            let mut result_g: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut prefactor: libc::c_double = pow(
                x,
                (n - 1 as libc::c_int) as libc::c_double,
            );
            let mut status: libc::c_int = gsl_sf_gamma_inc_e(
                (1 as libc::c_int - n) as libc::c_double,
                x,
                &mut result_g,
            );
            let mut scale_factor: libc::c_double = if scale != 0 {
                exp(x)
            } else {
                1.0f64
            };
            (*result).val = scale_factor * prefactor * result_g.val;
            (*result)
                .err = 2 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64
                * fabs((*result).val);
            (*result).err
                += 2 as libc::c_int as libc::c_double
                    * fabs(scale_factor * prefactor * result_g.err);
            if status == GSL_SUCCESS as libc::c_int {
                if fabs((*result).val) < 2.2250738585072014e-308f64 {
                    gsl_error(
                        b"underflow\0" as *const u8 as *const libc::c_char,
                        b"expint.c\0" as *const u8 as *const libc::c_char,
                        457 as libc::c_int,
                        GSL_EUNDRFLW as libc::c_int,
                    );
                    return GSL_EUNDRFLW as libc::c_int;
                }
            }
            return status;
        }
    };
}
pub unsafe extern "C" fn gsl_sf_expint_E1_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    return expint_E1_impl(x, result, 0 as libc::c_int);
}
pub unsafe extern "C" fn gsl_sf_expint_E1_scaled_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    return expint_E1_impl(x, result, 1 as libc::c_int);
}
pub unsafe extern "C" fn gsl_sf_expint_E2_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    return expint_E2_impl(x, result, 0 as libc::c_int);
}
pub unsafe extern "C" fn gsl_sf_expint_E2_scaled_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    return expint_E2_impl(x, result, 1 as libc::c_int);
}
pub unsafe extern "C" fn gsl_sf_expint_En_e(
    n: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    return expint_En_impl(n, x, result, 0 as libc::c_int);
}
pub unsafe extern "C" fn gsl_sf_expint_En_scaled_e(
    n: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    return expint_En_impl(n, x, result, 1 as libc::c_int);
}
pub unsafe extern "C" fn gsl_sf_expint_Ei_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_sf_expint_E1_e(-x, result);
    (*result).val = -(*result).val;
    return status;
}
pub unsafe extern "C" fn gsl_sf_expint_Ei_scaled_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_sf_expint_E1_scaled_e(-x, result);
    (*result).val = -(*result).val;
    return status;
}
pub unsafe extern "C" fn gsl_sf_expint_E1(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_expint_E1_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_expint_E1_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"expint.c\0" as *const u8 as *const libc::c_char,
            545 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_expint_E1_scaled(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_expint_E1_scaled_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_expint_E1_scaled_e(x, &result)\0" as *const u8
                as *const libc::c_char,
            b"expint.c\0" as *const u8 as *const libc::c_char,
            550 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_expint_E2(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_expint_E2_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_expint_E2_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"expint.c\0" as *const u8 as *const libc::c_char,
            555 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_expint_E2_scaled(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_expint_E2_scaled_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_expint_E2_scaled_e(x, &result)\0" as *const u8
                as *const libc::c_char,
            b"expint.c\0" as *const u8 as *const libc::c_char,
            560 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_expint_En(
    n: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_expint_En_e(n, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_expint_En_e(n, x, &result)\0" as *const u8 as *const libc::c_char,
            b"expint.c\0" as *const u8 as *const libc::c_char,
            565 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_expint_En_scaled(
    n: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_expint_En_scaled_e(n, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_expint_En_scaled_e(n, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"expint.c\0" as *const u8 as *const libc::c_char,
            570 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_expint_Ei(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_expint_Ei_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_expint_Ei_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"expint.c\0" as *const u8 as *const libc::c_char,
            575 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_expint_Ei_scaled(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_expint_Ei_scaled_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_expint_Ei_scaled_e(x, &result)\0" as *const u8
                as *const libc::c_char,
            b"expint.c\0" as *const u8 as *const libc::c_char,
            580 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
