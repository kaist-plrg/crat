use ::libc;
extern "C" {
    fn sin(_: libc::c_double) -> libc::c_double;
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
    fn gsl_sf_complex_log_e(
        zr: libc::c_double,
        zi: libc::c_double,
        lnr: *mut gsl_sf_result,
        theta: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_psi_int_e(n: libc::c_int, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_psi_1_int_e(n: libc::c_int, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_psi_n_e(
        n: libc::c_int,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_complex_logsin_e(
        zr: libc::c_double,
        zi: libc::c_double,
        lszr: *mut gsl_sf_result,
        lszi: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_angle_restrict_symm_e(theta: *mut libc::c_double) -> libc::c_int;
    fn gsl_sf_angle_restrict_symm_err_e(
        theta: libc::c_double,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub n: libc::c_int,
    pub f: libc::c_double,
    pub i: libc::c_long,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub n: libc::c_int,
    pub f: libc::c_double,
    pub i: libc::c_long,
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
static mut fact_table: [C2RustUnnamed_0; 171] = [
    {
        let mut init = C2RustUnnamed_0 {
            n: 0 as libc::c_int,
            f: 1.0f64,
            i: 1 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 1 as libc::c_int,
            f: 1.0f64,
            i: 1 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 2 as libc::c_int,
            f: 2.0f64,
            i: 2 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 3 as libc::c_int,
            f: 6.0f64,
            i: 6 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 4 as libc::c_int,
            f: 24.0f64,
            i: 24 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 5 as libc::c_int,
            f: 120.0f64,
            i: 120 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 6 as libc::c_int,
            f: 720.0f64,
            i: 720 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 7 as libc::c_int,
            f: 5040.0f64,
            i: 5040 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 8 as libc::c_int,
            f: 40320.0f64,
            i: 40320 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 9 as libc::c_int,
            f: 362880.0f64,
            i: 362880 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 10 as libc::c_int,
            f: 3628800.0f64,
            i: 3628800 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 11 as libc::c_int,
            f: 39916800.0f64,
            i: 39916800 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 12 as libc::c_int,
            f: 479001600.0f64,
            i: 479001600 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 13 as libc::c_int,
            f: 6227020800.0f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 14 as libc::c_int,
            f: 87178291200.0f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 15 as libc::c_int,
            f: 1307674368000.0f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 16 as libc::c_int,
            f: 20922789888000.0f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 17 as libc::c_int,
            f: 355687428096000.0f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 18 as libc::c_int,
            f: 6402373705728000.0f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 19 as libc::c_int,
            f: 121645100408832000.0f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 20 as libc::c_int,
            f: 2432902008176640000.0f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 21 as libc::c_int,
            f: 51090942171709440000.0f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 22 as libc::c_int,
            f: 1124000727777607680000.0f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 23 as libc::c_int,
            f: 25852016738884976640000.0f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 24 as libc::c_int,
            f: 620448401733239439360000.0f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 25 as libc::c_int,
            f: 15511210043330985984000000.0f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 26 as libc::c_int,
            f: 403291461126605635584000000.0f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 27 as libc::c_int,
            f: 10888869450418352160768000000.0f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 28 as libc::c_int,
            f: 304888344611713860501504000000.0f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 29 as libc::c_int,
            f: 8841761993739701954543616000000.0f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 30 as libc::c_int,
            f: 265252859812191058636308480000000.0f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 31 as libc::c_int,
            f: 8222838654177922817725562880000000.0f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 32 as libc::c_int,
            f: 263130836933693530167218012160000000.0f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 33 as libc::c_int,
            f: 8683317618811886495518194401280000000.0f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 34 as libc::c_int,
            f: 2.95232799039604140847618609644e38f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 35 as libc::c_int,
            f: 1.03331479663861449296666513375e40f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 36 as libc::c_int,
            f: 3.71993326789901217467999448151e41f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 37 as libc::c_int,
            f: 1.37637530912263450463159795816e43f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 38 as libc::c_int,
            f: 5.23022617466601111760007224100e44f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 39 as libc::c_int,
            f: 2.03978820811974433586402817399e46f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 40 as libc::c_int,
            f: 8.15915283247897734345611269600e47f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 41 as libc::c_int,
            f: 3.34525266131638071081700620534e49f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 42 as libc::c_int,
            f: 1.40500611775287989854314260624e51f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 43 as libc::c_int,
            f: 6.04152630633738356373551320685e52f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 44 as libc::c_int,
            f: 2.65827157478844876804362581101e54f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 45 as libc::c_int,
            f: 1.19622220865480194561963161496e56f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 46 as libc::c_int,
            f: 5.50262215981208894985030542880e57f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 47 as libc::c_int,
            f: 2.58623241511168180642964355154e59f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 48 as libc::c_int,
            f: 1.24139155925360726708622890474e61f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 49 as libc::c_int,
            f: 6.08281864034267560872252163321e62f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 50 as libc::c_int,
            f: 3.04140932017133780436126081661e64f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 51 as libc::c_int,
            f: 1.55111875328738228022424301647e66f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 52 as libc::c_int,
            f: 8.06581751709438785716606368564e67f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 53 as libc::c_int,
            f: 4.27488328406002556429801375339e69f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 54 as libc::c_int,
            f: 2.30843697339241380472092742683e71f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 55 as libc::c_int,
            f: 1.26964033536582759259651008476e73f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 56 as libc::c_int,
            f: 7.10998587804863451854045647464e74f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 57 as libc::c_int,
            f: 4.05269195048772167556806019054e76f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 58 as libc::c_int,
            f: 2.35056133128287857182947491052e78f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 59 as libc::c_int,
            f: 1.38683118545689835737939019720e80f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 60 as libc::c_int,
            f: 8.32098711274139014427634118320e81f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 61 as libc::c_int,
            f: 5.07580213877224798800856812177e83f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 62 as libc::c_int,
            f: 3.14699732603879375256531223550e85f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 63 as libc::c_int,
            f: 1.982608315404440064116146708360e87f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 64 as libc::c_int,
            f: 1.268869321858841641034333893350e89f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 65 as libc::c_int,
            f: 8.247650592082470666723170306800e90f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 66 as libc::c_int,
            f: 5.443449390774430640037292402480e92f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 67 as libc::c_int,
            f: 3.647111091818868528824985909660e94f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 68 as libc::c_int,
            f: 2.480035542436830599600990418570e96f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 69 as libc::c_int,
            f: 1.711224524281413113724683388810e98f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 70 as libc::c_int,
            f: 1.197857166996989179607278372170e100f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 71 as libc::c_int,
            f: 8.504785885678623175211676442400e101f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 72 as libc::c_int,
            f: 6.123445837688608686152407038530e103f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 73 as libc::c_int,
            f: 4.470115461512684340891257138130e105f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 74 as libc::c_int,
            f: 3.307885441519386412259530282210e107f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 75 as libc::c_int,
            f: 2.480914081139539809194647711660e109f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 76 as libc::c_int,
            f: 1.885494701666050254987932260860e111f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 77 as libc::c_int,
            f: 1.451830920282858696340707840860e113f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 78 as libc::c_int,
            f: 1.132428117820629783145752115870e115f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 79 as libc::c_int,
            f: 8.946182130782975286851441715400e116f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 80 as libc::c_int,
            f: 7.156945704626380229481153372320e118f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 81 as libc::c_int,
            f: 5.797126020747367985879734231580e120f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 82 as libc::c_int,
            f: 4.753643337012841748421382069890e122f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 83 as libc::c_int,
            f: 3.945523969720658651189747118010e124f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 84 as libc::c_int,
            f: 3.314240134565353266999387579130e126f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 85 as libc::c_int,
            f: 2.817104114380550276949479442260e128f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 86 as libc::c_int,
            f: 2.422709538367273238176552320340e130f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 87 as libc::c_int,
            f: 2.107757298379527717213600518700e132f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 88 as libc::c_int,
            f: 1.854826422573984391147968456460e134f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 89 as libc::c_int,
            f: 1.650795516090846108121691926250e136f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 90 as libc::c_int,
            f: 1.485715964481761497309522733620e138f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 91 as libc::c_int,
            f: 1.352001527678402962551665687590e140f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 92 as libc::c_int,
            f: 1.243841405464130725547532432590e142f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 93 as libc::c_int,
            f: 1.156772507081641574759205162310e144f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 94 as libc::c_int,
            f: 1.087366156656743080273652852570e146f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 95 as libc::c_int,
            f: 1.032997848823905926259970209940e148f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 96 as libc::c_int,
            f: 9.916779348709496892095714015400e149f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 97 as libc::c_int,
            f: 9.619275968248211985332842594960e151f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 98 as libc::c_int,
            f: 9.426890448883247745626185743100e153f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 99 as libc::c_int,
            f: 9.332621544394415268169923885600e155f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 100 as libc::c_int,
            f: 9.33262154439441526816992388563e157f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 101 as libc::c_int,
            f: 9.42594775983835942085162312450e159f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 102 as libc::c_int,
            f: 9.61446671503512660926865558700e161f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 103 as libc::c_int,
            f: 9.90290071648618040754671525458e163f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 104 as libc::c_int,
            f: 1.02990167451456276238485838648e166f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 105 as libc::c_int,
            f: 1.08139675824029090050410130580e168f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 106 as libc::c_int,
            f: 1.146280563734708354534347384148e170f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 107 as libc::c_int,
            f: 1.226520203196137939351751701040e172f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 108 as libc::c_int,
            f: 1.324641819451828974499891837120e174f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 109 as libc::c_int,
            f: 1.443859583202493582204882102460e176f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 110 as libc::c_int,
            f: 1.588245541522742940425370312710e178f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 111 as libc::c_int,
            f: 1.762952551090244663872161047110e180f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 112 as libc::c_int,
            f: 1.974506857221074023536820372760e182f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 113 as libc::c_int,
            f: 2.231192748659813646596607021220e184f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 114 as libc::c_int,
            f: 2.543559733472187557120132004190e186f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 115 as libc::c_int,
            f: 2.925093693493015690688151804820e188f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 116 as libc::c_int,
            f: 3.393108684451898201198256093590e190f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 117 as libc::c_int,
            f: 3.96993716080872089540195962950e192f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 118 as libc::c_int,
            f: 4.68452584975429065657431236281e194f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 119 as libc::c_int,
            f: 5.57458576120760588132343171174e196f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 120 as libc::c_int,
            f: 6.68950291344912705758811805409e198f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 121 as libc::c_int,
            f: 8.09429852527344373968162284545e200f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 122 as libc::c_int,
            f: 9.87504420083360136241157987140e202f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 123 as libc::c_int,
            f: 1.21463043670253296757662432419e205f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 124 as libc::c_int,
            f: 1.50614174151114087979501416199e207f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 125 as libc::c_int,
            f: 1.88267717688892609974376770249e209f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 126 as libc::c_int,
            f: 2.37217324288004688567714730514e211f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 127 as libc::c_int,
            f: 3.01266001845765954480997707753e213f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 128 as libc::c_int,
            f: 3.85620482362580421735677065923e215f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 129 as libc::c_int,
            f: 4.97450422247728744039023415041e217f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 130 as libc::c_int,
            f: 6.46685548922047367250730439554e219f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 131 as libc::c_int,
            f: 8.47158069087882051098456875820e221f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 132 as libc::c_int,
            f: 1.11824865119600430744996307608e224f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 133 as libc::c_int,
            f: 1.48727070609068572890845089118e226f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 134 as libc::c_int,
            f: 1.99294274616151887673732419418e228f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 135 as libc::c_int,
            f: 2.69047270731805048359538766215e230f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 136 as libc::c_int,
            f: 3.65904288195254865768972722052e232f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 137 as libc::c_int,
            f: 5.01288874827499166103492629211e234f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 138 as libc::c_int,
            f: 6.91778647261948849222819828311e236f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 139 as libc::c_int,
            f: 9.61572319694108900419719561353e238f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 140 as libc::c_int,
            f: 1.34620124757175246058760738589e241f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 141 as libc::c_int,
            f: 1.89814375907617096942852641411e243f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 142 as libc::c_int,
            f: 2.69536413788816277658850750804e245f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 143 as libc::c_int,
            f: 3.85437071718007277052156573649e247f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 144 as libc::c_int,
            f: 5.55029383273930478955105466055e249f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 145 as libc::c_int,
            f: 8.04792605747199194484902925780e251f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 146 as libc::c_int,
            f: 1.17499720439091082394795827164e254f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 147 as libc::c_int,
            f: 1.72724589045463891120349865931e256f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 148 as libc::c_int,
            f: 2.55632391787286558858117801578e258f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 149 as libc::c_int,
            f: 3.80892263763056972698595524351e260f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 150 as libc::c_int,
            f: 5.71338395644585459047893286526e262f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 151 as libc::c_int,
            f: 8.62720977423324043162318862650e264f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 152 as libc::c_int,
            f: 1.31133588568345254560672467123e267f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 153 as libc::c_int,
            f: 2.00634390509568239477828874699e269f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 154 as libc::c_int,
            f: 3.08976961384735088795856467036e271f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 155 as libc::c_int,
            f: 4.78914290146339387633577523906e273f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 156 as libc::c_int,
            f: 7.47106292628289444708380937294e275f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 157 as libc::c_int,
            f: 1.17295687942641442819215807155e278f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 158 as libc::c_int,
            f: 1.85327186949373479654360975305e280f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 159 as libc::c_int,
            f: 2.94670227249503832650433950735e282f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 160 as libc::c_int,
            f: 4.71472363599206132240694321176e284f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 161 as libc::c_int,
            f: 7.59070505394721872907517857094e286f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 162 as libc::c_int,
            f: 1.22969421873944943411017892849e289f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 163 as libc::c_int,
            f: 2.00440157654530257759959165344e291f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 164 as libc::c_int,
            f: 3.28721858553429622726333031164e293f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 165 as libc::c_int,
            f: 5.42391066613158877498449501421e295f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 166 as libc::c_int,
            f: 9.00369170577843736647426172359e297f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 167 as libc::c_int,
            f: 1.50361651486499904020120170784e300f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 168 as libc::c_int,
            f: 2.52607574497319838753801886917e302f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 169 as libc::c_int,
            f: 4.26906800900470527493925188890e304f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            n: 170 as libc::c_int,
            f: 7.25741561530799896739672821113e306f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
];
static mut doub_fact_table: [C2RustUnnamed_1; 298] = [
    {
        let mut init = C2RustUnnamed_1 {
            n: 0 as libc::c_int,
            f: 1.000000000000000000000000000f64,
            i: 1 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 1 as libc::c_int,
            f: 1.000000000000000000000000000f64,
            i: 1 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 2 as libc::c_int,
            f: 2.000000000000000000000000000f64,
            i: 2 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 3 as libc::c_int,
            f: 3.000000000000000000000000000f64,
            i: 3 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 4 as libc::c_int,
            f: 8.000000000000000000000000000f64,
            i: 8 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 5 as libc::c_int,
            f: 15.00000000000000000000000000f64,
            i: 15 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 6 as libc::c_int,
            f: 48.00000000000000000000000000f64,
            i: 48 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 7 as libc::c_int,
            f: 105.0000000000000000000000000f64,
            i: 105 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 8 as libc::c_int,
            f: 384.0000000000000000000000000f64,
            i: 384 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 9 as libc::c_int,
            f: 945.0000000000000000000000000f64,
            i: 945 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 10 as libc::c_int,
            f: 3840.000000000000000000000000f64,
            i: 3840 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 11 as libc::c_int,
            f: 10395.00000000000000000000000f64,
            i: 10395 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 12 as libc::c_int,
            f: 46080.00000000000000000000000f64,
            i: 46080 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 13 as libc::c_int,
            f: 135135.0000000000000000000000f64,
            i: 135135 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 14 as libc::c_int,
            f: 645120.00000000000000000000000f64,
            i: 645120 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 15 as libc::c_int,
            f: 2.02702500000000000000000000000e6f64,
            i: 2027025 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 16 as libc::c_int,
            f: 1.03219200000000000000000000000e7f64,
            i: 10321920 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 17 as libc::c_int,
            f: 3.4459425000000000000000000000e7f64,
            i: 34459425 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 18 as libc::c_int,
            f: 1.85794560000000000000000000000e8f64,
            i: 185794560 as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 19 as libc::c_int,
            f: 6.5472907500000000000000000000e8f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 20 as libc::c_int,
            f: 3.7158912000000000000000000000e9f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 21 as libc::c_int,
            f: 1.37493105750000000000000000000e10f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 22 as libc::c_int,
            f: 8.1749606400000000000000000000e10f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 23 as libc::c_int,
            f: 3.1623414322500000000000000000e11f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 24 as libc::c_int,
            f: 1.96199055360000000000000000000e12f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 25 as libc::c_int,
            f: 7.9058535806250000000000000000e12f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 26 as libc::c_int,
            f: 5.1011754393600000000000000000e13f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 27 as libc::c_int,
            f: 2.13458046676875000000000000000e14f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 28 as libc::c_int,
            f: 1.42832912302080000000000000000e15f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 29 as libc::c_int,
            f: 6.1902833536293750000000000000e15f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 30 as libc::c_int,
            f: 4.2849873690624000000000000000e16f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 31 as libc::c_int,
            f: 1.91898783962510625000000000000e17f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 32 as libc::c_int,
            f: 1.37119595809996800000000000000e18f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 33 as libc::c_int,
            f: 6.3326598707628506250000000000e18f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 34 as libc::c_int,
            f: 4.6620662575398912000000000000e19f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 35 as libc::c_int,
            f: 2.21643095476699771875000000000e20f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 36 as libc::c_int,
            f: 1.67834385271436083200000000000e21f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 37 as libc::c_int,
            f: 8.2007945326378915593750000000e21f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 38 as libc::c_int,
            f: 6.3777066403145711616000000000e22f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 39 as libc::c_int,
            f: 3.1983098677287777081562500000e23f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 40 as libc::c_int,
            f: 2.55108265612582846464000000000e24f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 41 as libc::c_int,
            f: 1.31130704576879886034406250000e25f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 42 as libc::c_int,
            f: 1.07145471557284795514880000000e26f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 43 as libc::c_int,
            f: 5.6386202968058350994794687500e26f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 44 as libc::c_int,
            f: 4.7144007485205310026547200000e27f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 45 as libc::c_int,
            f: 2.53737913356262579476576093750e28f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 46 as libc::c_int,
            f: 2.16862434431944426122117120000e29f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 47 as libc::c_int,
            f: 1.19256819277443412353990764062e30f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 48 as libc::c_int,
            f: 1.04093968527333324538616217600e31f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 49 as libc::c_int,
            f: 5.8435841445947272053455474391e31f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 50 as libc::c_int,
            f: 5.2046984263666662269308108800e32f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 51 as libc::c_int,
            f: 2.98022791374331087472622919392e33f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 52 as libc::c_int,
            f: 2.70644318171066643800402165760e34f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 53 as libc::c_int,
            f: 1.57952079428395476360490147278e35f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 54 as libc::c_int,
            f: 1.46147931812375987652217169510e36f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 55 as libc::c_int,
            f: 8.6873643685617511998269581003e36f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 56 as libc::c_int,
            f: 8.1842841814930553085241614926e37f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 57 as libc::c_int,
            f: 4.9517976900801981839013661172e38f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 58 as libc::c_int,
            f: 4.7468848252659720789440136657e39f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 59 as libc::c_int,
            f: 2.92156063714731692850180600912e40f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 60 as libc::c_int,
            f: 2.84813089515958324736640819942e41f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 61 as libc::c_int,
            f: 1.78215198865986332638610166557e42f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 62 as libc::c_int,
            f: 1.76584115499894161336717308364e43f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 63 as libc::c_int,
            f: 1.12275575285571389562324404931e44f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 64 as libc::c_int,
            f: 1.13013833919932263255499077353e45f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 65 as libc::c_int,
            f: 7.2979123935621403215510863205e45f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 66 as libc::c_int,
            f: 7.4589130387155293748629391053e46f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 67 as libc::c_int,
            f: 4.8896013036866340154392278347e47f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 68 as libc::c_int,
            f: 5.0720608663265599749067985916e48f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 69 as libc::c_int,
            f: 3.3738248995437774706530672060e49f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 70 as libc::c_int,
            f: 3.5504426064285919824347590141e50f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 71 as libc::c_int,
            f: 2.39541567867608200416367771623e51f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 72 as libc::c_int,
            f: 2.55631867662858622735302649017e52f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 73 as libc::c_int,
            f: 1.74865344543353986303948473285e53f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 74 as libc::c_int,
            f: 1.89167582070515380824123960272e54f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 75 as libc::c_int,
            f: 1.31149008407515489727961354964e55f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 76 as libc::c_int,
            f: 1.43767362373591689426334209807e56f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 77 as libc::c_int,
            f: 1.00984736473786927090530243322e57f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 78 as libc::c_int,
            f: 1.12138542651401517752540683649e58f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 79 as libc::c_int,
            f: 7.9777941814291672401518892225e58f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 80 as libc::c_int,
            f: 8.9710834121121214202032546920e59f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 81 as libc::c_int,
            f: 6.4620132869576254645230302702e60f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 82 as libc::c_int,
            f: 7.3562883979319395645666688474e61f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 83 as libc::c_int,
            f: 5.3634710281748291355541151243e62f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 84 as libc::c_int,
            f: 6.1792822542628292342360018318e63f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 85 as libc::c_int,
            f: 4.5589503739486047652209978556e64f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 86 as libc::c_int,
            f: 5.3141827386660331414429615754e65f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 87 as libc::c_int,
            f: 3.9662868253352861457422681344e66f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 88 as libc::c_int,
            f: 4.6764808100261091644698061863e67f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 89 as libc::c_int,
            f: 3.5299952745484046697106186396e68f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 90 as libc::c_int,
            f: 4.2088327290234982480228255677e69f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 91 as libc::c_int,
            f: 3.2122956998390482494366629620e70f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 92 as libc::c_int,
            f: 3.8721261107016183881809995223e71f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 93 as libc::c_int,
            f: 2.98743500085031487197609655470e72f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 94 as libc::c_int,
            f: 3.6397985440595212848901395509e73f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 95 as libc::c_int,
            f: 2.83806325080779912837729172696e74f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 96 as libc::c_int,
            f: 3.4942066022971404334945339689e75f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 97 as libc::c_int,
            f: 2.75292135328356515452597297515e76f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 98 as libc::c_int,
            f: 3.4243224702511976248246432895e77f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 99 as libc::c_int,
            f: 2.72539213975072950298071324540e78f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 100 as libc::c_int,
            f: 3.4243224702511976248246432895e79f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 101 as libc::c_int,
            f: 2.75264606114823679801052037785e80f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 102 as libc::c_int,
            f: 3.4928089196562215773211361553e81f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 103 as libc::c_int,
            f: 2.83522544298268390195083598919e82f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 104 as libc::c_int,
            f: 3.6325212764424704404139816015e83f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 105 as libc::c_int,
            f: 2.97698671513181809704837778865e84f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 106 as libc::c_int,
            f: 3.8504725530290186668388204976e85f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 107 as libc::c_int,
            f: 3.1853757851910453638417642339e86f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 108 as libc::c_int,
            f: 4.1585103572713401601859261374e87f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 109 as libc::c_int,
            f: 3.4720596058582394465875230149e88f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 110 as libc::c_int,
            f: 4.5743613929984741762045187512e89f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 111 as libc::c_int,
            f: 3.8539861625026457857121505465e90f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 112 as libc::c_int,
            f: 5.1232847601582910773490610013e91f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 113 as libc::c_int,
            f: 4.3550043636279897378547301176e92f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 114 as libc::c_int,
            f: 5.8405446265804518281779295415e93f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 115 as libc::c_int,
            f: 5.0082550181721881985329396352e94f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 116 as libc::c_int,
            f: 6.7750317668333241206863982681e95f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 117 as libc::c_int,
            f: 5.8596583712614601922835393732e96f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 118 as libc::c_int,
            f: 7.9945374848633224624099499564e97f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 119 as libc::c_int,
            f: 6.9729934618011376288174118541e98f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 120 as libc::c_int,
            f: 9.5934449818359869548919399477e99f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 121 as libc::c_int,
            f: 8.4373220887793765308690683435e100f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 122 as libc::c_int,
            f: 1.17040028778399040849681667362e102f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 123 as libc::c_int,
            f: 1.03779061691986331329689540625e103f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 124 as libc::c_int,
            f: 1.45129635685214810653605267528e104f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 125 as libc::c_int,
            f: 1.29723827114982914162111925781e105f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 126 as libc::c_int,
            f: 1.82863340963370661423542637086e106f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 127 as libc::c_int,
            f: 1.64749260436028300985882145742e107f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 128 as libc::c_int,
            f: 2.34065076433114446622134575470e108f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 129 as libc::c_int,
            f: 2.12526545962476508271787968008e109f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 130 as libc::c_int,
            f: 3.04284599363048780608774948111e110f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 131 as libc::c_int,
            f: 2.78409775210844225836042238090e111f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 132 as libc::c_int,
            f: 4.0165567115922439040358293151e112f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 133 as libc::c_int,
            f: 3.7028500103042282036193617666e113f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 134 as libc::c_int,
            f: 5.3821859935336068314080112822e114f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 135 as libc::c_int,
            f: 4.9988475139107080748861383849e115f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 136 as libc::c_int,
            f: 7.3197729512057052907148953438e116f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 137 as libc::c_int,
            f: 6.8484210940576700625940095873e117f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 138 as libc::c_int,
            f: 1.01012866726638733011865555744e119f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 139 as libc::c_int,
            f: 9.5193053207401613870056733264e119f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 140 as libc::c_int,
            f: 1.41418013417294226216611778042e121f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 141 as libc::c_int,
            f: 1.34222205022436275556779993902e122f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 142 as libc::c_int,
            f: 2.00813579052557801227588724819e123f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 143 as libc::c_int,
            f: 1.91937753182083874046195391280e124f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 144 as libc::c_int,
            f: 2.89171553835683233767727763739e125f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 145 as libc::c_int,
            f: 2.78309742114021617366983317355e126f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 146 as libc::c_int,
            f: 4.2219046860009752130088253506e127f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 147 as libc::c_int,
            f: 4.0911532090761177752946547651e128f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 148 as libc::c_int,
            f: 6.2484189352814433152530615189e129f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 149 as libc::c_int,
            f: 6.0958182815234154851890356000e130f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 150 as libc::c_int,
            f: 9.3726284029221649728795922783e131f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 151 as libc::c_int,
            f: 9.2046856051003573826354437561e132f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 152 as libc::c_int,
            f: 1.42463951724416907587769802630e134f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 153 as libc::c_int,
            f: 1.40831689758035467954322289468e135f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 154 as libc::c_int,
            f: 2.19394485655602037685165496051e136f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 155 as libc::c_int,
            f: 2.18289119124954975329199548675e137f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 156 as libc::c_int,
            f: 3.4225539762273917878885817384e138f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 157 as libc::c_int,
            f: 3.4271391702617931126684329142e139f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 158 as libc::c_int,
            f: 5.4076352824392790248639591467e140f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 159 as libc::c_int,
            f: 5.4491512807162510491428083336e141f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 160 as libc::c_int,
            f: 8.6522164519028464397823346347e142f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 161 as libc::c_int,
            f: 8.7731335619531641891199214170e143f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 162 as libc::c_int,
            f: 1.40165906520826112324473821082e145f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 163 as libc::c_int,
            f: 1.43002077059836576282654719098e146f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 164 as libc::c_int,
            f: 2.29872086694154824212137066574e147f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 165 as libc::c_int,
            f: 2.35953427148730350866380286512e148f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 166 as libc::c_int,
            f: 3.8158766391229700819214753051e149f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 167 as libc::c_int,
            f: 3.9404222333837968594685507847e150f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 168 as libc::c_int,
            f: 6.4106727537265897376280785126e151f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 169 as libc::c_int,
            f: 6.6593135744186166925018508262e152f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 170 as libc::c_int,
            f: 1.08981436813352025539677334714e154f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 171 as libc::c_int,
            f: 1.13874262122558345441781649128e155f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 172 as libc::c_int,
            f: 1.87448071318965483928245015709e156f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 173 as libc::c_int,
            f: 1.97002473472025937614282252992e157f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 174 as libc::c_int,
            f: 3.2615964409499994203514632733e158f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 175 as libc::c_int,
            f: 3.4475432857604539082499394274e159f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 176 as libc::c_int,
            f: 5.7404097360719989798185753611e160f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 177 as libc::c_int,
            f: 6.1021516157960034176023927864e161f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 178 as libc::c_int,
            f: 1.02179293302081581840770641427e163f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 179 as libc::c_int,
            f: 1.09228513922748461175082830877e164f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 180 as libc::c_int,
            f: 1.83922727943746847313387154568e165f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 181 as libc::c_int,
            f: 1.97703610200174714726899923887e166f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 182 as libc::c_int,
            f: 3.3473936485761926211036462131e167f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 183 as libc::c_int,
            f: 3.6179760666631972795022686071e168f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 184 as libc::c_int,
            f: 6.1592043133801944228307090322e169f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 185 as libc::c_int,
            f: 6.6932557233269149670791969232e170f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 186 as libc::c_int,
            f: 1.14561200228871616264651187999e172f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 187 as libc::c_int,
            f: 1.25163882026213309884380982464e173f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 188 as libc::c_int,
            f: 2.15375056430278638577544233437e174f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 189 as libc::c_int,
            f: 2.36559737029543155681480056857e175f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 190 as libc::c_int,
            f: 4.0921260721752941329733404353e176f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 191 as libc::c_int,
            f: 4.5182909772642742735162690860e177f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 192 as libc::c_int,
            f: 7.8568820585765647353088136358e178f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 193 as libc::c_int,
            f: 8.7203015861200493478863993359e179f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 194 as libc::c_int,
            f: 1.52423511936385355864990984535e181f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 195 as libc::c_int,
            f: 1.70045880929340962283784787050e182f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 196 as libc::c_int,
            f: 2.98750083395315297495382329688e183f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 197 as libc::c_int,
            f: 3.3499038543080169569905603049e184f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 198 as libc::c_int,
            f: 5.9152516512272428904085701278e185f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 199 as libc::c_int,
            f: 6.6663086700729537444112150067e186f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 200 as libc::c_int,
            f: 1.18305033024544857808171402556e188f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 201 as libc::c_int,
            f: 1.33992804268466370262665421635e189f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 202 as libc::c_int,
            f: 2.38976166709580612772506233164e190f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 203 as libc::c_int,
            f: 2.72005392664986731633210805920e191f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 204 as libc::c_int,
            f: 4.8751138008754445005591271565e192f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 205 as libc::c_int,
            f: 5.5761105496322279984808215214e193f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 206 as libc::c_int,
            f: 1.00427344298034156711518019425e195f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 207 as libc::c_int,
            f: 1.15425488377387119568553005492e196f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 208 as libc::c_int,
            f: 2.08888876139911045959957480403e197f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 209 as libc::c_int,
            f: 2.41239270708739079898275781478e198f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 210 as libc::c_int,
            f: 4.3866663989381319651591070885e199f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 211 as libc::c_int,
            f: 5.0901486119543945858536189892e200f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 212 as libc::c_int,
            f: 9.2997327657488397661373070276e201f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 213 as libc::c_int,
            f: 1.08420165434628604678682084470e203f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 214 as libc::c_int,
            f: 1.99014281187025170995338370390e204f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 215 as libc::c_int,
            f: 2.33103355684451500059166481610e205f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 216 as libc::c_int,
            f: 4.2987084736397436934993088004e206f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 217 as libc::c_int,
            f: 5.0583428183525975512839126509e207f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 218 as libc::c_int,
            f: 9.3711844725346412518284931849e208f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 219 as libc::c_int,
            f: 1.10777707721921886373117687056e210f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 220 as libc::c_int,
            f: 2.06166058395762107540226850068e211f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 221 as libc::c_int,
            f: 2.44818734065447368884590088393e212f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 222 as libc::c_int,
            f: 4.5768864963859187873930360715e213f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 223 as libc::c_int,
            f: 5.4594577696594763261263589712e214f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 224 as libc::c_int,
            f: 1.02522257519044580837604008002e216f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 225 as libc::c_int,
            f: 1.22837799817338217337843076851e217f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 226 as libc::c_int,
            f: 2.31700301993040752692985058084e218f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 227 as libc::c_int,
            f: 2.78841805585357753356903784452e219f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 228 as libc::c_int,
            f: 5.2827668854413291614000593243e220f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 229 as libc::c_int,
            f: 6.3854773479046925518730966640e221f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 230 as libc::c_int,
            f: 1.21503638365150570712201364459e223f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 231 as libc::c_int,
            f: 1.47504526736598397948268532937e224f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 232 as libc::c_int,
            f: 2.81888441007149324052307165546e225f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 233 as libc::c_int,
            f: 3.4368554729627426721946568174e226f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 234 as libc::c_int,
            f: 6.5961895195672941828239876738e227f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 235 as libc::c_int,
            f: 8.0766103614624452796574435210e228f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 236 as libc::c_int,
            f: 1.55670072661788142714646109101e230f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 237 as libc::c_int,
            f: 1.91415665566659953127881411447e231f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 238 as libc::c_int,
            f: 3.7049477293505577966085773966e232f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 239 as libc::c_int,
            f: 4.5748344070431728797563657336e233f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 240 as libc::c_int,
            f: 8.8918745504413387118605857518e234f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 241 as libc::c_int,
            f: 1.10253509209740466402128414180e236f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 242 as libc::c_int,
            f: 2.15183364120680396827026175195e237f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 243 as libc::c_int,
            f: 2.67916027379669333357172046456e238f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 244 as libc::c_int,
            f: 5.2504740845446016825794386748e239f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 245 as libc::c_int,
            f: 6.5639426708018986672507151382e240f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 246 as libc::c_int,
            f: 1.29161662479797201391454191399e242f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 247 as libc::c_int,
            f: 1.62129383968806897081092663913e243f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 248 as libc::c_int,
            f: 3.2032092294989705945080639467e244f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 249 as libc::c_int,
            f: 4.0370216608232917373192073314e245f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 250 as libc::c_int,
            f: 8.0080230737474264862701598667e246f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 251 as libc::c_int,
            f: 1.01329243686664622606712104019e248f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 252 as libc::c_int,
            f: 2.01802181458435147454008028642e249f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 253 as libc::c_int,
            f: 2.56362986527261495194981623168e250f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 254 as libc::c_int,
            f: 5.1257754090442527453318039275e251f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 255 as libc::c_int,
            f: 6.5372561564451681274720313908e252f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 256 as libc::c_int,
            f: 1.31219850471532870280494180544e254f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 257 as libc::c_int,
            f: 1.68007483220640820876031206743e255f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 258 as libc::c_int,
            f: 3.3854721421655480532367498580e256f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 259 as libc::c_int,
            f: 4.3513938154145972606892082546e257f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 260 as libc::c_int,
            f: 8.8022275696304249384155496309e258f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 261 as libc::c_int,
            f: 1.13571378582320988503988335446e260f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 262 as libc::c_int,
            f: 2.30618362324317133386487400329e261f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 263 as libc::c_int,
            f: 2.98692725671504199765489322224e262f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 264 as libc::c_int,
            f: 6.0883247653619723214032673687e263f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 265 as libc::c_int,
            f: 7.9153572302948612937854670389e264f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 266 as libc::c_int,
            f: 1.61949438758628463749326912007e266f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 267 as libc::c_int,
            f: 2.11340038048872796544071969939e267f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 268 as libc::c_int,
            f: 4.3402449587312428284819612418e268f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 269 as libc::c_int,
            f: 5.6850470235146782270355359914e269f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 270 as libc::c_int,
            f: 1.17186613885743556369012953528e271f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 271 as libc::c_int,
            f: 1.54064774337247779952663025366e272f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 272 as libc::c_int,
            f: 3.1874758976922247332371523360e273f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 273 as libc::c_int,
            f: 4.2059683394068643927077005925e274f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 274 as libc::c_int,
            f: 8.7336839596766957690697974006e275f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 275 as libc::c_int,
            f: 1.15664129333688770799461766294e277f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 276 as libc::c_int,
            f: 2.41049677287076803226326408256e278f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 277 as libc::c_int,
            f: 3.2038963825431789511450909263e279f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 278 as libc::c_int,
            f: 6.7011810285807351296918741495e280f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 279 as libc::c_int,
            f: 8.9388709072954692736948036845e281f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 280 as libc::c_int,
            f: 1.87633068800260583631372476186e283f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 281 as libc::c_int,
            f: 2.51182272495002686590823983534e284f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 282 as libc::c_int,
            f: 5.2912525401673484584047038284e285f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 283 as libc::c_int,
            f: 7.1084583116085760305203187340e286f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 284 as libc::c_int,
            f: 1.50271572140752696218693588728e288f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 285 as libc::c_int,
            f: 2.02591061880844416869829083919e289f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 286 as libc::c_int,
            f: 4.2977669632255271118546366376e290f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 287 as libc::c_int,
            f: 5.8143634759802347641640947085e291f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 288 as libc::c_int,
            f: 1.23775688540895180821413535163e293f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 289 as libc::c_int,
            f: 1.68035104455828784684342337075e294f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 290 as libc::c_int,
            f: 3.5894949676859602438209925197e295f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 291 as libc::c_int,
            f: 4.8898215396646176343143620089e296f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 292 as libc::c_int,
            f: 1.04813253056430039119572981576e298f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 293 as libc::c_int,
            f: 1.43271771112173296685410806860e299f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 294 as libc::c_int,
            f: 3.08150963985904315011544565835e300f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 295 as libc::c_int,
            f: 4.2265172478091122522196188024e301f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 296 as libc::c_int,
            f: 9.1212685339827677243417191487e302f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            n: 297 as libc::c_int,
            f: 1.25527562259930633890922678431e304f64,
            i: 0 as libc::c_int as libc::c_long,
        };
        init
    },
];
static mut gstar_a_data: [libc::c_double; 30] = [
    2.16786447866463034423060819465f64,
    -0.05533249018745584258035832802f64,
    0.01800392431460719960888319748f64,
    -0.00580919269468937714480019814f64,
    0.00186523689488400339978881560f64,
    -0.00059746524113955531852595159f64,
    0.00019125169907783353925426722f64,
    -0.00006124996546944685735909697f64,
    0.00001963889633130842586440945f64,
    -6.3067741254637180272515795142e-06f64,
    2.0288698405861392526872789863e-06f64,
    -6.5384896660838465981983750582e-07f64,
    2.1108698058908865476480734911e-07f64,
    -6.8260714912274941677892994580e-08f64,
    2.2108560875880560555583978510e-08f64,
    -7.1710331930255456643627187187e-09f64,
    2.3290892983985406754602564745e-09f64,
    -7.5740371598505586754890405359e-10f64,
    2.4658267222594334398525312084e-10f64,
    -8.0362243171659883803428749516e-11f64,
    2.6215616826341594653521346229e-11f64,
    -8.5596155025948750540420068109e-12f64,
    2.7970831499487963614315315444e-12f64,
    -9.1471771211886202805502562414e-13f64,
    2.9934720198063397094916415927e-13f64,
    -9.8026575909753445931073620469e-14f64,
    3.2116773667767153777571410671e-14f64,
    -1.0518035333878147029650507254e-14f64,
    3.4144405720185253938994854173e-15f64,
    -1.0115153943081187052322643819e-15f64,
];
static mut gstar_a_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: gstar_a_data.as_ptr() as *mut _,
            order: 29 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 17 as libc::c_int,
        };
        init
    }
};
static mut gstar_b_data: [libc::c_double; 30] = [
    0.0057502277273114339831606096782f64,
    0.0004496689534965685038254147807f64,
    -0.0001672763153188717308905047405f64,
    0.0000615137014913154794776670946f64,
    -0.0000223726551711525016380862195f64,
    8.0507405356647954540694800545e-06f64,
    -2.8671077107583395569766746448e-06f64,
    1.0106727053742747568362254106e-06f64,
    -3.5265558477595061262310873482e-07f64,
    1.2179216046419401193247254591e-07f64,
    -4.1619640180795366971160162267e-08f64,
    1.4066283500795206892487241294e-08f64,
    -4.6982570380537099016106141654e-09f64,
    1.5491248664620612686423108936e-09f64,
    -5.0340936319394885789686867772e-10f64,
    1.6084448673736032249959475006e-10f64,
    -5.0349733196835456497619787559e-11f64,
    1.5357154939762136997591808461e-11f64,
    -4.5233809655775649997667176224e-12f64,
    1.2664429179254447281068538964e-12f64,
    -3.2648287937449326771785041692e-13f64,
    7.1528272726086133795579071407e-14f64,
    -9.4831735252566034505739531258e-15f64,
    -2.3124001991413207293120906691e-15f64,
    2.8406613277170391482590129474e-15f64,
    -1.7245370321618816421281770927e-15f64,
    8.6507923128671112154695006592e-16f64,
    -3.9506563665427555895391869919e-16f64,
    1.6779342132074761078792361165e-16f64,
    -6.0483153034414765129837716260e-17f64,
];
static mut gstar_b_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: gstar_b_data.as_ptr() as *mut _,
            order: 29 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 18 as libc::c_int,
        };
        init
    }
};
static mut lanczos_7_c: [libc::c_double; 9] = [
    0.99999999999980993227684700473478f64,
    676.520368121885098567009190444019f64,
    -1259.13921672240287047156078755283f64,
    771.3234287776530788486528258894f64,
    -176.61502916214059906584551354f64,
    12.507343278686904814458936853f64,
    -0.13857109526572011689554707f64,
    9.984369578019570859563e-6f64,
    1.50563273514931155834e-7f64,
];
unsafe extern "C" fn lngamma_lanczos_complex(
    mut zr: libc::c_double,
    mut zi: libc::c_double,
    mut yr: *mut gsl_sf_result,
    mut yi: *mut gsl_sf_result,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut log1_r: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut log1_i: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut logAg_r: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut logAg_i: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut Ag_r: libc::c_double = 0.;
    let mut Ag_i: libc::c_double = 0.;
    let mut yi_tmp_val: libc::c_double = 0.;
    let mut yi_tmp_err: libc::c_double = 0.;
    zr -= 1.0f64;
    Ag_r = lanczos_7_c[0 as libc::c_int as usize];
    Ag_i = 0.0f64;
    k = 1 as libc::c_int;
    while k <= 8 as libc::c_int {
        let mut R: libc::c_double = zr + k as libc::c_double;
        let mut I: libc::c_double = zi;
        let mut a: libc::c_double = lanczos_7_c[k as usize] / (R * R + I * I);
        Ag_r += a * R;
        Ag_i -= a * I;
        k += 1;
        k;
    }
    gsl_sf_complex_log_e(zr + 7.5f64, zi, &mut log1_r, &mut log1_i);
    gsl_sf_complex_log_e(Ag_r, Ag_i, &mut logAg_r, &mut logAg_i);
    (*yr)
        .val = (zr + 0.5f64) * log1_r.val - zi * log1_i.val - (zr + 7.5f64)
        + 0.9189385332046727418f64 + logAg_r.val;
    (*yi).val = zi * log1_r.val + (zr + 0.5f64) * log1_i.val - zi + logAg_i.val;
    (*yr).err = 4.0f64 * 2.2204460492503131e-16f64 * fabs((*yr).val);
    (*yi).err = 4.0f64 * 2.2204460492503131e-16f64 * fabs((*yi).val);
    yi_tmp_val = (*yi).val;
    yi_tmp_err = (*yi).err;
    gsl_sf_angle_restrict_symm_err_e(yi_tmp_val, yi);
    (*yi).err += yi_tmp_err;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn lngamma_lanczos(
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut Ag: libc::c_double = 0.;
    let mut term1: libc::c_double = 0.;
    let mut term2: libc::c_double = 0.;
    x -= 1.0f64;
    Ag = lanczos_7_c[0 as libc::c_int as usize];
    k = 1 as libc::c_int;
    while k <= 8 as libc::c_int {
        Ag += lanczos_7_c[k as usize] / (x + k as libc::c_double);
        k += 1;
        k;
    }
    term1 = (x + 0.5f64) * log((x + 7.5f64) / 2.7182818284590452354f64);
    term2 = 0.9189385332046727418f64 + log(Ag);
    (*result).val = term1 + (term2 - 7.0f64);
    (*result)
        .err = 2.0f64 * 2.2204460492503131e-16f64 * (fabs(term1) + fabs(term2) + 7.0f64);
    (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn lngamma_sgn_0(
    mut eps: libc::c_double,
    mut lng: *mut gsl_sf_result,
    mut sgn: *mut libc::c_double,
) -> libc::c_int {
    let c1: libc::c_double = -0.07721566490153286061f64;
    let c2: libc::c_double = -0.01094400467202744461f64;
    let c3: libc::c_double = 0.09252092391911371098f64;
    let c4: libc::c_double = -0.01827191316559981266f64;
    let c5: libc::c_double = 0.01800493109685479790f64;
    let c6: libc::c_double = -0.00685088537872380685f64;
    let c7: libc::c_double = 0.00399823955756846603f64;
    let c8: libc::c_double = -0.00189430621687107802f64;
    let c9: libc::c_double = 0.00097473237804513221f64;
    let c10: libc::c_double = -0.00048434392722255893f64;
    let g6: libc::c_double = c6 + eps * (c7 + eps * (c8 + eps * (c9 + eps * c10)));
    let g: libc::c_double = eps
        * (c1 + eps * (c2 + eps * (c3 + eps * (c4 + eps * (c5 + eps * g6)))));
    let gee: libc::c_double = g + 1.0f64 / (1.0f64 + eps) + 0.5f64 * eps;
    (*lng).val = log(gee / fabs(eps));
    (*lng).err = 4.0f64 * 2.2204460492503131e-16f64 * fabs((*lng).val);
    *sgn = (if eps >= 0.0f64 { 1 as libc::c_int } else { -(1 as libc::c_int) })
        as libc::c_double;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn lngamma_sgn_sing(
    mut N: libc::c_int,
    mut eps: libc::c_double,
    mut lng: *mut gsl_sf_result,
    mut sgn: *mut libc::c_double,
) -> libc::c_int {
    if eps == 0.0f64 {
        (*lng).val = 0.0f64;
        (*lng).err = 0.0f64;
        *sgn = 0.0f64;
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"gamma.c\0" as *const u8 as *const libc::c_char,
            769 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if N == 1 as libc::c_int {
        let c0: libc::c_double = 0.07721566490153286061f64;
        let c1: libc::c_double = 0.08815966957356030521f64;
        let c2: libc::c_double = -0.00436125434555340577f64;
        let c3: libc::c_double = 0.01391065882004640689f64;
        let c4: libc::c_double = -0.00409427227680839100f64;
        let c5: libc::c_double = 0.00275661310191541584f64;
        let c6: libc::c_double = -0.00124162645565305019f64;
        let c7: libc::c_double = 0.00065267976121802783f64;
        let c8: libc::c_double = -0.00032205261682710437f64;
        let c9: libc::c_double = 0.00016229131039545456f64;
        let g5: libc::c_double = c5 + eps * (c6 + eps * (c7 + eps * (c8 + eps * c9)));
        let g: libc::c_double = eps
            * (c0 + eps * (c1 + eps * (c2 + eps * (c3 + eps * (c4 + eps * g5)))));
        let gam_e: libc::c_double = g - 1.0f64
            - 0.5f64 * eps * (1.0f64 + 3.0f64 * eps) / (1.0f64 - eps * eps);
        (*lng).val = log(fabs(gam_e) / fabs(eps));
        (*lng).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*lng).val);
        *sgn = if eps > 0.0f64 { -1.0f64 } else { 1.0f64 };
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut g_0: libc::c_double = 0.;
        let cs1: libc::c_double = -1.6449340668482264365f64;
        let cs2: libc::c_double = 0.8117424252833536436f64;
        let cs3: libc::c_double = -0.1907518241220842137f64;
        let cs4: libc::c_double = 0.0261478478176548005f64;
        let cs5: libc::c_double = -0.0023460810354558236f64;
        let e2: libc::c_double = eps * eps;
        let sin_ser: libc::c_double = 1.0f64
            + e2 * (cs1 + e2 * (cs2 + e2 * (cs3 + e2 * (cs4 + e2 * cs5))));
        let mut aeps: libc::c_double = fabs(eps);
        let mut c1_0: libc::c_double = 0.;
        let mut c2_0: libc::c_double = 0.;
        let mut c3_0: libc::c_double = 0.;
        let mut c4_0: libc::c_double = 0.;
        let mut c5_0: libc::c_double = 0.;
        let mut c6_0: libc::c_double = 0.;
        let mut c7_0: libc::c_double = 0.;
        let mut lng_ser: libc::c_double = 0.;
        let mut c0_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut psi_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut psi_1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut psi_2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut psi_3: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut psi_4: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut psi_5: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut psi_6: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        psi_2.val = 0.0f64;
        psi_3.val = 0.0f64;
        psi_4.val = 0.0f64;
        psi_5.val = 0.0f64;
        psi_6.val = 0.0f64;
        gsl_sf_lnfact_e(N as libc::c_uint, &mut c0_0);
        gsl_sf_psi_int_e(N + 1 as libc::c_int, &mut psi_0);
        gsl_sf_psi_1_int_e(N + 1 as libc::c_int, &mut psi_1);
        if aeps > 0.00001f64 {
            gsl_sf_psi_n_e(2 as libc::c_int, N as libc::c_double + 1.0f64, &mut psi_2);
        }
        if aeps > 0.0002f64 {
            gsl_sf_psi_n_e(3 as libc::c_int, N as libc::c_double + 1.0f64, &mut psi_3);
        }
        if aeps > 0.001f64 {
            gsl_sf_psi_n_e(4 as libc::c_int, N as libc::c_double + 1.0f64, &mut psi_4);
        }
        if aeps > 0.005f64 {
            gsl_sf_psi_n_e(5 as libc::c_int, N as libc::c_double + 1.0f64, &mut psi_5);
        }
        if aeps > 0.01f64 {
            gsl_sf_psi_n_e(6 as libc::c_int, N as libc::c_double + 1.0f64, &mut psi_6);
        }
        c1_0 = psi_0.val;
        c2_0 = psi_1.val / 2.0f64;
        c3_0 = psi_2.val / 6.0f64;
        c4_0 = psi_3.val / 24.0f64;
        c5_0 = psi_4.val / 120.0f64;
        c6_0 = psi_5.val / 720.0f64;
        c7_0 = psi_6.val / 5040.0f64;
        lng_ser = c0_0.val
            - eps
                * (c1_0
                    - eps
                        * (c2_0
                            - eps
                                * (c3_0
                                    - eps
                                        * (c4_0 - eps * (c5_0 - eps * (c6_0 - eps * c7_0))))));
        g_0 = -lng_ser - log(sin_ser);
        (*lng).val = g_0 - log(fabs(eps));
        (*lng)
            .err = c0_0.err
            + 2.0f64 * 2.2204460492503131e-16f64 * (fabs(g_0) + fabs((*lng).val));
        *sgn = (if N & 1 as libc::c_int != 0 { -1.0f64 } else { 1.0f64 })
            * (if eps > 0.0f64 { 1.0f64 } else { -1.0f64 });
        return GSL_SUCCESS as libc::c_int;
    };
}
#[inline]
unsafe extern "C" fn lngamma_1_pade(
    eps: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let n1: libc::c_double = -1.0017419282349508699871138440f64;
    let n2: libc::c_double = 1.7364839209922879823280541733f64;
    let d1: libc::c_double = 1.2433006018858751556055436011f64;
    let d2: libc::c_double = 5.0456274100274010152489597514f64;
    let num: libc::c_double = (eps + n1) * (eps + n2);
    let den: libc::c_double = (eps + d1) * (eps + d2);
    let pade: libc::c_double = 2.0816265188662692474880210318f64 * num / den;
    let c0: libc::c_double = 0.004785324257581753f64;
    let c1: libc::c_double = -0.01192457083645441f64;
    let c2: libc::c_double = 0.01931961413960498f64;
    let c3: libc::c_double = -0.02594027398725020f64;
    let c4: libc::c_double = 0.03141928755021455f64;
    let eps5: libc::c_double = eps * eps * eps * eps * eps;
    let corr: libc::c_double = eps5
        * (c0 + eps * (c1 + eps * (c2 + eps * (c3 + c4 * eps))));
    (*result).val = eps * (pade + corr);
    (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
    return GSL_SUCCESS as libc::c_int;
}
#[inline]
unsafe extern "C" fn lngamma_2_pade(
    eps: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let n1: libc::c_double = 1.000895834786669227164446568f64;
    let n2: libc::c_double = 4.209376735287755081642901277f64;
    let d1: libc::c_double = 2.618851904903217274682578255f64;
    let d2: libc::c_double = 10.85766559900983515322922936f64;
    let num: libc::c_double = (eps + n1) * (eps + n2);
    let den: libc::c_double = (eps + d1) * (eps + d2);
    let pade: libc::c_double = 2.85337998765781918463568869f64 * num / den;
    let c0: libc::c_double = 0.0001139406357036744f64;
    let c1: libc::c_double = -0.0001365435269792533f64;
    let c2: libc::c_double = 0.0001067287169183665f64;
    let c3: libc::c_double = -0.0000693271800931282f64;
    let c4: libc::c_double = 0.0000407220927867950f64;
    let eps5: libc::c_double = eps * eps * eps * eps * eps;
    let corr: libc::c_double = eps5
        * (c0 + eps * (c1 + eps * (c2 + eps * (c3 + c4 * eps))));
    (*result).val = eps * (pade + corr);
    (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn gammastar_ser(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let y: libc::c_double = 1.0f64 / (x * x);
    let c0: libc::c_double = 1.0f64 / 12.0f64;
    let c1: libc::c_double = -1.0f64 / 360.0f64;
    let c2: libc::c_double = 1.0f64 / 1260.0f64;
    let c3: libc::c_double = -1.0f64 / 1680.0f64;
    let c4: libc::c_double = 1.0f64 / 1188.0f64;
    let c5: libc::c_double = -691.0f64 / 360360.0f64;
    let c6: libc::c_double = 1.0f64 / 156.0f64;
    let c7: libc::c_double = -3617.0f64 / 122400.0f64;
    let ser: libc::c_double = c0
        + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * (c5 + y * (c6 + y * c7))))));
    (*result).val = exp(ser / x);
    (*result)
        .err = 2.0f64 * 2.2204460492503131e-16f64 * (*result).val
        * GSL_MAX_DBL(1.0f64, ser / x);
    return GSL_SUCCESS as libc::c_int;
}
static mut gamma_5_10_data: [libc::c_double; 24] = [
    -1.5285594096661578881275075214f64,
    4.8259152300595906319768555035f64,
    0.2277712320977614992970601978f64,
    -0.0138867665685617873604917300f64,
    0.0012704876495201082588139723f64,
    -0.0001393841240254993658962470f64,
    0.0000169709242992322702260663f64,
    -2.2108528820210580075775889168e-06f64,
    3.0196602854202309805163918716e-07f64,
    -4.2705675000079118380587357358e-08f64,
    6.2026423818051402794663551945e-09f64,
    -9.1993973208880910416311405656e-10f64,
    1.3875551258028145778301211638e-10f64,
    -2.1218861491906788718519522978e-11f64,
    3.2821736040381439555133562600e-12f64,
    -5.1260001009953791220611135264e-13f64,
    8.0713532554874636696982146610e-14f64,
    -1.2798522376569209083811628061e-14f64,
    2.0417711600852502310258808643e-15f64,
    -3.2745239502992355776882614137e-16f64,
    5.2759418422036579482120897453e-17f64,
    -8.5354147151695233960425725513e-18f64,
    1.3858639703888078291599886143e-18f64,
    -2.2574398807738626571560124396e-19f64,
];
static mut gamma_5_10_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: gamma_5_10_data.as_ptr() as *mut _,
            order: 23 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 11 as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn gamma_xgthalf(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x == 0.5f64 {
        (*result).val = 1.77245385090551602729817f64;
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else if x <= 170 as libc::c_int as libc::c_double + 1.0f64 && x == floor(x) {
        let mut n: libc::c_int = floor(x) as libc::c_int;
        (*result).val = fact_table[(n - 1 as libc::c_int) as usize].f;
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else if fabs(x - 1.0f64) < 0.01f64 {
        let eps: libc::c_double = x - 1.0f64;
        let c1: libc::c_double = 0.4227843350984671394f64;
        let c2: libc::c_double = -0.01094400467202744461f64;
        let c3: libc::c_double = 0.09252092391911371098f64;
        let c4: libc::c_double = -0.018271913165599812664f64;
        let c5: libc::c_double = 0.018004931096854797895f64;
        let c6: libc::c_double = -0.006850885378723806846f64;
        let c7: libc::c_double = 0.003998239557568466030f64;
        (*result)
            .val = 1.0f64 / x
            + eps
                * (c1
                    + eps
                        * (c2
                            + eps
                                * (c3 + eps * (c4 + eps * (c5 + eps * (c6 + eps * c7))))));
        (*result).err = 2.2204460492503131e-16f64;
        return GSL_SUCCESS as libc::c_int;
    } else if fabs(x - 2.0f64) < 0.01f64 {
        let eps_0: libc::c_double = x - 2.0f64;
        let c1_0: libc::c_double = 0.4227843350984671394f64;
        let c2_0: libc::c_double = 0.4118403304264396948f64;
        let c3_0: libc::c_double = 0.08157691924708626638f64;
        let c4_0: libc::c_double = 0.07424901075351389832f64;
        let c5_0: libc::c_double = -0.00026698206874501476832f64;
        let c6_0: libc::c_double = 0.011154045718130991049f64;
        let c7_0: libc::c_double = -0.002852645821155340816f64;
        let c8: libc::c_double = 0.0021039333406973880085f64;
        (*result)
            .val = 1.0f64
            + eps_0
                * (c1_0
                    + eps_0
                        * (c2_0
                            + eps_0
                                * (c3_0
                                    + eps_0
                                        * (c4_0
                                            + eps_0
                                                * (c5_0 + eps_0 * (c6_0 + eps_0 * (c7_0 + eps_0 * c8)))))));
        (*result).err = 2.2204460492503131e-16f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x < 5.0f64 {
        let mut lg: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        lngamma_lanczos(x, &mut lg);
        (*result).val = exp(lg.val);
        (*result).err = (*result).val * (lg.err + 2.0f64 * 2.2204460492503131e-16f64);
        return GSL_SUCCESS as libc::c_int;
    } else if x < 10.0f64 {
        let gamma_8: libc::c_double = 5040.0f64;
        let t: libc::c_double = (2.0f64 * x - 15.0f64) / 5.0f64;
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&gamma_5_10_cs, t, &mut c);
        (*result).val = exp(c.val) * gamma_8;
        (*result).err = (*result).val * c.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else if x < 171.0f64 {
        let mut p: libc::c_double = pow(x, 0.5f64 * x);
        let mut e: libc::c_double = exp(-x);
        let mut q: libc::c_double = p * e * p;
        let mut pre: libc::c_double = 1.41421356237309504880f64
            * 1.77245385090551602729816748334f64 * q / sqrt(x);
        let mut gstar: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_gs: libc::c_int = gammastar_ser(x, &mut gstar);
        (*result).val = pre * gstar.val;
        (*result).err = (x + 2.5f64) * 2.2204460492503131e-16f64 * (*result).val;
        return stat_gs;
    } else {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"gamma.c\0" as *const u8 as *const libc::c_char,
            1106 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_lngamma_e(
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if fabs(x - 1.0f64) < 0.01f64 {
        let mut stat: libc::c_int = lngamma_1_pade(x - 1.0f64, result);
        (*result).err *= 1.0f64 / (2.2204460492503131e-16f64 + fabs(x - 1.0f64));
        return stat;
    } else if fabs(x - 2.0f64) < 0.01f64 {
        let mut stat_0: libc::c_int = lngamma_2_pade(x - 2.0f64, result);
        (*result).err *= 1.0f64 / (2.2204460492503131e-16f64 + fabs(x - 2.0f64));
        return stat_0;
    } else if x >= 0.5f64 {
        return lngamma_lanczos(x, result)
    } else if x == 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"gamma.c\0" as *const u8 as *const libc::c_char,
            1138 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if fabs(x) < 0.02f64 {
        let mut sgn: libc::c_double = 0.;
        return lngamma_sgn_0(x, result, &mut sgn);
    } else if x > -0.5f64 / (2.2204460492503131e-16f64 * 3.14159265358979323846f64) {
        let mut z: libc::c_double = 1.0f64 - x;
        let mut s: libc::c_double = sin(3.14159265358979323846f64 * z);
        let mut as_0: libc::c_double = fabs(s);
        if s == 0.0f64 {
            (*result).val = ::std::f32::NAN as libc::c_double;
            (*result).err = ::std::f32::NAN as libc::c_double;
            gsl_error(
                b"domain error\0" as *const u8 as *const libc::c_char,
                b"gamma.c\0" as *const u8 as *const libc::c_char,
                1152 as libc::c_int,
                GSL_EDOM as libc::c_int,
            );
            return GSL_EDOM as libc::c_int;
        } else if as_0 < 3.14159265358979323846f64 * 0.015f64 {
            if x
                < (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_double
                    + 2.0f64
            {
                (*result).val = 0.0f64;
                (*result).err = 0.0f64;
                gsl_error(
                    b"error\0" as *const u8 as *const libc::c_char,
                    b"gamma.c\0" as *const u8 as *const libc::c_char,
                    1159 as libc::c_int,
                    GSL_EROUND as libc::c_int,
                );
                return GSL_EROUND as libc::c_int;
            } else {
                let mut N: libc::c_int = -((x - 0.5f64) as libc::c_int);
                let mut eps: libc::c_double = x + N as libc::c_double;
                let mut sgn_0: libc::c_double = 0.;
                return lngamma_sgn_sing(N, eps, result, &mut sgn_0);
            }
        } else {
            let mut lg_z: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            lngamma_lanczos(z, &mut lg_z);
            (*result).val = 1.14472988584940017414342735135f64 - (log(as_0) + lg_z.val);
            (*result)
                .err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val)
                + lg_z.err;
            return GSL_SUCCESS as libc::c_int;
        }
    } else {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"gamma.c\0" as *const u8 as *const libc::c_char,
            1180 as libc::c_int,
            GSL_EROUND as libc::c_int,
        );
        return GSL_EROUND as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_lngamma_sgn_e(
    mut x: libc::c_double,
    mut result_lg: *mut gsl_sf_result,
    mut sgn: *mut libc::c_double,
) -> libc::c_int {
    if fabs(x - 1.0f64) < 0.01f64 {
        let mut stat: libc::c_int = lngamma_1_pade(x - 1.0f64, result_lg);
        (*result_lg).err *= 1.0f64 / (2.2204460492503131e-16f64 + fabs(x - 1.0f64));
        *sgn = 1.0f64;
        return stat;
    } else if fabs(x - 2.0f64) < 0.01f64 {
        let mut stat_0: libc::c_int = lngamma_2_pade(x - 2.0f64, result_lg);
        (*result_lg).err *= 1.0f64 / (2.2204460492503131e-16f64 + fabs(x - 2.0f64));
        *sgn = 1.0f64;
        return stat_0;
    } else if x >= 0.5f64 {
        *sgn = 1.0f64;
        return lngamma_lanczos(x, result_lg);
    } else if x == 0.0f64 {
        *sgn = 0.0f64;
        (*result_lg).val = ::std::f32::NAN as libc::c_double;
        (*result_lg).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"gamma.c\0" as *const u8 as *const libc::c_char,
            1205 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if fabs(x) < 0.02f64 {
        return lngamma_sgn_0(x, result_lg, sgn)
    } else if x > -0.5f64 / (2.2204460492503131e-16f64 * 3.14159265358979323846f64) {
        let mut z: libc::c_double = 1.0f64 - x;
        let mut s: libc::c_double = sin(3.14159265358979323846f64 * x);
        let mut as_0: libc::c_double = fabs(s);
        if s == 0.0f64 {
            *sgn = 0.0f64;
            (*result_lg).val = ::std::f32::NAN as libc::c_double;
            (*result_lg).err = ::std::f32::NAN as libc::c_double;
            gsl_error(
                b"domain error\0" as *const u8 as *const libc::c_char,
                b"gamma.c\0" as *const u8 as *const libc::c_char,
                1219 as libc::c_int,
                GSL_EDOM as libc::c_int,
            );
            return GSL_EDOM as libc::c_int;
        } else if as_0 < 3.14159265358979323846f64 * 0.015f64 {
            if x
                < (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_double
                    + 2.0f64
            {
                (*result_lg).val = 0.0f64;
                (*result_lg).err = 0.0f64;
                *sgn = 0.0f64;
                gsl_error(
                    b"error\0" as *const u8 as *const libc::c_char,
                    b"gamma.c\0" as *const u8 as *const libc::c_char,
                    1227 as libc::c_int,
                    GSL_EROUND as libc::c_int,
                );
                return GSL_EROUND as libc::c_int;
            } else {
                let mut N: libc::c_int = -((x - 0.5f64) as libc::c_int);
                let mut eps: libc::c_double = x + N as libc::c_double;
                return lngamma_sgn_sing(N, eps, result_lg, sgn);
            }
        } else {
            let mut lg_z: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            lngamma_lanczos(z, &mut lg_z);
            *sgn = if s > 0.0f64 { 1.0f64 } else { -1.0f64 };
            (*result_lg)
                .val = 1.14472988584940017414342735135f64 - (log(as_0) + lg_z.val);
            (*result_lg)
                .err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result_lg).val)
                + lg_z.err;
            return GSL_SUCCESS as libc::c_int;
        }
    } else {
        (*result_lg).val = 0.0f64;
        (*result_lg).err = 0.0f64;
        *sgn = 0.0f64;
        gsl_error(
            b"x too large to extract fraction part\0" as *const u8
                as *const libc::c_char,
            b"gamma.c\0" as *const u8 as *const libc::c_char,
            1249 as libc::c_int,
            GSL_EROUND as libc::c_int,
        );
        return GSL_EROUND as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_gamma_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x < 0.5f64 {
        let mut rint_x: libc::c_int = floor(x + 0.5f64) as libc::c_int;
        let mut f_x: libc::c_double = x - rint_x as libc::c_double;
        let mut sgn_gamma: libc::c_double = if rint_x & 1 as libc::c_int == 0 {
            1.0f64
        } else {
            -1.0f64
        };
        let mut sin_term: libc::c_double = sgn_gamma
            * sin(3.14159265358979323846f64 * f_x) / 3.14159265358979323846f64;
        if sin_term == 0.0f64 {
            (*result).val = ::std::f32::NAN as libc::c_double;
            (*result).err = ::std::f32::NAN as libc::c_double;
            gsl_error(
                b"domain error\0" as *const u8 as *const libc::c_char,
                b"gamma.c\0" as *const u8 as *const libc::c_char,
                1264 as libc::c_int,
                GSL_EDOM as libc::c_int,
            );
            return GSL_EDOM as libc::c_int;
        } else if x > -169.0f64 {
            let mut g: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            gamma_xgthalf(1.0f64 - x, &mut g);
            if fabs(sin_term) * g.val * 2.2250738585072014e-308f64 < 1.0f64 {
                (*result).val = 1.0f64 / (sin_term * g.val);
                (*result).err = fabs(g.err / g.val) * fabs((*result).val);
                (*result).err
                    += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
                return GSL_SUCCESS as libc::c_int;
            } else {
                (*result).val = 0.0f64;
                (*result).err = 2.2250738585072014e-308f64;
                gsl_error(
                    b"underflow\0" as *const u8 as *const libc::c_char,
                    b"gamma.c\0" as *const u8 as *const libc::c_char,
                    1276 as libc::c_int,
                    GSL_EUNDRFLW as libc::c_int,
                );
                return GSL_EUNDRFLW as libc::c_int;
            }
        } else {
            let mut lng: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut sgn: libc::c_double = 0.;
            let mut stat_lng: libc::c_int = gsl_sf_lngamma_sgn_e(x, &mut lng, &mut sgn);
            let mut stat_e: libc::c_int = gsl_sf_exp_mult_err_e(
                lng.val,
                lng.err,
                sgn,
                0.0f64,
                result,
            );
            return if stat_e != GSL_SUCCESS as libc::c_int {
                stat_e
            } else if stat_lng != GSL_SUCCESS as libc::c_int {
                stat_lng
            } else {
                GSL_SUCCESS as libc::c_int
            };
        }
    } else {
        return gamma_xgthalf(x, result)
    };
}
pub unsafe extern "C" fn gsl_sf_gammastar_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"gamma.c\0" as *const u8 as *const libc::c_char,
            1304 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x < 0.5f64 {
        let mut lg: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let stat_lg: libc::c_int = gsl_sf_lngamma_e(x, &mut lg);
        let lx: libc::c_double = log(x);
        let c: libc::c_double = 0.5f64
            * (0.69314718055994530942f64 + 1.14472988584940017414342735135f64);
        let lnr_val: libc::c_double = lg.val - (x - 0.5f64) * lx + x - c;
        let lnr_err: libc::c_double = lg.err
            + 2.0f64 * 2.2204460492503131e-16f64 * ((x + 0.5f64) * fabs(lx) + c);
        let stat_e: libc::c_int = gsl_sf_exp_err_e(lnr_val, lnr_err, result);
        return if stat_lg != GSL_SUCCESS as libc::c_int {
            stat_lg
        } else if stat_e != GSL_SUCCESS as libc::c_int {
            stat_e
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if x < 2.0f64 {
        let t: libc::c_double = 4.0f64 / 3.0f64 * (x - 0.5f64) - 1.0f64;
        return cheb_eval_e(&mut gstar_a_cs, t, result);
    } else if x < 10.0f64 {
        let t_0: libc::c_double = 0.25f64 * (x - 2.0f64) - 1.0f64;
        let mut c_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut gstar_b_cs, t_0, &mut c_0);
        (*result).val = c_0.val / (x * x) + 1.0f64 + 1.0f64 / (12.0f64 * x);
        (*result).err = c_0.err / (x * x);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < 1.0f64 / 1.2207031250000000e-04f64 {
        return gammastar_ser(x, result)
    } else if x < 1.0f64 / 2.2204460492503131e-16f64 {
        let xi: libc::c_double = 1.0f64 / x;
        (*result)
            .val = 1.0f64
            + xi / 12.0f64
                * (1.0f64
                    + xi / 24.0f64
                        * (1.0f64
                            - xi * (139.0f64 / 180.0f64 + 571.0f64 / 8640.0f64 * xi)));
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = 1.0f64;
        (*result).err = 1.0f64 / x;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_gammainv_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= 0.0f64 && x == floor(x) {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x < 0.5f64 {
        let mut lng: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sgn: libc::c_double = 0.;
        let mut stat_lng: libc::c_int = gsl_sf_lngamma_sgn_e(x, &mut lng, &mut sgn);
        if stat_lng == GSL_EDOM as libc::c_int {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return GSL_SUCCESS as libc::c_int;
        } else if stat_lng != GSL_SUCCESS as libc::c_int {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return stat_lng;
        } else {
            return gsl_sf_exp_mult_err_e(-lng.val, lng.err, sgn, 0.0f64, result)
        }
    } else {
        let mut g: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_g: libc::c_int = gamma_xgthalf(x, &mut g);
        if stat_g == GSL_EOVRFLW as libc::c_int {
            (*result).val = 0.0f64;
            (*result).err = 2.2250738585072014e-308f64;
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"gamma.c\0" as *const u8 as *const libc::c_char,
                1379 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        } else {
            (*result).val = 1.0f64 / g.val;
            (*result).err = fabs(g.err / g.val) * fabs((*result).val);
            (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            if fabs((*result).val) < 2.2250738585072014e-308f64 {
                gsl_error(
                    b"underflow\0" as *const u8 as *const libc::c_char,
                    b"gamma.c\0" as *const u8 as *const libc::c_char,
                    1385 as libc::c_int,
                    GSL_EUNDRFLW as libc::c_int,
                );
                return GSL_EUNDRFLW as libc::c_int;
            }
            return GSL_SUCCESS as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn gsl_sf_lngamma_complex_e(
    mut zr: libc::c_double,
    mut zi: libc::c_double,
    mut lnr: *mut gsl_sf_result,
    mut arg: *mut gsl_sf_result,
) -> libc::c_int {
    if zr <= 0.5f64 {
        let mut x: libc::c_double = 1.0f64 - zr;
        let mut y: libc::c_double = -zi;
        let mut a: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut b: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lnsin_r: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut lnsin_i: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_l: libc::c_int = lngamma_lanczos_complex(x, y, &mut a, &mut b);
        let mut stat_s: libc::c_int = gsl_sf_complex_logsin_e(
            3.14159265358979323846f64 * zr,
            3.14159265358979323846f64 * zi,
            &mut lnsin_r,
            &mut lnsin_i,
        );
        if stat_s == GSL_SUCCESS as libc::c_int {
            let mut stat_r: libc::c_int = 0;
            (*lnr).val = 1.14472988584940017414342735135f64 - lnsin_r.val - a.val;
            (*lnr)
                .err = lnsin_r.err + a.err
                + 2.0f64 * 2.2204460492503131e-16f64 * fabs((*lnr).val);
            (*arg).val = -lnsin_i.val - b.val;
            (*arg)
                .err = lnsin_i.err + b.err
                + 2.0f64 * 2.2204460492503131e-16f64 * fabs((*arg).val);
            stat_r = gsl_sf_angle_restrict_symm_e(&mut (*arg).val);
            return if stat_r != GSL_SUCCESS as libc::c_int {
                stat_r
            } else if stat_l != GSL_SUCCESS as libc::c_int {
                stat_l
            } else {
                GSL_SUCCESS as libc::c_int
            };
        } else {
            (*lnr).val = ::std::f32::NAN as libc::c_double;
            (*lnr).err = ::std::f32::NAN as libc::c_double;
            (*arg).val = ::std::f32::NAN as libc::c_double;
            (*arg).err = ::std::f32::NAN as libc::c_double;
            gsl_error(
                b"domain error\0" as *const u8 as *const libc::c_char,
                b"gamma.c\0" as *const u8 as *const libc::c_char,
                1417 as libc::c_int,
                GSL_EDOM as libc::c_int,
            );
            return GSL_EDOM as libc::c_int;
        }
    } else {
        return lngamma_lanczos_complex(zr, zi, lnr, arg)
    };
}
pub unsafe extern "C" fn gsl_sf_taylorcoeff_e(
    n: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x < 0.0f64 || n < 0 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"gamma.c\0" as *const u8 as *const libc::c_char,
            1432 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if n == 0 as libc::c_int {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if n == 1 as libc::c_int {
        (*result).val = x;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if x == 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let log2pi: libc::c_double = 1.14472988584940017414342735135f64
            + 0.69314718055994530942f64;
        let ln_test: libc::c_double = n as libc::c_double * (log(x) + 1.0f64) + 1.0f64
            - (n as libc::c_double + 0.5f64) * log(n as libc::c_double + 1.0f64)
            + 0.5f64 * log2pi;
        if ln_test < -7.0839641853226408e+02f64 + 1.0f64 {
            (*result).val = 0.0f64;
            (*result).err = 2.2250738585072014e-308f64;
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"gamma.c\0" as *const u8 as *const libc::c_char,
                1454 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        } else if ln_test > 7.0978271289338397e+02f64 - 1.0f64 {
            (*result).val = ::std::f32::INFINITY as libc::c_double;
            (*result).err = ::std::f32::INFINITY as libc::c_double;
            gsl_error(
                b"overflow\0" as *const u8 as *const libc::c_char,
                b"gamma.c\0" as *const u8 as *const libc::c_char,
                1457 as libc::c_int,
                GSL_EOVRFLW as libc::c_int,
            );
            return GSL_EOVRFLW as libc::c_int;
        } else {
            let mut product: libc::c_double = 1.0f64;
            let mut k: libc::c_int = 0;
            k = 1 as libc::c_int;
            while k <= n {
                product *= x / k as libc::c_double;
                k += 1;
                k;
            }
            (*result).val = product;
            (*result).err = n as libc::c_double * 2.2204460492503131e-16f64 * product;
            if fabs((*result).val) < 2.2250738585072014e-308f64 {
                gsl_error(
                    b"underflow\0" as *const u8 as *const libc::c_char,
                    b"gamma.c\0" as *const u8 as *const libc::c_char,
                    1467 as libc::c_int,
                    GSL_EUNDRFLW as libc::c_int,
                );
                return GSL_EUNDRFLW as libc::c_int;
            }
            return GSL_SUCCESS as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn gsl_sf_fact_e(
    n: libc::c_uint,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if n < 18 as libc::c_int as libc::c_uint {
        (*result).val = fact_table[n as usize].f;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if n <= 170 as libc::c_int as libc::c_uint {
        (*result).val = fact_table[n as usize].f;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"gamma.c\0" as *const u8 as *const libc::c_char,
            1489 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_doublefact_e(
    n: libc::c_uint,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if n < 26 as libc::c_int as libc::c_uint {
        (*result).val = doub_fact_table[n as usize].f;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if n <= 297 as libc::c_int as libc::c_uint {
        (*result).val = doub_fact_table[n as usize].f;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"gamma.c\0" as *const u8 as *const libc::c_char,
            1509 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_lnfact_e(
    n: libc::c_uint,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if n <= 170 as libc::c_int as libc::c_uint {
        (*result).val = log(fact_table[n as usize].f);
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        gsl_sf_lngamma_e(n as libc::c_double + 1.0f64, result);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_lndoublefact_e(
    n: libc::c_uint,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if n <= 297 as libc::c_int as libc::c_uint {
        (*result).val = log(doub_fact_table[n as usize].f);
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if n & 1 as libc::c_int as libc::c_uint != 0 {
        let mut lg: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        gsl_sf_lngamma_e(0.5f64 * (n as libc::c_double + 2.0f64), &mut lg);
        (*result)
            .val = 0.5f64 * (n as libc::c_double + 1.0f64) * 0.69314718055994530942f64
            - 0.5f64 * 1.14472988584940017414342735135f64 + lg.val;
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val) + lg.err;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut lg_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        gsl_sf_lngamma_e(0.5f64 * n as libc::c_double + 1.0f64, &mut lg_0);
        (*result)
            .val = 0.5f64 * n as libc::c_double * 0.69314718055994530942f64 + lg_0.val;
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val) + lg_0.err;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_lnchoose_e(
    mut n: libc::c_uint,
    mut m: libc::c_uint,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if m > n {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"gamma.c\0" as *const u8 as *const libc::c_char,
            1561 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if m == n || m == 0 as libc::c_int as libc::c_uint {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut nf: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut mf: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut nmmf: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        if m.wrapping_mul(2 as libc::c_int as libc::c_uint) > n {
            m = n.wrapping_sub(m);
        }
        gsl_sf_lnfact_e(n, &mut nf);
        gsl_sf_lnfact_e(m, &mut mf);
        gsl_sf_lnfact_e(n.wrapping_sub(m), &mut nmmf);
        (*result).val = nf.val - mf.val - nmmf.val;
        (*result).err = nf.err + mf.err + nmmf.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_choose_e(
    mut n: libc::c_uint,
    mut m: libc::c_uint,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if m > n {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"gamma.c\0" as *const u8 as *const libc::c_char,
            1587 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if m == n || m == 0 as libc::c_int as libc::c_uint {
        (*result).val = 1.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if n <= 170 as libc::c_int as libc::c_uint {
        (*result)
            .val = fact_table[n as usize].f / fact_table[m as usize].f
            / fact_table[n.wrapping_sub(m) as usize].f;
        (*result).err = 6.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        if m.wrapping_mul(2 as libc::c_int as libc::c_uint) < n {
            m = n.wrapping_sub(m);
        }
        if n.wrapping_sub(m) < 64 as libc::c_int as libc::c_uint {
            let mut prod: libc::c_double = 1.0f64;
            let mut k: libc::c_uint = 0;
            k = n;
            while k >= m.wrapping_add(1 as libc::c_int as libc::c_uint) {
                let mut tk: libc::c_double = k as libc::c_double
                    / k.wrapping_sub(m) as libc::c_double;
                if tk > 1.7976931348623157e+308f64 / prod {
                    (*result).val = ::std::f32::INFINITY as libc::c_double;
                    (*result).err = ::std::f32::INFINITY as libc::c_double;
                    gsl_error(
                        b"overflow\0" as *const u8 as *const libc::c_char,
                        b"gamma.c\0" as *const u8 as *const libc::c_char,
                        1609 as libc::c_int,
                        GSL_EOVRFLW as libc::c_int,
                    );
                    return GSL_EOVRFLW as libc::c_int;
                }
                prod *= tk;
                k = k.wrapping_sub(1);
                k;
            }
            (*result).val = prod;
            (*result)
                .err = 2.0f64 * 2.2204460492503131e-16f64 * prod
                * fabs(n.wrapping_sub(m) as libc::c_double);
            return GSL_SUCCESS as libc::c_int;
        } else {
            let mut lc: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let stat_lc: libc::c_int = gsl_sf_lnchoose_e(n, m, &mut lc);
            let stat_e: libc::c_int = gsl_sf_exp_err_e(lc.val, lc.err, result);
            return if stat_lc != GSL_SUCCESS as libc::c_int {
                stat_lc
            } else if stat_e != GSL_SUCCESS as libc::c_int {
                stat_e
            } else {
                GSL_SUCCESS as libc::c_int
            };
        }
    };
}
pub unsafe extern "C" fn gsl_sf_fact(n: libc::c_uint) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_fact_e(n, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_fact_e(n, &result)\0" as *const u8 as *const libc::c_char,
            b"gamma.c\0" as *const u8 as *const libc::c_char,
            1634 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_lnfact(n: libc::c_uint) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_lnfact_e(n, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_lnfact_e(n, &result)\0" as *const u8 as *const libc::c_char,
            b"gamma.c\0" as *const u8 as *const libc::c_char,
            1639 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_doublefact(n: libc::c_uint) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_doublefact_e(n, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_doublefact_e(n, &result)\0" as *const u8 as *const libc::c_char,
            b"gamma.c\0" as *const u8 as *const libc::c_char,
            1644 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_lndoublefact(n: libc::c_uint) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_lndoublefact_e(n, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_lndoublefact_e(n, &result)\0" as *const u8 as *const libc::c_char,
            b"gamma.c\0" as *const u8 as *const libc::c_char,
            1649 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_lngamma(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_lngamma_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_lngamma_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"gamma.c\0" as *const u8 as *const libc::c_char,
            1654 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_gamma(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_gamma_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_gamma_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"gamma.c\0" as *const u8 as *const libc::c_char,
            1659 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_gammastar(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_gammastar_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_gammastar_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"gamma.c\0" as *const u8 as *const libc::c_char,
            1664 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_gammainv(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_gammainv_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_gammainv_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"gamma.c\0" as *const u8 as *const libc::c_char,
            1669 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_taylorcoeff(
    n: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_taylorcoeff_e(n, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_taylorcoeff_e(n, x, &result)\0" as *const u8 as *const libc::c_char,
            b"gamma.c\0" as *const u8 as *const libc::c_char,
            1674 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_choose(
    mut n: libc::c_uint,
    mut m: libc::c_uint,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_choose_e(n, m, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_choose_e(n, m, &result)\0" as *const u8 as *const libc::c_char,
            b"gamma.c\0" as *const u8 as *const libc::c_char,
            1679 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_lnchoose(
    mut n: libc::c_uint,
    mut m: libc::c_uint,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_lnchoose_e(n, m, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_lnchoose_e(n, m, &result)\0" as *const u8 as *const libc::c_char,
            b"gamma.c\0" as *const u8 as *const libc::c_char,
            1684 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
