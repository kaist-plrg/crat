use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_bspline_knots_uniform(
        a: libc::c_double,
        b: libc::c_double,
        w: *mut gsl_bspline_workspace,
    ) -> libc::c_int;
    fn gsl_bspline_knots(
        breakpts: *const gsl_vector,
        w: *mut gsl_bspline_workspace,
    ) -> libc::c_int;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_view_array(v: *mut libc::c_double, n: size_t) -> _gsl_vector_view;
    fn gsl_vector_view_array_with_stride(
        base: *mut libc::c_double,
        stride: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_matrix_view_array(
        base: *mut libc::c_double,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_linalg_QR_decomp(A: *mut gsl_matrix, tau: *mut gsl_vector) -> libc::c_int;
    fn gsl_linalg_QR_lssolve(
        QR: *const gsl_matrix,
        tau: *const gsl_vector,
        b: *const gsl_vector,
        x: *mut gsl_vector,
        residual: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_stats_mean(
        data: *const libc::c_double,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
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
pub struct _gsl_vector_view {
    pub vector: gsl_vector,
}
pub type gsl_vector_view = _gsl_vector_view;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_matrix_view {
    pub matrix: gsl_matrix,
}
pub type gsl_matrix_view = _gsl_matrix_view;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bspline_workspace {
    pub k: size_t,
    pub km1: size_t,
    pub l: size_t,
    pub nbreak: size_t,
    pub n: size_t,
    pub knots: *mut gsl_vector,
    pub deltal: *mut gsl_vector,
    pub deltar: *mut gsl_vector,
    pub B: *mut gsl_vector,
    pub A: *mut gsl_matrix,
    pub dB: *mut gsl_matrix,
}
#[inline]
unsafe extern "C" fn gsl_vector_get(
    mut v: *const gsl_vector,
    i: size_t,
) -> libc::c_double {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_set(
    mut v: *mut gsl_vector,
    i: size_t,
    mut x: libc::c_double,
) {
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
#[inline]
unsafe extern "C" fn gsl_vector_ptr(
    mut v: *mut gsl_vector,
    i: size_t,
) -> *mut libc::c_double {
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_set(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
    x: libc::c_double,
) {
    *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize) = x;
}
pub unsafe extern "C" fn gsl_bspline_greville_abscissa(
    mut i: size_t,
    mut w: *mut gsl_bspline_workspace,
) -> libc::c_double {
    let stride: size_t = (*(*w).knots).stride;
    let mut km1: size_t = (*w).km1;
    let mut data: *mut libc::c_double = ((*(*w).knots).data)
        .offset(
            i.wrapping_add(1 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                as isize,
        );
    if km1 == 0 as libc::c_int as libc::c_ulong {
        km1 = 2 as libc::c_int as size_t;
        data = data.offset(-(stride as isize));
    }
    return gsl_stats_mean(data as *const libc::c_double, stride, km1);
}
pub unsafe extern "C" fn gsl_bspline_knots_greville(
    mut abscissae: *const gsl_vector,
    mut w: *mut gsl_bspline_workspace,
    mut abserr: *mut libc::c_double,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    if (*w).k < 2 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"w->k must be at least 2\0" as *const u8 as *const libc::c_char,
            b"greville.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*abscissae).size < 2 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"abscissae->size must be at least 2\0" as *const u8 as *const libc::c_char,
            b"greville.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else if (*w).nbreak
        != ((*abscissae).size)
            .wrapping_sub((*w).k)
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
    {
        gsl_error(
            b"w->nbreak must equal abscissae->size - w->k + 2\0" as *const u8
                as *const libc::c_char,
            b"greville.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if (*w).nbreak == 2 as libc::c_int as libc::c_ulong {
        s = gsl_bspline_knots_uniform(
            gsl_vector_get(abscissae, 0 as libc::c_int as size_t),
            gsl_vector_get(
                abscissae,
                ((*abscissae).size).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ),
            w,
        );
    } else {
        let mut storage: *mut libc::c_double = 0 as *mut libc::c_double;
        let mut A: gsl_matrix_view = gsl_matrix_view {
            matrix: gsl_matrix {
                size1: 0,
                size2: 0,
                tda: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        let mut tau: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        let mut b: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        let mut x: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        let mut r: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let km2: size_t = ((*w).k).wrapping_sub(2 as libc::c_int as libc::c_ulong);
        let M: size_t = ((*abscissae).size)
            .wrapping_sub(2 as libc::c_int as libc::c_ulong);
        let N: size_t = ((*w).nbreak).wrapping_sub(2 as libc::c_int as libc::c_ulong);
        let invkm1: libc::c_double = 1.0f64 / (*w).km1 as libc::c_double;
        storage = calloc(
            M
                .wrapping_mul(N)
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(N))
                .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(M)),
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        ) as *mut libc::c_double;
        if storage.is_null() {
            gsl_error(
                b"failed to allocate working storage\0" as *const u8
                    as *const libc::c_char,
                b"greville.c\0" as *const u8 as *const libc::c_char,
                93 as libc::c_int,
                GSL_ENOMEM as libc::c_int,
            );
            return GSL_ENOMEM as libc::c_int;
        }
        A = gsl_matrix_view_array(storage, M, N);
        tau = gsl_vector_view_array(storage.offset(M.wrapping_mul(N) as isize), N);
        b = gsl_vector_view_array(
            storage.offset(M.wrapping_mul(N) as isize).offset(N as isize),
            M,
        );
        x = gsl_vector_view_array(
            storage
                .offset(M.wrapping_mul(N) as isize)
                .offset(N as isize)
                .offset(M as isize),
            N,
        );
        r = gsl_vector_view_array(
            storage
                .offset(M.wrapping_mul(N) as isize)
                .offset(N as isize)
                .offset(M as isize)
                .offset(N as isize),
            M,
        );
        j = 0 as libc::c_int as size_t;
        while j < N {
            i = 0 as libc::c_int as size_t;
            while i <= km2 {
                gsl_matrix_set(&mut A.matrix, i.wrapping_add(j), j, invkm1);
                i = i.wrapping_add(1);
                i;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = 0 as libc::c_int as size_t;
        while i < M {
            gsl_vector_set(
                &mut b.vector,
                i,
                gsl_vector_get(
                    abscissae,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                ),
            );
            i = i.wrapping_add(1);
            i;
        }
        i = 0 as libc::c_int as size_t;
        while i < km2 {
            let v: *mut libc::c_double = gsl_vector_ptr(&mut b.vector, i);
            *v
                -= (1 as libc::c_int as libc::c_double
                    - i.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double
                        * invkm1)
                    * gsl_vector_get(abscissae, 0 as libc::c_int as size_t);
            i = i.wrapping_add(1);
            i;
        }
        i = 0 as libc::c_int as size_t;
        while i < km2 {
            let v_0: *mut libc::c_double = gsl_vector_ptr(
                &mut b.vector,
                M.wrapping_sub(km2).wrapping_add(i),
            );
            *v_0
                -= i.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double
                    * invkm1
                    * gsl_vector_get(
                        abscissae,
                        ((*abscissae).size)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
            i = i.wrapping_add(1);
            i;
        }
        s = (gsl_linalg_QR_decomp(&mut A.matrix, &mut tau.vector) != 0
            || gsl_linalg_QR_lssolve(
                &mut A.matrix,
                &mut tau.vector,
                &mut b.vector,
                &mut x.vector,
                &mut r.vector,
            ) != 0) as libc::c_int;
        if s != 0 {
            free(storage as *mut libc::c_void);
            return s;
        }
        x = gsl_vector_view_array_with_stride(
            (gsl_vector_ptr(&mut x.vector, 0 as libc::c_int as size_t))
                .offset(-(x.vector.stride as isize)),
            x.vector.stride,
            (x.vector.size).wrapping_add(2 as libc::c_int as libc::c_ulong),
        );
        gsl_vector_set(
            &mut x.vector,
            0 as libc::c_int as size_t,
            gsl_vector_get(abscissae, 0 as libc::c_int as size_t),
        );
        gsl_vector_set(
            &mut x.vector,
            (x.vector.size).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            gsl_vector_get(
                abscissae,
                ((*abscissae).size).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ),
        );
        s = gsl_bspline_knots(&mut x.vector, w);
        free(storage as *mut libc::c_void);
    }
    if s == 0 && !abserr.is_null() {
        let mut i_0: size_t = 0;
        *abserr = 0 as libc::c_int as libc::c_double;
        i_0 = 1 as libc::c_int as size_t;
        while i_0 < ((*abscissae).size).wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            *abserr
                += fabs(
                    gsl_bspline_greville_abscissa(i_0, w)
                        - gsl_vector_get(abscissae, i_0),
                );
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
    }
    return s;
}
