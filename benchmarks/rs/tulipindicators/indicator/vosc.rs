use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_vosc_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(1 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_vosc(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let short_period: libc::c_int = *options.offset(0 as libc::c_int as isize)
        as libc::c_int;
    let long_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    let short_div: libc::c_double = 1.0f64 / short_period as libc::c_double;
    let long_div: libc::c_double = 1.0f64 / long_period as libc::c_double;
    if short_period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if long_period < short_period {
        return 1 as libc::c_int;
    }
    if size <= ti_vosc_start(options) {
        return 0 as libc::c_int;
    }
    let mut short_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut long_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < long_period {
        if i >= long_period - short_period {
            short_sum += *input.offset(i as isize);
        }
        long_sum += *input.offset(i as isize);
        i += 1;
        i;
    }
    let savg: libc::c_double = short_sum * short_div;
    let lavg: libc::c_double = long_sum * long_div;
    let fresh0 = output;
    output = output.offset(1);
    *fresh0 = 100.0f64 * (savg - lavg) / lavg;
    i = long_period;
    while i < size {
        short_sum += *input.offset(i as isize);
        short_sum -= *input.offset((i - short_period) as isize);
        long_sum += *input.offset(i as isize);
        long_sum -= *input.offset((i - long_period) as isize);
        let savg_0: libc::c_double = short_sum * short_div;
        let lavg_0: libc::c_double = long_sum * long_div;
        let fresh1 = output;
        output = output.offset(1);
        *fresh1 = 100.0f64 * (savg_0 - lavg_0) / lavg_0;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_vosc_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_vosc_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/vosc.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_vosc(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
