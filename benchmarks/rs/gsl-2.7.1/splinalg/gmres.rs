use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_subvector(
        v: *mut gsl_vector,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_vector_set_zero(v: *mut gsl_vector);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_add(a: *mut gsl_vector, b: *const gsl_vector) -> libc::c_int;
    fn gsl_vector_scale(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_column(m: *mut gsl_matrix, j: size_t) -> _gsl_vector_view;
    fn gsl_matrix_subcolumn(
        m: *mut gsl_matrix,
        j: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_matrix_set_zero(m: *mut gsl_matrix);
    fn gsl_linalg_householder_transform(v: *mut gsl_vector) -> libc::c_double;
    fn gsl_linalg_householder_hv(
        tau: libc::c_double,
        v: *const gsl_vector,
        w: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> libc::c_double;
    fn gsl_blas_dtrsv(
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        A: *const gsl_matrix,
        X: *mut gsl_vector,
    ) -> libc::c_int;
    fn gsl_spblas_dgemv(
        TransA: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_spmatrix,
        x: *const gsl_vector,
        beta: libc::c_double,
        y: *mut gsl_vector,
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
pub type CBLAS_TRANSPOSE = libc::c_uint;
pub const CblasConjTrans: CBLAS_TRANSPOSE = 113;
pub const CblasTrans: CBLAS_TRANSPOSE = 112;
pub const CblasNoTrans: CBLAS_TRANSPOSE = 111;
pub type CBLAS_UPLO = libc::c_uint;
pub const CblasLower: CBLAS_UPLO = 122;
pub const CblasUpper: CBLAS_UPLO = 121;
pub type CBLAS_DIAG = libc::c_uint;
pub const CblasUnit: CBLAS_DIAG = 132;
pub const CblasNonUnit: CBLAS_DIAG = 131;
pub type CBLAS_TRANSPOSE_t = CBLAS_TRANSPOSE;
pub type CBLAS_UPLO_t = CBLAS_UPLO;
pub type CBLAS_DIAG_t = CBLAS_DIAG;
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
pub struct gsl_spmatrix_pool_node {
    pub next: *mut gsl_spmatrix_pool_node,
    pub block_ptr: *mut libc::c_void,
    pub free_slot: *mut libc::c_uchar,
}
pub type gsl_spmatrix_pool = gsl_spmatrix_pool_node;
pub type gsl_bst_cmp_function = unsafe extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_allocator {
    pub alloc: Option::<
        unsafe extern "C" fn(size_t, *mut libc::c_void) -> *mut libc::c_void,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_avl_node {
    pub avl_link: [*mut gsl_bst_avl_node; 2],
    pub avl_data: *mut libc::c_void,
    pub avl_balance: libc::c_schar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_avl_table {
    pub avl_root: *mut gsl_bst_avl_node,
    pub avl_compare: Option::<gsl_bst_cmp_function>,
    pub avl_param: *mut libc::c_void,
    pub avl_alloc: *const gsl_bst_allocator,
    pub avl_count: size_t,
    pub avl_generation: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_rb_node {
    pub rb_link: [*mut gsl_bst_rb_node; 2],
    pub rb_data: *mut libc::c_void,
    pub rb_color: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_rb_table {
    pub rb_root: *mut gsl_bst_rb_node,
    pub rb_compare: Option::<gsl_bst_cmp_function>,
    pub rb_param: *mut libc::c_void,
    pub rb_alloc: *const gsl_bst_allocator,
    pub rb_count: size_t,
    pub rb_generation: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_type {
    pub name: *const libc::c_char,
    pub node_size: size_t,
    pub init: Option::<
        unsafe extern "C" fn(
            *const gsl_bst_allocator,
            Option::<gsl_bst_cmp_function>,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub nodes: Option::<unsafe extern "C" fn(*const libc::c_void) -> size_t>,
    pub insert: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
    >,
    pub find: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_void,
        ) -> *mut libc::c_void,
    >,
    pub remove: Option::<
        unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
    >,
    pub empty: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub trav_init: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
    pub trav_first: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void) -> *mut libc::c_void,
    >,
    pub trav_last: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void) -> *mut libc::c_void,
    >,
    pub trav_find: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            *mut libc::c_void,
            *const libc::c_void,
        ) -> *mut libc::c_void,
    >,
    pub trav_insert: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> *mut libc::c_void,
    >,
    pub trav_copy: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void) -> *mut libc::c_void,
    >,
    pub trav_next: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
    >,
    pub trav_prev: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
    >,
    pub trav_cur: Option::<
        unsafe extern "C" fn(*const libc::c_void) -> *mut libc::c_void,
    >,
    pub trav_replace: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_workspace {
    pub type_0: *const gsl_bst_type,
    pub table: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub avl_table: gsl_bst_avl_table,
    pub rb_table: gsl_bst_rb_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_spmatrix {
    pub size1: size_t,
    pub size2: size_t,
    pub i: *mut libc::c_int,
    pub data: *mut libc::c_double,
    pub p: *mut libc::c_int,
    pub nzmax: size_t,
    pub nz: size_t,
    pub tree: *mut gsl_bst_workspace,
    pub pool: *mut gsl_spmatrix_pool,
    pub node_size: size_t,
    pub work: C2RustUnnamed_1,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub work_void: *mut libc::c_void,
    pub work_int: *mut libc::c_int,
    pub work_atomic: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_splinalg_itersolve_type {
    pub name: *const libc::c_char,
    pub alloc: Option::<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>,
    pub iterate: Option::<
        unsafe extern "C" fn(
            *const gsl_spmatrix,
            *const gsl_vector,
            libc::c_double,
            *mut gsl_vector,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub normr: Option::<unsafe extern "C" fn(*const libc::c_void) -> libc::c_double>,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gmres_state_t {
    pub n: size_t,
    pub m: size_t,
    pub r: *mut gsl_vector,
    pub H: *mut gsl_matrix,
    pub tau: *mut gsl_vector,
    pub y: *mut gsl_vector,
    pub c: *mut libc::c_double,
    pub s: *mut libc::c_double,
    pub normr: libc::c_double,
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
unsafe extern "C" fn gsl_linalg_givens_gv(
    mut v: *mut gsl_vector,
    i: size_t,
    j: size_t,
    c: libc::c_double,
    s: libc::c_double,
) {
    let mut vi: libc::c_double = gsl_vector_get(v, i);
    let mut vj: libc::c_double = gsl_vector_get(v, j);
    gsl_vector_set(v, i, c * vi - s * vj);
    gsl_vector_set(v, j, s * vi + c * vj);
}
#[inline]
unsafe extern "C" fn gsl_linalg_givens(
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
unsafe extern "C" fn gmres_alloc(n: size_t, m: size_t) -> *mut libc::c_void {
    let mut state: *mut gmres_state_t = 0 as *mut gmres_state_t;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"matrix dimension n must be a positive integer\0" as *const u8
                as *const libc::c_char,
            b"gmres.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    state = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gmres_state_t>() as libc::c_ulong,
    ) as *mut gmres_state_t;
    if state.is_null() {
        gsl_error(
            b"failed to allocate gmres state\0" as *const u8 as *const libc::c_char,
            b"gmres.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).n = n;
    if m == 0 as libc::c_int as libc::c_ulong {
        (*state)
            .m = if n < 10 as libc::c_int as libc::c_ulong {
            n
        } else {
            10 as libc::c_int as libc::c_ulong
        };
    } else {
        (*state).m = if n < m { n } else { m };
    }
    (*state).r = gsl_vector_alloc(n);
    if ((*state).r).is_null() {
        gmres_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate r vector\0" as *const u8 as *const libc::c_char,
            b"gmres.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .H = gsl_matrix_alloc(
        n,
        ((*state).m).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    if ((*state).H).is_null() {
        gmres_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate H matrix\0" as *const u8 as *const libc::c_char,
            b"gmres.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .tau = gsl_vector_alloc(
        ((*state).m).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    if ((*state).tau).is_null() {
        gmres_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate tau vector\0" as *const u8 as *const libc::c_char,
            b"gmres.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .y = gsl_vector_alloc(
        ((*state).m).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    if ((*state).y).is_null() {
        gmres_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate y vector\0" as *const u8 as *const libc::c_char,
            b"gmres.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state)
        .c = malloc(
        ((*state).m)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    (*state)
        .s = malloc(
        ((*state).m)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*state).c).is_null() || ((*state).s).is_null() {
        gmres_free(state as *mut libc::c_void);
        gsl_error(
            b"failed to allocate Givens vectors\0" as *const u8 as *const libc::c_char,
            b"gmres.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut libc::c_void;
    }
    (*state).normr = 0.0f64;
    return state as *mut libc::c_void;
}
unsafe extern "C" fn gmres_free(mut vstate: *mut libc::c_void) {
    let mut state: *mut gmres_state_t = vstate as *mut gmres_state_t;
    if !((*state).r).is_null() {
        gsl_vector_free((*state).r);
    }
    if !((*state).H).is_null() {
        gsl_matrix_free((*state).H);
    }
    if !((*state).tau).is_null() {
        gsl_vector_free((*state).tau);
    }
    if !((*state).y).is_null() {
        gsl_vector_free((*state).y);
    }
    if !((*state).c).is_null() {
        free((*state).c as *mut libc::c_void);
    }
    if !((*state).s).is_null() {
        free((*state).s as *mut libc::c_void);
    }
    free(state as *mut libc::c_void);
}
unsafe extern "C" fn gmres_iterate(
    mut A: *const gsl_spmatrix,
    mut b: *const gsl_vector,
    tol: libc::c_double,
    mut x: *mut gsl_vector,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let N: size_t = (*A).size1;
    let mut state: *mut gmres_state_t = vstate as *mut gmres_state_t;
    if N != (*A).size2 {
        gsl_error(
            b"matrix must be square\0" as *const u8 as *const libc::c_char,
            b"gmres.c\0" as *const u8 as *const libc::c_char,
            204 as libc::c_int,
            GSL_ENOTSQR as libc::c_int,
        );
        return GSL_ENOTSQR as libc::c_int;
    } else if N != (*b).size {
        gsl_error(
            b"matrix does not match right hand side\0" as *const u8
                as *const libc::c_char,
            b"gmres.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N != (*x).size {
        gsl_error(
            b"matrix does not match solution vector\0" as *const u8
                as *const libc::c_char,
            b"gmres.c\0" as *const u8 as *const libc::c_char,
            212 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if N != (*state).n {
        gsl_error(
            b"matrix does not match workspace\0" as *const u8 as *const libc::c_char,
            b"gmres.c\0" as *const u8 as *const libc::c_char,
            216 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut status: libc::c_int = GSL_SUCCESS as libc::c_int;
        let maxit: size_t = (*state).m;
        let normb: libc::c_double = gsl_blas_dnrm2(b);
        let reltol: libc::c_double = tol * normb;
        let mut normr: libc::c_double = 0.;
        let mut m: size_t = 0;
        let mut k: size_t = 0;
        let mut tau: libc::c_double = 0.;
        let mut H: *mut gsl_matrix = (*state).H;
        let mut r: *mut gsl_vector = (*state).r;
        let mut w: *mut gsl_vector = (*state).y;
        let mut Rm: gsl_matrix_view = gsl_matrix_view {
            matrix: gsl_matrix {
                size1: 0,
                size2: 0,
                tda: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        let mut ym: gsl_vector_view = gsl_vector_view {
            vector: gsl_vector {
                size: 0,
                stride: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        let mut h0: gsl_vector_view = gsl_matrix_column(H, 0 as libc::c_int as size_t);
        gsl_matrix_set_zero(H);
        gsl_vector_memcpy(r, b);
        gsl_spblas_dgemv(CblasNoTrans, -1.0f64, A, x, 1.0f64, r);
        gsl_vector_memcpy(&mut h0.vector, r);
        tau = gsl_linalg_householder_transform(&mut h0.vector);
        gsl_vector_set((*state).tau, 0 as libc::c_int as size_t, tau);
        gsl_vector_set_zero(w);
        gsl_vector_set(
            w,
            0 as libc::c_int as size_t,
            gsl_vector_get(&mut h0.vector, 0 as libc::c_int as size_t),
        );
        m = 1 as libc::c_int as size_t;
        while m <= maxit {
            let mut j: size_t = m.wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let mut c: libc::c_double = 0.;
            let mut s: libc::c_double = 0.;
            let mut vm: gsl_vector_view = gsl_matrix_column(H, m);
            let mut vv: gsl_vector_view = gsl_vector_subvector(
                &mut vm.vector,
                j,
                N.wrapping_sub(j),
            );
            let mut um: gsl_vector_view = gsl_matrix_subcolumn(
                H,
                j,
                j,
                N.wrapping_sub(j),
            );
            gsl_vector_set_zero(&mut vm.vector);
            gsl_vector_memcpy(&mut vv.vector, &mut um.vector);
            tau = gsl_vector_get((*state).tau, j);
            gsl_vector_scale(&mut vv.vector, -tau);
            gsl_vector_set(&mut vv.vector, 0 as libc::c_int as size_t, 1.0f64 - tau);
            k = j;
            while k > 0 as libc::c_int as libc::c_ulong
                && {
                    let fresh0 = k;
                    k = k.wrapping_sub(1);
                    fresh0 != 0
                }
            {
                let mut uk: gsl_vector_view = gsl_matrix_subcolumn(
                    H,
                    k,
                    k,
                    N.wrapping_sub(k),
                );
                let mut vk: gsl_vector_view = gsl_vector_subvector(
                    &mut vm.vector,
                    k,
                    N.wrapping_sub(k),
                );
                tau = gsl_vector_get((*state).tau, k);
                gsl_linalg_householder_hv(tau, &mut uk.vector, &mut vk.vector);
            }
            gsl_spblas_dgemv(CblasNoTrans, 1.0f64, A, &mut vm.vector, 0.0f64, r);
            gsl_vector_memcpy(&mut vm.vector, r);
            k = 0 as libc::c_int as size_t;
            while k <= j {
                let mut uk_0: gsl_vector_view = gsl_matrix_subcolumn(
                    H,
                    k,
                    k,
                    N.wrapping_sub(k),
                );
                let mut vk_0: gsl_vector_view = gsl_vector_subvector(
                    &mut vm.vector,
                    k,
                    N.wrapping_sub(k),
                );
                tau = gsl_vector_get((*state).tau, k);
                gsl_linalg_householder_hv(tau, &mut uk_0.vector, &mut vk_0.vector);
                k = k.wrapping_add(1);
                k;
            }
            if m < N {
                let mut ump1: gsl_vector_view = gsl_matrix_subcolumn(
                    H,
                    m,
                    m,
                    N.wrapping_sub(m),
                );
                tau = gsl_linalg_householder_transform(&mut ump1.vector);
                gsl_vector_set(
                    (*state).tau,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    tau,
                );
            }
            k = 0 as libc::c_int as size_t;
            while k < j {
                gsl_linalg_givens_gv(
                    &mut vm.vector,
                    k,
                    k.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    *((*state).c).offset(k as isize),
                    *((*state).s).offset(k as isize),
                );
                k = k.wrapping_add(1);
                k;
            }
            if m < N {
                gsl_linalg_givens(
                    gsl_vector_get(&mut vm.vector, j),
                    gsl_vector_get(
                        &mut vm.vector,
                        j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ),
                    &mut c,
                    &mut s,
                );
                *((*state).c).offset(j as isize) = c;
                *((*state).s).offset(j as isize) = s;
                gsl_linalg_givens_gv(
                    &mut vm.vector,
                    j,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    c,
                    s,
                );
                gsl_linalg_givens_gv(
                    w,
                    j,
                    j.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    c,
                    s,
                );
            }
            normr = fabs(
                gsl_vector_get(w, j.wrapping_add(1 as libc::c_int as libc::c_ulong)),
            );
            if normr <= reltol {
                break;
            }
            m = m.wrapping_add(1);
            m;
        }
        if m > maxit {
            m = m.wrapping_sub(1);
            m;
        }
        Rm = gsl_matrix_submatrix(
            H,
            0 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
            m,
            m,
        );
        ym = gsl_vector_subvector(w, 0 as libc::c_int as size_t, m);
        gsl_blas_dtrsv(
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            &mut Rm.matrix,
            &mut ym.vector,
        );
        gsl_vector_set_zero(r);
        k = m;
        while k > 0 as libc::c_int as libc::c_ulong
            && {
                let fresh1 = k;
                k = k.wrapping_sub(1);
                fresh1 != 0
            }
        {
            let mut ymk: libc::c_double = gsl_vector_get(&mut ym.vector, k);
            let mut uk_1: gsl_vector_view = gsl_matrix_subcolumn(
                H,
                k,
                k,
                N.wrapping_sub(k),
            );
            let mut rk: gsl_vector_view = gsl_vector_subvector(r, k, N.wrapping_sub(k));
            gsl_vector_set(r, k, gsl_vector_get(r, k) + ymk);
            tau = gsl_vector_get((*state).tau, k);
            gsl_linalg_householder_hv(tau, &mut uk_1.vector, &mut rk.vector);
        }
        gsl_vector_add(x, r);
        gsl_vector_memcpy(r, b);
        gsl_spblas_dgemv(CblasNoTrans, -1.0f64, A, x, 1.0f64, r);
        normr = gsl_blas_dnrm2(r);
        if normr <= reltol {
            status = GSL_SUCCESS as libc::c_int;
        } else {
            status = GSL_CONTINUE as libc::c_int;
        }
        (*state).normr = normr;
        return status;
    };
}
unsafe extern "C" fn gmres_normr(mut vstate: *const libc::c_void) -> libc::c_double {
    let mut state: *const gmres_state_t = vstate as *const gmres_state_t;
    return (*state).normr;
}
static mut gmres_type: gsl_splinalg_itersolve_type = {
    let mut init = gsl_splinalg_itersolve_type {
        name: b"gmres\0" as *const u8 as *const libc::c_char,
        alloc: Some(
            gmres_alloc as unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void,
        ),
        iterate: Some(
            gmres_iterate
                as unsafe extern "C" fn(
                    *const gsl_spmatrix,
                    *const gsl_vector,
                    libc::c_double,
                    *mut gsl_vector,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        normr: Some(
            gmres_normr as unsafe extern "C" fn(*const libc::c_void) -> libc::c_double,
        ),
        free: Some(gmres_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    };
    init
};
pub static mut gsl_splinalg_itersolve_gmres: *const gsl_splinalg_itersolve_type = unsafe {
    &gmres_type as *const gsl_splinalg_itersolve_type
};
