use ::libc;
pub unsafe extern "C" fn gsl_pow_2(x: libc::c_double) -> libc::c_double {
    return x * x;
}
pub unsafe extern "C" fn gsl_pow_3(x: libc::c_double) -> libc::c_double {
    return x * x * x;
}
pub unsafe extern "C" fn gsl_pow_4(x: libc::c_double) -> libc::c_double {
    let mut x2: libc::c_double = x * x;
    return x2 * x2;
}
pub unsafe extern "C" fn gsl_pow_5(x: libc::c_double) -> libc::c_double {
    let mut x2: libc::c_double = x * x;
    return x2 * x2 * x;
}
pub unsafe extern "C" fn gsl_pow_6(x: libc::c_double) -> libc::c_double {
    let mut x2: libc::c_double = x * x;
    return x2 * x2 * x2;
}
pub unsafe extern "C" fn gsl_pow_7(x: libc::c_double) -> libc::c_double {
    let mut x3: libc::c_double = x * x * x;
    return x3 * x3 * x;
}
pub unsafe extern "C" fn gsl_pow_8(x: libc::c_double) -> libc::c_double {
    let mut x2: libc::c_double = x * x;
    let mut x4: libc::c_double = x2 * x2;
    return x4 * x4;
}
pub unsafe extern "C" fn gsl_pow_9(x: libc::c_double) -> libc::c_double {
    let mut x3: libc::c_double = x * x * x;
    return x3 * x3 * x3;
}
pub unsafe extern "C" fn gsl_pow_int(
    mut x: libc::c_double,
    mut n: libc::c_int,
) -> libc::c_double {
    let mut un: libc::c_uint = 0;
    if n < 0 as libc::c_int {
        x = 1.0f64 / x;
        un = -n as libc::c_uint;
    } else {
        un = n as libc::c_uint;
    }
    return gsl_pow_uint(x, un);
}
pub unsafe extern "C" fn gsl_pow_uint(
    mut x: libc::c_double,
    mut n: libc::c_uint,
) -> libc::c_double {
    let mut value: libc::c_double = 1.0f64;
    loop {
        if n & 1 as libc::c_int as libc::c_uint != 0 {
            value *= x;
        }
        n >>= 1 as libc::c_int;
        x *= x;
        if !(n != 0) {
            break;
        }
    }
    return value;
}
