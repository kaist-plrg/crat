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
    fn gsl_sf_exp_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
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
unsafe extern "C" fn erfc8_sum(mut x: libc::c_double) -> libc::c_double {
    static mut P: [libc::c_double; 6] = [
        2.97886562639399288862f64,
        7.409740605964741794425f64,
        6.1602098531096305440906f64,
        5.019049726784267463450058f64,
        1.275366644729965952479585264f64,
        0.5641895835477550741253201704f64,
    ];
    static mut Q: [libc::c_double; 7] = [
        3.3690752069827527677f64,
        9.608965327192787870698f64,
        17.08144074746600431571095f64,
        12.0489519278551290360340491f64,
        9.396034016235054150430579648f64,
        2.260528520767326969591866945f64,
        1.0f64,
    ];
    let mut num: libc::c_double = 0.0f64;
    let mut den: libc::c_double = 0.0f64;
    let mut i: libc::c_int = 0;
    num = P[5 as libc::c_int as usize];
    i = 4 as libc::c_int;
    while i >= 0 as libc::c_int {
        num = x * num + P[i as usize];
        i -= 1;
        i;
    }
    den = Q[6 as libc::c_int as usize];
    i = 5 as libc::c_int;
    while i >= 0 as libc::c_int {
        den = x * den + Q[i as usize];
        i -= 1;
        i;
    }
    return num / den;
}
#[inline]
unsafe extern "C" fn erfc8(mut x: libc::c_double) -> libc::c_double {
    let mut e: libc::c_double = 0.;
    e = erfc8_sum(x);
    e *= exp(-x * x);
    return e;
}
#[inline]
unsafe extern "C" fn log_erfc8(mut x: libc::c_double) -> libc::c_double {
    let mut e: libc::c_double = 0.;
    e = erfc8_sum(x);
    e = log(e) - x * x;
    return e;
}
unsafe extern "C" fn erfseries(
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut coef: libc::c_double = x;
    let mut e: libc::c_double = coef;
    let mut del: libc::c_double = 0.;
    let mut k: libc::c_int = 0;
    k = 1 as libc::c_int;
    while k < 30 as libc::c_int {
        coef *= -x * x / k as libc::c_double;
        del = coef / (2.0f64 * k as libc::c_double + 1.0f64);
        e += del;
        k += 1;
        k;
    }
    (*result).val = 2.0f64 / 1.77245385090551602729816748334f64 * e;
    (*result)
        .err = 2.0f64 / 1.77245385090551602729816748334f64
        * (fabs(del) + 2.2204460492503131e-16f64);
    return GSL_SUCCESS as libc::c_int;
}
static mut erfc_xlt1_data: [libc::c_double; 20] = [
    1.06073416421769980345174155056f64,
    -0.42582445804381043569204735291f64,
    0.04955262679620434040357683080f64,
    0.00449293488768382749558001242f64,
    -0.00129194104658496953494224761f64,
    -0.00001836389292149396270416979f64,
    0.00002211114704099526291538556f64,
    -5.23337485234257134673693179020e-7f64,
    -2.78184788833537885382530989578e-7f64,
    1.41158092748813114560316684249e-8f64,
    2.72571296330561699984539141865e-9f64,
    -2.06343904872070629406401492476e-10f64,
    -2.14273991996785367924201401812e-11f64,
    2.22990255539358204580285098119e-12f64,
    1.36250074650698280575807934155e-13f64,
    -1.95144010922293091898995913038e-14f64,
    -6.85627169231704599442806370690e-16f64,
    1.44506492869699938239521607493e-16f64,
    2.45935306460536488037576200030e-18f64,
    -9.29599561220523396007359328540e-19f64,
];
static mut erfc_xlt1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: erfc_xlt1_data.as_ptr() as *mut _,
            order: 19 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 12 as libc::c_int,
        };
        init
    }
};
static mut erfc_x15_data: [libc::c_double; 25] = [
    0.44045832024338111077637466616f64,
    -0.143958836762168335790826895326f64,
    0.044786499817939267247056666937f64,
    -0.013343124200271211203618353102f64,
    0.003824682739750469767692372556f64,
    -0.001058699227195126547306482530f64,
    0.000283859419210073742736310108f64,
    -0.000073906170662206760483959432f64,
    0.000018725312521489179015872934f64,
    -4.62530981164919445131297264430e-6f64,
    1.11558657244432857487884006422e-6f64,
    -2.63098662650834130067808832725e-7f64,
    6.07462122724551777372119408710e-8f64,
    -1.37460865539865444777251011793e-8f64,
    3.05157051905475145520096717210e-9f64,
    -6.65174789720310713757307724790e-10f64,
    1.42483346273207784489792999706e-10f64,
    -3.00141127395323902092018744545e-11f64,
    6.22171792645348091472914001250e-12f64,
    -1.26994639225668496876152836555e-12f64,
    2.55385883033257575402681845385e-13f64,
    -5.06258237507038698392265499770e-14f64,
    9.89705409478327321641264227110e-15f64,
    -1.90685978789192181051961024995e-15f64,
    3.50826648032737849245113757340e-16f64,
];
static mut erfc_x15_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: erfc_x15_data.as_ptr() as *mut _,
            order: 24 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 16 as libc::c_int,
        };
        init
    }
};
static mut erfc_x510_data: [libc::c_double; 20] = [
    1.11684990123545698684297865808f64,
    0.003736240359381998520654927536f64,
    -0.000916623948045470238763619870f64,
    0.000199094325044940833965078819f64,
    -0.000040276384918650072591781859f64,
    7.76515264697061049477127605790e-6f64,
    -1.44464794206689070402099225301e-6f64,
    2.61311930343463958393485241947e-7f64,
    -4.61833026634844152345304095560e-8f64,
    8.00253111512943601598732144340e-9f64,
    -1.36291114862793031395712122089e-9f64,
    2.28570483090160869607683087722e-10f64,
    -3.78022521563251805044056974560e-11f64,
    6.17253683874528285729910462130e-12f64,
    -9.96019290955316888445830597430e-13f64,
    1.58953143706980770269506726000e-13f64,
    -2.51045971047162509999527428316e-14f64,
    3.92607828989125810013581287560e-15f64,
    -6.07970619384160374392535453420e-16f64,
    9.12600607264794717315507477670e-17f64,
];
static mut erfc_x510_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: erfc_x510_data.as_ptr() as *mut _,
            order: 19 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 12 as libc::c_int,
        };
        init
    }
};
pub unsafe extern "C" fn gsl_sf_erfc_e(
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let ax: libc::c_double = fabs(x);
    let mut e_val: libc::c_double = 0.;
    let mut e_err: libc::c_double = 0.;
    if ax <= 1.0f64 {
        let mut t: libc::c_double = 2.0f64 * ax - 1.0f64;
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut erfc_xlt1_cs, t, &mut c);
        e_val = c.val;
        e_err = c.err;
    } else if ax <= 5.0f64 {
        let mut ex2: libc::c_double = exp(-x * x);
        let mut t_0: libc::c_double = 0.5f64 * (ax - 3.0f64);
        let mut c_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut erfc_x15_cs, t_0, &mut c_0);
        e_val = ex2 * c_0.val;
        e_err = ex2 * (c_0.err + 2.0f64 * fabs(x) * 2.2204460492503131e-16f64);
    } else if ax < 10.0f64 {
        let mut exterm: libc::c_double = exp(-x * x) / ax;
        let mut t_1: libc::c_double = (2.0f64 * ax - 15.0f64) / 5.0f64;
        let mut c_1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut erfc_x510_cs, t_1, &mut c_1);
        e_val = exterm * c_1.val;
        e_err = exterm
            * (c_1.err + 2.0f64 * fabs(x) * 2.2204460492503131e-16f64
                + 2.2204460492503131e-16f64);
    } else {
        e_val = erfc8(ax);
        e_err = (x * x + 1.0f64) * 2.2204460492503131e-16f64 * fabs(e_val);
    }
    if x < 0.0f64 {
        (*result).val = 2.0f64 - e_val;
        (*result).err = e_err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
    } else {
        (*result).val = e_val;
        (*result).err = e_err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_log_erfc_e(
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x * x < 10.0f64 * 2.4607833005759251e-03f64 {
        let y: libc::c_double = x / 1.77245385090551602729816748334f64;
        let c3: libc::c_double = (4.0f64 - 3.14159265358979323846f64) / 3.0f64;
        let c4: libc::c_double = 2.0f64 * (1.0f64 - 3.14159265358979323846f64 / 3.0f64);
        let c5: libc::c_double = -0.001829764677455021f64;
        let c6: libc::c_double = 0.02629651521057465f64;
        let c7: libc::c_double = -0.01621575378835404f64;
        let c8: libc::c_double = 0.00125993961762116f64;
        let c9: libc::c_double = 0.00556964649138f64;
        let c10: libc::c_double = -0.0045563339802f64;
        let c11: libc::c_double = 0.0009461589032f64;
        let c12: libc::c_double = 0.0013200243174f64;
        let c13: libc::c_double = -0.00142906f64;
        let c14: libc::c_double = 0.00048204f64;
        let mut series: libc::c_double = c8
            + y * (c9 + y * (c10 + y * (c11 + y * (c12 + y * (c13 + c14 * y)))));
        series = y
            * (1.0f64
                + y
                    * (1.0f64
                        + y
                            * (c3
                                + y * (c4 + y * (c5 + y * (c6 + y * (c7 + y * series)))))));
        (*result).val = -2.0f64 * series;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x > 8.0f64 {
        (*result).val = log_erfc8(x);
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut result_erfc: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        gsl_sf_erfc_e(x, &mut result_erfc);
        (*result).val = log(result_erfc.val);
        (*result).err = fabs(result_erfc.err / result_erfc.val);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_erf_e(
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if fabs(x) < 1.0f64 {
        return erfseries(x, result)
    } else {
        let mut result_erfc: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        gsl_sf_erfc_e(x, &mut result_erfc);
        (*result).val = 1.0f64 - result_erfc.val;
        (*result).err = result_erfc.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_erf_Z_e(
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let ex2: libc::c_double = exp(-x * x / 2.0f64);
    (*result)
        .val = ex2 / (1.41421356237309504880f64 * 1.77245385090551602729816748334f64);
    (*result).err = fabs(x * (*result).val) * 2.2204460492503131e-16f64;
    (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
    if fabs((*result).val) < 2.2250738585072014e-308f64 {
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"erfc.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_erf_Q_e(
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut result_erfc: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat: libc::c_int = gsl_sf_erfc_e(
        x / 1.41421356237309504880f64,
        &mut result_erfc,
    );
    (*result).val = 0.5f64 * result_erfc.val;
    (*result).err = 0.5f64 * result_erfc.err;
    (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
    return stat;
}
pub unsafe extern "C" fn gsl_sf_hazard_e(
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x < 25.0f64 {
        let mut result_ln_erfc: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let stat_l: libc::c_int = gsl_sf_log_erfc_e(
            x / 1.41421356237309504880f64,
            &mut result_ln_erfc,
        );
        let lnc: libc::c_double = -0.22579135264472743236f64;
        let arg: libc::c_double = lnc - 0.5f64 * x * x - result_ln_erfc.val;
        let stat_e: libc::c_int = gsl_sf_exp_e(arg, result);
        (*result).err
            += 3.0f64 * (1.0f64 + fabs(x)) * 2.2204460492503131e-16f64
                * fabs((*result).val);
        (*result).err += fabs(result_ln_erfc.err * (*result).val);
        return if stat_l != GSL_SUCCESS as libc::c_int {
            stat_l
        } else if stat_e != GSL_SUCCESS as libc::c_int {
            stat_e
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else {
        let ix2: libc::c_double = 1.0f64 / (x * x);
        let corrB: libc::c_double = 1.0f64 - 9.0f64 * ix2 * (1.0f64 - 11.0f64 * ix2);
        let corrM: libc::c_double = 1.0f64
            - 5.0f64 * ix2 * (1.0f64 - 7.0f64 * ix2 * corrB);
        let corrT: libc::c_double = 1.0f64 - ix2 * (1.0f64 - 3.0f64 * ix2 * corrM);
        (*result).val = x / corrT;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_erfc(mut x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_erfc_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_erfc_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"erfc.c\0" as *const u8 as *const libc::c_char,
            438 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_log_erfc(mut x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_log_erfc_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_log_erfc_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"erfc.c\0" as *const u8 as *const libc::c_char,
            443 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_erf(mut x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_erf_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_erf_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"erfc.c\0" as *const u8 as *const libc::c_char,
            448 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_erf_Z(mut x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_erf_Z_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_erf_Z_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"erfc.c\0" as *const u8 as *const libc::c_char,
            453 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_erf_Q(mut x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_erf_Q_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_erf_Q_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"erfc.c\0" as *const u8 as *const libc::c_char,
            458 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_hazard(mut x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hazard_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hazard_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"erfc.c\0" as *const u8 as *const libc::c_char,
            463 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
