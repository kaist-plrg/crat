use ::libc;
extern "C" {
    pub type DMP;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn _glp_hbm_read_mat(fname: *const libc::c_char) -> *mut HBM;
    fn _glp_hbm_free_mat(hbm: *mut HBM);
    fn _glp_rgr_write_bmp16(
        fname: *const libc::c_char,
        m: libc::c_int,
        n: libc::c_int,
        map: *const libc::c_char,
    ) -> libc::c_int;
    fn _glp_dmp_get_atom(pool: *mut DMP, size: libc::c_int) -> *mut libc::c_void;
    fn _glp_dmp_create_pool() -> *mut DMP;
    fn _glp_dmp_free_atom(pool: *mut DMP, atom: *mut libc::c_void, size: libc::c_int);
    fn _glp_dmp_delete_pool(pool: *mut DMP);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HBM {
    pub title: [libc::c_char; 73],
    pub key: [libc::c_char; 9],
    pub mxtype: [libc::c_char; 4],
    pub rhstyp: [libc::c_char; 4],
    pub ptrfmt: [libc::c_char; 17],
    pub indfmt: [libc::c_char; 17],
    pub valfmt: [libc::c_char; 21],
    pub rhsfmt: [libc::c_char; 21],
    pub totcrd: libc::c_int,
    pub ptrcrd: libc::c_int,
    pub indcrd: libc::c_int,
    pub valcrd: libc::c_int,
    pub rhscrd: libc::c_int,
    pub nrow: libc::c_int,
    pub ncol: libc::c_int,
    pub nnzero: libc::c_int,
    pub neltvl: libc::c_int,
    pub nrhs: libc::c_int,
    pub nrhsix: libc::c_int,
    pub nrhsvl: libc::c_int,
    pub nguess: libc::c_int,
    pub nexact: libc::c_int,
    pub colptr: *mut libc::c_int,
    pub rowind: *mut libc::c_int,
    pub rhsptr: *mut libc::c_int,
    pub rhsind: *mut libc::c_int,
    pub values: *mut libc::c_double,
    pub rhsval: *mut libc::c_double,
    pub sguess: *mut libc::c_double,
    pub xexact: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPM {
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub pool: *mut DMP,
    pub row: *mut *mut SPME,
    pub col: *mut *mut SPME,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPME {
    pub i: libc::c_int,
    pub j: libc::c_int,
    pub val: libc::c_double,
    pub r_prev: *mut SPME,
    pub r_next: *mut SPME,
    pub c_prev: *mut SPME,
    pub c_next: *mut SPME,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PER {
    pub n: libc::c_int,
    pub row: *mut libc::c_int,
    pub col: *mut libc::c_int,
}
pub unsafe extern "C" fn _glp_spm_create_mat(
    mut m: libc::c_int,
    mut n: libc::c_int,
) -> *mut SPM {
    let mut A: *mut SPM = 0 as *mut SPM;
    (0 as libc::c_int <= m && m < 2147483647 as libc::c_int
        || {
            glp_assert_(
                b"0 <= m && m < INT_MAX\0" as *const u8 as *const libc::c_char,
                b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                49 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (0 as libc::c_int <= n && n < 2147483647 as libc::c_int
        || {
            glp_assert_(
                b"0 <= n && n < INT_MAX\0" as *const u8 as *const libc::c_char,
                b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                50 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    A = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<SPM>() as libc::c_ulong as libc::c_int,
    ) as *mut SPM;
    (*A).m = m;
    (*A).n = n;
    if m == 0 as libc::c_int || n == 0 as libc::c_int {
        (*A).pool = 0 as *mut DMP;
        (*A).row = 0 as *mut *mut SPME;
        (*A).col = 0 as *mut *mut SPME;
    } else {
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        (*A).pool = _glp_dmp_create_pool();
        (*A)
            .row = glp_alloc(
            1 as libc::c_int + m,
            ::std::mem::size_of::<*mut SPME>() as libc::c_ulong as libc::c_int,
        ) as *mut *mut SPME;
        i = 1 as libc::c_int;
        while i <= m {
            let ref mut fresh0 = *((*A).row).offset(i as isize);
            *fresh0 = 0 as *mut SPME;
            i += 1;
            i;
        }
        (*A)
            .col = glp_alloc(
            1 as libc::c_int + n,
            ::std::mem::size_of::<*mut SPME>() as libc::c_ulong as libc::c_int,
        ) as *mut *mut SPME;
        j = 1 as libc::c_int;
        while j <= n {
            let ref mut fresh1 = *((*A).col).offset(j as isize);
            *fresh1 = 0 as *mut SPME;
            j += 1;
            j;
        }
    }
    return A;
}
pub unsafe extern "C" fn _glp_spm_new_elem(
    mut A: *mut SPM,
    mut i: libc::c_int,
    mut j: libc::c_int,
    mut val: libc::c_double,
) -> *mut SPME {
    let mut e: *mut SPME = 0 as *mut SPME;
    (1 as libc::c_int <= i && i <= (*A).m
        || {
            glp_assert_(
                b"1 <= i && i <= A->m\0" as *const u8 as *const libc::c_char,
                b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                92 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= j && j <= (*A).n
        || {
            glp_assert_(
                b"1 <= j && j <= A->n\0" as *const u8 as *const libc::c_char,
                b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                93 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    e = _glp_dmp_get_atom(
        (*A).pool,
        ::std::mem::size_of::<SPME>() as libc::c_ulong as libc::c_int,
    ) as *mut SPME;
    (*e).i = i;
    (*e).j = j;
    (*e).val = val;
    (*e).r_prev = 0 as *mut SPME;
    (*e).r_next = *((*A).row).offset(i as isize);
    if !((*e).r_next).is_null() {
        (*(*e).r_next).r_prev = e;
    }
    (*e).c_prev = 0 as *mut SPME;
    (*e).c_next = *((*A).col).offset(j as isize);
    if !((*e).c_next).is_null() {
        (*(*e).c_next).c_prev = e;
    }
    let ref mut fresh2 = *((*A).col).offset(j as isize);
    *fresh2 = e;
    let ref mut fresh3 = *((*A).row).offset(i as isize);
    *fresh3 = *fresh2;
    return e;
}
pub unsafe extern "C" fn _glp_spm_delete_mat(mut A: *mut SPM) {
    if !((*A).pool).is_null() {
        _glp_dmp_delete_pool((*A).pool);
    }
    if !((*A).row).is_null() {
        glp_free((*A).row as *mut libc::c_void);
    }
    if !((*A).col).is_null() {
        glp_free((*A).col as *mut libc::c_void);
    }
    glp_free(A as *mut libc::c_void);
}
pub unsafe extern "C" fn _glp_spm_test_mat_e(
    mut n: libc::c_int,
    mut c: libc::c_int,
) -> *mut SPM {
    let mut A: *mut SPM = 0 as *mut SPM;
    let mut i: libc::c_int = 0;
    (n >= 3 as libc::c_int && 2 as libc::c_int <= c && c <= n - 1 as libc::c_int
        || {
            glp_assert_(
                b"n >= 3 && 2 <= c && c <= n-1\0" as *const u8 as *const libc::c_char,
                b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                163 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    A = _glp_spm_create_mat(n, n);
    i = 1 as libc::c_int;
    while i <= n {
        _glp_spm_new_elem(A, i, i, 4.0f64);
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i <= n - 1 as libc::c_int {
        _glp_spm_new_elem(A, i, i + 1 as libc::c_int, -1.0f64);
        _glp_spm_new_elem(A, i + 1 as libc::c_int, i, -1.0f64);
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i <= n - c {
        _glp_spm_new_elem(A, i, i + c, -1.0f64);
        _glp_spm_new_elem(A, i + c, i, -1.0f64);
        i += 1;
        i;
    }
    return A;
}
pub unsafe extern "C" fn _glp_spm_test_mat_d(
    mut n: libc::c_int,
    mut c: libc::c_int,
) -> *mut SPM {
    let mut A: *mut SPM = 0 as *mut SPM;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    (n >= 14 as libc::c_int && 1 as libc::c_int <= c && c <= n - 13 as libc::c_int
        || {
            glp_assert_(
                b"n >= 14 && 1 <= c && c <= n-13\0" as *const u8 as *const libc::c_char,
                b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                209 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    A = _glp_spm_create_mat(n, n);
    i = 1 as libc::c_int;
    while i <= n {
        _glp_spm_new_elem(A, i, i, 1.0f64);
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i <= n - c {
        _glp_spm_new_elem(A, i, i + c, (i + 1 as libc::c_int) as libc::c_double);
        i += 1;
        i;
    }
    i = n - c + 1 as libc::c_int;
    while i <= n {
        _glp_spm_new_elem(A, i, i - n + c, (i + 1 as libc::c_int) as libc::c_double);
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i <= n - c - 1 as libc::c_int {
        _glp_spm_new_elem(A, i, i + c + 1 as libc::c_int, -i as libc::c_double);
        i += 1;
        i;
    }
    i = n - c;
    while i <= n {
        _glp_spm_new_elem(A, i, i - n + c + 1 as libc::c_int, -i as libc::c_double);
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i <= n - c - 2 as libc::c_int {
        _glp_spm_new_elem(A, i, i + c + 2 as libc::c_int, 16.0f64);
        i += 1;
        i;
    }
    i = n - c - 1 as libc::c_int;
    while i <= n {
        _glp_spm_new_elem(A, i, i - n + c + 2 as libc::c_int, 16.0f64);
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= 10 as libc::c_int {
        i = 1 as libc::c_int;
        while i <= 11 as libc::c_int - j {
            _glp_spm_new_elem(
                A,
                i,
                n - 11 as libc::c_int + i + j,
                100.0f64 * j as libc::c_double,
            );
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    return A;
}
pub unsafe extern "C" fn _glp_spm_show_mat(
    mut A: *const SPM,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut m: libc::c_int = (*A).m;
    let mut n: libc::c_int = (*A).n;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut map: *mut libc::c_char = 0 as *mut libc::c_char;
    glp_printf(
        b"spm_show_mat: writing matrix pattern to '%s'...\n\0" as *const u8
            as *const libc::c_char,
        fname,
    );
    (1 as libc::c_int <= m && m <= 32767 as libc::c_int
        || {
            glp_assert_(
                b"1 <= m && m <= 32767\0" as *const u8 as *const libc::c_char,
                b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                268 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= n && n <= 32767 as libc::c_int
        || {
            glp_assert_(
                b"1 <= n && n <= 32767\0" as *const u8 as *const libc::c_char,
                b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                269 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    map = glp_alloc(1 as libc::c_int, m * n) as *mut libc::c_char;
    memset(map as *mut libc::c_void, 0x8 as libc::c_int, (m * n) as libc::c_ulong);
    i = 1 as libc::c_int;
    while i <= m {
        let mut e: *mut SPME = 0 as *mut SPME;
        e = *((*A).row).offset(i as isize);
        while !e.is_null() {
            j = (*e).j;
            (1 as libc::c_int <= j && j <= n
                || {
                    glp_assert_(
                        b"1 <= j && j <= n\0" as *const u8 as *const libc::c_char,
                        b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                        276 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            k = n * (i - 1 as libc::c_int) + (j - 1 as libc::c_int);
            if *map.offset(k as isize) as libc::c_int != 0x8 as libc::c_int {
                *map.offset(k as isize) = 0xc as libc::c_int as libc::c_char;
            } else if (*e).val > 0.0f64 {
                *map.offset(k as isize) = 0xf as libc::c_int as libc::c_char;
            } else if (*e).val < 0.0f64 {
                *map.offset(k as isize) = 0xb as libc::c_int as libc::c_char;
            } else {
                *map.offset(k as isize) = 0xa as libc::c_int as libc::c_char;
            }
            e = (*e).r_next;
        }
        i += 1;
        i;
    }
    ret = _glp_rgr_write_bmp16(fname, m, n, map as *const libc::c_char);
    glp_free(map as *mut libc::c_void);
    return ret;
}
pub unsafe extern "C" fn _glp_spm_read_hbm(mut fname: *const libc::c_char) -> *mut SPM {
    let mut A: *mut SPM = 0 as *mut SPM;
    let mut hbm: *mut HBM = 0 as *mut HBM;
    let mut nrow: libc::c_int = 0;
    let mut ncol: libc::c_int = 0;
    let mut nnzero: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut beg: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut colptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rowind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut val: libc::c_double = 0.;
    let mut values: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut mxtype: *mut libc::c_char = 0 as *mut libc::c_char;
    hbm = _glp_hbm_read_mat(fname);
    if hbm.is_null() {
        glp_printf(
            b"spm_read_hbm: unable to read matrix\n\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        mxtype = ((*hbm).mxtype).as_mut_ptr();
        nrow = (*hbm).nrow;
        ncol = (*hbm).ncol;
        nnzero = (*hbm).nnzero;
        colptr = (*hbm).colptr;
        rowind = (*hbm).rowind;
        values = (*hbm).values;
        if !(strcmp(mxtype, b"RSA\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(mxtype, b"PSA\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            || strcmp(mxtype, b"RUA\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            || strcmp(mxtype, b"PUA\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            || strcmp(mxtype, b"RRA\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            || strcmp(mxtype, b"PRA\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int)
        {
            glp_printf(
                b"spm_read_hbm: matrix type '%s' not supported\n\0" as *const u8
                    as *const libc::c_char,
                mxtype,
            );
        } else {
            A = _glp_spm_create_mat(nrow, ncol);
            if *mxtype.offset(1 as libc::c_int as isize) as libc::c_int == 'S' as i32
                || *mxtype.offset(1 as libc::c_int as isize) as libc::c_int == 'U' as i32
            {
                (nrow == ncol
                    || {
                        glp_assert_(
                            b"nrow == ncol\0" as *const u8 as *const libc::c_char,
                            b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                            352 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            j = 1 as libc::c_int;
            while j <= ncol {
                beg = *colptr.offset(j as isize);
                end = *colptr.offset((j + 1 as libc::c_int) as isize);
                (1 as libc::c_int <= beg && beg <= end
                    && end <= nnzero + 1 as libc::c_int
                    || {
                        glp_assert_(
                            b"1 <= beg && beg <= end && end <= nnzero + 1\0" as *const u8
                                as *const libc::c_char,
                            b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                            356 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                ptr = beg;
                while ptr < end {
                    i = *rowind.offset(ptr as isize);
                    (1 as libc::c_int <= i && i <= nrow
                        || {
                            glp_assert_(
                                b"1 <= i && i <= nrow\0" as *const u8
                                    as *const libc::c_char,
                                b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                                359 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    if *mxtype.offset(0 as libc::c_int as isize) as libc::c_int
                        == 'R' as i32
                    {
                        val = *values.offset(ptr as isize);
                    } else {
                        val = 1.0f64;
                    }
                    _glp_spm_new_elem(A, i, j, val);
                    if *mxtype.offset(1 as libc::c_int as isize) as libc::c_int
                        == 'S' as i32 && i != j
                    {
                        _glp_spm_new_elem(A, j, i, val);
                    }
                    ptr += 1;
                    ptr;
                }
                j += 1;
                j;
            }
        }
    }
    if !hbm.is_null() {
        _glp_hbm_free_mat(hbm);
    }
    return A;
}
pub unsafe extern "C" fn _glp_spm_count_nnz(mut A: *const SPM) -> libc::c_int {
    let mut e: *mut SPME = 0 as *mut SPME;
    let mut i: libc::c_int = 0;
    let mut nnz: libc::c_int = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= (*A).m {
        e = *((*A).row).offset(i as isize);
        while !e.is_null() {
            nnz += 1;
            nnz;
            e = (*e).r_next;
        }
        i += 1;
        i;
    }
    return nnz;
}
pub unsafe extern "C" fn _glp_spm_drop_zeros(
    mut A: *mut SPM,
    mut eps: libc::c_double,
) -> libc::c_int {
    let mut e: *mut SPME = 0 as *mut SPME;
    let mut next: *mut SPME = 0 as *mut SPME;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= (*A).m {
        e = *((*A).row).offset(i as isize);
        while !e.is_null() {
            next = (*e).r_next;
            if (*e).val == 0.0f64 || fabs((*e).val) < eps {
                if ((*e).r_prev).is_null() {
                    let ref mut fresh4 = *((*A).row).offset((*e).i as isize);
                    *fresh4 = (*e).r_next;
                } else {
                    (*(*e).r_prev).r_next = (*e).r_next;
                }
                if !((*e).r_next).is_null() {
                    (*(*e).r_next).r_prev = (*e).r_prev;
                }
                if ((*e).c_prev).is_null() {
                    let ref mut fresh5 = *((*A).col).offset((*e).j as isize);
                    *fresh5 = (*e).c_next;
                } else {
                    (*(*e).c_prev).c_next = (*e).c_next;
                }
                if !((*e).c_next).is_null() {
                    (*(*e).c_next).c_prev = (*e).c_prev;
                }
                _glp_dmp_free_atom(
                    (*A).pool,
                    e as *mut libc::c_void,
                    ::std::mem::size_of::<SPME>() as libc::c_ulong as libc::c_int,
                );
                count += 1;
                count;
            }
            e = next;
        }
        i += 1;
        i;
    }
    return count;
}
pub unsafe extern "C" fn _glp_spm_read_mat(mut fname: *const libc::c_char) -> *mut SPM {
    (fname != fname
        || {
            glp_assert_(
                b"fname != fname\0" as *const u8 as *const libc::c_char,
                b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                476 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return 0 as *mut SPM;
}
pub unsafe extern "C" fn _glp_spm_write_mat(
    mut A: *const SPM,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    (A != A
        || {
            glp_assert_(
                b"A != A\0" as *const u8 as *const libc::c_char,
                b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                574 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (fname != fname
        || {
            glp_assert_(
                b"fname != fname\0" as *const u8 as *const libc::c_char,
                b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                575 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_spm_transpose(mut A: *const SPM) -> *mut SPM {
    let mut B: *mut SPM = 0 as *mut SPM;
    let mut i: libc::c_int = 0;
    B = _glp_spm_create_mat((*A).n, (*A).m);
    i = 1 as libc::c_int;
    while i <= (*A).m {
        let mut e: *mut SPME = 0 as *mut SPME;
        e = *((*A).row).offset(i as isize);
        while !e.is_null() {
            _glp_spm_new_elem(B, (*e).j, i, (*e).val);
            e = (*e).r_next;
        }
        i += 1;
        i;
    }
    return B;
}
pub unsafe extern "C" fn _glp_spm_add_sym(
    mut A: *const SPM,
    mut B: *const SPM,
) -> *mut SPM {
    let mut C: *mut SPM = 0 as *mut SPM;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut flag: *mut libc::c_int = 0 as *mut libc::c_int;
    ((*A).m == (*B).m
        || {
            glp_assert_(
                b"A->m == B->m\0" as *const u8 as *const libc::c_char,
                b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                646 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*A).n == (*B).n
        || {
            glp_assert_(
                b"A->n == B->n\0" as *const u8 as *const libc::c_char,
                b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                647 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    C = _glp_spm_create_mat((*A).m, (*A).n);
    flag = glp_alloc(
        1 as libc::c_int + (*C).n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    j = 1 as libc::c_int;
    while j <= (*C).n {
        *flag.offset(j as isize) = 0 as libc::c_int;
        j += 1;
        j;
    }
    i = 1 as libc::c_int;
    while i <= (*C).m {
        let mut e: *mut SPME = 0 as *mut SPME;
        e = *((*A).row).offset(i as isize);
        while !e.is_null() {
            j = (*e).j;
            if *flag.offset(j as isize) == 0 {
                _glp_spm_new_elem(C, i, j, 0.0f64);
                *flag.offset(j as isize) = 1 as libc::c_int;
            }
            e = (*e).r_next;
        }
        e = *((*B).row).offset(i as isize);
        while !e.is_null() {
            j = (*e).j;
            if *flag.offset(j as isize) == 0 {
                _glp_spm_new_elem(C, i, j, 0.0f64);
                *flag.offset(j as isize) = 1 as libc::c_int;
            }
            e = (*e).r_next;
        }
        e = *((*C).row).offset(i as isize);
        while !e.is_null() {
            *flag.offset((*e).j as isize) = 0 as libc::c_int;
            e = (*e).r_next;
        }
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= (*C).n {
        (*flag.offset(j as isize) == 0
            || {
                glp_assert_(
                    b"!flag[j]\0" as *const u8 as *const libc::c_char,
                    b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                    682 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        j += 1;
        j;
    }
    glp_free(flag as *mut libc::c_void);
    return C;
}
pub unsafe extern "C" fn _glp_spm_add_num(
    mut C: *mut SPM,
    mut alfa: libc::c_double,
    mut A: *const SPM,
    mut beta: libc::c_double,
    mut B: *const SPM,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut work: *mut libc::c_double = 0 as *mut libc::c_double;
    work = glp_alloc(
        1 as libc::c_int + (*C).n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    j = 1 as libc::c_int;
    while j <= (*C).n {
        *work.offset(j as isize) = 0.0f64;
        j += 1;
        j;
    }
    i = 1 as libc::c_int;
    while i <= (*C).n {
        let mut e: *mut SPME = 0 as *mut SPME;
        e = *((*A).row).offset(i as isize);
        while !e.is_null() {
            *work.offset((*e).j as isize) += alfa * (*e).val;
            e = (*e).r_next;
        }
        e = *((*B).row).offset(i as isize);
        while !e.is_null() {
            *work.offset((*e).j as isize) += beta * (*e).val;
            e = (*e).r_next;
        }
        e = *((*C).row).offset(i as isize);
        while !e.is_null() {
            j = (*e).j;
            (*e).val = *work.offset(j as isize);
            *work.offset(j as isize) = 0.0f64;
            e = (*e).r_next;
        }
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= (*C).n {
        (*work.offset(j as isize) == 0.0f64
            || {
                glp_assert_(
                    b"work[j] == 0.0\0" as *const u8 as *const libc::c_char,
                    b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                    714 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        j += 1;
        j;
    }
    glp_free(work as *mut libc::c_void);
}
pub unsafe extern "C" fn _glp_spm_add_mat(
    mut alfa: libc::c_double,
    mut A: *const SPM,
    mut beta: libc::c_double,
    mut B: *const SPM,
) -> *mut SPM {
    let mut C: *mut SPM = 0 as *mut SPM;
    C = _glp_spm_add_sym(A, B);
    _glp_spm_add_num(C, alfa, A, beta, B);
    return C;
}
pub unsafe extern "C" fn _glp_spm_mul_sym(
    mut A: *const SPM,
    mut B: *const SPM,
) -> *mut SPM {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut flag: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut C: *mut SPM = 0 as *mut SPM;
    ((*A).n == (*B).m
        || {
            glp_assert_(
                b"A->n == B->m\0" as *const u8 as *const libc::c_char,
                b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                731 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    C = _glp_spm_create_mat((*A).m, (*B).n);
    flag = glp_alloc(
        1 as libc::c_int + (*C).n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    j = 1 as libc::c_int;
    while j <= (*C).n {
        *flag.offset(j as isize) = 0 as libc::c_int;
        j += 1;
        j;
    }
    i = 1 as libc::c_int;
    while i <= (*C).m {
        let mut e: *mut SPME = 0 as *mut SPME;
        let mut ee: *mut SPME = 0 as *mut SPME;
        e = *((*A).row).offset(i as isize);
        while !e.is_null() {
            k = (*e).j;
            ee = *((*B).row).offset(k as isize);
            while !ee.is_null() {
                j = (*ee).j;
                if *flag.offset(j as isize) == 0 {
                    _glp_spm_new_elem(C, i, j, 0.0f64);
                    *flag.offset(j as isize) = 1 as libc::c_int;
                }
                ee = (*ee).r_next;
            }
            e = (*e).r_next;
        }
        e = *((*C).row).offset(i as isize);
        while !e.is_null() {
            *flag.offset((*e).j as isize) = 0 as libc::c_int;
            e = (*e).r_next;
        }
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= (*C).n {
        (*flag.offset(j as isize) == 0
            || {
                glp_assert_(
                    b"!flag[j]\0" as *const u8 as *const libc::c_char,
                    b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                    760 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        j += 1;
        j;
    }
    glp_free(flag as *mut libc::c_void);
    return C;
}
pub unsafe extern "C" fn _glp_spm_mul_num(
    mut C: *mut SPM,
    mut A: *const SPM,
    mut B: *const SPM,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut work: *mut libc::c_double = 0 as *mut libc::c_double;
    work = glp_alloc(
        1 as libc::c_int + (*A).n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    j = 1 as libc::c_int;
    while j <= (*A).n {
        *work.offset(j as isize) = 0.0f64;
        j += 1;
        j;
    }
    i = 1 as libc::c_int;
    while i <= (*C).m {
        let mut e: *mut SPME = 0 as *mut SPME;
        let mut ee: *mut SPME = 0 as *mut SPME;
        let mut temp: libc::c_double = 0.;
        e = *((*A).row).offset(i as isize);
        while !e.is_null() {
            *work.offset((*e).j as isize) += (*e).val;
            e = (*e).r_next;
        }
        e = *((*C).row).offset(i as isize);
        while !e.is_null() {
            j = (*e).j;
            temp = 0.0f64;
            ee = *((*B).col).offset(j as isize);
            while !ee.is_null() {
                temp += *work.offset((*ee).i as isize) * (*ee).val;
                ee = (*ee).c_next;
            }
            (*e).val = temp;
            e = (*e).r_next;
        }
        e = *((*A).row).offset(i as isize);
        while !e.is_null() {
            *work.offset((*e).j as isize) = 0.0f64;
            e = (*e).r_next;
        }
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= (*A).n {
        (*work.offset(j as isize) == 0.0f64
            || {
                glp_assert_(
                    b"work[j] == 0.0\0" as *const u8 as *const libc::c_char,
                    b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                    796 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        j += 1;
        j;
    }
    glp_free(work as *mut libc::c_void);
}
pub unsafe extern "C" fn _glp_spm_mul_mat(
    mut A: *const SPM,
    mut B: *const SPM,
) -> *mut SPM {
    let mut C: *mut SPM = 0 as *mut SPM;
    C = _glp_spm_mul_sym(A, B);
    _glp_spm_mul_num(C, A, B);
    return C;
}
pub unsafe extern "C" fn _glp_spm_create_per(mut n: libc::c_int) -> *mut PER {
    let mut P: *mut PER = 0 as *mut PER;
    let mut k: libc::c_int = 0;
    (n >= 0 as libc::c_int
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const libc::c_char,
                b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                813 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    P = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<PER>() as libc::c_ulong as libc::c_int,
    ) as *mut PER;
    (*P).n = n;
    (*P)
        .row = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*P)
        .col = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    k = 1 as libc::c_int;
    while k <= n {
        let ref mut fresh6 = *((*P).col).offset(k as isize);
        *fresh6 = k;
        *((*P).row).offset(k as isize) = *fresh6;
        k += 1;
        k;
    }
    return P;
}
pub unsafe extern "C" fn _glp_spm_check_per(mut P: *mut PER) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    ((*P).n >= 0 as libc::c_int
        || {
            glp_assert_(
                b"P->n >= 0\0" as *const u8 as *const libc::c_char,
                b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                827 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    i = 1 as libc::c_int;
    while i <= (*P).n {
        j = *((*P).row).offset(i as isize);
        (1 as libc::c_int <= j && j <= (*P).n
            || {
                glp_assert_(
                    b"1 <= j && j <= P->n\0" as *const u8 as *const libc::c_char,
                    b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                    830 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*((*P).col).offset(j as isize) == i
            || {
                glp_assert_(
                    b"P->col[j] == i\0" as *const u8 as *const libc::c_char,
                    b"misc/spm.c\0" as *const u8 as *const libc::c_char,
                    831 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn _glp_spm_delete_per(mut P: *mut PER) {
    glp_free((*P).row as *mut libc::c_void);
    glp_free((*P).col as *mut libc::c_void);
    glp_free(P as *mut libc::c_void);
}
