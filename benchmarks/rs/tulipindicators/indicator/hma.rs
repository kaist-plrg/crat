use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
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
pub unsafe extern "C" fn ti_hma_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let periodsqrt: libc::c_int = sqrt(period as libc::c_double) as libc::c_int;
    return period + periodsqrt - 2 as libc::c_int;
}
pub unsafe extern "C" fn ti_hma(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 3 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_hma_start(options) {
        return 0 as libc::c_int;
    }
    let period2: libc::c_int = period / 2 as libc::c_int;
    let periodsqrt: libc::c_int = sqrt(period as libc::c_double) as libc::c_int;
    let weights: libc::c_double = (period * (period + 1 as libc::c_int)
        / 2 as libc::c_int) as libc::c_double;
    let weights2: libc::c_double = (period2 * (period2 + 1 as libc::c_int)
        / 2 as libc::c_int) as libc::c_double;
    let weightssqrt: libc::c_double = (periodsqrt * (periodsqrt + 1 as libc::c_int)
        / 2 as libc::c_int) as libc::c_double;
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut weight_sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sum2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut weight_sum2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sumsqrt: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut weight_sumsqrt: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < period - 1 as libc::c_int {
        weight_sum
            += *input.offset(i as isize) * (i + 1 as libc::c_int) as libc::c_double;
        sum += *input.offset(i as isize);
        if i >= period - period2 {
            weight_sum2
                += *input.offset(i as isize)
                    * (i + 1 as libc::c_int - (period - period2)) as libc::c_double;
            sum2 += *input.offset(i as isize);
        }
        i += 1;
        i;
    }
    let mut buff: *mut ti_buffer = ti_buffer_new(periodsqrt);
    i = period - 1 as libc::c_int;
    while i < size {
        weight_sum += *input.offset(i as isize) * period as libc::c_double;
        sum += *input.offset(i as isize);
        weight_sum2 += *input.offset(i as isize) * period2 as libc::c_double;
        sum2 += *input.offset(i as isize);
        let wma: libc::c_double = weight_sum / weights;
        let wma2: libc::c_double = weight_sum2 / weights2;
        let diff: libc::c_double = 2 as libc::c_int as libc::c_double * wma2 - wma;
        weight_sumsqrt += diff * periodsqrt as libc::c_double;
        sumsqrt += diff;
        *((*buff).vals).as_mut_ptr().offset((*buff).index as isize) = diff;
        (*buff).index = (*buff).index + 1 as libc::c_int;
        if (*buff).index >= (*buff).size {
            (*buff).index = 0 as libc::c_int;
        }
        if i >= period - 1 as libc::c_int + (periodsqrt - 1 as libc::c_int) {
            let fresh0 = output;
            output = output.offset(1);
            *fresh0 = weight_sumsqrt / weightssqrt;
            weight_sumsqrt -= sumsqrt;
            sumsqrt
                -= *((*buff).vals)
                    .as_mut_ptr()
                    .offset(
                        (((*buff).index + (*buff).size - 1 as libc::c_int
                            + 1 as libc::c_int) % (*buff).size) as isize,
                    );
        } else {
            weight_sumsqrt -= sumsqrt;
        }
        weight_sum -= sum;
        sum -= *input.offset((i - period + 1 as libc::c_int) as isize);
        weight_sum2 -= sum2;
        sum2 -= *input.offset((i - period2 + 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    ti_buffer_free(buff);
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_hma_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_hma_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/hma.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_hma(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
