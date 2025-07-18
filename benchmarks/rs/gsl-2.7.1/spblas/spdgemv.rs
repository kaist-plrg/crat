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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const GSL_SPMATRIX_CRS: C2RustUnnamed_0 = 2;
pub const GSL_SPMATRIX_CCS: C2RustUnnamed_0 = 1;
pub const GSL_SPMATRIX_TRIPLET: C2RustUnnamed_0 = 0;
pub const GSL_SPMATRIX_CSR: C2RustUnnamed_0 = 2;
pub const GSL_SPMATRIX_CSC: C2RustUnnamed_0 = 1;
pub const GSL_SPMATRIX_COO: C2RustUnnamed_0 = 0;
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
    pub table: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub avl_table: gsl_bst_avl_table,
    pub rb_table: gsl_bst_rb_table,
}
pub type CBLAS_TRANSPOSE = libc::c_uint;
pub const CblasConjTrans: CBLAS_TRANSPOSE = 113;
pub const CblasTrans: CBLAS_TRANSPOSE = 112;
pub const CblasNoTrans: CBLAS_TRANSPOSE = 111;
pub type CBLAS_TRANSPOSE_t = CBLAS_TRANSPOSE;
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
    pub work: C2RustUnnamed_2,
    pub sptype: libc::c_int,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub work_void: *mut libc::c_void,
    pub work_int: *mut libc::c_int,
    pub work_atomic: *mut libc::c_double,
}
pub unsafe extern "C" fn gsl_spblas_dgemv(
    TransA: CBLAS_TRANSPOSE_t,
    alpha: libc::c_double,
    mut A: *const gsl_spmatrix,
    mut x: *const gsl_vector,
    beta: libc::c_double,
    mut y: *mut gsl_vector,
) -> libc::c_int {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if TransA as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint
        && N != (*x).size
        || TransA as libc::c_uint == CblasTrans as libc::c_int as libc::c_uint
            && M != (*x).size
    {
        gsl_error(
            b"invalid length of x vector\0" as *const u8 as *const libc::c_char,
            b"spdgemv.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if TransA as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint
        && M != (*y).size
        || TransA as libc::c_uint == CblasTrans as libc::c_int as libc::c_uint
            && N != (*y).size
    {
        gsl_error(
            b"invalid length of y vector\0" as *const u8 as *const libc::c_char,
            b"spdgemv.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut j: size_t = 0;
        let mut incX: size_t = 0;
        let mut incY: size_t = 0;
        let mut lenX: size_t = 0;
        let mut lenY: size_t = 0;
        let mut X: *mut libc::c_double = 0 as *mut libc::c_double;
        let mut Y: *mut libc::c_double = 0 as *mut libc::c_double;
        let mut Ad: *mut libc::c_double = 0 as *mut libc::c_double;
        let mut Ap: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Ai: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut Aj: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut p: libc::c_int = 0;
        if TransA as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint {
            lenX = N;
            lenY = M;
        } else {
            lenX = M;
            lenY = N;
        }
        Y = (*y).data;
        incY = (*y).stride;
        if beta == 0.0f64 {
            let mut jy: size_t = 0 as libc::c_int as size_t;
            j = 0 as libc::c_int as size_t;
            while j < lenY {
                *Y.offset(jy as isize) = 0.0f64;
                jy = (jy as libc::c_ulong).wrapping_add(incY) as size_t as size_t;
                j = j.wrapping_add(1);
                j;
            }
        } else if beta != 1.0f64 {
            let mut jy_0: size_t = 0 as libc::c_int as size_t;
            j = 0 as libc::c_int as size_t;
            while j < lenY {
                *Y.offset(jy_0 as isize) *= beta;
                jy_0 = (jy_0 as libc::c_ulong).wrapping_add(incY) as size_t as size_t;
                j = j.wrapping_add(1);
                j;
            }
        }
        if alpha == 0.0f64 {
            return GSL_SUCCESS as libc::c_int;
        }
        Ap = (*A).p;
        Ad = (*A).data;
        X = (*x).data;
        incX = (*x).stride;
        if (*A).sptype == GSL_SPMATRIX_CSC as libc::c_int
            && TransA as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint
            || (*A).sptype == GSL_SPMATRIX_CSR as libc::c_int
                && TransA as libc::c_uint == CblasTrans as libc::c_int as libc::c_uint
        {
            Ai = (*A).i;
            j = 0 as libc::c_int as size_t;
            while j < lenX {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *Y
                        .offset(
                            (*Ai.offset(p as isize) as libc::c_ulong).wrapping_mul(incY)
                                as isize,
                        )
                        += alpha * *Ad.offset(p as isize)
                            * *X.offset(j.wrapping_mul(incX) as isize);
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*A).sptype == GSL_SPMATRIX_CSC as libc::c_int
            && TransA as libc::c_uint == CblasTrans as libc::c_int as libc::c_uint
            || (*A).sptype == GSL_SPMATRIX_CSR as libc::c_int
                && TransA as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint
        {
            Ai = (*A).i;
            j = 0 as libc::c_int as size_t;
            while j < lenY {
                p = *Ap.offset(j as isize);
                while p
                    < *Ap
                        .offset(
                            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        )
                {
                    *Y.offset(j.wrapping_mul(incY) as isize)
                        += alpha * *Ad.offset(p as isize)
                            * *X
                                .offset(
                                    (*Ai.offset(p as isize) as libc::c_ulong).wrapping_mul(incX)
                                        as isize,
                                );
                    p += 1;
                    p;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else if (*A).sptype == GSL_SPMATRIX_COO as libc::c_int {
            if TransA as libc::c_uint == CblasNoTrans as libc::c_int as libc::c_uint {
                Ai = (*A).i;
                Aj = (*A).p;
            } else {
                Ai = (*A).p;
                Aj = (*A).i;
            }
            p = 0 as libc::c_int;
            while p < (*A).nz as libc::c_int {
                *Y
                    .offset(
                        (*Ai.offset(p as isize) as libc::c_ulong).wrapping_mul(incY)
                            as isize,
                    )
                    += alpha * *Ad.offset(p as isize)
                        * *X
                            .offset(
                                (*Aj.offset(p as isize) as libc::c_ulong).wrapping_mul(incX)
                                    as isize,
                            );
                p += 1;
                p;
            }
        } else {
            gsl_error(
                b"unsupported matrix type\0" as *const u8 as *const libc::c_char,
                b"spdgemv.c\0" as *const u8 as *const libc::c_char,
                162 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
