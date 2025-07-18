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
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
pub type gsl_linalg_matrix_mod_t = libc::c_uint;
pub const GSL_LINALG_MOD_CONJUGATE: gsl_linalg_matrix_mod_t = 2;
pub const GSL_LINALG_MOD_TRANSPOSE: gsl_linalg_matrix_mod_t = 1;
pub const GSL_LINALG_MOD_NONE: gsl_linalg_matrix_mod_t = 0;
#[inline]
unsafe extern "C" fn gsl_matrix_get(
    mut m: *const gsl_matrix,
    i: size_t,
    j: size_t,
) -> libc::c_double {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
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
pub unsafe extern "C" fn gsl_linalg_matmult(
    mut A: *const gsl_matrix,
    mut B: *const gsl_matrix,
    mut C: *mut gsl_matrix,
) -> libc::c_int {
    if (*A).size2 != (*B).size1 || (*A).size1 != (*C).size1 || (*B).size2 != (*C).size2 {
        gsl_error(
            b"matrix sizes are not conformant\0" as *const u8 as *const libc::c_char,
            b"multiply.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        let mut a: libc::c_double = 0.;
        let mut b: libc::c_double = 0.;
        let mut temp: libc::c_double = 0.;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < (*C).size1 {
            j = 0 as libc::c_int as size_t;
            while j < (*C).size2 {
                a = gsl_matrix_get(A, i, 0 as libc::c_int as size_t);
                b = gsl_matrix_get(B, 0 as libc::c_int as size_t, j);
                temp = a * b;
                k = 1 as libc::c_int as size_t;
                while k < (*A).size2 {
                    a = gsl_matrix_get(A, i, k);
                    b = gsl_matrix_get(B, k, j);
                    temp += a * b;
                    k = k.wrapping_add(1);
                    k;
                }
                gsl_matrix_set(C, i, j, temp);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_linalg_matmult_mod(
    mut A: *const gsl_matrix,
    mut modA: gsl_linalg_matrix_mod_t,
    mut B: *const gsl_matrix,
    mut modB: gsl_linalg_matrix_mod_t,
    mut C: *mut gsl_matrix,
) -> libc::c_int {
    if modA as libc::c_uint == GSL_LINALG_MOD_NONE as libc::c_int as libc::c_uint
        && modB as libc::c_uint == GSL_LINALG_MOD_NONE as libc::c_int as libc::c_uint
    {
        return gsl_linalg_matmult(A, B, C)
    } else {
        let mut dim1_A: size_t = (*A).size1;
        let mut dim2_A: size_t = (*A).size2;
        let mut dim1_B: size_t = (*B).size1;
        let mut dim2_B: size_t = (*B).size2;
        let mut dim1_C: size_t = (*C).size1;
        let mut dim2_C: size_t = (*C).size2;
        if modA as libc::c_uint & GSL_LINALG_MOD_TRANSPOSE as libc::c_int as libc::c_uint
            != 0
        {
            let mut swap_tmp: size_t = dim1_A;
            dim1_A = dim2_A;
            dim2_A = swap_tmp;
        }
        if modB as libc::c_uint & GSL_LINALG_MOD_TRANSPOSE as libc::c_int as libc::c_uint
            != 0
        {
            let mut swap_tmp_0: size_t = dim1_B;
            dim1_B = dim2_B;
            dim2_B = swap_tmp_0;
        }
        if dim2_A != dim1_B || dim1_A != dim1_C || dim2_B != dim2_C {
            gsl_error(
                b"matrix sizes are not conformant\0" as *const u8 as *const libc::c_char,
                b"multiply.c\0" as *const u8 as *const libc::c_char,
                89 as libc::c_int,
                GSL_EBADLEN as libc::c_int,
            );
            return GSL_EBADLEN as libc::c_int;
        } else {
            let mut a: libc::c_double = 0.;
            let mut b: libc::c_double = 0.;
            let mut temp: libc::c_double = 0.;
            let mut i: size_t = 0;
            let mut j: size_t = 0;
            let mut k: size_t = 0;
            let mut a1: size_t = 0;
            let mut a2: size_t = 0;
            let mut b1: size_t = 0;
            let mut b2: size_t = 0;
            i = 0 as libc::c_int as size_t;
            while i < dim1_C {
                j = 0 as libc::c_int as size_t;
                while j < dim2_C {
                    a1 = i;
                    a2 = 0 as libc::c_int as size_t;
                    b1 = 0 as libc::c_int as size_t;
                    b2 = j;
                    if modA as libc::c_uint
                        & GSL_LINALG_MOD_TRANSPOSE as libc::c_int as libc::c_uint != 0
                    {
                        let mut swap_tmp_1: size_t = a1;
                        a1 = a2;
                        a2 = swap_tmp_1;
                    }
                    if modB as libc::c_uint
                        & GSL_LINALG_MOD_TRANSPOSE as libc::c_int as libc::c_uint != 0
                    {
                        let mut swap_tmp_2: size_t = b1;
                        b1 = b2;
                        b2 = swap_tmp_2;
                    }
                    a = gsl_matrix_get(A, a1, a2);
                    b = gsl_matrix_get(B, b1, b2);
                    temp = a * b;
                    k = 1 as libc::c_int as size_t;
                    while k < dim2_A {
                        a1 = i;
                        a2 = k;
                        b1 = k;
                        b2 = j;
                        if modA as libc::c_uint
                            & GSL_LINALG_MOD_TRANSPOSE as libc::c_int as libc::c_uint
                            != 0
                        {
                            let mut swap_tmp_3: size_t = a1;
                            a1 = a2;
                            a2 = swap_tmp_3;
                        }
                        if modB as libc::c_uint
                            & GSL_LINALG_MOD_TRANSPOSE as libc::c_int as libc::c_uint
                            != 0
                        {
                            let mut swap_tmp_4: size_t = b1;
                            b1 = b2;
                            b2 = swap_tmp_4;
                        }
                        a = gsl_matrix_get(A, a1, a2);
                        b = gsl_matrix_get(B, b1, b2);
                        temp += a * b;
                        k = k.wrapping_add(1);
                        k;
                    }
                    gsl_matrix_set(C, i, j, temp);
                    j = j.wrapping_add(1);
                    j;
                }
                i = i.wrapping_add(1);
                i;
            }
            return GSL_SUCCESS as libc::c_int;
        }
    };
}
