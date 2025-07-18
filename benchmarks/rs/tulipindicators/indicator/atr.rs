use ::libc;
extern "C" {
    pub type ti_stream;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn ti_tr_start(options: *const libc::c_double) -> libc::c_int;
    fn ti_tr(
        size: libc::c_int,
        inputs: *const *const libc::c_double,
        options: *const libc::c_double,
        outputs: *const *mut libc::c_double,
    ) -> libc::c_int;
    fn ti_wilders_start(options: *const libc::c_double) -> libc::c_int;
    fn ti_wilders(
        size: libc::c_int,
        inputs: *const *const libc::c_double,
        options: *const libc::c_double,
        outputs: *const *mut libc::c_double,
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_stream_atr {
    pub index: libc::c_int,
    pub progress: libc::c_int,
    pub period: libc::c_int,
    pub sum: libc::c_double,
    pub last: libc::c_double,
    pub last_close: libc::c_double,
}
pub unsafe extern "C" fn ti_atr_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_atr(
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
    if size <= ti_atr_start(options) {
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
    *fresh0 = val;
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
        *fresh1 = val;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_atr_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_atr_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/atr.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_atr(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ti_atr_ref(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_atr_start(options) {
        return 0 as libc::c_int;
    }
    let tr_start: libc::c_int = ti_tr_start(options);
    let tr_size: libc::c_int = size - tr_start;
    let mut truerange: *mut libc::c_double = malloc(
        (tr_size as libc::c_uint as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if truerange.is_null() {
        return 2 as libc::c_int;
    }
    let mut tr_outputs: [*mut libc::c_double; 1] = [truerange];
    let tr_ret: libc::c_int = ti_tr(
        size,
        inputs,
        0 as *const libc::c_double,
        tr_outputs.as_mut_ptr(),
    );
    if tr_ret != 0 as libc::c_int {
        free(truerange as *mut libc::c_void);
        return tr_ret;
    }
    let mut wilders_inputs: [*const libc::c_double; 1] = [
        truerange as *const libc::c_double,
    ];
    let wilders_ret: libc::c_int = ti_wilders(
        tr_size,
        wilders_inputs.as_mut_ptr(),
        options,
        outputs,
    );
    free(truerange as *mut libc::c_void);
    if size - ti_atr_start(options) == size - ti_wilders_start(options) {} else {
        __assert_fail(
            b"size - ti_atr_start(options) == size - ti_wilders_start(options)\0"
                as *const u8 as *const libc::c_char,
            b"indicator/atr.c\0" as *const u8 as *const libc::c_char,
            101 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"int ti_atr_ref(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return wilders_ret;
}
pub unsafe extern "C" fn ti_atr_stream_new(
    mut options: *const libc::c_double,
    mut stream_in: *mut *mut ti_stream,
) -> libc::c_int {
    let mut stream: *mut *mut ti_stream_atr = stream_in as *mut *mut ti_stream_atr;
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    *stream = malloc(::std::mem::size_of::<ti_stream_atr>() as libc::c_ulong)
        as *mut ti_stream_atr;
    if (*stream).is_null() {
        return 2 as libc::c_int;
    }
    (**stream).index = 13 as libc::c_int;
    (**stream).progress = -ti_atr_start(options);
    (**stream).period = period;
    (**stream).sum = 0.0f64;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ti_atr_stream_run(
    mut stream_in: *mut ti_stream,
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut stream: *mut ti_stream_atr = stream_in as *mut ti_stream_atr;
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let per: libc::c_double = 1.0f64 / (*stream).period as libc::c_double;
    let start: libc::c_int = -((*stream).period - 1 as libc::c_int);
    let mut i: libc::c_int = 0 as libc::c_int;
    if (*stream).progress < 1 as libc::c_int {
        if (*stream).progress == start {
            (*stream)
                .sum = *high.offset(0 as libc::c_int as isize)
                - *low.offset(0 as libc::c_int as isize);
            (*stream).last_close = *close.offset(0 as libc::c_int as isize);
            (*stream).progress += 1;
            (*stream).progress;
            i += 1;
            i;
        }
        while (*stream).progress <= 0 as libc::c_int && i < size {
            let mut truerange: libc::c_double = 0.;
            let ych: libc::c_double = fabs(
                *high.offset(i as isize) - (*stream).last_close,
            );
            let ycl: libc::c_double = fabs(
                *low.offset(i as isize) - (*stream).last_close,
            );
            let mut v: libc::c_double = *high.offset(i as isize)
                - *low.offset(i as isize);
            if ych > v {
                v = ych;
            }
            if ycl > v {
                v = ycl;
            }
            truerange = v;
            (*stream).sum += truerange;
            (*stream).last_close = *close.offset(i as isize);
            (*stream).progress += 1;
            (*stream).progress;
            i += 1;
            i;
        }
        if (*stream).progress == 1 as libc::c_int {
            let val: libc::c_double = (*stream).sum * per;
            (*stream).last = val;
            let fresh2 = output;
            output = output.offset(1);
            *fresh2 = val;
        }
    }
    if (*stream).progress >= 1 as libc::c_int {
        let mut val_0: libc::c_double = (*stream).last;
        while i < size {
            let mut truerange_0: libc::c_double = 0.;
            let ych_0: libc::c_double = fabs(
                *high.offset(i as isize) - (*stream).last_close,
            );
            let ycl_0: libc::c_double = fabs(
                *low.offset(i as isize) - (*stream).last_close,
            );
            let mut v_0: libc::c_double = *high.offset(i as isize)
                - *low.offset(i as isize);
            if ych_0 > v_0 {
                v_0 = ych_0;
            }
            if ycl_0 > v_0 {
                v_0 = ycl_0;
            }
            truerange_0 = v_0;
            val_0 = (truerange_0 - val_0) * per + val_0;
            let fresh3 = output;
            output = output.offset(1);
            *fresh3 = val_0;
            (*stream).last_close = *close.offset(i as isize);
            (*stream).progress += 1;
            (*stream).progress;
            i += 1;
            i;
        }
        (*stream).last = val_0;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ti_atr_stream_free(mut stream: *mut ti_stream) {
    free(stream as *mut libc::c_void);
}
