use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_difftime(t1: libc::c_double, t0: libc::c_double) -> libc::c_double;
    fn glp_time() -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn abs(_: libc::c_int) -> libc::c_int;
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
    fn _glp_fvs_alloc_vec(x: *mut FVS, n: libc::c_int);
    fn _glp_fvs_gather_vec(x: *mut FVS, eps: libc::c_double);
    fn _glp_fvs_free_vec(x: *mut FVS);
    fn _glp_bfd_condest(bfd: *mut BFD) -> libc::c_double;
    fn _glp_bfd_get_count(bfd: *mut BFD) -> libc::c_int;
    fn _glp_spx_alloc_at(lp: *mut SPXLP, at: *mut SPXAT);
    fn _glp_spx_build_at(lp: *mut SPXLP, at: *mut SPXAT);
    fn _glp_spx_update_beta_s(
        lp: *mut SPXLP,
        beta: *mut libc::c_double,
        p: libc::c_int,
        p_flag: libc::c_int,
        q: libc::c_int,
        tcol: *const FVS,
    );
    fn _glp_spx_free_at(lp: *mut SPXLP, at: *mut SPXAT);
    fn _glp_spx_eval_trow1(
        lp: *mut SPXLP,
        at: *mut SPXAT,
        rho: *const libc::c_double,
        trow: *mut libc::c_double,
    );
    fn _glp_spx_factorize(lp: *mut SPXLP) -> libc::c_int;
    fn _glp_spx_eval_dj(
        lp: *mut SPXLP,
        pi: *const libc::c_double,
        j: libc::c_int,
    ) -> libc::c_double;
    fn _glp_spx_eval_pi(lp: *mut SPXLP, pi: *mut libc::c_double);
    fn _glp_spx_eval_tcol(lp: *mut SPXLP, j: libc::c_int, tcol: *mut libc::c_double);
    fn _glp_spx_eval_obj(lp: *mut SPXLP, beta: *const libc::c_double) -> libc::c_double;
    fn _glp_spx_eval_rho(lp: *mut SPXLP, i: libc::c_int, rho: *mut libc::c_double);
    fn _glp_spx_eval_beta(lp: *mut SPXLP, beta: *mut libc::c_double);
    fn _glp_spx_update_d_s(
        lp: *mut SPXLP,
        d: *mut libc::c_double,
        p: libc::c_int,
        q: libc::c_int,
        trow: *const FVS,
        tcol: *const FVS,
    ) -> libc::c_double;
    fn _glp_spx_change_basis(
        lp: *mut SPXLP,
        p: libc::c_int,
        p_flag: libc::c_int,
        q: libc::c_int,
    );
    fn _glp_spx_update_invb(
        lp: *mut SPXLP,
        i: libc::c_int,
        k: libc::c_int,
    ) -> libc::c_int;
    fn _glp_spx_alloc_nt(lp: *mut SPXLP, nt: *mut SPXNT);
    fn _glp_spx_init_nt(lp: *mut SPXLP, nt: *mut SPXNT);
    fn _glp_spx_build_nt(lp: *mut SPXLP, nt: *mut SPXNT);
    fn _glp_spx_update_nt(
        lp: *mut SPXLP,
        nt: *mut SPXNT,
        p: libc::c_int,
        q: libc::c_int,
    );
    fn _glp_spx_nt_prod(
        lp: *mut SPXLP,
        nt: *mut SPXNT,
        y: *mut libc::c_double,
        ign: libc::c_int,
        s: libc::c_double,
        x: *const libc::c_double,
    );
    fn _glp_spx_free_nt(lp: *mut SPXLP, nt: *mut SPXNT);
    fn _glp_spx_chuzc_sel(
        lp: *mut SPXLP,
        d: *const libc::c_double,
        tol: libc::c_double,
        tol1: libc::c_double,
        list: *mut libc::c_int,
    ) -> libc::c_int;
    fn _glp_spx_chuzc_std(
        lp: *mut SPXLP,
        d: *const libc::c_double,
        num: libc::c_int,
        list: *const libc::c_int,
    ) -> libc::c_int;
    fn _glp_spx_alloc_se(lp: *mut SPXLP, se: *mut SPXSE);
    fn _glp_spx_reset_refsp(lp: *mut SPXLP, se: *mut SPXSE);
    fn _glp_spx_chuzc_pse(
        lp: *mut SPXLP,
        se: *mut SPXSE,
        d: *const libc::c_double,
        num: libc::c_int,
        list: *const libc::c_int,
    ) -> libc::c_int;
    fn _glp_spx_update_gamma(
        lp: *mut SPXLP,
        se: *mut SPXSE,
        p: libc::c_int,
        q: libc::c_int,
        trow: *const libc::c_double,
        tcol: *const libc::c_double,
    ) -> libc::c_double;
    fn _glp_spx_free_se(lp: *mut SPXLP, se: *mut SPXSE);
    fn _glp_spx_chuzr_std(
        lp: *mut SPXLP,
        phase: libc::c_int,
        beta: *const libc::c_double,
        q: libc::c_int,
        s: libc::c_double,
        tcol: *const libc::c_double,
        p_flag: *mut libc::c_int,
        tol_piv: libc::c_double,
        tol: libc::c_double,
        tol1: libc::c_double,
    ) -> libc::c_int;
    fn _glp_spx_chuzr_harris(
        lp: *mut SPXLP,
        phase: libc::c_int,
        beta: *const libc::c_double,
        q: libc::c_int,
        s: libc::c_double,
        tcol: *const libc::c_double,
        p_flag: *mut libc::c_int,
        tol_piv: libc::c_double,
        tol: libc::c_double,
        tol1: libc::c_double,
    ) -> libc::c_int;
    fn _glp_spx_ls_eval_bp(
        lp: *mut SPXLP,
        beta: *const libc::c_double,
        q: libc::c_int,
        dq: libc::c_double,
        tcol: *const libc::c_double,
        tol_piv: libc::c_double,
        bp: *mut SPXBP,
    ) -> libc::c_int;
    fn _glp_spx_ls_select_bp(
        lp: *mut SPXLP,
        tcol: *const libc::c_double,
        nbp: libc::c_int,
        bp: *mut SPXBP,
        num: libc::c_int,
        slope: *mut libc::c_double,
        teta_lim: libc::c_double,
    ) -> libc::c_int;
    fn _glp_spx_init_lp(lp: *mut SPXLP, P: *mut glp_prob, excl: libc::c_int);
    fn _glp_spx_alloc_lp(lp: *mut SPXLP);
    fn _glp_spx_build_lp(
        lp: *mut SPXLP,
        P: *mut glp_prob,
        excl: libc::c_int,
        shift: libc::c_int,
        map: *mut libc::c_int,
    );
    fn _glp_spx_build_basis(lp: *mut SPXLP, P: *mut glp_prob, map: *const libc::c_int);
    fn _glp_spx_store_basis(
        lp: *mut SPXLP,
        P: *mut glp_prob,
        map: *const libc::c_int,
        daeh: *mut libc::c_int,
    );
    fn _glp_spx_store_sol(
        lp: *mut SPXLP,
        P: *mut glp_prob,
        shift: libc::c_int,
        map: *const libc::c_int,
        daeh: *const libc::c_int,
        beta: *const libc::c_double,
        pi: *const libc::c_double,
        d: *const libc::c_double,
    );
    fn _glp_spx_free_lp(lp: *mut SPXLP);
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
pub struct glp_prob {
    pub pool: *mut DMP,
    pub tree: *mut glp_tree,
    pub name: *mut libc::c_char,
    pub obj: *mut libc::c_char,
    pub dir: libc::c_int,
    pub c0: libc::c_double,
    pub m_max: libc::c_int,
    pub n_max: libc::c_int,
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub nnz: libc::c_int,
    pub row: *mut *mut GLPROW,
    pub col: *mut *mut GLPCOL,
    pub r_tree: *mut AVL,
    pub c_tree: *mut AVL,
    pub valid: libc::c_int,
    pub head: *mut libc::c_int,
    pub bfd: *mut BFD,
    pub pbs_stat: libc::c_int,
    pub dbs_stat: libc::c_int,
    pub obj_val: libc::c_double,
    pub it_cnt: libc::c_int,
    pub some: libc::c_int,
    pub ipt_stat: libc::c_int,
    pub ipt_obj: libc::c_double,
    pub mip_stat: libc::c_int,
    pub mip_obj: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPCOL {
    pub j: libc::c_int,
    pub name: *mut libc::c_char,
    pub node: *mut AVLNODE,
    pub kind: libc::c_int,
    pub type_0: libc::c_int,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub coef: libc::c_double,
    pub ptr: *mut GLPAIJ,
    pub sjj: libc::c_double,
    pub stat: libc::c_int,
    pub bind: libc::c_int,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
    pub pval: libc::c_double,
    pub dval: libc::c_double,
    pub mipx: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPAIJ {
    pub row: *mut GLPROW,
    pub col: *mut GLPCOL,
    pub val: libc::c_double,
    pub r_prev: *mut GLPAIJ,
    pub r_next: *mut GLPAIJ,
    pub c_prev: *mut GLPAIJ,
    pub c_next: *mut GLPAIJ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPROW {
    pub i: libc::c_int,
    pub name: *mut libc::c_char,
    pub node: *mut AVLNODE,
    pub level: libc::c_int,
    pub origin: libc::c_uchar,
    pub klass: libc::c_uchar,
    pub type_0: libc::c_int,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut GLPAIJ,
    pub rii: libc::c_double,
    pub stat: libc::c_int,
    pub bind: libc::c_int,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
    pub pval: libc::c_double,
    pub dval: libc::c_double,
    pub mipx: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_smcp {
    pub msg_lev: libc::c_int,
    pub meth: libc::c_int,
    pub pricing: libc::c_int,
    pub r_test: libc::c_int,
    pub tol_bnd: libc::c_double,
    pub tol_dj: libc::c_double,
    pub tol_piv: libc::c_double,
    pub obj_ll: libc::c_double,
    pub obj_ul: libc::c_double,
    pub it_lim: libc::c_int,
    pub tm_lim: libc::c_int,
    pub out_frq: libc::c_int,
    pub out_dly: libc::c_int,
    pub presolve: libc::c_int,
    pub excl: libc::c_int,
    pub shift: libc::c_int,
    pub aorn: libc::c_int,
    pub foo_bar: [libc::c_double; 33],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csa {
    pub lp: *mut SPXLP,
    pub dir: libc::c_int,
    pub fz: libc::c_double,
    pub orig_c: *mut libc::c_double,
    pub orig_l: *mut libc::c_double,
    pub orig_u: *mut libc::c_double,
    pub at: *mut SPXAT,
    pub nt: *mut SPXNT,
    pub phase: libc::c_int,
    pub beta: *mut libc::c_double,
    pub beta_st: libc::c_int,
    pub d: *mut libc::c_double,
    pub d_st: libc::c_int,
    pub se: *mut SPXSE,
    pub num: libc::c_int,
    pub list: *mut libc::c_int,
    pub q: libc::c_int,
    pub tcol: FVS,
    pub bp: *mut SPXBP,
    pub p: libc::c_int,
    pub p_flag: libc::c_int,
    pub trow: FVS,
    pub work: FVS,
    pub p_stat: libc::c_int,
    pub d_stat: libc::c_int,
    pub msg_lev: libc::c_int,
    pub r_test: libc::c_int,
    pub tol_bnd: libc::c_double,
    pub tol_bnd1: libc::c_double,
    pub tol_dj: libc::c_double,
    pub tol_dj1: libc::c_double,
    pub tol_piv: libc::c_double,
    pub it_lim: libc::c_int,
    pub tm_lim: libc::c_int,
    pub out_frq: libc::c_int,
    pub out_dly: libc::c_int,
    pub tm_beg: libc::c_double,
    pub it_beg: libc::c_int,
    pub it_cnt: libc::c_int,
    pub it_dpy: libc::c_int,
    pub tm_dpy: libc::c_double,
    pub inv_cnt: libc::c_int,
    pub degen: libc::c_int,
    pub ns_cnt: libc::c_int,
    pub ls_cnt: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPXBP {
    pub i: libc::c_int,
    pub teta: libc::c_double,
    pub dc: libc::c_double,
    pub dz: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPXSE {
    pub valid: libc::c_int,
    pub refsp: *mut libc::c_char,
    pub gamma: *mut libc::c_double,
    pub work: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPXNT {
    pub ptr: *mut libc::c_int,
    pub len: *mut libc::c_int,
    pub ind: *mut libc::c_int,
    pub val: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPXAT {
    pub ptr: *mut libc::c_int,
    pub ind: *mut libc::c_int,
    pub val: *mut libc::c_double,
    pub work: *mut libc::c_double,
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
unsafe extern "C" fn set_penalty(
    mut csa: *mut csa,
    mut tol: libc::c_double,
    mut tol1: libc::c_double,
) -> libc::c_int {
    let mut lp: *mut SPXLP = (*csa).lp;
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut beta: *mut libc::c_double = (*csa).beta;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut t: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    k = 0 as libc::c_int;
    while k <= n {
        *c.offset(k as isize) = 0.0f64;
        k += 1;
        k;
    }
    i = 1 as libc::c_int;
    while i <= m {
        k = *head.offset(i as isize);
        t = *l.offset(k as isize);
        if t != -1.7976931348623157e+308f64 {
            eps = tol + tol1 * (if t >= 0.0f64 { t } else { -t });
            if *beta.offset(i as isize) < t - eps {
                *c.offset(k as isize) = -1.0f64;
                count += 1;
                count;
            }
        }
        t = *u.offset(k as isize);
        if t != 1.7976931348623157e+308f64 {
            eps = tol + tol1 * (if t >= 0.0f64 { t } else { -t });
            if *beta.offset(i as isize) > t + eps {
                *c.offset(k as isize) = 1.0f64;
                count += 1;
                count;
            }
        }
        i += 1;
        i;
    }
    return count;
}
unsafe extern "C" fn check_feas(
    mut csa: *mut csa,
    mut phase: libc::c_int,
    mut tol: libc::c_double,
    mut tol1: libc::c_double,
) -> libc::c_int {
    let mut lp: *mut SPXLP = (*csa).lp;
    let mut m: libc::c_int = (*lp).m;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut beta: *mut libc::c_double = (*csa).beta;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut orig: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut lk: libc::c_double = 0.;
    let mut uk: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    (phase == 1 as libc::c_int || phase == 2 as libc::c_int
        || {
            glp_assert_(
                b"phase == 1 || phase == 2\0" as *const u8 as *const libc::c_char,
                b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                295 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    i = 1 as libc::c_int;
    while i <= m {
        k = *head.offset(i as isize);
        if phase == 1 as libc::c_int && *c.offset(k as isize) < 0.0f64 {
            lk = -1.7976931348623157e+308f64;
            uk = *l.offset(k as isize);
            orig = 0 as libc::c_int;
        } else if phase == 1 as libc::c_int && *c.offset(k as isize) > 0.0f64 {
            lk = *u.offset(k as isize);
            uk = 1.7976931348623157e+308f64;
            orig = 0 as libc::c_int;
        } else {
            lk = *l.offset(k as isize);
            uk = *u.offset(k as isize);
            orig = 1 as libc::c_int;
        }
        if lk != -1.7976931348623157e+308f64 {
            eps = tol + tol1 * (if lk >= 0.0f64 { lk } else { -lk });
            if *beta.offset(i as isize) < lk - eps {
                if orig != 0 {
                    ret = 2 as libc::c_int;
                    break;
                } else {
                    ret = 1 as libc::c_int;
                }
            }
        }
        if uk != 1.7976931348623157e+308f64 {
            eps = tol + tol1 * (if uk >= 0.0f64 { uk } else { -uk });
            if *beta.offset(i as isize) > uk + eps {
                if orig != 0 {
                    ret = 2 as libc::c_int;
                    break;
                } else {
                    ret = 1 as libc::c_int;
                }
            }
        }
        i += 1;
        i;
    }
    return ret;
}
unsafe extern "C" fn adjust_penalty(
    mut csa: *mut csa,
    mut num: libc::c_int,
    mut ind: *const libc::c_int,
    mut tol: libc::c_double,
    mut tol1: libc::c_double,
) -> libc::c_int {
    let mut lp: *mut SPXLP = (*csa).lp;
    let mut m: libc::c_int = (*lp).m;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut beta: *mut libc::c_double = (*csa).beta;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut lk: libc::c_double = 0.;
    let mut uk: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    ((*csa).phase == 1 as libc::c_int
        || {
            glp_assert_(
                b"csa->phase == 1\0" as *const u8 as *const libc::c_char,
                b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                416 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    t = 1 as libc::c_int;
    while t <= num {
        i = *ind.offset(t as isize);
        (1 as libc::c_int <= i && i <= m
            || {
                glp_assert_(
                    b"1 <= i && i <= m\0" as *const u8 as *const libc::c_char,
                    b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                    420 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        k = *head.offset(i as isize);
        if *c.offset(k as isize) < 0.0f64 {
            lk = *l.offset(k as isize);
            (lk != -1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"lk != -DBL_MAX\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                        425 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            eps = tol + tol1 * (if lk >= 0.0f64 { lk } else { -lk });
            if *beta.offset(i as isize) >= lk - eps {
                *c.offset(k as isize) = 0.0f64;
                cnt += 1;
                cnt;
            }
        } else if *c.offset(k as isize) > 0.0f64 {
            uk = *u.offset(k as isize);
            (uk != 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"uk != +DBL_MAX\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                        435 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            eps = tol + tol1 * (if uk >= 0.0f64 { uk } else { -uk });
            if *beta.offset(i as isize) <= uk + eps {
                *c.offset(k as isize) = 0.0f64;
                cnt += 1;
                cnt;
            }
        }
        t += 1;
        t;
    }
    return cnt;
}
unsafe extern "C" fn choose_pivot(mut csa: *mut csa) -> libc::c_int {
    let mut lp: *mut SPXLP = (*csa).lp;
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut beta: *mut libc::c_double = (*csa).beta;
    let mut d: *mut libc::c_double = (*csa).d;
    let mut se: *mut SPXSE = (*csa).se;
    let mut list: *mut libc::c_int = (*csa).list;
    let mut tcol: *mut libc::c_double = (*csa).work.vec;
    let mut tol_piv: libc::c_double = (*csa).tol_piv;
    let mut try_0: libc::c_int = 0;
    let mut nnn: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut p_flag: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut big: libc::c_double = 0.;
    let mut best_ratio: libc::c_double = 0.;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut bp: *mut SPXBP = (*csa).bp;
    let mut nbp: libc::c_int = 0;
    let mut t_best: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut dz_best: libc::c_double = 0.;
    ((*csa).beta_st != 0
        || {
            glp_assert_(
                b"csa->beta_st\0" as *const u8 as *const libc::c_char,
                b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                605 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*csa).d_st != 0
        || {
            glp_assert_(
                b"csa->d_st\0" as *const u8 as *const libc::c_char,
                b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                606 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    loop {
        nnn = (*csa).num;
        (*csa).q = 0 as libc::c_int;
        best_ratio = 0.0f64;
        ret = 0 as libc::c_int;
        try_0 = ret;
        loop {
            (nnn > 0 as libc::c_int
                || {
                    glp_assert_(
                        b"nnn > 0\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                        618 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            try_0 += 1;
            try_0;
            if se.is_null() {
                q = _glp_spx_chuzc_std(
                    lp,
                    d as *const libc::c_double,
                    nnn,
                    list as *const libc::c_int,
                );
            } else {
                q = _glp_spx_chuzc_pse(
                    lp,
                    se,
                    d as *const libc::c_double,
                    nnn,
                    list as *const libc::c_int,
                );
            }
            (1 as libc::c_int <= q && q <= n - m
                || {
                    glp_assert_(
                        b"1 <= q && q <= n-m\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                        628 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            _glp_spx_eval_tcol(lp, q, tcol);
            big = 1.0f64;
            if (*csa).phase == 1 as libc::c_int && (*csa).r_test == 0x33 as libc::c_int
                && try_0 <= 2 as libc::c_int
            {
                let mut t_0: libc::c_int = 0;
                let mut num: libc::c_int = 0;
                let mut num1: libc::c_int = 0;
                let mut slope: libc::c_double = 0.;
                let mut teta_lim: libc::c_double = 0.;
                nbp = _glp_spx_ls_eval_bp(
                    lp,
                    beta as *const libc::c_double,
                    q,
                    *d.offset(q as isize),
                    tcol as *const libc::c_double,
                    tol_piv,
                    bp,
                );
                if !(nbp < 2 as libc::c_int) {
                    slope = -fabs(*d.offset(q as isize));
                    teta_lim = 1.7976931348623157e+308f64;
                    t_0 = 1 as libc::c_int;
                    while t_0 <= nbp {
                        if teta_lim > (*bp.offset(t_0 as isize)).teta {
                            teta_lim = (*bp.offset(t_0 as isize)).teta;
                        }
                        t_0 += 1;
                        t_0;
                    }
                    (teta_lim >= 0.0f64
                        || {
                            glp_assert_(
                                b"teta_lim >= 0.0\0" as *const u8 as *const libc::c_char,
                                b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                                663 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    if teta_lim < 1e-3f64 {
                        teta_lim = 1e-3f64;
                    }
                    t_best = 0 as libc::c_int;
                    dz_best = 0.0f64;
                    num = 0 as libc::c_int;
                    while num < nbp {
                        num1 = _glp_spx_ls_select_bp(
                            lp,
                            tcol as *const libc::c_double,
                            nbp,
                            bp,
                            num,
                            &mut slope,
                            teta_lim,
                        );
                        t_0 = num + 1 as libc::c_int;
                        while t_0 <= num1 {
                            let mut i: libc::c_int = if (*bp.offset(t_0 as isize)).i
                                >= 0 as libc::c_int
                            {
                                (*bp.offset(t_0 as isize)).i
                            } else {
                                -(*bp.offset(t_0 as isize)).i
                            };
                            (0 as libc::c_int <= i && i <= m
                                || {
                                    glp_assert_(
                                        b"0 <= i && i <= m\0" as *const u8 as *const libc::c_char,
                                        b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                                        675 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                            if i == 0 as libc::c_int
                                || fabs(*tcol.offset(i as isize)) / big >= 0.0001f64
                            {
                                if dz_best > (*bp.offset(t_0 as isize)).dz {
                                    t_best = t_0;
                                    dz_best = (*bp.offset(t_0 as isize)).dz;
                                }
                            }
                            t_0 += 1;
                            t_0;
                        }
                        if slope > 0.0f64 {
                            break;
                        }
                        num = num1;
                        teta_lim += teta_lim;
                    }
                    if !(dz_best == 0.0f64) {
                        (1 as libc::c_int <= t_best && t_best <= num1
                            || {
                                glp_assert_(
                                    b"1 <= t_best && t_best <= num1\0" as *const u8
                                        as *const libc::c_char,
                                    b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                                    703 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        if !(t_best == 1 as libc::c_int) {
                            (*csa).q = q;
                            memcpy(
                                &mut *((*csa).tcol.vec).offset(1 as libc::c_int as isize)
                                    as *mut libc::c_double as *mut libc::c_void,
                                &mut *tcol.offset(1 as libc::c_int as isize)
                                    as *mut libc::c_double as *const libc::c_void,
                                (m as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                                    ),
                            );
                            _glp_fvs_gather_vec(
                                &mut (*csa).tcol,
                                2.2204460492503131e-16f64,
                            );
                            if (*bp.offset(t_best as isize)).i == 0 as libc::c_int {
                                (*csa).p = -(1 as libc::c_int);
                                (*csa).p_flag = 0 as libc::c_int;
                                best_ratio = 1.0f64;
                            } else if (*bp.offset(t_best as isize)).i > 0 as libc::c_int
                            {
                                (*csa).p = (*bp.offset(t_best as isize)).i;
                                (1 as libc::c_int <= (*csa).p && (*csa).p <= m
                                    || {
                                        glp_assert_(
                                            b"1 <= csa->p && csa->p <= m\0" as *const u8
                                                as *const libc::c_char,
                                            b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                                            721 as libc::c_int,
                                        );
                                        1 as libc::c_int != 0
                                    }) as libc::c_int;
                                (*csa).p_flag = 0 as libc::c_int;
                                best_ratio = fabs(*tcol.offset((*csa).p as isize)) / big;
                            } else {
                                (*csa).p = -(*bp.offset(t_best as isize)).i;
                                (1 as libc::c_int <= (*csa).p && (*csa).p <= m
                                    || {
                                        glp_assert_(
                                            b"1 <= csa->p && csa->p <= m\0" as *const u8
                                                as *const libc::c_char,
                                            b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                                            728 as libc::c_int,
                                        );
                                        1 as libc::c_int != 0
                                    }) as libc::c_int;
                                (*csa).p_flag = 1 as libc::c_int;
                                best_ratio = fabs(*tcol.offset((*csa).p as isize)) / big;
                            }
                            ret = 1 as libc::c_int;
                            break;
                        }
                    }
                }
            }
            if (*csa).r_test == 0x11 as libc::c_int {
                p = _glp_spx_chuzr_std(
                    lp,
                    (*csa).phase,
                    beta as *const libc::c_double,
                    q,
                    if *d.offset(q as isize) < 0.0f64 { 1.0f64 } else { -1.0f64 },
                    tcol as *const libc::c_double,
                    &mut p_flag,
                    tol_piv,
                    0.30f64 * (*csa).tol_bnd,
                    0.30f64 * (*csa).tol_bnd1,
                );
            } else {
                p = _glp_spx_chuzr_harris(
                    lp,
                    (*csa).phase,
                    beta as *const libc::c_double,
                    q,
                    if *d.offset(q as isize) < 0.0f64 { 1.0f64 } else { -1.0f64 },
                    tcol as *const libc::c_double,
                    &mut p_flag,
                    tol_piv,
                    0.50f64 * (*csa).tol_bnd,
                    0.50f64 * (*csa).tol_bnd1,
                );
            }
            if p <= 0 as libc::c_int {
                (*csa).q = q;
                memcpy(
                    &mut *((*csa).tcol.vec).offset(1 as libc::c_int as isize)
                        as *mut libc::c_double as *mut libc::c_void,
                    &mut *tcol.offset(1 as libc::c_int as isize) as *mut libc::c_double
                        as *const libc::c_void,
                    (m as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                        ),
                );
                _glp_fvs_gather_vec(&mut (*csa).tcol, 2.2204460492503131e-16f64);
                (*csa).p = p;
                (*csa).p_flag = p_flag;
                best_ratio = 1.0f64;
                break;
            } else {
                if best_ratio < fabs(*tcol.offset(p as isize)) / big {
                    (*csa).q = q;
                    memcpy(
                        &mut *((*csa).tcol.vec).offset(1 as libc::c_int as isize)
                            as *mut libc::c_double as *mut libc::c_void,
                        &mut *tcol.offset(1 as libc::c_int as isize)
                            as *mut libc::c_double as *const libc::c_void,
                        (m as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                            ),
                    );
                    _glp_fvs_gather_vec(&mut (*csa).tcol, 2.2204460492503131e-16f64);
                    (*csa).p = p;
                    (*csa).p_flag = p_flag;
                    best_ratio = fabs(*tcol.offset(p as isize)) / big;
                }
                if best_ratio >= 0.0001f64 || nnn == 1 as libc::c_int
                    || try_0 == 5 as libc::c_int
                {
                    break;
                }
                t = 1 as libc::c_int;
                while t <= nnn {
                    if *list.offset(t as isize) == q {
                        break;
                    }
                    t += 1;
                    t;
                }
                (t <= nnn
                    || {
                        glp_assert_(
                            b"t <= nnn\0" as *const u8 as *const libc::c_char,
                            b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                            792 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                *list.offset(t as isize) = *list.offset(nnn as isize);
                *list.offset(nnn as isize) = q;
                nnn -= 1;
                nnn;
            }
        }
        if !(best_ratio < 0.001f64 * 0.0001f64) {
            break;
        }
        if _glp_bfd_get_count((*lp).bfd) > 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        if !(tol_piv == (*csa).tol_piv) {
            break;
        }
        tol_piv *= 1000.0f64;
    }
    if ret != 0 {
        (*csa).d_st = 0 as libc::c_int;
        t = 1 as libc::c_int;
        while t < t_best {
            let mut i_0: libc::c_int = if (*bp.offset(t as isize)).i >= 0 as libc::c_int
            {
                (*bp.offset(t as isize)).i
            } else {
                -(*bp.offset(t as isize)).i
            };
            (0 as libc::c_int <= i_0 && i_0 <= m
                || {
                    glp_assert_(
                        b"0 <= i && i <= m\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                        822 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if i_0 == 0 as libc::c_int {
                (1 as libc::c_int <= (*csa).q && (*csa).q <= n - m
                    || {
                        glp_assert_(
                            b"1 <= csa->q && csa->q <= n-m\0" as *const u8
                                as *const libc::c_char,
                            b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                            825 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                k = *head.offset((m + (*csa).q) as isize);
            } else {
                k = *head.offset(i_0 as isize);
            }
            *c.offset(k as isize) += (*bp.offset(t as isize)).dc;
            (*c.offset(k as isize) == 0.0f64 || *c.offset(k as isize) == 1.0f64
                || *c.offset(k as isize) == -1.0f64
                || {
                    glp_assert_(
                        b"c[k] == 0.0 || c[k] == +1.0 || c[k] == -1.0\0" as *const u8
                            as *const libc::c_char,
                        b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                        833 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            t += 1;
            t;
        }
    }
    return ret;
}
unsafe extern "C" fn play_bounds(mut csa: *mut csa, mut all: libc::c_int) {
    let mut lp: *mut SPXLP = (*csa).lp;
    let mut m: libc::c_int = (*lp).m;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut orig_l: *mut libc::c_double = (*csa).orig_l;
    let mut orig_u: *mut libc::c_double = (*csa).orig_u;
    let mut beta: *mut libc::c_double = (*csa).beta;
    let mut tcol: *const libc::c_double = (*csa).tcol.vec;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    ((*csa).phase == 1 as libc::c_int || (*csa).phase == 2 as libc::c_int
        || {
            glp_assert_(
                b"csa->phase == 1 || csa->phase == 2\0" as *const u8
                    as *const libc::c_char,
                b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                878 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*csa).beta_st != 0
        || {
            glp_assert_(
                b"csa->beta_st\0" as *const u8 as *const libc::c_char,
                b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                880 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    let mut current_block_29: u64;
    i = 1 as libc::c_int;
    while i <= m {
        if all != 0 || *tcol.offset(i as isize) != 0.0f64 {
            k = *head.offset(i as isize);
            if (*csa).phase == 1 as libc::c_int && *c.offset(k as isize) < 0.0f64 {
                if *beta.offset(i as isize) < *l.offset(k as isize) - 1e-9f64 {
                    current_block_29 = 15619007995458559411;
                } else {
                    *c.offset(k as isize) = 0.0f64;
                    (*csa).d_st = 0 as libc::c_int;
                    current_block_29 = 7651349459974463963;
                }
            } else {
                current_block_29 = 7651349459974463963;
            }
            match current_block_29 {
                15619007995458559411 => {}
                _ => {
                    if (*csa).phase == 1 as libc::c_int && *c.offset(k as isize) > 0.0f64
                    {
                        if *beta.offset(i as isize) > *u.offset(k as isize) + 1e-9f64 {
                            current_block_29 = 15619007995458559411;
                        } else {
                            *c.offset(k as isize) = 0.0f64;
                            (*csa).d_st = 0 as libc::c_int;
                            current_block_29 = 17407779659766490442;
                        }
                    } else {
                        current_block_29 = 17407779659766490442;
                    }
                    match current_block_29 {
                        15619007995458559411 => {}
                        _ => {
                            if (*csa).phase == 1 as libc::c_int {
                                (*c.offset(k as isize) == 0.0f64
                                    || {
                                        glp_assert_(
                                            b"c[k] == 0.0\0" as *const u8 as *const libc::c_char,
                                            b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                                            904 as libc::c_int,
                                        );
                                        1 as libc::c_int != 0
                                    }) as libc::c_int;
                            }
                            if *l.offset(k as isize) != -1.7976931348623157e+308f64 {
                                if *beta.offset(i as isize) < *l.offset(k as isize) {
                                    *l.offset(k as isize) = *beta.offset(i as isize) - 1e-9f64;
                                } else if *l.offset(k as isize) < *orig_l.offset(k as isize)
                                {
                                    if *beta.offset(i as isize) >= *orig_l.offset(k as isize) {
                                        *l.offset(k as isize) = *orig_l.offset(k as isize);
                                    } else {
                                        *l.offset(k as isize) = *beta.offset(i as isize);
                                    }
                                }
                            }
                            if *u.offset(k as isize) != 1.7976931348623157e+308f64 {
                                if *beta.offset(i as isize) > *u.offset(k as isize) {
                                    *u.offset(k as isize) = *beta.offset(i as isize) + 1e-9f64;
                                } else if *u.offset(k as isize) > *orig_u.offset(k as isize)
                                {
                                    if *beta.offset(i as isize) <= *orig_u.offset(k as isize) {
                                        *u.offset(k as isize) = *orig_u.offset(k as isize);
                                    } else {
                                        *u.offset(k as isize) = *beta.offset(i as isize);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn remove_perturb(mut csa: *mut csa) {
    let mut lp: *mut SPXLP = (*csa).lp;
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut flag: *mut libc::c_char = (*lp).flag;
    let mut orig_l: *mut libc::c_double = (*csa).orig_l;
    let mut orig_u: *mut libc::c_double = (*csa).orig_u;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    memcpy(
        l as *mut libc::c_void,
        orig_l as *const libc::c_void,
        ((1 as libc::c_int + n) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memcpy(
        u as *mut libc::c_void,
        orig_u as *const libc::c_void,
        ((1 as libc::c_int + n) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    j = 1 as libc::c_int;
    while j <= n - m {
        k = *head.offset((m + j) as isize);
        if *l.offset(k as isize) == *u.offset(k as isize) {
            *flag.offset(j as isize) = 0 as libc::c_int as libc::c_char;
        }
        j += 1;
        j;
    }
    (*csa).beta_st = 0 as libc::c_int;
    (*csa).phase = (*csa).beta_st;
    if (*csa).msg_lev >= 3 as libc::c_int {
        glp_printf(
            b"Removing LP perturbation [%d]...\n\0" as *const u8 as *const libc::c_char,
            (*csa).it_cnt,
        );
    }
}
unsafe extern "C" fn sum_infeas(
    mut lp: *mut SPXLP,
    mut beta: *const libc::c_double,
) -> libc::c_double {
    let mut m: libc::c_int = (*lp).m;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut sum: libc::c_double = 0.0f64;
    i = 1 as libc::c_int;
    while i <= m {
        k = *head.offset(i as isize);
        if *l.offset(k as isize) != -1.7976931348623157e+308f64
            && *beta.offset(i as isize) < *l.offset(k as isize)
        {
            sum += *l.offset(k as isize) - *beta.offset(i as isize);
        }
        if *u.offset(k as isize) != 1.7976931348623157e+308f64
            && *beta.offset(i as isize) > *u.offset(k as isize)
        {
            sum += *beta.offset(i as isize) - *u.offset(k as isize);
        }
        i += 1;
        i;
    }
    return sum;
}
unsafe extern "C" fn display(mut csa: *mut csa, mut spec: libc::c_int) {
    let mut nnn: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut obj: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    let mut save: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut save1: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut tm_cur: libc::c_double = 0.;
    if !((*csa).msg_lev < 2 as libc::c_int) {
        tm_cur = glp_time();
        if !((*csa).out_dly > 0 as libc::c_int
            && 1000.0f64 * glp_difftime(tm_cur, (*csa).tm_beg)
                < (*csa).out_dly as libc::c_double)
        {
            if !((*csa).it_cnt == (*csa).it_dpy) {
                if !(spec == 0
                    && 1000.0f64 * glp_difftime(tm_cur, (*csa).tm_dpy)
                        < (*csa).out_frq as libc::c_double)
                {
                    save = (*(*csa).lp).c;
                    (*(*csa).lp).c = (*csa).orig_c;
                    obj = (*csa).dir as libc::c_double
                        * _glp_spx_eval_obj(
                            (*csa).lp,
                            (*csa).beta as *const libc::c_double,
                        );
                    (*(*csa).lp).c = save;
                    obj *= (*csa).fz;
                    save = (*(*csa).lp).l;
                    save1 = (*(*csa).lp).u;
                    (*(*csa).lp).l = (*csa).orig_l;
                    (*(*csa).lp).u = (*csa).orig_u;
                    sum = sum_infeas((*csa).lp, (*csa).beta as *const libc::c_double);
                    (*(*csa).lp).l = save;
                    (*(*csa).lp).u = save1;
                    match (*csa).phase {
                        1 => {
                            nnn = 0 as libc::c_int;
                            k = 1 as libc::c_int;
                            while k <= (*(*csa).lp).n {
                                if *((*(*csa).lp).c).offset(k as isize) != 0.0f64 {
                                    nnn += 1;
                                    nnn;
                                }
                                k += 1;
                                k;
                            }
                        }
                        2 => {
                            ((*csa).d_st != 0
                                || {
                                    glp_assert_(
                                        b"csa->d_st\0" as *const u8 as *const libc::c_char,
                                        b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                                        1074 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                            nnn = _glp_spx_chuzc_sel(
                                (*csa).lp,
                                (*csa).d as *const libc::c_double,
                                (*csa).tol_dj,
                                (*csa).tol_dj1,
                                0 as *mut libc::c_int,
                            );
                        }
                        _ => {
                            (csa != csa
                                || {
                                    glp_assert_(
                                        b"csa != csa\0" as *const u8 as *const libc::c_char,
                                        b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                                        1079 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                        }
                    }
                    glp_printf(
                        b"%c%6d: obj = %17.9e inf = %11.3e (%d)\0" as *const u8
                            as *const libc::c_char,
                        if (*csa).phase == 2 as libc::c_int {
                            '*' as i32
                        } else {
                            ' ' as i32
                        },
                        (*csa).it_cnt,
                        obj,
                        sum,
                        nnn,
                    );
                    if (*csa).inv_cnt != 0 {
                        glp_printf(
                            b" %d\0" as *const u8 as *const libc::c_char,
                            (*csa).inv_cnt,
                        );
                        (*csa).inv_cnt = 0 as libc::c_int;
                    }
                    if (*csa).phase == 1 as libc::c_int
                        && (*csa).r_test == 0x33 as libc::c_int
                    {
                        if (*csa).ns_cnt + (*csa).ls_cnt != 0 {
                            glp_printf(
                                b" %d%%\0" as *const u8 as *const libc::c_char,
                                100 as libc::c_int * (*csa).ls_cnt
                                    / ((*csa).ns_cnt + (*csa).ls_cnt),
                            );
                        }
                        (*csa).ls_cnt = 0 as libc::c_int;
                        (*csa).ns_cnt = (*csa).ls_cnt;
                    }
                    glp_printf(b"\n\0" as *const u8 as *const libc::c_char);
                    (*csa).it_dpy = (*csa).it_cnt;
                    (*csa).tm_dpy = tm_cur;
                }
            }
        }
    }
}
unsafe extern "C" fn primal_simplex(mut csa: *mut csa) -> libc::c_int {
    let mut current_block: u64;
    let mut lp: *mut SPXLP = (*csa).lp;
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut at: *mut SPXAT = (*csa).at;
    let mut nt: *mut SPXNT = (*csa).nt;
    let mut beta: *mut libc::c_double = (*csa).beta;
    let mut d: *mut libc::c_double = (*csa).d;
    let mut se: *mut SPXSE = (*csa).se;
    let mut list: *mut libc::c_int = (*csa).list;
    let mut pi: *mut libc::c_double = (*csa).work.vec;
    let mut rho: *mut libc::c_double = (*csa).work.vec;
    let mut msg_lev: libc::c_int = (*csa).msg_lev;
    let mut tol_bnd: libc::c_double = (*csa).tol_bnd;
    let mut tol_bnd1: libc::c_double = (*csa).tol_bnd1;
    let mut tol_dj: libc::c_double = (*csa).tol_dj;
    let mut tol_dj1: libc::c_double = (*csa).tol_dj1;
    let mut perturb: libc::c_int = -(1 as libc::c_int);
    let mut j: libc::c_int = 0;
    let mut refct: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    loop {
        if (*lp).valid == 0 {
            let mut cond: libc::c_double = 0.;
            ret = _glp_spx_factorize(lp);
            (*csa).inv_cnt += 1;
            (*csa).inv_cnt;
            if ret != 0 as libc::c_int {
                if msg_lev >= 1 as libc::c_int {
                    glp_printf(
                        b"Error: unable to factorize the basis matrix (%d)\n\0"
                            as *const u8 as *const libc::c_char,
                        ret,
                    );
                }
                (*csa).d_stat = 1 as libc::c_int;
                (*csa).p_stat = (*csa).d_stat;
                ret = 0x5 as libc::c_int;
                break;
            } else {
                cond = _glp_bfd_condest((*lp).bfd);
                if cond > 1.0f64 / 2.2204460492503131e-16f64 {
                    if msg_lev >= 1 as libc::c_int {
                        glp_printf(
                            b"Error: basis matrix is singular to working precision (cond = %.3g)\n\0"
                                as *const u8 as *const libc::c_char,
                            cond,
                        );
                    }
                    (*csa).d_stat = 1 as libc::c_int;
                    (*csa).p_stat = (*csa).d_stat;
                    ret = 0x5 as libc::c_int;
                    break;
                } else {
                    if cond > 0.001f64 / 2.2204460492503131e-16f64 {
                        if msg_lev >= 1 as libc::c_int {
                            glp_printf(
                                b"Warning: basis matrix is ill-conditioned (cond = %.3g)\n\0"
                                    as *const u8 as *const libc::c_char,
                                cond,
                            );
                        }
                    }
                    (*csa).d_st = 0 as libc::c_int;
                    (*csa).beta_st = (*csa).d_st;
                }
            }
        }
        if (*csa).beta_st == 0 {
            _glp_spx_eval_beta(lp, beta);
            (*csa).beta_st = 1 as libc::c_int;
            if (*csa).phase == 0 {
                if set_penalty(csa, 0.97f64 * tol_bnd, 0.97f64 * tol_bnd1) != 0 {
                    (*csa).phase = 1 as libc::c_int;
                } else {
                    (*csa).phase = 2 as libc::c_int;
                    memcpy(
                        c as *mut libc::c_void,
                        (*csa).orig_c as *const libc::c_void,
                        ((1 as libc::c_int + n) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                            ),
                    );
                }
                (*csa).d_st = 0 as libc::c_int;
            }
            if perturb <= 0 as libc::c_int {
                if check_feas(csa, (*csa).phase, tol_bnd, tol_bnd1) != 0 {
                    if perturb < 0 as libc::c_int {
                        if msg_lev >= 3 as libc::c_int {
                            glp_printf(
                                b"Perturbing LP to avoid instability [%d]...\n\0"
                                    as *const u8 as *const libc::c_char,
                                (*csa).it_cnt,
                            );
                        }
                        perturb = 1 as libc::c_int;
                        continue;
                    } else {
                        if msg_lev >= 1 as libc::c_int {
                            glp_printf(
                                b"Warning: numerical instability (primal simplex, phase %s)\n\0"
                                    as *const u8 as *const libc::c_char,
                                if (*csa).phase == 1 as libc::c_int {
                                    b"I\0" as *const u8 as *const libc::c_char
                                } else {
                                    b"II\0" as *const u8 as *const libc::c_char
                                },
                            );
                        }
                        (*lp).valid = 0 as libc::c_int;
                        (*csa).phase = 0 as libc::c_int;
                        continue;
                    }
                } else if (*csa).phase == 1 as libc::c_int {
                    let mut i: libc::c_int = 0;
                    let mut cnt: libc::c_int = 0;
                    i = 1 as libc::c_int;
                    while i <= m {
                        *((*csa).tcol.ind).offset(i as isize) = i;
                        i += 1;
                        i;
                    }
                    cnt = adjust_penalty(
                        csa,
                        m,
                        (*csa).tcol.ind as *const libc::c_int,
                        0.99f64 * tol_bnd,
                        0.99f64 * tol_bnd1,
                    );
                    if cnt != 0 {
                        (*csa).d_st = 0 as libc::c_int;
                    }
                }
            } else {
                play_bounds(csa, 1 as libc::c_int);
            }
        }
        ((*csa).phase == 1 as libc::c_int || (*csa).phase == 2 as libc::c_int
            || {
                glp_assert_(
                    b"csa->phase == 1 || csa->phase == 2\0" as *const u8
                        as *const libc::c_char,
                    b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                    1251 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if (*csa).d_st == 0 {
            _glp_spx_eval_pi(lp, pi);
            j = 1 as libc::c_int;
            while j <= n - m {
                *d
                    .offset(
                        j as isize,
                    ) = _glp_spx_eval_dj(lp, pi as *const libc::c_double, j);
                j += 1;
                j;
            }
            (*csa).d_st = 1 as libc::c_int;
        }
        if !se.is_null() && (*se).valid == 0 {
            _glp_spx_reset_refsp(lp, se);
            refct = 1000 as libc::c_int;
        }
        ((*lp).valid != 0 && (*csa).beta_st != 0 && (*csa).d_st != 0
            || {
                glp_assert_(
                    b"lp->valid && csa->beta_st && csa->d_st\0" as *const u8
                        as *const libc::c_char,
                    b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                    1264 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if (*csa).it_cnt - (*csa).it_beg >= (*csa).it_lim {
            if perturb > 0 as libc::c_int {
                remove_perturb(csa);
                perturb = 0 as libc::c_int;
            }
            if (*csa).beta_st != 1 as libc::c_int {
                (*csa).beta_st = 0 as libc::c_int;
            }
            if (*csa).d_st != 1 as libc::c_int {
                (*csa).d_st = 0 as libc::c_int;
            }
            if !((*csa).beta_st != 0 && (*csa).d_st != 0) {
                continue;
            }
            display(csa, 1 as libc::c_int);
            if msg_lev >= 3 as libc::c_int {
                glp_printf(
                    b"ITERATION LIMIT EXCEEDED; SEARCH TERMINATED\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            (*csa)
                .p_stat = if (*csa).phase == 2 as libc::c_int {
                2 as libc::c_int
            } else {
                3 as libc::c_int
            };
            (*csa).d_stat = 1 as libc::c_int;
            ret = 0x8 as libc::c_int;
            break;
        } else if 1000.0f64 * glp_difftime(glp_time(), (*csa).tm_beg)
            >= (*csa).tm_lim as libc::c_double
        {
            if perturb > 0 as libc::c_int {
                remove_perturb(csa);
                perturb = 0 as libc::c_int;
            }
            if (*csa).beta_st != 1 as libc::c_int {
                (*csa).beta_st = 0 as libc::c_int;
            }
            if (*csa).d_st != 1 as libc::c_int {
                (*csa).d_st = 0 as libc::c_int;
            }
            if !((*csa).beta_st != 0 && (*csa).d_st != 0) {
                continue;
            }
            display(csa, 1 as libc::c_int);
            if msg_lev >= 3 as libc::c_int {
                glp_printf(
                    b"TIME LIMIT EXCEEDED; SEARCH TERMINATED\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            (*csa)
                .p_stat = if (*csa).phase == 2 as libc::c_int {
                2 as libc::c_int
            } else {
                3 as libc::c_int
            };
            (*csa).d_stat = 1 as libc::c_int;
            ret = 0x9 as libc::c_int;
            break;
        } else {
            display(csa, 0 as libc::c_int);
            match (*csa).phase {
                1 => {
                    (*csa)
                        .num = _glp_spx_chuzc_sel(
                        lp,
                        d as *const libc::c_double,
                        1e-8f64,
                        0.0f64,
                        list,
                    );
                }
                2 => {
                    (*csa)
                        .num = _glp_spx_chuzc_sel(
                        lp,
                        d as *const libc::c_double,
                        tol_dj,
                        tol_dj1,
                        list,
                    );
                }
                _ => {
                    (csa != csa
                        || {
                            glp_assert_(
                                b"csa != csa\0" as *const u8 as *const libc::c_char,
                                b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                                1323 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
            }
            if (*csa).num == 0 as libc::c_int {
                if perturb > 0 as libc::c_int && (*csa).phase == 2 as libc::c_int {
                    remove_perturb(csa);
                    perturb = 0 as libc::c_int;
                }
                if (*csa).beta_st != 1 as libc::c_int {
                    (*csa).beta_st = 0 as libc::c_int;
                }
                if (*csa).d_st != 1 as libc::c_int {
                    (*csa).d_st = 0 as libc::c_int;
                }
                if !((*csa).beta_st != 0 && (*csa).d_st != 0) {
                    continue;
                }
                display(csa, 1 as libc::c_int);
                match (*csa).phase {
                    1 => {
                        if check_feas(csa, 2 as libc::c_int, tol_bnd, tol_bnd1) == 0 {
                            memcpy(
                                c as *mut libc::c_void,
                                (*csa).orig_c as *const libc::c_void,
                                ((1 as libc::c_int + n) as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                                    ),
                            );
                            (*csa).phase = 2 as libc::c_int;
                            (*csa).d_st = 0 as libc::c_int;
                            continue;
                        } else {
                            if msg_lev >= 3 as libc::c_int {
                                glp_printf(
                                    b"LP HAS NO PRIMAL FEASIBLE SOLUTION\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            (*csa).p_stat = 4 as libc::c_int;
                            (*csa).d_stat = 1 as libc::c_int;
                            ret = 0 as libc::c_int;
                            break;
                        }
                    }
                    2 => {
                        if msg_lev >= 3 as libc::c_int {
                            glp_printf(
                                b"OPTIMAL LP SOLUTION FOUND\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        (*csa).d_stat = 2 as libc::c_int;
                        (*csa).p_stat = (*csa).d_stat;
                        ret = 0 as libc::c_int;
                        break;
                    }
                    _ => {
                        (csa != csa
                            || {
                                glp_assert_(
                                    b"csa != csa\0" as *const u8 as *const libc::c_char,
                                    b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                                    1368 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                    }
                }
            }
            ret = choose_pivot(csa);
            if ret < 0 as libc::c_int {
                (*lp).valid = 0 as libc::c_int;
            } else {
                if ret == 0 as libc::c_int {
                    (*csa).ns_cnt += 1;
                    (*csa).ns_cnt;
                } else {
                    (*csa).ls_cnt += 1;
                    (*csa).ls_cnt;
                }
                if (*csa).p == 0 as libc::c_int {
                    if perturb > 0 as libc::c_int {
                        remove_perturb(csa);
                        perturb = 0 as libc::c_int;
                    }
                    if (*csa).beta_st != 1 as libc::c_int {
                        (*csa).beta_st = 0 as libc::c_int;
                    }
                    if (*csa).d_st != 1 as libc::c_int {
                        (*csa).d_st = 0 as libc::c_int;
                    }
                    if !((*csa).beta_st != 0 && (*csa).d_st != 0) {
                        continue;
                    }
                    display(csa, 1 as libc::c_int);
                    match (*csa).phase {
                        1 => {
                            if msg_lev >= 1 as libc::c_int {
                                glp_printf(
                                    b"Error: primal simplex failed\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            (*csa).d_stat = 1 as libc::c_int;
                            (*csa).p_stat = (*csa).d_stat;
                            ret = 0x5 as libc::c_int;
                            break;
                        }
                        2 => {
                            if msg_lev >= 3 as libc::c_int {
                                glp_printf(
                                    b"LP HAS UNBOUNDED PRIMAL SOLUTION\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            (*csa).p_stat = 2 as libc::c_int;
                            (*csa).d_stat = 4 as libc::c_int;
                            ret = 0 as libc::c_int;
                            break;
                        }
                        _ => {
                            (csa != csa
                                || {
                                    glp_assert_(
                                        b"csa != csa\0" as *const u8 as *const libc::c_char,
                                        b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                                        1423 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                        }
                    }
                }
                if (*csa).p > 0 as libc::c_int {
                    let mut k: libc::c_int = 0;
                    (1 as libc::c_int <= (*csa).p && (*csa).p <= m
                        || {
                            glp_assert_(
                                b"1 <= csa->p && csa->p <= m\0" as *const u8
                                    as *const libc::c_char,
                                b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                                1430 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    k = *head.offset((*csa).p as isize);
                    if *((*lp).l).offset(k as isize) != *((*lp).u).offset(k as isize) {
                        if (*csa).p_flag != 0 {
                            (*((*lp).u).offset(k as isize) != 1.7976931348623157e+308f64
                                || {
                                    glp_assert_(
                                        b"lp->u[k] != +DBL_MAX\0" as *const u8
                                            as *const libc::c_char,
                                        b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                                        1435 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                            if fabs(
                                *beta.offset((*csa).p as isize)
                                    - *((*lp).u).offset(k as isize),
                            ) >= 1e-6f64
                            {
                                (*csa).degen = 0 as libc::c_int;
                                current_block = 1541960655964344436;
                            } else {
                                current_block = 4367251730605750521;
                            }
                        } else if *((*lp).l).offset(k as isize)
                            == -1.7976931348623157e+308f64
                        {
                            current_block = 1541960655964344436;
                        } else {
                            (*((*lp).l).offset(k as isize) != -1.7976931348623157e+308f64
                                || {
                                    glp_assert_(
                                        b"lp->l[k] != -DBL_MAX\0" as *const u8
                                            as *const libc::c_char,
                                        b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                                        1447 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                            if fabs(
                                *beta.offset((*csa).p as isize)
                                    - *((*lp).l).offset(k as isize),
                            ) >= 1e-6f64
                            {
                                (*csa).degen = 0 as libc::c_int;
                                current_block = 1541960655964344436;
                            } else {
                                current_block = 4367251730605750521;
                            }
                        }
                        match current_block {
                            1541960655964344436 => {}
                            _ => {
                                (*csa).degen += 1;
                                (*csa).degen;
                                if perturb < 0 as libc::c_int
                                    && (*csa).degen >= 200 as libc::c_int
                                {
                                    if msg_lev >= 3 as libc::c_int {
                                        glp_printf(
                                            b"Perturbing LP to avoid stalling [%d]...\n\0" as *const u8
                                                as *const libc::c_char,
                                            (*csa).it_cnt,
                                        );
                                    }
                                    perturb = 1 as libc::c_int;
                                }
                            }
                        }
                    }
                }
                _glp_spx_update_beta_s(
                    lp,
                    beta,
                    (*csa).p,
                    (*csa).p_flag,
                    (*csa).q,
                    &mut (*csa).tcol,
                );
                (*csa).beta_st = 2 as libc::c_int;
                if !((*csa).p < 0 as libc::c_int) {
                    _glp_spx_eval_rho(lp, (*csa).p, rho);
                    if !at.is_null() {
                        _glp_spx_eval_trow1(
                            lp,
                            at,
                            rho as *const libc::c_double,
                            (*csa).trow.vec,
                        );
                    } else {
                        _glp_spx_nt_prod(
                            lp,
                            nt,
                            (*csa).trow.vec,
                            1 as libc::c_int,
                            -1.0f64,
                            rho as *const libc::c_double,
                        );
                    }
                    _glp_fvs_gather_vec(&mut (*csa).trow, 2.2204460492503131e-16f64);
                    if *((*csa).trow.vec).offset((*csa).q as isize) == 0.0f64 {
                        if msg_lev >= 1 as libc::c_int {
                            glp_printf(
                                b"Error: trow[q] = 0.0\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        (*csa).d_stat = 1 as libc::c_int;
                        (*csa).p_stat = (*csa).d_stat;
                        ret = 0x5 as libc::c_int;
                        break;
                    } else {
                        if (*csa).d_st != 0 {
                            if _glp_spx_update_d_s(
                                lp,
                                d,
                                (*csa).p,
                                (*csa).q,
                                &mut (*csa).trow,
                                &mut (*csa).tcol,
                            ) <= 1e-9f64
                            {
                                (*csa).d_st = 2 as libc::c_int;
                                if (*csa).phase == 1 as libc::c_int {
                                    *d.offset((*csa).q as isize)
                                        -= *c.offset(*head.offset((*csa).p as isize) as isize);
                                }
                            } else {
                                (*csa).d_st = 0 as libc::c_int;
                            }
                        }
                        if (*csa).phase == 1 as libc::c_int {
                            *c.offset(*head.offset((*csa).p as isize) as isize) = 0.0f64;
                        }
                        if !se.is_null() {
                            if refct > 0 as libc::c_int {
                                if _glp_spx_update_gamma(
                                    lp,
                                    se,
                                    (*csa).p,
                                    (*csa).q,
                                    (*csa).trow.vec as *const libc::c_double,
                                    (*csa).tcol.vec as *const libc::c_double,
                                ) <= 1e-3f64
                                {
                                    refct -= 1;
                                    refct;
                                } else {
                                    (*se).valid = 0 as libc::c_int;
                                }
                            } else {
                                (*se).valid = 0 as libc::c_int;
                            }
                        }
                        if !nt.is_null() {
                            _glp_spx_update_nt(lp, nt, (*csa).p, (*csa).q);
                        }
                    }
                }
                _glp_spx_change_basis(lp, (*csa).p, (*csa).p_flag, (*csa).q);
                if (*csa).p > 0 as libc::c_int {
                    _glp_spx_update_invb(lp, (*csa).p, *head.offset((*csa).p as isize));
                }
                if perturb <= 0 as libc::c_int {
                    if (*csa).phase == 1 as libc::c_int {
                        let mut cnt_0: libc::c_int = 0;
                        cnt_0 = adjust_penalty(
                            csa,
                            (*csa).tcol.nnz,
                            (*csa).tcol.ind as *const libc::c_int,
                            0.99f64 * tol_bnd,
                            0.99f64 * tol_bnd1,
                        );
                        if cnt_0 != 0 {
                            (*csa).d_st = 0 as libc::c_int;
                        }
                    }
                } else {
                    play_bounds(csa, 0 as libc::c_int);
                }
                (*csa).it_cnt += 1;
                (*csa).it_cnt;
            }
        }
    }
    memcpy(
        c as *mut libc::c_void,
        (*csa).orig_c as *const libc::c_void,
        ((1 as libc::c_int + n) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    if (*csa).p_stat != 1 as libc::c_int && (*csa).d_stat == 1 as libc::c_int {
        (ret != 0x5 as libc::c_int
            || {
                glp_assert_(
                    b"ret != GLP_EFAIL\0" as *const u8 as *const libc::c_char,
                    b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                    1592 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        _glp_spx_eval_pi(lp, pi);
        j = 1 as libc::c_int;
        while j <= n - m {
            *d.offset(j as isize) = _glp_spx_eval_dj(lp, pi as *const libc::c_double, j);
            j += 1;
            j;
        }
        (*csa)
            .num = _glp_spx_chuzc_sel(
            lp,
            d as *const libc::c_double,
            tol_dj,
            tol_dj1,
            0 as *mut libc::c_int,
        );
        (*csa)
            .d_stat = if (*csa).num == 0 as libc::c_int {
            2 as libc::c_int
        } else {
            3 as libc::c_int
        };
    }
    return ret;
}
pub unsafe extern "C" fn _glp_spx_primal(
    mut P: *mut glp_prob,
    mut parm: *const glp_smcp,
) -> libc::c_int {
    let mut csa_: csa = csa {
        lp: 0 as *mut SPXLP,
        dir: 0,
        fz: 0.,
        orig_c: 0 as *mut libc::c_double,
        orig_l: 0 as *mut libc::c_double,
        orig_u: 0 as *mut libc::c_double,
        at: 0 as *mut SPXAT,
        nt: 0 as *mut SPXNT,
        phase: 0,
        beta: 0 as *mut libc::c_double,
        beta_st: 0,
        d: 0 as *mut libc::c_double,
        d_st: 0,
        se: 0 as *mut SPXSE,
        num: 0,
        list: 0 as *mut libc::c_int,
        q: 0,
        tcol: FVS {
            n: 0,
            nnz: 0,
            ind: 0 as *mut libc::c_int,
            vec: 0 as *mut libc::c_double,
        },
        bp: 0 as *mut SPXBP,
        p: 0,
        p_flag: 0,
        trow: FVS {
            n: 0,
            nnz: 0,
            ind: 0 as *mut libc::c_int,
            vec: 0 as *mut libc::c_double,
        },
        work: FVS {
            n: 0,
            nnz: 0,
            ind: 0 as *mut libc::c_int,
            vec: 0 as *mut libc::c_double,
        },
        p_stat: 0,
        d_stat: 0,
        msg_lev: 0,
        r_test: 0,
        tol_bnd: 0.,
        tol_bnd1: 0.,
        tol_dj: 0.,
        tol_dj1: 0.,
        tol_piv: 0.,
        it_lim: 0,
        tm_lim: 0,
        out_frq: 0,
        out_dly: 0,
        tm_beg: 0.,
        it_beg: 0,
        it_cnt: 0,
        it_dpy: 0,
        tm_dpy: 0.,
        inv_cnt: 0,
        degen: 0,
        ns_cnt: 0,
        ls_cnt: 0,
    };
    let mut csa: *mut csa = &mut csa_;
    let mut lp: SPXLP = SPXLP {
        m: 0,
        n: 0,
        nnz: 0,
        A_ptr: 0 as *mut libc::c_int,
        A_ind: 0 as *mut libc::c_int,
        A_val: 0 as *mut libc::c_double,
        b: 0 as *mut libc::c_double,
        c: 0 as *mut libc::c_double,
        l: 0 as *mut libc::c_double,
        u: 0 as *mut libc::c_double,
        head: 0 as *mut libc::c_int,
        flag: 0 as *mut libc::c_char,
        valid: 0,
        bfd: 0 as *mut BFD,
    };
    let mut at: SPXAT = SPXAT {
        ptr: 0 as *mut libc::c_int,
        ind: 0 as *mut libc::c_int,
        val: 0 as *mut libc::c_double,
        work: 0 as *mut libc::c_double,
    };
    let mut nt: SPXNT = SPXNT {
        ptr: 0 as *mut libc::c_int,
        len: 0 as *mut libc::c_int,
        ind: 0 as *mut libc::c_int,
        val: 0 as *mut libc::c_double,
    };
    let mut se: SPXSE = SPXSE {
        valid: 0,
        refsp: 0 as *mut libc::c_char,
        gamma: 0 as *mut libc::c_double,
        work: 0 as *mut libc::c_double,
    };
    let mut ret: libc::c_int = 0;
    let mut map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut daeh: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    memset(
        csa as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<csa>() as libc::c_ulong,
    );
    (*csa).lp = &mut lp;
    _glp_spx_init_lp((*csa).lp, P, (*parm).excl);
    _glp_spx_alloc_lp((*csa).lp);
    map = glp_alloc(
        1 as libc::c_int + (*P).m + (*P).n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    _glp_spx_build_lp((*csa).lp, P, (*parm).excl, (*parm).shift, map);
    _glp_spx_build_basis((*csa).lp, P, map as *const libc::c_int);
    match (*P).dir {
        1 => {
            (*csa).dir = 1 as libc::c_int;
        }
        2 => {
            (*csa).dir = -(1 as libc::c_int);
        }
        _ => {
            (P != P
                || {
                    glp_assert_(
                        b"P != P\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                        1629 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    (*csa).fz = 0.0f64;
    k = 1 as libc::c_int;
    while k <= (*(*csa).lp).n {
        let mut t: libc::c_double = fabs(*((*(*csa).lp).c).offset(k as isize));
        if (*csa).fz < t {
            (*csa).fz = t;
        }
        k += 1;
        k;
    }
    if (*csa).fz <= 1000.0f64 {
        (*csa).fz = 1.0f64;
    } else {
        (*csa).fz /= 1000.0f64;
    }
    k = 0 as libc::c_int;
    while k <= (*(*csa).lp).n {
        *((*(*csa).lp).c).offset(k as isize) /= (*csa).fz;
        k += 1;
        k;
    }
    (*csa)
        .orig_c = glp_alloc(
        1 as libc::c_int + (*(*csa).lp).n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    memcpy(
        (*csa).orig_c as *mut libc::c_void,
        (*(*csa).lp).c as *const libc::c_void,
        ((1 as libc::c_int + (*(*csa).lp).n) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    (*csa)
        .orig_l = glp_alloc(
        1 as libc::c_int + (*(*csa).lp).n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    memcpy(
        (*csa).orig_l as *mut libc::c_void,
        (*(*csa).lp).l as *const libc::c_void,
        ((1 as libc::c_int + (*(*csa).lp).n) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    (*csa)
        .orig_u = glp_alloc(
        1 as libc::c_int + (*(*csa).lp).n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    memcpy(
        (*csa).orig_u as *mut libc::c_void,
        (*(*csa).lp).u as *const libc::c_void,
        ((1 as libc::c_int + (*(*csa).lp).n) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    match (*parm).aorn {
        1 => {
            (*csa).at = &mut at;
            (*csa).nt = 0 as *mut SPXNT;
            _glp_spx_alloc_at((*csa).lp, (*csa).at);
            _glp_spx_build_at((*csa).lp, (*csa).at);
        }
        2 => {
            (*csa).at = 0 as *mut SPXAT;
            (*csa).nt = &mut nt;
            _glp_spx_alloc_nt((*csa).lp, (*csa).nt);
            _glp_spx_init_nt((*csa).lp, (*csa).nt);
            _glp_spx_build_nt((*csa).lp, (*csa).nt);
        }
        _ => {
            (parm != parm
                || {
                    glp_assert_(
                        b"parm != parm\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                        1673 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    (*csa).phase = 0 as libc::c_int;
    (*csa)
        .beta = glp_alloc(
        1 as libc::c_int + (*(*csa).lp).m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*csa).beta_st = 0 as libc::c_int;
    (*csa)
        .d = glp_alloc(
        1 as libc::c_int + (*(*csa).lp).n - (*(*csa).lp).m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*csa).d_st = 0 as libc::c_int;
    match (*parm).pricing {
        17 => {
            (*csa).se = 0 as *mut SPXSE;
        }
        34 => {
            (*csa).se = &mut se;
            _glp_spx_alloc_se((*csa).lp, (*csa).se);
        }
        _ => {
            (parm != parm
                || {
                    glp_assert_(
                        b"parm != parm\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                        1690 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    (*csa)
        .list = glp_alloc(
        1 as libc::c_int + (*(*csa).lp).n - (*(*csa).lp).m,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    _glp_fvs_alloc_vec(&mut (*csa).tcol, (*(*csa).lp).m);
    _glp_fvs_alloc_vec(&mut (*csa).trow, (*(*csa).lp).n - (*(*csa).lp).m);
    (*csa).bp = 0 as *mut SPXBP;
    _glp_fvs_alloc_vec(&mut (*csa).work, (*(*csa).lp).m);
    (*csa).msg_lev = (*parm).msg_lev;
    match (*parm).r_test {
        17 | 34 => {}
        51 => {
            (*csa)
                .bp = glp_alloc(
                1 as libc::c_int + 2 as libc::c_int * (*(*csa).lp).m + 1 as libc::c_int,
                ::std::mem::size_of::<SPXBP>() as libc::c_ulong as libc::c_int,
            ) as *mut SPXBP;
        }
        _ => {
            (parm != parm
                || {
                    glp_assert_(
                        b"parm != parm\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                        1736 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    (*csa).r_test = (*parm).r_test;
    (*csa).tol_bnd = (*parm).tol_bnd;
    (*csa).tol_bnd1 = 0.001f64 * (*parm).tol_bnd;
    (*csa).tol_dj = (*parm).tol_dj;
    (*csa).tol_dj1 = 0.001f64 * (*parm).tol_dj;
    (*csa).tol_piv = (*parm).tol_piv;
    (*csa).it_lim = (*parm).it_lim;
    (*csa).tm_lim = (*parm).tm_lim;
    (*csa).out_frq = (*parm).out_frq;
    (*csa).out_dly = (*parm).out_dly;
    (*csa).tm_beg = glp_time();
    (*csa).it_cnt = (*P).it_cnt;
    (*csa).it_beg = (*csa).it_cnt;
    (*csa).it_dpy = -(1 as libc::c_int);
    (*csa).tm_dpy = 0.0f64;
    (*csa).inv_cnt = 0 as libc::c_int;
    (*csa).degen = 0 as libc::c_int;
    (*csa).ls_cnt = 0 as libc::c_int;
    (*csa).ns_cnt = (*csa).ls_cnt;
    ret = primal_simplex(csa);
    (*P).valid = (*(*csa).lp).valid;
    (*P).bfd = (*(*csa).lp).bfd;
    (*P).pbs_stat = (*csa).p_stat;
    (*P).dbs_stat = (*csa).d_stat;
    if !(ret == 0x5 as libc::c_int) {
        daeh = glp_alloc(
            1 as libc::c_int + (*(*csa).lp).n,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        _glp_spx_store_basis((*csa).lp, P, map as *const libc::c_int, daeh);
        _glp_spx_eval_pi((*csa).lp, (*csa).work.vec);
        i = 1 as libc::c_int;
        while i <= (*(*csa).lp).m {
            *((*csa).work.vec).offset(i as isize) *= (*csa).fz;
            i += 1;
            i;
        }
        j = 1 as libc::c_int;
        while j <= (*(*csa).lp).n - (*(*csa).lp).m {
            *((*csa).d).offset(j as isize) *= (*csa).fz;
            j += 1;
            j;
        }
        _glp_spx_store_sol(
            (*csa).lp,
            P,
            (*parm).shift,
            map as *const libc::c_int,
            daeh as *const libc::c_int,
            (*csa).beta as *const libc::c_double,
            (*csa).work.vec as *const libc::c_double,
            (*csa).d as *const libc::c_double,
        );
        glp_free(daeh as *mut libc::c_void);
        (*P).it_cnt = (*csa).it_cnt;
        (*P).some = 0 as libc::c_int;
        if (*csa).p_stat == 2 as libc::c_int && (*csa).d_stat == 4 as libc::c_int {
            let mut k_0: libc::c_int = 0;
            let mut kk: libc::c_int = 0;
            (1 as libc::c_int <= (*csa).q && (*csa).q <= (*(*csa).lp).n - (*(*csa).lp).m
                || {
                    glp_assert_(
                        b"1 <= csa->q && csa->q <= csa->lp->n - csa->lp->m\0"
                            as *const u8 as *const libc::c_char,
                        b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                        1809 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            k_0 = *((*(*csa).lp).head).offset(((*(*csa).lp).m + (*csa).q) as isize);
            (1 as libc::c_int <= k_0 && k_0 <= (*(*csa).lp).n
                || {
                    glp_assert_(
                        b"1 <= k && k <= csa->lp->n\0" as *const u8
                            as *const libc::c_char,
                        b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                        1811 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            kk = 1 as libc::c_int;
            while kk <= (*P).m + (*P).n {
                if abs(*map.offset(kk as isize)) == k_0 {
                    (*P).some = kk;
                    break;
                } else {
                    kk += 1;
                    kk;
                }
            }
            ((*P).some != 0 as libc::c_int
                || {
                    glp_assert_(
                        b"P->some != 0\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxprim.c\0" as *const u8 as *const libc::c_char,
                        1819 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    _glp_spx_free_lp((*csa).lp);
    glp_free(map as *mut libc::c_void);
    glp_free((*csa).orig_c as *mut libc::c_void);
    glp_free((*csa).orig_l as *mut libc::c_void);
    glp_free((*csa).orig_u as *mut libc::c_void);
    if !((*csa).at).is_null() {
        _glp_spx_free_at((*csa).lp, (*csa).at);
    }
    if !((*csa).nt).is_null() {
        _glp_spx_free_nt((*csa).lp, (*csa).nt);
    }
    glp_free((*csa).beta as *mut libc::c_void);
    glp_free((*csa).d as *mut libc::c_void);
    if !((*csa).se).is_null() {
        _glp_spx_free_se((*csa).lp, (*csa).se);
    }
    glp_free((*csa).list as *mut libc::c_void);
    _glp_fvs_free_vec(&mut (*csa).tcol);
    _glp_fvs_free_vec(&mut (*csa).trow);
    if !((*csa).bp).is_null() {
        glp_free((*csa).bp as *mut libc::c_void);
    }
    _glp_fvs_free_vec(&mut (*csa).work);
    return ret;
}
