use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn log1p(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_sf_gamma_inc_Q(a: libc::c_double, x: libc::c_double) -> libc::c_double;
    fn gsl_sf_gamma_inc_P(a: libc::c_double, x: libc::c_double) -> libc::c_double;
    fn gsl_sf_lnbeta(a: libc::c_double, b: libc::c_double) -> libc::c_double;
}
unsafe extern "C" fn beta_cont_frac(
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
    epsabs: libc::c_double,
) -> libc::c_double {
    let max_iter: libc::c_uint = 512 as libc::c_int as libc::c_uint;
    let cutoff: libc::c_double = 2.0f64 * 2.2250738585072014e-308f64;
    let mut iter_count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut cf: libc::c_double = 0.;
    let mut num_term: libc::c_double = 1.0f64;
    let mut den_term: libc::c_double = 1.0f64 - (a + b) * x / (a + 1.0f64);
    if fabs(den_term) < cutoff {
        den_term = ::std::f32::NAN as libc::c_double;
    }
    den_term = 1.0f64 / den_term;
    cf = den_term;
    while iter_count < max_iter {
        let k: libc::c_int = iter_count.wrapping_add(1 as libc::c_int as libc::c_uint)
            as libc::c_int;
        let mut coeff: libc::c_double = k as libc::c_double * (b - k as libc::c_double)
            * x
            / ((a - 1.0f64 + (2 as libc::c_int * k) as libc::c_double)
                * (a + (2 as libc::c_int * k) as libc::c_double));
        let mut delta_frac: libc::c_double = 0.;
        den_term = 1.0f64 + coeff * den_term;
        num_term = 1.0f64 + coeff / num_term;
        if fabs(den_term) < cutoff {
            den_term = ::std::f32::NAN as libc::c_double;
        }
        if fabs(num_term) < cutoff {
            num_term = ::std::f32::NAN as libc::c_double;
        }
        den_term = 1.0f64 / den_term;
        delta_frac = den_term * num_term;
        cf *= delta_frac;
        coeff = -(a + k as libc::c_double) * (a + b + k as libc::c_double) * x
            / ((a + (2 as libc::c_int * k) as libc::c_double)
                * (a + (2 as libc::c_int * k) as libc::c_double + 1.0f64));
        den_term = 1.0f64 + coeff * den_term;
        num_term = 1.0f64 + coeff / num_term;
        if fabs(den_term) < cutoff {
            den_term = ::std::f32::NAN as libc::c_double;
        }
        if fabs(num_term) < cutoff {
            num_term = ::std::f32::NAN as libc::c_double;
        }
        den_term = 1.0f64 / den_term;
        delta_frac = den_term * num_term;
        cf *= delta_frac;
        if fabs(delta_frac - 1.0f64) < 2.0f64 * 2.2204460492503131e-16f64 {
            break;
        }
        if cf * fabs(delta_frac - 1.0f64) < epsabs {
            break;
        }
        iter_count = iter_count.wrapping_add(1);
        iter_count;
    }
    if iter_count >= max_iter {
        return ::std::f32::NAN as libc::c_double;
    }
    return cf;
}
unsafe extern "C" fn beta_inc_AXPY(
    A: libc::c_double,
    Y: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
    x: libc::c_double,
) -> libc::c_double {
    if x == 0.0f64 {
        return A * 0 as libc::c_int as libc::c_double + Y
    } else if x == 1.0f64 {
        return A * 1 as libc::c_int as libc::c_double + Y
    } else if a > 1e5f64 && b < 10 as libc::c_int as libc::c_double && x > a / (a + b) {
        let mut N: libc::c_double = a + (b - 1.0f64) / 2.0f64;
        return A * gsl_sf_gamma_inc_Q(b, -N * log(x)) + Y;
    } else if b > 1e5f64 && a < 10 as libc::c_int as libc::c_double && x < b / (a + b) {
        let mut N_0: libc::c_double = b + (a - 1.0f64) / 2.0f64;
        return A * gsl_sf_gamma_inc_P(a, -N_0 * log1p(-x)) + Y;
    } else {
        let mut ln_beta: libc::c_double = gsl_sf_lnbeta(a, b);
        let mut ln_pre: libc::c_double = -ln_beta + a * log(x) + b * log1p(-x);
        let mut prefactor: libc::c_double = exp(ln_pre);
        if x < (a + 1.0f64) / (a + b + 2.0f64) {
            let mut epsabs: libc::c_double = fabs(Y / (A * prefactor / a))
                * 2.2204460492503131e-16f64;
            let mut cf: libc::c_double = beta_cont_frac(a, b, x, epsabs);
            return A * (prefactor * cf / a) + Y;
        } else {
            let mut epsabs_0: libc::c_double = fabs((A + Y) / (A * prefactor / b))
                * 2.2204460492503131e-16f64;
            let mut cf_0: libc::c_double = beta_cont_frac(b, a, 1.0f64 - x, epsabs_0);
            let mut term: libc::c_double = prefactor * cf_0 / b;
            if A == -Y {
                return -A * term
            } else {
                return A * (1 as libc::c_int as libc::c_double - term) + Y
            }
        }
    };
}
pub unsafe extern "C" fn gsl_cdf_beta_P(
    x: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut P: libc::c_double = 0.;
    if x <= 0.0f64 {
        return 0.0f64;
    }
    if x >= 1.0f64 {
        return 1.0f64;
    }
    P = beta_inc_AXPY(1.0f64, 0.0f64, a, b, x);
    return P;
}
pub unsafe extern "C" fn gsl_cdf_beta_Q(
    x: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut Q: libc::c_double = 0.;
    if x >= 1.0f64 {
        return 0.0f64;
    }
    if x <= 0.0f64 {
        return 1.0f64;
    }
    Q = beta_inc_AXPY(-1.0f64, 1.0f64, a, b, x);
    return Q;
}
