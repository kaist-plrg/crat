use ::libc;
extern "C" {
    fn atan(_: libc::c_double) -> libc::c_double;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn tanh(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_expm1_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_exp_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_psi_1piy_e(y: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_airy_Ai_scaled_e(
        x: libc::c_double,
        mode: gsl_mode_t,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_airy_Bi_scaled_e(
        x: libc::c_double,
        mode: gsl_mode_t,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_lngamma_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_lngamma_complex_e(
        zr: libc::c_double,
        zi: libc::c_double,
        lnr: *mut gsl_sf_result,
        arg: *mut gsl_sf_result,
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
pub type gsl_mode_t = libc::c_uint;
unsafe extern "C" fn C0sq(mut eta: libc::c_double) -> libc::c_double {
    let mut twopieta: libc::c_double = 2.0f64 * 3.14159265358979323846f64 * eta;
    if fabs(eta) < 2.2204460492503131e-16f64 {
        return 1.0f64
    } else if twopieta > 7.0978271289338397e+02f64 {
        return 0.0f64
    } else {
        let mut scale: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        gsl_sf_expm1_e(twopieta, &mut scale);
        return twopieta / scale.val;
    };
}
unsafe extern "C" fn CLeta(
    mut L: libc::c_double,
    mut eta: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut ln1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut ln2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut sgn: libc::c_double = 1.0f64;
    let mut arg_val: libc::c_double = 0.;
    let mut arg_err: libc::c_double = 0.;
    if fabs(eta / (L + 1.0f64)) < 2.2204460492503131e-16f64 {
        gsl_sf_lngamma_e(L + 1.0f64, &mut ln1);
    } else {
        let mut p1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        gsl_sf_lngamma_complex_e(L + 1.0f64, eta, &mut ln1, &mut p1);
    }
    gsl_sf_lngamma_e(2.0f64 * (L + 1.0f64), &mut ln2);
    if L < -1.0f64 {
        sgn = -sgn;
    }
    arg_val = L * 0.69314718055994530942f64 - 0.5f64 * eta * 3.14159265358979323846f64
        + ln1.val - ln2.val;
    arg_err = ln1.err + ln2.err;
    arg_err
        += 2.2204460492503131e-16f64
            * (fabs(L * 0.69314718055994530942f64)
                + fabs(0.5f64 * eta * 3.14159265358979323846f64));
    return gsl_sf_exp_err_e(arg_val, arg_err, result);
}
pub unsafe extern "C" fn gsl_sf_coulomb_CL_e(
    mut lam: libc::c_double,
    mut eta: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if lam <= -1.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"coulomb.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if fabs(lam) < 2.2204460492503131e-16f64 {
        (*result).val = sqrt(C0sq(eta));
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else {
        return CLeta(lam, eta, result)
    };
}
pub unsafe extern "C" fn gsl_sf_coulomb_CL_array(
    mut lam_min: libc::c_double,
    mut kmax: libc::c_int,
    mut eta: libc::c_double,
    mut cl: *mut libc::c_double,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut cl_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    gsl_sf_coulomb_CL_e(lam_min, eta, &mut cl_0);
    *cl.offset(0 as libc::c_int as isize) = cl_0.val;
    k = 1 as libc::c_int;
    while k <= kmax {
        let mut L: libc::c_double = lam_min + k as libc::c_double;
        *cl
            .offset(
                k as isize,
            ) = *cl.offset((k - 1 as libc::c_int) as isize) * hypot(L, eta)
            / (L * (2.0f64 * L + 1.0f64));
        k += 1;
        k;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn coulomb_connection(
    lam: libc::c_double,
    eta: libc::c_double,
    mut cos_phi: *mut libc::c_double,
    mut sin_phi: *mut libc::c_double,
) -> libc::c_int {
    if eta > --7.0839641853226408e+02f64 / 2.0f64 * 3.14159265358979323846f64 - 1.0f64 {
        *cos_phi = 1.0f64;
        *sin_phi = 0.0f64;
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"coulomb.c\0" as *const u8 as *const libc::c_char,
            242 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    } else if eta > --3.6043653389117154e+01f64 / (4.0f64 * 3.14159265358979323846f64) {
        let eps: libc::c_double = 2.0f64
            * exp(-2.0f64 * 3.14159265358979323846f64 * eta);
        let tpl: libc::c_double = tan(3.14159265358979323846f64 * lam);
        let dth: libc::c_double = eps * tpl / (tpl * tpl + 1.0f64);
        *cos_phi = -1.0f64 + 0.5f64 * dth * dth;
        *sin_phi = -dth;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut X: libc::c_double = tanh(3.14159265358979323846f64 * eta)
            / tan(3.14159265358979323846f64 * lam);
        let mut phi: libc::c_double = -atan(X)
            - (lam + 0.5f64) * 3.14159265358979323846f64;
        *cos_phi = cos(phi);
        *sin_phi = sin(phi);
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn coulomb_FG_series(
    lam: libc::c_double,
    eta: libc::c_double,
    x: libc::c_double,
    mut F: *mut gsl_sf_result,
    mut G: *mut gsl_sf_result,
) -> libc::c_int {
    let max_iter: libc::c_int = 800 as libc::c_int;
    let mut ClamA: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut ClamB: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_A: libc::c_int = CLeta(lam, eta, &mut ClamA);
    let mut stat_B: libc::c_int = CLeta(-lam - 1.0f64, eta, &mut ClamB);
    let tlp1: libc::c_double = 2.0f64 * lam + 1.0f64;
    let pow_x: libc::c_double = pow(x, lam);
    let mut cos_phi_lam: libc::c_double = 0.;
    let mut sin_phi_lam: libc::c_double = 0.;
    let mut uA_mm2: libc::c_double = 1.0f64;
    let mut uA_mm1: libc::c_double = x * eta / (lam + 1.0f64);
    let mut uA_m: libc::c_double = 0.;
    let mut uB_mm2: libc::c_double = 1.0f64;
    let mut uB_mm1: libc::c_double = -x * eta / lam;
    let mut uB_m: libc::c_double = 0.;
    let mut A_sum: libc::c_double = uA_mm2 + uA_mm1;
    let mut B_sum: libc::c_double = uB_mm2 + uB_mm1;
    let mut A_abs_del_prev: libc::c_double = fabs(A_sum);
    let mut B_abs_del_prev: libc::c_double = fabs(B_sum);
    let mut FA: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut FB: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut m: libc::c_int = 2 as libc::c_int;
    let mut stat_conn: libc::c_int = coulomb_connection(
        lam,
        eta,
        &mut cos_phi_lam,
        &mut sin_phi_lam,
    );
    if stat_conn == GSL_EUNDRFLW as libc::c_int {
        (*F).val = 0.0f64;
        (*F).err = 0.0f64;
        (*G).val = ::std::f32::INFINITY as libc::c_double;
        (*G).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"coulomb.c\0" as *const u8 as *const libc::c_char,
            308 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    }
    while m < max_iter {
        let mut abs_dA: libc::c_double = 0.;
        let mut abs_dB: libc::c_double = 0.;
        uA_m = x * (2.0f64 * eta * uA_mm1 - x * uA_mm2)
            / (m as libc::c_double * (m as libc::c_double + tlp1));
        uB_m = x * (2.0f64 * eta * uB_mm1 - x * uB_mm2)
            / (m as libc::c_double * (m as libc::c_double - tlp1));
        A_sum += uA_m;
        B_sum += uB_m;
        abs_dA = fabs(uA_m);
        abs_dB = fabs(uB_m);
        if m > 15 as libc::c_int {
            let mut max_abs_dA: libc::c_double = if abs_dA > A_abs_del_prev {
                abs_dA
            } else {
                A_abs_del_prev
            };
            let mut max_abs_dB: libc::c_double = if abs_dB > B_abs_del_prev {
                abs_dB
            } else {
                B_abs_del_prev
            };
            let mut abs_A: libc::c_double = fabs(A_sum);
            let mut abs_B: libc::c_double = fabs(B_sum);
            if max_abs_dA / (max_abs_dA + abs_A) < 4.0f64 * 2.2204460492503131e-16f64
                && max_abs_dB / (max_abs_dB + abs_B) < 4.0f64 * 2.2204460492503131e-16f64
            {
                break;
            }
        }
        A_abs_del_prev = abs_dA;
        B_abs_del_prev = abs_dB;
        uA_mm2 = uA_mm1;
        uA_mm1 = uA_m;
        uB_mm2 = uB_mm1;
        uB_mm1 = uB_m;
        m += 1;
        m;
    }
    FA.val = A_sum * ClamA.val * pow_x * x;
    FA
        .err = fabs(A_sum) * ClamA.err * pow_x * x
        + 2.0f64 * 2.2204460492503131e-16f64 * fabs(FA.val);
    FB.val = B_sum * ClamB.val / pow_x;
    FB
        .err = fabs(B_sum) * ClamB.err / pow_x
        + 2.0f64 * 2.2204460492503131e-16f64 * fabs(FB.val);
    (*F).val = FA.val;
    (*F).err = FA.err;
    (*G).val = (FA.val * cos_phi_lam - FB.val) / sin_phi_lam;
    (*G).err = (FA.err * fabs(cos_phi_lam) + FB.err) / fabs(sin_phi_lam);
    if m >= max_iter {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"coulomb.c\0" as *const u8 as *const libc::c_char,
            356 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return if stat_A != GSL_SUCCESS as libc::c_int {
            stat_A
        } else if stat_B != GSL_SUCCESS as libc::c_int {
            stat_B
        } else {
            GSL_SUCCESS as libc::c_int
        }
    };
}
unsafe extern "C" fn coulomb_FG0_series(
    eta: libc::c_double,
    x: libc::c_double,
    mut F: *mut gsl_sf_result,
    mut G: *mut gsl_sf_result,
) -> libc::c_int {
    let max_iter: libc::c_int = 800 as libc::c_int;
    let x2: libc::c_double = x * x;
    let tex: libc::c_double = 2.0f64 * eta * x;
    let mut C0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_CL: libc::c_int = CLeta(0.0f64, eta, &mut C0);
    let mut r1pie: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut psi_stat: libc::c_int = gsl_sf_psi_1piy_e(eta, &mut r1pie);
    let mut u_mm2: libc::c_double = 0.0f64;
    let mut u_mm1: libc::c_double = x;
    let mut u_m: libc::c_double = 0.;
    let mut v_mm2: libc::c_double = 1.0f64;
    let mut v_mm1: libc::c_double = tex
        * (2.0f64 * 0.57721566490153286060651209008f64 - 1.0f64 + r1pie.val);
    let mut v_m: libc::c_double = 0.;
    let mut u_sum: libc::c_double = u_mm2 + u_mm1;
    let mut v_sum: libc::c_double = v_mm2 + v_mm1;
    let mut u_abs_del_prev: libc::c_double = fabs(u_sum);
    let mut v_abs_del_prev: libc::c_double = fabs(v_sum);
    let mut m: libc::c_int = 2 as libc::c_int;
    let mut u_sum_err: libc::c_double = 2.0f64 * 2.2204460492503131e-16f64 * fabs(u_sum);
    let mut v_sum_err: libc::c_double = 2.0f64 * 2.2204460492503131e-16f64 * fabs(v_sum);
    let mut ln2x: libc::c_double = log(2.0f64 * x);
    while m < max_iter {
        let mut abs_du: libc::c_double = 0.;
        let mut abs_dv: libc::c_double = 0.;
        let mut m_mm1: libc::c_double = m as libc::c_double
            * (m as libc::c_double - 1.0f64);
        u_m = (tex * u_mm1 - x2 * u_mm2) / m_mm1;
        v_m = (tex * v_mm1 - x2 * v_mm2
            - 2.0f64 * eta * (2 as libc::c_int * m - 1 as libc::c_int) as libc::c_double
                * u_m) / m_mm1;
        u_sum += u_m;
        v_sum += v_m;
        abs_du = fabs(u_m);
        abs_dv = fabs(v_m);
        u_sum_err += 2.0f64 * 2.2204460492503131e-16f64 * abs_du;
        v_sum_err += 2.0f64 * 2.2204460492503131e-16f64 * abs_dv;
        if m > 15 as libc::c_int {
            let mut max_abs_du: libc::c_double = if abs_du > u_abs_del_prev {
                abs_du
            } else {
                u_abs_del_prev
            };
            let mut max_abs_dv: libc::c_double = if abs_dv > v_abs_del_prev {
                abs_dv
            } else {
                v_abs_del_prev
            };
            let mut abs_u: libc::c_double = fabs(u_sum);
            let mut abs_v: libc::c_double = fabs(v_sum);
            if max_abs_du / (max_abs_du + abs_u) < 40.0f64 * 2.2204460492503131e-16f64
                && max_abs_dv / (max_abs_dv + abs_v)
                    < 40.0f64 * 2.2204460492503131e-16f64
            {
                break;
            }
        }
        u_abs_del_prev = abs_du;
        v_abs_del_prev = abs_dv;
        u_mm2 = u_mm1;
        u_mm1 = u_m;
        v_mm2 = v_mm1;
        v_mm1 = v_m;
        m += 1;
        m;
    }
    (*F).val = C0.val * u_sum;
    (*F).err = C0.err * fabs(u_sum);
    (*F).err += fabs(C0.val) * u_sum_err;
    (*F).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*F).val);
    (*G).val = (v_sum + 2.0f64 * eta * u_sum * ln2x) / C0.val;
    (*G)
        .err = (fabs(v_sum) + fabs(2.0f64 * eta * u_sum * ln2x)) / fabs(C0.val)
        * fabs(C0.err / C0.val);
    (*G).err += (v_sum_err + fabs(2.0f64 * eta * u_sum_err * ln2x)) / fabs(C0.val);
    (*G).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*G).val);
    if m == max_iter {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"coulomb.c\0" as *const u8 as *const libc::c_char,
            440 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return if psi_stat != GSL_SUCCESS as libc::c_int {
            psi_stat
        } else if stat_CL != GSL_SUCCESS as libc::c_int {
            stat_CL
        } else {
            GSL_SUCCESS as libc::c_int
        }
    };
}
unsafe extern "C" fn coulomb_FGmhalf_series(
    eta: libc::c_double,
    x: libc::c_double,
    mut F: *mut gsl_sf_result,
    mut G: *mut gsl_sf_result,
) -> libc::c_int {
    let max_iter: libc::c_int = 800 as libc::c_int;
    let rx: libc::c_double = sqrt(x);
    let x2: libc::c_double = x * x;
    let tex: libc::c_double = 2.0f64 * eta * x;
    let mut Cmhalf: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_CL: libc::c_int = CLeta(-0.5f64, eta, &mut Cmhalf);
    let mut u_mm2: libc::c_double = 1.0f64;
    let mut u_mm1: libc::c_double = tex * u_mm2;
    let mut u_m: libc::c_double = 0.;
    let mut v_mm2: libc::c_double = 0.;
    let mut v_mm1: libc::c_double = 0.;
    let mut v_m: libc::c_double = 0.;
    let mut f_sum: libc::c_double = 0.;
    let mut g_sum: libc::c_double = 0.;
    let mut tmp1: libc::c_double = 0.;
    let mut rpsi_1pe: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut rpsi_1p2e: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut m: libc::c_int = 2 as libc::c_int;
    gsl_sf_psi_1piy_e(eta, &mut rpsi_1pe);
    gsl_sf_psi_1piy_e(2.0f64 * eta, &mut rpsi_1p2e);
    v_mm2 = 2.0f64 * 0.57721566490153286060651209008f64 - 0.69314718055994530942f64
        - rpsi_1pe.val + 2.0f64 * rpsi_1p2e.val;
    v_mm1 = tex * (v_mm2 - 2.0f64 * u_mm2);
    f_sum = u_mm2 + u_mm1;
    g_sum = v_mm2 + v_mm1;
    while m < max_iter {
        let mut m2: libc::c_double = (m * m) as libc::c_double;
        u_m = (tex * u_mm1 - x2 * u_mm2) / m2;
        v_m = (tex * v_mm1 - x2 * v_mm2 - 2.0f64 * m as libc::c_double * u_m) / m2;
        f_sum += u_m;
        g_sum += v_m;
        if f_sum != 0.0f64 && g_sum != 0.0f64
            && fabs(u_m / f_sum) + fabs(v_m / g_sum)
                < 10.0f64 * 2.2204460492503131e-16f64
        {
            break;
        }
        u_mm2 = u_mm1;
        u_mm1 = u_m;
        v_mm2 = v_mm1;
        v_mm1 = v_m;
        m += 1;
        m;
    }
    (*F).val = Cmhalf.val * rx * f_sum;
    (*F)
        .err = Cmhalf.err * fabs(rx * f_sum)
        + 2.0f64 * 2.2204460492503131e-16f64 * fabs((*F).val);
    tmp1 = f_sum * log(x);
    (*G).val = -rx * (tmp1 + g_sum) / Cmhalf.val;
    (*G)
        .err = fabs(rx) * (fabs(tmp1) + fabs(g_sum)) / fabs(Cmhalf.val)
        * fabs(Cmhalf.err / Cmhalf.val);
    if m == max_iter {
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"coulomb.c\0" as *const u8 as *const libc::c_char,
            503 as libc::c_int,
            GSL_EMAXITER as libc::c_int,
        );
        return GSL_EMAXITER as libc::c_int;
    } else {
        return stat_CL
    };
}
unsafe extern "C" fn coulomb_F_recur(
    mut lam_min: libc::c_double,
    mut kmax: libc::c_int,
    mut eta: libc::c_double,
    mut x: libc::c_double,
    mut F_lam_max: libc::c_double,
    mut Fp_lam_max: libc::c_double,
    mut F_lam_min: *mut libc::c_double,
    mut Fp_lam_min: *mut libc::c_double,
) -> libc::c_int {
    let mut x_inv: libc::c_double = 1.0f64 / x;
    let mut fcl: libc::c_double = F_lam_max;
    let mut fpl: libc::c_double = Fp_lam_max;
    let mut lam_max: libc::c_double = lam_min + kmax as libc::c_double;
    let mut lam: libc::c_double = lam_max;
    let mut k: libc::c_int = 0;
    k = kmax - 1 as libc::c_int;
    while k >= 0 as libc::c_int {
        let mut el: libc::c_double = eta / lam;
        let mut rl: libc::c_double = hypot(1.0f64, el);
        let mut sl: libc::c_double = el + lam * x_inv;
        let mut fc_lm1: libc::c_double = 0.;
        fc_lm1 = (fcl * sl + fpl) / rl;
        fpl = fc_lm1 * sl - fcl * rl;
        fcl = fc_lm1;
        lam -= 1.0f64;
        k -= 1;
        k;
    }
    *F_lam_min = fcl;
    *Fp_lam_min = fpl;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn coulomb_G_recur(
    lam_min: libc::c_double,
    kmax: libc::c_int,
    eta: libc::c_double,
    x: libc::c_double,
    G_lam_min: libc::c_double,
    Gp_lam_min: libc::c_double,
    mut G_lam_max: *mut libc::c_double,
    mut Gp_lam_max: *mut libc::c_double,
) -> libc::c_int {
    let mut x_inv: libc::c_double = 1.0f64 / x;
    let mut gcl: libc::c_double = G_lam_min;
    let mut gpl: libc::c_double = Gp_lam_min;
    let mut lam: libc::c_double = lam_min + 1.0f64;
    let mut k: libc::c_int = 0;
    k = 1 as libc::c_int;
    while k <= kmax {
        let mut el: libc::c_double = eta / lam;
        let mut rl: libc::c_double = hypot(1.0f64, el);
        let mut sl: libc::c_double = el + lam * x_inv;
        let mut gcl1: libc::c_double = (sl * gcl - gpl) / rl;
        gpl = rl * gcl - sl * gcl1;
        gcl = gcl1;
        lam += 1.0f64;
        k += 1;
        k;
    }
    *G_lam_max = gcl;
    *Gp_lam_max = gpl;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn coulomb_CF1(
    mut lambda: libc::c_double,
    mut eta: libc::c_double,
    mut x: libc::c_double,
    mut fcl_sign: *mut libc::c_double,
    mut result: *mut libc::c_double,
    mut count: *mut libc::c_int,
) -> libc::c_int {
    let CF1_small: libc::c_double = 1.0e-30f64;
    let CF1_abort: libc::c_double = 1.0e+05f64;
    let CF1_acc: libc::c_double = 2.0f64 * 2.2204460492503131e-16f64;
    let x_inv: libc::c_double = 1.0f64 / x;
    let px: libc::c_double = lambda + 1.0f64 + CF1_abort;
    let mut pk: libc::c_double = lambda + 1.0f64;
    let mut F: libc::c_double = eta / pk + pk * x_inv;
    let mut D: libc::c_double = 0.;
    let mut C: libc::c_double = 0.;
    let mut df: libc::c_double = 0.;
    *fcl_sign = 1.0f64;
    *count = 0 as libc::c_int;
    if fabs(F) < CF1_small {
        F = CF1_small;
    }
    D = 0.0f64;
    C = F;
    loop {
        let mut pk1: libc::c_double = pk + 1.0f64;
        let mut ek: libc::c_double = eta / pk;
        let mut rk2: libc::c_double = 1.0f64 + ek * ek;
        let mut tk: libc::c_double = (pk + pk1) * (x_inv + ek / pk1);
        D = tk - rk2 * D;
        C = tk - rk2 / C;
        if fabs(C) < CF1_small {
            C = CF1_small;
        }
        if fabs(D) < CF1_small {
            D = CF1_small;
        }
        D = 1.0f64 / D;
        df = D * C;
        F = F * df;
        if D < 0.0f64 {
            *fcl_sign = -*fcl_sign;
        }
        pk = pk1;
        if pk > px {
            *result = F;
            gsl_error(
                b"error\0" as *const u8 as *const libc::c_char,
                b"coulomb.c\0" as *const u8 as *const libc::c_char,
                639 as libc::c_int,
                GSL_ERUNAWAY as libc::c_int,
            );
            return GSL_ERUNAWAY as libc::c_int;
        }
        *count += 1;
        *count;
        if !(fabs(df - 1.0f64) > CF1_acc) {
            break;
        }
    }
    *result = F;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn coulomb_CF2(
    lambda: libc::c_double,
    eta: libc::c_double,
    x: libc::c_double,
    mut result_P: *mut libc::c_double,
    mut result_Q: *mut libc::c_double,
    mut count: *mut libc::c_int,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let CF2_acc: libc::c_double = 4.0f64 * 2.2204460492503131e-16f64;
    let CF2_abort: libc::c_double = 2.0e+05f64;
    let wi: libc::c_double = 2.0f64 * eta;
    let x_inv: libc::c_double = 1.0f64 / x;
    let e2mm1: libc::c_double = eta * eta + lambda * (lambda + 1.0f64);
    let mut ar: libc::c_double = -e2mm1;
    let mut ai: libc::c_double = eta;
    let mut br: libc::c_double = 2.0f64 * (x - eta);
    let mut bi: libc::c_double = 2.0f64;
    let mut dr: libc::c_double = br / (br * br + bi * bi);
    let mut di: libc::c_double = -bi / (br * br + bi * bi);
    let mut dp: libc::c_double = -x_inv * (ar * di + ai * dr);
    let mut dq: libc::c_double = x_inv * (ar * dr - ai * di);
    let mut A: libc::c_double = 0.;
    let mut B: libc::c_double = 0.;
    let mut C: libc::c_double = 0.;
    let mut D: libc::c_double = 0.;
    let mut pk: libc::c_double = 0.0f64;
    let mut P: libc::c_double = 0.0f64;
    let mut Q: libc::c_double = 1.0f64 - eta * x_inv;
    *count = 0 as libc::c_int;
    loop {
        P += dp;
        Q += dq;
        pk += 2.0f64;
        ar += pk;
        ai += wi;
        bi += 2.0f64;
        D = ar * dr - ai * di + br;
        di = ai * dr + ar * di + bi;
        C = 1.0f64 / (D * D + di * di);
        dr = C * D;
        di = -C * di;
        A = br * dr - bi * di - 1.0f64;
        B = bi * dr + br * di;
        C = dp * A - dq * B;
        dq = dp * B + dq * A;
        dp = C;
        if pk > CF2_abort {
            status = GSL_ERUNAWAY as libc::c_int;
            break;
        } else {
            *count += 1;
            *count;
            if !(fabs(dp) + fabs(dq) > (fabs(P) + fabs(Q)) * CF2_acc) {
                break;
            }
        }
    }
    if Q < CF2_abort * 2.2204460492503131e-16f64 * fabs(P) {
        status = GSL_ELOSS as libc::c_int;
    }
    *result_P = P;
    *result_Q = Q;
    return status;
}
unsafe extern "C" fn coulomb_jwkb(
    lam: libc::c_double,
    eta: libc::c_double,
    x: libc::c_double,
    mut fjwkb: *mut gsl_sf_result,
    mut gjwkb: *mut gsl_sf_result,
    mut exponent: *mut libc::c_double,
) -> libc::c_int {
    let llp1: libc::c_double = lam * (lam + 1.0f64) + 6.0f64 / 35.0f64;
    let llp1_eff: libc::c_double = if llp1 > 0.0f64 { llp1 } else { 0.0f64 };
    let rho_ghalf: libc::c_double = sqrt(x * (2.0f64 * eta - x) + llp1_eff);
    let sinh_arg: libc::c_double = sqrt(llp1_eff / (eta * eta + llp1_eff)) * rho_ghalf
        / x;
    let sinh_inv: libc::c_double = log(sinh_arg + hypot(1.0f64, sinh_arg));
    let phi: libc::c_double = fabs(
        rho_ghalf - eta * atan2(rho_ghalf, x - eta) - sqrt(llp1_eff) * sinh_inv,
    );
    let zeta_half: libc::c_double = pow(3.0f64 * phi / 2.0f64, 1.0f64 / 3.0f64);
    let prefactor: libc::c_double = sqrt(
        3.14159265358979323846f64 * phi * x / (6.0f64 * rho_ghalf),
    );
    let mut F: libc::c_double = prefactor * 3.0f64 / zeta_half;
    let mut G: libc::c_double = prefactor * 3.0f64 / zeta_half;
    let mut F_exp: libc::c_double = 0.;
    let mut G_exp: libc::c_double = 0.;
    let airy_scale_exp: libc::c_double = phi;
    let mut ai: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut bi: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    gsl_sf_airy_Ai_scaled_e(
        zeta_half * zeta_half,
        0 as libc::c_int as gsl_mode_t,
        &mut ai,
    );
    gsl_sf_airy_Bi_scaled_e(
        zeta_half * zeta_half,
        0 as libc::c_int as gsl_mode_t,
        &mut bi,
    );
    F *= ai.val;
    G *= bi.val;
    F_exp = log(F) - airy_scale_exp;
    G_exp = log(G) + airy_scale_exp;
    if G_exp >= 7.0978271289338397e+02f64 {
        (*fjwkb).val = F;
        (*gjwkb).val = G;
        (*fjwkb).err = 1.0e-3f64 * fabs(F);
        (*gjwkb).err = 1.0e-3f64 * fabs(G);
        *exponent = airy_scale_exp;
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"coulomb.c\0" as *const u8 as *const libc::c_char,
            858 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    } else {
        (*fjwkb).val = exp(F_exp);
        (*gjwkb).val = exp(G_exp);
        (*fjwkb).err = 1.0e-3f64 * fabs((*fjwkb).val);
        (*gjwkb).err = 1.0e-3f64 * fabs((*gjwkb).val);
        *exponent = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_coulomb_wave_FG_e(
    eta: libc::c_double,
    x: libc::c_double,
    lam_F: libc::c_double,
    k_lam_G: libc::c_int,
    mut F: *mut gsl_sf_result,
    mut Fp: *mut gsl_sf_result,
    mut G: *mut gsl_sf_result,
    mut Gp: *mut gsl_sf_result,
    mut exp_F: *mut libc::c_double,
    mut exp_G: *mut libc::c_double,
) -> libc::c_int {
    let lam_G: libc::c_double = lam_F - k_lam_G as libc::c_double;
    if x < 0.0f64 || lam_F <= -0.5f64 || lam_G <= -0.5f64 {
        (*F).val = 0.0f64;
        (*F).err = 0.0f64;
        (*Fp).val = 0.0f64;
        (*Fp).err = 0.0f64;
        (*G).val = 0.0f64;
        (*G).err = 0.0f64;
        (*Gp).val = 0.0f64;
        (*Gp).err = 0.0f64;
        *exp_F = 0.0f64;
        *exp_G = 0.0f64;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"coulomb.c\0" as *const u8 as *const libc::c_char,
            912 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x == 0.0f64 {
        let mut C0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        CLeta(0.0f64, eta, &mut C0);
        (*F).val = 0.0f64;
        (*F).err = 0.0f64;
        (*Fp).val = 0.0f64;
        (*Fp).err = 0.0f64;
        (*G).val = 0.0f64;
        (*G).err = 0.0f64;
        (*Gp).val = 0.0f64;
        (*Gp).err = 0.0f64;
        *exp_F = 0.0f64;
        *exp_G = 0.0f64;
        if lam_F == 0.0f64 {
            (*Fp).val = C0.val;
            (*Fp).err = C0.err;
        }
        if lam_G == 0.0f64 {
            (*Gp).val = 1.0f64 / C0.val;
            (*Gp).err = fabs(C0.err / C0.val) / fabs(C0.val);
        }
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"coulomb.c\0" as *const u8 as *const libc::c_char,
            929 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x < 1.2f64
        && 2.0f64 * 3.14159265358979323846f64 * eta
            < 0.9f64 * --7.0839641853226408e+02f64 && fabs(eta * x) < 10.0f64
    {
        let SMALL: libc::c_double = 1.4901161193847656e-08f64;
        let N: libc::c_int = (lam_F + 0.5f64) as libc::c_int;
        let span: libc::c_int = if k_lam_G > N { k_lam_G } else { N };
        let lam_min: libc::c_double = lam_F - N as libc::c_double;
        let mut F_lam_F: libc::c_double = 0.;
        let mut Fp_lam_F: libc::c_double = 0.;
        let mut G_lam_G: libc::c_double = 0.0f64;
        let mut Gp_lam_G: libc::c_double = 0.0f64;
        let mut F_lam_F_err: libc::c_double = 0.;
        let mut Fp_lam_F_err: libc::c_double = 0.;
        let mut Fp_over_F_lam_F: libc::c_double = 0.;
        let mut F_sign_lam_F: libc::c_double = 0.;
        let mut F_lam_min_unnorm: libc::c_double = 0.;
        let mut Fp_lam_min_unnorm: libc::c_double = 0.;
        let mut Fp_over_F_lam_min: libc::c_double = 0.;
        let mut F_lam_min: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut G_lam_min: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut Gp_lam_min: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut F_scale: libc::c_double = 0.;
        let mut Gerr_frac: libc::c_double = 0.;
        let mut F_scale_frac_err: libc::c_double = 0.;
        let mut F_unnorm_frac_err: libc::c_double = 0.;
        let mut CF1_count: libc::c_int = 0;
        let mut stat_CF1: libc::c_int = coulomb_CF1(
            lam_F,
            eta,
            x,
            &mut F_sign_lam_F,
            &mut Fp_over_F_lam_F,
            &mut CF1_count,
        );
        let mut stat_ser: libc::c_int = 0;
        let mut stat_Fr: libc::c_int = 0;
        let mut stat_Gr: libc::c_int = 0;
        F_lam_F = SMALL;
        Fp_lam_F = Fp_over_F_lam_F * F_lam_F;
        if span != 0 as libc::c_int {
            stat_Fr = coulomb_F_recur(
                lam_min,
                span,
                eta,
                x,
                F_lam_F,
                Fp_lam_F,
                &mut F_lam_min_unnorm,
                &mut Fp_lam_min_unnorm,
            );
        } else {
            F_lam_min_unnorm = F_lam_F;
            Fp_lam_min_unnorm = Fp_lam_F;
            stat_Fr = GSL_SUCCESS as libc::c_int;
        }
        if lam_min == -0.5f64 {
            stat_ser = coulomb_FGmhalf_series(eta, x, &mut F_lam_min, &mut G_lam_min);
        } else if lam_min == 0.0f64 {
            stat_ser = coulomb_FG0_series(eta, x, &mut F_lam_min, &mut G_lam_min);
        } else if lam_min == 0.5f64 {
            (*F).val = F_lam_F;
            (*F).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*F).val);
            (*Fp).val = Fp_lam_F;
            (*Fp).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*Fp).val);
            (*G).val = G_lam_G;
            (*G).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*G).val);
            (*Gp).val = Gp_lam_G;
            (*Gp).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*Gp).val);
            *exp_F = 0.0f64;
            *exp_G = 0.0f64;
            gsl_error(
                b"error\0" as *const u8 as *const libc::c_char,
                b"coulomb.c\0" as *const u8 as *const libc::c_char,
                1000 as libc::c_int,
                GSL_ESANITY as libc::c_int,
            );
            return GSL_ESANITY as libc::c_int;
        } else {
            stat_ser = coulomb_FG_series(
                lam_min,
                eta,
                x,
                &mut F_lam_min,
                &mut G_lam_min,
            );
        }
        Fp_over_F_lam_min = Fp_lam_min_unnorm / F_lam_min_unnorm;
        Gp_lam_min.val = Fp_over_F_lam_min * G_lam_min.val - 1.0f64 / F_lam_min.val;
        Gp_lam_min.err = fabs(Fp_over_F_lam_min) * G_lam_min.err;
        Gp_lam_min.err
            += fabs(1.0f64 / F_lam_min.val) * fabs(F_lam_min.err / F_lam_min.val);
        F_scale = F_lam_min.val / F_lam_min_unnorm;
        F_scale_frac_err = fabs(F_lam_min.err / F_lam_min.val);
        F_unnorm_frac_err = 2.0f64 * 2.2204460492503131e-16f64
            * (CF1_count + span + 1 as libc::c_int) as libc::c_double;
        F_lam_F *= F_scale;
        F_lam_F_err = fabs(F_lam_F) * (F_unnorm_frac_err + F_scale_frac_err);
        Fp_lam_F *= F_scale;
        Fp_lam_F_err = fabs(Fp_lam_F) * (F_unnorm_frac_err + F_scale_frac_err);
        stat_Gr = coulomb_G_recur(
            lam_min,
            if N - k_lam_G > 0 as libc::c_int { N - k_lam_G } else { 0 as libc::c_int },
            eta,
            x,
            G_lam_min.val,
            Gp_lam_min.val,
            &mut G_lam_G,
            &mut Gp_lam_G,
        );
        (*F).val = F_lam_F;
        (*F).err = F_lam_F_err;
        (*F).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs(F_lam_F);
        (*Fp).val = Fp_lam_F;
        (*Fp).err = Fp_lam_F_err;
        (*Fp).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs(Fp_lam_F);
        Gerr_frac = fabs(G_lam_min.err / G_lam_min.val)
            + fabs(Gp_lam_min.err / Gp_lam_min.val);
        (*G).val = G_lam_G;
        (*G).err = Gerr_frac * fabs(G_lam_G);
        (*G).err
            += 2.0f64 * (CF1_count + 1 as libc::c_int) as libc::c_double
                * 2.2204460492503131e-16f64 * fabs((*G).val);
        (*Gp).val = Gp_lam_G;
        (*Gp).err = Gerr_frac * fabs((*Gp).val);
        (*Gp).err
            += 2.0f64 * (CF1_count + 1 as libc::c_int) as libc::c_double
                * 2.2204460492503131e-16f64 * fabs((*Gp).val);
        *exp_F = 0.0f64;
        *exp_G = 0.0f64;
        return if stat_ser != GSL_SUCCESS as libc::c_int {
            stat_ser
        } else if stat_CF1 != GSL_SUCCESS as libc::c_int {
            stat_CF1
        } else if stat_Fr != GSL_SUCCESS as libc::c_int {
            stat_Fr
        } else if stat_Gr != GSL_SUCCESS as libc::c_int {
            stat_Gr
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if x < 2.0f64 * eta {
        let mut F_lam_F_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut G_lam_F: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut F_lam_G: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut G_lam_G_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut exp_lam_F: libc::c_double = 0.;
        let mut exp_lam_G: libc::c_double = 0.;
        let mut stat_lam_F: libc::c_int = 0;
        let mut stat_lam_G: libc::c_int = 0;
        let mut stat_CF1_lam_F: libc::c_int = 0;
        let mut stat_CF1_lam_G: libc::c_int = 0;
        let mut CF1_count_0: libc::c_int = 0;
        let mut Fp_over_F_lam_F_0: libc::c_double = 0.;
        let mut Fp_over_F_lam_G: libc::c_double = 0.;
        let mut F_sign_lam_F_0: libc::c_double = 0.;
        let mut F_sign_lam_G: libc::c_double = 0.;
        stat_lam_F = coulomb_jwkb(
            lam_F,
            eta,
            x,
            &mut F_lam_F_0,
            &mut G_lam_F,
            &mut exp_lam_F,
        );
        if k_lam_G == 0 as libc::c_int {
            stat_lam_G = stat_lam_F;
            F_lam_G = F_lam_F_0;
            G_lam_G_0 = G_lam_F;
            exp_lam_G = exp_lam_F;
        } else {
            stat_lam_G = coulomb_jwkb(
                lam_G,
                eta,
                x,
                &mut F_lam_G,
                &mut G_lam_G_0,
                &mut exp_lam_G,
            );
        }
        stat_CF1_lam_F = coulomb_CF1(
            lam_F,
            eta,
            x,
            &mut F_sign_lam_F_0,
            &mut Fp_over_F_lam_F_0,
            &mut CF1_count_0,
        );
        if k_lam_G == 0 as libc::c_int {
            stat_CF1_lam_G = stat_CF1_lam_F;
            F_sign_lam_G = F_sign_lam_F_0;
            Fp_over_F_lam_G = Fp_over_F_lam_F_0;
        } else {
            stat_CF1_lam_G = coulomb_CF1(
                lam_G,
                eta,
                x,
                &mut F_sign_lam_G,
                &mut Fp_over_F_lam_G,
                &mut CF1_count_0,
            );
        }
        (*F).val = F_lam_F_0.val;
        (*F).err = F_lam_F_0.err;
        (*G).val = G_lam_G_0.val;
        (*G).err = G_lam_G_0.err;
        (*Fp).val = Fp_over_F_lam_F_0 * F_lam_F_0.val;
        (*Fp).err = fabs(Fp_over_F_lam_F_0) * F_lam_F_0.err;
        (*Fp).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*Fp).val);
        (*Gp).val = Fp_over_F_lam_G * G_lam_G_0.val - 1.0f64 / F_lam_G.val;
        (*Gp).err = fabs(Fp_over_F_lam_G) * G_lam_G_0.err;
        (*Gp).err += fabs(1.0f64 / F_lam_G.val) * fabs(F_lam_G.err / F_lam_G.val);
        *exp_F = exp_lam_F;
        *exp_G = exp_lam_G;
        if stat_lam_F == GSL_EOVRFLW as libc::c_int
            || stat_lam_G == GSL_EOVRFLW as libc::c_int
        {
            gsl_error(
                b"overflow\0" as *const u8 as *const libc::c_char,
                b"coulomb.c\0" as *const u8 as *const libc::c_char,
                1107 as libc::c_int,
                GSL_EOVRFLW as libc::c_int,
            );
            return GSL_EOVRFLW as libc::c_int;
        } else {
            return if stat_lam_F != GSL_SUCCESS as libc::c_int {
                stat_lam_F
            } else if stat_lam_G != GSL_SUCCESS as libc::c_int {
                stat_lam_G
            } else {
                GSL_SUCCESS as libc::c_int
            }
        }
    } else {
        let SMALL_0: libc::c_double = 1.4901161193847656e-08f64;
        let C: libc::c_double = sqrt(1.0f64 + 4.0f64 * x * (x - 2.0f64 * eta));
        let N_0: libc::c_int = ceil(lam_F - C + 0.5f64) as libc::c_int;
        let lam_0: libc::c_double = lam_F
            - (if N_0 > 0 as libc::c_int { N_0 } else { 0 as libc::c_int })
                as libc::c_double;
        let lam_min_0: libc::c_double = if lam_0 < lam_G { lam_0 } else { lam_G };
        let mut F_lam_F_1: libc::c_double = 0.;
        let mut Fp_lam_F_0: libc::c_double = 0.;
        let mut G_lam_G_1: libc::c_double = 0.;
        let mut Gp_lam_G_0: libc::c_double = 0.;
        let mut F_lam_min_unnorm_0: libc::c_double = 0.;
        let mut Fp_lam_min_unnorm_0: libc::c_double = 0.;
        let mut F_lam_min_0: libc::c_double = 0.;
        let mut Fp_lam_min: libc::c_double = 0.;
        let mut G_lam_min_0: libc::c_double = 0.;
        let mut Gp_lam_min_0: libc::c_double = 0.;
        let mut Fp_over_F_lam_F_1: libc::c_double = 0.;
        let mut Fp_over_F_lam_min_0: libc::c_double = 0.;
        let mut F_sign_lam_F_1: libc::c_double = 0.;
        let mut F_sign_lam_min: libc::c_double = 0.;
        let mut P_lam_min: libc::c_double = 0.;
        let mut Q_lam_min: libc::c_double = 0.;
        let mut alpha: libc::c_double = 0.;
        let mut gamma: libc::c_double = 0.;
        let mut F_scale_0: libc::c_double = 0.;
        let mut CF1_count_1: libc::c_int = 0;
        let mut CF2_count: libc::c_int = 0;
        let mut stat_CF1_0: libc::c_int = coulomb_CF1(
            lam_F,
            eta,
            x,
            &mut F_sign_lam_F_1,
            &mut Fp_over_F_lam_F_1,
            &mut CF1_count_1,
        );
        let mut stat_CF2: libc::c_int = 0;
        let mut stat_Fr_0: libc::c_int = 0;
        let mut stat_Gr_0: libc::c_int = 0;
        let mut F_recur_count: libc::c_int = 0;
        let mut G_recur_count: libc::c_int = 0;
        let mut err_amplify: libc::c_double = 0.;
        F_lam_F_1 = F_sign_lam_F_1 * SMALL_0;
        Fp_lam_F_0 = Fp_over_F_lam_F_1 * F_lam_F_1;
        F_recur_count = if k_lam_G > N_0 { k_lam_G } else { N_0 };
        stat_Fr_0 = coulomb_F_recur(
            lam_min_0,
            F_recur_count,
            eta,
            x,
            F_lam_F_1,
            Fp_lam_F_0,
            &mut F_lam_min_unnorm_0,
            &mut Fp_lam_min_unnorm_0,
        );
        Fp_over_F_lam_min_0 = Fp_lam_min_unnorm_0 / F_lam_min_unnorm_0;
        stat_CF2 = coulomb_CF2(
            lam_min_0,
            eta,
            x,
            &mut P_lam_min,
            &mut Q_lam_min,
            &mut CF2_count,
        );
        alpha = Fp_over_F_lam_min_0 - P_lam_min;
        gamma = alpha / Q_lam_min;
        F_sign_lam_min = (if F_lam_min_unnorm_0 >= 0.0f64 {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_double;
        F_lam_min_0 = F_sign_lam_min / sqrt(alpha * alpha / Q_lam_min + Q_lam_min);
        Fp_lam_min = Fp_over_F_lam_min_0 * F_lam_min_0;
        G_lam_min_0 = gamma * F_lam_min_0;
        Gp_lam_min_0 = (P_lam_min * gamma - Q_lam_min) * F_lam_min_0;
        F_scale_0 = F_lam_min_0 / F_lam_min_unnorm_0;
        F_lam_F_1 *= F_scale_0;
        Fp_lam_F_0 *= F_scale_0;
        G_recur_count = if N_0 - k_lam_G > 0 as libc::c_int {
            N_0 - k_lam_G
        } else {
            0 as libc::c_int
        };
        stat_Gr_0 = coulomb_G_recur(
            lam_min_0,
            G_recur_count,
            eta,
            x,
            G_lam_min_0,
            Gp_lam_min_0,
            &mut G_lam_G_1,
            &mut Gp_lam_G_0,
        );
        err_amplify = (CF1_count_1 + CF2_count + F_recur_count + G_recur_count
            + 1 as libc::c_int) as libc::c_double;
        (*F).val = F_lam_F_1;
        (*F).err = 8.0f64 * err_amplify * 2.2204460492503131e-16f64 * fabs((*F).val);
        (*Fp).val = Fp_lam_F_0;
        (*Fp).err = 8.0f64 * err_amplify * 2.2204460492503131e-16f64 * fabs((*Fp).val);
        (*G).val = G_lam_G_1;
        (*G).err = 8.0f64 * err_amplify * 2.2204460492503131e-16f64 * fabs((*G).val);
        (*Gp).val = Gp_lam_G_0;
        (*Gp).err = 8.0f64 * err_amplify * 2.2204460492503131e-16f64 * fabs((*Gp).val);
        *exp_F = 0.0f64;
        *exp_G = 0.0f64;
        return if stat_CF1_0 != GSL_SUCCESS as libc::c_int {
            stat_CF1_0
        } else if stat_CF2 != GSL_SUCCESS as libc::c_int {
            stat_CF2
        } else if stat_Fr_0 != GSL_SUCCESS as libc::c_int {
            stat_Fr_0
        } else if stat_Gr_0 != GSL_SUCCESS as libc::c_int {
            stat_Gr_0
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_coulomb_wave_F_array(
    mut lam_min: libc::c_double,
    mut kmax: libc::c_int,
    mut eta: libc::c_double,
    mut x: libc::c_double,
    mut fc_array: *mut libc::c_double,
    mut F_exp: *mut libc::c_double,
) -> libc::c_int {
    if x == 0.0f64 {
        let mut k: libc::c_int = 0;
        *F_exp = 0.0f64;
        k = 0 as libc::c_int;
        while k <= kmax {
            *fc_array.offset(k as isize) = 0.0f64;
            k += 1;
            k;
        }
        if lam_min == 0.0f64 {
            let mut f_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            CLeta(0.0f64, eta, &mut f_0);
            *fc_array.offset(0 as libc::c_int as isize) = f_0.val;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        let x_inv: libc::c_double = 1.0f64 / x;
        let lam_max: libc::c_double = lam_min + kmax as libc::c_double;
        let mut F: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut Fp: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut G: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut Gp: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut G_exp: libc::c_double = 0.;
        let mut stat_FG: libc::c_int = gsl_sf_coulomb_wave_FG_e(
            eta,
            x,
            lam_max,
            0 as libc::c_int,
            &mut F,
            &mut Fp,
            &mut G,
            &mut Gp,
            F_exp,
            &mut G_exp,
        );
        let mut fcl: libc::c_double = F.val;
        let mut fpl: libc::c_double = Fp.val;
        let mut lam: libc::c_double = lam_max;
        let mut k_0: libc::c_int = 0;
        *fc_array.offset(kmax as isize) = F.val;
        k_0 = kmax - 1 as libc::c_int;
        while k_0 >= 0 as libc::c_int {
            let mut el: libc::c_double = eta / lam;
            let mut rl: libc::c_double = hypot(1.0f64, el);
            let mut sl: libc::c_double = el + lam * x_inv;
            let mut fc_lm1: libc::c_double = (fcl * sl + fpl) / rl;
            *fc_array.offset(k_0 as isize) = fc_lm1;
            fpl = fc_lm1 * sl - fcl * rl;
            fcl = fc_lm1;
            lam -= 1.0f64;
            k_0 -= 1;
            k_0;
        }
        return stat_FG;
    };
}
pub unsafe extern "C" fn gsl_sf_coulomb_wave_FG_array(
    mut lam_min: libc::c_double,
    mut kmax: libc::c_int,
    mut eta: libc::c_double,
    mut x: libc::c_double,
    mut fc_array: *mut libc::c_double,
    mut gc_array: *mut libc::c_double,
    mut F_exp: *mut libc::c_double,
    mut G_exp: *mut libc::c_double,
) -> libc::c_int {
    let x_inv: libc::c_double = 1.0f64 / x;
    let lam_max: libc::c_double = lam_min + kmax as libc::c_double;
    let mut F: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut Fp: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut G: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut Gp: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_FG: libc::c_int = gsl_sf_coulomb_wave_FG_e(
        eta,
        x,
        lam_max,
        kmax,
        &mut F,
        &mut Fp,
        &mut G,
        &mut Gp,
        F_exp,
        G_exp,
    );
    let mut fcl: libc::c_double = F.val;
    let mut fpl: libc::c_double = Fp.val;
    let mut lam: libc::c_double = lam_max;
    let mut k: libc::c_int = 0;
    let mut gcl: libc::c_double = 0.;
    let mut gpl: libc::c_double = 0.;
    *fc_array.offset(kmax as isize) = F.val;
    k = kmax - 1 as libc::c_int;
    while k >= 0 as libc::c_int {
        let mut el: libc::c_double = eta / lam;
        let mut rl: libc::c_double = hypot(1.0f64, el);
        let mut sl: libc::c_double = el + lam * x_inv;
        let mut fc_lm1: libc::c_double = 0.;
        fc_lm1 = (fcl * sl + fpl) / rl;
        *fc_array.offset(k as isize) = fc_lm1;
        fpl = fc_lm1 * sl - fcl * rl;
        fcl = fc_lm1;
        lam -= 1.0f64;
        k -= 1;
        k;
    }
    gcl = G.val;
    gpl = Gp.val;
    lam = lam_min + 1.0f64;
    *gc_array.offset(0 as libc::c_int as isize) = G.val;
    k = 1 as libc::c_int;
    while k <= kmax {
        let mut el_0: libc::c_double = eta / lam;
        let mut rl_0: libc::c_double = hypot(1.0f64, el_0);
        let mut sl_0: libc::c_double = el_0 + lam * x_inv;
        let mut gcl1: libc::c_double = (sl_0 * gcl - gpl) / rl_0;
        *gc_array.offset(k as isize) = gcl1;
        gpl = rl_0 * gcl - sl_0 * gcl1;
        gcl = gcl1;
        lam += 1.0f64;
        k += 1;
        k;
    }
    return stat_FG;
}
pub unsafe extern "C" fn gsl_sf_coulomb_wave_FGp_array(
    mut lam_min: libc::c_double,
    mut kmax: libc::c_int,
    mut eta: libc::c_double,
    mut x: libc::c_double,
    mut fc_array: *mut libc::c_double,
    mut fcp_array: *mut libc::c_double,
    mut gc_array: *mut libc::c_double,
    mut gcp_array: *mut libc::c_double,
    mut F_exp: *mut libc::c_double,
    mut G_exp: *mut libc::c_double,
) -> libc::c_int {
    let x_inv: libc::c_double = 1.0f64 / x;
    let lam_max: libc::c_double = lam_min + kmax as libc::c_double;
    let mut F: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut Fp: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut G: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut Gp: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut stat_FG: libc::c_int = gsl_sf_coulomb_wave_FG_e(
        eta,
        x,
        lam_max,
        kmax,
        &mut F,
        &mut Fp,
        &mut G,
        &mut Gp,
        F_exp,
        G_exp,
    );
    let mut fcl: libc::c_double = F.val;
    let mut fpl: libc::c_double = Fp.val;
    let mut lam: libc::c_double = lam_max;
    let mut k: libc::c_int = 0;
    let mut gcl: libc::c_double = 0.;
    let mut gpl: libc::c_double = 0.;
    *fc_array.offset(kmax as isize) = F.val;
    *fcp_array.offset(kmax as isize) = Fp.val;
    k = kmax - 1 as libc::c_int;
    while k >= 0 as libc::c_int {
        let mut el: libc::c_double = eta / lam;
        let mut rl: libc::c_double = hypot(1.0f64, el);
        let mut sl: libc::c_double = el + lam * x_inv;
        let mut fc_lm1: libc::c_double = 0.;
        fc_lm1 = (fcl * sl + fpl) / rl;
        *fc_array.offset(k as isize) = fc_lm1;
        fpl = fc_lm1 * sl - fcl * rl;
        *fcp_array.offset(k as isize) = fpl;
        fcl = fc_lm1;
        lam -= 1.0f64;
        k -= 1;
        k;
    }
    gcl = G.val;
    gpl = Gp.val;
    lam = lam_min + 1.0f64;
    *gc_array.offset(0 as libc::c_int as isize) = G.val;
    *gcp_array.offset(0 as libc::c_int as isize) = Gp.val;
    k = 1 as libc::c_int;
    while k <= kmax {
        let mut el_0: libc::c_double = eta / lam;
        let mut rl_0: libc::c_double = hypot(1.0f64, el_0);
        let mut sl_0: libc::c_double = el_0 + lam * x_inv;
        let mut gcl1: libc::c_double = (sl_0 * gcl - gpl) / rl_0;
        *gc_array.offset(k as isize) = gcl1;
        gpl = rl_0 * gcl - sl_0 * gcl1;
        *gcp_array.offset(k as isize) = gpl;
        gcl = gcl1;
        lam += 1.0f64;
        k += 1;
        k;
    }
    return stat_FG;
}
pub unsafe extern "C" fn gsl_sf_coulomb_wave_sphF_array(
    mut lam_min: libc::c_double,
    mut kmax: libc::c_int,
    mut eta: libc::c_double,
    mut x: libc::c_double,
    mut fc_array: *mut libc::c_double,
    mut F_exp: *mut libc::c_double,
) -> libc::c_int {
    if x < 0.0f64 || lam_min < -0.5f64 {
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"coulomb.c\0" as *const u8 as *const libc::c_char,
            1387 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x < 10.0f64 / 1.7976931348623157e+308f64 {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while k <= kmax {
            *fc_array.offset(k as isize) = 0.0f64;
            k += 1;
            k;
        }
        if lam_min == 0.0f64 {
            *fc_array.offset(0 as libc::c_int as isize) = sqrt(C0sq(eta));
        }
        *F_exp = 0.0f64;
        if x == 0.0f64 {
            return GSL_SUCCESS as libc::c_int
        } else {
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"coulomb.c\0" as *const u8 as *const libc::c_char,
                1401 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        }
    } else {
        let mut k_0: libc::c_int = 0;
        let mut stat_F: libc::c_int = gsl_sf_coulomb_wave_F_array(
            lam_min,
            kmax,
            eta,
            x,
            fc_array,
            F_exp,
        );
        k_0 = 0 as libc::c_int;
        while k_0 <= kmax {
            *fc_array.offset(k_0 as isize) = *fc_array.offset(k_0 as isize) / x;
            k_0 += 1;
            k_0;
        }
        return stat_F;
    };
}
