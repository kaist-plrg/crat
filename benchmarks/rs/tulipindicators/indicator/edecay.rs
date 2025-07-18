use ::libc;
pub unsafe extern "C" fn ti_edecay_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ti_edecay(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let scale: libc::c_double = 1.0f64 - 1.0f64 / period as libc::c_double;
    let fresh0 = output;
    output = output.offset(1);
    *fresh0 = *input.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        let mut d: libc::c_double = *output.offset(-(1 as libc::c_int) as isize) * scale;
        let fresh1 = output;
        output = output.offset(1);
        *fresh1 = if *input.offset(i as isize) > d {
            *input.offset(i as isize)
        } else {
            d
        };
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
