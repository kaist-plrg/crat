use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_marketfi_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ti_marketfi(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut volume: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if size <= ti_marketfi_start(options) {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = (*high.offset(i as isize) - *low.offset(i as isize))
            / *volume.offset(i as isize);
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_marketfi_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_marketfi_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/marketfi.c\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 77],
                &[libc::c_char; 77],
            >(
                b"int ti_marketfi(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
