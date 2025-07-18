use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_rsi_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int;
}
pub unsafe extern "C" fn ti_rsi(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let per: libc::c_double = 1.0f64 / period as libc::c_double;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_rsi_start(options) {
        return 0 as libc::c_int;
    }
    let mut smooth_up: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut smooth_down: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= period {
        let upward: libc::c_double = if *input.offset(i as isize)
            > *input.offset((i - 1 as libc::c_int) as isize)
        {
            *input.offset(i as isize) - *input.offset((i - 1 as libc::c_int) as isize)
        } else {
            0 as libc::c_int as libc::c_double
        };
        let downward: libc::c_double = if *input.offset(i as isize)
            < *input.offset((i - 1 as libc::c_int) as isize)
        {
            *input.offset((i - 1 as libc::c_int) as isize) - *input.offset(i as isize)
        } else {
            0 as libc::c_int as libc::c_double
        };
        smooth_up += upward;
        smooth_down += downward;
        i += 1;
        i;
    }
    smooth_up /= period as libc::c_double;
    smooth_down /= period as libc::c_double;
    let fresh0 = output;
    output = output.offset(1);
    *fresh0 = 100.0f64 * (smooth_up / (smooth_up + smooth_down));
    i = period + 1 as libc::c_int;
    while i < size {
        let upward_0: libc::c_double = if *input.offset(i as isize)
            > *input.offset((i - 1 as libc::c_int) as isize)
        {
            *input.offset(i as isize) - *input.offset((i - 1 as libc::c_int) as isize)
        } else {
            0 as libc::c_int as libc::c_double
        };
        let downward_0: libc::c_double = if *input.offset(i as isize)
            < *input.offset((i - 1 as libc::c_int) as isize)
        {
            *input.offset((i - 1 as libc::c_int) as isize) - *input.offset(i as isize)
        } else {
            0 as libc::c_int as libc::c_double
        };
        smooth_up = (upward_0 - smooth_up) * per + smooth_up;
        smooth_down = (downward_0 - smooth_down) * per + smooth_down;
        let fresh1 = output;
        output = output.offset(1);
        *fresh1 = 100.0f64 * (smooth_up / (smooth_up + smooth_down));
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_rsi_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_rsi_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/rsi.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_rsi(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
