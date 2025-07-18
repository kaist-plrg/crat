use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
}
unsafe extern "C" fn get_del(
    mut x: libc::c_double,
    mut rational: libc::c_double,
) -> libc::c_double {
    let mut xsq: libc::c_double = 0.0f64;
    let mut del: libc::c_double = 0.0f64;
    let mut result: libc::c_double = 0.0f64;
    xsq = floor(x * 16.0f64) / 16.0f64;
    del = (x - xsq) * (x + xsq);
    del *= 0.5f64;
    result = exp(-0.5f64 * xsq * xsq) * exp(-1.0f64 * del) * rational;
    return result;
}
unsafe extern "C" fn gauss_small(x: libc::c_double) -> libc::c_double {
    let mut i: libc::c_uint = 0;
    let mut result: libc::c_double = 0.0f64;
    let mut xsq: libc::c_double = 0.;
    let mut xnum: libc::c_double = 0.;
    let mut xden: libc::c_double = 0.;
    let a: [libc::c_double; 5] = [
        2.2352520354606839287f64,
        161.02823106855587881f64,
        1067.6894854603709582f64,
        18154.981253343561249f64,
        0.065682337918207449113f64,
    ];
    let b: [libc::c_double; 4] = [
        47.20258190468824187f64,
        976.09855173777669322f64,
        10260.932208618978205f64,
        45507.789335026729956f64,
    ];
    xsq = x * x;
    xnum = a[4 as libc::c_int as usize] * xsq;
    xden = xsq;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 3 as libc::c_int as libc::c_uint {
        xnum = (xnum + a[i as usize]) * xsq;
        xden = (xden + b[i as usize]) * xsq;
        i = i.wrapping_add(1);
        i;
    }
    result = x * (xnum + a[3 as libc::c_int as usize])
        / (xden + b[3 as libc::c_int as usize]);
    return result;
}
unsafe extern "C" fn gauss_medium(x: libc::c_double) -> libc::c_double {
    let mut i: libc::c_uint = 0;
    let mut temp: libc::c_double = 0.0f64;
    let mut result: libc::c_double = 0.0f64;
    let mut xnum: libc::c_double = 0.;
    let mut xden: libc::c_double = 0.;
    let mut absx: libc::c_double = 0.;
    let c: [libc::c_double; 9] = [
        0.39894151208813466764f64,
        8.8831497943883759412f64,
        93.506656132177855979f64,
        597.27027639480026226f64,
        2494.5375852903726711f64,
        6848.1904505362823326f64,
        11602.651437647350124f64,
        9842.7148383839780218f64,
        1.0765576773720192317e-8f64,
    ];
    let d: [libc::c_double; 8] = [
        22.266688044328115691f64,
        235.38790178262499861f64,
        1519.377599407554805f64,
        6485.558298266760755f64,
        18615.571640885098091f64,
        34900.952721145977266f64,
        38912.003286093271411f64,
        19685.429676859990727f64,
    ];
    absx = fabs(x);
    xnum = c[8 as libc::c_int as usize] * absx;
    xden = absx;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 7 as libc::c_int as libc::c_uint {
        xnum = (xnum + c[i as usize]) * absx;
        xden = (xden + d[i as usize]) * absx;
        i = i.wrapping_add(1);
        i;
    }
    temp = (xnum + c[7 as libc::c_int as usize]) / (xden + d[7 as libc::c_int as usize]);
    result = get_del(x, temp);
    return result;
}
unsafe extern "C" fn gauss_large(x: libc::c_double) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut result: libc::c_double = 0.;
    let mut xsq: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut xnum: libc::c_double = 0.;
    let mut xden: libc::c_double = 0.;
    let mut absx: libc::c_double = 0.;
    let p: [libc::c_double; 6] = [
        0.21589853405795699f64,
        0.1274011611602473639f64,
        0.022235277870649807f64,
        0.001421619193227893466f64,
        2.9112874951168792e-5f64,
        0.02307344176494017303f64,
    ];
    let q: [libc::c_double; 5] = [
        1.28426009614491121f64,
        0.468238212480865118f64,
        0.0659881378689285515f64,
        0.00378239633202758244f64,
        7.29751555083966205e-5f64,
    ];
    absx = fabs(x);
    xsq = 1.0f64 / (x * x);
    xnum = p[5 as libc::c_int as usize] * xsq;
    xden = xsq;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        xnum = (xnum + p[i as usize]) * xsq;
        xden = (xden + q[i as usize]) * xsq;
        i += 1;
        i;
    }
    temp = xsq * (xnum + p[4 as libc::c_int as usize])
        / (xden + q[4 as libc::c_int as usize]);
    temp = (1.12837916709551257390f64 * 0.70710678118654752440f64 / 2.0f64 - temp)
        / absx;
    result = get_del(x, temp);
    return result;
}
pub unsafe extern "C" fn gsl_cdf_ugaussian_P(x: libc::c_double) -> libc::c_double {
    let mut result: libc::c_double = 0.;
    let mut absx: libc::c_double = fabs(x);
    if absx < 2.2204460492503131e-16f64 / 2 as libc::c_int as libc::c_double {
        result = 0.5f64;
        return result;
    } else if absx < 0.66291f64 {
        result = 0.5f64 + gauss_small(x);
        return result;
    } else if absx < 4.0f64 * 1.41421356237309504880f64 {
        result = gauss_medium(x);
        if x > 0.0f64 {
            result = 1.0f64 - result;
        }
        return result;
    } else if x > 8.572f64 {
        result = 1.0f64;
        return result;
    } else if x < -37.519f64 {
        result = 0.0f64;
        return result;
    } else {
        result = gauss_large(x);
        if x > 0.0f64 {
            result = 1.0f64 - result;
        }
    }
    return result;
}
pub unsafe extern "C" fn gsl_cdf_ugaussian_Q(x: libc::c_double) -> libc::c_double {
    let mut result: libc::c_double = 0.;
    let mut absx: libc::c_double = fabs(x);
    if absx < 2.2204460492503131e-16f64 / 2 as libc::c_int as libc::c_double {
        result = 0.5f64;
        return result;
    } else if absx < 0.66291f64 {
        result = gauss_small(x);
        if x < 0.0f64 {
            result = fabs(result) + 0.5f64;
        } else {
            result = 0.5f64 - result;
        }
        return result;
    } else if absx < 4.0f64 * 1.41421356237309504880f64 {
        result = gauss_medium(x);
        if x < 0.0f64 {
            result = 1.0f64 - result;
        }
        return result;
    } else if x > --37.519f64 {
        result = 0.0f64;
        return result;
    } else if x < -8.572f64 {
        result = 1.0f64;
        return result;
    } else {
        result = gauss_large(x);
        if x < 0.0f64 {
            result = 1.0f64 - result;
        }
    }
    return result;
}
pub unsafe extern "C" fn gsl_cdf_gaussian_P(
    x: libc::c_double,
    sigma: libc::c_double,
) -> libc::c_double {
    return gsl_cdf_ugaussian_P(x / sigma);
}
pub unsafe extern "C" fn gsl_cdf_gaussian_Q(
    x: libc::c_double,
    sigma: libc::c_double,
) -> libc::c_double {
    return gsl_cdf_ugaussian_Q(x / sigma);
}
