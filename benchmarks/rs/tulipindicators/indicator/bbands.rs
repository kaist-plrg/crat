use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_bbands_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_bbands(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut lower: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut middle: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    let mut upper: *mut libc::c_double = *outputs.offset(2 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let stddev: libc::c_double = *options.offset(1 as libc::c_int as isize);
    let scale: libc::c_double = 1.0f64 / period as libc::c_double;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_bbands_start(options) {
        return 0 as libc::c_int;
    }
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sum2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        sum += *input.offset(i as isize);
        sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        i += 1;
        i;
    }
    let mut sd: libc::c_double = sqrt(sum2 * scale - sum * scale * (sum * scale));
    *middle = sum * scale;
    let fresh0 = lower;
    lower = lower.offset(1);
    *fresh0 = *middle - stddev * sd;
    let fresh1 = upper;
    upper = upper.offset(1);
    *fresh1 = *middle + stddev * sd;
    middle = middle.offset(1);
    middle;
    i = period;
    while i < size {
        sum += *input.offset(i as isize);
        sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        sum -= *input.offset((i - period) as isize);
        sum2
            -= *input.offset((i - period) as isize)
                * *input.offset((i - period) as isize);
        sd = sqrt(sum2 * scale - sum * scale * (sum * scale));
        *middle = sum * scale;
        let fresh2 = upper;
        upper = upper.offset(1);
        *fresh2 = *middle + stddev * sd;
        let fresh3 = lower;
        lower = lower.offset(1);
        *fresh3 = *middle - stddev * sd;
        middle = middle.offset(1);
        middle;
        i += 1;
        i;
    }
    if lower.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_bbands_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"lower - outputs[0] == size - ti_bbands_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/bbands.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"int ti_bbands(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    if middle.offset_from(*outputs.offset(1 as libc::c_int as isize)) as libc::c_long
        == (size - ti_bbands_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"middle - outputs[1] == size - ti_bbands_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/bbands.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"int ti_bbands(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    if upper.offset_from(*outputs.offset(2 as libc::c_int as isize)) as libc::c_long
        == (size - ti_bbands_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"upper - outputs[2] == size - ti_bbands_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/bbands.c\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"int ti_bbands(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
