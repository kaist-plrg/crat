use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_emv_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_emv(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut volume: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if size <= ti_emv_start(options) {
        return 0 as libc::c_int;
    }
    let mut last: libc::c_double = (*high.offset(0 as libc::c_int as isize)
        + *low.offset(0 as libc::c_int as isize)) * 0.5f64;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        let mut hl: libc::c_double = (*high.offset(i as isize) + *low.offset(i as isize))
            * 0.5f64;
        let mut br: libc::c_double = *volume.offset(i as isize) / 10000.0f64
            / (*high.offset(i as isize) - *low.offset(i as isize));
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = (hl - last) / br;
        last = hl;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_emv_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_emv_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/emv.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_emv(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
