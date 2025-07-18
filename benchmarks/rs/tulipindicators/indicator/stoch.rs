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
pub unsafe extern "C" fn ti_stoch_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    let kperiod: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let kslow: libc::c_int = *options.offset(1 as libc::c_int as isize) as libc::c_int;
    let dperiod: libc::c_int = *options.offset(2 as libc::c_int as isize) as libc::c_int;
    return kperiod + kslow + dperiod - 3 as libc::c_int;
}
pub unsafe extern "C" fn ti_stoch(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let kperiod: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    let kslow: libc::c_int = *options.offset(1 as libc::c_int as isize) as libc::c_int;
    let dperiod: libc::c_int = *options.offset(2 as libc::c_int as isize) as libc::c_int;
    let kper: libc::c_double = 1.0f64 / kslow as libc::c_double;
    let dper: libc::c_double = 1.0f64 / dperiod as libc::c_double;
    let mut stoch: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut stoch_ma: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    if kperiod < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if kslow < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if dperiod < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_stoch_start(options) {
        return 0 as libc::c_int;
    }
    let mut trail: libc::c_int = 0 as libc::c_int;
    let mut maxi: libc::c_int = -(1 as libc::c_int);
    let mut mini: libc::c_int = -(1 as libc::c_int);
    let mut max: libc::c_double = *high.offset(0 as libc::c_int as isize);
    let mut min: libc::c_double = *low.offset(0 as libc::c_int as isize);
    let mut bar: libc::c_double = 0.;
    let mut k_sum: *mut ti_buffer = ti_buffer_new(kslow);
    let mut d_sum: *mut ti_buffer = ti_buffer_new(dperiod);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        if i >= kperiod {
            trail += 1;
            trail;
        }
        bar = *high.offset(i as isize);
        if maxi < trail {
            maxi = trail;
            max = *high.offset(maxi as isize);
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = *high.offset(j as isize);
                if bar >= max {
                    max = bar;
                    maxi = j;
                }
            }
        } else if bar >= max {
            maxi = i;
            max = bar;
        }
        bar = *low.offset(i as isize);
        if mini < trail {
            mini = trail;
            min = *low.offset(mini as isize);
            j = trail;
            loop {
                j += 1;
                if !(j <= i) {
                    break;
                }
                bar = *low.offset(j as isize);
                if bar <= min {
                    min = bar;
                    mini = j;
                }
            }
        } else if bar <= min {
            mini = i;
            min = bar;
        }
        let kdiff: libc::c_double = max - min;
        let kfast: libc::c_double = if kdiff == 0.0f64 {
            0.0f64
        } else {
            100 as libc::c_int as libc::c_double
                * ((*close.offset(i as isize) - min) / kdiff)
        };
        if (*k_sum).pushes >= (*k_sum).size {
            (*k_sum).sum
                -= *((*k_sum).vals).as_mut_ptr().offset((*k_sum).index as isize);
        }
        (*k_sum).sum += kfast;
        *((*k_sum).vals).as_mut_ptr().offset((*k_sum).index as isize) = kfast;
        (*k_sum).pushes += 1 as libc::c_int;
        (*k_sum).index = (*k_sum).index + 1 as libc::c_int;
        if (*k_sum).index >= (*k_sum).size {
            (*k_sum).index = 0 as libc::c_int;
        }
        if i >= kperiod - 1 as libc::c_int + kslow - 1 as libc::c_int {
            let k: libc::c_double = (*k_sum).sum * kper;
            if (*d_sum).pushes >= (*d_sum).size {
                (*d_sum).sum
                    -= *((*d_sum).vals).as_mut_ptr().offset((*d_sum).index as isize);
            }
            (*d_sum).sum += k;
            *((*d_sum).vals).as_mut_ptr().offset((*d_sum).index as isize) = k;
            (*d_sum).pushes += 1 as libc::c_int;
            (*d_sum).index = (*d_sum).index + 1 as libc::c_int;
            if (*d_sum).index >= (*d_sum).size {
                (*d_sum).index = 0 as libc::c_int;
            }
            if i
                >= kperiod - 1 as libc::c_int + kslow - 1 as libc::c_int + dperiod
                    - 1 as libc::c_int
            {
                let fresh0 = stoch;
                stoch = stoch.offset(1);
                *fresh0 = k;
                let fresh1 = stoch_ma;
                stoch_ma = stoch_ma.offset(1);
                *fresh1 = (*d_sum).sum * dper;
            }
        }
        i += 1;
        i;
    }
    ti_buffer_free(k_sum);
    ti_buffer_free(d_sum);
    if stoch.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_stoch_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"stoch - outputs[0] == size - ti_stoch_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/stoch.c\0" as *const u8 as *const libc::c_char,
            125 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"int ti_stoch(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    if stoch_ma.offset_from(*outputs.offset(1 as libc::c_int as isize)) as libc::c_long
        == (size - ti_stoch_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"stoch_ma - outputs[1] == size - ti_stoch_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/stoch.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 74],
                &[libc::c_char; 74],
            >(
                b"int ti_stoch(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
