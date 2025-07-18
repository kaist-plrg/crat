use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn ti_tr_start(mut options: *const libc::c_double) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ti_tr(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut truerange: libc::c_double = 0.;
    *output
        .offset(
            0 as libc::c_int as isize,
        ) = *high.offset(0 as libc::c_int as isize)
        - *low.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        let l: libc::c_double = *low.offset(i as isize);
        let h: libc::c_double = *high.offset(i as isize);
        let c: libc::c_double = *close.offset((i - 1 as libc::c_int) as isize);
        let ych: libc::c_double = fabs(h - c);
        let ycl: libc::c_double = fabs(l - c);
        let mut v: libc::c_double = h - l;
        if ych > v {
            v = ych;
        }
        if ycl > v {
            v = ycl;
        }
        truerange = v;
        *output.offset(i as isize) = truerange;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
