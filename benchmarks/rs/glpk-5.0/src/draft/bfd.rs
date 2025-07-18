use ::libc;
extern "C" {
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
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
    fn _glp_fhvint_estimate(fi: *mut FHVINT) -> libc::c_double;
    fn _glp_fhvint_factorize(
        fi: *mut FHVINT,
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
    fn _glp_fhvint_create() -> *mut FHVINT;
    fn _glp_fhvint_delete(fi: *mut FHVINT);
    fn _glp_fhvint_ftran(fi: *mut FHVINT, x: *mut libc::c_double);
    fn _glp_fhvint_btran(fi: *mut FHVINT, x: *mut libc::c_double);
    fn _glp_fhvint_update(
        fi: *mut FHVINT,
        j: libc::c_int,
        len: libc::c_int,
        ind: *const libc::c_int,
        val: *const libc::c_double,
    ) -> libc::c_int;
    fn _glp_scfint_delete(fi: *mut SCFINT);
    fn _glp_scfint_create(type_0: libc::c_int) -> *mut SCFINT;
    fn _glp_scfint_factorize(
        fi: *mut SCFINT,
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
    fn _glp_scfint_estimate(fi: *mut SCFINT) -> libc::c_double;
    fn _glp_scfint_update(
        fi: *mut SCFINT,
        upd: libc::c_int,
        j: libc::c_int,
        len: libc::c_int,
        ind: *const libc::c_int,
        val: *const libc::c_double,
    ) -> libc::c_int;
    fn _glp_scfint_btran(fi: *mut SCFINT, x: *mut libc::c_double);
    fn _glp_scfint_ftran(fi: *mut SCFINT, x: *mut libc::c_double);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_bfcp {
    pub msg_lev: libc::c_int,
    pub type_0: libc::c_int,
    pub lu_size: libc::c_int,
    pub piv_tol: libc::c_double,
    pub piv_lim: libc::c_int,
    pub suhl: libc::c_int,
    pub eps_tol: libc::c_double,
    pub max_gro: libc::c_double,
    pub nfs_max: libc::c_int,
    pub upd_tol: libc::c_double,
    pub nrs_max: libc::c_int,
    pub rs_size: libc::c_int,
    pub foo_bar: [libc::c_double; 38],
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
pub struct BFD {
    pub valid: libc::c_int,
    pub type_0: libc::c_int,
    pub u: C2RustUnnamed,
    pub parm: glp_bfcp,
    pub upd_cnt: libc::c_int,
    pub b_norm: libc::c_double,
    pub i_norm: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub none: *mut libc::c_void,
    pub fhvi: *mut FHVINT,
    pub scfi: *mut SCFINT,
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
pub struct SCF {
    pub n: libc::c_int,
    pub n0: libc::c_int,
    pub type_0: libc::c_int,
    pub a0: C2RustUnnamed_1,
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
pub struct IFU {
    pub n_max: libc::c_int,
    pub n: libc::c_int,
    pub f: *mut libc::c_double,
    pub u: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub luf: *mut LUF,
    pub btf: *mut BTF,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FHVINT {
    pub valid: libc::c_int,
    pub fhv: FHV,
    pub lufi: *mut LUFINT,
    pub nfs_max: libc::c_int,
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
pub struct bfd_info {
    pub bfd: *mut BFD,
    pub col: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *mut libc::c_int,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    pub info: *mut libc::c_void,
}
pub unsafe extern "C" fn _glp_bfd_create_it() -> *mut BFD {
    let mut bfd: *mut BFD = 0 as *mut BFD;
    bfd = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<BFD>() as libc::c_ulong as libc::c_int,
    ) as *mut BFD;
    (*bfd).valid = 0 as libc::c_int;
    (*bfd).type_0 = 0 as libc::c_int;
    (*bfd).u.none = 0 as *mut libc::c_void;
    _glp_bfd_set_bfcp(bfd, 0 as *const libc::c_void);
    (*bfd).upd_cnt = 0 as libc::c_int;
    return bfd;
}
pub unsafe extern "C" fn _glp_bfd_get_bfcp(
    mut bfd: *mut BFD,
    mut parm: *mut libc::c_void,
) {
    memcpy(
        parm,
        &mut (*bfd).parm as *mut glp_bfcp as *const libc::c_void,
        ::std::mem::size_of::<glp_bfcp>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn _glp_bfd_set_bfcp(
    mut bfd: *mut BFD,
    mut parm: *const libc::c_void,
) {
    if parm.is_null() {
        memset(
            &mut (*bfd).parm as *mut glp_bfcp as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<glp_bfcp>() as libc::c_ulong,
        );
        (*bfd).parm.type_0 = 0 as libc::c_int + 0x1 as libc::c_int;
        (*bfd).parm.piv_tol = 0.10f64;
        (*bfd).parm.piv_lim = 4 as libc::c_int;
        (*bfd).parm.suhl = 1 as libc::c_int;
        (*bfd).parm.eps_tol = 2.2204460492503131e-16f64;
        (*bfd).parm.nfs_max = 100 as libc::c_int;
        (*bfd).parm.nrs_max = 70 as libc::c_int;
    } else {
        memcpy(
            &mut (*bfd).parm as *mut glp_bfcp as *mut libc::c_void,
            parm,
            ::std::mem::size_of::<glp_bfcp>() as libc::c_ulong,
        );
    };
}
unsafe extern "C" fn bfd_col(
    mut info_: *mut libc::c_void,
    mut j: libc::c_int,
    mut ind: *mut libc::c_int,
    mut val: *mut libc::c_double,
) -> libc::c_int {
    let mut info: *mut bfd_info = info_ as *mut bfd_info;
    let mut t: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut sum: libc::c_double = 0.;
    len = ((*info).col).unwrap()((*info).info, j, ind, val);
    sum = 0.0f64;
    t = 1 as libc::c_int;
    while t <= len {
        if *val.offset(t as isize) >= 0.0f64 {
            sum += *val.offset(t as isize);
        } else {
            sum -= *val.offset(t as isize);
        }
        t += 1;
        t;
    }
    if (*(*info).bfd).b_norm < sum {
        (*(*info).bfd).b_norm = sum;
    }
    return len;
}
pub unsafe extern "C" fn _glp_bfd_factorize(
    mut bfd: *mut BFD,
    mut m: libc::c_int,
    mut col1: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *mut libc::c_int,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    mut info1: *mut libc::c_void,
) -> libc::c_int {
    let mut info: bfd_info = bfd_info {
        bfd: 0 as *mut BFD,
        col: None,
        info: 0 as *mut libc::c_void,
    };
    let mut type_0: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    (*bfd).valid = 0 as libc::c_int;
    match (*bfd).parm.type_0 {
        1 => {
            type_0 = 1 as libc::c_int;
        }
        2 | 3 | 18 | 19 => {
            type_0 = 2 as libc::c_int;
        }
        _ => {
            (bfd != bfd
                || {
                    glp_assert_(
                        b"bfd != bfd\0" as *const u8 as *const libc::c_char,
                        b"draft/bfd.c\0" as *const u8 as *const libc::c_char,
                        159 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    match (*bfd).type_0 {
        0 => {}
        1 => {
            if type_0 != 1 as libc::c_int {
                (*bfd).type_0 = 0 as libc::c_int;
                _glp_fhvint_delete((*bfd).u.fhvi);
                (*bfd).u.fhvi = 0 as *mut FHVINT;
            }
        }
        2 => {
            if type_0 != 2 as libc::c_int {
                (*bfd).type_0 = 0 as libc::c_int;
                _glp_scfint_delete((*bfd).u.scfi);
                (*bfd).u.scfi = 0 as *mut SCFINT;
            }
        }
        _ => {
            (bfd != bfd
                || {
                    glp_assert_(
                        b"bfd != bfd\0" as *const u8 as *const libc::c_char,
                        b"draft/bfd.c\0" as *const u8 as *const libc::c_char,
                        180 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    if (*bfd).type_0 == 0 as libc::c_int {
        match type_0 {
            1 => {
                (*bfd).type_0 = 1 as libc::c_int;
                (((*bfd).u.fhvi).is_null()
                    || {
                        glp_assert_(
                            b"bfd->u.fhvi == NULL\0" as *const u8 as *const libc::c_char,
                            b"draft/bfd.c\0" as *const u8 as *const libc::c_char,
                            187 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*bfd).u.fhvi = _glp_fhvint_create();
            }
            2 => {
                (*bfd).type_0 = 2 as libc::c_int;
                (((*bfd).u.scfi).is_null()
                    || {
                        glp_assert_(
                            b"bfd->u.scfi == NULL\0" as *const u8 as *const libc::c_char,
                            b"draft/bfd.c\0" as *const u8 as *const libc::c_char,
                            192 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                if (*bfd).parm.type_0 & 0x10 as libc::c_int == 0 {
                    (*bfd).u.scfi = _glp_scfint_create(1 as libc::c_int);
                } else {
                    (*bfd).u.scfi = _glp_scfint_create(2 as libc::c_int);
                }
            }
            _ => {
                (type_0 != type_0
                    || {
                        glp_assert_(
                            b"type != type\0" as *const u8 as *const libc::c_char,
                            b"draft/bfd.c\0" as *const u8 as *const libc::c_char,
                            199 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
    }
    (*bfd).i_norm = 0.0f64;
    (*bfd).b_norm = (*bfd).i_norm;
    info.bfd = bfd;
    info.col = col1;
    info.info = info1;
    match (*bfd).type_0 {
        1 => {
            (*(*(*bfd).u.fhvi).lufi).sgf_piv_tol = (*bfd).parm.piv_tol;
            (*(*(*bfd).u.fhvi).lufi).sgf_piv_lim = (*bfd).parm.piv_lim;
            (*(*(*bfd).u.fhvi).lufi).sgf_suhl = (*bfd).parm.suhl;
            (*(*(*bfd).u.fhvi).lufi).sgf_eps_tol = (*bfd).parm.eps_tol;
            (*(*bfd).u.fhvi).nfs_max = (*bfd).parm.nfs_max;
            ret = _glp_fhvint_factorize(
                (*bfd).u.fhvi,
                m,
                Some(
                    bfd_col
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            libc::c_int,
                            *mut libc::c_int,
                            *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                &mut info as *mut bfd_info as *mut libc::c_void,
            );
            if ret == 0 as libc::c_int {
                (*bfd).i_norm = _glp_fhvint_estimate((*bfd).u.fhvi);
            } else {
                ret = 1 as libc::c_int;
            }
        }
        2 => {
            if (*(*bfd).u.scfi).scf.type_0 == 1 as libc::c_int {
                (*(*(*bfd).u.scfi).u.lufi).sgf_piv_tol = (*bfd).parm.piv_tol;
                (*(*(*bfd).u.scfi).u.lufi).sgf_piv_lim = (*bfd).parm.piv_lim;
                (*(*(*bfd).u.scfi).u.lufi).sgf_suhl = (*bfd).parm.suhl;
                (*(*(*bfd).u.scfi).u.lufi).sgf_eps_tol = (*bfd).parm.eps_tol;
            } else if (*(*bfd).u.scfi).scf.type_0 == 2 as libc::c_int {
                (*(*(*bfd).u.scfi).u.btfi).sgf_piv_tol = (*bfd).parm.piv_tol;
                (*(*(*bfd).u.scfi).u.btfi).sgf_piv_lim = (*bfd).parm.piv_lim;
                (*(*(*bfd).u.scfi).u.btfi).sgf_suhl = (*bfd).parm.suhl;
                (*(*(*bfd).u.scfi).u.btfi).sgf_eps_tol = (*bfd).parm.eps_tol;
            } else {
                (bfd != bfd
                    || {
                        glp_assert_(
                            b"bfd != bfd\0" as *const u8 as *const libc::c_char,
                            b"draft/bfd.c\0" as *const u8 as *const libc::c_char,
                            238 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            (*(*bfd).u.scfi).nn_max = (*bfd).parm.nrs_max;
            ret = _glp_scfint_factorize(
                (*bfd).u.scfi,
                m,
                Some(
                    bfd_col
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            libc::c_int,
                            *mut libc::c_int,
                            *mut libc::c_double,
                        ) -> libc::c_int,
                ),
                &mut info as *mut bfd_info as *mut libc::c_void,
            );
            if ret == 0 as libc::c_int {
                (*bfd).i_norm = _glp_scfint_estimate((*bfd).u.scfi);
            } else {
                ret = 1 as libc::c_int;
            }
        }
        _ => {
            (bfd != bfd
                || {
                    glp_assert_(
                        b"bfd != bfd\0" as *const u8 as *const libc::c_char,
                        b"draft/bfd.c\0" as *const u8 as *const libc::c_char,
                        249 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    if ret == 0 as libc::c_int {
        let mut cond: libc::c_double = 0.;
        (*bfd).valid = 1 as libc::c_int;
    }
    (*bfd).upd_cnt = 0 as libc::c_int;
    return ret;
}
pub unsafe extern "C" fn _glp_bfd_condest(mut bfd: *mut BFD) -> libc::c_double {
    let mut cond: libc::c_double = 0.;
    ((*bfd).valid != 0
        || {
            glp_assert_(
                b"bfd->valid\0" as *const u8 as *const libc::c_char,
                b"draft/bfd.c\0" as *const u8 as *const libc::c_char,
                309 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    cond = (*bfd).b_norm * (*bfd).i_norm;
    if cond < 1.0f64 {
        cond = 1.0f64;
    }
    return cond;
}
pub unsafe extern "C" fn _glp_bfd_ftran(mut bfd: *mut BFD, mut x: *mut libc::c_double) {
    ((*bfd).valid != 0
        || {
            glp_assert_(
                b"bfd->valid\0" as *const u8 as *const libc::c_char,
                b"draft/bfd.c\0" as *const u8 as *const libc::c_char,
                330 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    match (*bfd).type_0 {
        1 => {
            _glp_fhvint_ftran((*bfd).u.fhvi, x);
        }
        2 => {
            _glp_scfint_ftran((*bfd).u.scfi, x);
        }
        _ => {
            (bfd != bfd
                || {
                    glp_assert_(
                        b"bfd != bfd\0" as *const u8 as *const libc::c_char,
                        b"draft/bfd.c\0" as *const u8 as *const libc::c_char,
                        339 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn _glp_bfd_ftran_s(mut bfd: *mut BFD, mut x: *mut FVS) {
    let mut n: libc::c_int = (*x).n;
    let mut ind: *mut libc::c_int = (*x).ind;
    let mut vec: *mut libc::c_double = (*x).vec;
    let mut j: libc::c_int = 0;
    let mut nnz: libc::c_int = 0 as libc::c_int;
    _glp_bfd_ftran(bfd, vec);
    j = n;
    while j >= 1 as libc::c_int {
        if *vec.offset(j as isize) != 0.0f64 {
            nnz += 1;
            *ind.offset(nnz as isize) = j;
        }
        j -= 1;
        j;
    }
    (*x).nnz = nnz;
}
pub unsafe extern "C" fn _glp_bfd_btran(mut bfd: *mut BFD, mut x: *mut libc::c_double) {
    ((*bfd).valid != 0
        || {
            glp_assert_(
                b"bfd->valid\0" as *const u8 as *const libc::c_char,
                b"draft/bfd.c\0" as *const u8 as *const libc::c_char,
                389 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    match (*bfd).type_0 {
        1 => {
            _glp_fhvint_btran((*bfd).u.fhvi, x);
        }
        2 => {
            _glp_scfint_btran((*bfd).u.scfi, x);
        }
        _ => {
            (bfd != bfd
                || {
                    glp_assert_(
                        b"bfd != bfd\0" as *const u8 as *const libc::c_char,
                        b"draft/bfd.c\0" as *const u8 as *const libc::c_char,
                        398 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn _glp_bfd_btran_s(mut bfd: *mut BFD, mut x: *mut FVS) {
    let mut n: libc::c_int = (*x).n;
    let mut ind: *mut libc::c_int = (*x).ind;
    let mut vec: *mut libc::c_double = (*x).vec;
    let mut j: libc::c_int = 0;
    let mut nnz: libc::c_int = 0 as libc::c_int;
    _glp_bfd_btran(bfd, vec);
    j = n;
    while j >= 1 as libc::c_int {
        if *vec.offset(j as isize) != 0.0f64 {
            nnz += 1;
            *ind.offset(nnz as isize) = j;
        }
        j -= 1;
        j;
    }
    (*x).nnz = nnz;
}
pub unsafe extern "C" fn _glp_bfd_update(
    mut bfd: *mut BFD,
    mut j: libc::c_int,
    mut len: libc::c_int,
    mut ind: *const libc::c_int,
    mut val: *const libc::c_double,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ((*bfd).valid != 0
        || {
            glp_assert_(
                b"bfd->valid\0" as *const u8 as *const libc::c_char,
                b"draft/bfd.c\0" as *const u8 as *const libc::c_char,
                440 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    match (*bfd).type_0 {
        1 => {
            ret = _glp_fhvint_update((*bfd).u.fhvi, j, len, ind, val);
            match ret {
                0 => {}
                1 => {
                    ret = 1 as libc::c_int;
                }
                2 | 3 => {
                    ret = 2 as libc::c_int;
                }
                4 => {
                    ret = 4 as libc::c_int;
                }
                5 => {
                    ret = 3 as libc::c_int;
                }
                _ => {
                    (ret != ret
                        || {
                            glp_assert_(
                                b"ret != ret\0" as *const u8 as *const libc::c_char,
                                b"draft/bfd.c\0" as *const u8 as *const libc::c_char,
                                462 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
            }
        }
        2 => {
            match (*bfd).parm.type_0 & 0xf as libc::c_int {
                2 => {
                    ret = _glp_scfint_update(
                        (*bfd).u.scfi,
                        1 as libc::c_int,
                        j,
                        len,
                        ind,
                        val,
                    );
                }
                3 => {
                    ret = _glp_scfint_update(
                        (*bfd).u.scfi,
                        2 as libc::c_int,
                        j,
                        len,
                        ind,
                        val,
                    );
                }
                _ => {
                    (bfd != bfd
                        || {
                            glp_assert_(
                                b"bfd != bfd\0" as *const u8 as *const libc::c_char,
                                b"draft/bfd.c\0" as *const u8 as *const libc::c_char,
                                475 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
            }
            match ret {
                0 => {}
                1 => {
                    ret = 4 as libc::c_int;
                }
                2 => {
                    ret = 2 as libc::c_int;
                }
                _ => {
                    (ret != ret
                        || {
                            glp_assert_(
                                b"ret != ret\0" as *const u8 as *const libc::c_char,
                                b"draft/bfd.c\0" as *const u8 as *const libc::c_char,
                                488 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
            }
        }
        _ => {
            (bfd != bfd
                || {
                    glp_assert_(
                        b"bfd != bfd\0" as *const u8 as *const libc::c_char,
                        b"draft/bfd.c\0" as *const u8 as *const libc::c_char,
                        493 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    if ret != 0 as libc::c_int {
        (*bfd).valid = 0 as libc::c_int;
    }
    if ret == 0 as libc::c_int {
        (*bfd).upd_cnt += 1;
        (*bfd).upd_cnt;
    }
    return ret;
}
pub unsafe extern "C" fn _glp_bfd_get_count(mut bfd: *mut BFD) -> libc::c_int {
    return (*bfd).upd_cnt;
}
pub unsafe extern "C" fn _glp_bfd_delete_it(mut bfd: *mut BFD) {
    match (*bfd).type_0 {
        0 => {}
        1 => {
            _glp_fhvint_delete((*bfd).u.fhvi);
        }
        2 => {
            _glp_scfint_delete((*bfd).u.scfi);
        }
        _ => {
            (bfd != bfd
                || {
                    glp_assert_(
                        b"bfd != bfd\0" as *const u8 as *const libc::c_char,
                        b"draft/bfd.c\0" as *const u8 as *const libc::c_char,
                        532 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    glp_free(bfd as *mut libc::c_void);
}
