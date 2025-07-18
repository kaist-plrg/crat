use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_fcmp(
        x1: libc::c_double,
        x2: libc::c_double,
        epsilon: libc::c_double,
    ) -> libc::c_int;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_swap_elements(v: *mut gsl_vector, i: size_t, j: size_t) -> libc::c_int;
    fn gsl_vector_complex_swap_elements(
        v: *mut gsl_vector_complex,
        i: size_t,
        j: size_t,
    ) -> libc::c_int;
    fn gsl_matrix_complex_swap_columns(
        m: *mut gsl_matrix_complex,
        i: size_t,
        j: size_t,
    ) -> libc::c_int;
    fn gsl_matrix_swap_columns(m: *mut gsl_matrix, i: size_t, j: size_t) -> libc::c_int;
    fn gsl_complex_abs(z: gsl_complex) -> libc::c_double;
    fn gsl_complex_div_real(a: gsl_complex, x: libc::c_double) -> gsl_complex;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_complex {
    pub dat: [libc::c_double; 2],
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
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block_complex = gsl_block_complex_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_complex {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block_complex,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_complex {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block_complex,
    pub owner: libc::c_int,
}
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
pub type gsl_eigen_sort_t = libc::c_uint;
pub const GSL_EIGEN_SORT_ABS_DESC: gsl_eigen_sort_t = 3;
pub const GSL_EIGEN_SORT_ABS_ASC: gsl_eigen_sort_t = 2;
pub const GSL_EIGEN_SORT_VAL_DESC: gsl_eigen_sort_t = 1;
pub const GSL_EIGEN_SORT_VAL_ASC: gsl_eigen_sort_t = 0;
#[inline]
unsafe extern "C" fn gsl_vector_get(
    mut v: *const gsl_vector,
    i: size_t,
) -> libc::c_double {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_complex_get(
    mut v: *const gsl_vector_complex,
    i: size_t,
) -> gsl_complex {
    return *(&mut *((*v).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(i).wrapping_mul((*v).stride)
                as isize,
        ) as *mut libc::c_double as *mut gsl_complex);
}
unsafe extern "C" fn complex_less(
    mut a: gsl_complex,
    mut b: gsl_complex,
) -> libc::c_int {
    return if gsl_fcmp(
        a.dat[0 as libc::c_int as usize],
        b.dat[0 as libc::c_int as usize],
        2.2204460492503131e-16f64,
    ) == 0 as libc::c_int
    {
        (a.dat[1 as libc::c_int as usize] < b.dat[1 as libc::c_int as usize])
            as libc::c_int
    } else {
        (a.dat[0 as libc::c_int as usize] < b.dat[0 as libc::c_int as usize])
            as libc::c_int
    };
}
pub unsafe extern "C" fn gsl_eigen_symmv_sort(
    mut eval: *mut gsl_vector,
    mut evec: *mut gsl_matrix,
    mut sort_type: gsl_eigen_sort_t,
) -> libc::c_int {
    if (*evec).size1 != (*evec).size2 {
        gsl_error(
            b"eigenvector matrix must be square\0" as *const u8 as *const libc::c_char,
            b"sort.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*eval).size != (*evec).size1 {
        gsl_error(
            b"eigenvalues must match eigenvector matrix\0" as *const u8
                as *const libc::c_char,
            b"sort.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let N: size_t = (*eval).size;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            let mut j: size_t = 0;
            let mut k: size_t = i;
            let mut ek: libc::c_double = gsl_vector_get(eval, i);
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < N {
                let mut test: libc::c_int = 0;
                let ej: libc::c_double = gsl_vector_get(eval, j);
                match sort_type as libc::c_uint {
                    0 => {
                        test = (ej < ek) as libc::c_int;
                    }
                    1 => {
                        test = (ej > ek) as libc::c_int;
                    }
                    2 => {
                        test = (fabs(ej) < fabs(ek)) as libc::c_int;
                    }
                    3 => {
                        test = (fabs(ej) > fabs(ek)) as libc::c_int;
                    }
                    _ => {
                        gsl_error(
                            b"unrecognized sort type\0" as *const u8
                                as *const libc::c_char,
                            b"sort.c\0" as *const u8 as *const libc::c_char,
                            86 as libc::c_int,
                            GSL_EINVAL as libc::c_int,
                        );
                        return GSL_EINVAL as libc::c_int;
                    }
                }
                if test != 0 {
                    k = j;
                    ek = ej;
                }
                j = j.wrapping_add(1);
                j;
            }
            if k != i {
                gsl_vector_swap_elements(eval, i, k);
                gsl_matrix_swap_columns(evec, i, k);
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_eigen_hermv_sort(
    mut eval: *mut gsl_vector,
    mut evec: *mut gsl_matrix_complex,
    mut sort_type: gsl_eigen_sort_t,
) -> libc::c_int {
    if (*evec).size1 != (*evec).size2 {
        gsl_error(
            b"eigenvector matrix must be square\0" as *const u8 as *const libc::c_char,
            b"sort.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*eval).size != (*evec).size1 {
        gsl_error(
            b"eigenvalues must match eigenvector matrix\0" as *const u8
                as *const libc::c_char,
            b"sort.c\0" as *const u8 as *const libc::c_char,
            121 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let N: size_t = (*eval).size;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            let mut j: size_t = 0;
            let mut k: size_t = i;
            let mut ek: libc::c_double = gsl_vector_get(eval, i);
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < N {
                let mut test: libc::c_int = 0;
                let ej: libc::c_double = gsl_vector_get(eval, j);
                match sort_type as libc::c_uint {
                    0 => {
                        test = (ej < ek) as libc::c_int;
                    }
                    1 => {
                        test = (ej > ek) as libc::c_int;
                    }
                    2 => {
                        test = (fabs(ej) < fabs(ek)) as libc::c_int;
                    }
                    3 => {
                        test = (fabs(ej) > fabs(ek)) as libc::c_int;
                    }
                    _ => {
                        gsl_error(
                            b"unrecognized sort type\0" as *const u8
                                as *const libc::c_char,
                            b"sort.c\0" as *const u8 as *const libc::c_char,
                            156 as libc::c_int,
                            GSL_EINVAL as libc::c_int,
                        );
                        return GSL_EINVAL as libc::c_int;
                    }
                }
                if test != 0 {
                    k = j;
                    ek = ej;
                }
                j = j.wrapping_add(1);
                j;
            }
            if k != i {
                gsl_vector_swap_elements(eval, i, k);
                gsl_matrix_complex_swap_columns(evec, i, k);
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_eigen_nonsymmv_sort(
    mut eval: *mut gsl_vector_complex,
    mut evec: *mut gsl_matrix_complex,
    mut sort_type: gsl_eigen_sort_t,
) -> libc::c_int {
    if !evec.is_null() && (*evec).size1 != (*evec).size2 {
        gsl_error(
            b"eigenvector matrix must be square\0" as *const u8 as *const libc::c_char,
            b"sort.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if !evec.is_null() && (*eval).size != (*evec).size1 {
        gsl_error(
            b"eigenvalues must match eigenvector matrix\0" as *const u8
                as *const libc::c_char,
            b"sort.c\0" as *const u8 as *const libc::c_char,
            191 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let N: size_t = (*eval).size;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            let mut j: size_t = 0;
            let mut k: size_t = i;
            let mut ek: gsl_complex = gsl_vector_complex_get(eval, i);
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < N {
                let mut test: libc::c_int = 0;
                let ej: gsl_complex = gsl_vector_complex_get(eval, j);
                match sort_type as libc::c_uint {
                    2 => {
                        test = (gsl_complex_abs(ej) < gsl_complex_abs(ek))
                            as libc::c_int;
                    }
                    3 => {
                        test = (gsl_complex_abs(ej) > gsl_complex_abs(ek))
                            as libc::c_int;
                    }
                    0 => {
                        test = complex_less(ej, ek);
                    }
                    1 => {
                        test = complex_less(ek, ej);
                    }
                    _ => {
                        gsl_error(
                            b"invalid sort type\0" as *const u8 as *const libc::c_char,
                            b"sort.c\0" as *const u8 as *const libc::c_char,
                            226 as libc::c_int,
                            GSL_EINVAL as libc::c_int,
                        );
                        return GSL_EINVAL as libc::c_int;
                    }
                }
                if test != 0 {
                    k = j;
                    ek = ej;
                }
                j = j.wrapping_add(1);
                j;
            }
            if k != i {
                gsl_vector_complex_swap_elements(eval, i, k);
                if !evec.is_null() {
                    gsl_matrix_complex_swap_columns(evec, i, k);
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_eigen_gensymmv_sort(
    mut eval: *mut gsl_vector,
    mut evec: *mut gsl_matrix,
    mut sort_type: gsl_eigen_sort_t,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    s = gsl_eigen_symmv_sort(eval, evec, sort_type);
    return s;
}
pub unsafe extern "C" fn gsl_eigen_genhermv_sort(
    mut eval: *mut gsl_vector,
    mut evec: *mut gsl_matrix_complex,
    mut sort_type: gsl_eigen_sort_t,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    s = gsl_eigen_hermv_sort(eval, evec, sort_type);
    return s;
}
pub unsafe extern "C" fn gsl_eigen_genv_sort(
    mut alpha: *mut gsl_vector_complex,
    mut beta: *mut gsl_vector,
    mut evec: *mut gsl_matrix_complex,
    mut sort_type: gsl_eigen_sort_t,
) -> libc::c_int {
    if (*evec).size1 != (*evec).size2 {
        gsl_error(
            b"eigenvector matrix must be square\0" as *const u8 as *const libc::c_char,
            b"sort.c\0" as *const u8 as *const libc::c_char,
            279 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*alpha).size != (*evec).size1 || (*beta).size != (*evec).size1 {
        gsl_error(
            b"eigenvalues must match eigenvector matrix\0" as *const u8
                as *const libc::c_char,
            b"sort.c\0" as *const u8 as *const libc::c_char,
            283 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let N: size_t = (*alpha).size;
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            let mut j: size_t = 0;
            let mut k: size_t = i;
            let mut ak: gsl_complex = gsl_vector_complex_get(alpha, i);
            let mut bk: libc::c_double = gsl_vector_get(beta, i);
            let mut ek: gsl_complex = gsl_complex { dat: [0.; 2] };
            if bk < 2.2204460492503131e-16f64 {
                ek
                    .dat[0 as libc::c_int
                    as usize] = (if if ak.dat[0 as libc::c_int as usize] >= 0.0f64 {
                    1 as libc::c_int
                } else {
                    -(1 as libc::c_int)
                } != 0
                {
                    ::std::f32::INFINITY
                } else {
                    -::std::f32::INFINITY
                }) as libc::c_double;
                ek
                    .dat[1 as libc::c_int
                    as usize] = (if if ak.dat[1 as libc::c_int as usize] >= 0.0f64 {
                    1 as libc::c_int
                } else {
                    -(1 as libc::c_int)
                } != 0
                {
                    ::std::f32::INFINITY
                } else {
                    -::std::f32::INFINITY
                }) as libc::c_double;
            } else {
                ek = gsl_complex_div_real(ak, bk);
            }
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < N {
                let mut test: libc::c_int = 0;
                let aj: gsl_complex = gsl_vector_complex_get(alpha, j);
                let mut bj: libc::c_double = gsl_vector_get(beta, j);
                let mut ej: gsl_complex = gsl_complex { dat: [0.; 2] };
                if bj < 2.2204460492503131e-16f64 {
                    ej
                        .dat[0 as libc::c_int
                        as usize] = (if if aj.dat[0 as libc::c_int as usize] >= 0.0f64 {
                        1 as libc::c_int
                    } else {
                        -(1 as libc::c_int)
                    } != 0
                    {
                        ::std::f32::INFINITY
                    } else {
                        -::std::f32::INFINITY
                    }) as libc::c_double;
                    ej
                        .dat[1 as libc::c_int
                        as usize] = (if if aj.dat[1 as libc::c_int as usize] >= 0.0f64 {
                        1 as libc::c_int
                    } else {
                        -(1 as libc::c_int)
                    } != 0
                    {
                        ::std::f32::INFINITY
                    } else {
                        -::std::f32::INFINITY
                    }) as libc::c_double;
                } else {
                    ej = gsl_complex_div_real(aj, bj);
                }
                match sort_type as libc::c_uint {
                    2 => {
                        test = (gsl_complex_abs(ej) < gsl_complex_abs(ek))
                            as libc::c_int;
                    }
                    3 => {
                        test = (gsl_complex_abs(ej) > gsl_complex_abs(ek))
                            as libc::c_int;
                    }
                    0 | 1 | _ => {
                        gsl_error(
                            b"invalid sort type\0" as *const u8 as *const libc::c_char,
                            b"sort.c\0" as *const u8 as *const libc::c_char,
                            336 as libc::c_int,
                            GSL_EINVAL as libc::c_int,
                        );
                        return GSL_EINVAL as libc::c_int;
                    }
                }
                if test != 0 {
                    k = j;
                    ek = ej;
                }
                j = j.wrapping_add(1);
                j;
            }
            if k != i {
                gsl_vector_complex_swap_elements(alpha, i, k);
                gsl_vector_swap_elements(beta, i, k);
                gsl_matrix_complex_swap_columns(evec, i, k);
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
