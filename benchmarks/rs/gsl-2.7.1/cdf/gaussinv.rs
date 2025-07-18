use ::libc;
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
unsafe extern "C" fn rat_eval(
    mut a: *const libc::c_double,
    na: size_t,
    mut b: *const libc::c_double,
    nb: size_t,
    x: libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut u: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    u = *a.offset(na.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    i = na.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while i > 0 as libc::c_int as libc::c_ulong {
        u = x * u
            + *a.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        i = i.wrapping_sub(1);
        i;
    }
    v = *b.offset(nb.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    j = nb.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while j > 0 as libc::c_int as libc::c_ulong {
        v = x * v
            + *b.offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        j = j.wrapping_sub(1);
        j;
    }
    r = u / v;
    return r;
}
unsafe extern "C" fn small(mut q: libc::c_double) -> libc::c_double {
    let a: [libc::c_double; 8] = [
        3.387132872796366608f64,
        133.14166789178437745f64,
        1971.5909503065514427f64,
        13731.693765509461125f64,
        45921.953931549871457f64,
        67265.770927008700853f64,
        33430.575583588128105f64,
        2509.0809287301226727f64,
    ];
    let b: [libc::c_double; 8] = [
        1.0f64,
        42.313330701600911252f64,
        687.1870074920579083f64,
        5394.1960214247511077f64,
        21213.794301586595867f64,
        39307.89580009271061f64,
        28729.085735721942674f64,
        5226.495278852854561f64,
    ];
    let mut r: libc::c_double = 0.180625f64 - q * q;
    let mut x: libc::c_double = q
        * rat_eval(
            a.as_ptr(),
            8 as libc::c_int as size_t,
            b.as_ptr(),
            8 as libc::c_int as size_t,
            r,
        );
    return x;
}
unsafe extern "C" fn intermediate(mut r: libc::c_double) -> libc::c_double {
    let a: [libc::c_double; 8] = [
        1.42343711074968357734f64,
        4.6303378461565452959f64,
        5.7694972214606914055f64,
        3.64784832476320460504f64,
        1.27045825245236838258f64,
        0.24178072517745061177f64,
        0.0227238449892691845833f64,
        7.7454501427834140764e-4f64,
    ];
    let b: [libc::c_double; 8] = [
        1.0f64,
        2.05319162663775882187f64,
        1.6763848301838038494f64,
        0.68976733498510000455f64,
        0.14810397642748007459f64,
        0.0151986665636164571966f64,
        5.475938084995344946e-4f64,
        1.05075007164441684324e-9f64,
    ];
    let mut x: libc::c_double = rat_eval(
        a.as_ptr(),
        8 as libc::c_int as size_t,
        b.as_ptr(),
        8 as libc::c_int as size_t,
        r - 1.6f64,
    );
    return x;
}
unsafe extern "C" fn tail(mut r: libc::c_double) -> libc::c_double {
    let a: [libc::c_double; 8] = [
        6.6579046435011037772f64,
        5.4637849111641143699f64,
        1.7848265399172913358f64,
        0.29656057182850489123f64,
        0.026532189526576123093f64,
        0.0012426609473880784386f64,
        2.71155556874348757815e-5f64,
        2.01033439929228813265e-7f64,
    ];
    let b: [libc::c_double; 8] = [
        1.0f64,
        0.59983220655588793769f64,
        0.13692988092273580531f64,
        0.0148753612908506148525f64,
        7.868691311456132591e-4f64,
        1.8463183175100546818e-5f64,
        1.4215117583164458887e-7f64,
        2.04426310338993978564e-15f64,
    ];
    let mut x: libc::c_double = rat_eval(
        a.as_ptr(),
        8 as libc::c_int as size_t,
        b.as_ptr(),
        8 as libc::c_int as size_t,
        r - 5.0f64,
    );
    return x;
}
pub unsafe extern "C" fn gsl_cdf_ugaussian_Pinv(P: libc::c_double) -> libc::c_double {
    let mut r: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut pp: libc::c_double = 0.;
    let mut dP: libc::c_double = P - 0.5f64;
    if P == 1.0f64 {
        return ::std::f32::INFINITY as libc::c_double
    } else if P == 0.0f64 {
        return -::std::f32::INFINITY as libc::c_double
    }
    if fabs(dP) <= 0.425f64 {
        x = small(dP);
        return x;
    }
    pp = if P < 0.5f64 { P } else { 1.0f64 - P };
    r = sqrt(-log(pp));
    if r <= 5.0f64 {
        x = intermediate(r);
    } else {
        x = tail(r);
    }
    if P < 0.5f64 { return -x } else { return x };
}
pub unsafe extern "C" fn gsl_cdf_ugaussian_Qinv(Q: libc::c_double) -> libc::c_double {
    let mut r: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut pp: libc::c_double = 0.;
    let mut dQ: libc::c_double = Q - 0.5f64;
    if Q == 1.0f64 {
        return -::std::f32::INFINITY as libc::c_double
    } else if Q == 0.0f64 {
        return ::std::f32::INFINITY as libc::c_double
    }
    if fabs(dQ) <= 0.425f64 {
        x = small(dQ);
        return -x;
    }
    pp = if Q < 0.5f64 { Q } else { 1.0f64 - Q };
    r = sqrt(-log(pp));
    if r <= 5.0f64 {
        x = intermediate(r);
    } else {
        x = tail(r);
    }
    if Q < 0.5f64 { return x } else { return -x };
}
pub unsafe extern "C" fn gsl_cdf_gaussian_Pinv(
    P: libc::c_double,
    sigma: libc::c_double,
) -> libc::c_double {
    return sigma * gsl_cdf_ugaussian_Pinv(P);
}
pub unsafe extern "C" fn gsl_cdf_gaussian_Qinv(
    Q: libc::c_double,
    sigma: libc::c_double,
) -> libc::c_double {
    return sigma * gsl_cdf_ugaussian_Qinv(Q);
}
