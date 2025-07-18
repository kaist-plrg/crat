use ::libc;
extern "C" {
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_bessel_Jnu_e(
        nu: libc::c_double,
        x: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
}
pub type gsl_prec_t = libc::c_uint;
pub type size_t = libc::c_ulong;
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
pub type gsl_mode_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_sf_result_struct {
    pub val: libc::c_double,
    pub err: libc::c_double,
}
pub type gsl_sf_result = gsl_sf_result_struct;
#[inline]
unsafe extern "C" fn GSL_MODE_PREC(mut mt: gsl_mode_t) -> libc::c_uint {
    return mt & 7 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn rk_step(
    mut nu: libc::c_double,
    mut x: libc::c_double,
    mut dx: libc::c_double,
    mut Jp: *mut libc::c_double,
    mut J: *mut libc::c_double,
) -> libc::c_int {
    let mut p_0: libc::c_double = *Jp;
    let mut u_0: libc::c_double = *J;
    let mut p_1: libc::c_double = dx * (-p_0 / x + (nu * nu / (x * x) - 1.0f64) * u_0);
    let mut u_1: libc::c_double = dx * p_0;
    let mut p_2: libc::c_double = dx
        * (-(p_0 + 0.5f64 * p_1) / (x + 0.5f64 * dx)
            + (nu * nu / ((x + 0.5f64 * dx) * (x + 0.5f64 * dx)) - 1.0f64)
                * (u_0 + 0.5f64 * u_1));
    let mut u_2: libc::c_double = dx * (p_0 + 0.5f64 * p_1);
    let mut p_3: libc::c_double = dx
        * (-(p_0 + 0.5f64 * p_2) / (x + 0.5f64 * dx)
            + (nu * nu / ((x + 0.5f64 * dx) * (x + 0.5f64 * dx)) - 1.0f64)
                * (u_0 + 0.5f64 * u_2));
    let mut u_3: libc::c_double = dx * (p_0 + 0.5f64 * p_2);
    let mut p_4: libc::c_double = dx
        * (-(p_0 + p_3) / (x + dx)
            + (nu * nu / ((x + dx) * (x + dx)) - 1.0f64) * (u_0 + u_3));
    let mut u_4: libc::c_double = dx * (p_0 + p_3);
    *Jp = p_0 + p_1 / 6.0f64 + p_2 / 3.0f64 + p_3 / 3.0f64 + p_4 / 6.0f64;
    *J = u_0 + u_1 / 6.0f64 + u_2 / 3.0f64 + u_3 / 3.0f64 + u_4 / 6.0f64;
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_bessel_sequence_Jnu_e(
    mut nu: libc::c_double,
    mut mode: gsl_mode_t,
    mut size: size_t,
    mut v: *mut libc::c_double,
) -> libc::c_int {
    if nu < 0.0f64 {
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_sequence.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if size == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"bessel_sequence.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        let goal: gsl_prec_t = GSL_MODE_PREC(mode);
        let dx_array: [libc::c_double; 3] = [0.001f64, 0.03f64, 0.1f64];
        let dx_nominal: libc::c_double = dx_array[goal as usize];
        let cnu: libc::c_int = ceil(nu) as libc::c_int;
        let nu13: libc::c_double = pow(nu, 1.0f64 / 3.0f64);
        let smalls: [libc::c_double; 11] = [
            0.01f64,
            0.02f64,
            0.4f64,
            0.7f64,
            1.3f64,
            2.0f64,
            2.5f64,
            3.2f64,
            3.5f64,
            4.5f64,
            6.0f64,
        ];
        let x_small: libc::c_double = if nu >= 10.0f64 {
            nu - nu13
        } else {
            smalls[cnu as usize]
        };
        let mut J0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut J1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut Jp: libc::c_double = 0.;
        let mut J: libc::c_double = 0.;
        let mut x: libc::c_double = 0.;
        let mut i: size_t = 0 as libc::c_int as size_t;
        x = *v.offset(0 as libc::c_int as isize);
        gsl_sf_bessel_Jnu_e(nu, x, &mut J0);
        *v.offset(0 as libc::c_int as isize) = J0.val;
        i = i.wrapping_add(1);
        i;
        if x == 0.0f64 {
            if *v.offset(1 as libc::c_int as isize) <= x {
                gsl_error(
                    b"error\0" as *const u8 as *const libc::c_char,
                    b"bessel_sequence.c\0" as *const u8 as *const libc::c_char,
                    94 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            x = *v.offset(1 as libc::c_int as isize);
            gsl_sf_bessel_Jnu_e(nu, x, &mut J0);
            *v.offset(1 as libc::c_int as isize) = J0.val;
            i = i.wrapping_add(1);
            i;
        }
        while *v.offset(i as isize) < x_small && i < size {
            if *v.offset(i as isize) <= x {
                gsl_error(
                    b"error\0" as *const u8 as *const libc::c_char,
                    b"bessel_sequence.c\0" as *const u8 as *const libc::c_char,
                    109 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            x = *v.offset(i as isize);
            gsl_sf_bessel_Jnu_e(nu, x, &mut J0);
            *v.offset(i as isize) = J0.val;
            i = i.wrapping_add(1);
            i;
        }
        gsl_sf_bessel_Jnu_e(nu + 1.0f64, x, &mut J1);
        J = J0.val;
        Jp = -J1.val + nu / x * J0.val;
        while i < size {
            let dv: libc::c_double = *v.offset(i as isize) - x;
            let Nd: libc::c_int = ceil(dv / dx_nominal) as libc::c_int;
            let dx: libc::c_double = dv / Nd as libc::c_double;
            let mut xj: libc::c_double = 0.;
            let mut j: libc::c_int = 0;
            if *v.offset(i as isize) <= x {
                gsl_error(
                    b"error\0" as *const u8 as *const libc::c_char,
                    b"bessel_sequence.c\0" as *const u8 as *const libc::c_char,
                    137 as libc::c_int,
                    GSL_EFAILED as libc::c_int,
                );
                return GSL_EFAILED as libc::c_int;
            }
            j = 0 as libc::c_int;
            xj = x;
            while j < Nd {
                rk_step(nu, xj, dx, &mut Jp, &mut J);
                j += 1;
                j;
                xj += dx;
            }
            x = *v.offset(i as isize);
            *v.offset(i as isize) = J;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
