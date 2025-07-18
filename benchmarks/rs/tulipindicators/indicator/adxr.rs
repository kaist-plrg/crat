use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn ti_buffer_new(size: libc::c_int) -> *mut ti_buffer;
    fn ti_buffer_free(buffer: *mut ti_buffer);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_buffer {
    pub size: libc::c_int,
    pub pushes: libc::c_int,
    pub index: libc::c_int,
    pub sum: libc::c_double,
    pub vals: [libc::c_double; 1],
}
pub unsafe extern "C" fn ti_adxr_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return (*options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int)
        * 3 as libc::c_int;
}
pub unsafe extern "C" fn ti_adxr(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 2 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_adxr_start(options) {
        return 0 as libc::c_int;
    }
    let per: libc::c_double = (period as libc::c_double
        - 1 as libc::c_int as libc::c_double) / period as libc::c_double;
    let invper: libc::c_double = 1.0f64 / period as libc::c_double;
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
    let mut adx: libc::c_double = 0.0f64;
    let mut di_up: libc::c_double = dmup;
    let mut di_down: libc::c_double = dmdown;
    let mut dm_diff: libc::c_double = fabs(di_up - di_down);
    let mut dm_sum: libc::c_double = di_up + di_down;
    let mut dx: libc::c_double = dm_diff / dm_sum * 100 as libc::c_int as libc::c_double;
    adx += dx;
    let mut adxr: *mut ti_buffer = ti_buffer_new(period - 1 as libc::c_int);
    let first_adxr: libc::c_int = ti_adxr_start(options);
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
        let mut di_up_0: libc::c_double = dmup;
        let mut di_down_0: libc::c_double = dmdown;
        let mut dm_diff_0: libc::c_double = fabs(di_up_0 - di_down_0);
        let mut dm_sum_0: libc::c_double = di_up_0 + di_down_0;
        let mut dx_0: libc::c_double = dm_diff_0 / dm_sum_0
            * 100 as libc::c_int as libc::c_double;
        if i - period < period - 2 as libc::c_int {
            adx += dx_0;
        } else if i - period == period - 2 as libc::c_int {
            adx += dx_0;
            *((*adxr).vals).as_mut_ptr().offset((*adxr).index as isize) = adx * invper;
            (*adxr).index = (*adxr).index + 1 as libc::c_int;
            if (*adxr).index >= (*adxr).size {
                (*adxr).index = 0 as libc::c_int;
            }
        } else {
            adx = adx * per + dx_0;
            if i >= first_adxr {
                let fresh0 = output;
                output = output.offset(1);
                *fresh0 = 0.5f64
                    * (adx * invper
                        + *((*adxr).vals)
                            .as_mut_ptr()
                            .offset(
                                (((*adxr).index + (*adxr).size - 1 as libc::c_int
                                    + 1 as libc::c_int) % (*adxr).size) as isize,
                            ));
            }
            *((*adxr).vals).as_mut_ptr().offset((*adxr).index as isize) = adx * invper;
            (*adxr).index = (*adxr).index + 1 as libc::c_int;
            if (*adxr).index >= (*adxr).size {
                (*adxr).index = 0 as libc::c_int;
            }
        }
        i += 1;
        i;
    }
    ti_buffer_free(adxr);
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_adxr_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_adxr_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/adxr.c\0" as *const u8 as *const libc::c_char,
            112 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_adxr(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
