use ::libc;
extern "C" {
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_function_struct {
    pub function: Option::<
        unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_double,
    >,
    pub params: *mut libc::c_void,
}
pub type gsl_function = gsl_function_struct;
unsafe extern "C" fn rescale_error(
    mut err: libc::c_double,
    result_abs: libc::c_double,
    result_asc: libc::c_double,
) -> libc::c_double {
    err = fabs(err);
    if result_asc != 0 as libc::c_int as libc::c_double
        && err != 0 as libc::c_int as libc::c_double
    {
        let mut scale: libc::c_double = pow(
            200 as libc::c_int as libc::c_double * err / result_asc,
            1.5f64,
        );
        if scale < 1 as libc::c_int as libc::c_double {
            err = result_asc * scale;
        } else {
            err = result_asc;
        }
    }
    if result_abs
        > 2.2250738585072014e-308f64
            / (50 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64)
    {
        let mut min_err: libc::c_double = 50 as libc::c_int as libc::c_double
            * 2.2204460492503131e-16f64 * result_abs;
        if min_err > err {
            err = min_err;
        }
    }
    return err;
}
pub unsafe extern "C" fn gsl_integration_qk(
    n: libc::c_int,
    mut xgk: *const libc::c_double,
    mut wg: *const libc::c_double,
    mut wgk: *const libc::c_double,
    mut fv1: *mut libc::c_double,
    mut fv2: *mut libc::c_double,
    mut f: *const gsl_function,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
    mut resabs: *mut libc::c_double,
    mut resasc: *mut libc::c_double,
) {
    let center: libc::c_double = 0.5f64 * (a + b);
    let half_length: libc::c_double = 0.5f64 * (b - a);
    let abs_half_length: libc::c_double = fabs(half_length);
    let f_center: libc::c_double = (Some(((*f).function).unwrap()))
        .unwrap()(center, (*f).params);
    let mut result_gauss: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut result_kronrod: libc::c_double = f_center
        * *wgk.offset((n - 1 as libc::c_int) as isize);
    let mut result_abs: libc::c_double = fabs(result_kronrod);
    let mut result_asc: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut mean: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut err: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut j: libc::c_int = 0;
    if n % 2 as libc::c_int == 0 as libc::c_int {
        result_gauss = f_center
            * *wg.offset((n / 2 as libc::c_int - 1 as libc::c_int) as isize);
    }
    j = 0 as libc::c_int;
    while j < (n - 1 as libc::c_int) / 2 as libc::c_int {
        let jtw: libc::c_int = j * 2 as libc::c_int + 1 as libc::c_int;
        let abscissa: libc::c_double = half_length * *xgk.offset(jtw as isize);
        let fval1: libc::c_double = (Some(((*f).function).unwrap()))
            .unwrap()(center - abscissa, (*f).params);
        let fval2: libc::c_double = (Some(((*f).function).unwrap()))
            .unwrap()(center + abscissa, (*f).params);
        let fsum: libc::c_double = fval1 + fval2;
        *fv1.offset(jtw as isize) = fval1;
        *fv2.offset(jtw as isize) = fval2;
        result_gauss += *wg.offset(j as isize) * fsum;
        result_kronrod += *wgk.offset(jtw as isize) * fsum;
        result_abs += *wgk.offset(jtw as isize) * (fabs(fval1) + fabs(fval2));
        j += 1;
        j;
    }
    j = 0 as libc::c_int;
    while j < n / 2 as libc::c_int {
        let mut jtwm1: libc::c_int = j * 2 as libc::c_int;
        let abscissa_0: libc::c_double = half_length * *xgk.offset(jtwm1 as isize);
        let fval1_0: libc::c_double = (Some(((*f).function).unwrap()))
            .unwrap()(center - abscissa_0, (*f).params);
        let fval2_0: libc::c_double = (Some(((*f).function).unwrap()))
            .unwrap()(center + abscissa_0, (*f).params);
        *fv1.offset(jtwm1 as isize) = fval1_0;
        *fv2.offset(jtwm1 as isize) = fval2_0;
        result_kronrod += *wgk.offset(jtwm1 as isize) * (fval1_0 + fval2_0);
        result_abs += *wgk.offset(jtwm1 as isize) * (fabs(fval1_0) + fabs(fval2_0));
        j += 1;
        j;
    }
    mean = result_kronrod * 0.5f64;
    result_asc = *wgk.offset((n - 1 as libc::c_int) as isize) * fabs(f_center - mean);
    j = 0 as libc::c_int;
    while j < n - 1 as libc::c_int {
        result_asc
            += *wgk.offset(j as isize)
                * (fabs(*fv1.offset(j as isize) - mean)
                    + fabs(*fv2.offset(j as isize) - mean));
        j += 1;
        j;
    }
    err = (result_kronrod - result_gauss) * half_length;
    result_kronrod *= half_length;
    result_abs *= abs_half_length;
    result_asc *= abs_half_length;
    *result = result_kronrod;
    *resabs = result_abs;
    *resasc = result_asc;
    *abserr = rescale_error(err, result_abs, result_asc);
}
