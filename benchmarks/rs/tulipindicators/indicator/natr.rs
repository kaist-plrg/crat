use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_natr_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_natr(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_natr_start(options) {
        return 0 as libc::c_int;
    }
    let per: libc::c_double = 1.0f64 / period as libc::c_double;
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut truerange: libc::c_double = 0.;
    sum
        += *high.offset(0 as libc::c_int as isize)
            - *low.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < period {
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
        sum += truerange;
        i += 1;
        i;
    }
    let mut val: libc::c_double = sum / period as libc::c_double;
    let fresh0 = output;
    output = output.offset(1);
    *fresh0 = 100 as libc::c_int as libc::c_double * val
        / *close.offset((period - 1 as libc::c_int) as isize);
    i = period;
    while i < size {
        let l_0: libc::c_double = *low.offset(i as isize);
        let h_0: libc::c_double = *high.offset(i as isize);
        let c_0: libc::c_double = *close.offset((i - 1 as libc::c_int) as isize);
        let ych_0: libc::c_double = fabs(h_0 - c_0);
        let ycl_0: libc::c_double = fabs(l_0 - c_0);
        let mut v_0: libc::c_double = h_0 - l_0;
        if ych_0 > v_0 {
            v_0 = ych_0;
        }
        if ycl_0 > v_0 {
            v_0 = ycl_0;
        }
        truerange = v_0;
        val = (truerange - val) * per + val;
        let fresh1 = output;
        output = output.offset(1);
        *fresh1 = 100 as libc::c_int as libc::c_double * val / *close.offset(i as isize);
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_natr_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_natr_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/natr.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_natr(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
