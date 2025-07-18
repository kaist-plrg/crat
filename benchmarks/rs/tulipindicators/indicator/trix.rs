use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_trix_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    return (period - 1 as libc::c_int) * 3 as libc::c_int + 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_trix(
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
    if size <= ti_trix_start(options) {
        return 0 as libc::c_int;
    }
    let start: libc::c_int = period * 3 as libc::c_int - 2 as libc::c_int;
    if start == ti_trix_start(options) {} else {
        __assert_fail(
            b"start == ti_trix_start(options)\0" as *const u8 as *const libc::c_char,
            b"indicator/trix.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_trix(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    let per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let mut ema1: libc::c_double = *input.offset(0 as libc::c_int as isize);
    let mut ema2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut ema3: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < start {
        ema1 = (*input.offset(i as isize) - ema1) * per + ema1;
        if i == period - 1 as libc::c_int {
            ema2 = ema1;
        } else if i > period - 1 as libc::c_int {
            ema2 = (ema1 - ema2) * per + ema2;
            if i == period * 2 as libc::c_int - 2 as libc::c_int {
                ema3 = ema2;
            } else if i > period * 2 as libc::c_int - 2 as libc::c_int {
                ema3 = (ema2 - ema3) * per + ema3;
            }
        }
        i += 1;
        i;
    }
    i = start;
    while i < size {
        ema1 = (*input.offset(i as isize) - ema1) * per + ema1;
        ema2 = (ema1 - ema2) * per + ema2;
        let last: libc::c_double = ema3;
        ema3 = (ema2 - ema3) * per + ema3;
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = (ema3 - last) / ema3 * 100.0f64;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_trix_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_trix_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/trix.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_trix(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
