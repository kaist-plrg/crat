use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
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
pub struct cheb_series_struct {
    pub c: *mut libc::c_double,
    pub order: libc::c_int,
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub order_sp: libc::c_int,
}
pub type cheb_series = cheb_series_struct;
static mut bm0_data: [libc::c_double; 21] = [
    0.09284961637381644f64,
    -0.00142987707403484f64,
    0.00002830579271257f64,
    -0.00000143300611424f64,
    0.00000012028628046f64,
    -0.00000001397113013f64,
    0.00000000204076188f64,
    -0.00000000035399669f64,
    0.00000000007024759f64,
    -0.00000000001554107f64,
    0.00000000000376226f64,
    -0.00000000000098282f64,
    0.00000000000027408f64,
    -0.00000000000008091f64,
    0.00000000000002511f64,
    -0.00000000000000814f64,
    0.00000000000000275f64,
    -0.00000000000000096f64,
    0.00000000000000034f64,
    -0.00000000000000012f64,
    0.00000000000000004f64,
];
pub static mut _gsl_sf_bessel_amp_phase_bm0_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: bm0_data.as_ptr() as *mut _,
            order: 20 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 10 as libc::c_int,
        };
        init
    }
};
static mut bth0_data: [libc::c_double; 24] = [
    -0.24639163774300119f64,
    0.001737098307508963f64,
    -0.000062183633402968f64,
    0.000004368050165742f64,
    -0.000000456093019869f64,
    0.000000062197400101f64,
    -0.000000010300442889f64,
    0.000000001979526776f64,
    -0.000000000428198396f64,
    0.000000000102035840f64,
    -0.000000000026363898f64,
    0.000000000007297935f64,
    -0.000000000002144188f64,
    0.000000000000663693f64,
    -0.000000000000215126f64,
    0.000000000000072659f64,
    -0.000000000000025465f64,
    0.000000000000009229f64,
    -0.000000000000003448f64,
    0.000000000000001325f64,
    -0.000000000000000522f64,
    0.000000000000000210f64,
    -0.000000000000000087f64,
    0.000000000000000036f64,
];
pub static mut _gsl_sf_bessel_amp_phase_bth0_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: bth0_data.as_ptr() as *mut _,
            order: 23 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 12 as libc::c_int,
        };
        init
    }
};
static mut bm1_data: [libc::c_double; 21] = [
    0.1047362510931285f64,
    0.00442443893702345f64,
    -0.00005661639504035f64,
    0.00000231349417339f64,
    -0.00000017377182007f64,
    0.00000001893209930f64,
    -0.00000000265416023f64,
    0.00000000044740209f64,
    -0.00000000008691795f64,
    0.00000000001891492f64,
    -0.00000000000451884f64,
    0.00000000000116765f64,
    -0.00000000000032265f64,
    0.00000000000009450f64,
    -0.00000000000002913f64,
    0.00000000000000939f64,
    -0.00000000000000315f64,
    0.00000000000000109f64,
    -0.00000000000000039f64,
    0.00000000000000014f64,
    -0.00000000000000005f64,
];
pub static mut _gsl_sf_bessel_amp_phase_bm1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: bm1_data.as_ptr() as *mut _,
            order: 20 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 10 as libc::c_int,
        };
        init
    }
};
static mut bth1_data: [libc::c_double; 24] = [
    0.74060141026313850f64,
    -0.004571755659637690f64,
    0.000119818510964326f64,
    -0.000006964561891648f64,
    0.000000655495621447f64,
    -0.000000084066228945f64,
    0.000000013376886564f64,
    -0.000000002499565654f64,
    0.000000000529495100f64,
    -0.000000000124135944f64,
    0.000000000031656485f64,
    -0.000000000008668640f64,
    0.000000000002523758f64,
    -0.000000000000775085f64,
    0.000000000000249527f64,
    -0.000000000000083773f64,
    0.000000000000029205f64,
    -0.000000000000010534f64,
    0.000000000000003919f64,
    -0.000000000000001500f64,
    0.000000000000000589f64,
    -0.000000000000000237f64,
    0.000000000000000097f64,
    -0.000000000000000040f64,
];
pub static mut _gsl_sf_bessel_amp_phase_bth1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: bth1_data.as_ptr() as *mut _,
            order: 23 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 12 as libc::c_int,
        };
        init
    }
};
pub unsafe extern "C" fn gsl_sf_bessel_asymp_Mnu_e(
    nu: libc::c_double,
    x: libc::c_double,
    mut result: *mut libc::c_double,
) -> libc::c_int {
    let r: libc::c_double = 2.0f64 * nu / x;
    let r2: libc::c_double = r * r;
    let x2: libc::c_double = x * x;
    let term1: libc::c_double = (r2 - 1.0f64 / x2) / 8.0f64;
    let term2: libc::c_double = (r2 - 1.0f64 / x2) * (r2 - 9.0f64 / x2) * 3.0f64
        / 128.0f64;
    let Mnu2_c: libc::c_double = 2.0f64 / 3.14159265358979323846f64
        * (1.0f64 + term1 + term2);
    *result = sqrt(Mnu2_c) / sqrt(x);
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_bessel_asymp_thetanu_corr_e(
    nu: libc::c_double,
    x: libc::c_double,
    mut result: *mut libc::c_double,
) -> libc::c_int {
    let r: libc::c_double = 2.0f64 * nu / x;
    let r2: libc::c_double = r * r;
    let x2: libc::c_double = x * x;
    let term1: libc::c_double = x * (r2 - 1.0f64 / x2) / 8.0f64;
    let term2: libc::c_double = x * (r2 - 1.0f64 / x2) * (r2 - 25.0f64 / x2) / 384.0f64;
    *result = -0.25f64 * 3.14159265358979323846f64 + term1 + term2;
    return GSL_SUCCESS as libc::c_int;
}
