use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_wma_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_wma(
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
    if size <= ti_wma_start(options) {
        return 0 as libc::c_int;
    }
    let weights: libc::c_double = (period * (period + 1 as libc::c_int)
        / 2 as libc::c_int) as libc::c_double;
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut weight_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period - 1 as libc::c_int {
        weight_sum
            += *input.offset(i as isize) * (i + 1 as libc::c_int) as libc::c_double;
        sum += *input.offset(i as isize);
        i += 1;
        i;
    }
    i = period - 1 as libc::c_int;
    while i < size {
        weight_sum += *input.offset(i as isize) * period as libc::c_double;
        sum += *input.offset(i as isize);
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = weight_sum / weights;
        weight_sum -= sum;
        sum -= *input.offset((i - period + 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_wma_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_wma_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/wma.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_wma(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
