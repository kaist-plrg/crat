use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_linregintercept_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_linregintercept(
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
    if size <= ti_linregintercept_start(options) {
        return 0 as libc::c_int;
    }
    let mut x: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut x2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut y: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut xy: libc::c_double = 0 as libc::c_int as libc::c_double;
    let p: libc::c_double = 1.0f64 / period as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period - 1 as libc::c_int {
        x += (i + 1 as libc::c_int) as libc::c_double;
        x2 += ((i + 1 as libc::c_int) * (i + 1 as libc::c_int)) as libc::c_double;
        xy += *input.offset(i as isize) * (i + 1 as libc::c_int) as libc::c_double;
        y += *input.offset(i as isize);
        i += 1;
        i;
    }
    x += period as libc::c_double;
    x2 += (period * period) as libc::c_double;
    let bd: libc::c_double = 1.0f64 / (period as libc::c_double * x2 - x * x);
    i = period - 1 as libc::c_int;
    while i < size {
        xy += *input.offset(i as isize) * period as libc::c_double;
        y += *input.offset(i as isize);
        let b: libc::c_double = (period as libc::c_double * xy - x * y) * bd;
        let a: libc::c_double = (y - b * x) * p;
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = a + b * 1 as libc::c_int as libc::c_double;
        xy -= y;
        y -= *input.offset((i - period + 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_linregintercept_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_linregintercept_start(options)\0"
                as *const u8 as *const libc::c_char,
            b"indicator/linregintercept.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 84],
                &[libc::c_char; 84],
            >(
                b"int ti_linregintercept(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
