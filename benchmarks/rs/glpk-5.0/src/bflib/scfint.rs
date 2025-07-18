use ::libc;
extern "C" {
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn _glp_sva_alloc_vecs(sva: *mut SVA, nnn: libc::c_int) -> libc::c_int;
    fn _glp_btf_estimate_norm(
        btf: *mut BTF,
        w1: *mut libc::c_double,
        w2: *mut libc::c_double,
        w3: *mut libc::c_double,
        w4: *mut libc::c_double,
    ) -> libc::c_double;
    fn _glp_luf_estimate_norm(
        luf: *mut LUF,
        w1: *mut libc::c_double,
        w2: *mut libc::c_double,
    ) -> libc::c_double;
    fn _glp_scf_a_solve(
        scf: *mut SCF,
        x: *mut libc::c_double,
        w: *mut libc::c_double,
        work1: *mut libc::c_double,
        work2: *mut libc::c_double,
        work3: *mut libc::c_double,
    );
    fn _glp_scf_at_solve(
        scf: *mut SCF,
        x: *mut libc::c_double,
        w: *mut libc::c_double,
        work1: *mut libc::c_double,
        work2: *mut libc::c_double,
        work3: *mut libc::c_double,
    );
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
    fn _glp_btfint_create() -> *mut BTFINT;
    fn _glp_btfint_factorize(
        fi: *mut BTFINT,
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
    fn _glp_btfint_delete(fi: *mut BTFINT);
    fn _glp_scf_update_aug(
        scf: *mut SCF,
        b: *mut libc::c_double,
        d: *mut libc::c_double,
        f: *mut libc::c_double,
        g: *mut libc::c_double,
        h: libc::c_double,
        upd: libc::c_int,
        w1: *mut libc::c_double,
        w2: *mut libc::c_double,
        w3: *mut libc::c_double,
    ) -> libc::c_int;
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
pub struct BTF {
    pub n: libc::c_int,
    pub sva: *mut SVA,
    pub pp_ind: *mut libc::c_int,
    pub pp_inv: *mut libc::c_int,
    pub qq_ind: *mut libc::c_int,
    pub qq_inv: *mut libc::c_int,
    pub num: libc::c_int,
    pub beg: *mut libc::c_int,
    pub ar_ref: libc::c_int,
    pub ac_ref: libc::c_int,
    pub fr_ref: libc::c_int,
    pub fc_ref: libc::c_int,
    pub vr_ref: libc::c_int,
    pub vr_piv: *mut libc::c_double,
    pub vc_ref: libc::c_int,
    pub p1_ind: *mut libc::c_int,
    pub p1_inv: *mut libc::c_int,
    pub q1_ind: *mut libc::c_int,
    pub q1_inv: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IFU {
    pub n_max: libc::c_int,
    pub n: libc::c_int,
    pub f: *mut libc::c_double,
    pub u: *mut libc::c_double,
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
pub struct SCF {
    pub n: libc::c_int,
    pub n0: libc::c_int,
    pub type_0: libc::c_int,
    pub a0: C2RustUnnamed,
    pub nn_max: libc::c_int,
    pub nn: libc::c_int,
    pub sva: *mut SVA,
    pub rr_ref: libc::c_int,
    pub ss_ref: libc::c_int,
    pub ifu: IFU,
    pub pp_ind: *mut libc::c_int,
    pub pp_inv: *mut libc::c_int,
    pub qq_ind: *mut libc::c_int,
    pub qq_inv: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub luf: *mut LUF,
    pub btf: *mut BTF,
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
pub struct BTFINT {
    pub n_max: libc::c_int,
    pub valid: libc::c_int,
    pub sva: *mut SVA,
    pub btf: *mut BTF,
    pub sgf: *mut SGF,
    pub sva_n_max: libc::c_int,
    pub sva_size: libc::c_int,
    pub delta_n0: libc::c_int,
    pub delta_n: libc::c_int,
    pub sgf_piv_tol: libc::c_double,
    pub sgf_piv_lim: libc::c_int,
    pub sgf_suhl: libc::c_int,
    pub sgf_eps_tol: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCFINT {
    pub valid: libc::c_int,
    pub scf: SCF,
    pub u: C2RustUnnamed_0,
    pub w1: *mut libc::c_double,
    pub w2: *mut libc::c_double,
    pub w3: *mut libc::c_double,
    pub w4: *mut libc::c_double,
    pub w5: *mut libc::c_double,
    pub nn_max: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub lufi: *mut LUFINT,
    pub btfi: *mut BTFINT,
}
pub unsafe extern "C" fn _glp_scfint_create(mut type_0: libc::c_int) -> *mut SCFINT {
    let mut fi: *mut SCFINT = 0 as *mut SCFINT;
    fi = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<SCFINT>() as libc::c_ulong as libc::c_int,
    ) as *mut SCFINT;
    memset(
        fi as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<SCFINT>() as libc::c_ulong,
    );
    (*fi).scf.type_0 = type_0;
    match (*fi).scf.type_0 {
        1 => {
            (*fi).u.lufi = _glp_lufint_create();
        }
        2 => {
            (*fi).u.btfi = _glp_btfint_create();
        }
        _ => {
            (type_0 != type_0
                || {
                    glp_assert_(
                        b"type != type\0" as *const u8 as *const libc::c_char,
                        b"bflib/scfint.c\0" as *const u8 as *const libc::c_char,
                        38 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    return fi;
}
pub unsafe extern "C" fn _glp_scfint_factorize(
    mut fi: *mut SCFINT,
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
    let mut nn_max: libc::c_int = 0;
    let mut old_n0_max: libc::c_int = 0;
    let mut n0_max: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    (n > 0 as libc::c_int
        || {
            glp_assert_(
                b"n > 0\0" as *const u8 as *const libc::c_char,
                b"bflib/scfint.c\0" as *const u8 as *const libc::c_char,
                47 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*fi).valid = 0 as libc::c_int;
    nn_max = (*fi).nn_max;
    if nn_max == 0 as libc::c_int {
        nn_max = 100 as libc::c_int;
    }
    (nn_max > 0 as libc::c_int
        || {
            glp_assert_(
                b"nn_max > 0\0" as *const u8 as *const libc::c_char,
                b"bflib/scfint.c\0" as *const u8 as *const libc::c_char,
                53 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    match (*fi).scf.type_0 {
        1 => {
            old_n0_max = (*(*fi).u.lufi).n_max;
            (*(*fi).u.lufi).sva_n_max = 4 as libc::c_int * n + 2 as libc::c_int * nn_max;
            ret = _glp_lufint_factorize((*fi).u.lufi, n, col, info);
            n0_max = (*(*fi).u.lufi).n_max;
            (*fi).scf.sva = (*(*fi).u.lufi).sva;
            (*fi).scf.a0.luf = (*(*fi).u.lufi).luf;
        }
        2 => {
            old_n0_max = (*(*fi).u.btfi).n_max;
            (*(*fi).u.btfi).sva_n_max = 6 as libc::c_int * n + 2 as libc::c_int * nn_max;
            ret = _glp_btfint_factorize((*fi).u.btfi, n, col, info);
            n0_max = (*(*fi).u.btfi).n_max;
            (*fi).scf.sva = (*(*fi).u.btfi).sva;
            (*fi).scf.a0.btf = (*(*fi).u.btfi).btf;
        }
        _ => {
            (fi != fi
                || {
                    glp_assert_(
                        b"fi != fi\0" as *const u8 as *const libc::c_char,
                        b"bflib/scfint.c\0" as *const u8 as *const libc::c_char,
                        73 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    if old_n0_max < n0_max {
        if !((*fi).w1).is_null() {
            glp_free((*fi).w1 as *mut libc::c_void);
        }
        if !((*fi).w2).is_null() {
            glp_free((*fi).w2 as *mut libc::c_void);
        }
        if !((*fi).w3).is_null() {
            glp_free((*fi).w3 as *mut libc::c_void);
        }
        (*fi)
            .w1 = glp_alloc(
            1 as libc::c_int + n0_max,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        (*fi)
            .w2 = glp_alloc(
            1 as libc::c_int + n0_max,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        (*fi)
            .w3 = glp_alloc(
            1 as libc::c_int + n0_max,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
    }
    if (*fi).scf.nn_max != nn_max {
        if !((*fi).scf.ifu.f).is_null() {
            glp_free((*fi).scf.ifu.f as *mut libc::c_void);
        }
        if !((*fi).scf.ifu.u).is_null() {
            glp_free((*fi).scf.ifu.u as *mut libc::c_void);
        }
        (*fi)
            .scf
            .ifu
            .f = glp_alloc(
            nn_max * nn_max,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        (*fi)
            .scf
            .ifu
            .u = glp_alloc(
            nn_max * nn_max,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
    }
    if old_n0_max < n0_max || (*fi).scf.nn_max != nn_max {
        if !((*fi).scf.pp_ind).is_null() {
            glp_free((*fi).scf.pp_ind as *mut libc::c_void);
        }
        if !((*fi).scf.pp_inv).is_null() {
            glp_free((*fi).scf.pp_inv as *mut libc::c_void);
        }
        if !((*fi).scf.qq_ind).is_null() {
            glp_free((*fi).scf.qq_ind as *mut libc::c_void);
        }
        if !((*fi).scf.qq_inv).is_null() {
            glp_free((*fi).scf.qq_inv as *mut libc::c_void);
        }
        if !((*fi).w4).is_null() {
            glp_free((*fi).w4 as *mut libc::c_void);
        }
        if !((*fi).w5).is_null() {
            glp_free((*fi).w5 as *mut libc::c_void);
        }
        (*fi)
            .scf
            .pp_ind = glp_alloc(
            1 as libc::c_int + n0_max + nn_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*fi)
            .scf
            .pp_inv = glp_alloc(
            1 as libc::c_int + n0_max + nn_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*fi)
            .scf
            .qq_ind = glp_alloc(
            1 as libc::c_int + n0_max + nn_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*fi)
            .scf
            .qq_inv = glp_alloc(
            1 as libc::c_int + n0_max + nn_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*fi)
            .w4 = glp_alloc(
            1 as libc::c_int + n0_max + nn_max,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        (*fi)
            .w5 = glp_alloc(
            1 as libc::c_int + n0_max + nn_max,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
    }
    (*fi).scf.n = n;
    (*fi).scf.n0 = n;
    (*fi).scf.nn_max = nn_max;
    (*fi).scf.nn = 0 as libc::c_int;
    (*fi).scf.rr_ref = _glp_sva_alloc_vecs((*fi).scf.sva, nn_max);
    (*fi).scf.ss_ref = _glp_sva_alloc_vecs((*fi).scf.sva, nn_max);
    (*fi).scf.ifu.n_max = nn_max;
    (*fi).scf.ifu.n = 0 as libc::c_int;
    k = 1 as libc::c_int;
    while k <= n {
        *((*fi).scf.pp_ind).offset(k as isize) = k;
        *((*fi).scf.pp_inv).offset(k as isize) = k;
        *((*fi).scf.qq_ind).offset(k as isize) = k;
        *((*fi).scf.qq_inv).offset(k as isize) = k;
        k += 1;
        k;
    }
    if ret == 0 as libc::c_int {
        (*fi).valid = 1 as libc::c_int;
    }
    return ret;
}
pub unsafe extern "C" fn _glp_scfint_update(
    mut fi: *mut SCFINT,
    mut upd: libc::c_int,
    mut j: libc::c_int,
    mut len: libc::c_int,
    mut ind: *const libc::c_int,
    mut val: *const libc::c_double,
) -> libc::c_int {
    let mut n: libc::c_int = (*fi).scf.n;
    let mut n0: libc::c_int = (*fi).scf.n0;
    let mut nn: libc::c_int = (*fi).scf.nn;
    let mut pp_ind: *mut libc::c_int = (*fi).scf.pp_ind;
    let mut qq_ind: *mut libc::c_int = (*fi).scf.qq_ind;
    let mut qq_inv: *mut libc::c_int = (*fi).scf.qq_inv;
    let mut bf: *mut libc::c_double = (*fi).w4;
    let mut dg: *mut libc::c_double = (*fi).w5;
    let mut k: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    ((*fi).valid != 0
        || {
            glp_assert_(
                b"fi->valid\0" as *const u8 as *const libc::c_char,
                b"bflib/scfint.c\0" as *const u8 as *const libc::c_char,
                148 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (0 as libc::c_int <= n && n <= n0 + nn
        || {
            glp_assert_(
                b"0 <= n && n <= n0+nn\0" as *const u8 as *const libc::c_char,
                b"bflib/scfint.c\0" as *const u8 as *const libc::c_char,
                149 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = 1 as libc::c_int;
    while k <= n0 + nn {
        *bf.offset(k as isize) = 0.0f64;
        k += 1;
        k;
    }
    t = 1 as libc::c_int;
    while t <= len {
        k = *ind.offset(t as isize);
        (1 as libc::c_int <= k && k <= n
            || {
                glp_assert_(
                    b"1 <= k && k <= n\0" as *const u8 as *const libc::c_char,
                    b"bflib/scfint.c\0" as *const u8 as *const libc::c_char,
                    155 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*pp_ind.offset(k as isize) == k
            || {
                glp_assert_(
                    b"pp_ind[k] == k\0" as *const u8 as *const libc::c_char,
                    b"bflib/scfint.c\0" as *const u8 as *const libc::c_char,
                    157 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*bf.offset(k as isize) == 0.0f64
            || {
                glp_assert_(
                    b"bf[k] == 0.0\0" as *const u8 as *const libc::c_char,
                    b"bflib/scfint.c\0" as *const u8 as *const libc::c_char,
                    159 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*val.offset(t as isize) != 0.0f64
            || {
                glp_assert_(
                    b"val[t] != 0.0\0" as *const u8 as *const libc::c_char,
                    b"bflib/scfint.c\0" as *const u8 as *const libc::c_char,
                    160 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        *bf.offset(k as isize) = *val.offset(t as isize);
        t += 1;
        t;
    }
    k = 1 as libc::c_int;
    while k <= n0 + nn {
        *dg.offset(k as isize) = 0.0f64;
        k += 1;
        k;
    }
    (1 as libc::c_int <= j && j <= n
        || {
            glp_assert_(
                b"1 <= j && j <= n\0" as *const u8 as *const libc::c_char,
                b"bflib/scfint.c\0" as *const u8 as *const libc::c_char,
                166 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    *dg
        .offset(
            *((*fi).scf.qq_inv).offset(j as isize) as isize,
        ) = 1 as libc::c_int as libc::c_double;
    ret = _glp_scf_update_aug(
        &mut (*fi).scf,
        &mut *bf.offset(0 as libc::c_int as isize),
        &mut *dg.offset(0 as libc::c_int as isize),
        &mut *bf.offset(n0 as isize),
        &mut *dg.offset(n0 as isize),
        0.0f64,
        upd,
        (*fi).w1,
        (*fi).w2,
        (*fi).w3,
    );
    if ret == 0 as libc::c_int {
        let mut i1: libc::c_int = 0;
        let mut i2: libc::c_int = 0;
        i1 = *qq_inv.offset(j as isize);
        i2 = *qq_inv.offset((n0 + nn + 1 as libc::c_int) as isize);
        *qq_ind.offset(i1 as isize) = n0 + nn + 1 as libc::c_int;
        *qq_inv.offset((n0 + nn + 1 as libc::c_int) as isize) = i1;
        *qq_ind.offset(i2 as isize) = j;
        *qq_inv.offset(j as isize) = i2;
    } else {
        (*fi).valid = 0 as libc::c_int;
    }
    return ret;
}
pub unsafe extern "C" fn _glp_scfint_ftran(
    mut fi: *mut SCFINT,
    mut x: *mut libc::c_double,
) {
    ((*fi).valid != 0
        || {
            glp_assert_(
                b"fi->valid\0" as *const u8 as *const libc::c_char,
                b"bflib/scfint.c\0" as *const u8 as *const libc::c_char,
                184 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_scf_a_solve(&mut (*fi).scf, x, (*fi).w4, (*fi).w5, (*fi).w1, (*fi).w2);
}
pub unsafe extern "C" fn _glp_scfint_btran(
    mut fi: *mut SCFINT,
    mut x: *mut libc::c_double,
) {
    ((*fi).valid != 0
        || {
            glp_assert_(
                b"fi->valid\0" as *const u8 as *const libc::c_char,
                b"bflib/scfint.c\0" as *const u8 as *const libc::c_char,
                191 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_scf_at_solve(&mut (*fi).scf, x, (*fi).w4, (*fi).w5, (*fi).w1, (*fi).w2);
}
pub unsafe extern "C" fn _glp_scfint_estimate(mut fi: *mut SCFINT) -> libc::c_double {
    let mut norm: libc::c_double = 0.;
    ((*fi).valid != 0
        || {
            glp_assert_(
                b"fi->valid\0" as *const u8 as *const libc::c_char,
                b"bflib/scfint.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*fi).scf.n == (*fi).scf.n0
        || {
            glp_assert_(
                b"fi->scf.n == fi->scf.n0\0" as *const u8 as *const libc::c_char,
                b"bflib/scfint.c\0" as *const u8 as *const libc::c_char,
                200 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    match (*fi).scf.type_0 {
        1 => {
            norm = _glp_luf_estimate_norm((*fi).scf.a0.luf, (*fi).w1, (*fi).w2);
        }
        2 => {
            norm = _glp_btf_estimate_norm(
                (*fi).scf.a0.btf,
                (*fi).w1,
                (*fi).w2,
                (*fi).w3,
                (*fi).w4,
            );
        }
        _ => {
            (fi != fi
                || {
                    glp_assert_(
                        b"fi != fi\0" as *const u8 as *const libc::c_char,
                        b"bflib/scfint.c\0" as *const u8 as *const libc::c_char,
                        210 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    return norm;
}
pub unsafe extern "C" fn _glp_scfint_delete(mut fi: *mut SCFINT) {
    match (*fi).scf.type_0 {
        1 => {
            _glp_lufint_delete((*fi).u.lufi);
        }
        2 => {
            _glp_btfint_delete((*fi).u.btfi);
        }
        _ => {
            (fi != fi
                || {
                    glp_assert_(
                        b"fi != fi\0" as *const u8 as *const libc::c_char,
                        b"bflib/scfint.c\0" as *const u8 as *const libc::c_char,
                        225 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    if !((*fi).scf.ifu.f).is_null() {
        glp_free((*fi).scf.ifu.f as *mut libc::c_void);
    }
    if !((*fi).scf.ifu.u).is_null() {
        glp_free((*fi).scf.ifu.u as *mut libc::c_void);
    }
    if !((*fi).scf.pp_ind).is_null() {
        glp_free((*fi).scf.pp_ind as *mut libc::c_void);
    }
    if !((*fi).scf.pp_inv).is_null() {
        glp_free((*fi).scf.pp_inv as *mut libc::c_void);
    }
    if !((*fi).scf.qq_ind).is_null() {
        glp_free((*fi).scf.qq_ind as *mut libc::c_void);
    }
    if !((*fi).scf.qq_inv).is_null() {
        glp_free((*fi).scf.qq_inv as *mut libc::c_void);
    }
    if !((*fi).w1).is_null() {
        glp_free((*fi).w1 as *mut libc::c_void);
    }
    if !((*fi).w2).is_null() {
        glp_free((*fi).w2 as *mut libc::c_void);
    }
    if !((*fi).w3).is_null() {
        glp_free((*fi).w3 as *mut libc::c_void);
    }
    if !((*fi).w4).is_null() {
        glp_free((*fi).w4 as *mut libc::c_void);
    }
    if !((*fi).w5).is_null() {
        glp_free((*fi).w5 as *mut libc::c_void);
    }
    glp_free(fi as *mut libc::c_void);
}
