use ::libc;
extern "C" {
    fn sin(_: libc::c_double) -> libc::c_double;
    fn cosh(_: libc::c_double) -> libc::c_double;
    fn sinh(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
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
static mut g1_dat: [libc::c_double; 14] = [
    -1.14516408366268311786898152867f64,
    0.00636085311347084238122955495f64,
    0.00186245193007206848934643657f64,
    0.000152833085873453507081227824f64,
    0.000017017464011802038795324732f64,
    -6.4597502923347254354668326451e-07f64,
    -5.1819848432519380894104312968e-08f64,
    4.5189092894858183051123180797e-10f64,
    3.2433227371020873043666259180e-11f64,
    6.8309434024947522875432400828e-13f64,
    2.8353502755172101513119628130e-14f64,
    -7.9883905769323592875638087541e-16f64,
    -3.3726677300771949833341213457e-17f64,
    -3.6586334809210520744054437104e-20f64,
];
static mut g1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: g1_dat.as_ptr() as *mut _,
            order: 13 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 7 as libc::c_int,
        };
        init
    }
};
static mut g2_dat: [libc::c_double; 15] = [
    1.882645524949671835019616975350f64,
    -0.077490658396167518329547945212f64,
    -0.018256714847324929419579340950f64,
    0.0006338030209074895795923971731f64,
    0.0000762290543508729021194461175f64,
    -9.5501647561720443519853993526e-07f64,
    -8.8927268107886351912431512955e-08f64,
    -1.9521334772319613740511880132e-09f64,
    -9.4003052735885162111769579771e-11f64,
    4.6875133849532393179290879101e-12f64,
    2.2658535746925759582447545145e-13f64,
    -1.1725509698488015111878735251e-15f64,
    -7.0441338200245222530843155877e-17f64,
    -2.4377878310107693650659740228e-18f64,
    -7.5225243218253901727164675011e-20f64,
];
static mut g2_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: g2_dat.as_ptr() as *mut _,
            order: 14 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 8 as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn gsl_sf_temme_gamma(
    nu: libc::c_double,
    mut g_1pnu: *mut libc::c_double,
    mut g_1mnu: *mut libc::c_double,
    mut g1: *mut libc::c_double,
    mut g2: *mut libc::c_double,
) -> libc::c_int {
    let anu: libc::c_double = fabs(nu);
    let x: libc::c_double = 4.0f64 * anu - 1.0f64;
    let mut r_g1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut r_g2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    cheb_eval_e(&mut g1_cs, x, &mut r_g1);
    cheb_eval_e(&mut g2_cs, x, &mut r_g2);
    *g1 = r_g1.val;
    *g2 = r_g2.val;
    *g_1mnu = 1.0f64 / (r_g2.val + nu * r_g1.val);
    *g_1pnu = 1.0f64 / (r_g2.val - nu * r_g1.val);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_bessel_Y_temme(
    nu: libc::c_double,
    x: libc::c_double,
    mut Ynu: *mut gsl_sf_result,
    mut Ynup1: *mut gsl_sf_result,
) -> libc::c_int {
    let max_iter: libc::c_int = 15000 as libc::c_int;
    let half_x: libc::c_double = 0.5f64 * x;
    let ln_half_x: libc::c_double = log(half_x);
    let half_x_nu: libc::c_double = exp(nu * ln_half_x);
    let pi_nu: libc::c_double = 3.14159265358979323846f64 * nu;
    let alpha: libc::c_double = pi_nu / 2.0f64;
    let sigma: libc::c_double = -nu * ln_half_x;
    let sinrat: libc::c_double = if fabs(pi_nu) < 2.2204460492503131e-16f64 {
        1.0f64
    } else {
        pi_nu / sin(pi_nu)
    };
    let sinhrat: libc::c_double = if fabs(sigma) < 2.2204460492503131e-16f64 {
        1.0f64
    } else {
        sinh(sigma) / sigma
    };
    let sinhalf: libc::c_double = if fabs(alpha) < 2.2204460492503131e-16f64 {
        1.0f64
    } else {
        sin(alpha) / alpha
    };
    let sin_sqr: libc::c_double = nu * 3.14159265358979323846f64
        * 3.14159265358979323846f64 * 0.5f64 * sinhalf * sinhalf;
    let mut sum0: libc::c_double = 0.;
    let mut sum1: libc::c_double = 0.;
    let mut fk: libc::c_double = 0.;
    let mut pk: libc::c_double = 0.;
    let mut qk: libc::c_double = 0.;
    let mut hk: libc::c_double = 0.;
    let mut ck: libc::c_double = 0.;
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut stat_iter: libc::c_int = 0;
    let mut g_1pnu: libc::c_double = 0.;
    let mut g_1mnu: libc::c_double = 0.;
    let mut g1: libc::c_double = 0.;
    let mut g2: libc::c_double = 0.;
    let mut stat_g: libc::c_int = gsl_sf_temme_gamma(
        nu,
        &mut g_1pnu,
        &mut g_1mnu,
        &mut g1,
        &mut g2,
    );
    fk = 2.0f64 / 3.14159265358979323846f64 * sinrat
        * (cosh(sigma) * g1 - sinhrat * ln_half_x * g2);
    pk = 1.0f64 / 3.14159265358979323846f64 / half_x_nu * g_1pnu;
    qk = 1.0f64 / 3.14159265358979323846f64 * half_x_nu * g_1mnu;
    hk = pk;
    ck = 1.0f64;
    sum0 = fk + sin_sqr * qk;
    sum1 = pk;
    while k < max_iter {
        let mut del0: libc::c_double = 0.;
        let mut del1: libc::c_double = 0.;
        let mut gk: libc::c_double = 0.;
        k += 1;
        k;
        fk = (k as libc::c_double * fk + pk + qk)
            / ((k * k) as libc::c_double - nu * nu);
        ck *= -half_x * half_x / k as libc::c_double;
        pk /= k as libc::c_double - nu;
        qk /= k as libc::c_double + nu;
        gk = fk + sin_sqr * qk;
        hk = -k as libc::c_double * gk + pk;
        del0 = ck * gk;
        del1 = ck * hk;
        sum0 += del0;
        sum1 += del1;
        if fabs(del0) < 0.5f64 * (1.0f64 + fabs(sum0)) * 2.2204460492503131e-16f64 {
            break;
        }
    }
    (*Ynu).val = -sum0;
    (*Ynu)
        .err = (2.0f64 + 0.5f64 * k as libc::c_double) * 2.2204460492503131e-16f64
        * fabs((*Ynu).val);
    (*Ynup1).val = -sum1 * 2.0f64 / x;
    (*Ynup1)
        .err = (2.0f64 + 0.5f64 * k as libc::c_double) * 2.2204460492503131e-16f64
        * fabs((*Ynup1).val);
    stat_iter = if k >= max_iter {
        GSL_EMAXITER as libc::c_int
    } else {
        GSL_SUCCESS as libc::c_int
    };
    return if stat_iter != GSL_SUCCESS as libc::c_int {
        stat_iter
    } else if stat_g != GSL_SUCCESS as libc::c_int {
        stat_g
    } else {
        GSL_SUCCESS as libc::c_int
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_K_scaled_temme(
    nu: libc::c_double,
    x: libc::c_double,
    mut K_nu: *mut libc::c_double,
    mut K_nup1: *mut libc::c_double,
    mut Kp_nu: *mut libc::c_double,
) -> libc::c_int {
    let max_iter: libc::c_int = 15000 as libc::c_int;
    let half_x: libc::c_double = 0.5f64 * x;
    let ln_half_x: libc::c_double = log(half_x);
    let half_x_nu: libc::c_double = exp(nu * ln_half_x);
    let pi_nu: libc::c_double = 3.14159265358979323846f64 * nu;
    let sigma: libc::c_double = -nu * ln_half_x;
    let sinrat: libc::c_double = if fabs(pi_nu) < 2.2204460492503131e-16f64 {
        1.0f64
    } else {
        pi_nu / sin(pi_nu)
    };
    let sinhrat: libc::c_double = if fabs(sigma) < 2.2204460492503131e-16f64 {
        1.0f64
    } else {
        sinh(sigma) / sigma
    };
    let ex: libc::c_double = exp(x);
    let mut sum0: libc::c_double = 0.;
    let mut sum1: libc::c_double = 0.;
    let mut fk: libc::c_double = 0.;
    let mut pk: libc::c_double = 0.;
    let mut qk: libc::c_double = 0.;
    let mut hk: libc::c_double = 0.;
    let mut ck: libc::c_double = 0.;
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut stat_iter: libc::c_int = 0;
    let mut g_1pnu: libc::c_double = 0.;
    let mut g_1mnu: libc::c_double = 0.;
    let mut g1: libc::c_double = 0.;
    let mut g2: libc::c_double = 0.;
    let mut stat_g: libc::c_int = gsl_sf_temme_gamma(
        nu,
        &mut g_1pnu,
        &mut g_1mnu,
        &mut g1,
        &mut g2,
    );
    fk = sinrat * (cosh(sigma) * g1 - sinhrat * ln_half_x * g2);
    pk = 0.5f64 / half_x_nu * g_1pnu;
    qk = 0.5f64 * half_x_nu * g_1mnu;
    hk = pk;
    ck = 1.0f64;
    sum0 = fk;
    sum1 = hk;
    while k < max_iter {
        let mut del0: libc::c_double = 0.;
        let mut del1: libc::c_double = 0.;
        k += 1;
        k;
        fk = (k as libc::c_double * fk + pk + qk)
            / ((k * k) as libc::c_double - nu * nu);
        ck *= half_x * half_x / k as libc::c_double;
        pk /= k as libc::c_double - nu;
        qk /= k as libc::c_double + nu;
        hk = -k as libc::c_double * fk + pk;
        del0 = ck * fk;
        del1 = ck * hk;
        sum0 += del0;
        sum1 += del1;
        if fabs(del0) < 0.5f64 * fabs(sum0) * 2.2204460492503131e-16f64 {
            break;
        }
    }
    *K_nu = sum0 * ex;
    *K_nup1 = sum1 * 2.0f64 / x * ex;
    *Kp_nu = -*K_nup1 + nu / x * *K_nu;
    stat_iter = if k == max_iter {
        GSL_EMAXITER as libc::c_int
    } else {
        GSL_SUCCESS as libc::c_int
    };
    return if stat_iter != GSL_SUCCESS as libc::c_int {
        stat_iter
    } else if stat_g != GSL_SUCCESS as libc::c_int {
        stat_g
    } else {
        GSL_SUCCESS as libc::c_int
    };
}
