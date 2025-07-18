use ::libc;
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_fisher_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_fisher(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut fisher: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut signal: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_fisher_start(options) {
        return 0 as libc::c_int;
    }
    let mut trail: libc::c_int = 0 as libc::c_int;
    let mut maxi: libc::c_int = -(1 as libc::c_int);
    let mut mini: libc::c_int = -(1 as libc::c_int);
    let mut max: libc::c_double = 0.5f64
        * (*high.offset(0 as libc::c_int as isize)
            + *low.offset(0 as libc::c_int as isize));
    let mut min: libc::c_double = 0.5f64
        * (*high.offset(0 as libc::c_int as isize)
            + *low.offset(0 as libc::c_int as isize));
    let mut val1: libc::c_double = 0.0f64;
    let mut bar: libc::c_double = 0.;
    let mut fish: libc::c_double = 0.0f64;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = period - 1 as libc::c_int;
    while i < size {
        bar = 0.5f64 * (*high.offset(i as isize) + *low.offset(i as isize));
        if maxi < trail {
            maxi = trail;
            max = 0.5f64 * (*high.offset(maxi as isize) + *low.offset(maxi as isize));
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = 0.5f64 * (*high.offset(j as isize) + *low.offset(j as isize));
                if bar >= max {
                    max = bar;
                    maxi = j;
                }
            }
        } else if bar >= max {
            maxi = i;
            max = bar;
        }
        bar = 0.5f64 * (*high.offset(i as isize) + *low.offset(i as isize));
        if mini < trail {
            mini = trail;
            min = 0.5f64 * (*high.offset(mini as isize) + *low.offset(mini as isize));
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = 0.5f64 * (*high.offset(j as isize) + *low.offset(j as isize));
                if bar <= min {
                    min = bar;
                    mini = j;
                }
            }
        } else if bar <= min {
            mini = i;
            min = bar;
        }
        let mut mm: libc::c_double = max - min;
        if mm == 0.0f64 {
            mm = 0.001f64;
        }
        val1 = 0.33f64 * 2.0f64
            * ((0.5f64 * (*high.offset(i as isize) + *low.offset(i as isize)) - min) / mm
                - 0.5f64) + 0.67f64 * val1;
        if val1 > 0.99f64 {
            val1 = 0.999f64;
        }
        if val1 < -0.99f64 {
            val1 = -0.999f64;
        }
        let fresh0 = signal;
        signal = signal.offset(1);
        *fresh0 = fish;
        fish = 0.5f64 * log((1.0f64 + val1) / (1.0f64 - val1)) + 0.5f64 * fish;
        let fresh1 = fisher;
        fisher = fisher.offset(1);
        *fresh1 = fish;
        i += 1;
        i;
        trail += 1;
        trail;
    }
    if fisher.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_fisher_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"fisher - outputs[0] == size - ti_fisher_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/fisher.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"int ti_fisher(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    if signal.offset_from(*outputs.offset(1 as libc::c_int as isize)) as libc::c_long
        == (size - ti_fisher_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"signal - outputs[1] == size - ti_fisher_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/fisher.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"int ti_fisher(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
