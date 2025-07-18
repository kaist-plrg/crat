use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_psar_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_psar(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let accel_step: libc::c_double = *options.offset(0 as libc::c_int as isize);
    let accel_max: libc::c_double = *options.offset(1 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if accel_step <= 0 as libc::c_int as libc::c_double {
        return 1 as libc::c_int;
    }
    if accel_max <= accel_step {
        return 1 as libc::c_int;
    }
    if size < 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut lng: libc::c_int = 0;
    if *high.offset(0 as libc::c_int as isize) + *low.offset(0 as libc::c_int as isize)
        <= *high.offset(1 as libc::c_int as isize)
            + *low.offset(1 as libc::c_int as isize)
    {
        lng = 1 as libc::c_int;
    } else {
        lng = 0 as libc::c_int;
    }
    let mut sar: libc::c_double = 0.;
    let mut extreme: libc::c_double = 0.;
    if lng != 0 {
        extreme = *high.offset(0 as libc::c_int as isize);
        sar = *low.offset(0 as libc::c_int as isize);
    } else {
        extreme = *low.offset(0 as libc::c_int as isize);
        sar = *high.offset(0 as libc::c_int as isize);
    }
    let mut accel: libc::c_double = accel_step;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        sar = (extreme - sar) * accel + sar;
        if lng != 0 {
            if i >= 2 as libc::c_int
                && sar > *low.offset((i - 2 as libc::c_int) as isize)
            {
                sar = *low.offset((i - 2 as libc::c_int) as isize);
            }
            if sar > *low.offset((i - 1 as libc::c_int) as isize) {
                sar = *low.offset((i - 1 as libc::c_int) as isize);
            }
            if accel < accel_max && *high.offset(i as isize) > extreme {
                accel += accel_step;
                if accel > accel_max {
                    accel = accel_max;
                }
            }
            if *high.offset(i as isize) > extreme {
                extreme = *high.offset(i as isize);
            }
        } else {
            if i >= 2 as libc::c_int
                && sar < *high.offset((i - 2 as libc::c_int) as isize)
            {
                sar = *high.offset((i - 2 as libc::c_int) as isize);
            }
            if sar < *high.offset((i - 1 as libc::c_int) as isize) {
                sar = *high.offset((i - 1 as libc::c_int) as isize);
            }
            if accel < accel_max && *low.offset(i as isize) < extreme {
                accel += accel_step;
                if accel > accel_max {
                    accel = accel_max;
                }
            }
            if *low.offset(i as isize) < extreme {
                extreme = *low.offset(i as isize);
            }
        }
        if lng != 0 && *low.offset(i as isize) < sar
            || lng == 0 && *high.offset(i as isize) > sar
        {
            accel = accel_step;
            sar = extreme;
            lng = (lng == 0) as libc::c_int;
            if lng == 0 {
                extreme = *low.offset(i as isize);
            } else {
                extreme = *high.offset(i as isize);
            }
        }
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = sar;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_psar_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_psar_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/psar.c\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_psar(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
