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
pub unsafe extern "C" fn ti_stderr_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_stderr(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let scale: libc::c_double = 1.0f64 / period as libc::c_double;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_stderr_start(options) {
        return 0 as libc::c_int;
    }
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sum2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mul: libc::c_double = 1.0f64 / sqrt(period as libc::c_double);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        sum += *input.offset(i as isize);
        sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        i += 1;
        i;
    }
    let mut s2s2: libc::c_double = sum2 * scale - sum * scale * (sum * scale);
    if s2s2 > 0.0f64 {
        s2s2 = sqrt(s2s2);
    }
    let fresh0 = output;
    output = output.offset(1);
    *fresh0 = mul * s2s2;
    i = period;
    while i < size {
        sum += *input.offset(i as isize);
        sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        sum -= *input.offset((i - period) as isize);
        sum2
            -= *input.offset((i - period) as isize)
                * *input.offset((i - period) as isize);
        let mut s2s2_0: libc::c_double = sum2 * scale - sum * scale * (sum * scale);
        if s2s2_0 > 0.0f64 {
            s2s2_0 = sqrt(s2s2_0);
        }
        let fresh1 = output;
        output = output.offset(1);
        *fresh1 = mul * s2s2_0;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_stderr_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_stderr_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/stderr.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"int ti_stderr(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
