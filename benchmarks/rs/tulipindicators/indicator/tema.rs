use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_tema_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    return (period - 1 as libc::c_int) * 3 as libc::c_int;
}
pub unsafe extern "C" fn ti_tema(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_tema_start(options) {
        return 0 as libc::c_int;
    }
    let per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let per1: libc::c_double = 1.0f64 - per;
    let mut ema: libc::c_double = *input.offset(0 as libc::c_int as isize);
    let mut ema2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut ema3: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        ema = ema * per1 + *input.offset(i as isize) * per;
        if i == period - 1 as libc::c_int {
            ema2 = ema;
        }
        if i >= period - 1 as libc::c_int {
            ema2 = ema2 * per1 + ema * per;
            if i == (period - 1 as libc::c_int) * 2 as libc::c_int {
                ema3 = ema2;
            }
            if i >= (period - 1 as libc::c_int) * 2 as libc::c_int {
                ema3 = ema3 * per1 + ema2 * per;
                if i >= (period - 1 as libc::c_int) * 3 as libc::c_int {
                    *output = 3 as libc::c_int as libc::c_double * ema
                        - 3 as libc::c_int as libc::c_double * ema2 + ema3;
                    output = output.offset(1);
                    output;
                }
            }
        }
        i += 1;
        i;
    }
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_tema_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_tema_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/tema.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"int ti_tema(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
