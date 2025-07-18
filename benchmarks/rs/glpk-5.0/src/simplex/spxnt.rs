use ::libc;
extern "C" {
    pub type BFD;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn _glp_fvs_clear_vec(x: *mut FVS);
    fn _glp_fvs_adjust_vec(x: *mut FVS, eps: libc::c_double);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FVS {
    pub n: libc::c_int,
    pub nnz: libc::c_int,
    pub ind: *mut libc::c_int,
    pub vec: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPXLP {
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub nnz: libc::c_int,
    pub A_ptr: *mut libc::c_int,
    pub A_ind: *mut libc::c_int,
    pub A_val: *mut libc::c_double,
    pub b: *mut libc::c_double,
    pub c: *mut libc::c_double,
    pub l: *mut libc::c_double,
    pub u: *mut libc::c_double,
    pub head: *mut libc::c_int,
    pub flag: *mut libc::c_char,
    pub valid: libc::c_int,
    pub bfd: *mut BFD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPXNT {
    pub ptr: *mut libc::c_int,
    pub len: *mut libc::c_int,
    pub ind: *mut libc::c_int,
    pub val: *mut libc::c_double,
}
pub unsafe extern "C" fn _glp_spx_alloc_nt(mut lp: *mut SPXLP, mut nt: *mut SPXNT) {
    let mut m: libc::c_int = (*lp).m;
    let mut nnz: libc::c_int = (*lp).nnz;
    (*nt)
        .ptr = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*nt)
        .len = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*nt)
        .ind = glp_alloc(
        1 as libc::c_int + nnz,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*nt)
        .val = glp_alloc(
        1 as libc::c_int + nnz,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
}
pub unsafe extern "C" fn _glp_spx_init_nt(mut lp: *mut SPXLP, mut nt: *mut SPXNT) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut nnz: libc::c_int = (*lp).nnz;
    let mut A_ptr: *mut libc::c_int = (*lp).A_ptr;
    let mut A_ind: *mut libc::c_int = (*lp).A_ind;
    let mut NT_ptr: *mut libc::c_int = (*nt).ptr;
    let mut NT_len: *mut libc::c_int = (*nt).len;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    memset(
        &mut *NT_len.offset(1 as libc::c_int as isize) as *mut libc::c_int
            as *mut libc::c_void,
        0 as libc::c_int,
        (m as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    k = 1 as libc::c_int;
    while k <= n {
        ptr = *A_ptr.offset(k as isize);
        end = *A_ptr.offset((k + 1 as libc::c_int) as isize);
        while ptr < end {
            let ref mut fresh0 = *NT_len.offset(*A_ind.offset(ptr as isize) as isize);
            *fresh0 += 1;
            *fresh0;
            ptr += 1;
            ptr;
        }
        k += 1;
        k;
    }
    *NT_ptr.offset(1 as libc::c_int as isize) = 1 as libc::c_int;
    i = 2 as libc::c_int;
    while i <= m {
        *NT_ptr
            .offset(
                i as isize,
            ) = *NT_ptr.offset((i - 1 as libc::c_int) as isize)
            + *NT_len.offset((i - 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    (*NT_ptr.offset(m as isize) + *NT_len.offset(m as isize) == nnz + 1 as libc::c_int
        || {
            glp_assert_(
                b"NT_ptr[m] + NT_len[m] == nnz+1\0" as *const u8 as *const libc::c_char,
                b"simplex/spxnt.c\0" as *const u8 as *const libc::c_char,
                71 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
}
pub unsafe extern "C" fn _glp_spx_nt_add_col(
    mut lp: *mut SPXLP,
    mut nt: *mut SPXNT,
    mut j: libc::c_int,
    mut k: libc::c_int,
) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut nnz: libc::c_int = (*lp).nnz;
    let mut A_ptr: *mut libc::c_int = (*lp).A_ptr;
    let mut A_ind: *mut libc::c_int = (*lp).A_ind;
    let mut A_val: *mut libc::c_double = (*lp).A_val;
    let mut NT_ptr: *mut libc::c_int = (*nt).ptr;
    let mut NT_len: *mut libc::c_int = (*nt).len;
    let mut NT_ind: *mut libc::c_int = (*nt).ind;
    let mut NT_val: *mut libc::c_double = (*nt).val;
    let mut i: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    (1 as libc::c_int <= j && j <= n - m
        || {
            glp_assert_(
                b"1 <= j && j <= n-m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxnt.c\0" as *const u8 as *const libc::c_char,
                95 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= k && k <= n
        || {
            glp_assert_(
                b"1 <= k && k <= n\0" as *const u8 as *const libc::c_char,
                b"simplex/spxnt.c\0" as *const u8 as *const libc::c_char,
                96 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ptr = *A_ptr.offset(k as isize);
    end = *A_ptr.offset((k + 1 as libc::c_int) as isize);
    while ptr < end {
        i = *A_ind.offset(ptr as isize);
        let ref mut fresh1 = *NT_len.offset(i as isize);
        let fresh2 = *fresh1;
        *fresh1 = *fresh1 + 1;
        pos = *NT_ptr.offset(i as isize) + fresh2;
        if i < m {
            (pos < *NT_ptr.offset((i + 1 as libc::c_int) as isize)
                || {
                    glp_assert_(
                        b"pos < NT_ptr[i+1]\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxnt.c\0" as *const u8 as *const libc::c_char,
                        104 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        } else {
            (pos <= nnz
                || {
                    glp_assert_(
                        b"pos <= nnz\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxnt.c\0" as *const u8 as *const libc::c_char,
                        106 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        *NT_ind.offset(pos as isize) = j;
        *NT_val.offset(pos as isize) = *A_val.offset(ptr as isize);
        ptr += 1;
        ptr;
    }
}
pub unsafe extern "C" fn _glp_spx_build_nt(mut lp: *mut SPXLP, mut nt: *mut SPXNT) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut NT_len: *mut libc::c_int = (*nt).len;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    memset(
        &mut *NT_len.offset(1 as libc::c_int as isize) as *mut libc::c_int
            as *mut libc::c_void,
        0 as libc::c_int,
        (m as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    j = 1 as libc::c_int;
    while j <= n - m {
        k = *head.offset((m + j) as isize);
        _glp_spx_nt_add_col(lp, nt, j, k);
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn _glp_spx_nt_del_col(
    mut lp: *mut SPXLP,
    mut nt: *mut SPXNT,
    mut j: libc::c_int,
    mut k: libc::c_int,
) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut A_ptr: *mut libc::c_int = (*lp).A_ptr;
    let mut A_ind: *mut libc::c_int = (*lp).A_ind;
    let mut NT_ptr: *mut libc::c_int = (*nt).ptr;
    let mut NT_len: *mut libc::c_int = (*nt).len;
    let mut NT_ind: *mut libc::c_int = (*nt).ind;
    let mut NT_val: *mut libc::c_double = (*nt).val;
    let mut i: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut ptr1: libc::c_int = 0;
    let mut end1: libc::c_int = 0;
    (1 as libc::c_int <= j && j <= n - m
        || {
            glp_assert_(
                b"1 <= j && j <= n-m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxnt.c\0" as *const u8 as *const libc::c_char,
                154 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= k && k <= n
        || {
            glp_assert_(
                b"1 <= k && k <= n\0" as *const u8 as *const libc::c_char,
                b"simplex/spxnt.c\0" as *const u8 as *const libc::c_char,
                155 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ptr = *A_ptr.offset(k as isize);
    end = *A_ptr.offset((k + 1 as libc::c_int) as isize);
    while ptr < end {
        i = *A_ind.offset(ptr as isize);
        ptr1 = *NT_ptr.offset(i as isize);
        end1 = ptr1 + *NT_len.offset(i as isize);
        while *NT_ind.offset(ptr1 as isize) != j {
            ptr1 += 1;
            ptr1;
        }
        (ptr1 < end1
            || {
                glp_assert_(
                    b"ptr1 < end1\0" as *const u8 as *const libc::c_char,
                    b"simplex/spxnt.c\0" as *const u8 as *const libc::c_char,
                    165 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        let ref mut fresh3 = *NT_len.offset(i as isize);
        *fresh3 -= 1;
        *fresh3;
        *NT_ind
            .offset(ptr1 as isize) = *NT_ind.offset((end1 - 1 as libc::c_int) as isize);
        *NT_val
            .offset(ptr1 as isize) = *NT_val.offset((end1 - 1 as libc::c_int) as isize);
        ptr += 1;
        ptr;
    }
}
pub unsafe extern "C" fn _glp_spx_update_nt(
    mut lp: *mut SPXLP,
    mut nt: *mut SPXNT,
    mut p: libc::c_int,
    mut q: libc::c_int,
) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut head: *mut libc::c_int = (*lp).head;
    (1 as libc::c_int <= p && p <= m
        || {
            glp_assert_(
                b"1 <= p && p <= m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxnt.c\0" as *const u8 as *const libc::c_char,
                185 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= q && q <= n - m
        || {
            glp_assert_(
                b"1 <= q && q <= n-m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxnt.c\0" as *const u8 as *const libc::c_char,
                186 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_spx_nt_del_col(lp, nt, q, *head.offset((m + q) as isize));
    _glp_spx_nt_add_col(lp, nt, q, *head.offset(p as isize));
}
pub unsafe extern "C" fn _glp_spx_nt_prod(
    mut lp: *mut SPXLP,
    mut nt: *mut SPXNT,
    mut y: *mut libc::c_double,
    mut ign: libc::c_int,
    mut s: libc::c_double,
    mut x: *const libc::c_double,
) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut NT_ptr: *mut libc::c_int = (*nt).ptr;
    let mut NT_len: *mut libc::c_int = (*nt).len;
    let mut NT_ind: *mut libc::c_int = (*nt).ind;
    let mut NT_val: *mut libc::c_double = (*nt).val;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut t: libc::c_double = 0.;
    if ign != 0 {
        j = 1 as libc::c_int;
        while j <= n - m {
            *y.offset(j as isize) = 0.0f64;
            j += 1;
            j;
        }
    }
    i = 1 as libc::c_int;
    while i <= m {
        if *x.offset(i as isize) != 0.0f64 {
            t = s * *x.offset(i as isize);
            ptr = *NT_ptr.offset(i as isize);
            end = ptr + *NT_len.offset(i as isize);
            while ptr < end {
                *y.offset(*NT_ind.offset(ptr as isize) as isize)
                    += *NT_val.offset(ptr as isize) * t;
                ptr += 1;
                ptr;
            }
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn _glp_spx_nt_prod_s(
    mut lp: *mut SPXLP,
    mut nt: *mut SPXNT,
    mut y: *mut FVS,
    mut ign: libc::c_int,
    mut s: libc::c_double,
    mut x: *const FVS,
    mut eps: libc::c_double,
) {
    let mut NT_ptr: *mut libc::c_int = (*nt).ptr;
    let mut NT_len: *mut libc::c_int = (*nt).len;
    let mut NT_ind: *mut libc::c_int = (*nt).ind;
    let mut NT_val: *mut libc::c_double = (*nt).val;
    let mut x_ind: *mut libc::c_int = (*x).ind;
    let mut x_vec: *mut libc::c_double = (*x).vec;
    let mut y_ind: *mut libc::c_int = (*y).ind;
    let mut y_vec: *mut libc::c_double = (*y).vec;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nnz: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut t: libc::c_double = 0.;
    ((*x).n == (*lp).m
        || {
            glp_assert_(
                b"x->n == lp->m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxnt.c\0" as *const u8 as *const libc::c_char,
                257 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*y).n == (*lp).n - (*lp).m
        || {
            glp_assert_(
                b"y->n == lp->n-lp->m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxnt.c\0" as *const u8 as *const libc::c_char,
                258 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if ign != 0 {
        _glp_fvs_clear_vec(y);
    }
    nnz = (*y).nnz;
    k = (*x).nnz;
    while k >= 1 as libc::c_int {
        i = *x_ind.offset(k as isize);
        t = s * *x_vec.offset(i as isize);
        ptr = *NT_ptr.offset(i as isize);
        end = ptr + *NT_len.offset(i as isize);
        while ptr < end {
            j = *NT_ind.offset(ptr as isize);
            if *y_vec.offset(j as isize) == 0.0f64 {
                nnz += 1;
                *y_ind.offset(nnz as isize) = j;
            }
            *y_vec.offset(j as isize) += *NT_val.offset(ptr as isize) * t;
            if *y_vec.offset(j as isize) == 0.0f64 {
                *y_vec.offset(j as isize) = 2.2250738585072014e-308f64;
            }
            ptr += 1;
            ptr;
        }
        k -= 1;
        k;
    }
    (*y).nnz = nnz;
    _glp_fvs_adjust_vec(y, eps);
}
pub unsafe extern "C" fn _glp_spx_free_nt(mut lp: *mut SPXLP, mut nt: *mut SPXNT) {
    (lp == lp
        || {
            glp_assert_(
                b"lp == lp\0" as *const u8 as *const libc::c_char,
                b"simplex/spxnt.c\0" as *const u8 as *const libc::c_char,
                293 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    glp_free((*nt).ptr as *mut libc::c_void);
    glp_free((*nt).len as *mut libc::c_void);
    glp_free((*nt).ind as *mut libc::c_void);
    glp_free((*nt).val as *mut libc::c_void);
}
