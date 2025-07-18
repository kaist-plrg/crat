use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn ti_sma(
        size: libc::c_int,
        inputs: *const *const libc::c_double,
        options: *const libc::c_double,
        outputs: *const *mut libc::c_double,
    ) -> libc::c_int;
}
pub unsafe extern "C" fn ti_trima_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_trima(
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
    if size <= ti_trima_start(options) {
        return 0 as libc::c_int;
    }
    if period <= 2 as libc::c_int {
        return ti_sma(size, inputs, options, outputs);
    }
    let mut weights: libc::c_double = 1 as libc::c_int as libc::c_double
        / (if period % 2 as libc::c_int != 0 {
            (period / 2 as libc::c_int + 1 as libc::c_int)
                * (period / 2 as libc::c_int + 1 as libc::c_int)
        } else {
            (period / 2 as libc::c_int + 1 as libc::c_int) * (period / 2 as libc::c_int)
        }) as libc::c_double;
    let mut weight_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut lead_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut trail_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let lead_period: libc::c_int = if period % 2 as libc::c_int != 0 {
        period / 2 as libc::c_int
    } else {
        period / 2 as libc::c_int - 1 as libc::c_int
    };
    let trail_period: libc::c_int = lead_period + 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut w: libc::c_int = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < period - 1 as libc::c_int {
        weight_sum += *input.offset(i as isize) * w as libc::c_double;
        if i + 1 as libc::c_int > period - lead_period {
            lead_sum += *input.offset(i as isize);
        }
        if i + 1 as libc::c_int <= trail_period {
            trail_sum += *input.offset(i as isize);
        }
        if (i + 1 as libc::c_int) < trail_period {
            w += 1;
            w;
        }
        if i + 1 as libc::c_int >= period - lead_period {
            w -= 1;
            w;
        }
        i += 1;
        i;
    }
    let mut lsi: libc::c_int = period - 1 as libc::c_int - lead_period
        + 1 as libc::c_int;
    let mut tsi1: libc::c_int = period - 1 as libc::c_int - period + 1 as libc::c_int
        + trail_period;
    let mut tsi2: libc::c_int = period - 1 as libc::c_int - period + 1 as libc::c_int;
    i = period - 1 as libc::c_int;
    while i < size {
        weight_sum += *input.offset(i as isize);
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = weight_sum * weights;
        lead_sum += *input.offset(i as isize);
        weight_sum += lead_sum;
        weight_sum -= trail_sum;
        let fresh1 = lsi;
        lsi = lsi + 1;
        lead_sum -= *input.offset(fresh1 as isize);
        let fresh2 = tsi1;
        tsi1 = tsi1 + 1;
        trail_sum += *input.offset(fresh2 as isize);
        let fresh3 = tsi2;
        tsi2 = tsi2 + 1;
        trail_sum -= *input.offset(fresh3 as isize);
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_trima_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_trima_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/trima.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"int ti_trima(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
