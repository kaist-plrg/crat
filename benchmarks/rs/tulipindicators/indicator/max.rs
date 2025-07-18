use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_max_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_max(
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
    if size <= ti_max_start(options) {
        return 0 as libc::c_int;
    }
    let mut trail: libc::c_int = 0 as libc::c_int;
    let mut maxi: libc::c_int = -(1 as libc::c_int);
    let mut max: libc::c_double = *input.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = period - 1 as libc::c_int;
    while i < size {
        let mut bar: libc::c_double = *input.offset(i as isize);
        if maxi < trail {
            maxi = trail;
            max = *input.offset(maxi as isize);
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = *input.offset(j as isize);
                if bar >= max {
                    max = bar;
                    maxi = j;
                }
            }
        } else if bar >= max {
            maxi = i;
            max = bar;
        }
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = max;
        i += 1;
        i;
        trail += 1;
        trail;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_max_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_max_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/max.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_max(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ti_max_ref(
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
    if size <= ti_max_start(options) {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = period - 1 as libc::c_int;
    while i < size {
        let mut max: libc::c_double = *input
            .offset((i - period + 1 as libc::c_int) as isize);
        let mut j: libc::c_int = i - period + 2 as libc::c_int;
        while j <= i {
            max = if max > *input.offset(j as isize) {
                max
            } else {
                *input.offset(j as isize)
            };
            j += 1;
            j;
        }
        let fresh1 = output;
        output = output.offset(1);
        *fresh1 = max;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
