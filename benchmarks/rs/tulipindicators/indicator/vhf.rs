use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_vhf_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int;
}
pub unsafe extern "C" fn ti_vhf(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut in_0: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_vhf_start(options) {
        return 0 as libc::c_int;
    }
    let mut trail: libc::c_int = 1 as libc::c_int;
    let mut maxi: libc::c_int = -(1 as libc::c_int);
    let mut mini: libc::c_int = -(1 as libc::c_int);
    let mut max: libc::c_double = *in_0.offset(0 as libc::c_int as isize);
    let mut min: libc::c_double = *in_0.offset(0 as libc::c_int as isize);
    let mut bar: libc::c_double = 0.;
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut yc: libc::c_double = *in_0.offset(0 as libc::c_int as isize);
    let mut c: libc::c_double = 0.;
    i = 1 as libc::c_int;
    while i < period {
        c = *in_0.offset(i as isize);
        sum += fabs(c - yc);
        yc = c;
        i += 1;
        i;
    }
    i = period;
    while i < size {
        c = *in_0.offset(i as isize);
        sum += fabs(c - yc);
        yc = c;
        if i > period {
            sum
                -= fabs(
                    *in_0.offset((i - period) as isize)
                        - *in_0.offset((i - period - 1 as libc::c_int) as isize),
                );
        }
        bar = c;
        if maxi < trail {
            maxi = trail;
            max = *in_0.offset(maxi as isize);
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = *in_0.offset(j as isize);
                if bar >= max {
                    max = bar;
                    maxi = j;
                }
            }
        } else if bar >= max {
            maxi = i;
            max = bar;
        }
        bar = c;
        if mini < trail {
            mini = trail;
            min = *in_0.offset(mini as isize);
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = *in_0.offset(j as isize);
                if bar <= min {
                    min = bar;
                    mini = j;
                }
            }
        } else if bar <= min {
            mini = i;
            min = bar;
        }
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = fabs(max - min) / sum;
        i += 1;
        i;
        trail += 1;
        trail;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_vhf_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_vhf_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/vhf.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_vhf(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
