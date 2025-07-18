use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_volatility_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int;
}
pub unsafe extern "C" fn ti_volatility(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let scale: libc::c_double = 1.0f64 / period as libc::c_double;
    let annual: libc::c_double = sqrt(252 as libc::c_int as libc::c_double);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_volatility_start(options) {
        return 0 as libc::c_int;
    }
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sum2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= period {
        let c: libc::c_double = *input.offset(i as isize)
            / *input.offset((i - 1 as libc::c_int) as isize) - 1.0f64;
        sum += c;
        sum2 += c * c;
        i += 1;
        i;
    }
    let fresh0 = output;
    output = output.offset(1);
    *fresh0 = sqrt(sum2 * scale - sum * scale * (sum * scale)) * annual;
    i = period + 1 as libc::c_int;
    while i < size {
        let c_0: libc::c_double = *input.offset(i as isize)
            / *input.offset((i - 1 as libc::c_int) as isize) - 1.0f64;
        sum += c_0;
        sum2 += c_0 * c_0;
        let cp: libc::c_double = *input.offset((i - period) as isize)
            / *input.offset((i - period - 1 as libc::c_int) as isize) - 1.0f64;
        sum -= cp;
        sum2 -= cp * cp;
        let fresh1 = output;
        output = output.offset(1);
        *fresh1 = sqrt(sum2 * scale - sum * scale * (sum * scale)) * annual;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_volatility_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_volatility_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/volatility.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"int ti_volatility(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
