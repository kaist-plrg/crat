use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_zlema_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return (*options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int)
        / 2 as libc::c_int - 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_zlema(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let lag: libc::c_int = (period - 1 as libc::c_int) / 2 as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 3 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_zlema_start(options) {
        return 0 as libc::c_int;
    }
    let per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let mut val: libc::c_double = *input.offset((lag - 1 as libc::c_int) as isize);
    let fresh0 = output;
    output = output.offset(1);
    *fresh0 = val;
    let mut i: libc::c_int = 0;
    i = lag;
    while i < size {
        let mut c: libc::c_double = *input.offset(i as isize);
        let mut l: libc::c_double = *input.offset((i - lag) as isize);
        val = (c + (c - l) - val) * per + val;
        let fresh1 = output;
        output = output.offset(1);
        *fresh1 = val;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_zlema_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_zlema_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/zlema.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"int ti_zlema(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
