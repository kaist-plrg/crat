use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_nvi_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ti_nvi(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut close: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut volume: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if size <= ti_nvi_start(options) {
        return 0 as libc::c_int;
    }
    let mut nvi: libc::c_double = 1000 as libc::c_int as libc::c_double;
    let fresh0 = output;
    output = output.offset(1);
    *fresh0 = nvi;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        if *volume.offset(i as isize) < *volume.offset((i - 1 as libc::c_int) as isize) {
            nvi
                += (*close.offset(i as isize)
                    - *close.offset((i - 1 as libc::c_int) as isize))
                    / *close.offset((i - 1 as libc::c_int) as isize) * nvi;
        }
        let fresh1 = output;
        output = output.offset(1);
        *fresh1 = nvi;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_nvi_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_nvi_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/nvi.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_nvi(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
