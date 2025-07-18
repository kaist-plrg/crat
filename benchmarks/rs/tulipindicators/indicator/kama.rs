use ::libc;
extern "C" {
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_kama_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_kama(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_kama_start(options) {
        return 0 as libc::c_int;
    }
    let short_per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (2.0f64 + 1 as libc::c_int as libc::c_double);
    let long_per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (30.0f64 + 1 as libc::c_int as libc::c_double);
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < period {
        sum
            += fabs(
                *input.offset(i as isize)
                    - *input.offset((i - 1 as libc::c_int) as isize),
            );
        i += 1;
        i;
    }
    let mut kama: libc::c_double = *input.offset((period - 1 as libc::c_int) as isize);
    let fresh0 = output;
    output = output.offset(1);
    *fresh0 = kama;
    let mut er: libc::c_double = 0.;
    let mut sc: libc::c_double = 0.;
    i = period;
    while i < size {
        sum
            += fabs(
                *input.offset(i as isize)
                    - *input.offset((i - 1 as libc::c_int) as isize),
            );
        if i > period {
            sum
                -= fabs(
                    *input.offset((i - period) as isize)
                        - *input.offset((i - period - 1 as libc::c_int) as isize),
                );
        }
        if sum != 0.0f64 {
            er = fabs(*input.offset(i as isize) - *input.offset((i - period) as isize))
                / sum;
        } else {
            er = 1.0f64;
        }
        sc = pow(
            er * (short_per - long_per) + long_per,
            2 as libc::c_int as libc::c_double,
        );
        kama = kama + sc * (*input.offset(i as isize) - kama);
        let fresh1 = output;
        output = output.offset(1);
        *fresh1 = kama;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_kama_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_kama_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/kama.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_kama(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
