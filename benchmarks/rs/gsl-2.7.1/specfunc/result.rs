use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_sf_result_e10_struct {
    pub val: libc::c_double,
    pub err: libc::c_double,
    pub e10: libc::c_int,
}
pub type gsl_sf_result_e10 = gsl_sf_result_e10_struct;
pub unsafe extern "C" fn gsl_sf_result_smash_e(
    mut re: *const gsl_sf_result_e10,
    mut r: *mut gsl_sf_result,
) -> libc::c_int {
    if (*re).e10 == 0 as libc::c_int {
        (*r).val = (*re).val;
        (*r).err = (*re).err;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let av: libc::c_double = fabs((*re).val);
        let ae: libc::c_double = fabs((*re).err);
        if 1.4916681462400413e-154f64 < av && av < 1.3407807929942596e+154f64
            && 1.4916681462400413e-154f64 < ae && ae < 1.3407807929942596e+154f64
            && 0.49f64 * -7.0839641853226408e+02f64 < (*re).e10 as libc::c_double
            && ((*re).e10 as libc::c_double) < 0.49f64 * 7.0978271289338397e+02f64
        {
            let scale: libc::c_double = exp(
                (*re).e10 as libc::c_double * 2.30258509299404568402f64,
            );
            (*r).val = (*re).val * scale;
            (*r).err = (*re).err * scale;
            return GSL_SUCCESS as libc::c_int;
        } else {
            return gsl_sf_exp_mult_err_e(
                (*re).e10 as libc::c_double * 2.30258509299404568402f64,
                0.0f64,
                (*re).val,
                (*re).err,
                r,
            )
        }
    };
}
