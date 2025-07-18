use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_ao_start(mut options: *const libc::c_double) -> libc::c_int {
    return 33 as libc::c_int;
}
pub unsafe extern "C" fn ti_ao(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let period: libc::c_int = 34 as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if size <= ti_ao_start(options) {
        return 0 as libc::c_int;
    }
    let mut sum34: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sum5: libc::c_double = 0 as libc::c_int as libc::c_double;
    let per34: libc::c_double = 1.0f64 / 34.0f64;
    let per5: libc::c_double = 1.0f64 / 5.0f64;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 34 as libc::c_int {
        let mut hl: libc::c_double = 0.5f64
            * (*high.offset(i as isize) + *low.offset(i as isize));
        sum34 += hl;
        if i >= 29 as libc::c_int {
            sum5 += hl;
        }
        i += 1;
        i;
    }
    let fresh0 = output;
    output = output.offset(1);
    *fresh0 = per5 * sum5 - per34 * sum34;
    i = period;
    while i < size {
        let mut hl_0: libc::c_double = 0.5f64
            * (*high.offset(i as isize) + *low.offset(i as isize));
        sum34 += hl_0;
        sum5 += hl_0;
        sum34
            -= 0.5f64
                * (*high.offset((i - 34 as libc::c_int) as isize)
                    + *low.offset((i - 34 as libc::c_int) as isize));
        sum5
            -= 0.5f64
                * (*high.offset((i - 5 as libc::c_int) as isize)
                    + *low.offset((i - 5 as libc::c_int) as isize));
        let fresh1 = output;
        output = output.offset(1);
        *fresh1 = per5 * sum5 - per34 * sum34;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_ao_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_ao_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/ao.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"int ti_ao(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
