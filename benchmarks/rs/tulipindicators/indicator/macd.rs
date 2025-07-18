use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_macd_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let long_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    return long_period - 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_macd(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut macd: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut signal: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    let mut hist: *mut libc::c_double = *outputs.offset(2 as libc::c_int as isize);
    let short_period: libc::c_int = *options.offset(0 as libc::c_int as isize)
        as libc::c_int;
    let long_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    let signal_period: libc::c_int = *options.offset(2 as libc::c_int as isize)
        as libc::c_int;
    if short_period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if long_period < 2 as libc::c_int {
        return 1 as libc::c_int;
    }
    if long_period < short_period {
        return 1 as libc::c_int;
    }
    if signal_period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_macd_start(options) {
        return 0 as libc::c_int;
    }
    let mut short_per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (short_period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let mut long_per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (long_period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let mut signal_per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (signal_period as libc::c_double + 1 as libc::c_int as libc::c_double);
    if short_period == 12 as libc::c_int && long_period == 26 as libc::c_int {
        short_per = 0.15f64;
        long_per = 0.075f64;
    }
    let mut short_ema: libc::c_double = *input.offset(0 as libc::c_int as isize);
    let mut long_ema: libc::c_double = *input.offset(0 as libc::c_int as isize);
    let mut signal_ema: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        short_ema = (*input.offset(i as isize) - short_ema) * short_per + short_ema;
        long_ema = (*input.offset(i as isize) - long_ema) * long_per + long_ema;
        let out: libc::c_double = short_ema - long_ema;
        if i == long_period - 1 as libc::c_int {
            signal_ema = out;
        }
        if i >= long_period - 1 as libc::c_int {
            signal_ema = (out - signal_ema) * signal_per + signal_ema;
            let fresh0 = macd;
            macd = macd.offset(1);
            *fresh0 = out;
            let fresh1 = signal;
            signal = signal.offset(1);
            *fresh1 = signal_ema;
            let fresh2 = hist;
            hist = hist.offset(1);
            *fresh2 = out - signal_ema;
        }
        i += 1;
        i;
    }
    if macd.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_macd_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"macd - outputs[0] == size - ti_macd_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/macd.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_macd(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    if signal.offset_from(*outputs.offset(1 as libc::c_int as isize)) as libc::c_long
        == (size - ti_macd_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"signal - outputs[1] == size - ti_macd_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/macd.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_macd(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    if hist.offset_from(*outputs.offset(2 as libc::c_int as isize)) as libc::c_long
        == (size - ti_macd_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"hist - outputs[2] == size - ti_macd_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/macd.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_macd(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
