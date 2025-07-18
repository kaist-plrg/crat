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
    fn abs(_: libc::c_int) -> libc::c_int;
    fn _glp_bfd_get_count(bfd: *mut BFD) -> libc::c_int;
    fn _glp_bfd_condest(bfd: *mut BFD) -> libc::c_double;
    fn _glp_fvs_alloc_vec(x: *mut FVS, n: libc::c_int);
    fn _glp_fvs_gather_vec(x: *mut FVS, eps: libc::c_double);
    fn _glp_fvs_adjust_vec(x: *mut FVS, eps: libc::c_double);
    fn _glp_fvs_free_vec(x: *mut FVS);
    fn _glp_spx_factorize(lp: *mut SPXLP) -> libc::c_int;
    fn _glp_spx_eval_beta(lp: *mut SPXLP, beta: *mut libc::c_double);
    fn _glp_spx_eval_obj(lp: *mut SPXLP, beta: *const libc::c_double) -> libc::c_double;
    fn _glp_spx_eval_pi(lp: *mut SPXLP, pi: *mut libc::c_double);
    fn _glp_spx_eval_dj(
        lp: *mut SPXLP,
        pi: *const libc::c_double,
        j: libc::c_int,
    ) -> libc::c_double;
    fn _glp_spx_eval_tcol(lp: *mut SPXLP, j: libc::c_int, tcol: *mut libc::c_double);
    fn _glp_spx_eval_rho(lp: *mut SPXLP, i: libc::c_int, rho: *mut libc::c_double);
    fn _glp_spx_update_beta_s(
        lp: *mut SPXLP,
        beta: *mut libc::c_double,
        p: libc::c_int,
        p_flag: libc::c_int,
        q: libc::c_int,
        tcol: *const FVS,
    );
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
    fn _glp_spx_alloc_at(lp: *mut SPXLP, at: *mut SPXAT);
    fn _glp_spx_build_at(lp: *mut SPXLP, at: *mut SPXAT);
    fn _glp_spx_eval_trow1(
        lp: *mut SPXLP,
        at: *mut SPXAT,
        rho: *const libc::c_double,
        trow: *mut libc::c_double,
    );
    fn _glp_spx_free_at(lp: *mut SPXLP, at: *mut SPXAT);
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
    fn _glp_spy_chuzc_std(
        lp: *mut SPXLP,
        d: *const libc::c_double,
        r: libc::c_double,
        trow: *const libc::c_double,
        tol_piv: libc::c_double,
        tol: libc::c_double,
        tol1: libc::c_double,
    ) -> libc::c_int;
    fn _glp_spy_chuzc_harris(
        lp: *mut SPXLP,
        d: *const libc::c_double,
        r: libc::c_double,
        trow: *const libc::c_double,
        tol_piv: libc::c_double,
        tol: libc::c_double,
        tol1: libc::c_double,
    ) -> libc::c_int;
    fn _glp_spy_ls_eval_bp(
        lp: *mut SPXLP,
        d: *const libc::c_double,
        r: libc::c_double,
        trow: *const libc::c_double,
        tol_piv: libc::c_double,
        bp: *mut SPYBP,
    ) -> libc::c_int;
    fn _glp_spy_ls_select_bp(
        lp: *mut SPXLP,
        trow: *const libc::c_double,
        nbp: libc::c_int,
        bp: *mut SPYBP,
        num: libc::c_int,
        slope: *mut libc::c_double,
        teta_lim: libc::c_double,
    ) -> libc::c_int;
    fn _glp_spy_chuzr_sel(
        lp: *mut SPXLP,
        beta: *const libc::c_double,
        tol: libc::c_double,
        tol1: libc::c_double,
        list: *mut libc::c_int,
    ) -> libc::c_int;
    fn _glp_spy_chuzr_std(
        lp: *mut SPXLP,
        beta: *const libc::c_double,
        num: libc::c_int,
        list: *const libc::c_int,
    ) -> libc::c_int;
    fn _glp_spy_alloc_se(lp: *mut SPXLP, se: *mut SPYSE);
    fn _glp_spy_reset_refsp(lp: *mut SPXLP, se: *mut SPYSE);
    fn _glp_spy_chuzr_pse(
        lp: *mut SPXLP,
        se: *mut SPYSE,
        beta: *const libc::c_double,
        num: libc::c_int,
        list: *const libc::c_int,
    ) -> libc::c_int;
    fn _glp_spy_update_gamma_s(
        lp: *mut SPXLP,
        se: *mut SPYSE,
        p: libc::c_int,
        q: libc::c_int,
        trow: *const FVS,
        tcol: *const FVS,
    ) -> libc::c_double;
    fn _glp_spy_free_se(lp: *mut SPXLP, se: *mut SPYSE);
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
    pub orig_b: *mut libc::c_double,
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
    pub se: *mut SPYSE,
    pub r: FVS,
    pub p: libc::c_int,
    pub trow: FVS,
    pub bp: *mut SPYBP,
    pub q: libc::c_int,
    pub tcol: FVS,
    pub work: *mut libc::c_double,
    pub work1: *mut libc::c_double,
    pub p_stat: libc::c_int,
    pub d_stat: libc::c_int,
    pub msg_lev: libc::c_int,
    pub dualp: libc::c_int,
    pub r_test: libc::c_int,
    pub tol_bnd: libc::c_double,
    pub tol_bnd1: libc::c_double,
    pub tol_dj: libc::c_double,
    pub tol_dj1: libc::c_double,
    pub tol_piv: libc::c_double,
    pub obj_lim: libc::c_double,
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
pub struct SPYBP {
    pub j: libc::c_int,
    pub teta: libc::c_double,
    pub dz: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPYSE {
    pub valid: libc::c_int,
    pub refsp: *mut libc::c_char,
    pub gamma: *mut libc::c_double,
    pub work: *mut libc::c_double,
    pub u: FVS,
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
unsafe extern "C" fn check_flags(mut csa: *mut csa) {
    let mut lp: *mut SPXLP = (*csa).lp;
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut flag: *mut libc::c_char = (*lp).flag;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    j = 1 as libc::c_int;
    while j <= n - m {
        k = *head.offset((m + j) as isize);
        if *l.offset(k as isize) == -1.7976931348623157e+308f64
            && *u.offset(k as isize) == 1.7976931348623157e+308f64
        {
            (*flag.offset(j as isize) == 0
                || {
                    glp_assert_(
                        b"!flag[j]\0" as *const u8 as *const libc::c_char,
                        b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                        223 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        } else if *l.offset(k as isize) != -1.7976931348623157e+308f64
            && *u.offset(k as isize) == 1.7976931348623157e+308f64
        {
            (*flag.offset(j as isize) == 0
                || {
                    glp_assert_(
                        b"!flag[j]\0" as *const u8 as *const libc::c_char,
                        b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                        225 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        } else if *l.offset(k as isize) == -1.7976931348623157e+308f64
            && *u.offset(k as isize) != 1.7976931348623157e+308f64
        {
            (*flag.offset(j as isize) as libc::c_int != 0
                || {
                    glp_assert_(
                        b"flag[j]\0" as *const u8 as *const libc::c_char,
                        b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                        227 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        } else if *l.offset(k as isize) == *u.offset(k as isize) {
            (*flag.offset(j as isize) == 0
                || {
                    glp_assert_(
                        b"!flag[j]\0" as *const u8 as *const libc::c_char,
                        b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                        229 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        j += 1;
        j;
    }
}
unsafe extern "C" fn set_art_bounds(mut csa: *mut csa) {
    let mut lp: *mut SPXLP = (*csa).lp;
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut b: *mut libc::c_double = (*lp).b;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut flag: *mut libc::c_char = (*lp).flag;
    let mut d: *mut libc::c_double = (*csa).d;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= m {
        *b.offset(i as isize) = 0.0f64;
        i += 1;
        i;
    }
    k = 1 as libc::c_int;
    while k <= n {
        if *((*csa).orig_l).offset(k as isize) == -1.7976931348623157e+308f64
            && *((*csa).orig_u).offset(k as isize) == 1.7976931348623157e+308f64
        {
            *l.offset(k as isize) = -1e3f64;
            *u.offset(k as isize) = 1e3f64;
        } else if *((*csa).orig_l).offset(k as isize) != -1.7976931348623157e+308f64
            && *((*csa).orig_u).offset(k as isize) == 1.7976931348623157e+308f64
        {
            *l.offset(k as isize) = 0.0f64;
            *u.offset(k as isize) = 1.0f64;
        } else if *((*csa).orig_l).offset(k as isize) == -1.7976931348623157e+308f64
            && *((*csa).orig_u).offset(k as isize) != 1.7976931348623157e+308f64
        {
            *l.offset(k as isize) = -1.0f64;
            *u.offset(k as isize) = 0.0f64;
        } else {
            let ref mut fresh0 = *u.offset(k as isize);
            *fresh0 = 0.0f64;
            *l.offset(k as isize) = *fresh0;
        }
        k += 1;
        k;
    }
    ((*csa).d_st == 1 as libc::c_int
        || {
            glp_assert_(
                b"csa->d_st == 1\0" as *const u8 as *const libc::c_char,
                b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                274 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    j = 1 as libc::c_int;
    while j <= n - m {
        k = *head.offset((m + j) as isize);
        *flag
            .offset(
                j as isize,
            ) = (*l.offset(k as isize) != *u.offset(k as isize)
            && *d.offset(j as isize) < 0.0f64) as libc::c_int as libc::c_char;
        j += 1;
        j;
    }
    (*csa).beta_st = 0 as libc::c_int;
}
unsafe extern "C" fn set_orig_bounds(mut csa: *mut csa) {
    let mut lp: *mut SPXLP = (*csa).lp;
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut b: *mut libc::c_double = (*lp).b;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut flag: *mut libc::c_char = (*lp).flag;
    let mut d: *mut libc::c_double = (*csa).d;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    memcpy(
        b as *mut libc::c_void,
        (*csa).orig_b as *const libc::c_void,
        ((1 as libc::c_int + m) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memcpy(
        l as *mut libc::c_void,
        (*csa).orig_l as *const libc::c_void,
        ((1 as libc::c_int + n) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memcpy(
        u as *mut libc::c_void,
        (*csa).orig_u as *const libc::c_void,
        ((1 as libc::c_int + n) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    ((*csa).d_st == 1 as libc::c_int
        || {
            glp_assert_(
                b"csa->d_st == 1\0" as *const u8 as *const libc::c_char,
                b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                311 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    j = 1 as libc::c_int;
    while j <= n - m {
        k = *head.offset((m + j) as isize);
        if *l.offset(k as isize) == -1.7976931348623157e+308f64
            && *u.offset(k as isize) == 1.7976931348623157e+308f64
        {
            *flag.offset(j as isize) = 0 as libc::c_int as libc::c_char;
        } else if *l.offset(k as isize) != -1.7976931348623157e+308f64
            && *u.offset(k as isize) == 1.7976931348623157e+308f64
        {
            *flag.offset(j as isize) = 0 as libc::c_int as libc::c_char;
        } else if *l.offset(k as isize) == -1.7976931348623157e+308f64
            && *u.offset(k as isize) != 1.7976931348623157e+308f64
        {
            *flag.offset(j as isize) = 1 as libc::c_int as libc::c_char;
        } else if *l.offset(k as isize) != *u.offset(k as isize) {
            *flag
                .offset(
                    j as isize,
                ) = (*d.offset(j as isize) < 0.0f64) as libc::c_int as libc::c_char;
        } else {
            *flag.offset(j as isize) = 0 as libc::c_int as libc::c_char;
        }
        j += 1;
        j;
    }
    (*csa).beta_st = 0 as libc::c_int;
}
unsafe extern "C" fn check_feas(
    mut csa: *mut csa,
    mut tol: libc::c_double,
    mut tol1: libc::c_double,
    mut recov: libc::c_int,
) -> libc::c_int {
    let mut lp: *mut SPXLP = (*csa).lp;
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut flag: *mut libc::c_char = (*lp).flag;
    let mut d: *mut libc::c_double = (*csa).d;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut eps: libc::c_double = 0.;
    ((*csa).d_st == 1 as libc::c_int
        || {
            glp_assert_(
                b"csa->d_st == 1\0" as *const u8 as *const libc::c_char,
                b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                378 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    j = 1 as libc::c_int;
    while j <= n - m {
        k = *head.offset((m + j) as isize);
        if !(*l.offset(k as isize) == *u.offset(k as isize)) {
            eps = tol
                + tol1
                    * (if *c.offset(k as isize) >= 0.0f64 {
                        *c.offset(k as isize)
                    } else {
                        -*c.offset(k as isize)
                    });
            if *d.offset(j as isize) > eps {
                if *l.offset(k as isize) == -1.7976931348623157e+308f64
                    || *flag.offset(j as isize) as libc::c_int != 0
                {
                    if *l.offset(k as isize) == -1.7976931348623157e+308f64 {
                        ret = j;
                        break;
                    } else {
                        if recov != 0 {
                            *flag.offset(j as isize) = 0 as libc::c_int as libc::c_char;
                        }
                        ret = -(1 as libc::c_int);
                    }
                }
            } else if *d.offset(j as isize) < -eps {
                if *flag.offset(j as isize) == 0 {
                    if *u.offset(k as isize) == 1.7976931348623157e+308f64 {
                        ret = j;
                        break;
                    } else {
                        if recov != 0 {
                            *flag.offset(j as isize) = 1 as libc::c_int as libc::c_char;
                        }
                        ret = -(1 as libc::c_int);
                    }
                }
            }
        }
        j += 1;
        j;
    }
    if recov != 0 && ret != 0 {
        (*csa).beta_st = 0 as libc::c_int;
    }
    return ret;
}
unsafe extern "C" fn spy_eval_r(
    mut lp: *mut SPXLP,
    mut beta: *const libc::c_double,
    mut tol: libc::c_double,
    mut tol1: libc::c_double,
    mut r: *mut FVS,
) {
    let mut m: libc::c_int = (*lp).m;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut ind: *mut libc::c_int = (*r).ind;
    let mut vec: *mut libc::c_double = (*r).vec;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nnz: libc::c_int = 0 as libc::c_int;
    let mut lk: libc::c_double = 0.;
    let mut uk: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    ((*r).n == m
        || {
            glp_assert_(
                b"r->n == m\0" as *const u8 as *const libc::c_char,
                b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                598 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    i = 1 as libc::c_int;
    while i <= m {
        *vec.offset(i as isize) = 0.0f64;
        k = *head.offset(i as isize);
        lk = *l.offset(k as isize);
        uk = *u.offset(k as isize);
        if *beta.offset(i as isize) < lk {
            eps = tol + tol1 * (if lk >= 0.0f64 { lk } else { -lk });
            if *beta.offset(i as isize) < lk - eps {
                nnz += 1;
                *ind.offset(nnz as isize) = i;
                *vec.offset(i as isize) = lk - *beta.offset(i as isize);
            }
        } else if *beta.offset(i as isize) > uk {
            eps = tol + tol1 * (if uk >= 0.0f64 { uk } else { -uk });
            if *beta.offset(i as isize) > uk + eps {
                nnz += 1;
                *ind.offset(nnz as isize) = i;
                *vec.offset(i as isize) = uk - *beta.offset(i as isize);
            }
        }
        i += 1;
        i;
    }
    (*r).nnz = nnz;
}
unsafe extern "C" fn choose_pivot(mut csa: *mut csa) -> libc::c_int {
    let mut lp: *mut SPXLP = (*csa).lp;
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut at: *mut SPXAT = (*csa).at;
    let mut nt: *mut SPXNT = (*csa).nt;
    let mut beta: *mut libc::c_double = (*csa).beta;
    let mut d: *mut libc::c_double = (*csa).d;
    let mut se: *mut SPYSE = (*csa).se;
    let mut list: *mut libc::c_int = (*csa).r.ind;
    let mut rho: *mut libc::c_double = (*csa).work;
    let mut trow: *mut libc::c_double = (*csa).work1;
    let mut bp: *mut SPYBP = (*csa).bp;
    let mut tol_piv: libc::c_double = (*csa).tol_piv;
    let mut try_0: libc::c_int = 0;
    let mut nnn: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut t_best: libc::c_int = 0;
    let mut nbp: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut big: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut best_ratio: libc::c_double = 0.;
    let mut dz_best: libc::c_double = 0.;
    ((*csa).beta_st != 0
        || {
            glp_assert_(
                b"csa->beta_st\0" as *const u8 as *const libc::c_char,
                b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                674 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*csa).d_st != 0
        || {
            glp_assert_(
                b"csa->d_st\0" as *const u8 as *const libc::c_char,
                b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                675 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    loop {
        nnn = (*csa).r.nnz;
        (*csa).p = 0 as libc::c_int;
        best_ratio = 0.0f64;
        ret = 0 as libc::c_int;
        try_0 = ret;
        loop {
            (nnn > 0 as libc::c_int
                || {
                    glp_assert_(
                        b"nnn > 0\0" as *const u8 as *const libc::c_char,
                        b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                        687 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            try_0 += 1;
            try_0;
            if se.is_null() {
                p = _glp_spy_chuzr_std(
                    lp,
                    beta as *const libc::c_double,
                    nnn,
                    list as *const libc::c_int,
                );
            } else {
                p = _glp_spy_chuzr_pse(
                    lp,
                    se,
                    beta as *const libc::c_double,
                    nnn,
                    list as *const libc::c_int,
                );
            }
            (1 as libc::c_int <= p && p <= m
                || {
                    glp_assert_(
                        b"1 <= p && p <= m\0" as *const u8 as *const libc::c_char,
                        b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                        697 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            _glp_spx_eval_rho(lp, p, rho);
            if !at.is_null() {
                _glp_spx_eval_trow1(lp, at, rho as *const libc::c_double, trow);
            } else {
                _glp_spx_nt_prod(
                    lp,
                    nt,
                    trow,
                    1 as libc::c_int,
                    -1.0f64,
                    rho as *const libc::c_double,
                );
            }
            big = 1.0f64;
            j = 1 as libc::c_int;
            while j <= n - m {
                temp = *trow.offset(j as isize);
                if temp < 0.0f64 {
                    temp = -temp;
                }
                if big < temp {
                    big = temp;
                }
                j += 1;
                j;
            }
            k = *head.offset(p as isize);
            (*beta.offset(p as isize) < *l.offset(k as isize)
                || *beta.offset(p as isize) > *u.offset(k as isize)
                || {
                    glp_assert_(
                        b"beta[p] < l[k] || beta[p] > u[k]\0" as *const u8
                            as *const libc::c_char,
                        b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                        721 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            r = if *beta.offset(p as isize) < *l.offset(k as isize) {
                *l.offset(k as isize) - *beta.offset(p as isize)
            } else {
                *u.offset(k as isize) - *beta.offset(p as isize)
            };
            if (*csa).r_test == 0x33 as libc::c_int && try_0 <= 2 as libc::c_int {
                let mut t_0: libc::c_int = 0;
                let mut num: libc::c_int = 0;
                let mut num1: libc::c_int = 0;
                let mut slope: libc::c_double = 0.;
                let mut teta_lim: libc::c_double = 0.;
                nbp = _glp_spy_ls_eval_bp(
                    lp,
                    d as *const libc::c_double,
                    r,
                    trow as *const libc::c_double,
                    tol_piv,
                    bp,
                );
                if !(nbp < 2 as libc::c_int) {
                    slope = fabs(r);
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
                                b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                                755 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    if teta_lim < 1e-6f64 {
                        teta_lim = 1e-6f64;
                    }
                    t_best = 0 as libc::c_int;
                    dz_best = 0.0f64;
                    num = 0 as libc::c_int;
                    while num < nbp {
                        num1 = _glp_spy_ls_select_bp(
                            lp,
                            trow as *const libc::c_double,
                            nbp,
                            bp,
                            num,
                            &mut slope,
                            teta_lim,
                        );
                        t_0 = num + 1 as libc::c_int;
                        while t_0 <= num1 {
                            if fabs(*trow.offset((*bp.offset(t_0 as isize)).j as isize))
                                / big >= 0.0001f64
                            {
                                if dz_best < (*bp.offset(t_0 as isize)).dz {
                                    t_best = t_0;
                                    dz_best = (*bp.offset(t_0 as isize)).dz;
                                }
                            }
                            t_0 += 1;
                            t_0;
                        }
                        if slope < 0.0f64 {
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
                                    b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                                    781 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        (*csa).p = p;
                        memcpy(
                            &mut *((*csa).trow.vec).offset(1 as libc::c_int as isize)
                                as *mut libc::c_double as *mut libc::c_void,
                            &mut *trow.offset(1 as libc::c_int as isize)
                                as *mut libc::c_double as *const libc::c_void,
                            ((n - m) as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                                ),
                        );
                        _glp_fvs_gather_vec(&mut (*csa).trow, 2.2204460492503131e-16f64);
                        (*csa).q = (*bp.offset(t_best as isize)).j;
                        best_ratio = fabs(
                            *trow.offset((*bp.offset(t_best as isize)).j as isize),
                        ) / big;
                        ret = 1 as libc::c_int;
                        break;
                    }
                }
            }
            if (*csa).r_test == 0x11 as libc::c_int {
                q = _glp_spy_chuzc_std(
                    lp,
                    d as *const libc::c_double,
                    r,
                    trow as *const libc::c_double,
                    tol_piv,
                    0.30f64 * (*csa).tol_dj,
                    0.30f64 * (*csa).tol_dj1,
                );
            } else {
                q = _glp_spy_chuzc_harris(
                    lp,
                    d as *const libc::c_double,
                    r,
                    trow as *const libc::c_double,
                    tol_piv,
                    0.35f64 * (*csa).tol_dj,
                    0.35f64 * (*csa).tol_dj1,
                );
            }
            if q == 0 as libc::c_int {
                (*csa).p = p;
                memcpy(
                    &mut *((*csa).trow.vec).offset(1 as libc::c_int as isize)
                        as *mut libc::c_double as *mut libc::c_void,
                    &mut *trow.offset(1 as libc::c_int as isize) as *mut libc::c_double
                        as *const libc::c_void,
                    ((n - m) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                        ),
                );
                _glp_fvs_gather_vec(&mut (*csa).trow, 2.2204460492503131e-16f64);
                (*csa).q = q;
                best_ratio = 1.0f64;
                break;
            } else {
                if best_ratio < fabs(*trow.offset(q as isize)) / big {
                    (*csa).p = p;
                    memcpy(
                        &mut *((*csa).trow.vec).offset(1 as libc::c_int as isize)
                            as *mut libc::c_double as *mut libc::c_void,
                        &mut *trow.offset(1 as libc::c_int as isize)
                            as *mut libc::c_double as *const libc::c_void,
                        ((n - m) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                            ),
                    );
                    _glp_fvs_gather_vec(&mut (*csa).trow, 2.2204460492503131e-16f64);
                    (*csa).q = q;
                    best_ratio = fabs(*trow.offset(q as isize)) / big;
                }
                if best_ratio >= 0.0001f64 || nnn == 1 as libc::c_int
                    || try_0 == 5 as libc::c_int
                {
                    break;
                }
                t = 1 as libc::c_int;
                while t <= nnn {
                    if *list.offset(t as isize) == p {
                        break;
                    }
                    t += 1;
                    t;
                }
                (t <= nnn
                    || {
                        glp_assert_(
                            b"t <= nnn\0" as *const u8 as *const libc::c_char,
                            b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                            844 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                *list.offset(t as isize) = *list.offset(nnn as isize);
                *list.offset(nnn as isize) = p;
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
        (*csa).beta_st = 0 as libc::c_int;
        t = 1 as libc::c_int;
        while t < t_best {
            k = *head.offset((m + (*bp.offset(t as isize)).j) as isize);
            (-1.7976931348623157e+308f64 < *l.offset(k as isize)
                && *l.offset(k as isize) < *u.offset(k as isize)
                && *u.offset(k as isize) < 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"-DBL_MAX < l[k] && l[k] < u[k] && u[k] < +DBL_MAX\0"
                            as *const u8 as *const libc::c_char,
                        b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                        877 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            *((*lp).flag)
                .offset(
                    (*bp.offset(t as isize)).j as isize,
                ) = (*((*lp).flag).offset((*bp.offset(t as isize)).j as isize) == 0)
                as libc::c_int as libc::c_char;
            t += 1;
            t;
        }
    }
    return ret;
}
unsafe extern "C" fn play_coef(mut csa: *mut csa, mut all: libc::c_int) {
    let mut lp: *mut SPXLP = (*csa).lp;
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut flag: *mut libc::c_char = (*lp).flag;
    let mut orig_c: *mut libc::c_double = (*csa).orig_c;
    let mut d: *mut libc::c_double = (*csa).d;
    let mut trow: *const libc::c_double = (*csa).trow.vec;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    static mut eps: libc::c_double = 1e-9f64;
    ((*csa).d_st != 0
        || {
            glp_assert_(
                b"csa->d_st\0" as *const u8 as *const libc::c_char,
                b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                920 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    j = 1 as libc::c_int;
    while j <= n - m {
        if all != 0 || *trow.offset(j as isize) != 0.0f64 {
            k = *head.offset((m + j) as isize);
            if !(*l.offset(k as isize) == *u.offset(k as isize)) {
                if *l.offset(k as isize) == -1.7976931348623157e+308f64
                    && *u.offset(k as isize) == 1.7976931348623157e+308f64
                {
                    *c.offset(k as isize) -= *d.offset(j as isize);
                    *d.offset(j as isize) = 0.0f64;
                } else if *flag.offset(j as isize) == 0 {
                    (*l.offset(k as isize) != -1.7976931348623157e+308f64
                        || {
                            glp_assert_(
                                b"l[k] != -DBL_MAX\0" as *const u8 as *const libc::c_char,
                                b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                                939 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    *d.offset(j as isize)
                        -= *c.offset(k as isize) - *orig_c.offset(k as isize);
                    *c.offset(k as isize) = *orig_c.offset(k as isize);
                    if *d.offset(j as isize) < eps {
                        *c.offset(k as isize) -= *d.offset(j as isize) - eps;
                        *d.offset(j as isize) = eps;
                    }
                } else {
                    (*u.offset(k as isize) != 1.7976931348623157e+308f64
                        || {
                            glp_assert_(
                                b"u[k] != +DBL_MAX\0" as *const u8 as *const libc::c_char,
                                b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                                950 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    *d.offset(j as isize)
                        -= *c.offset(k as isize) - *orig_c.offset(k as isize);
                    *c.offset(k as isize) = *orig_c.offset(k as isize);
                    if *d.offset(j as isize) > -eps {
                        *c.offset(k as isize) -= *d.offset(j as isize) + eps;
                        *d.offset(j as isize) = -eps;
                    }
                }
            }
        }
        j += 1;
        j;
    }
}
unsafe extern "C" fn remove_perturb(mut csa: *mut csa) {
    let mut lp: *mut SPXLP = (*csa).lp;
    let mut n: libc::c_int = (*lp).n;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut orig_c: *mut libc::c_double = (*csa).orig_c;
    memcpy(
        c as *mut libc::c_void,
        orig_c as *const libc::c_void,
        ((1 as libc::c_int + n) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    (*csa).d_st = 0 as libc::c_int;
    (*csa).phase = (*csa).d_st;
    if (*csa).msg_lev >= 3 as libc::c_int {
        glp_printf(
            b"Removing LP perturbation [%d]...\n\0" as *const u8 as *const libc::c_char,
            (*csa).it_cnt,
        );
    }
}
unsafe extern "C" fn display(mut csa: *mut csa, mut spec: libc::c_int) {
    let mut lp: *mut SPXLP = (*csa).lp;
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut flag: *mut libc::c_char = (*lp).flag;
    let mut l: *mut libc::c_double = (*csa).orig_l;
    let mut u: *mut libc::c_double = (*csa).orig_u;
    let mut beta: *mut libc::c_double = (*csa).beta;
    let mut d: *mut libc::c_double = (*csa).d;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nnn: libc::c_int = 0;
    let mut sum: libc::c_double = 0.;
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
                    match (*csa).phase {
                        1 => {
                            sum = 0.0f64;
                            nnn = 0 as libc::c_int;
                            j = 1 as libc::c_int;
                            while j <= n - m {
                                k = *head.offset((m + j) as isize);
                                if *d.offset(j as isize) > 0.0f64 {
                                    if *l.offset(k as isize) == -1.7976931348623157e+308f64 {
                                        sum += *d.offset(j as isize);
                                        if *d.offset(j as isize) > 1e-7f64 {
                                            nnn += 1;
                                            nnn;
                                        }
                                    }
                                } else if *d.offset(j as isize) < 0.0f64 {
                                    if *u.offset(k as isize) == 1.7976931348623157e+308f64 {
                                        sum -= *d.offset(j as isize);
                                        if *d.offset(j as isize) < -1e-7f64 {
                                            nnn += 1;
                                            nnn;
                                        }
                                    }
                                }
                                j += 1;
                                j;
                            }
                            glp_printf(
                                b" %6d: sum = %17.9e inf = %11.3e (%d)\0" as *const u8
                                    as *const libc::c_char,
                                (*csa).it_cnt,
                                *((*lp).c).offset(0 as libc::c_int as isize)
                                    - _glp_spx_eval_obj(lp, beta as *const libc::c_double),
                                sum,
                                nnn,
                            );
                        }
                        2 => {
                            sum = 0.0f64;
                            nnn = 0 as libc::c_int;
                            j = 1 as libc::c_int;
                            while j <= n - m {
                                k = *head.offset((m + j) as isize);
                                if *d.offset(j as isize) > 0.0f64 {
                                    if *l.offset(k as isize) == -1.7976931348623157e+308f64
                                        || *flag.offset(j as isize) as libc::c_int != 0
                                    {
                                        sum += *d.offset(j as isize);
                                    }
                                } else if *d.offset(j as isize) < 0.0f64 {
                                    if *l.offset(k as isize) != *u.offset(k as isize)
                                        && *flag.offset(j as isize) == 0
                                    {
                                        sum -= *d.offset(j as isize);
                                    }
                                }
                                j += 1;
                                j;
                            }
                            nnn = _glp_spy_chuzr_sel(
                                lp,
                                beta as *const libc::c_double,
                                (*csa).tol_bnd,
                                (*csa).tol_bnd1,
                                0 as *mut libc::c_int,
                            );
                            glp_printf(
                                b"#%6d: obj = %17.9e inf = %11.3e (%d)\0" as *const u8
                                    as *const libc::c_char,
                                (*csa).it_cnt,
                                (*csa).dir as libc::c_double * (*csa).fz
                                    * _glp_spx_eval_obj(lp, beta as *const libc::c_double),
                                sum,
                                nnn,
                            );
                        }
                        _ => {
                            (csa != csa
                                || {
                                    glp_assert_(
                                        b"csa != csa\0" as *const u8 as *const libc::c_char,
                                        b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                                        1103 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                        }
                    }
                    if (*csa).inv_cnt != 0 {
                        glp_printf(
                            b" %d\0" as *const u8 as *const libc::c_char,
                            (*csa).inv_cnt,
                        );
                        (*csa).inv_cnt = 0 as libc::c_int;
                    }
                    if (*csa).r_test == 0x33 as libc::c_int {
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
unsafe extern "C" fn spy_update_r(
    mut lp: *mut SPXLP,
    mut p: libc::c_int,
    mut q: libc::c_int,
    mut beta: *const libc::c_double,
    mut tcol: *const FVS,
    mut tol: libc::c_double,
    mut tol1: libc::c_double,
    mut r: *mut FVS,
) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut tcol_ind: *mut libc::c_int = (*tcol).ind;
    let mut ind: *mut libc::c_int = (*r).ind;
    let mut vec: *mut libc::c_double = (*r).vec;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut nnz: libc::c_int = 0;
    let mut lk: libc::c_double = 0.;
    let mut uk: libc::c_double = 0.;
    let mut ri: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    (1 as libc::c_int <= p && p <= m
        || {
            glp_assert_(
                b"1 <= p && p <= m\0" as *const u8 as *const libc::c_char,
                b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                1145 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= q && q <= n - m
        || {
            glp_assert_(
                b"1 <= q && q <= n-m\0" as *const u8 as *const libc::c_char,
                b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                1146 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    nnz = (*r).nnz;
    t = (*tcol).nnz;
    while t >= 1 as libc::c_int {
        i = *tcol_ind.offset(t as isize);
        if i == p {
            k = *head.offset((m + q) as isize);
        } else {
            k = *head.offset(i as isize);
        }
        lk = *l.offset(k as isize);
        uk = *u.offset(k as isize);
        ri = 0.0f64;
        if *beta.offset(i as isize) < lk {
            eps = tol + tol1 * (if lk >= 0.0f64 { lk } else { -lk });
            if *beta.offset(i as isize) < lk - eps {
                ri = lk - *beta.offset(i as isize);
            }
        } else if *beta.offset(i as isize) > uk {
            eps = tol + tol1 * (if uk >= 0.0f64 { uk } else { -uk });
            if *beta.offset(i as isize) > uk + eps {
                ri = uk - *beta.offset(i as isize);
            }
        }
        if ri == 0.0f64 {
            if *vec.offset(i as isize) != 0.0f64 {
                *vec.offset(i as isize) = 2.2250738585072014e-308f64;
            }
        } else {
            if *vec.offset(i as isize) == 0.0f64 {
                nnz += 1;
                *ind.offset(nnz as isize) = i;
            }
            *vec.offset(i as isize) = ri;
        }
        t -= 1;
        t;
    }
    (*r).nnz = nnz;
    _glp_fvs_adjust_vec(r, 2.2250738585072014e-308f64 + 2.2250738585072014e-308f64);
}
unsafe extern "C" fn dual_simplex(mut csa: *mut csa) -> libc::c_int {
    let mut lp: *mut SPXLP = (*csa).lp;
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut nt: *mut SPXNT = (*csa).nt;
    let mut beta: *mut libc::c_double = (*csa).beta;
    let mut d: *mut libc::c_double = (*csa).d;
    let mut se: *mut SPYSE = (*csa).se;
    let mut pi: *mut libc::c_double = (*csa).work;
    let mut msg_lev: libc::c_int = (*csa).msg_lev;
    let mut tol_bnd: libc::c_double = (*csa).tol_bnd;
    let mut tol_bnd1: libc::c_double = (*csa).tol_bnd1;
    let mut tol_dj: libc::c_double = (*csa).tol_dj;
    let mut tol_dj1: libc::c_double = (*csa).tol_dj1;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut p_flag: libc::c_int = 0;
    let mut refct: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut perturb: libc::c_int = -(1 as libc::c_int);
    let mut instab: libc::c_int = 0 as libc::c_int;
    check_flags(csa);
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
            if (*csa).phase == 0 {
                j = check_feas(
                    csa,
                    0.97f64 * tol_dj,
                    0.97f64 * tol_dj1,
                    1 as libc::c_int,
                );
                if j > 0 as libc::c_int {
                    set_art_bounds(csa);
                    (*csa).phase = 1 as libc::c_int;
                } else {
                    (*csa).phase = 2 as libc::c_int;
                }
            }
            if perturb <= 0 as libc::c_int {
                if check_feas(csa, tol_dj, tol_dj1, 0 as libc::c_int) != 0 {
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
                                b"Warning: numerical instability (dual simplex, phase %s)\n\0"
                                    as *const u8 as *const libc::c_char,
                                if (*csa).phase == 1 as libc::c_int {
                                    b"I\0" as *const u8 as *const libc::c_char
                                } else {
                                    b"II\0" as *const u8 as *const libc::c_char
                                },
                            );
                        }
                        instab += 1;
                        instab;
                        if (*csa).dualp != 0 && instab >= 10 as libc::c_int {
                            if msg_lev >= 1 as libc::c_int {
                                glp_printf(
                                    b"Warning: dual simplex failed due to excessive numerical instability\n\0"
                                        as *const u8 as *const libc::c_char,
                                );
                            }
                            (*csa).d_stat = 1 as libc::c_int;
                            (*csa).p_stat = (*csa).d_stat;
                            ret = -(1 as libc::c_int);
                            break;
                        } else {
                            j = check_feas(
                                csa,
                                0.97f64 * tol_dj,
                                0.97f64 * tol_dj1,
                                1 as libc::c_int,
                            );
                            if j > 0 as libc::c_int {
                                ((*csa).phase == 2 as libc::c_int
                                    || {
                                        glp_assert_(
                                            b"csa->phase == 2\0" as *const u8 as *const libc::c_char,
                                            b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                                            1357 as libc::c_int,
                                        );
                                        1 as libc::c_int != 0
                                    }) as libc::c_int;
                                set_art_bounds(csa);
                                (*csa).phase = 1 as libc::c_int;
                            }
                        }
                    }
                }
            } else {
                play_coef(csa, 1 as libc::c_int);
            }
        }
        ((*csa).phase == 1 as libc::c_int || (*csa).phase == 2 as libc::c_int
            || {
                glp_assert_(
                    b"csa->phase == 1 || csa->phase == 2\0" as *const u8
                        as *const libc::c_char,
                    b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                    1371 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if (*csa).beta_st == 0 {
            _glp_spx_eval_beta(lp, beta);
            match (*csa).phase {
                1 => {
                    spy_eval_r(
                        lp,
                        beta as *const libc::c_double,
                        1e-8f64,
                        0.0f64,
                        &mut (*csa).r,
                    );
                }
                2 => {
                    spy_eval_r(
                        lp,
                        beta as *const libc::c_double,
                        tol_bnd,
                        tol_bnd1,
                        &mut (*csa).r,
                    );
                }
                _ => {
                    (csa != csa
                        || {
                            glp_assert_(
                                b"csa != csa\0" as *const u8 as *const libc::c_char,
                                b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                                1385 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
            }
            (*csa).beta_st = 1 as libc::c_int;
        }
        if !se.is_null() && (*se).valid == 0 {
            _glp_spy_reset_refsp(lp, se);
            refct = 1000 as libc::c_int;
        }
        ((*lp).valid != 0 && (*csa).beta_st != 0 && (*csa).d_st != 0
            || {
                glp_assert_(
                    b"lp->valid && csa->beta_st && csa->d_st\0" as *const u8
                        as *const libc::c_char,
                    b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                    1395 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if (*csa).phase == 2 as libc::c_int
            && (*csa).obj_lim != 1.7976931348623157e+308f64
            && _glp_spx_eval_obj(lp, beta as *const libc::c_double) >= (*csa).obj_lim
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
                    b"OBJECTIVE %s LIMIT REACHED; SEARCH TERMINATED\n\0" as *const u8
                        as *const libc::c_char,
                    if (*csa).dir > 0 as libc::c_int {
                        b"UPPER\0" as *const u8 as *const libc::c_char
                    } else {
                        b"LOWER\0" as *const u8 as *const libc::c_char
                    },
                );
            }
            spy_eval_r(
                lp,
                beta as *const libc::c_double,
                tol_bnd,
                tol_bnd1,
                &mut (*csa).r,
            );
            (*csa)
                .p_stat = if (*csa).r.nnz == 0 as libc::c_int {
                2 as libc::c_int
            } else {
                3 as libc::c_int
            };
            (*csa).d_stat = 2 as libc::c_int;
            ret = if (*csa).dir > 0 as libc::c_int {
                0x7 as libc::c_int
            } else {
                0x6 as libc::c_int
            };
            break;
        } else if (*csa).it_cnt - (*csa).it_beg >= (*csa).it_lim {
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
            if (*csa).phase == 1 as libc::c_int {
                set_orig_bounds(csa);
                check_flags(csa);
                _glp_spx_eval_beta(lp, beta);
            }
            spy_eval_r(
                lp,
                beta as *const libc::c_double,
                tol_bnd,
                tol_bnd1,
                &mut (*csa).r,
            );
            (*csa)
                .p_stat = if (*csa).r.nnz == 0 as libc::c_int {
                2 as libc::c_int
            } else {
                3 as libc::c_int
            };
            (*csa)
                .d_stat = if (*csa).phase == 1 as libc::c_int {
                3 as libc::c_int
            } else {
                2 as libc::c_int
            };
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
            if (*csa).phase == 1 as libc::c_int {
                set_orig_bounds(csa);
                check_flags(csa);
                _glp_spx_eval_beta(lp, beta);
            }
            spy_eval_r(
                lp,
                beta as *const libc::c_double,
                tol_bnd,
                tol_bnd1,
                &mut (*csa).r,
            );
            (*csa)
                .p_stat = if (*csa).r.nnz == 0 as libc::c_int {
                2 as libc::c_int
            } else {
                3 as libc::c_int
            };
            (*csa)
                .d_stat = if (*csa).phase == 1 as libc::c_int {
                3 as libc::c_int
            } else {
                2 as libc::c_int
            };
            ret = 0x9 as libc::c_int;
            break;
        } else {
            display(csa, 0 as libc::c_int);
            if (*csa).r.nnz == 0 as libc::c_int {
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
                        set_orig_bounds(csa);
                        check_flags(csa);
                        if check_feas(csa, tol_dj, tol_dj1, 0 as libc::c_int)
                            == 0 as libc::c_int
                        {
                            (*csa).phase = 2 as libc::c_int;
                            ((*csa).beta_st == 0
                                || {
                                    glp_assert_(
                                        b"!csa->beta_st\0" as *const u8 as *const libc::c_char,
                                        b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                                        1555 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                            continue;
                        } else if perturb > 0 as libc::c_int {
                            remove_perturb(csa);
                            perturb = 0 as libc::c_int;
                            continue;
                        } else {
                            if msg_lev >= 3 as libc::c_int {
                                glp_printf(
                                    b"LP HAS NO DUAL FEASIBLE SOLUTION\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            _glp_spx_eval_beta(lp, beta);
                            spy_eval_r(
                                lp,
                                beta as *const libc::c_double,
                                tol_bnd,
                                tol_bnd1,
                                &mut (*csa).r,
                            );
                            (*csa)
                                .p_stat = if (*csa).r.nnz == 0 as libc::c_int {
                                2 as libc::c_int
                            } else {
                                3 as libc::c_int
                            };
                            (*csa).d_stat = 4 as libc::c_int;
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
                                    b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                                    1589 as libc::c_int,
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
                if (*csa).q == 0 as libc::c_int {
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
                                    b"Error: dual simplex failed\n\0" as *const u8
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
                                    b"LP HAS NO PRIMAL FEASIBLE SOLUTION\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            (*csa).p_stat = 4 as libc::c_int;
                            (*csa).d_stat = 2 as libc::c_int;
                            ret = 0 as libc::c_int;
                            break;
                        }
                        _ => {
                            (csa != csa
                                || {
                                    glp_assert_(
                                        b"csa != csa\0" as *const u8 as *const libc::c_char,
                                        b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                                        1645 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                        }
                    }
                }
                _glp_spx_eval_tcol(lp, (*csa).q, (*csa).tcol.vec);
                _glp_fvs_gather_vec(&mut (*csa).tcol, 2.2204460492503131e-16f64);
                if *((*csa).tcol.vec).offset((*csa).p as isize) == 0.0f64 {
                    if msg_lev >= 1 as libc::c_int {
                        glp_printf(
                            b"Error: tcol[p] = 0.0\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    (*csa).d_stat = 1 as libc::c_int;
                    (*csa).p_stat = (*csa).d_stat;
                    ret = 0x5 as libc::c_int;
                    break;
                } else {
                    k = *head.offset((*csa).p as isize);
                    p_flag = (*l.offset(k as isize) != *u.offset(k as isize)
                        && *beta.offset((*csa).p as isize) > *u.offset(k as isize))
                        as libc::c_int;
                    if (*csa).beta_st != 0 {
                        _glp_spx_update_beta_s(
                            lp,
                            beta,
                            (*csa).p,
                            p_flag,
                            (*csa).q,
                            &mut (*csa).tcol,
                        );
                        match (*csa).phase {
                            1 => {
                                spy_update_r(
                                    lp,
                                    (*csa).p,
                                    (*csa).q,
                                    beta as *const libc::c_double,
                                    &mut (*csa).tcol,
                                    1e-8f64,
                                    0.0f64,
                                    &mut (*csa).r,
                                );
                            }
                            2 => {
                                spy_update_r(
                                    lp,
                                    (*csa).p,
                                    (*csa).q,
                                    beta as *const libc::c_double,
                                    &mut (*csa).tcol,
                                    tol_bnd,
                                    tol_bnd1,
                                    &mut (*csa).r,
                                );
                            }
                            _ => {
                                (csa != csa
                                    || {
                                        glp_assert_(
                                            b"csa != csa\0" as *const u8 as *const libc::c_char,
                                            b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                                            1702 as libc::c_int,
                                        );
                                        1 as libc::c_int != 0
                                    }) as libc::c_int;
                            }
                        }
                        (*csa).beta_st = 2 as libc::c_int;
                    }
                    let mut k_0: libc::c_int = 0;
                    (1 as libc::c_int <= (*csa).p && (*csa).p <= m
                        || {
                            glp_assert_(
                                b"1 <= csa->p && csa->p <= m\0" as *const u8
                                    as *const libc::c_char,
                                b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                                1715 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    (1 as libc::c_int <= (*csa).q && (*csa).q <= n - m
                        || {
                            glp_assert_(
                                b"1 <= csa->q && csa->q <= n-m\0" as *const u8
                                    as *const libc::c_char,
                                b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                                1716 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    k_0 = *head.offset((m + (*csa).q) as isize);
                    if !(*((*lp).l).offset(k_0 as isize) == -1.7976931348623157e+308f64
                        && *((*lp).u).offset(k_0 as isize) == 1.7976931348623157e+308f64)
                    {
                        if fabs(*d.offset((*csa).q as isize)) >= 1e-6f64 {
                            (*csa).degen = 0 as libc::c_int;
                        } else {
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
                    ((*csa).d_st != 0
                        || {
                            glp_assert_(
                                b"csa->d_st\0" as *const u8 as *const libc::c_char,
                                b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                                1739 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
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
                    } else {
                        (*csa).d_st = 0 as libc::c_int;
                    }
                    if !se.is_null() {
                        if refct > 0 as libc::c_int {
                            if _glp_spy_update_gamma_s(
                                lp,
                                se,
                                (*csa).p,
                                (*csa).q,
                                &mut (*csa).trow,
                                &mut (*csa).tcol,
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
                    _glp_spx_change_basis(lp, (*csa).p, p_flag, (*csa).q);
                    _glp_spx_update_invb(lp, (*csa).p, *head.offset((*csa).p as isize));
                    if perturb > 0 as libc::c_int && (*csa).d_st != 0 {
                        play_coef(csa, 0 as libc::c_int);
                    }
                    (*csa).it_cnt += 1;
                    (*csa).it_cnt;
                }
            }
        }
    }
    return ret;
}
pub unsafe extern "C" fn _glp_spy_dual(
    mut P: *mut glp_prob,
    mut parm: *const glp_smcp,
) -> libc::c_int {
    let mut csa_: csa = csa {
        lp: 0 as *mut SPXLP,
        dir: 0,
        fz: 0.,
        orig_b: 0 as *mut libc::c_double,
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
        se: 0 as *mut SPYSE,
        r: FVS {
            n: 0,
            nnz: 0,
            ind: 0 as *mut libc::c_int,
            vec: 0 as *mut libc::c_double,
        },
        p: 0,
        trow: FVS {
            n: 0,
            nnz: 0,
            ind: 0 as *mut libc::c_int,
            vec: 0 as *mut libc::c_double,
        },
        bp: 0 as *mut SPYBP,
        q: 0,
        tcol: FVS {
            n: 0,
            nnz: 0,
            ind: 0 as *mut libc::c_int,
            vec: 0 as *mut libc::c_double,
        },
        work: 0 as *mut libc::c_double,
        work1: 0 as *mut libc::c_double,
        p_stat: 0,
        d_stat: 0,
        msg_lev: 0,
        dualp: 0,
        r_test: 0,
        tol_bnd: 0.,
        tol_bnd1: 0.,
        tol_dj: 0.,
        tol_dj1: 0.,
        tol_piv: 0.,
        obj_lim: 0.,
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
    let mut se: SPYSE = SPYSE {
        valid: 0,
        refsp: 0 as *mut libc::c_char,
        gamma: 0 as *mut libc::c_double,
        work: 0 as *mut libc::c_double,
        u: FVS {
            n: 0,
            nnz: 0,
            ind: 0 as *mut libc::c_int,
            vec: 0 as *mut libc::c_double,
        },
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
                        b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                        1859 as libc::c_int,
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
        .orig_b = glp_alloc(
        1 as libc::c_int + (*(*csa).lp).m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    memcpy(
        (*csa).orig_b as *mut libc::c_void,
        (*(*csa).lp).b as *const libc::c_void,
        ((1 as libc::c_int + (*(*csa).lp).m) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
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
                        b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                        1901 as libc::c_int,
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
            (*csa).se = 0 as *mut SPYSE;
        }
        34 => {
            (*csa).se = &mut se;
            _glp_spy_alloc_se((*csa).lp, (*csa).se);
        }
        _ => {
            (parm != parm
                || {
                    glp_assert_(
                        b"parm != parm\0" as *const u8 as *const libc::c_char,
                        b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                        1918 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    _glp_fvs_alloc_vec(&mut (*csa).r, (*(*csa).lp).m);
    _glp_fvs_alloc_vec(&mut (*csa).trow, (*(*csa).lp).n - (*(*csa).lp).m);
    _glp_fvs_alloc_vec(&mut (*csa).tcol, (*(*csa).lp).m);
    (*csa).bp = 0 as *mut SPYBP;
    (*csa)
        .work = glp_alloc(
        1 as libc::c_int + (*(*csa).lp).m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*csa)
        .work1 = glp_alloc(
        1 as libc::c_int + (*(*csa).lp).n - (*(*csa).lp).m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*csa).msg_lev = (*parm).msg_lev;
    (*csa).dualp = ((*parm).meth == 2 as libc::c_int) as libc::c_int;
    match (*parm).r_test {
        17 | 34 => {}
        51 => {
            (*csa)
                .bp = glp_alloc(
                1 as libc::c_int + (*(*csa).lp).n - (*(*csa).lp).m,
                ::std::mem::size_of::<SPYBP>() as libc::c_ulong as libc::c_int,
            ) as *mut SPYBP;
        }
        _ => {
            (parm != parm
                || {
                    glp_assert_(
                        b"parm != parm\0" as *const u8 as *const libc::c_char,
                        b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                        1963 as libc::c_int,
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
    match (*P).dir {
        1 => {
            (*csa).obj_lim = (*parm).obj_ul;
        }
        2 => {
            (*csa).obj_lim = -(*parm).obj_ll;
        }
        _ => {
            (parm != parm
                || {
                    glp_assert_(
                        b"parm != parm\0" as *const u8 as *const libc::c_char,
                        b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                        1983 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    if (*csa).obj_lim != 1.7976931348623157e+308f64 {
        (*csa).obj_lim /= (*csa).fz;
    }
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
    ret = dual_simplex(csa);
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
        _glp_spx_eval_pi((*csa).lp, (*csa).work);
        i = 1 as libc::c_int;
        while i <= (*(*csa).lp).m {
            *((*csa).work).offset(i as isize) *= (*csa).fz;
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
            (*csa).work as *const libc::c_double,
            (*csa).d as *const libc::c_double,
        );
        glp_free(daeh as *mut libc::c_void);
        (*P).it_cnt = (*csa).it_cnt;
        (*P).some = 0 as libc::c_int;
        if (*csa).p_stat == 4 as libc::c_int && (*csa).d_stat == 2 as libc::c_int {
            let mut k_0: libc::c_int = 0;
            let mut kk: libc::c_int = 0;
            (1 as libc::c_int <= (*csa).p && (*csa).p <= (*(*csa).lp).m
                || {
                    glp_assert_(
                        b"1 <= csa->p && csa->p <= csa->lp->m\0" as *const u8
                            as *const libc::c_char,
                        b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                        2044 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            k_0 = *((*(*csa).lp).head).offset((*csa).p as isize);
            (1 as libc::c_int <= k_0 && k_0 <= (*(*csa).lp).n
                || {
                    glp_assert_(
                        b"1 <= k && k <= csa->lp->n\0" as *const u8
                            as *const libc::c_char,
                        b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                        2046 as libc::c_int,
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
                        b"simplex/spydual.c\0" as *const u8 as *const libc::c_char,
                        2054 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    _glp_spx_free_lp((*csa).lp);
    glp_free(map as *mut libc::c_void);
    glp_free((*csa).orig_b as *mut libc::c_void);
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
        _glp_spy_free_se((*csa).lp, (*csa).se);
    }
    _glp_fvs_free_vec(&mut (*csa).r);
    _glp_fvs_free_vec(&mut (*csa).trow);
    if !((*csa).bp).is_null() {
        glp_free((*csa).bp as *mut libc::c_void);
    }
    _glp_fvs_free_vec(&mut (*csa).tcol);
    glp_free((*csa).work as *mut libc::c_void);
    glp_free((*csa).work1 as *mut libc::c_void);
    return if ret >= 0 as libc::c_int { ret } else { 0x5 as libc::c_int };
}
