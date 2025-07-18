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
pub unsafe extern "C" fn ti_vidya_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(1 as libc::c_int as isize) as libc::c_int - 2 as libc::c_int;
}
pub unsafe extern "C" fn ti_vidya(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let short_period: libc::c_int = *options.offset(0 as libc::c_int as isize)
        as libc::c_int;
    let long_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    let alpha: libc::c_double = *options.offset(2 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let short_div: libc::c_double = 1.0f64 / short_period as libc::c_double;
    let long_div: libc::c_double = 1.0f64 / long_period as libc::c_double;
    if short_period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if long_period < short_period {
        return 1 as libc::c_int;
    }
    if long_period < 2 as libc::c_int {
        return 1 as libc::c_int;
    }
    if alpha < 0.0f64 || alpha > 1.0f64 {
        return 1 as libc::c_int;
    }
    if size <= ti_vidya_start(options) {
        return 0 as libc::c_int;
    }
    let mut short_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut short_sum2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut long_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut long_sum2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < long_period {
        long_sum += *input.offset(i as isize);
        long_sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        if i >= long_period - short_period {
            short_sum += *input.offset(i as isize);
            short_sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        }
        i += 1;
        i;
    }
    let mut val: libc::c_double = *input
        .offset((long_period - 2 as libc::c_int) as isize);
    let fresh0 = output;
    output = output.offset(1);
    *fresh0 = val;
    if (long_period - 1 as libc::c_int) < size {
        let mut short_stddev: libc::c_double = sqrt(
            short_sum2 * short_div - short_sum * short_div * (short_sum * short_div),
        );
        let mut long_stddev: libc::c_double = sqrt(
            long_sum2 * long_div - long_sum * long_div * (long_sum * long_div),
        );
        let mut k: libc::c_double = short_stddev / long_stddev;
        if k != k {
            k = 0 as libc::c_int as libc::c_double;
        }
        k *= alpha;
        val = (*input.offset((long_period - 1 as libc::c_int) as isize) - val) * k + val;
        let fresh1 = output;
        output = output.offset(1);
        *fresh1 = val;
    }
    i = long_period;
    while i < size {
        long_sum += *input.offset(i as isize);
        long_sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        short_sum += *input.offset(i as isize);
        short_sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        long_sum -= *input.offset((i - long_period) as isize);
        long_sum2
            -= *input.offset((i - long_period) as isize)
                * *input.offset((i - long_period) as isize);
        short_sum -= *input.offset((i - short_period) as isize);
        short_sum2
            -= *input.offset((i - short_period) as isize)
                * *input.offset((i - short_period) as isize);
        let mut short_stddev_0: libc::c_double = sqrt(
            short_sum2 * short_div - short_sum * short_div * (short_sum * short_div),
        );
        let mut long_stddev_0: libc::c_double = sqrt(
            long_sum2 * long_div - long_sum * long_div * (long_sum * long_div),
        );
        let mut k_0: libc::c_double = short_stddev_0 / long_stddev_0;
        if k_0 != k_0 {
            k_0 = 0 as libc::c_int as libc::c_double;
        }
        k_0 *= alpha;
        val = (*input.offset(i as isize) - val) * k_0 + val;
        let fresh2 = output;
        output = output.offset(1);
        *fresh2 = val;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_vidya_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_vidya_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/vidya.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"int ti_vidya(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
