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
pub struct SPXAT {
    pub ptr: *mut libc::c_int,
    pub ind: *mut libc::c_int,
    pub val: *mut libc::c_double,
    pub work: *mut libc::c_double,
}
pub unsafe extern "C" fn _glp_spx_alloc_at(mut lp: *mut SPXLP, mut at: *mut SPXAT) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut nnz: libc::c_int = (*lp).nnz;
    (*at)
        .ptr = glp_alloc(
        1 as libc::c_int + m + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*at)
        .ind = glp_alloc(
        1 as libc::c_int + nnz,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*at)
        .val = glp_alloc(
        1 as libc::c_int + nnz,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*at)
        .work = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
}
pub unsafe extern "C" fn _glp_spx_build_at(mut lp: *mut SPXLP, mut at: *mut SPXAT) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut nnz: libc::c_int = (*lp).nnz;
    let mut A_ptr: *mut libc::c_int = (*lp).A_ptr;
    let mut A_ind: *mut libc::c_int = (*lp).A_ind;
    let mut A_val: *mut libc::c_double = (*lp).A_val;
    let mut AT_ptr: *mut libc::c_int = (*at).ptr;
    let mut AT_ind: *mut libc::c_int = (*at).ind;
    let mut AT_val: *mut libc::c_double = (*at).val;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    memset(
        &mut *AT_ptr.offset(1 as libc::c_int as isize) as *mut libc::c_int
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
            let ref mut fresh0 = *AT_ptr.offset(*A_ind.offset(ptr as isize) as isize);
            *fresh0 += 1;
            *fresh0;
            ptr += 1;
            ptr;
        }
        k += 1;
        k;
    }
    let ref mut fresh1 = *AT_ptr.offset(1 as libc::c_int as isize);
    *fresh1 += 1;
    *fresh1;
    i = 2 as libc::c_int;
    while i <= m {
        *AT_ptr.offset(i as isize) += *AT_ptr.offset((i - 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    (*AT_ptr.offset(m as isize) == nnz + 1 as libc::c_int
        || {
            glp_assert_(
                b"AT_ptr[m] == nnz+1\0" as *const u8 as *const libc::c_char,
                b"simplex/spxat.c\0" as *const u8 as *const libc::c_char,
                72 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    *AT_ptr.offset((m + 1 as libc::c_int) as isize) = nnz + 1 as libc::c_int;
    k = n;
    while k >= 1 as libc::c_int {
        ptr = *A_ptr.offset(k as isize);
        end = *A_ptr.offset((k + 1 as libc::c_int) as isize);
        while ptr < end {
            let ref mut fresh2 = *AT_ptr.offset(*A_ind.offset(ptr as isize) as isize);
            *fresh2 -= 1;
            pos = *fresh2;
            *AT_ind.offset(pos as isize) = k;
            *AT_val.offset(pos as isize) = *A_val.offset(ptr as isize);
            ptr += 1;
            ptr;
        }
        k -= 1;
        k;
    }
    (*AT_ptr.offset(1 as libc::c_int as isize) == 1 as libc::c_int
        || {
            glp_assert_(
                b"AT_ptr[1] == 1\0" as *const u8 as *const libc::c_char,
                b"simplex/spxat.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
}
pub unsafe extern "C" fn _glp_spx_at_prod(
    mut lp: *mut SPXLP,
    mut at: *mut SPXAT,
    mut y: *mut libc::c_double,
    mut s: libc::c_double,
    mut x: *const libc::c_double,
) {
    let mut m: libc::c_int = (*lp).m;
    let mut AT_ptr: *mut libc::c_int = (*at).ptr;
    let mut AT_ind: *mut libc::c_int = (*at).ind;
    let mut AT_val: *mut libc::c_double = (*at).val;
    let mut i: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut t: libc::c_double = 0.;
    i = 1 as libc::c_int;
    while i <= m {
        if *x.offset(i as isize) != 0.0f64 {
            t = s * *x.offset(i as isize);
            ptr = *AT_ptr.offset(i as isize);
            end = *AT_ptr.offset((i + 1 as libc::c_int) as isize);
            while ptr < end {
                *y.offset(*AT_ind.offset(ptr as isize) as isize)
                    += *AT_val.offset(ptr as isize) * t;
                ptr += 1;
                ptr;
            }
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn _glp_spx_nt_prod1(
    mut lp: *mut SPXLP,
    mut at: *mut SPXAT,
    mut y: *mut libc::c_double,
    mut ign: libc::c_int,
    mut s: libc::c_double,
    mut x: *const libc::c_double,
) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut work: *mut libc::c_double = (*at).work;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    k = 1 as libc::c_int;
    while k <= n {
        *work.offset(k as isize) = 0.0f64;
        k += 1;
        k;
    }
    if ign == 0 {
        j = 1 as libc::c_int;
        while j <= n - m {
            *work
                .offset(*head.offset((m + j) as isize) as isize) = *y.offset(j as isize);
            j += 1;
            j;
        }
    }
    _glp_spx_at_prod(lp, at, work, s, x);
    j = 1 as libc::c_int;
    while j <= n - m {
        *y.offset(j as isize) = *work.offset(*head.offset((m + j) as isize) as isize);
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn _glp_spx_eval_trow1(
    mut lp: *mut SPXLP,
    mut at: *mut SPXAT,
    mut rho: *const libc::c_double,
    mut trow: *mut libc::c_double,
) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut nnz: libc::c_int = (*lp).nnz;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nnz_rho: libc::c_int = 0;
    let mut cnt1: libc::c_double = 0.;
    let mut cnt2: libc::c_double = 0.;
    nnz_rho = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= m {
        if *rho.offset(i as isize) != 0.0f64 {
            nnz_rho += 1;
            nnz_rho;
        }
        i += 1;
        i;
    }
    cnt1 = (n - m) as libc::c_double * (nnz as libc::c_double / n as libc::c_double);
    cnt2 = nnz_rho as libc::c_double * (nnz as libc::c_double / m as libc::c_double);
    if cnt1 < cnt2 {
        let mut A_ptr: *mut libc::c_int = (*lp).A_ptr;
        let mut A_ind: *mut libc::c_int = (*lp).A_ind;
        let mut A_val: *mut libc::c_double = (*lp).A_val;
        let mut head: *mut libc::c_int = (*lp).head;
        let mut k: libc::c_int = 0;
        let mut ptr: libc::c_int = 0;
        let mut end: libc::c_int = 0;
        let mut tij: libc::c_double = 0.;
        j = 1 as libc::c_int;
        while j <= n - m {
            k = *head.offset((m + j) as isize);
            tij = 0.0f64;
            ptr = *A_ptr.offset(k as isize);
            end = *A_ptr.offset((k + 1 as libc::c_int) as isize);
            while ptr < end {
                tij
                    -= *A_val.offset(ptr as isize)
                        * *rho.offset(*A_ind.offset(ptr as isize) as isize);
                ptr += 1;
                ptr;
            }
            *trow.offset(j as isize) = tij;
            j += 1;
            j;
        }
    } else {
        _glp_spx_nt_prod1(lp, at, trow, 1 as libc::c_int, -1.0f64, rho);
    };
}
pub unsafe extern "C" fn _glp_spx_free_at(mut lp: *mut SPXLP, mut at: *mut SPXAT) {
    (lp == lp
        || {
            glp_assert_(
                b"lp == lp\0" as *const u8 as *const libc::c_char,
                b"simplex/spxat.c\0" as *const u8 as *const libc::c_char,
                255 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    glp_free((*at).ptr as *mut libc::c_void);
    glp_free((*at).ind as *mut libc::c_void);
    glp_free((*at).val as *mut libc::c_void);
    glp_free((*at).work as *mut libc::c_void);
}
