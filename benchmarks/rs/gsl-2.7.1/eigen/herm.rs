use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_view_array(v: *mut libc::c_double, n: size_t) -> _gsl_vector_view;
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_complex_view_array(
        base: *mut libc::c_double,
        n: size_t,
    ) -> _gsl_vector_complex_view;
    fn gsl_linalg_hermtd_unpack_T(
        A: *const gsl_matrix_complex,
        diag: *mut gsl_vector,
        subdiag: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_linalg_hermtd_decomp(
        A: *mut gsl_matrix_complex,
        tau: *mut gsl_vector_complex,
    ) -> libc::c_int;
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
pub struct _gsl_vector_view {
    pub vector: gsl_vector,
}
pub type gsl_vector_view = _gsl_vector_view;
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
pub struct _gsl_vector_complex_view {
    pub vector: gsl_vector_complex,
}
pub type gsl_vector_complex_view = _gsl_vector_complex_view;
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
pub struct gsl_eigen_herm_workspace {
    pub size: size_t,
    pub d: *mut libc::c_double,
    pub sd: *mut libc::c_double,
    pub tau: *mut libc::c_double,
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
unsafe extern "C" fn gsl_matrix_complex_get(
    mut m: *const gsl_matrix_complex,
    i: size_t,
    j: size_t,
) -> gsl_complex {
    return *(((*m).data)
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(i.wrapping_mul((*m).tda).wrapping_add(j)) as isize,
        ) as *mut gsl_complex);
}
unsafe extern "C" fn chop_small_elements(
    N: size_t,
    mut d: *const libc::c_double,
    mut sd: *mut libc::c_double,
) {
    let mut d_i: libc::c_double = *d.offset(0 as libc::c_int as isize);
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        let mut sd_i: libc::c_double = *sd.offset(i as isize);
        let mut d_ip1: libc::c_double = *d
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        if fabs(sd_i) < 2.2204460492503131e-16f64 * (fabs(d_i) + fabs(d_ip1)) {
            *sd.offset(i as isize) = 0.0f64;
        }
        d_i = d_ip1;
        i = i.wrapping_add(1);
        i;
    }
}
#[inline]
unsafe extern "C" fn create_givens(
    a: libc::c_double,
    b: libc::c_double,
    mut c: *mut libc::c_double,
    mut s: *mut libc::c_double,
) {
    if b == 0 as libc::c_int as libc::c_double {
        *c = 1 as libc::c_int as libc::c_double;
        *s = 0 as libc::c_int as libc::c_double;
    } else if fabs(b) > fabs(a) {
        let mut t: libc::c_double = -a / b;
        let mut s1: libc::c_double = 1.0f64
            / sqrt(1 as libc::c_int as libc::c_double + t * t);
        *s = s1;
        *c = s1 * t;
    } else {
        let mut t_0: libc::c_double = -b / a;
        let mut c1: libc::c_double = 1.0f64
            / sqrt(1 as libc::c_int as libc::c_double + t_0 * t_0);
        *c = c1;
        *s = c1 * t_0;
    };
}
#[inline]
unsafe extern "C" fn trailing_eigenvalue(
    n: size_t,
    mut d: *const libc::c_double,
    mut sd: *const libc::c_double,
) -> libc::c_double {
    let mut ta: libc::c_double = *d
        .offset(n.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize);
    let mut tb: libc::c_double = *d
        .offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    let mut tab: libc::c_double = *sd
        .offset(n.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize);
    let mut dt: libc::c_double = (ta - tb) / 2.0f64;
    let mut mu: libc::c_double = 0.;
    if dt > 0 as libc::c_int as libc::c_double {
        mu = tb - tab * (tab / (dt + hypot(dt, tab)));
    } else if dt == 0 as libc::c_int as libc::c_double {
        mu = tb - fabs(tab);
    } else {
        mu = tb + tab * (tab / (-dt + hypot(dt, tab)));
    }
    return mu;
}
unsafe extern "C" fn qrstep(
    n: size_t,
    mut d: *mut libc::c_double,
    mut sd: *mut libc::c_double,
    mut gc: *mut libc::c_double,
    mut gs: *mut libc::c_double,
) {
    let mut x: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    let mut ak: libc::c_double = 0.;
    let mut bk: libc::c_double = 0.;
    let mut zk: libc::c_double = 0.;
    let mut ap: libc::c_double = 0.;
    let mut bp: libc::c_double = 0.;
    let mut aq: libc::c_double = 0.;
    let mut bq: libc::c_double = 0.;
    let mut k: size_t = 0;
    let mut mu: libc::c_double = trailing_eigenvalue(
        n,
        d as *const libc::c_double,
        sd as *const libc::c_double,
    );
    if 2.2204460492503131e-16f64 * fabs(mu)
        > fabs(*d.offset(0 as libc::c_int as isize))
            + fabs(*sd.offset(0 as libc::c_int as isize))
    {
        mu = 0 as libc::c_int as libc::c_double;
    }
    x = *d.offset(0 as libc::c_int as isize) - mu;
    z = *sd.offset(0 as libc::c_int as isize);
    ak = 0 as libc::c_int as libc::c_double;
    bk = 0 as libc::c_int as libc::c_double;
    zk = 0 as libc::c_int as libc::c_double;
    ap = *d.offset(0 as libc::c_int as isize);
    bp = *sd.offset(0 as libc::c_int as isize);
    aq = *d.offset(1 as libc::c_int as isize);
    if n == 2 as libc::c_int as libc::c_ulong {
        let mut c: libc::c_double = 0.;
        let mut s: libc::c_double = 0.;
        create_givens(x, z, &mut c, &mut s);
        if !gc.is_null() {
            *gc.offset(0 as libc::c_int as isize) = c;
        }
        if !gs.is_null() {
            *gs.offset(0 as libc::c_int as isize) = s;
        }
        let mut ap1: libc::c_double = c * (c * ap - s * bp) + s * (s * aq - c * bp);
        let mut bp1: libc::c_double = c * (s * ap + c * bp) - s * (s * bp + c * aq);
        let mut aq1: libc::c_double = s * (s * ap + c * bp) + c * (s * bp + c * aq);
        ak = ap1;
        bk = bp1;
        ap = aq1;
        *d.offset(0 as libc::c_int as isize) = ak;
        *sd.offset(0 as libc::c_int as isize) = bk;
        *d.offset(1 as libc::c_int as isize) = ap;
        return;
    }
    bq = *sd.offset(1 as libc::c_int as isize);
    k = 0 as libc::c_int as size_t;
    while k < n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        let mut c_0: libc::c_double = 0.;
        let mut s_0: libc::c_double = 0.;
        create_givens(x, z, &mut c_0, &mut s_0);
        if !gc.is_null() {
            *gc.offset(k as isize) = c_0;
        }
        if !gs.is_null() {
            *gs.offset(k as isize) = s_0;
        }
        let mut bk1: libc::c_double = c_0 * bk - s_0 * zk;
        let mut ap1_0: libc::c_double = c_0 * (c_0 * ap - s_0 * bp)
            + s_0 * (s_0 * aq - c_0 * bp);
        let mut bp1_0: libc::c_double = c_0 * (s_0 * ap + c_0 * bp)
            - s_0 * (s_0 * bp + c_0 * aq);
        let mut zp1: libc::c_double = -s_0 * bq;
        let mut aq1_0: libc::c_double = s_0 * (s_0 * ap + c_0 * bp)
            + c_0 * (s_0 * bp + c_0 * aq);
        let mut bq1: libc::c_double = c_0 * bq;
        ak = ap1_0;
        bk = bp1_0;
        zk = zp1;
        ap = aq1_0;
        bp = bq1;
        if k < n.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
            aq = *d.offset(k.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize);
        }
        if k < n.wrapping_sub(3 as libc::c_int as libc::c_ulong) {
            bq = *sd.offset(k.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize);
        }
        *d.offset(k as isize) = ak;
        if k > 0 as libc::c_int as libc::c_ulong {
            *sd.offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) = bk1;
        }
        if k < n.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
            *sd.offset(k.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) = bp;
        }
        x = bk;
        z = zk;
        k = k.wrapping_add(1);
        k;
    }
    *d.offset(k as isize) = ap;
    *sd.offset(k.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) = bk;
}
pub unsafe extern "C" fn gsl_eigen_herm_alloc(
    n: size_t,
) -> *mut gsl_eigen_herm_workspace {
    let mut w: *mut gsl_eigen_herm_workspace = 0 as *mut gsl_eigen_herm_workspace;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"herm.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut gsl_eigen_herm_workspace;
    }
    w = malloc(::std::mem::size_of::<gsl_eigen_herm_workspace>() as libc::c_ulong)
        as *mut gsl_eigen_herm_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate space for workspace\0" as *const u8
                as *const libc::c_char,
            b"herm.c\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_eigen_herm_workspace;
    }
    (*w)
        .d = malloc(
        n.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*w).d).is_null() {
        gsl_error(
            b"failed to allocate space for diagonal\0" as *const u8
                as *const libc::c_char,
            b"herm.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_eigen_herm_workspace;
    }
    (*w)
        .sd = malloc(
        n.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*w).sd).is_null() {
        gsl_error(
            b"failed to allocate space for subdiagonal\0" as *const u8
                as *const libc::c_char,
            b"herm.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_eigen_herm_workspace;
    }
    (*w)
        .tau = malloc(
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*w).tau).is_null() {
        gsl_error(
            b"failed to allocate space for tau\0" as *const u8 as *const libc::c_char,
            b"herm.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_eigen_herm_workspace;
    }
    (*w).size = n;
    return w;
}
pub unsafe extern "C" fn gsl_eigen_herm_free(mut w: *mut gsl_eigen_herm_workspace) {
    if w.is_null() {
        return;
    }
    free((*w).tau as *mut libc::c_void);
    free((*w).sd as *mut libc::c_void);
    free((*w).d as *mut libc::c_void);
    free(w as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_eigen_herm(
    mut A: *mut gsl_matrix_complex,
    mut eval: *mut gsl_vector,
    mut w: *mut gsl_eigen_herm_workspace,
) -> libc::c_int {
    if (*A).size1 != (*A).size2 {
        gsl_error(
            b"matrix must be square to compute eigenvalues\0" as *const u8
                as *const libc::c_char,
            b"herm.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if (*eval).size != (*A).size1 {
        gsl_error(
            b"eigenvalue vector must match matrix size\0" as *const u8
                as *const libc::c_char,
            b"herm.c\0" as *const u8 as *const libc::c_char,
            99 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let N: size_t = (*A).size1;
        let d: *mut libc::c_double = (*w).d;
        let sd: *mut libc::c_double = (*w).sd;
        let mut a: size_t = 0;
        let mut b: size_t = 0;
        if N == 1 as libc::c_int as libc::c_ulong {
            let mut A00: gsl_complex = gsl_matrix_complex_get(
                A,
                0 as libc::c_int as size_t,
                0 as libc::c_int as size_t,
            );
            gsl_vector_set(
                eval,
                0 as libc::c_int as size_t,
                A00.dat[0 as libc::c_int as usize],
            );
            return GSL_SUCCESS as libc::c_int;
        }
        let mut d_vec: gsl_vector_view = gsl_vector_view_array(d, N);
        let mut sd_vec: gsl_vector_view = gsl_vector_view_array(
            sd,
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        let mut tau_vec: gsl_vector_complex_view = gsl_vector_complex_view_array(
            (*w).tau,
            N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        gsl_linalg_hermtd_decomp(A, &mut tau_vec.vector);
        gsl_linalg_hermtd_unpack_T(A, &mut d_vec.vector, &mut sd_vec.vector);
        chop_small_elements(N, d as *const libc::c_double, sd);
        b = N.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        while b > 0 as libc::c_int as libc::c_ulong {
            if *sd.offset(b.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                == 0.0f64
                || (*sd
                    .offset(b.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                    .is_nan() as i32 != 0
            {
                b = b.wrapping_sub(1);
                b;
            } else {
                a = b.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while a > 0 as libc::c_int as libc::c_ulong {
                    if *sd
                        .offset(
                            a.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) == 0.0f64
                    {
                        break;
                    }
                    a = a.wrapping_sub(1);
                    a;
                }
                let n_block: size_t = b
                    .wrapping_sub(a)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                let mut d_block: *mut libc::c_double = d.offset(a as isize);
                let mut sd_block: *mut libc::c_double = sd.offset(a as isize);
                qrstep(
                    n_block,
                    d_block,
                    sd_block,
                    0 as *mut libc::c_double,
                    0 as *mut libc::c_double,
                );
                chop_small_elements(n_block, d_block as *const libc::c_double, sd_block);
            }
        }
        let mut d_vec_0: gsl_vector_view = gsl_vector_view_array(d, N);
        gsl_vector_memcpy(eval, &mut d_vec_0.vector);
        return GSL_SUCCESS as libc::c_int;
    };
}
