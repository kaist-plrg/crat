use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_adosc_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(1 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_adosc(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut volume: *const libc::c_double = *inputs.offset(3 as libc::c_int as isize);
    let short_period: libc::c_int = *options.offset(0 as libc::c_int as isize)
        as libc::c_int;
    let long_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    let start: libc::c_int = long_period - 1 as libc::c_int;
    if short_period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if long_period < short_period {
        return 1 as libc::c_int;
    }
    if size <= ti_adosc_start(options) {
        return 0 as libc::c_int;
    }
    let short_per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (short_period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let long_per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (long_period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut short_ema: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut long_ema: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        let hl: libc::c_double = *high.offset(i as isize) - *low.offset(i as isize);
        if hl != 0.0f64 {
            sum
                += (*close.offset(i as isize) - *low.offset(i as isize)
                    - *high.offset(i as isize) + *close.offset(i as isize)) / hl
                    * *volume.offset(i as isize);
        }
        if i == 0 as libc::c_int {
            short_ema = sum;
            long_ema = sum;
        } else {
            short_ema = (sum - short_ema) * short_per + short_ema;
            long_ema = (sum - long_ema) * long_per + long_ema;
        }
        if i >= start {
            let fresh0 = output;
            output = output.offset(1);
            *fresh0 = short_ema - long_ema;
        }
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_adosc_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_adosc_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/adosc.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"int ti_adosc(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
