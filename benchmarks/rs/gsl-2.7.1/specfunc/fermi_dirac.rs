use ::libc;
extern "C" {
    fn cos(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_exp_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_exp_mult_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        y: libc::c_double,
        dy: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_lngamma_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_fact_e(n: libc::c_uint, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_hyperg_1F1_int_e(
        m: libc::c_int,
        n: libc::c_int,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_hyperg_U_int_e(
        m: libc::c_int,
        n: libc::c_int,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_pow_int(x: libc::c_double, n: libc::c_int) -> libc::c_double;
    fn gsl_sf_eta_int_e(n: libc::c_int, result: *mut gsl_sf_result) -> libc::c_int;
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
pub const itmax: C2RustUnnamed_0 = 100;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const qsize: C2RustUnnamed_0 = 101;
pub const nsize: C2RustUnnamed_1 = 101;
pub type C2RustUnnamed_1 = libc::c_uint;
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
static mut fd_1_a_data: [libc::c_double; 22] = [
    1.8949340668482264365f64,
    0.7237719066890052793f64,
    0.1250000000000000000f64,
    0.0101065196435973942f64,
    0.0f64,
    -0.0000600615242174119f64,
    0.0f64,
    6.816528764623e-7f64,
    0.0f64,
    -9.5895779195e-9f64,
    0.0f64,
    1.515104135e-10f64,
    0.0f64,
    -2.5785616e-12f64,
    0.0f64,
    4.62270e-14f64,
    0.0f64,
    -8.612e-16f64,
    0.0f64,
    1.65e-17f64,
    0.0f64,
    -3.0e-19f64,
];
static mut fd_1_a_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: fd_1_a_data.as_ptr() as *mut _,
            order: 21 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 12 as libc::c_int,
        };
        init
    }
};
static mut fd_1_b_data: [libc::c_double; 22] = [
    10.409136795234611872f64,
    3.899445098225161947f64,
    0.513510935510521222f64,
    0.010618736770218426f64,
    -0.001584468020659694f64,
    0.000146139297161640f64,
    -1.408095734499e-6f64,
    -2.177993899484e-6f64,
    3.91423660640e-7f64,
    -2.3860262660e-8f64,
    -4.138309573e-9f64,
    1.283965236e-9f64,
    -1.39695990e-10f64,
    -4.907743e-12f64,
    4.399878e-12f64,
    -7.17291e-13f64,
    2.4320e-14f64,
    1.4230e-14f64,
    -3.446e-15f64,
    2.93e-16f64,
    3.7e-17f64,
    -1.6e-17f64,
];
static mut fd_1_b_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: fd_1_b_data.as_ptr() as *mut _,
            order: 21 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 11 as libc::c_int,
        };
        init
    }
};
static mut fd_1_c_data: [libc::c_double; 23] = [
    56.78099449124299762f64,
    21.00718468237668011f64,
    2.24592457063193457f64,
    0.00173793640425994f64,
    -0.00058716468739423f64,
    0.00016306958492437f64,
    -0.00003817425583020f64,
    7.64527252009e-6f64,
    -1.31348500162e-6f64,
    1.9000646056e-7f64,
    -2.141328223e-8f64,
    1.23906372e-9f64,
    2.1848049e-10f64,
    -1.0134282e-10f64,
    2.484728e-11f64,
    -4.73067e-12f64,
    7.3555e-13f64,
    -8.740e-14f64,
    4.85e-15f64,
    1.23e-15f64,
    -5.6e-16f64,
    1.4e-16f64,
    -3.0e-17f64,
];
static mut fd_1_c_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: fd_1_c_data.as_ptr() as *mut _,
            order: 22 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 13 as libc::c_int,
        };
        init
    }
};
static mut fd_1_d_data: [libc::c_double; 30] = [
    1.0126626021151374442f64,
    -0.0063312525536433793f64,
    0.0024837319237084326f64,
    -0.0008764333697726109f64,
    0.0002913344438921266f64,
    -0.0000931877907705692f64,
    0.0000290151342040275f64,
    -8.8548707259955e-6f64,
    2.6603474114517e-6f64,
    -7.891415690452e-7f64,
    2.315730237195e-7f64,
    -6.73179452963e-8f64,
    1.94048035606e-8f64,
    -5.5507129189e-9f64,
    1.5766090896e-9f64,
    -4.449310875e-10f64,
    1.248292745e-10f64,
    -3.48392894e-11f64,
    9.6791550e-12f64,
    -2.6786240e-12f64,
    7.388852e-13f64,
    -2.032828e-13f64,
    5.58115e-14f64,
    -1.52987e-14f64,
    4.1886e-15f64,
    -1.1458e-15f64,
    3.132e-16f64,
    -8.56e-17f64,
    2.33e-17f64,
    -5.9e-18f64,
];
static mut fd_1_d_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: fd_1_d_data.as_ptr() as *mut _,
            order: 29 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 14 as libc::c_int,
        };
        init
    }
};
static mut fd_1_e_data: [libc::c_double; 10] = [
    1.0013707783890401683f64,
    0.0009138522593601060f64,
    0.0002284630648400133f64,
    -1.57e-17f64,
    -1.27e-17f64,
    -9.7e-18f64,
    -6.9e-18f64,
    -4.6e-18f64,
    -2.9e-18f64,
    -1.7e-18f64,
];
static mut fd_1_e_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: fd_1_e_data.as_ptr() as *mut _,
            order: 9 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 4 as libc::c_int,
        };
        init
    }
};
static mut fd_2_a_data: [libc::c_double; 21] = [
    2.1573661917148458336f64,
    0.8849670334241132182f64,
    0.1784163467613519713f64,
    0.0208333333333333333f64,
    0.0012708226459768508f64,
    0.0f64,
    -5.0619314244895e-6f64,
    0.0f64,
    4.32026533989e-8f64,
    0.0f64,
    -4.870544166e-10f64,
    0.0f64,
    6.4203740e-12f64,
    0.0f64,
    -9.37424e-14f64,
    0.0f64,
    1.4715e-15f64,
    0.0f64,
    -2.44e-17f64,
    0.0f64,
    4.0e-19f64,
];
static mut fd_2_a_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: fd_2_a_data.as_ptr() as *mut _,
            order: 20 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 12 as libc::c_int,
        };
        init
    }
};
static mut fd_2_b_data: [libc::c_double; 22] = [
    16.508258811798623599f64,
    7.421719394793067988f64,
    1.458309885545603821f64,
    0.128773850882795229f64,
    0.001963612026198147f64,
    -0.000237458988738779f64,
    0.000018539661382641f64,
    -1.92805649479e-7f64,
    -2.01950028452e-7f64,
    3.2963497518e-8f64,
    -1.885817092e-9f64,
    -2.72632744e-10f64,
    8.0554561e-11f64,
    -8.313223e-12f64,
    -2.24489e-13f64,
    2.18778e-13f64,
    -3.4290e-14f64,
    1.225e-15f64,
    5.81e-16f64,
    -1.37e-16f64,
    1.2e-17f64,
    1.0e-18f64,
];
static mut fd_2_b_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: fd_2_b_data.as_ptr() as *mut _,
            order: 21 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 12 as libc::c_int,
        };
        init
    }
};
static mut fd_2_c_data: [libc::c_double; 20] = [
    168.87129776686440711f64,
    81.80260488091659458f64,
    15.75408505947931513f64,
    1.12325586765966440f64,
    0.00059057505725084f64,
    -0.00016469712946921f64,
    0.00003885607810107f64,
    -7.89873660613e-6f64,
    1.39786238616e-6f64,
    -2.1534528656e-7f64,
    2.831510953e-8f64,
    -2.94978583e-9f64,
    1.6755082e-10f64,
    2.234229e-11f64,
    -1.035130e-11f64,
    2.41117e-12f64,
    -4.3531e-13f64,
    6.447e-14f64,
    -7.39e-15f64,
    4.3e-16f64,
];
static mut fd_2_c_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: fd_2_c_data.as_ptr() as *mut _,
            order: 19 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 12 as libc::c_int,
        };
        init
    }
};
static mut fd_2_d_data: [libc::c_double; 30] = [
    0.3459960518965277589f64,
    -0.00633136397691958024f64,
    0.00248382959047594408f64,
    -0.00087651191884005114f64,
    0.00029139255351719932f64,
    -0.00009322746111846199f64,
    0.00002904021914564786f64,
    -8.86962264810663e-6f64,
    2.66844972574613e-6f64,
    -7.9331564996004e-7f64,
    2.3359868615516e-7f64,
    -6.824790880436e-8f64,
    1.981036528154e-8f64,
    -5.71940426300e-9f64,
    1.64379426579e-9f64,
    -4.7064937566e-10f64,
    1.3432614122e-10f64,
    -3.823400534e-11f64,
    1.085771994e-11f64,
    -3.07727465e-12f64,
    8.7064848e-13f64,
    -2.4595431e-13f64,
    6.938531e-14f64,
    -1.954939e-14f64,
    5.50162e-15f64,
    -1.54657e-15f64,
    4.3429e-16f64,
    -1.2178e-16f64,
    3.394e-17f64,
    -8.81e-18f64,
];
static mut fd_2_d_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: fd_2_d_data.as_ptr() as *mut _,
            order: 29 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 14 as libc::c_int,
        };
        init
    }
};
static mut fd_2_e_data: [libc::c_double; 4] = [
    0.3347041117223735227f64,
    0.00091385225936012645f64,
    0.00022846306484003205f64,
    5.2e-19f64,
];
static mut fd_2_e_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: fd_2_e_data.as_ptr() as *mut _,
            order: 3 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 3 as libc::c_int,
        };
        init
    }
};
static mut fd_mhalf_a_data: [libc::c_double; 20] = [
    1.2663290042859741974f64,
    0.3697876251911153071f64,
    0.0278131011214405055f64,
    -0.0033332848565672007f64,
    -0.0004438108265412038f64,
    0.0000616495177243839f64,
    8.7589611449897e-6f64,
    -1.2622936986172e-6f64,
    -1.837464037221e-7f64,
    2.69495091400e-8f64,
    3.9760866257e-9f64,
    -5.894468795e-10f64,
    -8.77321638e-11f64,
    1.31016571e-11f64,
    1.9621619e-12f64,
    -2.945887e-13f64,
    -4.43234e-14f64,
    6.6816e-15f64,
    1.0084e-15f64,
    -1.561e-16f64,
];
static mut fd_mhalf_a_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: fd_mhalf_a_data.as_ptr() as *mut _,
            order: 19 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 12 as libc::c_int,
        };
        init
    }
};
static mut fd_mhalf_b_data: [libc::c_double; 20] = [
    3.270796131942071484f64,
    0.5809004935853417887f64,
    -0.0299313438794694987f64,
    -0.0013287935412612198f64,
    0.0009910221228704198f64,
    -0.0001690954939688554f64,
    6.5955849946915e-6f64,
    3.5953966033618e-6f64,
    -9.430672023181e-7f64,
    8.75773958291e-8f64,
    1.06247652607e-8f64,
    -4.9587006215e-9f64,
    7.160432795e-10f64,
    4.5072219e-12f64,
    -2.3695425e-11f64,
    4.9122208e-12f64,
    -2.905277e-13f64,
    -9.59291e-14f64,
    3.00028e-14f64,
    -3.4970e-15f64,
];
static mut fd_mhalf_b_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: fd_mhalf_b_data.as_ptr() as *mut _,
            order: 19 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 12 as libc::c_int,
        };
        init
    }
};
static mut fd_mhalf_c_data: [libc::c_double; 25] = [
    5.828283273430595507f64,
    0.677521118293264655f64,
    -0.043946248736481554f64,
    0.005825595781828244f64,
    -0.000864858907380668f64,
    0.000110017890076539f64,
    -6.973305225404e-6f64,
    -1.716267414672e-6f64,
    8.59811582041e-7f64,
    -2.33066786976e-7f64,
    4.8503191159e-8f64,
    -8.130620247e-9f64,
    1.021068250e-9f64,
    -5.3188423e-11f64,
    -1.9430559e-11f64,
    8.750506e-12f64,
    -2.324897e-12f64,
    4.83102e-13f64,
    -8.1207e-14f64,
    1.0132e-14f64,
    -4.64e-16f64,
    -2.24e-16f64,
    9.7e-17f64,
    -2.6e-17f64,
    5.0e-18f64,
];
static mut fd_mhalf_c_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: fd_mhalf_c_data.as_ptr() as *mut _,
            order: 24 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 13 as libc::c_int,
        };
        init
    }
};
static mut fd_mhalf_d_data: [libc::c_double; 30] = [
    2.2530744202862438709f64,
    0.0018745152720114692f64,
    -0.0007550198497498903f64,
    0.0002759818676644382f64,
    -0.0000959406283465913f64,
    0.0000324056855537065f64,
    -0.0000107462396145761f64,
    3.5126865219224e-6f64,
    -1.1313072730092e-6f64,
    3.577454162766e-7f64,
    -1.104926666238e-7f64,
    3.31304165692e-8f64,
    -9.5837381008e-9f64,
    2.6575790141e-9f64,
    -7.015201447e-10f64,
    1.747111336e-10f64,
    -4.04909605e-11f64,
    8.5104999e-12f64,
    -1.5261885e-12f64,
    1.876851e-13f64,
    1.00574e-14f64,
    -1.82002e-14f64,
    8.6634e-15f64,
    -3.2058e-15f64,
    1.0572e-15f64,
    -3.259e-16f64,
    9.60e-17f64,
    -2.74e-17f64,
    7.6e-18f64,
    -1.9e-18f64,
];
static mut fd_mhalf_d_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: fd_mhalf_d_data.as_ptr() as *mut _,
            order: 29 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 15 as libc::c_int,
        };
        init
    }
};
static mut fd_half_a_data: [libc::c_double; 23] = [
    1.7177138871306189157f64,
    0.6192579515822668460f64,
    0.0932802275119206269f64,
    0.0047094853246636182f64,
    -0.0004243667967864481f64,
    -0.0000452569787686193f64,
    5.2426509519168e-6f64,
    6.387648249080e-7f64,
    -8.05777004848e-8f64,
    -1.04290272415e-8f64,
    1.3769478010e-9f64,
    1.847190359e-10f64,
    -2.51061890e-11f64,
    -3.4497818e-12f64,
    4.784373e-13f64,
    6.68828e-14f64,
    -9.4147e-15f64,
    -1.3333e-15f64,
    1.898e-16f64,
    2.72e-17f64,
    -3.9e-18f64,
    -6.0e-19f64,
    1.0e-19f64,
];
static mut fd_half_a_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: fd_half_a_data.as_ptr() as *mut _,
            order: 22 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 11 as libc::c_int,
        };
        init
    }
};
static mut fd_half_b_data: [libc::c_double; 20] = [
    7.651013792074984027f64,
    2.475545606866155737f64,
    0.218335982672476128f64,
    -0.007730591500584980f64,
    -0.000217443383867318f64,
    0.000147663980681359f64,
    -0.000021586361321527f64,
    8.07712735394e-7f64,
    3.28858050706e-7f64,
    -7.9474330632e-8f64,
    6.940207234e-9f64,
    6.75594681e-10f64,
    -3.10200490e-10f64,
    4.2677233e-11f64,
    -2.1696e-14f64,
    -1.170245e-12f64,
    2.34757e-13f64,
    -1.4139e-14f64,
    -3.864e-15f64,
    1.202e-15f64,
];
static mut fd_half_b_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: fd_half_b_data.as_ptr() as *mut _,
            order: 19 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 12 as libc::c_int,
        };
        init
    }
};
static mut fd_half_c_data: [libc::c_double; 23] = [
    29.584339348839816528f64,
    8.808344283250615592f64,
    0.503771641883577308f64,
    -0.021540694914550443f64,
    0.002143341709406890f64,
    -0.000257365680646579f64,
    0.000027933539372803f64,
    -1.678525030167e-6f64,
    -2.78100117693e-7f64,
    1.35218065147e-7f64,
    -3.3740425009e-8f64,
    6.474834942e-9f64,
    -1.009678978e-9f64,
    1.20057555e-10f64,
    -6.636314e-12f64,
    -1.710566e-12f64,
    7.75069e-13f64,
    -1.97973e-13f64,
    3.9414e-14f64,
    -6.374e-15f64,
    7.77e-16f64,
    -4.0e-17f64,
    -1.4e-17f64,
];
static mut fd_half_c_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: fd_half_c_data.as_ptr() as *mut _,
            order: 22 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 13 as libc::c_int,
        };
        init
    }
};
static mut fd_half_d_data: [libc::c_double; 30] = [
    1.5116909434145508537f64,
    -0.0036043405371630468f64,
    0.0014207743256393359f64,
    -0.0005045399052400260f64,
    0.0001690758006957347f64,
    -0.0000546305872688307f64,
    0.0000172223228484571f64,
    -5.3352603788706e-6f64,
    1.6315287543662e-6f64,
    -4.939021084898e-7f64,
    1.482515450316e-7f64,
    -4.41552276226e-8f64,
    1.30503160961e-8f64,
    -3.8262599802e-9f64,
    1.1123226976e-9f64,
    -3.204765534e-10f64,
    9.14870489e-11f64,
    -2.58778946e-11f64,
    7.2550731e-12f64,
    -2.0172226e-12f64,
    5.566891e-13f64,
    -1.526247e-13f64,
    4.16121e-14f64,
    -1.12933e-14f64,
    3.0537e-15f64,
    -8.234e-16f64,
    2.215e-16f64,
    -5.95e-17f64,
    1.59e-17f64,
    -4.0e-18f64,
];
static mut fd_half_d_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: fd_half_d_data.as_ptr() as *mut _,
            order: 29 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 15 as libc::c_int,
        };
        init
    }
};
static mut fd_3half_a_data: [libc::c_double; 20] = [
    2.0404775940601704976f64,
    0.8122168298093491444f64,
    0.1536371165644008069f64,
    0.0156174323847845125f64,
    0.0005943427879290297f64,
    -0.0000429609447738365f64,
    -3.8246452994606e-6f64,
    3.802306180287e-7f64,
    4.05746157593e-8f64,
    -4.5530360159e-9f64,
    -5.306873139e-10f64,
    6.37297268e-11f64,
    7.8403674e-12f64,
    -9.840241e-13f64,
    -1.255952e-13f64,
    1.62617e-14f64,
    2.1318e-15f64,
    -2.825e-16f64,
    -3.78e-17f64,
    5.1e-18f64,
];
static mut fd_3half_a_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: fd_3half_a_data.as_ptr() as *mut _,
            order: 19 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 11 as libc::c_int,
        };
        init
    }
};
static mut fd_3half_b_data: [libc::c_double; 22] = [
    13.403206654624176674f64,
    5.574508357051880924f64,
    0.931228574387527769f64,
    0.054638356514085862f64,
    -0.001477172902737439f64,
    -0.000029378553381869f64,
    0.000018357033493246f64,
    -2.348059218454e-6f64,
    8.3173787440e-8f64,
    2.6826486956e-8f64,
    -6.011244398e-9f64,
    4.94345981e-10f64,
    3.9557340e-11f64,
    -1.7894930e-11f64,
    2.348972e-12f64,
    -1.2823e-14f64,
    -5.4192e-14f64,
    1.0527e-14f64,
    -6.39e-16f64,
    -1.47e-16f64,
    4.5e-17f64,
    -5.0e-18f64,
];
static mut fd_3half_b_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: fd_3half_b_data.as_ptr() as *mut _,
            order: 21 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 12 as libc::c_int,
        };
        init
    }
};
static mut fd_3half_c_data: [libc::c_double; 21] = [
    101.03685253378877642f64,
    43.62085156043435883f64,
    6.62241373362387453f64,
    0.25081415008708521f64,
    -0.00798124846271395f64,
    0.00063462245101023f64,
    -0.00006392178890410f64,
    6.04535131939e-6f64,
    -3.4007683037e-7f64,
    -4.072661545e-8f64,
    1.931148453e-8f64,
    -4.46328355e-9f64,
    7.9434717e-10f64,
    -1.1573569e-10f64,
    1.304658e-11f64,
    -7.4114e-13f64,
    -1.4181e-13f64,
    6.491e-14f64,
    -1.597e-14f64,
    3.05e-15f64,
    -4.8e-16f64,
];
static mut fd_3half_c_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: fd_3half_c_data.as_ptr() as *mut _,
            order: 20 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 12 as libc::c_int,
        };
        init
    }
};
static mut fd_3half_d_data: [libc::c_double; 25] = [
    0.6160645215171852381f64,
    -0.0071239478492671463f64,
    0.0027906866139659846f64,
    -0.0009829521424317718f64,
    0.0003260229808519545f64,
    -0.0001040160912910890f64,
    0.0000322931223232439f64,
    -9.8243506588102e-6f64,
    2.9420132351277e-6f64,
    -8.699154670418e-7f64,
    2.545460071999e-7f64,
    -7.38305056331e-8f64,
    2.12545670310e-8f64,
    -6.0796532462e-9f64,
    1.7294556741e-9f64,
    -4.896540687e-10f64,
    1.380786037e-10f64,
    -3.88057305e-11f64,
    1.08753212e-11f64,
    -3.0407308e-12f64,
    8.485626e-13f64,
    -2.364275e-13f64,
    6.57636e-14f64,
    -1.81807e-14f64,
    4.6884e-15f64,
];
static mut fd_3half_d_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: fd_3half_d_data.as_ptr() as *mut _,
            order: 24 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 16 as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn fd_whiz(
    term: libc::c_double,
    iterm: libc::c_int,
    mut qnum: *mut libc::c_double,
    mut qden: *mut libc::c_double,
    mut result: *mut libc::c_double,
    mut s: *mut libc::c_double,
) -> libc::c_int {
    if iterm == 0 as libc::c_int {
        *s = 0.0f64;
    }
    *s += term;
    *qden
        .offset(
            iterm as isize,
        ) = 1.0f64
        / (term * (iterm as libc::c_double + 1.0f64)
            * (iterm as libc::c_double + 1.0f64));
    *qnum.offset(iterm as isize) = *s * *qden.offset(iterm as isize);
    if iterm > 0 as libc::c_int {
        let mut factor: libc::c_double = 1.0f64;
        let mut ratio: libc::c_double = iterm as libc::c_double
            / (iterm as libc::c_double + 1.0f64);
        let mut j: libc::c_int = 0;
        j = iterm - 1 as libc::c_int;
        while j >= 0 as libc::c_int {
            let mut c: libc::c_double = factor * (j as libc::c_double + 1.0f64)
                / (iterm as libc::c_double + 1.0f64);
            factor *= ratio;
            *qden
                .offset(
                    j as isize,
                ) = *qden.offset((j + 1 as libc::c_int) as isize)
                - c * *qden.offset(j as isize);
            *qnum
                .offset(
                    j as isize,
                ) = *qnum.offset((j + 1 as libc::c_int) as isize)
                - c * *qnum.offset(j as isize);
            j -= 1;
            j;
        }
    }
    *result = *qnum.offset(0 as libc::c_int as isize)
        / *qden.offset(0 as libc::c_int as isize);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn fd_nint(
    j: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut qcoeff: [libc::c_double; 101] = [0.; 101];
    if j >= -(1 as libc::c_int) {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"fermi_dirac.c\0" as *const u8 as *const libc::c_char,
            862 as libc::c_int,
            GSL_ESANITY as libc::c_int,
        );
        return GSL_ESANITY as libc::c_int;
    } else if j < -(nsize as libc::c_int) {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"fermi_dirac.c\0" as *const u8 as *const libc::c_char,
            867 as libc::c_int,
            GSL_EUNIMPL as libc::c_int,
        );
        return GSL_EUNIMPL as libc::c_int;
    } else {
        let mut a: libc::c_double = 0.;
        let mut p: libc::c_double = 0.;
        let mut f: libc::c_double = 0.;
        let mut i: libc::c_int = 0;
        let mut k: libc::c_int = 0;
        let mut n: libc::c_int = -(j + 1 as libc::c_int);
        qcoeff[1 as libc::c_int as usize] = 1.0f64;
        k = 2 as libc::c_int;
        while k <= n {
            qcoeff[k as usize] = -qcoeff[(k - 1 as libc::c_int) as usize];
            i = k - 1 as libc::c_int;
            while i >= 2 as libc::c_int {
                qcoeff[i
                    as usize] = i as libc::c_double * qcoeff[i as usize]
                    - (k - (i - 1 as libc::c_int)) as libc::c_double
                        * qcoeff[(i - 1 as libc::c_int) as usize];
                i -= 1;
                i;
            }
            k += 1;
            k;
        }
        if x >= 0.0f64 {
            a = exp(-x);
            f = qcoeff[1 as libc::c_int as usize];
            i = 2 as libc::c_int;
            while i <= n {
                f = f * a + qcoeff[i as usize];
                i += 1;
                i;
            }
        } else {
            a = exp(x);
            f = qcoeff[n as usize];
            i = n - 1 as libc::c_int;
            while i >= 1 as libc::c_int {
                f = f * a + qcoeff[i as usize];
                i -= 1;
                i;
            }
        }
        p = gsl_sf_pow_int(1.0f64 + a, j);
        (*result).val = f * a * p;
        (*result).err = 3.0f64 * 2.2204460492503131e-16f64 * fabs(f * a * p);
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn fd_neg(
    j: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut qnum: [libc::c_double; 101] = [0.; 101];
    let mut qden: [libc::c_double; 101] = [0.; 101];
    if x < -7.0839641853226408e+02f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x < -1.0f64 && x < -fabs(j + 1.0f64) {
        let mut ex: libc::c_double = exp(x);
        let mut term: libc::c_double = ex;
        let mut sum: libc::c_double = term;
        let mut n: libc::c_int = 0;
        n = 2 as libc::c_int;
        while n < 100 as libc::c_int {
            let mut rat: libc::c_double = (n as libc::c_double - 1.0f64)
                / n as libc::c_double;
            let mut p: libc::c_double = pow(rat, j + 1.0f64);
            term *= -ex * p;
            sum += term;
            if fabs(term / sum) < 2.2204460492503131e-16f64 {
                break;
            }
            n += 1;
            n;
        }
        (*result).val = sum;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs(sum);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut s: libc::c_double = 0.0f64;
        let mut xn: libc::c_double = x;
        let mut ex_0: libc::c_double = -exp(x);
        let mut enx: libc::c_double = -ex_0;
        let mut f: libc::c_double = 0.0f64;
        let mut f_previous: libc::c_double = 0.;
        let mut jterm: libc::c_int = 0;
        jterm = 0 as libc::c_int;
        while jterm <= itmax as libc::c_int {
            let mut p_0: libc::c_double = pow(
                jterm as libc::c_double + 1.0f64,
                j + 1.0f64,
            );
            let mut term_0: libc::c_double = enx / p_0;
            f_previous = f;
            fd_whiz(term_0, jterm, qnum.as_mut_ptr(), qden.as_mut_ptr(), &mut f, &mut s);
            xn += x;
            if fabs(f - f_previous) < fabs(f) * 2.0f64 * 2.2204460492503131e-16f64
                || xn < -7.0839641853226408e+02f64
            {
                break;
            }
            enx *= ex_0;
            jterm += 1;
            jterm;
        }
        (*result).val = f;
        (*result).err = fabs(f - f_previous);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs(f);
        if jterm == itmax as libc::c_int {
            gsl_error(
                b"error\0" as *const u8 as *const libc::c_char,
                b"fermi_dirac.c\0" as *const u8 as *const libc::c_char,
                968 as libc::c_int,
                GSL_EMAXITER as libc::c_int,
            );
            return GSL_EMAXITER as libc::c_int;
        } else {
            return GSL_SUCCESS as libc::c_int
        }
    };
}
unsafe extern "C" fn fd_asymp(
    j: libc::c_double,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let j_integer: libc::c_int = (fabs(j - floor(j + 0.5f64))
        < 100.0f64 * 2.2204460492503131e-16f64) as libc::c_int;
    let itmax_0: libc::c_int = 200 as libc::c_int;
    let mut lg: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_lg: libc::c_int = gsl_sf_lngamma_e(j + 2.0f64, &mut lg);
    let mut seqn_val: libc::c_double = 0.5f64;
    let mut seqn_err: libc::c_double = 0.0f64;
    let mut xm2: libc::c_double = 1.0f64 / x / x;
    let mut xgam: libc::c_double = 1.0f64;
    let mut add: libc::c_double = 1.7976931348623157e+308f64;
    let mut cos_term: libc::c_double = 0.;
    let mut ln_x: libc::c_double = 0.;
    let mut ex_term_1: libc::c_double = 0.;
    let mut ex_term_2: libc::c_double = 0.;
    let mut fneg: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut ex_arg: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut ex: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_fneg: libc::c_int = 0;
    let mut stat_e: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    n = 1 as libc::c_int;
    while n <= itmax_0 {
        let mut add_previous: libc::c_double = add;
        let mut eta: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        gsl_sf_eta_int_e(2 as libc::c_int * n, &mut eta);
        xgam = xgam * xm2
            * (j + 1.0f64 - (2 as libc::c_int * n - 2 as libc::c_int) as libc::c_double)
            * (j + 1.0f64 - (2 as libc::c_int * n - 1 as libc::c_int) as libc::c_double);
        add = eta.val * xgam;
        if j_integer == 0 && fabs(add) > fabs(add_previous) {
            break;
        }
        if fabs(add / seqn_val) < 2.2204460492503131e-16f64 {
            break;
        }
        seqn_val += add;
        seqn_err += 2.0f64 * 2.2204460492503131e-16f64 * fabs(add);
        n += 1;
        n;
    }
    seqn_err += fabs(add);
    stat_fneg = fd_neg(j, -x, &mut fneg);
    ln_x = log(x);
    ex_term_1 = (j + 1.0f64) * ln_x;
    ex_term_2 = lg.val;
    ex_arg.val = ex_term_1 - ex_term_2;
    ex_arg
        .err = 2.2204460492503131e-16f64 * (fabs(ex_term_1) + fabs(ex_term_2)) + lg.err;
    stat_e = gsl_sf_exp_err_e(ex_arg.val, ex_arg.err, &mut ex);
    cos_term = cos(j * 3.14159265358979323846f64);
    (*result).val = cos_term * fneg.val + 2.0f64 * seqn_val * ex.val;
    (*result).err = fabs(2.0f64 * ex.err * seqn_val);
    (*result).err += fabs(2.0f64 * ex.val * seqn_err);
    (*result).err += fabs(cos_term) * fneg.err;
    (*result).err += 4.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
    return if stat_e != GSL_SUCCESS as libc::c_int {
        stat_e
    } else if stat_fneg != GSL_SUCCESS as libc::c_int {
        stat_fneg
    } else if stat_lg != GSL_SUCCESS as libc::c_int {
        stat_lg
    } else {
        GSL_SUCCESS as libc::c_int
    };
}
unsafe extern "C" fn fd_series_int(
    j: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut sum: libc::c_double = 0.0f64;
    let mut del: libc::c_double = 0.;
    let mut pow_factor: libc::c_double = 1.0f64;
    let mut eta_factor: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    gsl_sf_eta_int_e(j + 1 as libc::c_int, &mut eta_factor);
    del = pow_factor * eta_factor.val;
    sum += del;
    n = 1 as libc::c_int;
    while n <= j + 2 as libc::c_int {
        gsl_sf_eta_int_e(j + 1 as libc::c_int - n, &mut eta_factor);
        pow_factor *= x / n as libc::c_double;
        del = pow_factor * eta_factor.val;
        sum += del;
        if fabs(del / sum) < 2.2204460492503131e-16f64 {
            break;
        }
        n += 1;
        n;
    }
    if j < 32 as libc::c_int {
        let mut m: libc::c_int = 0;
        let mut jfact: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sum2: libc::c_double = 0.;
        let mut pre2: libc::c_double = 0.;
        gsl_sf_fact_e(j as libc::c_uint, &mut jfact);
        pre2 = gsl_sf_pow_int(x, j) / jfact.val;
        gsl_sf_eta_int_e(-(3 as libc::c_int), &mut eta_factor);
        pow_factor = x * x * x * x
            / ((j + 4 as libc::c_int) * (j + 3 as libc::c_int) * (j + 2 as libc::c_int)
                * (j + 1 as libc::c_int)) as libc::c_double;
        sum2 = eta_factor.val * pow_factor;
        m = 3 as libc::c_int;
        while m < 24 as libc::c_int {
            gsl_sf_eta_int_e(1 as libc::c_int - 2 as libc::c_int * m, &mut eta_factor);
            pow_factor
                *= x * x
                    / ((j + 2 as libc::c_int * m)
                        * (j + 2 as libc::c_int * m - 1 as libc::c_int))
                        as libc::c_double;
            sum2 += eta_factor.val * pow_factor;
            m += 1;
            m;
        }
        sum += pre2 * sum2;
    }
    (*result).val = sum;
    (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs(sum);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn fd_UMseries_int(
    j: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let nmax: libc::c_int = 2000 as libc::c_int;
    let mut pre: libc::c_double = 0.;
    let mut lnpre_val: libc::c_double = 0.;
    let mut lnpre_err: libc::c_double = 0.;
    let mut sum_even_val: libc::c_double = 1.0f64;
    let mut sum_even_err: libc::c_double = 0.0f64;
    let mut sum_odd_val: libc::c_double = 0.0f64;
    let mut sum_odd_err: libc::c_double = 0.0f64;
    let mut stat_sum: libc::c_int = 0;
    let mut stat_e: libc::c_int = 0;
    let mut stat_h: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut n: libc::c_int = 0;
    if x < 500.0f64 && j < 80 as libc::c_int {
        let mut p: libc::c_double = gsl_sf_pow_int(x, j + 1 as libc::c_int);
        let mut g: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        gsl_sf_fact_e((j + 1 as libc::c_int) as libc::c_uint, &mut g);
        lnpre_val = 0.0f64;
        lnpre_err = 0.0f64;
        pre = p / g.val;
    } else {
        let mut lnx: libc::c_double = log(x);
        let mut lg: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        gsl_sf_lngamma_e(j as libc::c_double + 2.0f64, &mut lg);
        lnpre_val = (j as libc::c_double + 1.0f64) * lnx - lg.val;
        lnpre_err = 2.0f64 * 2.2204460492503131e-16f64
            * fabs((j as libc::c_double + 1.0f64) * lnx) + lg.err;
        pre = 1.0f64;
    }
    n = 1 as libc::c_int;
    while n < nmax {
        let mut del_val: libc::c_double = 0.;
        let mut del_err: libc::c_double = 0.;
        let mut U: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut M: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_h_U: libc::c_int = gsl_sf_hyperg_U_int_e(
            1 as libc::c_int,
            j + 2 as libc::c_int,
            n as libc::c_double * x,
            &mut U,
        );
        let mut stat_h_F: libc::c_int = gsl_sf_hyperg_1F1_int_e(
            1 as libc::c_int,
            j + 2 as libc::c_int,
            -n as libc::c_double * x,
            &mut M,
        );
        stat_h = if stat_h != GSL_SUCCESS as libc::c_int {
            stat_h
        } else if stat_h_U != GSL_SUCCESS as libc::c_int {
            stat_h_U
        } else if stat_h_F != GSL_SUCCESS as libc::c_int {
            stat_h_F
        } else {
            GSL_SUCCESS as libc::c_int
        };
        del_val = (j as libc::c_double + 1.0f64) * U.val - M.val;
        del_err = fabs(j as libc::c_double + 1.0f64) * U.err + M.err;
        sum_odd_val += del_val;
        sum_odd_err += del_err;
        if fabs(del_val / sum_odd_val) < 2.2204460492503131e-16f64 {
            break;
        }
        n += 2 as libc::c_int;
    }
    n = 2 as libc::c_int;
    while n < nmax {
        let mut del_val_0: libc::c_double = 0.;
        let mut del_err_0: libc::c_double = 0.;
        let mut U_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut M_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_h_U_0: libc::c_int = gsl_sf_hyperg_U_int_e(
            1 as libc::c_int,
            j + 2 as libc::c_int,
            n as libc::c_double * x,
            &mut U_0,
        );
        let mut stat_h_F_0: libc::c_int = gsl_sf_hyperg_1F1_int_e(
            1 as libc::c_int,
            j + 2 as libc::c_int,
            -n as libc::c_double * x,
            &mut M_0,
        );
        stat_h = if stat_h != GSL_SUCCESS as libc::c_int {
            stat_h
        } else if stat_h_U_0 != GSL_SUCCESS as libc::c_int {
            stat_h_U_0
        } else if stat_h_F_0 != GSL_SUCCESS as libc::c_int {
            stat_h_F_0
        } else {
            GSL_SUCCESS as libc::c_int
        };
        del_val_0 = (j as libc::c_double + 1.0f64) * U_0.val - M_0.val;
        del_err_0 = fabs(j as libc::c_double + 1.0f64) * U_0.err + M_0.err;
        sum_even_val -= del_val_0;
        sum_even_err += del_err_0;
        if fabs(del_val_0 / sum_even_val) < 2.2204460492503131e-16f64 {
            break;
        }
        n += 2 as libc::c_int;
    }
    stat_sum = if n >= nmax {
        GSL_EMAXITER as libc::c_int
    } else {
        GSL_SUCCESS as libc::c_int
    };
    stat_e = gsl_sf_exp_mult_err_e(
        lnpre_val,
        lnpre_err,
        pre * (sum_even_val + sum_odd_val),
        pre * (sum_even_err + sum_odd_err),
        result,
    );
    (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
    return if stat_e != GSL_SUCCESS as libc::c_int {
        stat_e
    } else if stat_h != GSL_SUCCESS as libc::c_int {
        stat_h
    } else if stat_sum != GSL_SUCCESS as libc::c_int {
        stat_sum
    } else {
        GSL_SUCCESS as libc::c_int
    };
}
pub unsafe extern "C" fn gsl_sf_fermi_dirac_m1_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x < -7.0839641853226408e+02f64 {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"fermi_dirac.c\0" as *const u8 as *const libc::c_char,
            1218 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    } else if x < 0.0f64 {
        let ex: libc::c_double = exp(x);
        (*result).val = ex / (1.0f64 + ex);
        (*result)
            .err = 2.0f64 * (fabs(x) + 1.0f64) * 2.2204460492503131e-16f64
            * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut ex_0: libc::c_double = exp(-x);
        (*result).val = 1.0f64 / (1.0f64 + ex_0);
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * (x + 1.0f64) * ex_0;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_fermi_dirac_0_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x < -7.0839641853226408e+02f64 {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"fermi_dirac.c\0" as *const u8 as *const libc::c_char,
            1239 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    } else if x < -5.0f64 {
        let mut ex: libc::c_double = exp(x);
        let mut ser: libc::c_double = 1.0f64
            - ex
                * (0.5f64
                    - ex
                        * (1.0f64 / 3.0f64
                            - ex
                                * (1.0f64 / 4.0f64
                                    - ex * (1.0f64 / 5.0f64 - ex / 6.0f64))));
        (*result).val = ex * ser;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < 10.0f64 {
        (*result).val = log(1.0f64 + exp(x));
        (*result).err = fabs(x * 2.2204460492503131e-16f64);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut ex_0: libc::c_double = exp(-x);
        (*result)
            .val = x
            + ex_0
                * (1.0f64 - 0.5f64 * ex_0 + ex_0 * ex_0 / 3.0f64
                    - ex_0 * ex_0 * ex_0 / 4.0f64);
        (*result).err = (x + ex_0) * 2.2204460492503131e-16f64;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_fermi_dirac_1_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x < -7.0839641853226408e+02f64 {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"fermi_dirac.c\0" as *const u8 as *const libc::c_char,
            1265 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    } else if x < -1.0f64 {
        let mut ex: libc::c_double = exp(x);
        let mut term: libc::c_double = ex;
        let mut sum: libc::c_double = term;
        let mut n: libc::c_int = 0;
        n = 2 as libc::c_int;
        while n < 100 as libc::c_int {
            let mut rat: libc::c_double = (n as libc::c_double - 1.0f64)
                / n as libc::c_double;
            term *= -ex * rat * rat;
            sum += term;
            if fabs(term / sum) < 2.2204460492503131e-16f64 {
                break;
            }
            n += 1;
            n;
        }
        (*result).val = sum;
        (*result).err = 2.0f64 * fabs(sum) * 2.2204460492503131e-16f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x < 1.0f64 {
        return cheb_eval_e(&mut fd_1_a_cs, x, result)
    } else if x < 4.0f64 {
        let mut t: libc::c_double = 2.0f64 / 3.0f64 * (x - 1.0f64) - 1.0f64;
        return cheb_eval_e(&mut fd_1_b_cs, t, result);
    } else if x < 10.0f64 {
        let mut t_0: libc::c_double = 1.0f64 / 3.0f64 * (x - 4.0f64) - 1.0f64;
        return cheb_eval_e(&mut fd_1_c_cs, t_0, result);
    } else if x < 30.0f64 {
        let mut t_1: libc::c_double = 0.1f64 * x - 2.0f64;
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut fd_1_d_cs, t_1, &mut c);
        (*result).val = c.val * x * x;
        (*result).err = c.err * x * x + 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < 1.0f64 / 1.4901161193847656e-08f64 {
        let mut t_2: libc::c_double = 60.0f64 / x - 1.0f64;
        let mut c_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut fd_1_e_cs, t_2, &mut c_0);
        (*result).val = c_0.val * x * x;
        (*result)
            .err = c_0.err * x * x + 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < 1.3407807929942596e+154f64 {
        (*result).val = 0.5f64 * x * x;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"fermi_dirac.c\0" as *const u8 as *const libc::c_char,
            1317 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_fermi_dirac_2_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x < -7.0839641853226408e+02f64 {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"fermi_dirac.c\0" as *const u8 as *const libc::c_char,
            1325 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    } else if x < -1.0f64 {
        let mut ex: libc::c_double = exp(x);
        let mut term: libc::c_double = ex;
        let mut sum: libc::c_double = term;
        let mut n: libc::c_int = 0;
        n = 2 as libc::c_int;
        while n < 100 as libc::c_int {
            let mut rat: libc::c_double = (n as libc::c_double - 1.0f64)
                / n as libc::c_double;
            term *= -ex * rat * rat * rat;
            sum += term;
            if fabs(term / sum) < 2.2204460492503131e-16f64 {
                break;
            }
            n += 1;
            n;
        }
        (*result).val = sum;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs(sum);
        return GSL_SUCCESS as libc::c_int;
    } else if x < 1.0f64 {
        return cheb_eval_e(&mut fd_2_a_cs, x, result)
    } else if x < 4.0f64 {
        let mut t: libc::c_double = 2.0f64 / 3.0f64 * (x - 1.0f64) - 1.0f64;
        return cheb_eval_e(&mut fd_2_b_cs, t, result);
    } else if x < 10.0f64 {
        let mut t_0: libc::c_double = 1.0f64 / 3.0f64 * (x - 4.0f64) - 1.0f64;
        return cheb_eval_e(&mut fd_2_c_cs, t_0, result);
    } else if x < 30.0f64 {
        let mut t_1: libc::c_double = 0.1f64 * x - 2.0f64;
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut fd_2_d_cs, t_1, &mut c);
        (*result).val = c.val * x * x * x;
        (*result)
            .err = c.err * x * x * x
            + 3.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < 1.0f64 / 6.0554544523933429e-06f64 {
        let mut t_2: libc::c_double = 60.0f64 / x - 1.0f64;
        let mut c_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut fd_2_e_cs, t_2, &mut c_0);
        (*result).val = c_0.val * x * x * x;
        (*result)
            .err = c_0.err * x * x * x
            + 3.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < 5.6438030941222897e+102f64 {
        (*result).val = 1.0f64 / 6.0f64 * x * x * x;
        (*result).err = 3.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"fermi_dirac.c\0" as *const u8 as *const libc::c_char,
            1377 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_fermi_dirac_int_e(
    j: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if j < -(1 as libc::c_int) {
        return fd_nint(j, x, result)
    } else if j == -(1 as libc::c_int) {
        return gsl_sf_fermi_dirac_m1_e(x, result)
    } else if j == 0 as libc::c_int {
        return gsl_sf_fermi_dirac_0_e(x, result)
    } else if j == 1 as libc::c_int {
        return gsl_sf_fermi_dirac_1_e(x, result)
    } else if j == 2 as libc::c_int {
        return gsl_sf_fermi_dirac_2_e(x, result)
    } else if x < 0.0f64 {
        return fd_neg(j as libc::c_double, x, result)
    } else if x == 0.0f64 {
        return gsl_sf_eta_int_e(j + 1 as libc::c_int, result)
    } else if x < 1.5f64 {
        return fd_series_int(j, x, result)
    } else {
        let mut fasymp: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_asymp: libc::c_int = fd_asymp(j as libc::c_double, x, &mut fasymp);
        if stat_asymp == GSL_SUCCESS as libc::c_int {
            (*result).val = fasymp.val;
            (*result).err = fasymp.err;
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            return stat_asymp;
        } else {
            return fd_UMseries_int(j, x, result)
        }
    };
}
pub unsafe extern "C" fn gsl_sf_fermi_dirac_mhalf_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x < -7.0839641853226408e+02f64 {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"fermi_dirac.c\0" as *const u8 as *const libc::c_char,
            1428 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    } else if x < -1.0f64 {
        let mut ex: libc::c_double = exp(x);
        let mut term: libc::c_double = ex;
        let mut sum: libc::c_double = term;
        let mut n: libc::c_int = 0;
        n = 2 as libc::c_int;
        while n < 200 as libc::c_int {
            let mut rat: libc::c_double = (n as libc::c_double - 1.0f64)
                / n as libc::c_double;
            term *= -ex * sqrt(rat);
            sum += term;
            if fabs(term / sum) < 2.2204460492503131e-16f64 {
                break;
            }
            n += 1;
            n;
        }
        (*result).val = sum;
        (*result).err = 2.0f64 * fabs(sum) * 2.2204460492503131e-16f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x < 1.0f64 {
        return cheb_eval_e(&mut fd_mhalf_a_cs, x, result)
    } else if x < 4.0f64 {
        let mut t: libc::c_double = 2.0f64 / 3.0f64 * (x - 1.0f64) - 1.0f64;
        return cheb_eval_e(&mut fd_mhalf_b_cs, t, result);
    } else if x < 10.0f64 {
        let mut t_0: libc::c_double = 1.0f64 / 3.0f64 * (x - 4.0f64) - 1.0f64;
        return cheb_eval_e(&mut fd_mhalf_c_cs, t_0, result);
    } else if x < 30.0f64 {
        let mut rtx: libc::c_double = sqrt(x);
        let mut t_1: libc::c_double = 0.1f64 * x - 2.0f64;
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut fd_mhalf_d_cs, t_1, &mut c);
        (*result).val = c.val * rtx;
        (*result)
            .err = c.err * rtx
            + 0.5f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        return fd_asymp(-0.5f64, x, result)
    };
}
pub unsafe extern "C" fn gsl_sf_fermi_dirac_half_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x < -7.0839641853226408e+02f64 {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"fermi_dirac.c\0" as *const u8 as *const libc::c_char,
            1476 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    } else if x < -1.0f64 {
        let mut ex: libc::c_double = exp(x);
        let mut term: libc::c_double = ex;
        let mut sum: libc::c_double = term;
        let mut n: libc::c_int = 0;
        n = 2 as libc::c_int;
        while n < 100 as libc::c_int {
            let mut rat: libc::c_double = (n as libc::c_double - 1.0f64)
                / n as libc::c_double;
            term *= -ex * rat * sqrt(rat);
            sum += term;
            if fabs(term / sum) < 2.2204460492503131e-16f64 {
                break;
            }
            n += 1;
            n;
        }
        (*result).val = sum;
        (*result).err = 2.0f64 * fabs(sum) * 2.2204460492503131e-16f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x < 1.0f64 {
        return cheb_eval_e(&mut fd_half_a_cs, x, result)
    } else if x < 4.0f64 {
        let mut t: libc::c_double = 2.0f64 / 3.0f64 * (x - 1.0f64) - 1.0f64;
        return cheb_eval_e(&mut fd_half_b_cs, t, result);
    } else if x < 10.0f64 {
        let mut t_0: libc::c_double = 1.0f64 / 3.0f64 * (x - 4.0f64) - 1.0f64;
        return cheb_eval_e(&mut fd_half_c_cs, t_0, result);
    } else if x < 30.0f64 {
        let mut x32: libc::c_double = x * sqrt(x);
        let mut t_1: libc::c_double = 0.1f64 * x - 2.0f64;
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut fd_half_d_cs, t_1, &mut c);
        (*result).val = c.val * x32;
        (*result)
            .err = c.err * x32
            + 1.5f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        return fd_asymp(0.5f64, x, result)
    };
}
pub unsafe extern "C" fn gsl_sf_fermi_dirac_3half_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x < -7.0839641853226408e+02f64 {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"fermi_dirac.c\0" as *const u8 as *const libc::c_char,
            1524 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    } else if x < -1.0f64 {
        let mut ex: libc::c_double = exp(x);
        let mut term: libc::c_double = ex;
        let mut sum: libc::c_double = term;
        let mut n: libc::c_int = 0;
        n = 2 as libc::c_int;
        while n < 100 as libc::c_int {
            let mut rat: libc::c_double = (n as libc::c_double - 1.0f64)
                / n as libc::c_double;
            term *= -ex * rat * rat * sqrt(rat);
            sum += term;
            if fabs(term / sum) < 2.2204460492503131e-16f64 {
                break;
            }
            n += 1;
            n;
        }
        (*result).val = sum;
        (*result).err = 2.0f64 * fabs(sum) * 2.2204460492503131e-16f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x < 1.0f64 {
        return cheb_eval_e(&mut fd_3half_a_cs, x, result)
    } else if x < 4.0f64 {
        let mut t: libc::c_double = 2.0f64 / 3.0f64 * (x - 1.0f64) - 1.0f64;
        return cheb_eval_e(&mut fd_3half_b_cs, t, result);
    } else if x < 10.0f64 {
        let mut t_0: libc::c_double = 1.0f64 / 3.0f64 * (x - 4.0f64) - 1.0f64;
        return cheb_eval_e(&mut fd_3half_c_cs, t_0, result);
    } else if x < 30.0f64 {
        let mut x52: libc::c_double = x * x * sqrt(x);
        let mut t_1: libc::c_double = 0.1f64 * x - 2.0f64;
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut fd_3half_d_cs, t_1, &mut c);
        (*result).val = c.val * x52;
        (*result)
            .err = c.err * x52
            + 2.5f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        return fd_asymp(1.5f64, x, result)
    };
}
pub unsafe extern "C" fn gsl_sf_fermi_dirac_inc_0_e(
    x: libc::c_double,
    b: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if b < 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"fermi_dirac.c\0" as *const u8 as *const libc::c_char,
            1572 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let mut arg: libc::c_double = b - x;
        let mut f0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut status: libc::c_int = gsl_sf_fermi_dirac_0_e(arg, &mut f0);
        (*result).val = f0.val - arg;
        (*result).err = f0.err + 2.2204460492503131e-16f64 * (fabs(x) + fabs(b));
        return status;
    };
}
pub unsafe extern "C" fn gsl_sf_fermi_dirac_m1(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_fermi_dirac_m1_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_fermi_dirac_m1_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"fermi_dirac.c\0" as *const u8 as *const libc::c_char,
            1592 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_fermi_dirac_0(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_fermi_dirac_0_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_fermi_dirac_0_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"fermi_dirac.c\0" as *const u8 as *const libc::c_char,
            1597 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_fermi_dirac_1(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_fermi_dirac_1_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_fermi_dirac_1_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"fermi_dirac.c\0" as *const u8 as *const libc::c_char,
            1602 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_fermi_dirac_2(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_fermi_dirac_2_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_fermi_dirac_2_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"fermi_dirac.c\0" as *const u8 as *const libc::c_char,
            1607 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_fermi_dirac_int(
    j: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_fermi_dirac_int_e(j, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_fermi_dirac_int_e(j, x, &result)\0" as *const u8
                as *const libc::c_char,
            b"fermi_dirac.c\0" as *const u8 as *const libc::c_char,
            1612 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_fermi_dirac_mhalf(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_fermi_dirac_mhalf_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_fermi_dirac_mhalf_e(x, &result)\0" as *const u8
                as *const libc::c_char,
            b"fermi_dirac.c\0" as *const u8 as *const libc::c_char,
            1617 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_fermi_dirac_half(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_fermi_dirac_half_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_fermi_dirac_half_e(x, &result)\0" as *const u8
                as *const libc::c_char,
            b"fermi_dirac.c\0" as *const u8 as *const libc::c_char,
            1622 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_fermi_dirac_3half(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_fermi_dirac_3half_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_fermi_dirac_3half_e(x, &result)\0" as *const u8
                as *const libc::c_char,
            b"fermi_dirac.c\0" as *const u8 as *const libc::c_char,
            1627 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_fermi_dirac_inc_0(
    x: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_fermi_dirac_inc_0_e(x, b, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_fermi_dirac_inc_0_e(x, b, &result)\0" as *const u8
                as *const libc::c_char,
            b"fermi_dirac.c\0" as *const u8 as *const libc::c_char,
            1632 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
