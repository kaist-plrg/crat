use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPV {
    pub n: libc::c_int,
    pub nnz: libc::c_int,
    pub pos: *mut libc::c_int,
    pub ind: *mut libc::c_int,
    pub val: *mut libc::c_double,
}
pub unsafe extern "C" fn _glp_spv_create_vec(mut n: libc::c_int) -> *mut SPV {
    let mut v: *mut SPV = 0 as *mut SPV;
    (n >= 0 as libc::c_int
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const libc::c_char,
                b"intopt/spv.c\0" as *const u8 as *const libc::c_char,
                46 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    v = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<SPV>() as libc::c_ulong as libc::c_int,
    ) as *mut SPV;
    (*v).n = n;
    (*v).nnz = 0 as libc::c_int;
    (*v)
        .pos = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    memset(
        &mut *((*v).pos).offset(1 as libc::c_int as isize) as *mut libc::c_int
            as *mut libc::c_void,
        0 as libc::c_int,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    (*v)
        .ind = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*v)
        .val = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    return v;
}
pub unsafe extern "C" fn _glp_spv_check_vec(mut v: *mut SPV) {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nnz: libc::c_int = 0;
    ((*v).n >= 0 as libc::c_int
        || {
            glp_assert_(
                b"v->n >= 0\0" as *const u8 as *const libc::c_char,
                b"intopt/spv.c\0" as *const u8 as *const libc::c_char,
                78 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    nnz = 0 as libc::c_int;
    j = (*v).n;
    while j >= 1 as libc::c_int {
        k = *((*v).pos).offset(j as isize);
        (0 as libc::c_int <= k && k <= (*v).nnz
            || {
                glp_assert_(
                    b"0 <= k && k <= v->nnz\0" as *const u8 as *const libc::c_char,
                    b"intopt/spv.c\0" as *const u8 as *const libc::c_char,
                    82 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if k != 0 as libc::c_int {
            (*((*v).ind).offset(k as isize) == j
                || {
                    glp_assert_(
                        b"v->ind[k] == j\0" as *const u8 as *const libc::c_char,
                        b"intopt/spv.c\0" as *const u8 as *const libc::c_char,
                        84 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            nnz += 1;
            nnz;
        }
        j -= 1;
        j;
    }
    ((*v).nnz == nnz
        || {
            glp_assert_(
                b"v->nnz == nnz\0" as *const u8 as *const libc::c_char,
                b"intopt/spv.c\0" as *const u8 as *const libc::c_char,
                88 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
}
pub unsafe extern "C" fn _glp_spv_get_vj(
    mut v: *mut SPV,
    mut j: libc::c_int,
) -> libc::c_double {
    let mut k: libc::c_int = 0;
    (1 as libc::c_int <= j && j <= (*v).n
        || {
            glp_assert_(
                b"1 <= j && j <= v->n\0" as *const u8 as *const libc::c_char,
                b"intopt/spv.c\0" as *const u8 as *const libc::c_char,
                109 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = *((*v).pos).offset(j as isize);
    (0 as libc::c_int <= k && k <= (*v).nnz
        || {
            glp_assert_(
                b"0 <= k && k <= v->nnz\0" as *const u8 as *const libc::c_char,
                b"intopt/spv.c\0" as *const u8 as *const libc::c_char,
                111 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return if k == 0 as libc::c_int { 0.0f64 } else { *((*v).val).offset(k as isize) };
}
pub unsafe extern "C" fn _glp_spv_set_vj(
    mut v: *mut SPV,
    mut j: libc::c_int,
    mut val: libc::c_double,
) {
    let mut k: libc::c_int = 0;
    (1 as libc::c_int <= j && j <= (*v).n
        || {
            glp_assert_(
                b"1 <= j && j <= v->n\0" as *const u8 as *const libc::c_char,
                b"intopt/spv.c\0" as *const u8 as *const libc::c_char,
                132 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = *((*v).pos).offset(j as isize);
    if val == 0.0f64 {
        if k != 0 as libc::c_int {
            *((*v).pos).offset(j as isize) = 0 as libc::c_int;
            if k < (*v).nnz {
                *((*v).pos).offset(*((*v).ind).offset((*v).nnz as isize) as isize) = k;
                *((*v).ind).offset(k as isize) = *((*v).ind).offset((*v).nnz as isize);
                *((*v).val).offset(k as isize) = *((*v).val).offset((*v).nnz as isize);
            }
            (*v).nnz -= 1;
            (*v).nnz;
        }
    } else {
        if k == 0 as libc::c_int {
            (*v).nnz += 1;
            k = (*v).nnz;
            *((*v).pos).offset(j as isize) = k;
            *((*v).ind).offset(k as isize) = j;
        }
        *((*v).val).offset(k as isize) = val;
    };
}
pub unsafe extern "C" fn _glp_spv_clear_vec(mut v: *mut SPV) {
    let mut k: libc::c_int = 0;
    k = 1 as libc::c_int;
    while k <= (*v).nnz {
        *((*v).pos).offset(*((*v).ind).offset(k as isize) as isize) = 0 as libc::c_int;
        k += 1;
        k;
    }
    (*v).nnz = 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_spv_clean_vec(mut v: *mut SPV, mut eps: libc::c_double) {
    let mut k: libc::c_int = 0;
    let mut nnz: libc::c_int = 0;
    nnz = 0 as libc::c_int;
    k = 1 as libc::c_int;
    while k <= (*v).nnz {
        if fabs(*((*v).val).offset(k as isize)) == 0.0f64
            || fabs(*((*v).val).offset(k as isize)) < eps
        {
            *((*v).pos)
                .offset(*((*v).ind).offset(k as isize) as isize) = 0 as libc::c_int;
        } else {
            nnz += 1;
            nnz;
            *((*v).pos).offset(*((*v).ind).offset(k as isize) as isize) = nnz;
            *((*v).ind).offset(nnz as isize) = *((*v).ind).offset(k as isize);
            *((*v).val).offset(nnz as isize) = *((*v).val).offset(k as isize);
        }
        k += 1;
        k;
    }
    (*v).nnz = nnz;
}
pub unsafe extern "C" fn _glp_spv_copy_vec(mut x: *mut SPV, mut y: *mut SPV) {
    let mut j: libc::c_int = 0;
    (x != y
        || {
            glp_assert_(
                b"x != y\0" as *const u8 as *const libc::c_char,
                b"intopt/spv.c\0" as *const u8 as *const libc::c_char,
                234 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*x).n == (*y).n
        || {
            glp_assert_(
                b"x->n == y->n\0" as *const u8 as *const libc::c_char,
                b"intopt/spv.c\0" as *const u8 as *const libc::c_char,
                235 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_spv_clear_vec(x);
    (*x).nnz = (*y).nnz;
    memcpy(
        &mut *((*x).ind).offset(1 as libc::c_int as isize) as *mut libc::c_int
            as *mut libc::c_void,
        &mut *((*y).ind).offset(1 as libc::c_int as isize) as *mut libc::c_int
            as *const libc::c_void,
        ((*x).nnz as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    memcpy(
        &mut *((*x).val).offset(1 as libc::c_int as isize) as *mut libc::c_double
            as *mut libc::c_void,
        &mut *((*y).val).offset(1 as libc::c_int as isize) as *mut libc::c_double
            as *const libc::c_void,
        ((*x).nnz as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    j = 1 as libc::c_int;
    while j <= (*x).nnz {
        *((*x).pos).offset(*((*x).ind).offset(j as isize) as isize) = j;
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn _glp_spv_linear_comb(
    mut x: *mut SPV,
    mut a: libc::c_double,
    mut y: *mut SPV,
) {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut xj: libc::c_double = 0.;
    let mut yj: libc::c_double = 0.;
    (x != y
        || {
            glp_assert_(
                b"x != y\0" as *const u8 as *const libc::c_char,
                b"intopt/spv.c\0" as *const u8 as *const libc::c_char,
                266 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*x).n == (*y).n
        || {
            glp_assert_(
                b"x->n == y->n\0" as *const u8 as *const libc::c_char,
                b"intopt/spv.c\0" as *const u8 as *const libc::c_char,
                267 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = 1 as libc::c_int;
    while k <= (*y).nnz {
        j = *((*y).ind).offset(k as isize);
        xj = _glp_spv_get_vj(x, j);
        yj = *((*y).val).offset(k as isize);
        _glp_spv_set_vj(x, j, xj + a * yj);
        k += 1;
        k;
    }
}
pub unsafe extern "C" fn _glp_spv_delete_vec(mut v: *mut SPV) {
    glp_free((*v).pos as *mut libc::c_void);
    glp_free((*v).ind as *mut libc::c_void);
    glp_free((*v).val as *mut libc::c_void);
    glp_free(v as *mut libc::c_void);
}
