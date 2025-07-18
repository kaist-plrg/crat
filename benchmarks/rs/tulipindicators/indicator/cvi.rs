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
pub unsafe extern "C" fn ti_cvi_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let n: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    return n * 2 as libc::c_int - 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_cvi(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_cvi_start(options) {
        return 0 as libc::c_int;
    }
    let per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let mut lag: *mut ti_buffer = ti_buffer_new(period);
    let mut val: libc::c_double = *high.offset(0 as libc::c_int as isize)
        - *low.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < period * 2 as libc::c_int - 1 as libc::c_int {
        val = (*high.offset(i as isize) - *low.offset(i as isize) - val) * per + val;
        *((*lag).vals).as_mut_ptr().offset((*lag).index as isize) = val;
        (*lag).index = (*lag).index + 1 as libc::c_int;
        if (*lag).index >= (*lag).size {
            (*lag).index = 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    i = period * 2 as libc::c_int - 1 as libc::c_int;
    while i < size {
        val = (*high.offset(i as isize) - *low.offset(i as isize) - val) * per + val;
        let old: libc::c_double = *((*lag).vals)
            .as_mut_ptr()
            .offset((*lag).index as isize);
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = 100.0f64 * (val - old) / old;
        *((*lag).vals).as_mut_ptr().offset((*lag).index as isize) = val;
        (*lag).index = (*lag).index + 1 as libc::c_int;
        if (*lag).index >= (*lag).size {
            (*lag).index = 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    ti_buffer_free(lag);
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_cvi_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_cvi_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/cvi.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_cvi(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
