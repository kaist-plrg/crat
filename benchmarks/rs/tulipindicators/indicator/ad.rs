use ::libc;
pub unsafe extern "C" fn ti_ad_start(mut options: *const libc::c_double) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ti_ad(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut volume: *const libc::c_double = *inputs.offset(3 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        let hl: libc::c_double = *high.offset(i as isize) - *low.offset(i as isize);
        if hl != 0.0f64 {
            sum
                += (*close.offset(i as isize) - *low.offset(i as isize)
                    - *high.offset(i as isize) + *close.offset(i as isize)) / hl
                    * *volume.offset(i as isize);
        }
        *output.offset(i as isize) = sum;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
