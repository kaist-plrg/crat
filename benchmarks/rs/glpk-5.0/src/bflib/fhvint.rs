use ::libc;
extern "C" {
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
    fn _glp_sva_alloc_vecs(sva: *mut SVA, nnn: libc::c_int) -> libc::c_int;
    fn _glp_luf_f_solve(luf: *mut LUF, x: *mut libc::c_double);
    fn _glp_luf_ft_solve(luf: *mut LUF, x: *mut libc::c_double);
    fn _glp_luf_v_solve(luf: *mut LUF, b: *mut libc::c_double, x: *mut libc::c_double);
    fn _glp_luf_vt_solve(luf: *mut LUF, b: *mut libc::c_double, x: *mut libc::c_double);
    fn _glp_luf_estimate_norm(
        luf: *mut LUF,
        w1: *mut libc::c_double,
        w2: *mut libc::c_double,
    ) -> libc::c_double;
    fn _glp_fhv_ft_update(
        fhv: *mut FHV,
        q: libc::c_int,
        aq_len: libc::c_int,
        aq_ind: *const libc::c_int,
        aq_val: *const libc::c_double,
        ind: *mut libc::c_int,
        val: *mut libc::c_double,
        work: *mut libc::c_double,
    ) -> libc::c_int;
    fn _glp_fhv_h_solve(fhv: *mut FHV, x: *mut libc::c_double);
    fn _glp_fhv_ht_solve(fhv: *mut FHV, x: *mut libc::c_double);
    fn _glp_lufint_create() -> *mut LUFINT;
    fn _glp_lufint_factorize(
        fi: *mut LUFINT,
        n: libc::c_int,
        col: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                libc::c_int,
                *mut libc::c_int,
                *mut libc::c_double,
            ) -> libc::c_int,
        >,
        info: *mut libc::c_void,
    ) -> libc::c_int;
    fn _glp_lufint_delete(fi: *mut LUFINT);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SVA {
    pub n_max: libc::c_int,
    pub n: libc::c_int,
    pub ptr: *mut libc::c_int,
    pub len: *mut libc::c_int,
    pub cap: *mut libc::c_int,
    pub size: libc::c_int,
    pub m_ptr: libc::c_int,
    pub r_ptr: libc::c_int,
    pub head: libc::c_int,
    pub tail: libc::c_int,
    pub prev: *mut libc::c_int,
    pub next: *mut libc::c_int,
    pub ind: *mut libc::c_int,
    pub val: *mut libc::c_double,
    pub talky: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LUF {
    pub n: libc::c_int,
    pub sva: *mut SVA,
    pub fr_ref: libc::c_int,
    pub fc_ref: libc::c_int,
    pub vr_ref: libc::c_int,
    pub vr_piv: *mut libc::c_double,
    pub vc_ref: libc::c_int,
    pub pp_ind: *mut libc::c_int,
    pub pp_inv: *mut libc::c_int,
    pub qq_ind: *mut libc::c_int,
    pub qq_inv: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FHV {
    pub luf: *mut LUF,
    pub nfs_max: libc::c_int,
    pub nfs: libc::c_int,
    pub hh_ind: *mut libc::c_int,
    pub hh_ref: libc::c_int,
    pub p0_ind: *mut libc::c_int,
    pub p0_inv: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SGF {
    pub luf: *mut LUF,
    pub rs_head: *mut libc::c_int,
    pub rs_prev: *mut libc::c_int,
    pub rs_next: *mut libc::c_int,
    pub cs_head: *mut libc::c_int,
    pub cs_prev: *mut libc::c_int,
    pub cs_next: *mut libc::c_int,
    pub vr_max: *mut libc::c_double,
    pub flag: *mut libc::c_char,
    pub work: *mut libc::c_double,
    pub updat: libc::c_int,
    pub piv_tol: libc::c_double,
    pub piv_lim: libc::c_int,
    pub suhl: libc::c_int,
    pub eps_tol: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LUFINT {
    pub n_max: libc::c_int,
    pub valid: libc::c_int,
    pub sva: *mut SVA,
    pub luf: *mut LUF,
    pub sgf: *mut SGF,
    pub sva_n_max: libc::c_int,
    pub sva_size: libc::c_int,
    pub delta_n0: libc::c_int,
    pub delta_n: libc::c_int,
    pub sgf_updat: libc::c_int,
    pub sgf_piv_tol: libc::c_double,
    pub sgf_piv_lim: libc::c_int,
    pub sgf_suhl: libc::c_int,
    pub sgf_eps_tol: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FHVINT {
    pub valid: libc::c_int,
    pub fhv: FHV,
    pub lufi: *mut LUFINT,
    pub nfs_max: libc::c_int,
}
pub unsafe extern "C" fn _glp_fhvint_create() -> *mut FHVINT {
    let mut fi: *mut FHVINT = 0 as *mut FHVINT;
    fi = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<FHVINT>() as libc::c_ulong as libc::c_int,
    ) as *mut FHVINT;
    memset(
        fi as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<FHVINT>() as libc::c_ulong,
    );
    (*fi).lufi = _glp_lufint_create();
    return fi;
}
pub unsafe extern "C" fn _glp_fhvint_factorize(
    mut fi: *mut FHVINT,
    mut n: libc::c_int,
    mut col: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *mut libc::c_int,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    mut info: *mut libc::c_void,
) -> libc::c_int {
    let mut nfs_max: libc::c_int = 0;
    let mut old_n_max: libc::c_int = 0;
    let mut n_max: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    (n > 0 as libc::c_int
        || {
            glp_assert_(
                b"n > 0\0" as *const u8 as *const libc::c_char,
                b"bflib/fhvint.c\0" as *const u8 as *const libc::c_char,
                38 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*fi).valid = 0 as libc::c_int;
    nfs_max = (*fi).nfs_max;
    if nfs_max == 0 as libc::c_int {
        nfs_max = 100 as libc::c_int;
    }
    (nfs_max > 0 as libc::c_int
        || {
            glp_assert_(
                b"nfs_max > 0\0" as *const u8 as *const libc::c_char,
                b"bflib/fhvint.c\0" as *const u8 as *const libc::c_char,
                44 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    old_n_max = (*(*fi).lufi).n_max;
    (*(*fi).lufi).sva_n_max = 4 as libc::c_int * n + nfs_max;
    (*(*fi).lufi).sgf_updat = 1 as libc::c_int;
    ret = _glp_lufint_factorize((*fi).lufi, n, col, info);
    n_max = (*(*fi).lufi).n_max;
    if (*fi).fhv.nfs_max != nfs_max {
        if !((*fi).fhv.hh_ind).is_null() {
            glp_free((*fi).fhv.hh_ind as *mut libc::c_void);
        }
        (*fi)
            .fhv
            .hh_ind = glp_alloc(
            1 as libc::c_int + nfs_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
    }
    if old_n_max < n_max {
        if !((*fi).fhv.p0_ind).is_null() {
            glp_free((*fi).fhv.p0_ind as *mut libc::c_void);
        }
        if !((*fi).fhv.p0_inv).is_null() {
            glp_free((*fi).fhv.p0_inv as *mut libc::c_void);
        }
        (*fi)
            .fhv
            .p0_ind = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*fi)
            .fhv
            .p0_inv = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
    }
    (*fi).fhv.luf = (*(*fi).lufi).luf;
    (*fi).fhv.nfs_max = nfs_max;
    (*fi).fhv.nfs = 0 as libc::c_int;
    (*fi).fhv.hh_ref = _glp_sva_alloc_vecs((*(*fi).lufi).sva, nfs_max);
    k = 1 as libc::c_int;
    while k <= n {
        *((*fi).fhv.p0_ind)
            .offset(k as isize) = *((*(*fi).fhv.luf).pp_ind).offset(k as isize);
        *((*fi).fhv.p0_inv)
            .offset(k as isize) = *((*(*fi).fhv.luf).pp_inv).offset(k as isize);
        k += 1;
        k;
    }
    if ret == 0 as libc::c_int {
        (*fi).valid = 1 as libc::c_int;
    }
    return ret;
}
pub unsafe extern "C" fn _glp_fhvint_update(
    mut fi: *mut FHVINT,
    mut j: libc::c_int,
    mut len: libc::c_int,
    mut ind: *const libc::c_int,
    mut val: *const libc::c_double,
) -> libc::c_int {
    let mut sgf: *mut SGF = (*(*fi).lufi).sgf;
    let mut ind1: *mut libc::c_int = (*sgf).rs_next;
    let mut val1: *mut libc::c_double = (*sgf).vr_max;
    let mut work: *mut libc::c_double = (*sgf).work;
    let mut ret: libc::c_int = 0;
    ((*fi).valid != 0
        || {
            glp_assert_(
                b"fi->valid\0" as *const u8 as *const libc::c_char,
                b"bflib/fhvint.c\0" as *const u8 as *const libc::c_char,
                90 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ret = _glp_fhv_ft_update(&mut (*fi).fhv, j, len, ind, val, ind1, val1, work);
    if ret != 0 as libc::c_int {
        (*fi).valid = 0 as libc::c_int;
    }
    return ret;
}
pub unsafe extern "C" fn _glp_fhvint_ftran(
    mut fi: *mut FHVINT,
    mut x: *mut libc::c_double,
) {
    let mut fhv: *mut FHV = &mut (*fi).fhv;
    let mut luf: *mut LUF = (*fhv).luf;
    let mut n: libc::c_int = (*luf).n;
    let mut pp_ind: *mut libc::c_int = (*luf).pp_ind;
    let mut pp_inv: *mut libc::c_int = (*luf).pp_inv;
    let mut sgf: *mut SGF = (*(*fi).lufi).sgf;
    let mut work: *mut libc::c_double = (*sgf).work;
    ((*fi).valid != 0
        || {
            glp_assert_(
                b"fi->valid\0" as *const u8 as *const libc::c_char,
                b"bflib/fhvint.c\0" as *const u8 as *const libc::c_char,
                106 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*luf).pp_ind = (*fhv).p0_ind;
    (*luf).pp_inv = (*fhv).p0_inv;
    _glp_luf_f_solve(luf, x);
    (*luf).pp_ind = pp_ind;
    (*luf).pp_inv = pp_inv;
    _glp_fhv_h_solve(fhv, x);
    _glp_luf_v_solve(luf, x, work);
    memcpy(
        &mut *x.offset(1 as libc::c_int as isize) as *mut libc::c_double
            as *mut libc::c_void,
        &mut *work.offset(1 as libc::c_int as isize) as *mut libc::c_double
            as *const libc::c_void,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
}
pub unsafe extern "C" fn _glp_fhvint_btran(
    mut fi: *mut FHVINT,
    mut x: *mut libc::c_double,
) {
    let mut fhv: *mut FHV = &mut (*fi).fhv;
    let mut luf: *mut LUF = (*fhv).luf;
    let mut n: libc::c_int = (*luf).n;
    let mut pp_ind: *mut libc::c_int = (*luf).pp_ind;
    let mut pp_inv: *mut libc::c_int = (*luf).pp_inv;
    let mut sgf: *mut SGF = (*(*fi).lufi).sgf;
    let mut work: *mut libc::c_double = (*sgf).work;
    ((*fi).valid != 0
        || {
            glp_assert_(
                b"fi->valid\0" as *const u8 as *const libc::c_char,
                b"bflib/fhvint.c\0" as *const u8 as *const libc::c_char,
                129 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_luf_vt_solve(luf, x, work);
    _glp_fhv_ht_solve(fhv, work);
    (*luf).pp_ind = (*fhv).p0_ind;
    (*luf).pp_inv = (*fhv).p0_inv;
    _glp_luf_ft_solve(luf, work);
    (*luf).pp_ind = pp_ind;
    (*luf).pp_inv = pp_inv;
    memcpy(
        &mut *x.offset(1 as libc::c_int as isize) as *mut libc::c_double
            as *mut libc::c_void,
        &mut *work.offset(1 as libc::c_int as isize) as *mut libc::c_double
            as *const libc::c_void,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
}
pub unsafe extern "C" fn _glp_fhvint_estimate(mut fi: *mut FHVINT) -> libc::c_double {
    let mut norm: libc::c_double = 0.;
    ((*fi).valid != 0
        || {
            glp_assert_(
                b"fi->valid\0" as *const u8 as *const libc::c_char,
                b"bflib/fhvint.c\0" as *const u8 as *const libc::c_char,
                146 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*fi).fhv.nfs == 0 as libc::c_int
        || {
            glp_assert_(
                b"fi->fhv.nfs == 0\0" as *const u8 as *const libc::c_char,
                b"bflib/fhvint.c\0" as *const u8 as *const libc::c_char,
                147 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    norm = _glp_luf_estimate_norm(
        (*fi).fhv.luf,
        (*(*(*fi).lufi).sgf).vr_max,
        (*(*(*fi).lufi).sgf).work,
    );
    return norm;
}
pub unsafe extern "C" fn _glp_fhvint_delete(mut fi: *mut FHVINT) {
    _glp_lufint_delete((*fi).lufi);
    if !((*fi).fhv.hh_ind).is_null() {
        glp_free((*fi).fhv.hh_ind as *mut libc::c_void);
    }
    if !((*fi).fhv.p0_ind).is_null() {
        glp_free((*fi).fhv.p0_ind as *mut libc::c_void);
    }
    if !((*fi).fhv.p0_inv).is_null() {
        glp_free((*fi).fhv.p0_inv as *mut libc::c_void);
    }
    glp_free(fi as *mut libc::c_void);
}
