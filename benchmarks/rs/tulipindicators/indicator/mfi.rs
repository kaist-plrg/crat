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
pub unsafe extern "C" fn ti_mfi_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int;
}
pub unsafe extern "C" fn ti_mfi(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut high: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut low: *const libc::c_double = *inputs.offset(1 as libc::c_int as isize);
    let mut close: *const libc::c_double = *inputs.offset(2 as libc::c_int as isize);
    let mut volume: *const libc::c_double = *inputs.offset(3 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_mfi_start(options) {
        return 0 as libc::c_int;
    }
    let mut output: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut ytyp: libc::c_double = (*high.offset(0 as libc::c_int as isize)
        + *low.offset(0 as libc::c_int as isize)
        + *close.offset(0 as libc::c_int as isize)) * (1.0f64 / 3.0f64);
    let mut i: libc::c_int = 0;
    let mut up: *mut ti_buffer = ti_buffer_new(period);
    let mut down: *mut ti_buffer = ti_buffer_new(period);
    i = 1 as libc::c_int;
    while i < size {
        let typ: libc::c_double = (*high.offset(i as isize) + *low.offset(i as isize)
            + *close.offset(i as isize)) * (1.0f64 / 3.0f64);
        let bar: libc::c_double = typ * *volume.offset(i as isize);
        if typ > ytyp {
            if (*up).pushes >= (*up).size {
                (*up).sum -= *((*up).vals).as_mut_ptr().offset((*up).index as isize);
            }
            (*up).sum += bar;
            *((*up).vals).as_mut_ptr().offset((*up).index as isize) = bar;
            (*up).pushes += 1 as libc::c_int;
            (*up).index = (*up).index + 1 as libc::c_int;
            if (*up).index >= (*up).size {
                (*up).index = 0 as libc::c_int;
            }
            if (*down).pushes >= (*down).size {
                (*down).sum
                    -= *((*down).vals).as_mut_ptr().offset((*down).index as isize);
            }
            (*down).sum += 0.0f64;
            *((*down).vals).as_mut_ptr().offset((*down).index as isize) = 0.0f64;
            (*down).pushes += 1 as libc::c_int;
            (*down).index = (*down).index + 1 as libc::c_int;
            if (*down).index >= (*down).size {
                (*down).index = 0 as libc::c_int;
            }
        } else if typ < ytyp {
            if (*down).pushes >= (*down).size {
                (*down).sum
                    -= *((*down).vals).as_mut_ptr().offset((*down).index as isize);
            }
            (*down).sum += bar;
            *((*down).vals).as_mut_ptr().offset((*down).index as isize) = bar;
            (*down).pushes += 1 as libc::c_int;
            (*down).index = (*down).index + 1 as libc::c_int;
            if (*down).index >= (*down).size {
                (*down).index = 0 as libc::c_int;
            }
            if (*up).pushes >= (*up).size {
                (*up).sum -= *((*up).vals).as_mut_ptr().offset((*up).index as isize);
            }
            (*up).sum += 0.0f64;
            *((*up).vals).as_mut_ptr().offset((*up).index as isize) = 0.0f64;
            (*up).pushes += 1 as libc::c_int;
            (*up).index = (*up).index + 1 as libc::c_int;
            if (*up).index >= (*up).size {
                (*up).index = 0 as libc::c_int;
            }
        } else {
            if (*up).pushes >= (*up).size {
                (*up).sum -= *((*up).vals).as_mut_ptr().offset((*up).index as isize);
            }
            (*up).sum += 0.0f64;
            *((*up).vals).as_mut_ptr().offset((*up).index as isize) = 0.0f64;
            (*up).pushes += 1 as libc::c_int;
            (*up).index = (*up).index + 1 as libc::c_int;
            if (*up).index >= (*up).size {
                (*up).index = 0 as libc::c_int;
            }
            if (*down).pushes >= (*down).size {
                (*down).sum
                    -= *((*down).vals).as_mut_ptr().offset((*down).index as isize);
            }
            (*down).sum += 0.0f64;
            *((*down).vals).as_mut_ptr().offset((*down).index as isize) = 0.0f64;
            (*down).pushes += 1 as libc::c_int;
            (*down).index = (*down).index + 1 as libc::c_int;
            if (*down).index >= (*down).size {
                (*down).index = 0 as libc::c_int;
            }
        }
        ytyp = typ;
        if i >= period {
            let fresh0 = output;
            output = output.offset(1);
            *fresh0 = (*up).sum / ((*up).sum + (*down).sum) * 100.0f64;
        }
        i += 1;
        i;
    }
    ti_buffer_free(up);
    ti_buffer_free(down);
    if output.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_mfi_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"output - outputs[0] == size - ti_mfi_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/mfi.c\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_mfi(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
