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
pub unsafe extern "C" fn ti_cci_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    return (period - 1 as libc::c_int) * 2 as libc::c_int;
}
pub unsafe extern "C" fn ti_cci(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let scale: libc::c_double = 1.0f64 / period as libc::c_double;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_cci_start(options) {
        return 0 as libc::c_int;
    }
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut sum: *mut ti_buffer = ti_buffer_new(period);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        let today: libc::c_double = (*high.offset(i as isize) + *low.offset(i as isize)
            + *close.offset(i as isize)) * (1.0f64 / 3.0f64);
        if (*sum).pushes >= (*sum).size {
            (*sum).sum -= *((*sum).vals).as_mut_ptr().offset((*sum).index as isize);
        }
        (*sum).sum += today;
        *((*sum).vals).as_mut_ptr().offset((*sum).index as isize) = today;
        (*sum).pushes += 1 as libc::c_int;
        (*sum).index = (*sum).index + 1 as libc::c_int;
        if (*sum).index >= (*sum).size {
            (*sum).index = 0 as libc::c_int;
        }
        let avg: libc::c_double = (*sum).sum * scale;
        if i >= period * 2 as libc::c_int - 2 as libc::c_int {
            let mut acc: libc::c_double = 0 as libc::c_int as libc::c_double;
            j = 0 as libc::c_int;
            while j < period {
                acc += fabs(avg - *((*sum).vals).as_mut_ptr().offset(j as isize));
                j += 1;
                j;
            }
            let mut cci: libc::c_double = acc * scale;
            cci *= 0.015f64;
            cci = (today - avg) / cci;
            let fresh0 = output;
            output = output.offset(1);
            *fresh0 = cci;
        }
        i += 1;
        i;
    }
    ti_buffer_free(sum);
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_cci_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_cci_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/cci.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_cci(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
