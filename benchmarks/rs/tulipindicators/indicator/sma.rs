use ::libc;
extern "C" {
    pub type ti_stream;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_stream_sma {
    pub index: libc::c_int,
    pub progress: libc::c_int,
    pub period: libc::c_int,
    pub per: libc::c_double,
    pub sum: libc::c_double,
    pub buffer_idx: libc::c_int,
    pub buffer: [libc::c_double; 0],
}
pub unsafe extern "C" fn ti_sma_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_sma(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let scale: libc::c_double = 1.0f64 / period as libc::c_double;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_sma_start(options) {
        return 0 as libc::c_int;
    }
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period {
        sum += *input.offset(i as isize);
        i += 1;
        i;
    }
    let fresh0 = output;
    output = output.offset(1);
    *fresh0 = sum * scale;
    i = period;
    while i < size {
        sum += *input.offset(i as isize);
        sum -= *input.offset((i - period) as isize);
        let fresh1 = output;
        output = output.offset(1);
        *fresh1 = sum * scale;
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_sma_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_sma_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/sma.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_sma(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ti_sma_stream_new(
    mut options: *const libc::c_double,
    mut stream_in: *mut *mut ti_stream,
) -> libc::c_int {
    let mut stream: *mut *mut ti_stream_sma = stream_in as *mut *mut ti_stream_sma;
    let mut period: libc::c_int = *options.offset(0 as libc::c_int as isize)
        as libc::c_int;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    *stream = malloc(
        (::std::mem::size_of::<ti_stream_sma>() as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul(period as libc::c_ulong),
            ),
    ) as *mut ti_stream_sma;
    if stream.is_null() {
        return 2 as libc::c_int;
    }
    (**stream).index = 72 as libc::c_int;
    (**stream).progress = -ti_sma_start(options);
    (**stream).period = period;
    (**stream).per = 1.0f64 / period as libc::c_double;
    (**stream).sum = 0.0f64;
    (**stream).buffer_idx = -(1 as libc::c_int);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ti_sma_stream_run(
    mut stream_in: *mut ti_stream,
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut stream: *mut ti_stream_sma = stream_in as *mut ti_stream_sma;
    let mut progress: libc::c_int = (*stream).progress;
    let mut real: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut period: libc::c_int = (*stream).period;
    let mut sma: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut per: libc::c_double = (*stream).per;
    let mut sum: libc::c_double = (*stream).sum;
    let mut buffer_idx: libc::c_int = (*stream).buffer_idx;
    let mut buffer: *mut libc::c_double = ((*stream).buffer).as_mut_ptr();
    progress == -period + 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while progress < 1 as libc::c_int && i < size {
        buffer_idx += 1;
        buffer_idx;
        let ref mut fresh2 = *buffer.offset(buffer_idx as isize);
        *fresh2 = *real.offset(i as isize) * per;
        sum += *fresh2;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    if i > 0 as libc::c_int && progress == 1 as libc::c_int {
        let fresh3 = sma;
        sma = sma.offset(1);
        *fresh3 = sum;
    }
    while i < size {
        buffer_idx += 1;
        buffer_idx;
        if buffer_idx == period {
            buffer_idx = 0 as libc::c_int;
        }
        sum -= *buffer.offset(buffer_idx as isize);
        let ref mut fresh4 = *buffer.offset(buffer_idx as isize);
        *fresh4 = *real.offset(i as isize) * per;
        sum += *fresh4;
        let fresh5 = sma;
        sma = sma.offset(1);
        *fresh5 = sum;
        i += 1;
        i;
        progress += 1;
        progress;
    }
    (*stream).progress = progress;
    (*stream).sum = sum;
    (*stream).buffer_idx = buffer_idx;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ti_sma_stream_free(mut stream: *mut ti_stream) {
    free(stream as *mut libc::c_void);
}
