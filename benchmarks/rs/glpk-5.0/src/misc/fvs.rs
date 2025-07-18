use ::libc;
extern "C" {
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FVS {
    pub n: libc::c_int,
    pub nnz: libc::c_int,
    pub ind: *mut libc::c_int,
    pub vec: *mut libc::c_double,
}
pub unsafe extern "C" fn _glp_fvs_alloc_vec(mut x: *mut FVS, mut n: libc::c_int) {
    let mut j: libc::c_int = 0;
    (n >= 0 as libc::c_int
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const libc::c_char,
                b"misc/fvs.c\0" as *const u8 as *const libc::c_char,
                28 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*x).n = n;
    (*x).nnz = 0 as libc::c_int;
    (*x)
        .ind = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*x)
        .vec = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    j = 1 as libc::c_int;
    while j <= n {
        *((*x).vec).offset(j as isize) = 0.0f64;
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn _glp_fvs_check_vec(mut x: *const FVS) {
    let mut n: libc::c_int = (*x).n;
    let mut nnz: libc::c_int = (*x).nnz;
    let mut ind: *mut libc::c_int = (*x).ind;
    let mut vec: *mut libc::c_double = (*x).vec;
    let mut map: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    (n >= 0 as libc::c_int
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const libc::c_char,
                b"misc/fvs.c\0" as *const u8 as *const libc::c_char,
                47 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (0 as libc::c_int <= nnz && nnz <= n
        || {
            glp_assert_(
                b"0 <= nnz && nnz <= n\0" as *const u8 as *const libc::c_char,
                b"misc/fvs.c\0" as *const u8 as *const libc::c_char,
                48 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    map = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    j = 1 as libc::c_int;
    while j <= n {
        *map
            .offset(
                j as isize,
            ) = (*vec.offset(j as isize) != 0.0f64) as libc::c_int as libc::c_char;
        j += 1;
        j;
    }
    k = 1 as libc::c_int;
    while k <= nnz {
        j = *ind.offset(k as isize);
        (1 as libc::c_int <= j && j <= n
            || {
                glp_assert_(
                    b"1 <= j && j <= n\0" as *const u8 as *const libc::c_char,
                    b"misc/fvs.c\0" as *const u8 as *const libc::c_char,
                    54 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*map.offset(j as isize) as libc::c_int != 0
            || {
                glp_assert_(
                    b"map[j]\0" as *const u8 as *const libc::c_char,
                    b"misc/fvs.c\0" as *const u8 as *const libc::c_char,
                    55 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        *map.offset(j as isize) = 0 as libc::c_int as libc::c_char;
        k += 1;
        k;
    }
    j = 1 as libc::c_int;
    while j <= n {
        (*map.offset(j as isize) == 0
            || {
                glp_assert_(
                    b"!map[j]\0" as *const u8 as *const libc::c_char,
                    b"misc/fvs.c\0" as *const u8 as *const libc::c_char,
                    59 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        j += 1;
        j;
    }
    glp_free(map as *mut libc::c_void);
}
pub unsafe extern "C" fn _glp_fvs_gather_vec(mut x: *mut FVS, mut eps: libc::c_double) {
    let mut n: libc::c_int = (*x).n;
    let mut ind: *mut libc::c_int = (*x).ind;
    let mut vec: *mut libc::c_double = (*x).vec;
    let mut j: libc::c_int = 0;
    let mut nnz: libc::c_int = 0 as libc::c_int;
    j = n;
    while j >= 1 as libc::c_int {
        if -eps < *vec.offset(j as isize) && *vec.offset(j as isize) < eps {
            *vec.offset(j as isize) = 0.0f64;
        } else {
            nnz += 1;
            *ind.offset(nnz as isize) = j;
        }
        j -= 1;
        j;
    }
    (*x).nnz = nnz;
}
pub unsafe extern "C" fn _glp_fvs_clear_vec(mut x: *mut FVS) {
    let mut ind: *mut libc::c_int = (*x).ind;
    let mut vec: *mut libc::c_double = (*x).vec;
    let mut k: libc::c_int = 0;
    k = (*x).nnz;
    while k >= 1 as libc::c_int {
        *vec.offset(*ind.offset(k as isize) as isize) = 0.0f64;
        k -= 1;
        k;
    }
    (*x).nnz = 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_fvs_copy_vec(mut x: *mut FVS, mut y: *const FVS) {
    let mut x_ind: *mut libc::c_int = (*x).ind;
    let mut x_vec: *mut libc::c_double = (*x).vec;
    let mut y_ind: *mut libc::c_int = (*y).ind;
    let mut y_vec: *mut libc::c_double = (*y).vec;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    (x != y as *mut FVS
        || {
            glp_assert_(
                b"x != y\0" as *const u8 as *const libc::c_char,
                b"misc/fvs.c\0" as *const u8 as *const libc::c_char,
                98 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*x).n == (*y).n
        || {
            glp_assert_(
                b"x->n == y->n\0" as *const u8 as *const libc::c_char,
                b"misc/fvs.c\0" as *const u8 as *const libc::c_char,
                99 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_fvs_clear_vec(x);
    (*x).nnz = (*y).nnz;
    k = (*x).nnz;
    while k >= 1 as libc::c_int {
        let ref mut fresh0 = *x_ind.offset(k as isize);
        *fresh0 = *y_ind.offset(k as isize);
        j = *fresh0;
        *x_vec.offset(j as isize) = *y_vec.offset(j as isize);
        k -= 1;
        k;
    }
}
pub unsafe extern "C" fn _glp_fvs_adjust_vec(mut x: *mut FVS, mut eps: libc::c_double) {
    let mut nnz: libc::c_int = (*x).nnz;
    let mut ind: *mut libc::c_int = (*x).ind;
    let mut vec: *mut libc::c_double = (*x).vec;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    k = 1 as libc::c_int;
    while k <= nnz {
        j = *ind.offset(k as isize);
        if -eps < *vec.offset(j as isize) && *vec.offset(j as isize) < eps {
            *vec.offset(j as isize) = 0.0f64;
        } else {
            cnt += 1;
            *ind.offset(cnt as isize) = j;
        }
        k += 1;
        k;
    }
    (*x).nnz = cnt;
}
pub unsafe extern "C" fn _glp_fvs_free_vec(mut x: *mut FVS) {
    glp_free((*x).ind as *mut libc::c_void);
    glp_free((*x).vec as *mut libc::c_void);
    (*x).nnz = -(1 as libc::c_int);
    (*x).n = (*x).nnz;
    (*x).ind = 0 as *mut libc::c_int;
    (*x).vec = 0 as *mut libc::c_double;
}
