use ::libc;
extern "C" {
    fn acos(_: libc::c_double) -> libc::c_double;
    fn asin(_: libc::c_double) -> libc::c_double;
    fn atan(_: libc::c_double) -> libc::c_double;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn cosh(_: libc::c_double) -> libc::c_double;
    fn sinh(_: libc::c_double) -> libc::c_double;
    fn tanh(_: libc::c_double) -> libc::c_double;
    fn acosh(_: libc::c_double) -> libc::c_double;
    fn atanh(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn log1p(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_complex {
    pub dat: [libc::c_double; 2],
}
pub unsafe extern "C" fn gsl_complex_polar(
    mut r: libc::c_double,
    mut theta: libc::c_double,
) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = r * cos(theta);
    z.dat[1 as libc::c_int as usize] = r * sin(theta);
    return z;
}
pub unsafe extern "C" fn gsl_complex_arg(mut z: gsl_complex) -> libc::c_double {
    let mut x: libc::c_double = z.dat[0 as libc::c_int as usize];
    let mut y: libc::c_double = z.dat[1 as libc::c_int as usize];
    if x == 0.0f64 && y == 0.0f64 {
        return 0 as libc::c_int as libc::c_double;
    }
    return atan2(y, x);
}
pub unsafe extern "C" fn gsl_complex_abs(mut z: gsl_complex) -> libc::c_double {
    return hypot(z.dat[0 as libc::c_int as usize], z.dat[1 as libc::c_int as usize]);
}
pub unsafe extern "C" fn gsl_complex_abs2(mut z: gsl_complex) -> libc::c_double {
    let mut x: libc::c_double = z.dat[0 as libc::c_int as usize];
    let mut y: libc::c_double = z.dat[1 as libc::c_int as usize];
    return x * x + y * y;
}
pub unsafe extern "C" fn gsl_complex_logabs(mut z: gsl_complex) -> libc::c_double {
    let mut xabs: libc::c_double = fabs(z.dat[0 as libc::c_int as usize]);
    let mut yabs: libc::c_double = fabs(z.dat[1 as libc::c_int as usize]);
    let mut max: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    if xabs >= yabs {
        max = xabs;
        u = yabs / xabs;
    } else {
        max = yabs;
        u = xabs / yabs;
    }
    return log(max) + 0.5f64 * log1p(u * u);
}
pub unsafe extern "C" fn gsl_complex_add(
    mut a: gsl_complex,
    mut b: gsl_complex,
) -> gsl_complex {
    let mut ar: libc::c_double = a.dat[0 as libc::c_int as usize];
    let mut ai: libc::c_double = a.dat[1 as libc::c_int as usize];
    let mut br: libc::c_double = b.dat[0 as libc::c_int as usize];
    let mut bi: libc::c_double = b.dat[1 as libc::c_int as usize];
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = ar + br;
    z.dat[1 as libc::c_int as usize] = ai + bi;
    return z;
}
pub unsafe extern "C" fn gsl_complex_add_real(
    mut a: gsl_complex,
    mut x: libc::c_double,
) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = a.dat[0 as libc::c_int as usize] + x;
    z.dat[1 as libc::c_int as usize] = a.dat[1 as libc::c_int as usize];
    return z;
}
pub unsafe extern "C" fn gsl_complex_add_imag(
    mut a: gsl_complex,
    mut y: libc::c_double,
) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = a.dat[0 as libc::c_int as usize];
    z.dat[1 as libc::c_int as usize] = a.dat[1 as libc::c_int as usize] + y;
    return z;
}
pub unsafe extern "C" fn gsl_complex_sub(
    mut a: gsl_complex,
    mut b: gsl_complex,
) -> gsl_complex {
    let mut ar: libc::c_double = a.dat[0 as libc::c_int as usize];
    let mut ai: libc::c_double = a.dat[1 as libc::c_int as usize];
    let mut br: libc::c_double = b.dat[0 as libc::c_int as usize];
    let mut bi: libc::c_double = b.dat[1 as libc::c_int as usize];
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = ar - br;
    z.dat[1 as libc::c_int as usize] = ai - bi;
    return z;
}
pub unsafe extern "C" fn gsl_complex_sub_real(
    mut a: gsl_complex,
    mut x: libc::c_double,
) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = a.dat[0 as libc::c_int as usize] - x;
    z.dat[1 as libc::c_int as usize] = a.dat[1 as libc::c_int as usize];
    return z;
}
pub unsafe extern "C" fn gsl_complex_sub_imag(
    mut a: gsl_complex,
    mut y: libc::c_double,
) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = a.dat[0 as libc::c_int as usize];
    z.dat[1 as libc::c_int as usize] = a.dat[1 as libc::c_int as usize] - y;
    return z;
}
pub unsafe extern "C" fn gsl_complex_mul(
    mut a: gsl_complex,
    mut b: gsl_complex,
) -> gsl_complex {
    let mut ar: libc::c_double = a.dat[0 as libc::c_int as usize];
    let mut ai: libc::c_double = a.dat[1 as libc::c_int as usize];
    let mut br: libc::c_double = b.dat[0 as libc::c_int as usize];
    let mut bi: libc::c_double = b.dat[1 as libc::c_int as usize];
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = ar * br - ai * bi;
    z.dat[1 as libc::c_int as usize] = ar * bi + ai * br;
    return z;
}
pub unsafe extern "C" fn gsl_complex_mul_real(
    mut a: gsl_complex,
    mut x: libc::c_double,
) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = x * a.dat[0 as libc::c_int as usize];
    z.dat[1 as libc::c_int as usize] = x * a.dat[1 as libc::c_int as usize];
    return z;
}
pub unsafe extern "C" fn gsl_complex_mul_imag(
    mut a: gsl_complex,
    mut y: libc::c_double,
) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = -y * a.dat[1 as libc::c_int as usize];
    z.dat[1 as libc::c_int as usize] = y * a.dat[0 as libc::c_int as usize];
    return z;
}
pub unsafe extern "C" fn gsl_complex_div(
    mut a: gsl_complex,
    mut b: gsl_complex,
) -> gsl_complex {
    let mut ar: libc::c_double = a.dat[0 as libc::c_int as usize];
    let mut ai: libc::c_double = a.dat[1 as libc::c_int as usize];
    let mut br: libc::c_double = b.dat[0 as libc::c_int as usize];
    let mut bi: libc::c_double = b.dat[1 as libc::c_int as usize];
    let mut s: libc::c_double = 1.0f64 / gsl_complex_abs(b);
    let mut sbr: libc::c_double = s * br;
    let mut sbi: libc::c_double = s * bi;
    let mut zr: libc::c_double = (ar * sbr + ai * sbi) * s;
    let mut zi: libc::c_double = (ai * sbr - ar * sbi) * s;
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = zr;
    z.dat[1 as libc::c_int as usize] = zi;
    return z;
}
pub unsafe extern "C" fn gsl_complex_div_real(
    mut a: gsl_complex,
    mut x: libc::c_double,
) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = a.dat[0 as libc::c_int as usize] / x;
    z.dat[1 as libc::c_int as usize] = a.dat[1 as libc::c_int as usize] / x;
    return z;
}
pub unsafe extern "C" fn gsl_complex_div_imag(
    mut a: gsl_complex,
    mut y: libc::c_double,
) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = a.dat[1 as libc::c_int as usize] / y;
    z.dat[1 as libc::c_int as usize] = -a.dat[0 as libc::c_int as usize] / y;
    return z;
}
pub unsafe extern "C" fn gsl_complex_conjugate(mut a: gsl_complex) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = a.dat[0 as libc::c_int as usize];
    z.dat[1 as libc::c_int as usize] = -a.dat[1 as libc::c_int as usize];
    return z;
}
pub unsafe extern "C" fn gsl_complex_negative(mut a: gsl_complex) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = -a.dat[0 as libc::c_int as usize];
    z.dat[1 as libc::c_int as usize] = -a.dat[1 as libc::c_int as usize];
    return z;
}
pub unsafe extern "C" fn gsl_complex_inverse(mut a: gsl_complex) -> gsl_complex {
    let mut s: libc::c_double = 1.0f64 / gsl_complex_abs(a);
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = a.dat[0 as libc::c_int as usize] * s * s;
    z.dat[1 as libc::c_int as usize] = -(a.dat[1 as libc::c_int as usize] * s) * s;
    return z;
}
pub unsafe extern "C" fn gsl_complex_sqrt(mut a: gsl_complex) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    if a.dat[0 as libc::c_int as usize] == 0.0f64
        && a.dat[1 as libc::c_int as usize] == 0.0f64
    {
        z.dat[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
        z.dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    } else {
        let mut x: libc::c_double = fabs(a.dat[0 as libc::c_int as usize]);
        let mut y: libc::c_double = fabs(a.dat[1 as libc::c_int as usize]);
        let mut w: libc::c_double = 0.;
        if x >= y {
            let mut t: libc::c_double = y / x;
            w = sqrt(x) * sqrt(0.5f64 * (1.0f64 + sqrt(1.0f64 + t * t)));
        } else {
            let mut t_0: libc::c_double = x / y;
            w = sqrt(y) * sqrt(0.5f64 * (t_0 + sqrt(1.0f64 + t_0 * t_0)));
        }
        if a.dat[0 as libc::c_int as usize] >= 0.0f64 {
            let mut ai: libc::c_double = a.dat[1 as libc::c_int as usize];
            z.dat[0 as libc::c_int as usize] = w;
            z.dat[1 as libc::c_int as usize] = ai / (2.0f64 * w);
        } else {
            let mut ai_0: libc::c_double = a.dat[1 as libc::c_int as usize];
            let mut vi: libc::c_double = if ai_0 >= 0 as libc::c_int as libc::c_double {
                w
            } else {
                -w
            };
            z.dat[0 as libc::c_int as usize] = ai_0 / (2.0f64 * vi);
            z.dat[1 as libc::c_int as usize] = vi;
        }
    }
    return z;
}
pub unsafe extern "C" fn gsl_complex_sqrt_real(mut x: libc::c_double) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    if x >= 0 as libc::c_int as libc::c_double {
        z.dat[0 as libc::c_int as usize] = sqrt(x);
        z.dat[1 as libc::c_int as usize] = 0.0f64;
    } else {
        z.dat[0 as libc::c_int as usize] = 0.0f64;
        z.dat[1 as libc::c_int as usize] = sqrt(-x);
    }
    return z;
}
pub unsafe extern "C" fn gsl_complex_exp(mut a: gsl_complex) -> gsl_complex {
    let mut rho: libc::c_double = exp(a.dat[0 as libc::c_int as usize]);
    let mut theta: libc::c_double = a.dat[1 as libc::c_int as usize];
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = rho * cos(theta);
    z.dat[1 as libc::c_int as usize] = rho * sin(theta);
    return z;
}
pub unsafe extern "C" fn gsl_complex_pow(
    mut a: gsl_complex,
    mut b: gsl_complex,
) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    if a.dat[0 as libc::c_int as usize] == 0 as libc::c_int as libc::c_double
        && a.dat[1 as libc::c_int as usize] == 0.0f64
    {
        if b.dat[0 as libc::c_int as usize] == 0 as libc::c_int as libc::c_double
            && b.dat[1 as libc::c_int as usize] == 0.0f64
        {
            z.dat[0 as libc::c_int as usize] = 1.0f64;
            z.dat[1 as libc::c_int as usize] = 0.0f64;
        } else {
            z.dat[0 as libc::c_int as usize] = 0.0f64;
            z.dat[1 as libc::c_int as usize] = 0.0f64;
        }
    } else if b.dat[0 as libc::c_int as usize] == 1.0f64
        && b.dat[1 as libc::c_int as usize] == 0.0f64
    {
        return a
    } else if b.dat[0 as libc::c_int as usize] == -1.0f64
        && b.dat[1 as libc::c_int as usize] == 0.0f64
    {
        return gsl_complex_inverse(a)
    } else {
        let mut logr: libc::c_double = gsl_complex_logabs(a);
        let mut theta: libc::c_double = gsl_complex_arg(a);
        let mut br: libc::c_double = b.dat[0 as libc::c_int as usize];
        let mut bi: libc::c_double = b.dat[1 as libc::c_int as usize];
        let mut rho: libc::c_double = exp(logr * br - bi * theta);
        let mut beta: libc::c_double = theta * br + bi * logr;
        z.dat[0 as libc::c_int as usize] = rho * cos(beta);
        z.dat[1 as libc::c_int as usize] = rho * sin(beta);
    }
    return z;
}
pub unsafe extern "C" fn gsl_complex_pow_real(
    mut a: gsl_complex,
    mut b: libc::c_double,
) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    if a.dat[0 as libc::c_int as usize] == 0 as libc::c_int as libc::c_double
        && a.dat[1 as libc::c_int as usize] == 0 as libc::c_int as libc::c_double
    {
        if b == 0 as libc::c_int as libc::c_double {
            z.dat[0 as libc::c_int as usize] = 1 as libc::c_int as libc::c_double;
            z.dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
        } else {
            z.dat[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
            z.dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
        }
    } else {
        let mut logr: libc::c_double = gsl_complex_logabs(a);
        let mut theta: libc::c_double = gsl_complex_arg(a);
        let mut rho: libc::c_double = exp(logr * b);
        let mut beta: libc::c_double = theta * b;
        z.dat[0 as libc::c_int as usize] = rho * cos(beta);
        z.dat[1 as libc::c_int as usize] = rho * sin(beta);
    }
    return z;
}
pub unsafe extern "C" fn gsl_complex_log(mut a: gsl_complex) -> gsl_complex {
    let mut logr: libc::c_double = gsl_complex_logabs(a);
    let mut theta: libc::c_double = gsl_complex_arg(a);
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = logr;
    z.dat[1 as libc::c_int as usize] = theta;
    return z;
}
pub unsafe extern "C" fn gsl_complex_log10(mut a: gsl_complex) -> gsl_complex {
    return gsl_complex_mul_real(
        gsl_complex_log(a),
        1 as libc::c_int as libc::c_double / log(10.0f64),
    );
}
pub unsafe extern "C" fn gsl_complex_log_b(
    mut a: gsl_complex,
    mut b: gsl_complex,
) -> gsl_complex {
    return gsl_complex_div(gsl_complex_log(a), gsl_complex_log(b));
}
pub unsafe extern "C" fn gsl_complex_sin(mut a: gsl_complex) -> gsl_complex {
    let mut R: libc::c_double = a.dat[0 as libc::c_int as usize];
    let mut I: libc::c_double = a.dat[1 as libc::c_int as usize];
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    if I == 0.0f64 {
        z.dat[0 as libc::c_int as usize] = sin(R);
        z.dat[1 as libc::c_int as usize] = 0.0f64;
    } else {
        z.dat[0 as libc::c_int as usize] = sin(R) * cosh(I);
        z.dat[1 as libc::c_int as usize] = cos(R) * sinh(I);
    }
    return z;
}
pub unsafe extern "C" fn gsl_complex_cos(mut a: gsl_complex) -> gsl_complex {
    let mut R: libc::c_double = a.dat[0 as libc::c_int as usize];
    let mut I: libc::c_double = a.dat[1 as libc::c_int as usize];
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    if I == 0.0f64 {
        z.dat[0 as libc::c_int as usize] = cos(R);
        z.dat[1 as libc::c_int as usize] = 0.0f64;
    } else {
        z.dat[0 as libc::c_int as usize] = cos(R) * cosh(I);
        z.dat[1 as libc::c_int as usize] = sin(R) * sinh(-I);
    }
    return z;
}
pub unsafe extern "C" fn gsl_complex_tan(mut a: gsl_complex) -> gsl_complex {
    let mut R: libc::c_double = a.dat[0 as libc::c_int as usize];
    let mut I: libc::c_double = a.dat[1 as libc::c_int as usize];
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    if fabs(I) < 1 as libc::c_int as libc::c_double {
        let mut D: libc::c_double = pow(cos(R), 2.0f64) + pow(sinh(I), 2.0f64);
        z
            .dat[0 as libc::c_int
            as usize] = 0.5f64 * sin(2 as libc::c_int as libc::c_double * R) / D;
        z
            .dat[1 as libc::c_int
            as usize] = 0.5f64 * sinh(2 as libc::c_int as libc::c_double * I) / D;
    } else {
        let mut D_0: libc::c_double = pow(cos(R), 2.0f64) + pow(sinh(I), 2.0f64);
        let mut F: libc::c_double = 1 as libc::c_int as libc::c_double
            + pow(cos(R) / sinh(I), 2.0f64);
        z
            .dat[0 as libc::c_int
            as usize] = 0.5f64 * sin(2 as libc::c_int as libc::c_double * R) / D_0;
        z
            .dat[1 as libc::c_int
            as usize] = 1 as libc::c_int as libc::c_double / (tanh(I) * F);
    }
    return z;
}
pub unsafe extern "C" fn gsl_complex_sec(mut a: gsl_complex) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex_cos(a);
    return gsl_complex_inverse(z);
}
pub unsafe extern "C" fn gsl_complex_csc(mut a: gsl_complex) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex_sin(a);
    return gsl_complex_inverse(z);
}
pub unsafe extern "C" fn gsl_complex_cot(mut a: gsl_complex) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex_tan(a);
    return gsl_complex_inverse(z);
}
pub unsafe extern "C" fn gsl_complex_arcsin(mut a: gsl_complex) -> gsl_complex {
    let mut R: libc::c_double = a.dat[0 as libc::c_int as usize];
    let mut I: libc::c_double = a.dat[1 as libc::c_int as usize];
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    if I == 0 as libc::c_int as libc::c_double {
        z = gsl_complex_arcsin_real(R);
    } else {
        let mut x: libc::c_double = fabs(R);
        let mut y: libc::c_double = fabs(I);
        let mut r: libc::c_double = hypot(x + 1 as libc::c_int as libc::c_double, y);
        let mut s: libc::c_double = hypot(x - 1 as libc::c_int as libc::c_double, y);
        let mut A: libc::c_double = 0.5f64 * (r + s);
        let mut B: libc::c_double = x / A;
        let mut y2: libc::c_double = y * y;
        let mut real: libc::c_double = 0.;
        let mut imag: libc::c_double = 0.;
        let A_crossover: libc::c_double = 1.5f64;
        let B_crossover: libc::c_double = 0.6417f64;
        if B <= B_crossover {
            real = asin(B);
        } else if x <= 1 as libc::c_int as libc::c_double {
            let mut D: libc::c_double = 0.5f64 * (A + x)
                * (y2 / (r + x + 1 as libc::c_int as libc::c_double)
                    + (s + (1 as libc::c_int as libc::c_double - x)));
            real = atan(x / sqrt(D));
        } else {
            let mut Apx: libc::c_double = A + x;
            let mut D_0: libc::c_double = 0.5f64
                * (Apx / (r + x + 1 as libc::c_int as libc::c_double)
                    + Apx / (s + (x - 1 as libc::c_int as libc::c_double)));
            real = atan(x / (y * sqrt(D_0)));
        }
        if A <= A_crossover {
            let mut Am1: libc::c_double = 0.;
            if x < 1 as libc::c_int as libc::c_double {
                Am1 = 0.5f64
                    * (y2 / (r + (x + 1 as libc::c_int as libc::c_double))
                        + y2 / (s + (1 as libc::c_int as libc::c_double - x)));
            } else {
                Am1 = 0.5f64
                    * (y2 / (r + (x + 1 as libc::c_int as libc::c_double))
                        + (s + (x - 1 as libc::c_int as libc::c_double)));
            }
            imag = log1p(Am1 + sqrt(Am1 * (A + 1 as libc::c_int as libc::c_double)));
        } else {
            imag = log(A + sqrt(A * A - 1 as libc::c_int as libc::c_double));
        }
        z
            .dat[0 as libc::c_int
            as usize] = if R >= 0 as libc::c_int as libc::c_double {
            real
        } else {
            -real
        };
        z
            .dat[1 as libc::c_int
            as usize] = if I >= 0 as libc::c_int as libc::c_double {
            imag
        } else {
            -imag
        };
    }
    return z;
}
pub unsafe extern "C" fn gsl_complex_arcsin_real(mut a: libc::c_double) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    if fabs(a) <= 1.0f64 {
        z.dat[0 as libc::c_int as usize] = asin(a);
        z.dat[1 as libc::c_int as usize] = 0.0f64;
    } else if a < 0.0f64 {
        z.dat[0 as libc::c_int as usize] = -1.57079632679489661923f64;
        z.dat[1 as libc::c_int as usize] = acosh(-a);
    } else {
        z.dat[0 as libc::c_int as usize] = 1.57079632679489661923f64;
        z.dat[1 as libc::c_int as usize] = -acosh(a);
    }
    return z;
}
pub unsafe extern "C" fn gsl_complex_arccos(mut a: gsl_complex) -> gsl_complex {
    let mut R: libc::c_double = a.dat[0 as libc::c_int as usize];
    let mut I: libc::c_double = a.dat[1 as libc::c_int as usize];
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    if I == 0 as libc::c_int as libc::c_double {
        z = gsl_complex_arccos_real(R);
    } else {
        let mut x: libc::c_double = fabs(R);
        let mut y: libc::c_double = fabs(I);
        let mut r: libc::c_double = hypot(x + 1 as libc::c_int as libc::c_double, y);
        let mut s: libc::c_double = hypot(x - 1 as libc::c_int as libc::c_double, y);
        let mut A: libc::c_double = 0.5f64 * (r + s);
        let mut B: libc::c_double = x / A;
        let mut y2: libc::c_double = y * y;
        let mut real: libc::c_double = 0.;
        let mut imag: libc::c_double = 0.;
        let A_crossover: libc::c_double = 1.5f64;
        let B_crossover: libc::c_double = 0.6417f64;
        if B <= B_crossover {
            real = acos(B);
        } else if x <= 1 as libc::c_int as libc::c_double {
            let mut D: libc::c_double = 0.5f64 * (A + x)
                * (y2 / (r + x + 1 as libc::c_int as libc::c_double)
                    + (s + (1 as libc::c_int as libc::c_double - x)));
            real = atan(sqrt(D) / x);
        } else {
            let mut Apx: libc::c_double = A + x;
            let mut D_0: libc::c_double = 0.5f64
                * (Apx / (r + x + 1 as libc::c_int as libc::c_double)
                    + Apx / (s + (x - 1 as libc::c_int as libc::c_double)));
            real = atan(y * sqrt(D_0) / x);
        }
        if A <= A_crossover {
            let mut Am1: libc::c_double = 0.;
            if x < 1 as libc::c_int as libc::c_double {
                Am1 = 0.5f64
                    * (y2 / (r + (x + 1 as libc::c_int as libc::c_double))
                        + y2 / (s + (1 as libc::c_int as libc::c_double - x)));
            } else {
                Am1 = 0.5f64
                    * (y2 / (r + (x + 1 as libc::c_int as libc::c_double))
                        + (s + (x - 1 as libc::c_int as libc::c_double)));
            }
            imag = log1p(Am1 + sqrt(Am1 * (A + 1 as libc::c_int as libc::c_double)));
        } else {
            imag = log(A + sqrt(A * A - 1 as libc::c_int as libc::c_double));
        }
        z
            .dat[0 as libc::c_int
            as usize] = if R >= 0 as libc::c_int as libc::c_double {
            real
        } else {
            3.14159265358979323846f64 - real
        };
        z
            .dat[1 as libc::c_int
            as usize] = if I >= 0 as libc::c_int as libc::c_double {
            -imag
        } else {
            imag
        };
    }
    return z;
}
pub unsafe extern "C" fn gsl_complex_arccos_real(mut a: libc::c_double) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    if fabs(a) <= 1.0f64 {
        z.dat[0 as libc::c_int as usize] = acos(a);
        z.dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    } else if a < 0.0f64 {
        z.dat[0 as libc::c_int as usize] = 3.14159265358979323846f64;
        z.dat[1 as libc::c_int as usize] = -acosh(-a);
    } else {
        z.dat[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
        z.dat[1 as libc::c_int as usize] = acosh(a);
    }
    return z;
}
pub unsafe extern "C" fn gsl_complex_arctan(mut a: gsl_complex) -> gsl_complex {
    let mut R: libc::c_double = a.dat[0 as libc::c_int as usize];
    let mut I: libc::c_double = a.dat[1 as libc::c_int as usize];
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    if I == 0 as libc::c_int as libc::c_double {
        z.dat[0 as libc::c_int as usize] = atan(R);
        z.dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    } else {
        let mut r: libc::c_double = hypot(R, I);
        let mut imag: libc::c_double = 0.;
        let mut u: libc::c_double = 2 as libc::c_int as libc::c_double * I
            / (1 as libc::c_int as libc::c_double + r * r);
        if fabs(u) < 0.1f64 {
            imag = 0.25f64 * (log1p(u) - log1p(-u));
        } else {
            let mut A: libc::c_double = hypot(R, I + 1 as libc::c_int as libc::c_double);
            let mut B: libc::c_double = hypot(R, I - 1 as libc::c_int as libc::c_double);
            imag = 0.5f64 * log(A / B);
        }
        if R == 0 as libc::c_int as libc::c_double {
            if I > 1 as libc::c_int as libc::c_double {
                z.dat[0 as libc::c_int as usize] = 1.57079632679489661923f64;
                z.dat[1 as libc::c_int as usize] = imag;
            } else if I < -(1 as libc::c_int) as libc::c_double {
                z.dat[0 as libc::c_int as usize] = -1.57079632679489661923f64;
                z.dat[1 as libc::c_int as usize] = imag;
            } else {
                z.dat[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
                z.dat[1 as libc::c_int as usize] = imag;
            }
        } else {
            z
                .dat[0 as libc::c_int
                as usize] = 0.5f64
                * atan2(
                    2 as libc::c_int as libc::c_double * R,
                    (1 as libc::c_int as libc::c_double + r)
                        * (1 as libc::c_int as libc::c_double - r),
                );
            z.dat[1 as libc::c_int as usize] = imag;
        }
    }
    return z;
}
pub unsafe extern "C" fn gsl_complex_arcsec(mut a: gsl_complex) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex_inverse(a);
    return gsl_complex_arccos(z);
}
pub unsafe extern "C" fn gsl_complex_arcsec_real(mut a: libc::c_double) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    if a <= -1.0f64 || a >= 1.0f64 {
        z.dat[0 as libc::c_int as usize] = acos(1 as libc::c_int as libc::c_double / a);
        z.dat[1 as libc::c_int as usize] = 0.0f64;
    } else if a >= 0.0f64 {
        z.dat[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
        z.dat[1 as libc::c_int as usize] = acosh(1 as libc::c_int as libc::c_double / a);
    } else {
        z.dat[0 as libc::c_int as usize] = 3.14159265358979323846f64;
        z
            .dat[1 as libc::c_int
            as usize] = -acosh(-(1 as libc::c_int) as libc::c_double / a);
    }
    return z;
}
pub unsafe extern "C" fn gsl_complex_arccsc(mut a: gsl_complex) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex_inverse(a);
    return gsl_complex_arcsin(z);
}
pub unsafe extern "C" fn gsl_complex_arccsc_real(mut a: libc::c_double) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    if a <= -1.0f64 || a >= 1.0f64 {
        z.dat[0 as libc::c_int as usize] = asin(1 as libc::c_int as libc::c_double / a);
        z.dat[1 as libc::c_int as usize] = 0.0f64;
    } else if a >= 0.0f64 {
        z.dat[0 as libc::c_int as usize] = 1.57079632679489661923f64;
        z
            .dat[1 as libc::c_int
            as usize] = -acosh(1 as libc::c_int as libc::c_double / a);
    } else {
        z.dat[0 as libc::c_int as usize] = -1.57079632679489661923f64;
        z
            .dat[1 as libc::c_int
            as usize] = acosh(-(1 as libc::c_int) as libc::c_double / a);
    }
    return z;
}
pub unsafe extern "C" fn gsl_complex_arccot(mut a: gsl_complex) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    if a.dat[0 as libc::c_int as usize] == 0.0f64
        && a.dat[1 as libc::c_int as usize] == 0.0f64
    {
        z.dat[0 as libc::c_int as usize] = 1.57079632679489661923f64;
        z.dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    } else {
        z = gsl_complex_inverse(a);
        z = gsl_complex_arctan(z);
    }
    return z;
}
pub unsafe extern "C" fn gsl_complex_sinh(mut a: gsl_complex) -> gsl_complex {
    let mut R: libc::c_double = a.dat[0 as libc::c_int as usize];
    let mut I: libc::c_double = a.dat[1 as libc::c_int as usize];
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = sinh(R) * cos(I);
    z.dat[1 as libc::c_int as usize] = cosh(R) * sin(I);
    return z;
}
pub unsafe extern "C" fn gsl_complex_cosh(mut a: gsl_complex) -> gsl_complex {
    let mut R: libc::c_double = a.dat[0 as libc::c_int as usize];
    let mut I: libc::c_double = a.dat[1 as libc::c_int as usize];
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = cosh(R) * cos(I);
    z.dat[1 as libc::c_int as usize] = sinh(R) * sin(I);
    return z;
}
pub unsafe extern "C" fn gsl_complex_tanh(mut a: gsl_complex) -> gsl_complex {
    let mut R: libc::c_double = a.dat[0 as libc::c_int as usize];
    let mut I: libc::c_double = a.dat[1 as libc::c_int as usize];
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    if fabs(R) < 1.0f64 {
        let mut D: libc::c_double = pow(cos(I), 2.0f64) + pow(sinh(R), 2.0f64);
        z.dat[0 as libc::c_int as usize] = sinh(R) * cosh(R) / D;
        z
            .dat[1 as libc::c_int
            as usize] = 0.5f64 * sin(2 as libc::c_int as libc::c_double * I) / D;
    } else {
        let mut D_0: libc::c_double = pow(cos(I), 2.0f64) + pow(sinh(R), 2.0f64);
        let mut F: libc::c_double = 1 as libc::c_int as libc::c_double
            + pow(cos(I) / sinh(R), 2.0f64);
        z.dat[0 as libc::c_int as usize] = 1.0f64 / (tanh(R) * F);
        z
            .dat[1 as libc::c_int
            as usize] = 0.5f64 * sin(2 as libc::c_int as libc::c_double * I) / D_0;
    }
    return z;
}
pub unsafe extern "C" fn gsl_complex_sech(mut a: gsl_complex) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex_cosh(a);
    return gsl_complex_inverse(z);
}
pub unsafe extern "C" fn gsl_complex_csch(mut a: gsl_complex) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex_sinh(a);
    return gsl_complex_inverse(z);
}
pub unsafe extern "C" fn gsl_complex_coth(mut a: gsl_complex) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex_tanh(a);
    return gsl_complex_inverse(z);
}
pub unsafe extern "C" fn gsl_complex_arcsinh(mut a: gsl_complex) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex_mul_imag(a, 1.0f64);
    z = gsl_complex_arcsin(z);
    z = gsl_complex_mul_imag(z, -1.0f64);
    return z;
}
pub unsafe extern "C" fn gsl_complex_arccosh(mut a: gsl_complex) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex_arccos(a);
    z = gsl_complex_mul_imag(
        z,
        if z.dat[1 as libc::c_int as usize] > 0 as libc::c_int as libc::c_double {
            -1.0f64
        } else {
            1.0f64
        },
    );
    return z;
}
pub unsafe extern "C" fn gsl_complex_arccosh_real(mut a: libc::c_double) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    if a >= 1 as libc::c_int as libc::c_double {
        z.dat[0 as libc::c_int as usize] = acosh(a);
        z.dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    } else if a >= -1.0f64 {
        z.dat[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
        z.dat[1 as libc::c_int as usize] = acos(a);
    } else {
        z.dat[0 as libc::c_int as usize] = acosh(-a);
        z.dat[1 as libc::c_int as usize] = 3.14159265358979323846f64;
    }
    return z;
}
pub unsafe extern "C" fn gsl_complex_arctanh(mut a: gsl_complex) -> gsl_complex {
    if a.dat[1 as libc::c_int as usize] == 0.0f64 {
        return gsl_complex_arctanh_real(a.dat[0 as libc::c_int as usize])
    } else {
        let mut z: gsl_complex = gsl_complex_mul_imag(a, 1.0f64);
        z = gsl_complex_arctan(z);
        z = gsl_complex_mul_imag(z, -1.0f64);
        return z;
    };
}
pub unsafe extern "C" fn gsl_complex_arctanh_real(mut a: libc::c_double) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    if a > -1.0f64 && a < 1.0f64 {
        z.dat[0 as libc::c_int as usize] = atanh(a);
        z.dat[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    } else {
        z.dat[0 as libc::c_int as usize] = atanh(1 as libc::c_int as libc::c_double / a);
        z
            .dat[1 as libc::c_int
            as usize] = if a < 0 as libc::c_int as libc::c_double {
            1.57079632679489661923f64
        } else {
            -1.57079632679489661923f64
        };
    }
    return z;
}
pub unsafe extern "C" fn gsl_complex_arcsech(mut a: gsl_complex) -> gsl_complex {
    let mut t: gsl_complex = gsl_complex_inverse(a);
    return gsl_complex_arccosh(t);
}
pub unsafe extern "C" fn gsl_complex_arccsch(mut a: gsl_complex) -> gsl_complex {
    let mut t: gsl_complex = gsl_complex_inverse(a);
    return gsl_complex_arcsinh(t);
}
pub unsafe extern "C" fn gsl_complex_arccoth(mut a: gsl_complex) -> gsl_complex {
    let mut t: gsl_complex = gsl_complex_inverse(a);
    return gsl_complex_arctanh(t);
}
