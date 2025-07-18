use ::libc;
pub unsafe extern "C" fn ti_obv_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ti_obv(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut close: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut volume: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let fresh0 = output;
    output = output.offset(1);
    *fresh0 = sum;
    let mut prev: libc::c_double = *close.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        if *close.offset(i as isize) > prev {
            sum += *volume.offset(i as isize);
        } else if *close.offset(i as isize) < prev {
            sum -= *volume.offset(i as isize);
        }
        prev = *close.offset(i as isize);
        let fresh1 = output;
        output = output.offset(1);
        *fresh1 = sum;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
