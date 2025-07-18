use ::libc;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_histogram {
    pub n: size_t,
    pub range: *mut libc::c_double,
    pub bin: *mut libc::c_double,
}
pub unsafe extern "C" fn gsl_histogram_max_val(
    mut h: *const gsl_histogram,
) -> libc::c_double {
    let n: size_t = (*h).n;
    let mut i: size_t = 0;
    let mut max: libc::c_double = *((*h).bin).offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int as size_t;
    while i < n {
        if *((*h).bin).offset(i as isize) > max {
            max = *((*h).bin).offset(i as isize);
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_histogram_max_bin(mut h: *const gsl_histogram) -> size_t {
    let mut i: size_t = 0;
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut max: libc::c_double = *((*h).bin).offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int as size_t;
    while i < (*h).n {
        if *((*h).bin).offset(i as isize) > max {
            max = *((*h).bin).offset(i as isize);
            imax = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imax;
}
pub unsafe extern "C" fn gsl_histogram_min_val(
    mut h: *const gsl_histogram,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut min: libc::c_double = *((*h).bin).offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int as size_t;
    while i < (*h).n {
        if *((*h).bin).offset(i as isize) < min {
            min = *((*h).bin).offset(i as isize);
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_histogram_min_bin(mut h: *const gsl_histogram) -> size_t {
    let mut i: size_t = 0;
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut min: libc::c_double = *((*h).bin).offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int as size_t;
    while i < (*h).n {
        if *((*h).bin).offset(i as isize) < min {
            min = *((*h).bin).offset(i as isize);
            imin = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return imin;
}
