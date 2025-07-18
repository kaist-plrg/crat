use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn fabs(_: libc::c_double) -> libc::c_double;
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
unsafe extern "C" fn solve_tridiag(
    mut diag: *const libc::c_double,
    mut d_stride: size_t,
    mut offdiag: *const libc::c_double,
    mut o_stride: size_t,
    mut b: *const libc::c_double,
    mut b_stride: size_t,
    mut x: *mut libc::c_double,
    mut x_stride: size_t,
    mut N: size_t,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut gamma: *mut libc::c_double = malloc(
        N.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut alpha: *mut libc::c_double = malloc(
        N.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut c: *mut libc::c_double = malloc(
        N.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut z: *mut libc::c_double = malloc(
        N.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if gamma.is_null() || alpha.is_null() || c.is_null() || z.is_null() {
        gsl_error(
            b"failed to allocate working space\0" as *const u8 as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        *alpha
            .offset(0 as libc::c_int as isize) = *diag.offset(0 as libc::c_int as isize);
        *gamma
            .offset(
                0 as libc::c_int as isize,
            ) = *offdiag.offset(0 as libc::c_int as isize)
            / *alpha.offset(0 as libc::c_int as isize);
        if *alpha.offset(0 as libc::c_int as isize) == 0 as libc::c_int as libc::c_double
        {
            status = GSL_EZERODIV as libc::c_int;
        }
        i = 1 as libc::c_int as size_t;
        while i < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            *alpha
                .offset(
                    i as isize,
                ) = *diag.offset(d_stride.wrapping_mul(i) as isize)
                - *offdiag
                    .offset(
                        o_stride
                            .wrapping_mul(
                                i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            ) as isize,
                    )
                    * *gamma
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
            *gamma
                .offset(
                    i as isize,
                ) = *offdiag.offset(o_stride.wrapping_mul(i) as isize)
                / *alpha.offset(i as isize);
            if *alpha.offset(i as isize) == 0 as libc::c_int as libc::c_double {
                status = GSL_EZERODIV as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
        if N > 1 as libc::c_int as libc::c_ulong {
            *alpha
                .offset(
                    N.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *diag
                .offset(
                    d_stride
                        .wrapping_mul(N.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                        as isize,
                )
                - *offdiag
                    .offset(
                        o_stride
                            .wrapping_mul(
                                N.wrapping_sub(2 as libc::c_int as libc::c_ulong),
                            ) as isize,
                    )
                    * *gamma
                        .offset(
                            N.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
                        );
        }
        *z.offset(0 as libc::c_int as isize) = *b.offset(0 as libc::c_int as isize);
        i = 1 as libc::c_int as size_t;
        while i < N {
            *z
                .offset(
                    i as isize,
                ) = *b.offset(b_stride.wrapping_mul(i) as isize)
                - *gamma
                    .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    * *z
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
            i = i.wrapping_add(1);
            i;
        }
        i = 0 as libc::c_int as size_t;
        while i < N {
            *c.offset(i as isize) = *z.offset(i as isize) / *alpha.offset(i as isize);
            i = i.wrapping_add(1);
            i;
        }
        *x
            .offset(
                x_stride.wrapping_mul(N.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = *c.offset(N.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        if N >= 2 as libc::c_int as libc::c_ulong {
            i = N.wrapping_sub(2 as libc::c_int as libc::c_ulong);
            j = 0 as libc::c_int as size_t;
            while j <= N.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
                *x
                    .offset(
                        x_stride.wrapping_mul(i) as isize,
                    ) = *c.offset(i as isize)
                    - *gamma.offset(i as isize)
                        * *x
                            .offset(
                                x_stride
                                    .wrapping_mul(
                                        i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    ) as isize,
                            );
                j = j.wrapping_add(1);
                j;
                i = i.wrapping_sub(1);
                i;
            }
        }
    }
    if !z.is_null() {
        free(z as *mut libc::c_void);
    }
    if !c.is_null() {
        free(c as *mut libc::c_void);
    }
    if !alpha.is_null() {
        free(alpha as *mut libc::c_void);
    }
    if !gamma.is_null() {
        free(gamma as *mut libc::c_void);
    }
    if status == GSL_EZERODIV as libc::c_int {
        gsl_error(
            b"matrix must be positive definite\0" as *const u8 as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int,
            status,
        );
        return status;
    }
    return status;
}
unsafe extern "C" fn solve_tridiag_nonsym(
    mut diag: *const libc::c_double,
    mut d_stride: size_t,
    mut abovediag: *const libc::c_double,
    mut a_stride: size_t,
    mut belowdiag: *const libc::c_double,
    mut b_stride: size_t,
    mut rhs: *const libc::c_double,
    mut r_stride: size_t,
    mut x: *mut libc::c_double,
    mut x_stride: size_t,
    mut N: size_t,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut alpha: *mut libc::c_double = malloc(
        N.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut z: *mut libc::c_double = malloc(
        N.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if alpha.is_null() || z.is_null() {
        gsl_error(
            b"failed to allocate working space\0" as *const u8 as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            146 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        *alpha
            .offset(0 as libc::c_int as isize) = *diag.offset(0 as libc::c_int as isize);
        *z.offset(0 as libc::c_int as isize) = *rhs.offset(0 as libc::c_int as isize);
        if *alpha.offset(0 as libc::c_int as isize) == 0 as libc::c_int as libc::c_double
        {
            status = GSL_EZERODIV as libc::c_int;
        }
        i = 1 as libc::c_int as size_t;
        while i < N {
            let t: libc::c_double = *belowdiag
                .offset(
                    b_stride
                        .wrapping_mul(i.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                        as isize,
                )
                / *alpha
                    .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            *alpha
                .offset(
                    i as isize,
                ) = *diag.offset(d_stride.wrapping_mul(i) as isize)
                - t
                    * *abovediag
                        .offset(
                            a_stride
                                .wrapping_mul(
                                    i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                ) as isize,
                        );
            *z
                .offset(
                    i as isize,
                ) = *rhs.offset(r_stride.wrapping_mul(i) as isize)
                - t
                    * *z
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
            if *alpha.offset(i as isize) == 0 as libc::c_int as libc::c_double {
                status = GSL_EZERODIV as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
        *x
            .offset(
                x_stride.wrapping_mul(N.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = *z.offset(N.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            / *alpha.offset(N.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        if N >= 2 as libc::c_int as libc::c_ulong {
            i = N.wrapping_sub(2 as libc::c_int as libc::c_ulong);
            j = 0 as libc::c_int as size_t;
            while j <= N.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
                *x
                    .offset(
                        x_stride.wrapping_mul(i) as isize,
                    ) = (*z.offset(i as isize)
                    - *abovediag.offset(a_stride.wrapping_mul(i) as isize)
                        * *x
                            .offset(
                                x_stride
                                    .wrapping_mul(
                                        i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    ) as isize,
                            )) / *alpha.offset(i as isize);
                j = j.wrapping_add(1);
                j;
                i = i.wrapping_sub(1);
                i;
            }
        }
    }
    if !z.is_null() {
        free(z as *mut libc::c_void);
    }
    if !alpha.is_null() {
        free(alpha as *mut libc::c_void);
    }
    if status == GSL_EZERODIV as libc::c_int {
        gsl_error(
            b"matrix must be positive definite\0" as *const u8 as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            191 as libc::c_int,
            status,
        );
        return status;
    }
    return status;
}
unsafe extern "C" fn solve_cyc_tridiag(
    mut diag: *const libc::c_double,
    mut d_stride: size_t,
    mut offdiag: *const libc::c_double,
    mut o_stride: size_t,
    mut b: *const libc::c_double,
    mut b_stride: size_t,
    mut x: *mut libc::c_double,
    mut x_stride: size_t,
    mut N: size_t,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut delta: *mut libc::c_double = malloc(
        N.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut gamma: *mut libc::c_double = malloc(
        N.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut alpha: *mut libc::c_double = malloc(
        N.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut c: *mut libc::c_double = malloc(
        N.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut z: *mut libc::c_double = malloc(
        N.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if delta.is_null() || gamma.is_null() || alpha.is_null() || c.is_null()
        || z.is_null()
    {
        gsl_error(
            b"failed to allocate working space\0" as *const u8 as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            225 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut sum: libc::c_double = 0.0f64;
        if N == 1 as libc::c_int as libc::c_ulong {
            *x
                .offset(
                    0 as libc::c_int as isize,
                ) = *b.offset(0 as libc::c_int as isize)
                / *diag.offset(0 as libc::c_int as isize);
            free(delta as *mut libc::c_void);
            free(gamma as *mut libc::c_void);
            free(alpha as *mut libc::c_void);
            free(c as *mut libc::c_void);
            free(z as *mut libc::c_void);
            return GSL_SUCCESS as libc::c_int;
        }
        *alpha
            .offset(0 as libc::c_int as isize) = *diag.offset(0 as libc::c_int as isize);
        *gamma
            .offset(
                0 as libc::c_int as isize,
            ) = *offdiag.offset(0 as libc::c_int as isize)
            / *alpha.offset(0 as libc::c_int as isize);
        *delta
            .offset(
                0 as libc::c_int as isize,
            ) = *offdiag
            .offset(
                o_stride.wrapping_mul(N.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) / *alpha.offset(0 as libc::c_int as isize);
        if *alpha.offset(0 as libc::c_int as isize) == 0 as libc::c_int as libc::c_double
        {
            status = GSL_EZERODIV as libc::c_int;
        }
        i = 1 as libc::c_int as size_t;
        while i < N.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
            *alpha
                .offset(
                    i as isize,
                ) = *diag.offset(d_stride.wrapping_mul(i) as isize)
                - *offdiag
                    .offset(
                        o_stride
                            .wrapping_mul(
                                i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            ) as isize,
                    )
                    * *gamma
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
            *gamma
                .offset(
                    i as isize,
                ) = *offdiag.offset(o_stride.wrapping_mul(i) as isize)
                / *alpha.offset(i as isize);
            *delta
                .offset(
                    i as isize,
                ) = -*delta
                .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                * *offdiag
                    .offset(
                        o_stride
                            .wrapping_mul(
                                i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            ) as isize,
                    ) / *alpha.offset(i as isize);
            if *alpha.offset(i as isize) == 0 as libc::c_int as libc::c_double {
                status = GSL_EZERODIV as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
        i = 0 as libc::c_int as size_t;
        while i < N.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
            sum
                += *alpha.offset(i as isize) * *delta.offset(i as isize)
                    * *delta.offset(i as isize);
            i = i.wrapping_add(1);
            i;
        }
        *alpha
            .offset(
                N.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
            ) = *diag
            .offset(
                d_stride.wrapping_mul(N.wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    as isize,
            )
            - *offdiag
                .offset(
                    o_stride
                        .wrapping_mul(N.wrapping_sub(3 as libc::c_int as libc::c_ulong))
                        as isize,
                )
                * *gamma
                    .offset(N.wrapping_sub(3 as libc::c_int as libc::c_ulong) as isize);
        *gamma
            .offset(
                N.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
            ) = (*offdiag
            .offset(
                o_stride.wrapping_mul(N.wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    as isize,
            )
            - *offdiag
                .offset(
                    o_stride
                        .wrapping_mul(N.wrapping_sub(3 as libc::c_int as libc::c_ulong))
                        as isize,
                )
                * *delta
                    .offset(N.wrapping_sub(3 as libc::c_int as libc::c_ulong) as isize))
            / *alpha.offset(N.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize);
        *alpha
            .offset(
                N.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = *diag
            .offset(
                d_stride.wrapping_mul(N.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) - sum
            - *alpha.offset(N.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
                * *gamma
                    .offset(N.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
                * *gamma
                    .offset(N.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize);
        *z.offset(0 as libc::c_int as isize) = *b.offset(0 as libc::c_int as isize);
        i = 1 as libc::c_int as size_t;
        while i < N.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
            *z
                .offset(
                    i as isize,
                ) = *b.offset(b_stride.wrapping_mul(i) as isize)
                - *z.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    * *gamma
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
            i = i.wrapping_add(1);
            i;
        }
        sum = 0.0f64;
        i = 0 as libc::c_int as size_t;
        while i < N.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
            sum += *delta.offset(i as isize) * *z.offset(i as isize);
            i = i.wrapping_add(1);
            i;
        }
        *z
            .offset(
                N.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = *b
            .offset(
                b_stride.wrapping_mul(N.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) - sum
            - *gamma.offset(N.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
                * *z.offset(N.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize);
        i = 0 as libc::c_int as size_t;
        while i < N {
            *c.offset(i as isize) = *z.offset(i as isize) / *alpha.offset(i as isize);
            i = i.wrapping_add(1);
            i;
        }
        *x
            .offset(
                x_stride.wrapping_mul(N.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = *c.offset(N.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        *x
            .offset(
                x_stride.wrapping_mul(N.wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = *c.offset(N.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
            - *gamma.offset(N.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
                * *x
                    .offset(
                        x_stride
                            .wrapping_mul(
                                N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            ) as isize,
                    );
        if N >= 3 as libc::c_int as libc::c_ulong {
            i = N.wrapping_sub(3 as libc::c_int as libc::c_ulong);
            j = 0 as libc::c_int as size_t;
            while j <= N.wrapping_sub(3 as libc::c_int as libc::c_ulong) {
                *x
                    .offset(
                        x_stride.wrapping_mul(i) as isize,
                    ) = *c.offset(i as isize)
                    - *gamma.offset(i as isize)
                        * *x
                            .offset(
                                x_stride
                                    .wrapping_mul(
                                        i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    ) as isize,
                            )
                    - *delta.offset(i as isize)
                        * *x
                            .offset(
                                x_stride
                                    .wrapping_mul(
                                        N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    ) as isize,
                            );
                j = j.wrapping_add(1);
                j;
                i = i.wrapping_sub(1);
                i;
            }
        }
    }
    if !z.is_null() {
        free(z as *mut libc::c_void);
    }
    if !c.is_null() {
        free(c as *mut libc::c_void);
    }
    if !alpha.is_null() {
        free(alpha as *mut libc::c_void);
    }
    if !gamma.is_null() {
        free(gamma as *mut libc::c_void);
    }
    if !delta.is_null() {
        free(delta as *mut libc::c_void);
    }
    if status == GSL_EZERODIV as libc::c_int {
        gsl_error(
            b"matrix must be positive definite\0" as *const u8 as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            315 as libc::c_int,
            status,
        );
        return status;
    }
    return status;
}
unsafe extern "C" fn solve_cyc_tridiag_nonsym(
    mut diag: *const libc::c_double,
    mut d_stride: size_t,
    mut abovediag: *const libc::c_double,
    mut a_stride: size_t,
    mut belowdiag: *const libc::c_double,
    mut b_stride: size_t,
    mut rhs: *const libc::c_double,
    mut r_stride: size_t,
    mut x: *mut libc::c_double,
    mut x_stride: size_t,
    mut N: size_t,
) -> libc::c_int {
    let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
    let mut alpha: *mut libc::c_double = malloc(
        N.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut zb: *mut libc::c_double = malloc(
        N.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut zu: *mut libc::c_double = malloc(
        N.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut w: *mut libc::c_double = malloc(
        N.wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if alpha.is_null() || zb.is_null() || zu.is_null() || w.is_null() {
        gsl_error(
            b"failed to allocate working space\0" as *const u8 as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            348 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    } else {
        let mut beta: libc::c_double = 0.;
        *zb.offset(0 as libc::c_int as isize) = *rhs.offset(0 as libc::c_int as isize);
        if *diag.offset(0 as libc::c_int as isize) != 0 as libc::c_int as libc::c_double
        {
            beta = -*diag.offset(0 as libc::c_int as isize);
        } else {
            beta = 1 as libc::c_int as libc::c_double;
        }
        let q: libc::c_double = 1 as libc::c_int as libc::c_double
            - *abovediag.offset(0 as libc::c_int as isize)
                * *belowdiag.offset(0 as libc::c_int as isize)
                / (*diag.offset(0 as libc::c_int as isize)
                    * *diag.offset(d_stride as isize));
        if fabs(q / beta) > 0.5f64 && fabs(q / beta) < 2 as libc::c_int as libc::c_double
        {
            beta
                *= if fabs(q / beta) < 1 as libc::c_int as libc::c_double {
                    0.5f64
                } else {
                    2 as libc::c_int as libc::c_double
                };
        }
        *zu.offset(0 as libc::c_int as isize) = beta;
        *alpha
            .offset(
                0 as libc::c_int as isize,
            ) = *diag.offset(0 as libc::c_int as isize) - beta;
        if *alpha.offset(0 as libc::c_int as isize) == 0 as libc::c_int as libc::c_double
        {
            status = GSL_EZERODIV as libc::c_int;
        }
        let mut i: size_t = 0;
        i = 1 as libc::c_int as size_t;
        while i.wrapping_add(1 as libc::c_int as libc::c_ulong) < N {
            let t: libc::c_double = *belowdiag
                .offset(
                    b_stride
                        .wrapping_mul(i.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                        as isize,
                )
                / *alpha
                    .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            *alpha
                .offset(
                    i as isize,
                ) = *diag.offset(d_stride.wrapping_mul(i) as isize)
                - t
                    * *abovediag
                        .offset(
                            a_stride
                                .wrapping_mul(
                                    i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                ) as isize,
                        );
            *zb
                .offset(
                    i as isize,
                ) = *rhs.offset(r_stride.wrapping_mul(i) as isize)
                - t
                    * *zb
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
            *zu
                .offset(
                    i as isize,
                ) = -t
                * *zu.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            if *alpha.offset(i as isize) == 0 as libc::c_int as libc::c_double {
                status = GSL_EZERODIV as libc::c_int;
            }
            i = i.wrapping_add(1);
            i;
        }
        let i_0: size_t = N.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let t_0: libc::c_double = *belowdiag
            .offset(
                b_stride
                    .wrapping_mul(i_0.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                    as isize,
            )
            / *alpha
                .offset(i_0.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        *alpha
            .offset(
                i_0 as isize,
            ) = *diag.offset(d_stride.wrapping_mul(i_0) as isize)
            - *abovediag.offset(a_stride.wrapping_mul(i_0) as isize)
                * *belowdiag.offset(b_stride.wrapping_mul(i_0) as isize) / beta
            - t_0
                * *abovediag
                    .offset(
                        a_stride
                            .wrapping_mul(
                                i_0.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            ) as isize,
                    );
        *zb
            .offset(
                i_0 as isize,
            ) = *rhs.offset(r_stride.wrapping_mul(i_0) as isize)
            - t_0
                * *zb
                    .offset(
                        i_0.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    );
        *zu
            .offset(
                i_0 as isize,
            ) = *abovediag.offset(a_stride.wrapping_mul(i_0) as isize)
            - t_0
                * *zu
                    .offset(
                        i_0.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    );
        if *alpha.offset(i_0 as isize) == 0 as libc::c_int as libc::c_double {
            status = GSL_EZERODIV as libc::c_int;
        }
        let mut i_1: size_t = 0;
        let mut j: size_t = 0;
        *w
            .offset(
                N.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = *zu.offset(N.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            / *alpha.offset(N.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        *x
            .offset(
                x_stride.wrapping_mul(N.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                    as isize,
            ) = *zb.offset(N.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            / *alpha.offset(N.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        i_1 = N.wrapping_sub(2 as libc::c_int as libc::c_ulong);
        j = 0 as libc::c_int as size_t;
        while j <= N.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
            *w
                .offset(
                    i_1 as isize,
                ) = (*zu.offset(i_1 as isize)
                - *abovediag.offset(a_stride.wrapping_mul(i_1) as isize)
                    * *w
                        .offset(
                            i_1.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )) / *alpha.offset(i_1 as isize);
            *x
                .offset(
                    i_1.wrapping_mul(x_stride) as isize,
                ) = (*zb.offset(i_1 as isize)
                - *abovediag.offset(a_stride.wrapping_mul(i_1) as isize)
                    * *x
                        .offset(
                            x_stride
                                .wrapping_mul(
                                    i_1.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as isize,
                        )) / *alpha.offset(i_1 as isize);
            j = j.wrapping_add(1);
            j;
            i_1 = i_1.wrapping_sub(1);
            i_1;
        }
        let vw: libc::c_double = *w.offset(0 as libc::c_int as isize)
            + *belowdiag
                .offset(
                    b_stride
                        .wrapping_mul(N.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                        as isize,
                ) / beta
                * *w.offset(N.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
        let vx: libc::c_double = *x.offset(0 as libc::c_int as isize)
            + *belowdiag
                .offset(
                    b_stride
                        .wrapping_mul(N.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                        as isize,
                ) / beta
                * *x
                    .offset(
                        x_stride
                            .wrapping_mul(
                                N.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            ) as isize,
                    );
        if vw + 1 as libc::c_int as libc::c_double == 0 as libc::c_int as libc::c_double
        {
            status = GSL_EZERODIV as libc::c_int;
        }
        let mut i_2: size_t = 0;
        i_2 = 0 as libc::c_int as size_t;
        while i_2 < N {
            *x.offset(i_2.wrapping_mul(x_stride) as isize)
                -= vx / (1 as libc::c_int as libc::c_double + vw)
                    * *w.offset(i_2 as isize);
            i_2 = i_2.wrapping_add(1);
            i_2;
        }
    }
    if !zb.is_null() {
        free(zb as *mut libc::c_void);
    }
    if !zu.is_null() {
        free(zu as *mut libc::c_void);
    }
    if !w.is_null() {
        free(w as *mut libc::c_void);
    }
    if !alpha.is_null() {
        free(alpha as *mut libc::c_void);
    }
    if status == GSL_EZERODIV as libc::c_int {
        gsl_error(
            b"matrix must be positive definite\0" as *const u8 as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            443 as libc::c_int,
            status,
        );
        return status;
    }
    return status;
}
pub unsafe extern "C" fn gsl_linalg_solve_symm_tridiag(
    mut diag: *const gsl_vector,
    mut offdiag: *const gsl_vector,
    mut rhs: *const gsl_vector,
    mut solution: *mut gsl_vector,
) -> libc::c_int {
    if (*diag).size != (*rhs).size {
        gsl_error(
            b"size of diag must match rhs\0" as *const u8 as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            458 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*offdiag).size
        != ((*rhs).size).wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        gsl_error(
            b"size of offdiag must match rhs-1\0" as *const u8 as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            462 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*solution).size != (*rhs).size {
        gsl_error(
            b"size of solution must match rhs\0" as *const u8 as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            466 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        return solve_tridiag(
            (*diag).data as *const libc::c_double,
            (*diag).stride,
            (*offdiag).data as *const libc::c_double,
            (*offdiag).stride,
            (*rhs).data as *const libc::c_double,
            (*rhs).stride,
            (*solution).data,
            (*solution).stride,
            (*diag).size,
        )
    };
}
pub unsafe extern "C" fn gsl_linalg_solve_tridiag(
    mut diag: *const gsl_vector,
    mut abovediag: *const gsl_vector,
    mut belowdiag: *const gsl_vector,
    mut rhs: *const gsl_vector,
    mut solution: *mut gsl_vector,
) -> libc::c_int {
    if (*diag).size != (*rhs).size {
        gsl_error(
            b"size of diag must match rhs\0" as *const u8 as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            489 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*abovediag).size
        != ((*rhs).size).wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        gsl_error(
            b"size of abovediag must match rhs-1\0" as *const u8 as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            493 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*belowdiag).size
        != ((*rhs).size).wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        gsl_error(
            b"size of belowdiag must match rhs-1\0" as *const u8 as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            497 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*solution).size != (*rhs).size {
        gsl_error(
            b"size of solution must match rhs\0" as *const u8 as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            501 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        return solve_tridiag_nonsym(
            (*diag).data as *const libc::c_double,
            (*diag).stride,
            (*abovediag).data as *const libc::c_double,
            (*abovediag).stride,
            (*belowdiag).data as *const libc::c_double,
            (*belowdiag).stride,
            (*rhs).data as *const libc::c_double,
            (*rhs).stride,
            (*solution).data,
            (*solution).stride,
            (*diag).size,
        )
    };
}
pub unsafe extern "C" fn gsl_linalg_solve_symm_cyc_tridiag(
    mut diag: *const gsl_vector,
    mut offdiag: *const gsl_vector,
    mut rhs: *const gsl_vector,
    mut solution: *mut gsl_vector,
) -> libc::c_int {
    if (*diag).size != (*rhs).size {
        gsl_error(
            b"size of diag must match rhs\0" as *const u8 as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            524 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*offdiag).size != (*rhs).size {
        gsl_error(
            b"size of offdiag must match rhs\0" as *const u8 as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            528 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*solution).size != (*rhs).size {
        gsl_error(
            b"size of solution must match rhs\0" as *const u8 as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            532 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*diag).size < 3 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"size of cyclic system must be 3 or more\0" as *const u8
                as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            536 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        return solve_cyc_tridiag(
            (*diag).data as *const libc::c_double,
            (*diag).stride,
            (*offdiag).data as *const libc::c_double,
            (*offdiag).stride,
            (*rhs).data as *const libc::c_double,
            (*rhs).stride,
            (*solution).data,
            (*solution).stride,
            (*diag).size,
        )
    };
}
pub unsafe extern "C" fn gsl_linalg_solve_cyc_tridiag(
    mut diag: *const gsl_vector,
    mut abovediag: *const gsl_vector,
    mut belowdiag: *const gsl_vector,
    mut rhs: *const gsl_vector,
    mut solution: *mut gsl_vector,
) -> libc::c_int {
    if (*diag).size != (*rhs).size {
        gsl_error(
            b"size of diag must match rhs\0" as *const u8 as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            558 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*abovediag).size != (*rhs).size {
        gsl_error(
            b"size of abovediag must match rhs\0" as *const u8 as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            562 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*belowdiag).size != (*rhs).size {
        gsl_error(
            b"size of belowdiag must match rhs\0" as *const u8 as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            566 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*solution).size != (*rhs).size {
        gsl_error(
            b"size of solution must match rhs\0" as *const u8 as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            570 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if (*diag).size < 3 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"size of cyclic system must be 3 or more\0" as *const u8
                as *const libc::c_char,
            b"tridiag.c\0" as *const u8 as *const libc::c_char,
            574 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        return solve_cyc_tridiag_nonsym(
            (*diag).data as *const libc::c_double,
            (*diag).stride,
            (*abovediag).data as *const libc::c_double,
            (*abovediag).stride,
            (*belowdiag).data as *const libc::c_double,
            (*belowdiag).stride,
            (*rhs).data as *const libc::c_double,
            (*rhs).stride,
            (*solution).data,
            (*solution).stride,
            (*diag).size,
        )
    };
}
