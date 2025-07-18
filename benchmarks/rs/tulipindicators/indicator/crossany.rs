use ::libc;
pub unsafe extern "C" fn ti_crossany_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_crossany(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut a: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut b: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = (*a.offset(i as isize) > *b.offset(i as isize)
            && *a.offset((i - 1 as libc::c_int) as isize)
                <= *b.offset((i - 1 as libc::c_int) as isize)
            || *a.offset(i as isize) < *b.offset(i as isize)
                && *a.offset((i - 1 as libc::c_int) as isize)
                    >= *b.offset((i - 1 as libc::c_int) as isize)) as libc::c_int
            as libc::c_double;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
