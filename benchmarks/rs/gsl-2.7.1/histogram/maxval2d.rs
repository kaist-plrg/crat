use ::libc;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_histogram2d {
    pub nx: size_t,
    pub ny: size_t,
    pub xrange: *mut libc::c_double,
    pub yrange: *mut libc::c_double,
    pub bin: *mut libc::c_double,
}
pub unsafe extern "C" fn gsl_histogram2d_max_val(
    mut h: *const gsl_histogram2d,
) -> libc::c_double {
    let nx: size_t = (*h).nx;
    let ny: size_t = (*h).ny;
    let mut i: size_t = 0;
    let mut max: libc::c_double = *((*h).bin)
        .offset(
            (0 as libc::c_int as libc::c_ulong)
                .wrapping_mul(ny)
                .wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
        );
    i = 0 as libc::c_int as size_t;
    while i < nx.wrapping_mul(ny) {
        if *((*h).bin).offset(i as isize) > max {
            max = *((*h).bin).offset(i as isize);
        }
        i = i.wrapping_add(1);
        i;
    }
    return max;
}
pub unsafe extern "C" fn gsl_histogram2d_max_bin(
    mut h: *const gsl_histogram2d,
    mut imax_out: *mut size_t,
    mut jmax_out: *mut size_t,
) {
    let nx: size_t = (*h).nx;
    let ny: size_t = (*h).ny;
    let mut imax: size_t = 0 as libc::c_int as size_t;
    let mut jmax: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut max: libc::c_double = *((*h).bin)
        .offset(
            (0 as libc::c_int as libc::c_ulong)
                .wrapping_mul(ny)
                .wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
        );
    i = 0 as libc::c_int as size_t;
    while i < nx {
        j = 0 as libc::c_int as size_t;
        while j < ny {
            let mut x: libc::c_double = *((*h).bin)
                .offset(i.wrapping_mul(ny).wrapping_add(j) as isize);
            if x > max {
                max = x;
                imax = i;
                jmax = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imax_out = imax;
    *jmax_out = jmax;
}
pub unsafe extern "C" fn gsl_histogram2d_min_val(
    mut h: *const gsl_histogram2d,
) -> libc::c_double {
    let nx: size_t = (*h).nx;
    let ny: size_t = (*h).ny;
    let mut i: size_t = 0;
    let mut min: libc::c_double = *((*h).bin)
        .offset(
            (0 as libc::c_int as libc::c_ulong)
                .wrapping_mul(ny)
                .wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
        );
    i = 0 as libc::c_int as size_t;
    while i < nx.wrapping_mul(ny) {
        if *((*h).bin).offset(i as isize) < min {
            min = *((*h).bin).offset(i as isize);
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
pub unsafe extern "C" fn gsl_histogram2d_min_bin(
    mut h: *const gsl_histogram2d,
    mut imin_out: *mut size_t,
    mut jmin_out: *mut size_t,
) {
    let nx: size_t = (*h).nx;
    let ny: size_t = (*h).ny;
    let mut imin: size_t = 0 as libc::c_int as size_t;
    let mut jmin: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut min: libc::c_double = *((*h).bin)
        .offset(
            (0 as libc::c_int as libc::c_ulong)
                .wrapping_mul(ny)
                .wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
        );
    i = 0 as libc::c_int as size_t;
    while i < nx {
        j = 0 as libc::c_int as size_t;
        while j < ny {
            let mut x: libc::c_double = *((*h).bin)
                .offset(i.wrapping_mul(ny).wrapping_add(j) as isize);
            if x < min {
                min = x;
                imin = i;
                jmin = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    *imin_out = imin;
    *jmin_out = jmin;
}
