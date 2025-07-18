use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_cfg;
    pub type glp_mir;
    pub type glp_cov;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
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
    fn glp_create_prob() -> *mut glp_prob;
    fn glp_set_col_bnds(
        P: *mut glp_prob,
        j: libc::c_int,
        type_0: libc::c_int,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_copy_prob(dest: *mut glp_prob, prob: *mut glp_prob, names: libc::c_int);
    fn glp_delete_prob(P: *mut glp_prob);
    fn glp_simplex(P: *mut glp_prob, parm: *const glp_smcp) -> libc::c_int;
    fn glp_init_smcp(parm: *mut glp_smcp);
    fn glp_get_status(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_prim_stat(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_dual_stat(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_obj_val(P: *mut glp_prob) -> libc::c_double;
    fn glp_get_row_stat(P: *mut glp_prob, i: libc::c_int) -> libc::c_int;
    fn glp_get_row_dual(P: *mut glp_prob, i: libc::c_int) -> libc::c_double;
    fn glp_get_col_stat(P: *mut glp_prob, j: libc::c_int) -> libc::c_int;
    fn glp_get_col_prim(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
    fn glp_get_col_dual(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
    fn glp_get_col_kind(P: *mut glp_prob, j: libc::c_int) -> libc::c_int;
    fn glp_eval_tab_row(
        P: *mut glp_prob,
        k: libc::c_int,
        ind: *mut libc::c_int,
        val: *mut libc::c_double,
    ) -> libc::c_int;
    fn glp_dual_rtest(
        P: *mut glp_prob,
        len: libc::c_int,
        ind: *const libc::c_int,
        val: *const libc::c_double,
        dir: libc::c_int,
        eps: libc::c_double,
    ) -> libc::c_int;
    fn glp_ios_can_branch(T: *mut glp_tree, j: libc::c_int) -> libc::c_int;
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
pub struct glp_tree {
    pub magic: libc::c_int,
    pub pool: *mut DMP,
    pub n: libc::c_int,
    pub orig_m: libc::c_int,
    pub orig_type: *mut libc::c_uchar,
    pub orig_lb: *mut libc::c_double,
    pub orig_ub: *mut libc::c_double,
    pub orig_stat: *mut libc::c_uchar,
    pub orig_prim: *mut libc::c_double,
    pub orig_dual: *mut libc::c_double,
    pub orig_obj: libc::c_double,
    pub nslots: libc::c_int,
    pub avail: libc::c_int,
    pub slot: *mut IOSLOT,
    pub head: *mut IOSNPD,
    pub tail: *mut IOSNPD,
    pub a_cnt: libc::c_int,
    pub n_cnt: libc::c_int,
    pub t_cnt: libc::c_int,
    pub root_m: libc::c_int,
    pub root_type: *mut libc::c_uchar,
    pub root_lb: *mut libc::c_double,
    pub root_ub: *mut libc::c_double,
    pub root_stat: *mut libc::c_uchar,
    pub curr: *mut IOSNPD,
    pub mip: *mut glp_prob,
    pub non_int: *mut libc::c_uchar,
    pub pred_m: libc::c_int,
    pub pred_max: libc::c_int,
    pub pred_type: *mut libc::c_uchar,
    pub pred_lb: *mut libc::c_double,
    pub pred_ub: *mut libc::c_double,
    pub pred_stat: *mut libc::c_uchar,
    pub local: *mut IOSPOOL,
    pub cov_gen: *mut glp_cov,
    pub mir_gen: *mut glp_mir,
    pub clq_gen: *mut glp_cfg,
    pub pcost: *mut libc::c_void,
    pub iwrk: *mut libc::c_int,
    pub dwrk: *mut libc::c_double,
    pub parm: *const glp_iocp,
    pub tm_beg: libc::c_double,
    pub tm_lag: libc::c_double,
    pub sol_cnt: libc::c_int,
    pub P: *mut libc::c_void,
    pub npp: *mut libc::c_void,
    pub save_sol: *const libc::c_char,
    pub save_cnt: libc::c_int,
    pub reason: libc::c_int,
    pub stop: libc::c_int,
    pub next_p: libc::c_int,
    pub reopt: libc::c_int,
    pub reinv: libc::c_int,
    pub br_var: libc::c_int,
    pub br_sel: libc::c_int,
    pub child: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_iocp {
    pub msg_lev: libc::c_int,
    pub br_tech: libc::c_int,
    pub bt_tech: libc::c_int,
    pub tol_int: libc::c_double,
    pub tol_obj: libc::c_double,
    pub tm_lim: libc::c_int,
    pub out_frq: libc::c_int,
    pub out_dly: libc::c_int,
    pub cb_func: Option::<unsafe extern "C" fn(*mut glp_tree, *mut libc::c_void) -> ()>,
    pub cb_info: *mut libc::c_void,
    pub cb_size: libc::c_int,
    pub pp_tech: libc::c_int,
    pub mip_gap: libc::c_double,
    pub mir_cuts: libc::c_int,
    pub gmi_cuts: libc::c_int,
    pub cov_cuts: libc::c_int,
    pub clq_cuts: libc::c_int,
    pub presolve: libc::c_int,
    pub binarize: libc::c_int,
    pub fp_heur: libc::c_int,
    pub ps_heur: libc::c_int,
    pub ps_tm_lim: libc::c_int,
    pub sr_heur: libc::c_int,
    pub use_sol: libc::c_int,
    pub save_sol: *const libc::c_char,
    pub alien: libc::c_int,
    pub flip: libc::c_int,
    pub foo_bar: [libc::c_double; 23],
}
pub type IOSPOOL = glp_prob;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSNPD {
    pub p: libc::c_int,
    pub up: *mut IOSNPD,
    pub level: libc::c_int,
    pub count: libc::c_int,
    pub b_ptr: *mut IOSBND,
    pub s_ptr: *mut IOSTAT,
    pub r_ptr: *mut IOSROW,
    pub solved: libc::c_int,
    pub lp_obj: libc::c_double,
    pub bound: libc::c_double,
    pub ii_cnt: libc::c_int,
    pub ii_sum: libc::c_double,
    pub changed: libc::c_int,
    pub br_var: libc::c_int,
    pub br_val: libc::c_double,
    pub data: *mut libc::c_void,
    pub temp: *mut IOSNPD,
    pub prev: *mut IOSNPD,
    pub next: *mut IOSNPD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSROW {
    pub name: *mut libc::c_char,
    pub origin: libc::c_uchar,
    pub klass: libc::c_uchar,
    pub type_0: libc::c_uchar,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut IOSAIJ,
    pub rii: libc::c_double,
    pub stat: libc::c_uchar,
    pub next: *mut IOSROW,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSAIJ {
    pub j: libc::c_int,
    pub val: libc::c_double,
    pub next: *mut IOSAIJ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSTAT {
    pub k: libc::c_int,
    pub stat: libc::c_uchar,
    pub next: *mut IOSTAT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSBND {
    pub k: libc::c_int,
    pub type_0: libc::c_uchar,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub next: *mut IOSBND,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSLOT {
    pub node: *mut IOSNPD,
    pub next: libc::c_int,
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
    pub dn_cnt: *mut libc::c_int,
    pub dn_sum: *mut libc::c_double,
    pub up_cnt: *mut libc::c_int,
    pub up_sum: *mut libc::c_double,
}
pub unsafe extern "C" fn _glp_ios_choose_var(
    mut T: *mut glp_tree,
    mut next: *mut libc::c_int,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    if (*(*T).parm).br_tech == 1 as libc::c_int {
        j = branch_first(T, next);
    } else if (*(*T).parm).br_tech == 2 as libc::c_int {
        j = branch_last(T, next);
    } else if (*(*T).parm).br_tech == 3 as libc::c_int {
        j = branch_mostf(T, next);
    } else if (*(*T).parm).br_tech == 4 as libc::c_int {
        j = branch_drtom(T, next);
    } else if (*(*T).parm).br_tech == 5 as libc::c_int {
        j = _glp_ios_pcost_branch(T, next);
    } else {
        (T != T
            || {
                glp_assert_(
                    b"T != T\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios09.c\0" as *const u8 as *const libc::c_char,
                    73 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    return j;
}
unsafe extern "C" fn branch_first(
    mut T: *mut glp_tree,
    mut _next: *mut libc::c_int,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    let mut beta: libc::c_double = 0.;
    j = 1 as libc::c_int;
    while j <= (*T).n {
        if *((*T).non_int).offset(j as isize) != 0 {
            break;
        }
        j += 1;
        j;
    }
    (1 as libc::c_int <= j && j <= (*T).n
        || {
            glp_assert_(
                b"1 <= j && j <= T->n\0" as *const u8 as *const libc::c_char,
                b"draft/glpios09.c\0" as *const u8 as *const libc::c_char,
                93 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    beta = glp_get_col_prim((*T).mip, j);
    if beta - floor(beta) < ceil(beta) - beta {
        next = 1 as libc::c_int;
    } else {
        next = 2 as libc::c_int;
    }
    *_next = next;
    return j;
}
unsafe extern "C" fn branch_last(
    mut T: *mut glp_tree,
    mut _next: *mut libc::c_int,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    let mut beta: libc::c_double = 0.;
    j = (*T).n;
    while j >= 1 as libc::c_int {
        if *((*T).non_int).offset(j as isize) != 0 {
            break;
        }
        j -= 1;
        j;
    }
    (1 as libc::c_int <= j && j <= (*T).n
        || {
            glp_assert_(
                b"1 <= j && j <= T->n\0" as *const u8 as *const libc::c_char,
                b"draft/glpios09.c\0" as *const u8 as *const libc::c_char,
                120 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    beta = glp_get_col_prim((*T).mip, j);
    if beta - floor(beta) < ceil(beta) - beta {
        next = 1 as libc::c_int;
    } else {
        next = 2 as libc::c_int;
    }
    *_next = next;
    return j;
}
unsafe extern "C" fn branch_mostf(
    mut T: *mut glp_tree,
    mut _next: *mut libc::c_int,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    let mut beta: libc::c_double = 0.;
    let mut most: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    jj = 0 as libc::c_int;
    most = 1.7976931348623157e+308f64;
    j = 1 as libc::c_int;
    while j <= (*T).n {
        if *((*T).non_int).offset(j as isize) != 0 {
            beta = glp_get_col_prim((*T).mip, j);
            temp = floor(beta) + 0.5f64;
            if most > fabs(beta - temp) {
                jj = j;
                most = fabs(beta - temp);
                if beta < temp {
                    next = 1 as libc::c_int;
                } else {
                    next = 2 as libc::c_int;
                }
            }
        }
        j += 1;
        j;
    }
    *_next = next;
    return jj;
}
unsafe extern "C" fn branch_drtom(
    mut T: *mut glp_tree,
    mut _next: *mut libc::c_int,
) -> libc::c_int {
    let mut mip: *mut glp_prob = (*T).mip;
    let mut m: libc::c_int = (*mip).m;
    let mut n: libc::c_int = (*mip).n;
    let mut non_int: *mut libc::c_uchar = (*T).non_int;
    let mut j: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    let mut kase: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut stat: libc::c_int = 0;
    let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut x: libc::c_double = 0.;
    let mut dk: libc::c_double = 0.;
    let mut alfa: libc::c_double = 0.;
    let mut delta_j: libc::c_double = 0.;
    let mut delta_k: libc::c_double = 0.;
    let mut delta_z: libc::c_double = 0.;
    let mut dz_dn: libc::c_double = 0.;
    let mut dz_up: libc::c_double = 0.;
    let mut dd_dn: libc::c_double = 0.;
    let mut dd_up: libc::c_double = 0.;
    let mut degrad: libc::c_double = 0.;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    (glp_get_status(mip) == 5 as libc::c_int
        || {
            glp_assert_(
                b"glp_get_status(mip) == GLP_OPT\0" as *const u8 as *const libc::c_char,
                b"draft/glpios09.c\0" as *const u8 as *const libc::c_char,
                200 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ind = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    val = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    jj = 0 as libc::c_int;
    degrad = -1.0f64;
    j = 1 as libc::c_int;
    while j <= n {
        if !(*non_int.offset(j as isize) == 0) {
            x = glp_get_col_prim(mip, j);
            len = glp_eval_tab_row(mip, m + j, ind, val);
            kase = -(1 as libc::c_int);
            while kase <= 1 as libc::c_int {
                k = glp_dual_rtest(
                    mip,
                    len,
                    ind as *const libc::c_int,
                    val as *const libc::c_double,
                    kase,
                    1e-9f64,
                );
                if k != 0 as libc::c_int {
                    k = *ind.offset(k as isize);
                }
                if k == 0 as libc::c_int {
                    delta_z = if (*(*T).mip).dir == 1 as libc::c_int {
                        1.7976931348623157e+308f64
                    } else {
                        -1.7976931348623157e+308f64
                    };
                } else {
                    t = 1 as libc::c_int;
                    while t <= len {
                        if *ind.offset(t as isize) == k {
                            break;
                        }
                        t += 1;
                        t;
                    }
                    (1 as libc::c_int <= t && t <= len
                        || {
                            glp_assert_(
                                b"1 <= t && t <= len\0" as *const u8 as *const libc::c_char,
                                b"draft/glpios09.c\0" as *const u8 as *const libc::c_char,
                                258 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    alfa = *val.offset(t as isize);
                    delta_j = (if kase < 0 as libc::c_int { floor(x) } else { ceil(x) })
                        - x;
                    delta_k = delta_j / alfa;
                    if k > m && glp_get_col_kind(mip, k - m) != 1 as libc::c_int {
                        if fabs(delta_k - floor(delta_k + 0.5f64)) > 1e-3f64 {
                            if delta_k > 0.0f64 {
                                delta_k = ceil(delta_k);
                            } else {
                                delta_k = floor(delta_k);
                            }
                        }
                    }
                    if k <= m {
                        stat = glp_get_row_stat(mip, k);
                        dk = glp_get_row_dual(mip, k);
                    } else {
                        stat = glp_get_col_stat(mip, k - m);
                        dk = glp_get_col_dual(mip, k - m);
                    }
                    match (*(*T).mip).dir {
                        1 => {
                            if stat == 2 as libc::c_int && dk < 0.0f64
                                || stat == 3 as libc::c_int && dk > 0.0f64
                                || stat == 4 as libc::c_int
                            {
                                dk = 0.0f64;
                            }
                        }
                        2 => {
                            if stat == 2 as libc::c_int && dk > 0.0f64
                                || stat == 3 as libc::c_int && dk < 0.0f64
                                || stat == 4 as libc::c_int
                            {
                                dk = 0.0f64;
                            }
                        }
                        _ => {
                            (T != T
                                || {
                                    glp_assert_(
                                        b"T != T\0" as *const u8 as *const libc::c_char,
                                        b"draft/glpios09.c\0" as *const u8 as *const libc::c_char,
                                        306 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                        }
                    }
                    delta_z = dk * delta_k;
                }
                match (*(*T).mip).dir {
                    1 => {
                        (delta_z >= 0.0f64
                            || {
                                glp_assert_(
                                    b"delta_z >= 0.0\0" as *const u8 as *const libc::c_char,
                                    b"draft/glpios09.c\0" as *const u8 as *const libc::c_char,
                                    318 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                    }
                    2 => {
                        (delta_z <= 0.0f64
                            || {
                                glp_assert_(
                                    b"delta_z <= 0.0\0" as *const u8 as *const libc::c_char,
                                    b"draft/glpios09.c\0" as *const u8 as *const libc::c_char,
                                    319 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                    }
                    _ => {
                        (T != T
                            || {
                                glp_assert_(
                                    b"T != T\0" as *const u8 as *const libc::c_char,
                                    b"draft/glpios09.c\0" as *const u8 as *const libc::c_char,
                                    320 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                    }
                }
                if kase < 0 as libc::c_int {
                    dz_dn = delta_z;
                } else {
                    dz_up = delta_z;
                }
                kase += 2 as libc::c_int;
            }
            if degrad < fabs(dz_dn) || degrad < fabs(dz_up) {
                jj = j;
                if fabs(dz_dn) < fabs(dz_up) {
                    next = 1 as libc::c_int;
                    degrad = fabs(dz_up);
                } else {
                    next = 2 as libc::c_int;
                    degrad = fabs(dz_dn);
                }
                dd_dn = dz_dn;
                dd_up = dz_up;
                if degrad == 1.7976931348623157e+308f64 {
                    break;
                }
            }
        }
        j += 1;
        j;
    }
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
    (1 as libc::c_int <= jj && jj <= n
        || {
            glp_assert_(
                b"1 <= jj && jj <= n\0" as *const u8 as *const libc::c_char,
                b"draft/glpios09.c\0" as *const u8 as *const libc::c_char,
                361 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if degrad < 1e-6f64 * (1.0f64 + 0.001f64 * fabs((*mip).obj_val)) {
        jj = branch_mostf(T, &mut next);
    } else if (*(*T).parm).msg_lev >= 4 as libc::c_int {
        glp_printf(
            b"branch_drtom: column %d chosen to branch on\n\0" as *const u8
                as *const libc::c_char,
            jj,
        );
        if fabs(dd_dn) == 1.7976931348623157e+308f64 {
            glp_printf(
                b"branch_drtom: down-branch is infeasible\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            glp_printf(
                b"branch_drtom: down-branch bound is %.9e\n\0" as *const u8
                    as *const libc::c_char,
                glp_get_obj_val(mip) + dd_dn,
            );
        }
        if fabs(dd_up) == 1.7976931348623157e+308f64 {
            glp_printf(
                b"branch_drtom: up-branch   is infeasible\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            glp_printf(
                b"branch_drtom: up-branch   bound is %.9e\n\0" as *const u8
                    as *const libc::c_char,
                glp_get_obj_val(mip) + dd_up,
            );
        }
    }
    *_next = next;
    return jj;
}
pub unsafe extern "C" fn _glp_ios_pcost_init(
    mut tree: *mut glp_tree,
) -> *mut libc::c_void {
    let mut csa: *mut csa = 0 as *mut csa;
    let mut n: libc::c_int = (*tree).n;
    let mut j: libc::c_int = 0;
    csa = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<csa>() as libc::c_ulong as libc::c_int,
    ) as *mut csa;
    (*csa)
        .dn_cnt = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*csa)
        .dn_sum = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*csa)
        .up_cnt = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*csa)
        .up_sum = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    j = 1 as libc::c_int;
    while j <= n {
        let ref mut fresh0 = *((*csa).up_cnt).offset(j as isize);
        *fresh0 = 0 as libc::c_int;
        *((*csa).dn_cnt).offset(j as isize) = *fresh0;
        let ref mut fresh1 = *((*csa).up_sum).offset(j as isize);
        *fresh1 = 0.0f64;
        *((*csa).dn_sum).offset(j as isize) = *fresh1;
        j += 1;
        j;
    }
    return csa as *mut libc::c_void;
}
unsafe extern "C" fn eval_degrad(
    mut P: *mut glp_prob,
    mut j: libc::c_int,
    mut bnd: libc::c_double,
) -> libc::c_double {
    let mut lp: *mut glp_prob = 0 as *mut glp_prob;
    let mut parm: glp_smcp = glp_smcp {
        msg_lev: 0,
        meth: 0,
        pricing: 0,
        r_test: 0,
        tol_bnd: 0.,
        tol_dj: 0.,
        tol_piv: 0.,
        obj_ll: 0.,
        obj_ul: 0.,
        it_lim: 0,
        tm_lim: 0,
        out_frq: 0,
        out_dly: 0,
        presolve: 0,
        excl: 0,
        shift: 0,
        aorn: 0,
        foo_bar: [0.; 33],
    };
    let mut ret: libc::c_int = 0;
    let mut degrad: libc::c_double = 0.;
    (glp_get_status(P) == 5 as libc::c_int
        || {
            glp_assert_(
                b"glp_get_status(P) == GLP_OPT\0" as *const u8 as *const libc::c_char,
                b"draft/glpios09.c\0" as *const u8 as *const libc::c_char,
                432 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    lp = glp_create_prob();
    glp_copy_prob(lp, P, 0 as libc::c_int);
    glp_set_col_bnds(lp, j, 5 as libc::c_int, bnd, bnd);
    glp_init_smcp(&mut parm);
    parm.msg_lev = 0 as libc::c_int;
    parm.meth = 3 as libc::c_int;
    parm.it_lim = 30 as libc::c_int;
    parm.out_dly = 1000 as libc::c_int;
    parm.meth = 3 as libc::c_int;
    ret = glp_simplex(lp, &mut parm);
    if ret == 0 as libc::c_int || ret == 0x8 as libc::c_int {
        if glp_get_prim_stat(lp) == 4 as libc::c_int {
            degrad = 1.7976931348623157e+308f64;
        } else if glp_get_dual_stat(lp) == 2 as libc::c_int {
            if (*P).dir == 1 as libc::c_int {
                degrad = (*lp).obj_val - (*P).obj_val;
            } else if (*P).dir == 2 as libc::c_int {
                degrad = (*P).obj_val - (*lp).obj_val;
            } else {
                (P != P
                    || {
                        glp_assert_(
                            b"P != P\0" as *const u8 as *const libc::c_char,
                            b"draft/glpios09.c\0" as *const u8 as *const libc::c_char,
                            459 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            if degrad < 1e-6f64 * (1.0f64 + 0.001f64 * fabs((*P).obj_val)) {
                degrad = 0.0f64;
            }
        } else {
            degrad = 0.0f64;
        }
    } else {
        degrad = 0.0f64;
    }
    glp_delete_prob(lp);
    return degrad;
}
pub unsafe extern "C" fn _glp_ios_pcost_update(mut tree: *mut glp_tree) {
    let mut j: libc::c_int = 0;
    let mut dx: libc::c_double = 0.;
    let mut dz: libc::c_double = 0.;
    let mut psi: libc::c_double = 0.;
    let mut csa: *mut csa = (*tree).pcost as *mut csa;
    (!csa.is_null()
        || {
            glp_assert_(
                b"csa != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios09.c\0" as *const u8 as *const libc::c_char,
                491 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!((*tree).curr).is_null()
        || {
            glp_assert_(
                b"tree->curr != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios09.c\0" as *const u8 as *const libc::c_char,
                492 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if !((*(*tree).curr).up).is_null() {
        j = (*(*(*tree).curr).up).br_var;
        (1 as libc::c_int <= j && j <= (*tree).n
            || {
                glp_assert_(
                    b"1 <= j && j <= tree->n\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios09.c\0" as *const u8 as *const libc::c_char,
                    498 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        dx = (**((*(*tree).mip).col).offset(j as isize)).prim
            - (*(*(*tree).curr).up).br_val;
        (dx != 0.0f64
            || {
                glp_assert_(
                    b"dx != 0.0\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios09.c\0" as *const u8 as *const libc::c_char,
                    505 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        dz = (*(*tree).mip).obj_val - (*(*(*tree).curr).up).lp_obj;
        psi = fabs(dz / dx);
        if dx < 0.0f64 {
            let ref mut fresh2 = *((*csa).dn_cnt).offset(j as isize);
            *fresh2 += 1;
            *fresh2;
            *((*csa).dn_sum).offset(j as isize) += psi;
        } else {
            let ref mut fresh3 = *((*csa).up_cnt).offset(j as isize);
            *fresh3 += 1;
            *fresh3;
            *((*csa).up_sum).offset(j as isize) += psi;
        }
    }
}
pub unsafe extern "C" fn _glp_ios_pcost_free(mut tree: *mut glp_tree) {
    let mut csa: *mut csa = (*tree).pcost as *mut csa;
    (!csa.is_null()
        || {
            glp_assert_(
                b"csa != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios09.c\0" as *const u8 as *const libc::c_char,
                528 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    glp_free((*csa).dn_cnt as *mut libc::c_void);
    glp_free((*csa).dn_sum as *mut libc::c_void);
    glp_free((*csa).up_cnt as *mut libc::c_void);
    glp_free((*csa).up_sum as *mut libc::c_void);
    glp_free(csa as *mut libc::c_void);
    (*tree).pcost = 0 as *mut libc::c_void;
}
unsafe extern "C" fn eval_psi(
    mut T: *mut glp_tree,
    mut j: libc::c_int,
    mut brnch: libc::c_int,
) -> libc::c_double {
    let mut current_block: u64;
    let mut csa: *mut csa = (*T).pcost as *mut csa;
    let mut beta: libc::c_double = 0.;
    let mut degrad: libc::c_double = 0.;
    let mut psi: libc::c_double = 0.;
    (!csa.is_null()
        || {
            glp_assert_(
                b"csa != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios09.c\0" as *const u8 as *const libc::c_char,
                543 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= j && j <= (*T).n
        || {
            glp_assert_(
                b"1 <= j && j <= T->n\0" as *const u8 as *const libc::c_char,
                b"draft/glpios09.c\0" as *const u8 as *const libc::c_char,
                544 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if brnch == 1 as libc::c_int {
        if *((*csa).dn_cnt).offset(j as isize) == 0 as libc::c_int {
            beta = (**((*(*T).mip).col).offset(j as isize)).prim;
            degrad = eval_degrad((*T).mip, j, floor(beta));
            if degrad == 1.7976931348623157e+308f64 {
                psi = 1.7976931348623157e+308f64;
                current_block = 11513243060649556525;
            } else {
                *((*csa).dn_cnt).offset(j as isize) = 1 as libc::c_int;
                *((*csa).dn_sum).offset(j as isize) = degrad / (beta - floor(beta));
                current_block = 17216689946888361452;
            }
        } else {
            current_block = 17216689946888361452;
        }
        match current_block {
            11513243060649556525 => {}
            _ => {
                psi = *((*csa).dn_sum).offset(j as isize)
                    / *((*csa).dn_cnt).offset(j as isize) as libc::c_double;
            }
        }
    } else if brnch == 2 as libc::c_int {
        if *((*csa).up_cnt).offset(j as isize) == 0 as libc::c_int {
            beta = (**((*(*T).mip).col).offset(j as isize)).prim;
            degrad = eval_degrad((*T).mip, j, ceil(beta));
            if degrad == 1.7976931348623157e+308f64 {
                psi = 1.7976931348623157e+308f64;
                current_block = 11513243060649556525;
            } else {
                *((*csa).up_cnt).offset(j as isize) = 1 as libc::c_int;
                *((*csa).up_sum).offset(j as isize) = degrad / (ceil(beta) - beta);
                current_block = 1054647088692577877;
            }
        } else {
            current_block = 1054647088692577877;
        }
        match current_block {
            11513243060649556525 => {}
            _ => {
                psi = *((*csa).up_sum).offset(j as isize)
                    / *((*csa).up_cnt).offset(j as isize) as libc::c_double;
            }
        }
    } else {
        (brnch != brnch
            || {
                glp_assert_(
                    b"brnch != brnch\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios09.c\0" as *const u8 as *const libc::c_char,
                    576 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    return psi;
}
unsafe extern "C" fn progress(mut T: *mut glp_tree) {
    let mut csa: *mut csa = (*T).pcost as *mut csa;
    let mut j: libc::c_int = 0;
    let mut nv: libc::c_int = 0 as libc::c_int;
    let mut ni: libc::c_int = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while j <= (*T).n {
        if glp_ios_can_branch(T, j) != 0 {
            nv += 1;
            nv;
            if *((*csa).dn_cnt).offset(j as isize) > 0 as libc::c_int
                && *((*csa).up_cnt).offset(j as isize) > 0 as libc::c_int
            {
                ni += 1;
                ni;
            }
        }
        j += 1;
        j;
    }
    glp_printf(
        b"Pseudocosts initialized for %d of %d variables\n\0" as *const u8
            as *const libc::c_char,
        ni,
        nv,
    );
}
pub unsafe extern "C" fn _glp_ios_pcost_branch(
    mut T: *mut glp_tree,
    mut _next: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut t: libc::c_double = glp_time();
    let mut j: libc::c_int = 0;
    let mut jjj: libc::c_int = 0;
    let mut sel: libc::c_int = 0;
    let mut beta: libc::c_double = 0.;
    let mut psi: libc::c_double = 0.;
    let mut d1: libc::c_double = 0.;
    let mut d2: libc::c_double = 0.;
    let mut d: libc::c_double = 0.;
    let mut dmax: libc::c_double = 0.;
    if ((*T).pcost).is_null() {
        (*T).pcost = _glp_ios_pcost_init(T);
    }
    jjj = 0 as libc::c_int;
    dmax = -1.0f64;
    j = 1 as libc::c_int;
    loop {
        if !(j <= (*T).n) {
            current_block = 1109700713171191020;
            break;
        }
        if !(glp_ios_can_branch(T, j) == 0) {
            beta = (**((*(*T).mip).col).offset(j as isize)).prim;
            psi = eval_psi(T, j, 1 as libc::c_int);
            if psi == 1.7976931348623157e+308f64 {
                jjj = j;
                sel = 1 as libc::c_int;
                current_block = 13258131111428678642;
                break;
            } else {
                d1 = psi * (beta - floor(beta));
                psi = eval_psi(T, j, 2 as libc::c_int);
                if psi == 1.7976931348623157e+308f64 {
                    jjj = j;
                    sel = 2 as libc::c_int;
                    current_block = 13258131111428678642;
                    break;
                } else {
                    d2 = psi * (ceil(beta) - beta);
                    d = if d1 > d2 { d1 } else { d2 };
                    if dmax < d {
                        dmax = d;
                        jjj = j;
                        sel = if d1 <= d2 { 1 as libc::c_int } else { 2 as libc::c_int };
                    }
                    if (*(*T).parm).msg_lev >= 1 as libc::c_int {
                        if glp_difftime(glp_time(), t) >= 10.0f64 {
                            progress(T);
                            t = glp_time();
                        }
                    }
                }
            }
        }
        j += 1;
        j;
    }
    match current_block {
        1109700713171191020 => {
            if dmax == 0.0f64 {
                jjj = branch_mostf(T, &mut sel);
            }
        }
        _ => {}
    }
    *_next = sel;
    return jjj;
}
