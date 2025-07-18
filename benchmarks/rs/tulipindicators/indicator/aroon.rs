use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_aroon_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int;
}
pub unsafe extern "C" fn ti_aroon(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut adown: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut aup: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_aroon_start(options) {
        return 0 as libc::c_int;
    }
    let scale: libc::c_double = 100.0f64 / period as libc::c_double;
    let mut trail: libc::c_int = 0 as libc::c_int;
    let mut maxi: libc::c_int = -(1 as libc::c_int);
    let mut mini: libc::c_int = -(1 as libc::c_int);
    let mut max: libc::c_double = *high.offset(0 as libc::c_int as isize);
    let mut min: libc::c_double = *low.offset(0 as libc::c_int as isize);
    let mut bar: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = period;
    while i < size {
        bar = *high.offset(i as isize);
        if maxi < trail {
            maxi = trail;
            max = *high.offset(maxi as isize);
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = *high.offset(j as isize);
                if bar >= max {
                    max = bar;
                    maxi = j;
                }
            }
        } else if bar >= max {
            maxi = i;
            max = bar;
        }
        bar = *low.offset(i as isize);
        if mini < trail {
            mini = trail;
            min = *low.offset(mini as isize);
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = *low.offset(j as isize);
                if bar <= min {
                    min = bar;
                    mini = j;
                }
            }
        } else if bar <= min {
            mini = i;
            min = bar;
        }
        let fresh0 = adown;
        adown = adown.offset(1);
        *fresh0 = (period as libc::c_double - (i - mini) as libc::c_double) * scale;
        let fresh1 = aup;
        aup = aup.offset(1);
        *fresh1 = (period as libc::c_double - (i - maxi) as libc::c_double) * scale;
        i += 1;
        i;
        trail += 1;
        trail;
    }
    if adown.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_aroon_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"adown - outputs[0] == size - ti_aroon_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/aroon.c\0" as *const u8 as *const libc::c_char,
            96 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"int ti_aroon(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    if aup.offset_from(*outputs.offset(1 as libc::c_int as isize)) as libc::c_long
        == (size - ti_aroon_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"aup - outputs[1] == size - ti_aroon_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/aroon.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"int ti_aroon(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
