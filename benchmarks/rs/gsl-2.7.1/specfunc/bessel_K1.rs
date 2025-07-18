use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_exp_mult_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        y: libc::c_double,
        dy: libc::c_double,
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
unsafe extern "C" fn gsl_poly_eval(
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
static mut k1_poly: [libc::c_double; 9] = [
    -3.0796575782920622440538935e-01f64,
    -8.5370719728650778045782736e-02f64,
    -4.6421827664715603298154971e-03f64,
    -1.1253607036630425931072996e-04f64,
    -1.5592887702110907110292728e-06f64,
    -1.4030163679125934402498239e-08f64,
    -8.8718998640336832196558868e-11f64,
    -4.1614323580221539328960335e-13f64,
    -1.5261293392975541707230366e-15f64,
];
static mut i1_poly: [libc::c_double; 7] = [
    8.3333333333333325191635191e-02f64,
    6.9444444444467956461838830e-03f64,
    3.4722222211230452695165215e-04f64,
    1.1574075952009842696580084e-05f64,
    2.7555870002088181016676934e-07f64,
    4.9724386164128529514040614e-09f64,
    0.,
];
static mut ak1_data: [libc::c_double; 25] = [
    2.07996868001418246e-01f64,
    1.62581565017881476e-01f64,
    -5.87070423518863640e-03f64,
    4.95021520115789501e-04f64,
    -5.78958347598556986e-05f64,
    8.18614610209334726e-06f64,
    -1.31604832009487277e-06f64,
    2.32546031520101213e-07f64,
    -4.42206518311557987e-08f64,
    8.92163994883100361e-09f64,
    -1.89046270526983427e-09f64,
    4.17568808108504702e-10f64,
    -9.55912361791375794e-11f64,
    2.25769353153867758e-11f64,
    -5.48128000211158482e-12f64,
    1.36386122546441926e-12f64,
    -3.46936690565986409e-13f64,
    9.00354564415705942e-14f64,
    -2.37950577776254432e-14f64,
    6.39447503964025336e-15f64,
    -1.74498363492322044e-15f64,
    4.82994547989290473e-16f64,
    -1.35460927805445606e-16f64,
    3.84604274446777234e-17f64,
    -1.10456856122581316e-17f64,
];
static mut ak1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: ak1_data.as_ptr() as *mut _,
            order: 24 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 9 as libc::c_int,
        };
        init
    }
};
static mut ak12_data: [libc::c_double; 14] = [
    0.637930834373900104E-1f64,
    0.283288781304972094E-1f64,
    -0.247537067390525035E-3f64,
    0.577197245160724882E-5f64,
    -0.206893921953654830E-6f64,
    0.973998344138180418E-8f64,
    -0.558533614038062498E-9f64,
    0.373299663404618524E-10f64,
    -0.282505196102322545E-11f64,
    0.237201900248414417E-12f64,
    -0.217667738799175398E-13f64,
    0.215791416161603245E-14f64,
    -0.229019693071826928E-15f64,
    0.258288572982327496E-16f64,
];
static mut ak12_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: ak12_data.as_ptr() as *mut _,
            order: 13 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 7 as libc::c_int,
        };
        init
    }
};
pub unsafe extern "C" fn gsl_sf_bessel_K1_scaled_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_K1.c\0" as *const u8 as *const libc::c_char,
            137 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x < 2.0f64 * 2.2250738585072014e-308f64 {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"bessel_K1.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    } else if x < 1.0f64 {
        let lx: libc::c_double = log(x);
        let ex: libc::c_double = exp(x);
        let x2: libc::c_double = x * x;
        let t: libc::c_double = 0.25f64 * x2;
        let i1: libc::c_double = 0.5f64 * x
            * (1.0f64
                + t
                    * (0.5f64
                        + t
                            * gsl_poly_eval(
                                i1_poly.as_mut_ptr() as *const libc::c_double,
                                6 as libc::c_int,
                                t,
                            )));
        (*result)
            .val = ex
            * (x2
                * gsl_poly_eval(
                    k1_poly.as_mut_ptr() as *const libc::c_double,
                    9 as libc::c_int,
                    x2,
                ) + x * lx * i1 + 1 as libc::c_int as libc::c_double) / x;
        (*result).err = ex * (1.6f64 + fabs(lx) * 0.6f64) * 2.2204460492503131e-16f64;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x <= 8.0f64 {
        let sx: libc::c_double = sqrt(x);
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut ak1_cs, (16.0f64 / x - 9.0f64) / 7.0f64, &mut c);
        (*result).val = (1.375f64 + c.val) / sx;
        (*result).err = c.err / sx;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let sx_0: libc::c_double = sqrt(x);
        let mut c_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut ak12_cs, 16.0f64 / x - 1.0f64, &mut c_0);
        (*result).val = (1.25f64 + c_0.val) / sx_0;
        (*result).err = c_0.err / sx_0;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_K1_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_K1.c\0" as *const u8 as *const libc::c_char,
            179 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x < 2.0f64 * 2.2250738585072014e-308f64 {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"bessel_K1.c\0" as *const u8 as *const libc::c_char,
            182 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    } else if x < 1.0f64 {
        let lx: libc::c_double = log(x);
        let x2: libc::c_double = x * x;
        let t: libc::c_double = 0.25f64 * x2;
        let i1: libc::c_double = 0.5f64 * x
            * (1.0f64
                + t
                    * (0.5f64
                        + t
                            * gsl_poly_eval(
                                i1_poly.as_mut_ptr() as *const libc::c_double,
                                6 as libc::c_int,
                                t,
                            )));
        (*result)
            .val = (x2
            * gsl_poly_eval(
                k1_poly.as_mut_ptr() as *const libc::c_double,
                9 as libc::c_int,
                x2,
            ) + x * lx * i1 + 1 as libc::c_int as libc::c_double) / x;
        (*result).err = (1.6f64 + fabs(lx) * 0.6f64) * 2.2204460492503131e-16f64;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut K1_scaled: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_K1: libc::c_int = gsl_sf_bessel_K1_scaled_e(x, &mut K1_scaled);
        let mut stat_e: libc::c_int = gsl_sf_exp_mult_err_e(
            -x,
            0.0f64,
            K1_scaled.val,
            K1_scaled.err,
            result,
        );
        (*result)
            .err = fabs((*result).val)
            * (2.2204460492503131e-16f64 * fabs(x) + K1_scaled.err / K1_scaled.val);
        return if stat_e != GSL_SUCCESS as libc::c_int {
            stat_e
        } else if stat_K1 != GSL_SUCCESS as libc::c_int {
            stat_K1
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_K1_scaled(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_K1_scaled_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_K1_scaled_e(x, &result)\0" as *const u8
                as *const libc::c_char,
            b"bessel_K1.c\0" as *const u8 as *const libc::c_char,
            211 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_bessel_K1(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_K1_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_K1_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"bessel_K1.c\0" as *const u8 as *const libc::c_char,
            216 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
