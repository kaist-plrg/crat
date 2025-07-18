use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_ppo_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn ti_ppo(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut ppo: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let short_period: libc::c_int = *options.offset(0 as libc::c_int as isize)
        as libc::c_int;
    let long_period: libc::c_int = *options.offset(1 as libc::c_int as isize)
        as libc::c_int;
    if short_period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if long_period < 2 as libc::c_int {
        return 1 as libc::c_int;
    }
    if long_period < short_period {
        return 1 as libc::c_int;
    }
    if size <= ti_ppo_start(options) {
        return 0 as libc::c_int;
    }
    let mut short_per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (short_period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let mut long_per: libc::c_double = 2 as libc::c_int as libc::c_double
        / (long_period as libc::c_double + 1 as libc::c_int as libc::c_double);
    let mut short_ema: libc::c_double = *input.offset(0 as libc::c_int as isize);
    let mut long_ema: libc::c_double = *input.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < size {
        short_ema = (*input.offset(i as isize) - short_ema) * short_per + short_ema;
        long_ema = (*input.offset(i as isize) - long_ema) * long_per + long_ema;
        let out: libc::c_double = 100.0f64 * (short_ema - long_ema) / long_ema;
        let fresh0 = ppo;
        ppo = ppo.offset(1);
        *fresh0 = out;
        i += 1;
        i;
    }
    if ppo.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_ppo_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"ppo - outputs[0] == size - ti_ppo_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/ppo.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_ppo(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
