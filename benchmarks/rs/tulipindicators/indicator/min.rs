use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_min_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_min(
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
    if size <= ti_min_start(options) {
        return 0 as libc::c_int;
    }
    let mut trail: libc::c_int = 0 as libc::c_int;
    let mut mini: libc::c_int = -(1 as libc::c_int);
    let mut min: libc::c_double = *input.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = period - 1 as libc::c_int;
    while i < size {
        let mut bar: libc::c_double = *input.offset(i as isize);
        if mini < trail {
            mini = trail;
            min = *input.offset(mini as isize);
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = *input.offset(j as isize);
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
        *fresh0 = min;
        i += 1;
        i;
        trail += 1;
        trail;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_min_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_min_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/min.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_min(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ti_min_ref(
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
    if size <= ti_min_start(options) {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = period - 1 as libc::c_int;
    while i < size {
        let mut min: libc::c_double = *input
            .offset((i - period + 1 as libc::c_int) as isize);
        let mut j: libc::c_int = i - period + 2 as libc::c_int;
        while j <= i {
            min = if min < *input.offset(j as isize) {
                min
            } else {
                *input.offset(j as isize)
            };
            j += 1;
            j;
        }
        let fresh1 = output;
        output = output.offset(1);
        *fresh1 = min;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_min_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_min_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/min.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"int ti_min_ref(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
