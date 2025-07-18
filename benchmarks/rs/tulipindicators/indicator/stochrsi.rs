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
pub unsafe extern "C" fn ti_stochrsi_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int * 2 as libc::c_int
        - 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_stochrsi(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let per: libc::c_double = 1.0f64 / period as libc::c_double;
    if period < 2 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_stochrsi_start(options) {
        return 0 as libc::c_int;
    }
    let mut rsi: *mut ti_buffer = ti_buffer_new(period);
    let mut smooth_up: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut smooth_down: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= period {
        let upward: libc::c_double = if *input.offset(i as isize)
            > *input.offset((i - 1 as libc::c_int) as isize)
        {
            *input.offset(i as isize) - *input.offset((i - 1 as libc::c_int) as isize)
        } else {
            0 as libc::c_int as libc::c_double
        };
        let downward: libc::c_double = if *input.offset(i as isize)
            < *input.offset((i - 1 as libc::c_int) as isize)
        {
            *input.offset((i - 1 as libc::c_int) as isize) - *input.offset(i as isize)
        } else {
            0 as libc::c_int as libc::c_double
        };
        smooth_up += upward;
        smooth_down += downward;
        i += 1;
        i;
    }
    smooth_up /= period as libc::c_double;
    smooth_down /= period as libc::c_double;
    let mut r: libc::c_double = 100.0f64 * (smooth_up / (smooth_up + smooth_down));
    if (*rsi).pushes >= (*rsi).size {
        (*rsi).sum -= *((*rsi).vals).as_mut_ptr().offset((*rsi).index as isize);
    }
    (*rsi).sum += r;
    *((*rsi).vals).as_mut_ptr().offset((*rsi).index as isize) = r;
    (*rsi).pushes += 1 as libc::c_int;
    (*rsi).index = (*rsi).index + 1 as libc::c_int;
    if (*rsi).index >= (*rsi).size {
        (*rsi).index = 0 as libc::c_int;
    }
    let mut min: libc::c_double = r;
    let mut max: libc::c_double = r;
    let mut mini: libc::c_int = 0 as libc::c_int;
    let mut maxi: libc::c_int = 0 as libc::c_int;
    i = period + 1 as libc::c_int;
    while i < size {
        let upward_0: libc::c_double = if *input.offset(i as isize)
            > *input.offset((i - 1 as libc::c_int) as isize)
        {
            *input.offset(i as isize) - *input.offset((i - 1 as libc::c_int) as isize)
        } else {
            0 as libc::c_int as libc::c_double
        };
        let downward_0: libc::c_double = if *input.offset(i as isize)
            < *input.offset((i - 1 as libc::c_int) as isize)
        {
            *input.offset((i - 1 as libc::c_int) as isize) - *input.offset(i as isize)
        } else {
            0 as libc::c_int as libc::c_double
        };
        smooth_up = (upward_0 - smooth_up) * per + smooth_up;
        smooth_down = (downward_0 - smooth_down) * per + smooth_down;
        r = 100.0f64 * (smooth_up / (smooth_up + smooth_down));
        if r > max {
            max = r;
            maxi = (*rsi).index;
        } else if maxi == (*rsi).index {
            max = r;
            let mut j: libc::c_int = 0;
            j = 0 as libc::c_int;
            while j < (*rsi).size {
                if !(j == (*rsi).index) {
                    if *((*rsi).vals).as_mut_ptr().offset(j as isize) > max {
                        max = *((*rsi).vals).as_mut_ptr().offset(j as isize);
                        maxi = j;
                    }
                }
                j += 1;
                j;
            }
        }
        if r < min {
            min = r;
            mini = (*rsi).index;
        } else if mini == (*rsi).index {
            min = r;
            let mut j_0: libc::c_int = 0;
            j_0 = 0 as libc::c_int;
            while j_0 < (*rsi).size {
                if !(j_0 == (*rsi).index) {
                    if *((*rsi).vals).as_mut_ptr().offset(j_0 as isize) < min {
                        min = *((*rsi).vals).as_mut_ptr().offset(j_0 as isize);
                        mini = j_0;
                    }
                }
                j_0 += 1;
                j_0;
            }
        }
        *((*rsi).vals).as_mut_ptr().offset((*rsi).index as isize) = r;
        (*rsi).index = (*rsi).index + 1 as libc::c_int;
        if (*rsi).index >= (*rsi).size {
            (*rsi).index = 0 as libc::c_int;
        }
        if i > period * 2 as libc::c_int - 2 as libc::c_int {
            let diff: libc::c_double = max - min;
            if diff == 0.0f64 {
                let fresh0 = output;
                output = output.offset(1);
                *fresh0 = 0.0f64;
            } else {
                let fresh1 = output;
                output = output.offset(1);
                *fresh1 = (r - min) / diff;
            }
        }
        i += 1;
        i;
    }
    ti_buffer_free(rsi);
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_stochrsi_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_stochrsi_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/stochrsi.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 77],
                &[libc::c_char; 77],
            >(
                b"int ti_stochrsi(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
