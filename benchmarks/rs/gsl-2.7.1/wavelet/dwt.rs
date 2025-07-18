use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
}
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_int;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
pub type gsl_wavelet_direction = libc::c_int;
pub const gsl_wavelet_backward: gsl_wavelet_direction = -1;
pub const gsl_wavelet_forward: gsl_wavelet_direction = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_wavelet_type {
    pub name: *const libc::c_char,
    pub init: Option::<
        unsafe extern "C" fn(
            *mut *const libc::c_double,
            *mut *const libc::c_double,
            *mut *const libc::c_double,
            *mut *const libc::c_double,
            *mut size_t,
            *mut size_t,
            size_t,
        ) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_wavelet {
    pub type_0: *const gsl_wavelet_type,
    pub h1: *const libc::c_double,
    pub g1: *const libc::c_double,
    pub h2: *const libc::c_double,
    pub g2: *const libc::c_double,
    pub nc: size_t,
    pub offset: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_wavelet_workspace {
    pub scratch: *mut libc::c_double,
    pub n: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block = gsl_block_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
unsafe extern "C" fn binary_logn(n: size_t) -> libc::c_int {
    let mut ntest: size_t = 0;
    let mut logn: size_t = 0 as libc::c_int as size_t;
    let mut k: size_t = 1 as libc::c_int as size_t;
    while k < n {
        k = (k as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        logn = logn.wrapping_add(1);
        logn;
    }
    ntest = (1 as libc::c_int as size_t) << logn;
    if n != ntest {
        return -(1 as libc::c_int);
    }
    return logn as libc::c_int;
}
unsafe extern "C" fn dwt_step(
    mut w: *const gsl_wavelet,
    mut a: *mut libc::c_double,
    mut stride: size_t,
    mut n: size_t,
    mut dir: gsl_wavelet_direction,
    mut work: *mut gsl_wavelet_workspace,
) {
    let mut ai: libc::c_double = 0.;
    let mut ai1: libc::c_double = 0.;
    let mut i: size_t = 0;
    let mut ii: size_t = 0;
    let mut jf: size_t = 0;
    let mut k: size_t = 0;
    let mut n1: size_t = 0;
    let mut ni: size_t = 0;
    let mut nh: size_t = 0;
    let mut nmod: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*work).n {
        *((*work).scratch).offset(i as isize) = 0.0f64;
        i = i.wrapping_add(1);
        i;
    }
    nmod = ((*w).nc).wrapping_mul(n);
    nmod = (nmod as libc::c_ulong).wrapping_sub((*w).offset) as size_t as size_t;
    n1 = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    nh = n >> 1 as libc::c_int;
    if dir as libc::c_int == gsl_wavelet_forward as libc::c_int {
        ii = 0 as libc::c_int as size_t;
        i = 0 as libc::c_int as size_t;
        while i < n {
            let mut h: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut g: libc::c_double = 0 as libc::c_int as libc::c_double;
            ni = i.wrapping_add(nmod);
            k = 0 as libc::c_int as size_t;
            while k < (*w).nc {
                jf = n1 & ni.wrapping_add(k);
                h
                    += *((*w).h1).offset(k as isize)
                        * *a.offset(stride.wrapping_mul(jf) as isize);
                g
                    += *((*w).g1).offset(k as isize)
                        * *a.offset(stride.wrapping_mul(jf) as isize);
                k = k.wrapping_add(1);
                k;
            }
            *((*work).scratch).offset(ii as isize) += h;
            *((*work).scratch).offset(ii.wrapping_add(nh) as isize) += g;
            i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            ii = ii.wrapping_add(1);
            ii;
        }
    } else {
        ii = 0 as libc::c_int as size_t;
        i = 0 as libc::c_int as size_t;
        while i < n {
            ai = *a.offset(stride.wrapping_mul(ii) as isize);
            ai1 = *a.offset(stride.wrapping_mul(ii.wrapping_add(nh)) as isize);
            ni = i.wrapping_add(nmod);
            k = 0 as libc::c_int as size_t;
            while k < (*w).nc {
                jf = n1 & ni.wrapping_add(k);
                *((*work).scratch).offset(jf as isize)
                    += *((*w).h2).offset(k as isize) * ai
                        + *((*w).g2).offset(k as isize) * ai1;
                k = k.wrapping_add(1);
                k;
            }
            i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            ii = ii.wrapping_add(1);
            ii;
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        *a
            .offset(
                stride.wrapping_mul(i) as isize,
            ) = *((*work).scratch).offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_wavelet_transform(
    mut w: *const gsl_wavelet,
    mut data: *mut libc::c_double,
    mut stride: size_t,
    mut n: size_t,
    mut dir: gsl_wavelet_direction,
    mut work: *mut gsl_wavelet_workspace,
) -> libc::c_int {
    let mut i: size_t = 0;
    if (*work).n < n {
        gsl_error(
            b"not enough workspace provided\0" as *const u8 as *const libc::c_char,
            b"dwt.c\0" as *const u8 as *const libc::c_char,
            128 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if binary_logn(n) == -(1 as libc::c_int) {
        gsl_error(
            b"n is not a power of 2\0" as *const u8 as *const libc::c_char,
            b"dwt.c\0" as *const u8 as *const libc::c_char,
            133 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if n < 2 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    if dir as libc::c_int == gsl_wavelet_forward as libc::c_int {
        i = n;
        while i >= 2 as libc::c_int as libc::c_ulong {
            dwt_step(w, data, stride, i, dir, work);
            i >>= 1 as libc::c_int;
        }
    } else {
        i = 2 as libc::c_int as size_t;
        while i <= n {
            dwt_step(w, data, stride, i, dir, work);
            i <<= 1 as libc::c_int;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_wavelet_transform_forward(
    mut w: *const gsl_wavelet,
    mut data: *mut libc::c_double,
    mut stride: size_t,
    mut n: size_t,
    mut work: *mut gsl_wavelet_workspace,
) -> libc::c_int {
    return gsl_wavelet_transform(w, data, stride, n, gsl_wavelet_forward, work);
}
pub unsafe extern "C" fn gsl_wavelet_transform_inverse(
    mut w: *const gsl_wavelet,
    mut data: *mut libc::c_double,
    mut stride: size_t,
    mut n: size_t,
    mut work: *mut gsl_wavelet_workspace,
) -> libc::c_int {
    return gsl_wavelet_transform(w, data, stride, n, gsl_wavelet_backward, work);
}
pub unsafe extern "C" fn gsl_wavelet2d_transform(
    mut w: *const gsl_wavelet,
    mut data: *mut libc::c_double,
    mut tda: size_t,
    mut size1: size_t,
    mut size2: size_t,
    mut dir: gsl_wavelet_direction,
    mut work: *mut gsl_wavelet_workspace,
) -> libc::c_int {
    let mut i: size_t = 0;
    if size1 != size2 {
        gsl_error(
            b"2d dwt works only with square matrix\0" as *const u8
                as *const libc::c_char,
            b"dwt.c\0" as *const u8 as *const libc::c_char,
            196 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if (*work).n < size1 {
        gsl_error(
            b"not enough workspace provided\0" as *const u8 as *const libc::c_char,
            b"dwt.c\0" as *const u8 as *const libc::c_char,
            201 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if binary_logn(size1) == -(1 as libc::c_int) {
        gsl_error(
            b"n is not a power of 2\0" as *const u8 as *const libc::c_char,
            b"dwt.c\0" as *const u8 as *const libc::c_char,
            206 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if size1 < 2 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    if dir as libc::c_int == gsl_wavelet_forward as libc::c_int {
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            gsl_wavelet_transform(
                w,
                &mut *data.offset(tda.wrapping_mul(i) as isize),
                1 as libc::c_int as size_t,
                size1,
                dir,
                work,
            );
            i = i.wrapping_add(1);
            i;
        }
        i = 0 as libc::c_int as size_t;
        while i < size2 {
            gsl_wavelet_transform(
                w,
                &mut *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
                    ),
                tda,
                size2,
                dir,
                work,
            );
            i = i.wrapping_add(1);
            i;
        }
    } else {
        i = 0 as libc::c_int as size_t;
        while i < size2 {
            gsl_wavelet_transform(
                w,
                &mut *data
                    .offset(
                        (1 as libc::c_int as libc::c_ulong).wrapping_mul(i) as isize,
                    ),
                tda,
                size2,
                dir,
                work,
            );
            i = i.wrapping_add(1);
            i;
        }
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            gsl_wavelet_transform(
                w,
                &mut *data.offset(tda.wrapping_mul(i) as isize),
                1 as libc::c_int as size_t,
                size1,
                dir,
                work,
            );
            i = i.wrapping_add(1);
            i;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_wavelet2d_nstransform(
    mut w: *const gsl_wavelet,
    mut data: *mut libc::c_double,
    mut tda: size_t,
    mut size1: size_t,
    mut size2: size_t,
    mut dir: gsl_wavelet_direction,
    mut work: *mut gsl_wavelet_workspace,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if size1 != size2 {
        gsl_error(
            b"2d dwt works only with square matrix\0" as *const u8
                as *const libc::c_char,
            b"dwt.c\0" as *const u8 as *const libc::c_char,
            250 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if (*work).n < size1 {
        gsl_error(
            b"not enough workspace provided\0" as *const u8 as *const libc::c_char,
            b"dwt.c\0" as *const u8 as *const libc::c_char,
            255 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if binary_logn(size1) == -(1 as libc::c_int) {
        gsl_error(
            b"n is not a power of 2\0" as *const u8 as *const libc::c_char,
            b"dwt.c\0" as *const u8 as *const libc::c_char,
            260 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if size1 < 2 as libc::c_int as libc::c_ulong {
        return GSL_SUCCESS as libc::c_int;
    }
    if dir as libc::c_int == gsl_wavelet_forward as libc::c_int {
        i = size1;
        while i >= 2 as libc::c_int as libc::c_ulong {
            j = 0 as libc::c_int as size_t;
            while j < i {
                dwt_step(
                    w,
                    &mut *data.offset(tda.wrapping_mul(j) as isize),
                    1 as libc::c_int as size_t,
                    i,
                    dir,
                    work,
                );
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < i {
                dwt_step(
                    w,
                    &mut *data
                        .offset(
                            (1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize,
                        ),
                    tda,
                    i,
                    dir,
                    work,
                );
                j = j.wrapping_add(1);
                j;
            }
            i >>= 1 as libc::c_int;
        }
    } else {
        i = 2 as libc::c_int as size_t;
        while i <= size1 {
            j = 0 as libc::c_int as size_t;
            while j < i {
                dwt_step(
                    w,
                    &mut *data
                        .offset(
                            (1 as libc::c_int as libc::c_ulong).wrapping_mul(j) as isize,
                        ),
                    tda,
                    i,
                    dir,
                    work,
                );
                j = j.wrapping_add(1);
                j;
            }
            j = 0 as libc::c_int as size_t;
            while j < i {
                dwt_step(
                    w,
                    &mut *data.offset(tda.wrapping_mul(j) as isize),
                    1 as libc::c_int as size_t,
                    i,
                    dir,
                    work,
                );
                j = j.wrapping_add(1);
                j;
            }
            i <<= 1 as libc::c_int;
        }
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_wavelet2d_transform_forward(
    mut w: *const gsl_wavelet,
    mut data: *mut libc::c_double,
    mut tda: size_t,
    mut size1: size_t,
    mut size2: size_t,
    mut work: *mut gsl_wavelet_workspace,
) -> libc::c_int {
    return gsl_wavelet2d_transform(
        w,
        data,
        tda,
        size1,
        size2,
        gsl_wavelet_forward,
        work,
    );
}
pub unsafe extern "C" fn gsl_wavelet2d_transform_inverse(
    mut w: *const gsl_wavelet,
    mut data: *mut libc::c_double,
    mut tda: size_t,
    mut size1: size_t,
    mut size2: size_t,
    mut work: *mut gsl_wavelet_workspace,
) -> libc::c_int {
    return gsl_wavelet2d_transform(
        w,
        data,
        tda,
        size1,
        size2,
        gsl_wavelet_backward,
        work,
    );
}
pub unsafe extern "C" fn gsl_wavelet2d_nstransform_forward(
    mut w: *const gsl_wavelet,
    mut data: *mut libc::c_double,
    mut tda: size_t,
    mut size1: size_t,
    mut size2: size_t,
    mut work: *mut gsl_wavelet_workspace,
) -> libc::c_int {
    return gsl_wavelet2d_nstransform(
        w,
        data,
        tda,
        size1,
        size2,
        gsl_wavelet_forward,
        work,
    );
}
pub unsafe extern "C" fn gsl_wavelet2d_nstransform_inverse(
    mut w: *const gsl_wavelet,
    mut data: *mut libc::c_double,
    mut tda: size_t,
    mut size1: size_t,
    mut size2: size_t,
    mut work: *mut gsl_wavelet_workspace,
) -> libc::c_int {
    return gsl_wavelet2d_nstransform(
        w,
        data,
        tda,
        size1,
        size2,
        gsl_wavelet_backward,
        work,
    );
}
pub unsafe extern "C" fn gsl_wavelet2d_transform_matrix(
    mut w: *const gsl_wavelet,
    mut a: *mut gsl_matrix,
    mut dir: gsl_wavelet_direction,
    mut work: *mut gsl_wavelet_workspace,
) -> libc::c_int {
    return gsl_wavelet2d_transform(
        w,
        (*a).data,
        (*a).tda,
        (*a).size1,
        (*a).size2,
        dir,
        work,
    );
}
pub unsafe extern "C" fn gsl_wavelet2d_transform_matrix_forward(
    mut w: *const gsl_wavelet,
    mut a: *mut gsl_matrix,
    mut work: *mut gsl_wavelet_workspace,
) -> libc::c_int {
    return gsl_wavelet2d_transform(
        w,
        (*a).data,
        (*a).tda,
        (*a).size1,
        (*a).size2,
        gsl_wavelet_forward,
        work,
    );
}
pub unsafe extern "C" fn gsl_wavelet2d_transform_matrix_inverse(
    mut w: *const gsl_wavelet,
    mut a: *mut gsl_matrix,
    mut work: *mut gsl_wavelet_workspace,
) -> libc::c_int {
    return gsl_wavelet2d_transform(
        w,
        (*a).data,
        (*a).tda,
        (*a).size1,
        (*a).size2,
        gsl_wavelet_backward,
        work,
    );
}
pub unsafe extern "C" fn gsl_wavelet2d_nstransform_matrix(
    mut w: *const gsl_wavelet,
    mut a: *mut gsl_matrix,
    mut dir: gsl_wavelet_direction,
    mut work: *mut gsl_wavelet_workspace,
) -> libc::c_int {
    return gsl_wavelet2d_nstransform(
        w,
        (*a).data,
        (*a).tda,
        (*a).size1,
        (*a).size2,
        dir,
        work,
    );
}
pub unsafe extern "C" fn gsl_wavelet2d_nstransform_matrix_forward(
    mut w: *const gsl_wavelet,
    mut a: *mut gsl_matrix,
    mut work: *mut gsl_wavelet_workspace,
) -> libc::c_int {
    return gsl_wavelet2d_nstransform(
        w,
        (*a).data,
        (*a).tda,
        (*a).size1,
        (*a).size2,
        gsl_wavelet_forward,
        work,
    );
}
pub unsafe extern "C" fn gsl_wavelet2d_nstransform_matrix_inverse(
    mut w: *const gsl_wavelet,
    mut a: *mut gsl_matrix,
    mut work: *mut gsl_wavelet_workspace,
) -> libc::c_int {
    return gsl_wavelet2d_nstransform(
        w,
        (*a).data,
        (*a).tda,
        (*a).size1,
        (*a).size2,
        gsl_wavelet_backward,
        work,
    );
}
