use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_dm_start(mut options: *const libc::c_double) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_dm(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut plus_dm: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut minus_dm: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_dm_start(options) {
        return 0 as libc::c_int;
    }
    let per: libc::c_double = (period as libc::c_double
        - 1 as libc::c_int as libc::c_double) / period as libc::c_double;
    let mut dmup: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut dmdown: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < period {
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
    let fresh0 = plus_dm;
    plus_dm = plus_dm.offset(1);
    *fresh0 = dmup;
    let fresh1 = minus_dm;
    minus_dm = minus_dm.offset(1);
    *fresh1 = dmdown;
    i = period;
    while i < size {
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
        let fresh2 = plus_dm;
        plus_dm = plus_dm.offset(1);
        *fresh2 = dmup;
        let fresh3 = minus_dm;
        minus_dm = minus_dm.offset(1);
        *fresh3 = dmdown;
        i += 1;
        i;
    }
    if plus_dm.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_dm_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"plus_dm - outputs[0] == size - ti_dm_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/dm.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"int ti_dm(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    if minus_dm.offset_from(*outputs.offset(1 as libc::c_int as isize)) as libc::c_long
        == (size - ti_dm_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"minus_dm - outputs[1] == size - ti_dm_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/dm.c\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 71],
                &[libc::c_char; 71],
            >(
                b"int ti_dm(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
