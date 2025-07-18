use ::libc;
extern "C" {
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
pub unsafe extern "C" fn ti_ultosc_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(2 as libc::c_int as isize) as libc::c_int;
}
pub unsafe extern "C" fn ti_ultosc(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let short_period: libc::c_int = *options.offset(0 as libc::c_int as isize)
        as libc::c_int;
    let medium_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    let long_period: libc::c_int = *options.offset(2 as libc::c_int as isize)
        as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if short_period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if medium_period < short_period {
        return 1 as libc::c_int;
    }
    if long_period < medium_period {
        return 1 as libc::c_int;
    }
    if size <= ti_ultosc_start(options) {
        return 0 as libc::c_int;
    }
    let mut bp_buf: *mut ti_buffer = ti_buffer_new(long_period);
    let mut r_buf: *mut ti_buffer = ti_buffer_new(long_period);
    let mut bp_short_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut bp_medium_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut r_short_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut r_medium_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        let true_low: libc::c_double = if *low.offset(i as isize)
            < *close.offset((i - 1 as libc::c_int) as isize)
        {
            *low.offset(i as isize)
        } else {
            *close.offset((i - 1 as libc::c_int) as isize)
        };
        let true_high: libc::c_double = if *high.offset(i as isize)
            > *close.offset((i - 1 as libc::c_int) as isize)
        {
            *high.offset(i as isize)
        } else {
            *close.offset((i - 1 as libc::c_int) as isize)
        };
        let bp: libc::c_double = *close.offset(i as isize) - true_low;
        let r: libc::c_double = true_high - true_low;
        bp_short_sum += bp;
        bp_medium_sum += bp;
        r_short_sum += r;
        r_medium_sum += r;
        if (*bp_buf).pushes >= (*bp_buf).size {
            (*bp_buf).sum
                -= *((*bp_buf).vals).as_mut_ptr().offset((*bp_buf).index as isize);
        }
        (*bp_buf).sum += bp;
        *((*bp_buf).vals).as_mut_ptr().offset((*bp_buf).index as isize) = bp;
        (*bp_buf).pushes += 1 as libc::c_int;
        (*bp_buf).index = (*bp_buf).index + 1 as libc::c_int;
        if (*bp_buf).index >= (*bp_buf).size {
            (*bp_buf).index = 0 as libc::c_int;
        }
        if (*r_buf).pushes >= (*r_buf).size {
            (*r_buf).sum
                -= *((*r_buf).vals).as_mut_ptr().offset((*r_buf).index as isize);
        }
        (*r_buf).sum += r;
        *((*r_buf).vals).as_mut_ptr().offset((*r_buf).index as isize) = r;
        (*r_buf).pushes += 1 as libc::c_int;
        (*r_buf).index = (*r_buf).index + 1 as libc::c_int;
        if (*r_buf).index >= (*r_buf).size {
            (*r_buf).index = 0 as libc::c_int;
        }
        if i > short_period {
            let mut short_index: libc::c_int = (*bp_buf).index - short_period
                - 1 as libc::c_int;
            if short_index < 0 as libc::c_int {
                short_index += long_period;
            }
            bp_short_sum -= *((*bp_buf).vals).as_mut_ptr().offset(short_index as isize);
            r_short_sum -= *((*r_buf).vals).as_mut_ptr().offset(short_index as isize);
            if i > medium_period {
                let mut medium_index: libc::c_int = (*bp_buf).index - medium_period
                    - 1 as libc::c_int;
                if medium_index < 0 as libc::c_int {
                    medium_index += long_period;
                }
                bp_medium_sum
                    -= *((*bp_buf).vals).as_mut_ptr().offset(medium_index as isize);
                r_medium_sum
                    -= *((*r_buf).vals).as_mut_ptr().offset(medium_index as isize);
            }
        }
        if i >= long_period {
            let first: libc::c_double = 4 as libc::c_int as libc::c_double * bp_short_sum
                / r_short_sum;
            let second: libc::c_double = 2 as libc::c_int as libc::c_double
                * bp_medium_sum / r_medium_sum;
            let third: libc::c_double = 1 as libc::c_int as libc::c_double
                * (*bp_buf).sum / (*r_buf).sum;
            let ult: libc::c_double = (first + second + third) * 100.0f64 / 7.0f64;
            let fresh0 = output;
            output = output.offset(1);
            *fresh0 = ult;
        }
        i += 1;
        i;
    }
    ti_buffer_free(bp_buf);
    ti_buffer_free(r_buf);
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_ultosc_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_ultosc_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/ultosc.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"int ti_ultosc(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
