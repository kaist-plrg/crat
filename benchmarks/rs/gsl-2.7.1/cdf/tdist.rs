use ::libc;
extern "C" {
    fn gsl_cdf_ugaussian_P(x: libc::c_double) -> libc::c_double;
    fn gsl_cdf_ugaussian_Q(x: libc::c_double) -> libc::c_double;
    fn gsl_sf_gamma_inc_Q(a: libc::c_double, x: libc::c_double) -> libc::c_double;
    fn gsl_sf_gamma_inc_P(a: libc::c_double, x: libc::c_double) -> libc::c_double;
    fn gsl_sf_lnbeta(a: libc::c_double, b: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn log1p(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
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
unsafe extern "C" fn poly_eval(
    mut c: *const libc::c_double,
    mut n: libc::c_uint,
    mut x: libc::c_double,
) -> libc::c_double {
    let mut i: libc::c_uint = 0;
    let mut y: libc::c_double = *c.offset(0 as libc::c_int as isize) * x;
    i = 1 as libc::c_int as libc::c_uint;
    while i < n {
        y = x * (y + *c.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    y += *c.offset(n as isize);
    return y;
}
unsafe extern "C" fn cornish_fisher(
    mut t: libc::c_double,
    mut n: libc::c_double,
) -> libc::c_double {
    let coeffs6: [libc::c_double; 10] = [
        0.265974025974025974026f64,
        5.449696969696969696970f64,
        122.20294372294372294372f64,
        2354.7298701298701298701f64,
        37625.00902597402597403f64,
        486996.1392857142857143f64,
        4960870.65f64,
        37978595.55f64,
        201505390.875f64,
        622437908.625f64,
    ];
    let coeffs5: [libc::c_double; 8] = [
        0.2742857142857142857142f64,
        4.499047619047619047619f64,
        78.45142857142857142857f64,
        1118.710714285714285714f64,
        12387.6f64,
        101024.55f64,
        559494.0f64,
        1764959.625f64,
    ];
    let coeffs4: [libc::c_double; 6] = [
        0.3047619047619047619048f64,
        3.752380952380952380952f64,
        46.67142857142857142857f64,
        427.5f64,
        2587.5f64,
        8518.5f64,
    ];
    let coeffs3: [libc::c_double; 4] = [0.4f64, 3.3f64, 24.0f64, 85.5f64];
    let mut a: libc::c_double = n - 0.5f64;
    let mut b: libc::c_double = 48.0f64 * a * a;
    let mut z2: libc::c_double = a * log1p(t * t / n);
    let mut z: libc::c_double = sqrt(z2);
    let mut p5: libc::c_double = z
        * poly_eval(coeffs6.as_ptr(), 9 as libc::c_int as libc::c_uint, z2);
    let mut p4: libc::c_double = -z
        * poly_eval(coeffs5.as_ptr(), 7 as libc::c_int as libc::c_uint, z2);
    let mut p3: libc::c_double = z
        * poly_eval(coeffs4.as_ptr(), 5 as libc::c_int as libc::c_uint, z2);
    let mut p2: libc::c_double = -z
        * poly_eval(coeffs3.as_ptr(), 3 as libc::c_int as libc::c_uint, z2);
    let mut p1: libc::c_double = z * (z2 + 3.0f64);
    let mut p0: libc::c_double = z;
    let mut y: libc::c_double = p5;
    y = y / b + p4;
    y = y / b + p3;
    y = y / b + p2;
    y = y / b + p1;
    y = y / b + p0;
    if t < 0 as libc::c_int as libc::c_double {
        y *= -(1 as libc::c_int) as libc::c_double;
    }
    return y;
}
pub unsafe extern "C" fn gsl_cdf_tdist_P(
    x: libc::c_double,
    nu: libc::c_double,
) -> libc::c_double {
    let mut P: libc::c_double = 0.;
    let mut x2: libc::c_double = x * x;
    if nu > 30 as libc::c_int as libc::c_double
        && x2 < 10 as libc::c_int as libc::c_double * nu
    {
        let mut u: libc::c_double = cornish_fisher(x, nu);
        P = gsl_cdf_ugaussian_P(u);
        return P;
    }
    if x2 < nu {
        let mut u_0: libc::c_double = x2 / nu;
        let mut eps: libc::c_double = u_0 / (1 as libc::c_int as libc::c_double + u_0);
        if x >= 0 as libc::c_int as libc::c_double {
            P = beta_inc_AXPY(0.5f64, 0.5f64, 0.5f64, nu / 2.0f64, eps);
        } else {
            P = beta_inc_AXPY(-0.5f64, 0.5f64, 0.5f64, nu / 2.0f64, eps);
        }
    } else {
        let mut v: libc::c_double = nu / (x * x);
        let mut eps_0: libc::c_double = v / (1 as libc::c_int as libc::c_double + v);
        if x >= 0 as libc::c_int as libc::c_double {
            P = beta_inc_AXPY(-0.5f64, 1.0f64, nu / 2.0f64, 0.5f64, eps_0);
        } else {
            P = beta_inc_AXPY(0.5f64, 0.0f64, nu / 2.0f64, 0.5f64, eps_0);
        }
    }
    return P;
}
pub unsafe extern "C" fn gsl_cdf_tdist_Q(
    x: libc::c_double,
    nu: libc::c_double,
) -> libc::c_double {
    let mut Q: libc::c_double = 0.;
    let mut x2: libc::c_double = x * x;
    if nu > 30 as libc::c_int as libc::c_double
        && x2 < 10 as libc::c_int as libc::c_double * nu
    {
        let mut u: libc::c_double = cornish_fisher(x, nu);
        Q = gsl_cdf_ugaussian_Q(u);
        return Q;
    }
    if x2 < nu {
        let mut u_0: libc::c_double = x2 / nu;
        let mut eps: libc::c_double = u_0 / (1 as libc::c_int as libc::c_double + u_0);
        if x >= 0 as libc::c_int as libc::c_double {
            Q = beta_inc_AXPY(-0.5f64, 0.5f64, 0.5f64, nu / 2.0f64, eps);
        } else {
            Q = beta_inc_AXPY(0.5f64, 0.5f64, 0.5f64, nu / 2.0f64, eps);
        }
    } else {
        let mut v: libc::c_double = nu / (x * x);
        let mut eps_0: libc::c_double = v / (1 as libc::c_int as libc::c_double + v);
        if x >= 0 as libc::c_int as libc::c_double {
            Q = beta_inc_AXPY(0.5f64, 0.0f64, nu / 2.0f64, 0.5f64, eps_0);
        } else {
            Q = beta_inc_AXPY(-0.5f64, 1.0f64, nu / 2.0f64, 0.5f64, eps_0);
        }
    }
    return Q;
}
