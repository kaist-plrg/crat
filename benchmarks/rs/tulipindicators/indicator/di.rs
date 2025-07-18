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
pub unsafe extern "C" fn ti_di_start(mut options: *const libc::c_double) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_di(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut plus_di: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut minus_di: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_di_start(options) {
        return 0 as libc::c_int;
    }
    let per: libc::c_double = (period as libc::c_double
        - 1 as libc::c_int as libc::c_double) / period as libc::c_double;
    let mut atr: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut dmup: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut dmdown: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < period {
        let mut truerange: libc::c_double = 0.;
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
        atr += truerange;
        let mut dp: libc::c_double = 0.;
        let mut dm: libc::c_double = 0.;
        dp = *high.offset(i as isize) - *high.offset((i - 1 as libc::c_int) as isize);
        dm = *low.offset((i - 1 as libc::c_int) as isize) - *low.offset(i as isize);
        if dp < 0 as libc::c_int as libc::c_double {
            dp = 0 as libc::c_int as libc::c_double;
        } else if dp > dm {
            dm = 0 as libc::c_int as libc::c_double;
        }
        if dm < 0 as libc::c_int as libc::c_double {
            dm = 0 as libc::c_int as libc::c_double;
        } else if dm > dp {
            dp = 0 as libc::c_int as libc::c_double;
        }
        dmup += dp;
        dmdown += dm;
        i += 1;
        i;
    }
    let fresh0 = plus_di;
    plus_di = plus_di.offset(1);
    *fresh0 = 100.0f64 * dmup / atr;
    let fresh1 = minus_di;
    minus_di = minus_di.offset(1);
    *fresh1 = 100.0f64 * dmdown / atr;
    i = period;
    while i < size {
        let mut truerange_0: libc::c_double = 0.;
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
        truerange_0 = v_0;
        atr = atr * per + truerange_0;
        let mut dp_0: libc::c_double = 0.;
        let mut dm_0: libc::c_double = 0.;
        dp_0 = *high.offset(i as isize) - *high.offset((i - 1 as libc::c_int) as isize);
        dm_0 = *low.offset((i - 1 as libc::c_int) as isize) - *low.offset(i as isize);
        if dp_0 < 0 as libc::c_int as libc::c_double {
            dp_0 = 0 as libc::c_int as libc::c_double;
        } else if dp_0 > dm_0 {
            dm_0 = 0 as libc::c_int as libc::c_double;
        }
        if dm_0 < 0 as libc::c_int as libc::c_double {
            dm_0 = 0 as libc::c_int as libc::c_double;
        } else if dm_0 > dp_0 {
            dp_0 = 0 as libc::c_int as libc::c_double;
        }
        dmup = dmup * per + dp_0;
        dmdown = dmdown * per + dm_0;
        let fresh2 = plus_di;
        plus_di = plus_di.offset(1);
        *fresh2 = 100.0f64 * dmup / atr;
        let fresh3 = minus_di;
        minus_di = minus_di.offset(1);
        *fresh3 = 100.0f64 * dmdown / atr;
        i += 1;
        i;
    }
    if plus_di.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_di_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"plus_di - outputs[0] == size - ti_di_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/di.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"int ti_di(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    if minus_di.offset_from(*outputs.offset(1 as libc::c_int as isize)) as libc::c_long
        == (size - ti_di_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"minus_di - outputs[1] == size - ti_di_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/di.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"int ti_di(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
