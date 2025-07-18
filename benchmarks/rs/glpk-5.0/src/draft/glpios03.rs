use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_cfg;
    pub type glp_mir;
    pub type glp_cov;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn glp_difftime(t1: libc::c_double, t0: libc::c_double) -> libc::c_double;
    fn glp_time() -> libc::c_double;
    fn glp_mem_usage(
        count: *mut libc::c_int,
        cpeak: *mut libc::c_int,
        total: *mut size_t,
        tpeak: *mut size_t,
    );
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_create_prob() -> *mut glp_prob;
    fn glp_add_cols(P: *mut glp_prob, ncs: libc::c_int) -> libc::c_int;
    fn glp_set_col_bnds(
        P: *mut glp_prob,
        j: libc::c_int,
        type_0: libc::c_int,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_del_rows(P: *mut glp_prob, nrs: libc::c_int, num: *const libc::c_int);
    fn glp_delete_prob(P: *mut glp_prob);
    fn glp_get_mat_row(
        P: *mut glp_prob,
        i: libc::c_int,
        ind: *mut libc::c_int,
        val: *mut libc::c_double,
    ) -> libc::c_int;
    fn glp_factorize(P: *mut glp_prob) -> libc::c_int;
    fn glp_ios_add_row(
        T: *mut glp_tree,
        name: *const libc::c_char,
        klass: libc::c_int,
        flags: libc::c_int,
        len: libc::c_int,
        ind: *const libc::c_int,
        val: *const libc::c_double,
        type_0: libc::c_int,
        rhs: libc::c_double,
    ) -> libc::c_int;
    fn glp_ios_heur_sol(T: *mut glp_tree, x: *const libc::c_double) -> libc::c_int;
    fn glp_gmi_gen(
        P: *mut glp_prob,
        pool: *mut glp_prob,
        max_cuts: libc::c_int,
    ) -> libc::c_int;
    fn glp_cov_init(P: *mut glp_prob) -> *mut glp_cov;
    fn glp_cov_gen1(P: *mut glp_prob, cov: *mut glp_cov, pool: *mut glp_prob);
    fn glp_cov_free(cov: *mut glp_cov);
    fn glp_mir_init(P: *mut glp_prob) -> *mut glp_mir;
    fn glp_mir_gen(
        P: *mut glp_prob,
        mir: *mut glp_mir,
        pool: *mut glp_prob,
    ) -> libc::c_int;
    fn glp_mir_free(mir: *mut glp_mir);
    fn glp_cfg_init(P: *mut glp_prob) -> *mut glp_cfg;
    fn glp_cfg_free(G: *mut glp_cfg);
    fn glp_clq_cut(
        P: *mut glp_prob,
        G: *mut glp_cfg,
        ind: *mut libc::c_int,
        val: *mut libc::c_double,
    ) -> libc::c_int;
    fn _glp_ios_revive_node(tree: *mut glp_tree, p: libc::c_int);
    fn _glp_ios_freeze_node(tree: *mut glp_tree);
    fn _glp_ios_clone_node(
        tree: *mut glp_tree,
        p: libc::c_int,
        nnn: libc::c_int,
        ref_0: *mut libc::c_int,
    );
    fn _glp_ios_delete_node(tree: *mut glp_tree, p: libc::c_int);
    fn _glp_ios_eval_degrad(
        tree: *mut glp_tree,
        j: libc::c_int,
        dn: *mut libc::c_double,
        up: *mut libc::c_double,
    );
    fn _glp_ios_round_bound(
        tree: *mut glp_tree,
        bound: libc::c_double,
    ) -> libc::c_double;
    fn _glp_ios_is_hopeful(tree: *mut glp_tree, bound: libc::c_double) -> libc::c_int;
    fn _glp_ios_best_node(tree: *mut glp_tree) -> libc::c_int;
    fn _glp_ios_relative_gap(tree: *mut glp_tree) -> libc::c_double;
    fn _glp_ios_solve_node(tree: *mut glp_tree) -> libc::c_int;
    fn _glp_dmp_in_use(pool: *mut DMP) -> size_t;
    fn _glp_dmp_get_atom(pool: *mut DMP, size: libc::c_int) -> *mut libc::c_void;
    fn _glp_ios_proxy_heur(T: *mut glp_tree);
    fn _glp_ios_feas_pump(T: *mut glp_tree);
    fn _glp_ios_process_cuts(T: *mut glp_tree);
    fn _glp_ios_choose_node(T: *mut glp_tree) -> libc::c_int;
    fn _glp_ios_clear_pool(tree: *mut glp_tree, pool: *mut IOSPOOL);
    fn _glp_ios_pcost_update(tree: *mut glp_tree);
    fn _glp_ios_process_sol(T: *mut glp_tree);
    fn _glp_ios_preprocess_node(
        tree: *mut glp_tree,
        max_pass: libc::c_int,
    ) -> libc::c_int;
    fn _glp_ios_choose_var(T: *mut glp_tree, next: *mut libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
unsafe extern "C" fn show_progress(mut T: *mut glp_tree, mut bingo: libc::c_int) {
    let mut p: libc::c_int = 0;
    let mut temp: libc::c_double = 0.;
    let mut best_mip: [libc::c_char; 50] = [0; 50];
    let mut best_bound: [libc::c_char; 50] = [0; 50];
    let mut rho: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rel_gap: [libc::c_char; 50] = [0; 50];
    if (*(*T).mip).mip_stat == 2 as libc::c_int {
        sprintf(
            best_mip.as_mut_ptr(),
            b"%17.9e\0" as *const u8 as *const libc::c_char,
            (*(*T).mip).mip_obj,
        );
    } else {
        sprintf(
            best_mip.as_mut_ptr(),
            b"%17s\0" as *const u8 as *const libc::c_char,
            b"not found yet\0" as *const u8 as *const libc::c_char,
        );
    }
    p = _glp_ios_best_node(T);
    if p == 0 as libc::c_int {
        sprintf(
            best_bound.as_mut_ptr(),
            b"%17s\0" as *const u8 as *const libc::c_char,
            b"tree is empty\0" as *const u8 as *const libc::c_char,
        );
    } else {
        temp = (*(*((*T).slot).offset(p as isize)).node).bound;
        if temp == -1.7976931348623157e+308f64 {
            sprintf(
                best_bound.as_mut_ptr(),
                b"%17s\0" as *const u8 as *const libc::c_char,
                b"-inf\0" as *const u8 as *const libc::c_char,
            );
        } else if temp == 1.7976931348623157e+308f64 {
            sprintf(
                best_bound.as_mut_ptr(),
                b"%17s\0" as *const u8 as *const libc::c_char,
                b"+inf\0" as *const u8 as *const libc::c_char,
            );
        } else {
            if fabs(temp) < 1e-9f64 {
                temp = 0 as libc::c_int as libc::c_double;
            }
            sprintf(
                best_bound.as_mut_ptr(),
                b"%17.9e\0" as *const u8 as *const libc::c_char,
                temp,
            );
        }
    }
    if (*(*T).mip).dir == 1 as libc::c_int {
        rho = b">=\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else if (*(*T).mip).dir == 2 as libc::c_int {
        rho = b"<=\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else {
        (T != T
            || {
                glp_assert_(
                    b"T != T\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                    83 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    temp = _glp_ios_relative_gap(T);
    if temp == 0.0f64 {
        sprintf(rel_gap.as_mut_ptr(), b"  0.0%%\0" as *const u8 as *const libc::c_char);
    } else if temp < 0.001f64 {
        sprintf(rel_gap.as_mut_ptr(), b"< 0.1%%\0" as *const u8 as *const libc::c_char);
    } else if temp <= 9.999f64 {
        sprintf(
            rel_gap.as_mut_ptr(),
            b"%5.1f%%\0" as *const u8 as *const libc::c_char,
            100.0f64 * temp,
        );
    } else {
        sprintf(
            rel_gap.as_mut_ptr(),
            b"%6s\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    glp_printf(
        b"+%6d: %s %s %s %s %s (%d; %d)\n\0" as *const u8 as *const libc::c_char,
        (*(*T).mip).it_cnt,
        if bingo != 0 {
            b">>>>>\0" as *const u8 as *const libc::c_char
        } else {
            b"mip =\0" as *const u8 as *const libc::c_char
        },
        best_mip.as_mut_ptr(),
        rho,
        best_bound.as_mut_ptr(),
        rel_gap.as_mut_ptr(),
        (*T).a_cnt,
        (*T).t_cnt - (*T).n_cnt,
    );
    (*T).tm_lag = glp_time();
}
unsafe extern "C" fn is_branch_hopeful(
    mut T: *mut glp_tree,
    mut p: libc::c_int,
) -> libc::c_int {
    (1 as libc::c_int <= p && p <= (*T).nslots
        || {
            glp_assert_(
                b"1 <= p && p <= T->nslots\0" as *const u8 as *const libc::c_char,
                b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                117 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!((*((*T).slot).offset(p as isize)).node).is_null()
        || {
            glp_assert_(
                b"T->slot[p].node != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                118 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return _glp_ios_is_hopeful(T, (*(*((*T).slot).offset(p as isize)).node).bound);
}
unsafe extern "C" fn check_integrality(mut T: *mut glp_tree) {
    let mut mip: *mut glp_prob = (*T).mip;
    let mut j: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut ii_cnt: libc::c_int = 0 as libc::c_int;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut temp1: libc::c_double = 0.;
    let mut temp2: libc::c_double = 0.;
    let mut ii_sum: libc::c_double = 0.0f64;
    let mut current_block_15: u64;
    j = 1 as libc::c_int;
    while j <= (*mip).n {
        let mut col: *mut GLPCOL = *((*mip).col).offset(j as isize);
        *((*T).non_int).offset(j as isize) = 0 as libc::c_int as libc::c_uchar;
        if !((*col).kind != 2 as libc::c_int) {
            if !((*col).stat != 1 as libc::c_int) {
                type_0 = (*col).type_0;
                lb = (*col).lb;
                ub = (*col).ub;
                x = (*col).prim;
                if type_0 == 2 as libc::c_int || type_0 == 4 as libc::c_int
                    || type_0 == 5 as libc::c_int
                {
                    temp1 = lb - (*(*T).parm).tol_int;
                    temp2 = lb + (*(*T).parm).tol_int;
                    if temp1 <= x && x <= temp2 {
                        current_block_15 = 6239978542346980191;
                    } else if x < lb {
                        current_block_15 = 6239978542346980191;
                    } else {
                        current_block_15 = 2968425633554183086;
                    }
                } else {
                    current_block_15 = 2968425633554183086;
                }
                match current_block_15 {
                    6239978542346980191 => {}
                    _ => {
                        if type_0 == 3 as libc::c_int || type_0 == 4 as libc::c_int
                            || type_0 == 5 as libc::c_int
                        {
                            temp1 = ub - (*(*T).parm).tol_int;
                            temp2 = ub + (*(*T).parm).tol_int;
                            if temp1 <= x && x <= temp2 {
                                current_block_15 = 6239978542346980191;
                            } else if x > ub {
                                current_block_15 = 6239978542346980191;
                            } else {
                                current_block_15 = 8831408221741692167;
                            }
                        } else {
                            current_block_15 = 8831408221741692167;
                        }
                        match current_block_15 {
                            6239978542346980191 => {}
                            _ => {
                                temp1 = floor(x + 0.5f64) - (*(*T).parm).tol_int;
                                temp2 = floor(x + 0.5f64) + (*(*T).parm).tol_int;
                                if !(temp1 <= x && x <= temp2) {
                                    *((*T).non_int)
                                        .offset(j as isize) = 1 as libc::c_int as libc::c_uchar;
                                    ii_cnt += 1;
                                    ii_cnt;
                                    temp1 = x - floor(x);
                                    temp2 = ceil(x) - x;
                                    (temp1 > 0.0f64 && temp2 > 0.0f64
                                        || {
                                            glp_assert_(
                                                b"temp1 > 0.0 && temp2 > 0.0\0" as *const u8
                                                    as *const libc::c_char,
                                                b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                                206 as libc::c_int,
                                            );
                                            1 as libc::c_int != 0
                                        }) as libc::c_int;
                                    ii_sum += if temp1 <= temp2 { temp1 } else { temp2 };
                                }
                            }
                        }
                    }
                }
            }
        }
        j += 1;
        j;
    }
    (!((*T).curr).is_null()
        || {
            glp_assert_(
                b"T->curr != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                210 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*(*T).curr).ii_cnt = ii_cnt;
    (*(*T).curr).ii_sum = ii_sum;
    if (*(*T).parm).msg_lev >= 4 as libc::c_int {
        if ii_cnt == 0 as libc::c_int {
            glp_printf(
                b"There are no fractional columns\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else if ii_cnt == 1 as libc::c_int {
            glp_printf(
                b"There is one fractional column, integer infeasibility is %.3e\n\0"
                    as *const u8 as *const libc::c_char,
                ii_sum,
            );
        } else {
            glp_printf(
                b"There are %d fractional columns, integer infeasibility is %.3e\n\0"
                    as *const u8 as *const libc::c_char,
                ii_cnt,
                ii_sum,
            );
        }
    }
}
unsafe extern "C" fn record_solution(mut T: *mut glp_tree) {
    let mut mip: *mut glp_prob = (*T).mip;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    (*mip).mip_stat = 2 as libc::c_int;
    (*mip).mip_obj = (*mip).obj_val;
    i = 1 as libc::c_int;
    while i <= (*mip).m {
        let mut row: *mut GLPROW = *((*mip).row).offset(i as isize);
        (*row).mipx = (*row).prim;
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= (*mip).n {
        let mut col: *mut GLPCOL = *((*mip).col).offset(j as isize);
        if (*col).kind == 1 as libc::c_int {
            (*col).mipx = (*col).prim;
        } else if (*col).kind == 2 as libc::c_int {
            (*col).mipx = floor((*col).prim + 0.5f64);
        } else {
            (col != col
                || {
                    glp_assert_(
                        b"col != col\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                        252 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        j += 1;
        j;
    }
    (*T).sol_cnt += 1;
    (*T).sol_cnt;
}
unsafe extern "C" fn fix_by_red_cost(mut T: *mut glp_tree) {
    let mut mip: *mut glp_prob = (*T).mip;
    let mut j: libc::c_int = 0;
    let mut stat: libc::c_int = 0;
    let mut fixed: libc::c_int = 0 as libc::c_int;
    let mut obj: libc::c_double = 0.;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    let mut dj: libc::c_double = 0.;
    ((*(*T).mip).mip_stat == 2 as libc::c_int
        || {
            glp_assert_(
                b"T->mip->mip_stat == GLP_FEAS\0" as *const u8 as *const libc::c_char,
                b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                271 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*mip).pbs_stat == 2 as libc::c_int && (*mip).dbs_stat == 2 as libc::c_int
        || {
            glp_assert_(
                b"mip->pbs_stat == GLP_FEAS && mip->dbs_stat == GLP_FEAS\0" as *const u8
                    as *const libc::c_char,
                b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                273 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    obj = (*mip).obj_val;
    j = 1 as libc::c_int;
    while j <= (*mip).n {
        let mut col: *mut GLPCOL = *((*mip).col).offset(j as isize);
        if !((*col).kind != 2 as libc::c_int) {
            lb = (*col).lb;
            ub = (*col).ub;
            stat = (*col).stat;
            dj = (*col).dual;
            match (*mip).dir {
                1 => {
                    if stat == 2 as libc::c_int {
                        if dj < 0.0f64 {
                            dj = 0.0f64;
                        }
                        if obj + dj >= (*mip).mip_obj {
                            glp_set_col_bnds(mip, j, 5 as libc::c_int, lb, lb);
                            fixed += 1;
                            fixed;
                        }
                    } else if stat == 3 as libc::c_int {
                        if dj > 0.0f64 {
                            dj = 0.0f64;
                        }
                        if obj - dj >= (*mip).mip_obj {
                            glp_set_col_bnds(mip, j, 5 as libc::c_int, ub, ub);
                            fixed += 1;
                            fixed;
                        }
                    }
                }
                2 => {
                    if stat == 2 as libc::c_int {
                        if dj > 0.0f64 {
                            dj = 0.0f64;
                        }
                        if obj + dj <= (*mip).mip_obj {
                            glp_set_col_bnds(mip, j, 5 as libc::c_int, lb, lb);
                            fixed += 1;
                            fixed;
                        }
                    } else if stat == 3 as libc::c_int {
                        if dj < 0.0f64 {
                            dj = 0.0f64;
                        }
                        if obj - dj <= (*mip).mip_obj {
                            glp_set_col_bnds(mip, j, 5 as libc::c_int, ub, ub);
                            fixed += 1;
                            fixed;
                        }
                    }
                }
                _ => {
                    (T != T
                        || {
                            glp_assert_(
                                b"T != T\0" as *const u8 as *const libc::c_char,
                                b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                318 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
            }
        }
        j += 1;
        j;
    }
    if (*(*T).parm).msg_lev >= 4 as libc::c_int {
        if !(fixed == 0 as libc::c_int) {
            if fixed == 1 as libc::c_int {
                glp_printf(
                    b"One column has been fixed by reduced cost\n\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                glp_printf(
                    b"%d columns have been fixed by reduced costs\n\0" as *const u8
                        as *const libc::c_char,
                    fixed,
                );
            }
        }
    }
    ((*mip).pbs_stat == 2 as libc::c_int && (*mip).dbs_stat == 2 as libc::c_int
        || {
            glp_assert_(
                b"mip->pbs_stat == GLP_FEAS && mip->dbs_stat == GLP_FEAS\0" as *const u8
                    as *const libc::c_char,
                b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                332 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
}
unsafe extern "C" fn branch_on(
    mut T: *mut glp_tree,
    mut j: libc::c_int,
    mut next: libc::c_int,
) -> libc::c_int {
    let mut mip: *mut glp_prob = (*T).mip;
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    let mut m: libc::c_int = (*mip).m;
    let mut n: libc::c_int = (*mip).n;
    let mut type_0: libc::c_int = 0;
    let mut dn_type: libc::c_int = 0;
    let mut up_type: libc::c_int = 0;
    let mut dn_bad: libc::c_int = 0;
    let mut up_bad: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut clone: [libc::c_int; 3] = [0; 3];
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut new_ub: libc::c_double = 0.;
    let mut new_lb: libc::c_double = 0.;
    let mut dn_lp: libc::c_double = 0.;
    let mut up_lp: libc::c_double = 0.;
    let mut dn_bnd: libc::c_double = 0.;
    let mut up_bnd: libc::c_double = 0.;
    (1 as libc::c_int <= j && j <= n
        || {
            glp_assert_(
                b"1 <= j && j <= n\0" as *const u8 as *const libc::c_char,
                b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                371 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    type_0 = (**((*mip).col).offset(j as isize)).type_0;
    lb = (**((*mip).col).offset(j as isize)).lb;
    ub = (**((*mip).col).offset(j as isize)).ub;
    beta = (**((*mip).col).offset(j as isize)).prim;
    new_ub = floor(beta);
    new_lb = ceil(beta);
    match type_0 {
        1 => {
            dn_type = 3 as libc::c_int;
            up_type = 2 as libc::c_int;
        }
        2 => {
            (lb <= new_ub
                || {
                    glp_assert_(
                        b"lb <= new_ub\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                        385 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            dn_type = if lb == new_ub { 5 as libc::c_int } else { 4 as libc::c_int };
            (lb + 1.0f64 <= new_lb
                || {
                    glp_assert_(
                        b"lb + 1.0 <= new_lb\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                        387 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            up_type = 2 as libc::c_int;
        }
        3 => {
            (new_ub <= ub - 1.0f64
                || {
                    glp_assert_(
                        b"new_ub <= ub - 1.0\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                        391 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            dn_type = 3 as libc::c_int;
            (new_lb <= ub
                || {
                    glp_assert_(
                        b"new_lb <= ub\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                        393 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            up_type = if new_lb == ub { 5 as libc::c_int } else { 4 as libc::c_int };
        }
        4 => {
            (lb <= new_ub && new_ub <= ub - 1.0f64
                || {
                    glp_assert_(
                        b"lb <= new_ub && new_ub <= ub - 1.0\0" as *const u8
                            as *const libc::c_char,
                        b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                        397 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            dn_type = if lb == new_ub { 5 as libc::c_int } else { 4 as libc::c_int };
            (lb + 1.0f64 <= new_lb && new_lb <= ub
                || {
                    glp_assert_(
                        b"lb + 1.0 <= new_lb && new_lb <= ub\0" as *const u8
                            as *const libc::c_char,
                        b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                        399 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            up_type = if new_lb == ub { 5 as libc::c_int } else { 4 as libc::c_int };
        }
        _ => {
            (type_0 != type_0
                || {
                    glp_assert_(
                        b"type != type\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                        403 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    _glp_ios_eval_degrad(T, j, &mut dn_lp, &mut up_lp);
    dn_bnd = _glp_ios_round_bound(T, dn_lp);
    up_bnd = _glp_ios_round_bound(T, up_lp);
    dn_bad = (_glp_ios_is_hopeful(T, dn_bnd) == 0) as libc::c_int;
    up_bad = (_glp_ios_is_hopeful(T, up_bnd) == 0) as libc::c_int;
    if dn_bad != 0 && up_bad != 0 {
        if (*(*T).parm).msg_lev >= 4 as libc::c_int {
            glp_printf(
                b"Both down- and up-branches are hopeless\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        ret = 2 as libc::c_int;
    } else if up_bad != 0 {
        if (*(*T).parm).msg_lev >= 4 as libc::c_int {
            glp_printf(b"Up-branch is hopeless\n\0" as *const u8 as *const libc::c_char);
        }
        glp_set_col_bnds(mip, j, dn_type, lb, new_ub);
        (*(*T).curr).lp_obj = dn_lp;
        if (*mip).dir == 1 as libc::c_int {
            if (*(*T).curr).bound < dn_bnd {
                (*(*T).curr).bound = dn_bnd;
            }
        } else if (*mip).dir == 2 as libc::c_int {
            if (*(*T).curr).bound > dn_bnd {
                (*(*T).curr).bound = dn_bnd;
            }
        } else {
            (mip != mip
                || {
                    glp_assert_(
                        b"mip != mip\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                        433 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        ret = 1 as libc::c_int;
    } else if dn_bad != 0 {
        if (*(*T).parm).msg_lev >= 4 as libc::c_int {
            glp_printf(
                b"Down-branch is hopeless\n\0" as *const u8 as *const libc::c_char,
            );
        }
        glp_set_col_bnds(mip, j, up_type, new_lb, ub);
        (*(*T).curr).lp_obj = up_lp;
        if (*mip).dir == 1 as libc::c_int {
            if (*(*T).curr).bound < up_bnd {
                (*(*T).curr).bound = up_bnd;
            }
        } else if (*mip).dir == 2 as libc::c_int {
            if (*(*T).curr).bound > up_bnd {
                (*(*T).curr).bound = up_bnd;
            }
        } else {
            (mip != mip
                || {
                    glp_assert_(
                        b"mip != mip\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                        451 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        ret = 1 as libc::c_int;
    } else {
        if (*(*T).parm).msg_lev >= 4 as libc::c_int {
            glp_printf(
                b"Branching on column %d, primal value is %.9e\n\0" as *const u8
                    as *const libc::c_char,
                j,
                beta,
            );
        }
        (!((*T).curr).is_null()
            || {
                glp_assert_(
                    b"T->curr != NULL\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                    460 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        p = (*(*T).curr).p;
        (*(*T).curr).br_var = j;
        (*(*T).curr).br_val = beta;
        _glp_ios_freeze_node(T);
        _glp_ios_clone_node(T, p, 2 as libc::c_int, clone.as_mut_ptr());
        if (*(*T).parm).msg_lev >= 4 as libc::c_int {
            glp_printf(
                b"Node %d begins down branch, node %d begins up branch \n\0" as *const u8
                    as *const libc::c_char,
                clone[1 as libc::c_int as usize],
                clone[2 as libc::c_int as usize],
            );
        }
        node = (*((*T).slot).offset(clone[1 as libc::c_int as usize] as isize)).node;
        (!node.is_null()
            || {
                glp_assert_(
                    b"node != NULL\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                    474 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (!((*node).up).is_null()
            || {
                glp_assert_(
                    b"node->up != NULL\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                    475 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (((*node).b_ptr).is_null()
            || {
                glp_assert_(
                    b"node->b_ptr == NULL\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                    476 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*node)
            .b_ptr = _glp_dmp_get_atom(
            (*T).pool,
            ::std::mem::size_of::<IOSBND>() as libc::c_ulong as libc::c_int,
        ) as *mut IOSBND;
        (*(*node).b_ptr).k = m + j;
        (*(*node).b_ptr).type_0 = dn_type as libc::c_uchar;
        (*(*node).b_ptr).lb = lb;
        (*(*node).b_ptr).ub = new_ub;
        (*(*node).b_ptr).next = 0 as *mut IOSBND;
        (*node).lp_obj = dn_lp;
        if (*mip).dir == 1 as libc::c_int {
            if (*node).bound < dn_bnd {
                (*node).bound = dn_bnd;
            }
        } else if (*mip).dir == 2 as libc::c_int {
            if (*node).bound > dn_bnd {
                (*node).bound = dn_bnd;
            }
        } else {
            (mip != mip
                || {
                    glp_assert_(
                        b"mip != mip\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                        493 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        node = (*((*T).slot).offset(clone[2 as libc::c_int as usize] as isize)).node;
        (!node.is_null()
            || {
                glp_assert_(
                    b"node != NULL\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                    496 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (!((*node).up).is_null()
            || {
                glp_assert_(
                    b"node->up != NULL\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                    497 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (((*node).b_ptr).is_null()
            || {
                glp_assert_(
                    b"node->b_ptr == NULL\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                    498 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*node)
            .b_ptr = _glp_dmp_get_atom(
            (*T).pool,
            ::std::mem::size_of::<IOSBND>() as libc::c_ulong as libc::c_int,
        ) as *mut IOSBND;
        (*(*node).b_ptr).k = m + j;
        (*(*node).b_ptr).type_0 = up_type as libc::c_uchar;
        (*(*node).b_ptr).lb = new_lb;
        (*(*node).b_ptr).ub = ub;
        (*(*node).b_ptr).next = 0 as *mut IOSBND;
        (*node).lp_obj = up_lp;
        if (*mip).dir == 1 as libc::c_int {
            if (*node).bound < up_bnd {
                (*node).bound = up_bnd;
            }
        } else if (*mip).dir == 2 as libc::c_int {
            if (*node).bound > up_bnd {
                (*node).bound = up_bnd;
            }
        } else {
            (mip != mip
                || {
                    glp_assert_(
                        b"mip != mip\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                        515 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        ((*T).child == 0 as libc::c_int
            || {
                glp_assert_(
                    b"T->child == 0\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                    517 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if next == 0 as libc::c_int {
            (*T).child = 0 as libc::c_int;
        } else if next == 1 as libc::c_int {
            (*T).child = clone[1 as libc::c_int as usize];
        } else if next == 2 as libc::c_int {
            (*T).child = clone[2 as libc::c_int as usize];
        } else {
            (next != next
                || {
                    glp_assert_(
                        b"next != next\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                        525 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        ret = 0 as libc::c_int;
    }
    return ret;
}
unsafe extern "C" fn cleanup_the_tree(mut T: *mut glp_tree) {
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    let mut next_node: *mut IOSNPD = 0 as *mut IOSNPD;
    let mut count: libc::c_int = 0 as libc::c_int;
    ((*(*T).mip).mip_stat == 2 as libc::c_int
        || {
            glp_assert_(
                b"T->mip->mip_stat == GLP_FEAS\0" as *const u8 as *const libc::c_char,
                b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                543 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    node = (*T).head;
    while !node.is_null() {
        next_node = (*node).next;
        if is_branch_hopeful(T, (*node).p) == 0 {
            _glp_ios_delete_node(T, (*node).p);
            count += 1;
            count;
        }
        node = next_node;
    }
    if (*(*T).parm).msg_lev >= 4 as libc::c_int {
        if count == 1 as libc::c_int {
            glp_printf(
                b"One hopeless branch has been pruned\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else if count > 1 as libc::c_int {
            glp_printf(
                b"%d hopeless branches have been pruned\n\0" as *const u8
                    as *const libc::c_char,
                count,
            );
        }
    }
}
unsafe extern "C" fn round_heur(mut T: *mut glp_tree) -> libc::c_int {
    let mut current_block: u64;
    let mut P: *mut glp_prob = (*T).mip;
    let mut n: libc::c_int = (*P).n;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut x: *mut libc::c_double = 0 as *mut libc::c_double;
    x = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    j = 1 as libc::c_int;
    loop {
        if !(j <= n) {
            current_block = 17965632435239708295;
            break;
        }
        let mut col: *mut GLPCOL = *((*P).col).offset(j as isize);
        if (*col).kind == 2 as libc::c_int {
            *x.offset(j as isize) = floor((*col).prim + 0.5f64);
        } else if (*col).type_0 == 5 as libc::c_int {
            *x.offset(j as isize) = (*col).prim;
        } else {
            ret = 3 as libc::c_int;
            current_block = 610055490595703683;
            break;
        }
        j += 1;
        j;
    }
    match current_block {
        17965632435239708295 => {
            i = 1 as libc::c_int;
            loop {
                if !(i <= (*T).orig_m) {
                    current_block = 6669252993407410313;
                    break;
                }
                let mut type_0: libc::c_int = *((*T).orig_type).offset(i as isize)
                    as libc::c_int;
                let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
                let mut sum: libc::c_double = 0.;
                if !(type_0 == 1 as libc::c_int) {
                    sum = 0.0f64;
                    aij = (**((*P).row).offset(i as isize)).ptr;
                    while !aij.is_null() {
                        sum += (*aij).val * *x.offset((*(*aij).col).j as isize);
                        aij = (*aij).r_next;
                    }
                    if type_0 == 2 as libc::c_int || type_0 == 4 as libc::c_int
                        || type_0 == 5 as libc::c_int
                    {
                        if sum < *((*T).orig_lb).offset(i as isize) - 1e-9f64 {
                            ret = 2 as libc::c_int;
                            current_block = 610055490595703683;
                            break;
                        }
                    }
                    if type_0 == 3 as libc::c_int || type_0 == 4 as libc::c_int
                        || type_0 == 5 as libc::c_int
                    {
                        if sum > *((*T).orig_ub).offset(i as isize) + 1e-9f64 {
                            ret = 2 as libc::c_int;
                            current_block = 610055490595703683;
                            break;
                        }
                    }
                }
                i += 1;
                i;
            }
            match current_block {
                610055490595703683 => {}
                _ => {
                    if glp_ios_heur_sol(T, x as *const libc::c_double)
                        == 0 as libc::c_int
                    {
                        ret = 0 as libc::c_int;
                    } else {
                        ret = 1 as libc::c_int;
                    }
                }
            }
        }
        _ => {}
    }
    glp_free(x as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn gmi_gen(mut T: *mut glp_tree) {
    let mut P: *mut glp_prob = 0 as *mut glp_prob;
    let mut pool: *mut glp_prob = 0 as *mut glp_prob;
    P = (*T).mip;
    pool = glp_create_prob();
    glp_add_cols(pool, (*P).n);
    glp_gmi_gen(P, pool, 50 as libc::c_int);
    if (*pool).m > 0 as libc::c_int {
        let mut i: libc::c_int = 0;
        let mut len: libc::c_int = 0;
        let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
        ind = glp_alloc(
            1 as libc::c_int + (*P).n,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        val = glp_alloc(
            1 as libc::c_int + (*P).n,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        i = 1 as libc::c_int;
        while i <= (*pool).m {
            len = glp_get_mat_row(pool, i, ind, val);
            glp_ios_add_row(
                T,
                0 as *const libc::c_char,
                1 as libc::c_int,
                0 as libc::c_int,
                len,
                ind as *const libc::c_int,
                val as *const libc::c_double,
                2 as libc::c_int,
                (**((*pool).row).offset(i as isize)).lb,
            );
            i += 1;
            i;
        }
        glp_free(ind as *mut libc::c_void);
        glp_free(val as *mut libc::c_void);
    }
    glp_delete_prob(pool);
}
unsafe extern "C" fn cov_gen(mut T: *mut glp_tree) {
    let mut P: *mut glp_prob = 0 as *mut glp_prob;
    let mut pool: *mut glp_prob = 0 as *mut glp_prob;
    if ((*T).cov_gen).is_null() {
        return;
    }
    P = (*T).mip;
    pool = glp_create_prob();
    glp_add_cols(pool, (*P).n);
    glp_cov_gen1(P, (*T).cov_gen, pool);
    if (*pool).m > 0 as libc::c_int {
        let mut i: libc::c_int = 0;
        let mut len: libc::c_int = 0;
        let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
        ind = glp_alloc(
            1 as libc::c_int + (*P).n,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        val = glp_alloc(
            1 as libc::c_int + (*P).n,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        i = 1 as libc::c_int;
        while i <= (*pool).m {
            len = glp_get_mat_row(pool, i, ind, val);
            glp_ios_add_row(
                T,
                0 as *const libc::c_char,
                3 as libc::c_int,
                0 as libc::c_int,
                len,
                ind as *const libc::c_int,
                val as *const libc::c_double,
                3 as libc::c_int,
                (**((*pool).row).offset(i as isize)).ub,
            );
            i += 1;
            i;
        }
        glp_free(ind as *mut libc::c_void);
        glp_free(val as *mut libc::c_void);
    }
    glp_delete_prob(pool);
}
unsafe extern "C" fn mir_gen(mut T: *mut glp_tree) {
    let mut P: *mut glp_prob = 0 as *mut glp_prob;
    let mut pool: *mut glp_prob = 0 as *mut glp_prob;
    P = (*T).mip;
    pool = glp_create_prob();
    glp_add_cols(pool, (*P).n);
    glp_mir_gen(P, (*T).mir_gen, pool);
    if (*pool).m > 0 as libc::c_int {
        let mut i: libc::c_int = 0;
        let mut len: libc::c_int = 0;
        let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
        ind = glp_alloc(
            1 as libc::c_int + (*P).n,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        val = glp_alloc(
            1 as libc::c_int + (*P).n,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        i = 1 as libc::c_int;
        while i <= (*pool).m {
            len = glp_get_mat_row(pool, i, ind, val);
            glp_ios_add_row(
                T,
                0 as *const libc::c_char,
                2 as libc::c_int,
                0 as libc::c_int,
                len,
                ind as *const libc::c_int,
                val as *const libc::c_double,
                3 as libc::c_int,
                (**((*pool).row).offset(i as isize)).ub,
            );
            i += 1;
            i;
        }
        glp_free(ind as *mut libc::c_void);
        glp_free(val as *mut libc::c_void);
    }
    glp_delete_prob(pool);
}
unsafe extern "C" fn clq_gen(mut T: *mut glp_tree, mut G: *mut glp_cfg) {
    let mut P: *mut glp_prob = (*T).mip;
    let mut n: libc::c_int = (*P).n;
    let mut len: libc::c_int = 0;
    let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    ind = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    val = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    len = glp_clq_cut((*T).mip, G, ind, val);
    if len > 0 as libc::c_int {
        glp_ios_add_row(
            T,
            0 as *const libc::c_char,
            4 as libc::c_int,
            0 as libc::c_int,
            len,
            ind as *const libc::c_int,
            val as *const libc::c_double,
            3 as libc::c_int,
            *val.offset(0 as libc::c_int as isize),
        );
    }
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
}
unsafe extern "C" fn generate_cuts(mut T: *mut glp_tree) {
    if (*(*T).parm).mir_cuts == 1 as libc::c_int
        || (*(*T).parm).gmi_cuts == 1 as libc::c_int
        || (*(*T).parm).cov_cuts == 1 as libc::c_int
        || (*(*T).parm).clq_cuts == 1 as libc::c_int
    {
        let mut i: libc::c_int = 0;
        let mut max_cuts: libc::c_int = 0;
        let mut added_cuts: libc::c_int = 0;
        max_cuts = (*T).n;
        if max_cuts < 1000 as libc::c_int {
            max_cuts = 1000 as libc::c_int;
        }
        added_cuts = 0 as libc::c_int;
        i = (*T).orig_m + 1 as libc::c_int;
        while i <= (*(*T).mip).m {
            if (**((*(*T).mip).row).offset(i as isize)).origin as libc::c_int
                == 2 as libc::c_int
            {
                added_cuts += 1;
                added_cuts;
            }
            i += 1;
            i;
        }
        if !(added_cuts >= max_cuts) {
            if (*(*T).parm).gmi_cuts == 1 as libc::c_int {
                if (*(*T).curr).changed < 7 as libc::c_int {
                    gmi_gen(T);
                }
            }
            if (*(*T).parm).mir_cuts == 1 as libc::c_int {
                (!((*T).mir_gen).is_null()
                    || {
                        glp_assert_(
                            b"T->mir_gen != NULL\0" as *const u8 as *const libc::c_char,
                            b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                            766 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                mir_gen(T);
            }
            if (*(*T).parm).cov_cuts == 1 as libc::c_int {
                cov_gen(T);
            }
            if (*(*T).parm).clq_cuts == 1 as libc::c_int {
                if !((*T).clq_gen).is_null() {
                    if (*(*T).curr).level == 0 as libc::c_int
                        && (*(*T).curr).changed < 500 as libc::c_int
                        || (*(*T).curr).level > 0 as libc::c_int
                            && (*(*T).curr).changed < 50 as libc::c_int
                    {
                        clq_gen(T, (*T).clq_gen);
                    }
                }
            }
        }
    }
}
unsafe extern "C" fn remove_cuts(mut T: *mut glp_tree) {
    let mut i: libc::c_int = 0;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut num: *mut libc::c_int = 0 as *mut libc::c_int;
    (!((*T).curr).is_null()
        || {
            glp_assert_(
                b"T->curr != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                806 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    i = (*T).orig_m + 1 as libc::c_int;
    while i <= (*(*T).mip).m {
        if (**((*(*T).mip).row).offset(i as isize)).origin as libc::c_int
            == 2 as libc::c_int
            && (**((*(*T).mip).row).offset(i as isize)).level == (*(*T).curr).level
            && (**((*(*T).mip).row).offset(i as isize)).stat == 1 as libc::c_int
        {
            if num.is_null() {
                num = glp_alloc(
                    1 as libc::c_int + (*(*T).mip).m,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
                ) as *mut libc::c_int;
            }
            cnt += 1;
            *num.offset(cnt as isize) = i;
        }
        i += 1;
        i;
    }
    if cnt > 0 as libc::c_int {
        glp_del_rows((*T).mip, cnt, num as *const libc::c_int);
        glp_free(num as *mut libc::c_void);
        (glp_factorize((*T).mip) == 0 as libc::c_int
            || {
                glp_assert_(
                    b"glp_factorize(T->mip) == 0\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                    822 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
}
unsafe extern "C" fn display_cut_info(mut T: *mut glp_tree) {
    let mut mip: *mut glp_prob = (*T).mip;
    let mut i: libc::c_int = 0;
    let mut gmi: libc::c_int = 0 as libc::c_int;
    let mut mir: libc::c_int = 0 as libc::c_int;
    let mut cov: libc::c_int = 0 as libc::c_int;
    let mut clq: libc::c_int = 0 as libc::c_int;
    let mut app: libc::c_int = 0 as libc::c_int;
    i = (*mip).m;
    while i > 0 as libc::c_int {
        let mut row: *mut GLPROW = 0 as *mut GLPROW;
        row = *((*mip).row).offset(i as isize);
        if (*row).origin as libc::c_int == 2 as libc::c_int {
            if (*row).klass as libc::c_int == 1 as libc::c_int {
                gmi += 1;
                gmi;
            } else if (*row).klass as libc::c_int == 2 as libc::c_int {
                mir += 1;
                mir;
            } else if (*row).klass as libc::c_int == 3 as libc::c_int {
                cov += 1;
                cov;
            } else if (*row).klass as libc::c_int == 4 as libc::c_int {
                clq += 1;
                clq;
            } else {
                app += 1;
                app;
            }
        }
        i -= 1;
        i;
    }
    (!((*T).curr).is_null()
        || {
            glp_assert_(
                b"T->curr != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                849 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if gmi + mir + cov + clq + app > 0 as libc::c_int {
        glp_printf(
            b"Cuts on level %d:\0" as *const u8 as *const libc::c_char,
            (*(*T).curr).level,
        );
        if gmi > 0 as libc::c_int {
            glp_printf(b" gmi = %d;\0" as *const u8 as *const libc::c_char, gmi);
        }
        if mir > 0 as libc::c_int {
            glp_printf(b" mir = %d;\0" as *const u8 as *const libc::c_char, mir);
        }
        if cov > 0 as libc::c_int {
            glp_printf(b" cov = %d;\0" as *const u8 as *const libc::c_char, cov);
        }
        if clq > 0 as libc::c_int {
            glp_printf(b" clq = %d;\0" as *const u8 as *const libc::c_char, clq);
        }
        if app > 0 as libc::c_int {
            glp_printf(b" app = %d;\0" as *const u8 as *const libc::c_char, app);
        }
        glp_printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn _glp_ios_driver(mut T: *mut glp_tree) -> libc::c_int {
    let mut current_block: u64;
    let mut p: libc::c_int = 0;
    let mut curr_p: libc::c_int = 0;
    let mut p_stat: libc::c_int = 0;
    let mut d_stat: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut pred_p: libc::c_int = 0 as libc::c_int;
    let mut bad_cut: libc::c_int = 0;
    let mut old_obj: libc::c_double = 0.;
    let mut ttt: libc::c_double = (*T).tm_beg;
    let mut root_done: libc::c_int = 0 as libc::c_int;
    if (*((*T).parm as *mut glp_iocp)).flip != 0 {
        if (*(*T).parm).msg_lev >= 3 as libc::c_int {
            glp_printf(
                b"Long-step dual simplex will be used\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    '_loop: loop {
        (((*T).curr).is_null()
            || {
                glp_assert_(
                    b"T->curr == NULL\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                    930 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if ((*T).head).is_null() {
            if (*(*T).parm).msg_lev >= 4 as libc::c_int {
                glp_printf(
                    b"Active list is empty!\n\0" as *const u8 as *const libc::c_char,
                );
            }
            (_glp_dmp_in_use((*T).pool) == 0 as libc::c_int as libc::c_ulong
                || {
                    glp_assert_(
                        b"dmp_in_use(T->pool) == 0\0" as *const u8
                            as *const libc::c_char,
                        b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                        938 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            ret = 0 as libc::c_int;
            break;
        } else {
            ((*T).next_p == 0 as libc::c_int
                || {
                    glp_assert_(
                        b"T->next_p == 0\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                        944 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if ((*(*T).parm).cb_func).is_some() {
                ((*T).reason == 0 as libc::c_int
                    || {
                        glp_assert_(
                            b"T->reason == 0\0" as *const u8 as *const libc::c_char,
                            b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                            947 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*T).reason = 0x6 as libc::c_int;
                ((*(*T).parm).cb_func).unwrap()(T, (*(*T).parm).cb_info);
                (*T).reason = 0 as libc::c_int;
                if (*T).stop != 0 {
                    ret = 0xd as libc::c_int;
                    break;
                }
            }
            if !((*T).next_p != 0 as libc::c_int) {
                if (*T).a_cnt == 1 as libc::c_int {
                    (((*(*T).head).next).is_null()
                        || {
                            glp_assert_(
                                b"T->head->next == NULL\0" as *const u8
                                    as *const libc::c_char,
                                b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                962 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    (*T).next_p = (*(*T).head).p;
                } else if (*T).child != 0 as libc::c_int {
                    (*T).next_p = (*T).child;
                } else {
                    (*T).next_p = _glp_ios_choose_node(T);
                }
            }
            _glp_ios_revive_node(T, (*T).next_p);
            (*T).child = 0 as libc::c_int;
            (*T).next_p = (*T).child;
            if !((*(*T).curr).up).is_null() && (*(*(*T).curr).up).p != pred_p {
                pred_p = 0 as libc::c_int;
            }
            p = (*(*T).curr).p;
            if (*(*T).parm).msg_lev >= 4 as libc::c_int {
                glp_printf(
                    b"------------------------------------------------------------------------\n\0"
                        as *const u8 as *const libc::c_char,
                );
                glp_printf(
                    b"Processing node %d at level %d\n\0" as *const u8
                        as *const libc::c_char,
                    p,
                    (*(*T).curr).level,
                );
            }
            if p == 1 as libc::c_int {
                if (*(*T).parm).sr_heur == 0 as libc::c_int {
                    if (*(*T).parm).msg_lev >= 3 as libc::c_int {
                        glp_printf(
                            b"Simple rounding heuristic disabled\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
            }
            if p == 1 as libc::c_int {
                if (*(*T).parm).gmi_cuts == 1 as libc::c_int {
                    if (*(*T).parm).msg_lev >= 3 as libc::c_int {
                        glp_printf(
                            b"Gomory's cuts enabled\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
                if (*(*T).parm).mir_cuts == 1 as libc::c_int {
                    if (*(*T).parm).msg_lev >= 3 as libc::c_int {
                        glp_printf(
                            b"MIR cuts enabled\n\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    (((*T).mir_gen).is_null()
                        || {
                            glp_assert_(
                                b"T->mir_gen == NULL\0" as *const u8 as *const libc::c_char,
                                b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                1009 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    (*T).mir_gen = glp_mir_init((*T).mip);
                }
                if (*(*T).parm).cov_cuts == 1 as libc::c_int {
                    if (*(*T).parm).msg_lev >= 3 as libc::c_int {
                        glp_printf(
                            b"Cover cuts enabled\n\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    (((*T).cov_gen).is_null()
                        || {
                            glp_assert_(
                                b"T->cov_gen == NULL\0" as *const u8 as *const libc::c_char,
                                b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                1020 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    (*T).cov_gen = glp_cov_init((*T).mip);
                }
                if (*(*T).parm).clq_cuts == 1 as libc::c_int {
                    (((*T).clq_gen).is_null()
                        || {
                            glp_assert_(
                                b"T->clq_gen == NULL\0" as *const u8 as *const libc::c_char,
                                b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                1025 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    if (*(*T).parm).msg_lev >= 3 as libc::c_int {
                        glp_printf(
                            b"Clique cuts enabled\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    (*T).clq_gen = glp_cfg_init((*T).mip);
                }
            }
            bad_cut = 0 as libc::c_int;
            loop {
                if (*(*T).parm).msg_lev >= 4 as libc::c_int
                    || (*(*T).parm).msg_lev >= 2 as libc::c_int
                        && ((*(*T).parm).out_frq - 1 as libc::c_int) as libc::c_double
                            <= 1000.0f64 * glp_difftime(glp_time(), (*T).tm_lag)
                {
                    show_progress(T, 0 as libc::c_int);
                }
                if (*(*T).parm).msg_lev >= 3 as libc::c_int
                    && glp_difftime(glp_time(), ttt) >= 60.0f64
                {
                    let mut total: size_t = 0;
                    glp_mem_usage(
                        0 as *mut libc::c_int,
                        0 as *mut libc::c_int,
                        &mut total,
                        0 as *mut size_t,
                    );
                    glp_printf(
                        b"Time used: %.1f secs.  Memory used: %.1f Mb.\n\0" as *const u8
                            as *const libc::c_char,
                        glp_difftime(glp_time(), (*T).tm_beg),
                        total as libc::c_double / 1048576.0f64,
                    );
                    ttt = glp_time();
                }
                if (*(*T).parm).mip_gap > 0.0f64
                    && _glp_ios_relative_gap(T) <= (*(*T).parm).mip_gap
                {
                    if (*(*T).parm).msg_lev >= 4 as libc::c_int {
                        glp_printf(
                            b"Relative gap tolerance reached; search terminated \n\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    ret = 0xe as libc::c_int;
                    break '_loop;
                } else if (*(*T).parm).tm_lim < 2147483647 as libc::c_int
                    && ((*(*T).parm).tm_lim - 1 as libc::c_int) as libc::c_double
                        <= 1000.0f64 * glp_difftime(glp_time(), (*T).tm_beg)
                {
                    if (*(*T).parm).msg_lev >= 4 as libc::c_int {
                        glp_printf(
                            b"Time limit exhausted; search terminated\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    ret = 0x9 as libc::c_int;
                    break '_loop;
                } else {
                    if ((*(*T).parm).cb_func).is_some() {
                        ((*T).reason == 0 as libc::c_int
                            || {
                                glp_assert_(
                                    b"T->reason == 0\0" as *const u8 as *const libc::c_char,
                                    b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                    1084 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        (*T).reason = 0x7 as libc::c_int;
                        ((*(*T).parm).cb_func).unwrap()(T, (*(*T).parm).cb_info);
                        (*T).reason = 0 as libc::c_int;
                        if (*T).stop != 0 {
                            ret = 0xd as libc::c_int;
                            break '_loop;
                        }
                    }
                    if !((*(*T).parm).pp_tech == 0 as libc::c_int) {
                        if (*(*T).parm).pp_tech == 1 as libc::c_int {
                            if root_done == 0 {
                                if _glp_ios_preprocess_node(T, 100 as libc::c_int) != 0 {
                                    current_block = 15395872843927266160;
                                    break;
                                }
                            }
                        } else if (*(*T).parm).pp_tech == 2 as libc::c_int {
                            if _glp_ios_preprocess_node(
                                T,
                                if root_done == 0 {
                                    100 as libc::c_int
                                } else {
                                    10 as libc::c_int
                                },
                            ) != 0
                            {
                                current_block = 15395872843927266160;
                                break;
                            }
                        } else {
                            (T != T
                                || {
                                    glp_assert_(
                                        b"T != T\0" as *const u8 as *const libc::c_char,
                                        b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                        1115 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                        }
                    }
                    if is_branch_hopeful(T, p) == 0 {
                        glp_printf(
                            b"*** not tested yet ***\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        current_block = 15395872843927266160;
                        break;
                    } else {
                        if (*(*T).parm).msg_lev >= 4 as libc::c_int {
                            glp_printf(
                                b"Solving LP relaxation...\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        ret = _glp_ios_solve_node(T);
                        if ret == 0x9 as libc::c_int {
                            break '_loop;
                        }
                        if !(ret == 0 as libc::c_int || ret == 0x6 as libc::c_int
                            || ret == 0x7 as libc::c_int)
                        {
                            if (*(*T).parm).msg_lev >= 1 as libc::c_int {
                                glp_printf(
                                    b"ios_driver: unable to solve current LP relaxation; glp_simplex returned %d\n\0"
                                        as *const u8 as *const libc::c_char,
                                    ret,
                                );
                            }
                            ret = 0x5 as libc::c_int;
                            break '_loop;
                        } else {
                            p_stat = (*(*T).mip).pbs_stat;
                            d_stat = (*(*T).mip).dbs_stat;
                            if p_stat == 2 as libc::c_int && d_stat == 2 as libc::c_int {
                                if (*(*T).parm).msg_lev >= 4 as libc::c_int {
                                    glp_printf(
                                        b"Found optimal solution to LP relaxation\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                            } else if d_stat == 4 as libc::c_int {
                                if (*(*T).parm).msg_lev >= 1 as libc::c_int {
                                    glp_printf(
                                        b"ios_driver: current LP relaxation has no dual feasible solution\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                }
                                ret = 0x5 as libc::c_int;
                                break '_loop;
                            } else if p_stat == 3 as libc::c_int
                                && d_stat == 2 as libc::c_int
                            {
                                ((*(*T).mip).mip_stat == 2 as libc::c_int
                                    || {
                                        glp_assert_(
                                            b"T->mip->mip_stat == GLP_FEAS\0" as *const u8
                                                as *const libc::c_char,
                                            b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                            1155 as libc::c_int,
                                        );
                                        1 as libc::c_int != 0
                                    }) as libc::c_int;
                                if (*(*T).parm).msg_lev >= 4 as libc::c_int {
                                    glp_printf(
                                        b"LP relaxation has no solution better than incumbent objective value\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                }
                                current_block = 15395872843927266160;
                                break;
                            } else if p_stat == 4 as libc::c_int {
                                if (*(*T).parm).msg_lev >= 4 as libc::c_int {
                                    glp_printf(
                                        b"LP relaxation has no feasible solution\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                current_block = 15395872843927266160;
                                break;
                            } else {
                                ((*T).mip != (*T).mip
                                    || {
                                        glp_assert_(
                                            b"T->mip != T->mip\0" as *const u8 as *const libc::c_char,
                                            b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                            1171 as libc::c_int,
                                        );
                                        1 as libc::c_int != 0
                                    }) as libc::c_int;
                            }
                            (p_stat == 2 as libc::c_int && d_stat == 2 as libc::c_int
                                || {
                                    glp_assert_(
                                        b"p_stat == GLP_FEAS && d_stat == GLP_FEAS\0" as *const u8
                                            as *const libc::c_char,
                                        b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                        1175 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                            (!((*T).curr).is_null()
                                || {
                                    glp_assert_(
                                        b"T->curr != NULL\0" as *const u8 as *const libc::c_char,
                                        b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                        1176 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                            (*(*T).curr).lp_obj = (*(*T).mip).obj_val;
                            let mut bound: libc::c_double = (*(*T).mip).obj_val;
                            bound = _glp_ios_round_bound(T, bound);
                            if (*(*T).mip).dir == 1 as libc::c_int {
                                if (*(*T).curr).bound < bound {
                                    (*(*T).curr).bound = bound;
                                }
                            } else if (*(*T).mip).dir == 2 as libc::c_int {
                                if (*(*T).curr).bound > bound {
                                    (*(*T).curr).bound = bound;
                                }
                            } else {
                                ((*T).mip != (*T).mip
                                    || {
                                        glp_assert_(
                                            b"T->mip != T->mip\0" as *const u8 as *const libc::c_char,
                                            b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                            1193 as libc::c_int,
                                        );
                                        1 as libc::c_int != 0
                                    }) as libc::c_int;
                            }
                            if (*(*T).parm).msg_lev >= 4 as libc::c_int {
                                glp_printf(
                                    b"Local bound is %.9e\n\0" as *const u8
                                        as *const libc::c_char,
                                    bound,
                                );
                            }
                            if is_branch_hopeful(T, p) == 0 {
                                if (*(*T).parm).msg_lev >= 4 as libc::c_int {
                                    glp_printf(
                                        b"Current branch is hopeless and can be pruned\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                }
                                current_block = 15395872843927266160;
                                break;
                            } else {
                                ((*T).reopt == 0 as libc::c_int
                                    || {
                                        glp_assert_(
                                            b"T->reopt == 0\0" as *const u8 as *const libc::c_char,
                                            b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                            1207 as libc::c_int,
                                        );
                                        1 as libc::c_int != 0
                                    }) as libc::c_int;
                                ((*T).reinv == 0 as libc::c_int
                                    || {
                                        glp_assert_(
                                            b"T->reinv == 0\0" as *const u8 as *const libc::c_char,
                                            b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                            1208 as libc::c_int,
                                        );
                                        1 as libc::c_int != 0
                                    }) as libc::c_int;
                                if ((*(*T).parm).cb_func).is_some() {
                                    ((*T).reason == 0 as libc::c_int
                                        || {
                                            glp_assert_(
                                                b"T->reason == 0\0" as *const u8 as *const libc::c_char,
                                                b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                                1210 as libc::c_int,
                                            );
                                            1 as libc::c_int != 0
                                        }) as libc::c_int;
                                    (*T).reason = 0x1 as libc::c_int;
                                    ((*(*T).parm).cb_func).unwrap()(T, (*(*T).parm).cb_info);
                                    (*T).reason = 0 as libc::c_int;
                                    if (*T).stop != 0 {
                                        ret = 0xd as libc::c_int;
                                        break '_loop;
                                    } else if (*T).reopt != 0 {
                                        (*T).reinv = 0 as libc::c_int;
                                        (*T).reopt = (*T).reinv;
                                        continue;
                                    } else if (*T).reinv != 0 {
                                        (*T).reinv = 0 as libc::c_int;
                                        (glp_factorize((*T).mip) == 0 as libc::c_int
                                            || {
                                                glp_assert_(
                                                    b"glp_factorize(T->mip) == 0\0" as *const u8
                                                        as *const libc::c_char,
                                                    b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                                    1227 as libc::c_int,
                                                );
                                                1 as libc::c_int != 0
                                            }) as libc::c_int;
                                    }
                                }
                                check_integrality(T);
                                if (*(*T).curr).ii_cnt == 0 as libc::c_int {
                                    if (*(*T).parm).msg_lev >= 4 as libc::c_int {
                                        glp_printf(
                                            b"New integer feasible solution found\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                    }
                                    if (*(*T).parm).msg_lev >= 3 as libc::c_int {
                                        display_cut_info(T);
                                    }
                                    record_solution(T);
                                    if (*(*T).parm).msg_lev >= 2 as libc::c_int {
                                        show_progress(T, 1 as libc::c_int);
                                    }
                                    _glp_ios_process_sol(T);
                                    if ((*(*T).parm).cb_func).is_some() {
                                        current_block = 4871270227279186910;
                                        break;
                                    } else {
                                        current_block = 8880031775101799352;
                                        break;
                                    }
                                } else {
                                    if (*(*T).mip).mip_stat == 2 as libc::c_int {
                                        fix_by_red_cost(T);
                                    }
                                    if ((*(*T).parm).cb_func).is_some() {
                                        ((*T).reason == 0 as libc::c_int
                                            || {
                                                glp_assert_(
                                                    b"T->reason == 0\0" as *const u8 as *const libc::c_char,
                                                    b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                                    1269 as libc::c_int,
                                                );
                                                1 as libc::c_int != 0
                                            }) as libc::c_int;
                                        (*T).reason = 0x3 as libc::c_int;
                                        ((*(*T).parm).cb_func).unwrap()(T, (*(*T).parm).cb_info);
                                        (*T).reason = 0 as libc::c_int;
                                        if (*T).stop != 0 {
                                            ret = 0xd as libc::c_int;
                                            break '_loop;
                                        } else if is_branch_hopeful(T, p) == 0 {
                                            if (*(*T).parm).msg_lev >= 4 as libc::c_int {
                                                glp_printf(
                                                    b"Current branch became hopeless and can be pruned\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                );
                                            }
                                            current_block = 15395872843927266160;
                                            break;
                                        }
                                    }
                                    if (*(*T).parm).fp_heur != 0 && root_done == 0 {
                                        ((*T).reason == 0 as libc::c_int
                                            || {
                                                glp_assert_(
                                                    b"T->reason == 0\0" as *const u8 as *const libc::c_char,
                                                    b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                                    1291 as libc::c_int,
                                                );
                                                1 as libc::c_int != 0
                                            }) as libc::c_int;
                                        (*T).reason = 0x3 as libc::c_int;
                                        _glp_ios_feas_pump(T);
                                        (*T).reason = 0 as libc::c_int;
                                        if is_branch_hopeful(T, p) == 0 {
                                            if (*(*T).parm).msg_lev >= 4 as libc::c_int {
                                                glp_printf(
                                                    b"Current branch became hopeless and can be pruned\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                );
                                            }
                                            current_block = 15395872843927266160;
                                            break;
                                        }
                                    }
                                    if (*(*T).parm).ps_heur != 0 && root_done == 0 {
                                        ((*T).reason == 0 as libc::c_int
                                            || {
                                                glp_assert_(
                                                    b"T->reason == 0\0" as *const u8 as *const libc::c_char,
                                                    b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                                    1310 as libc::c_int,
                                                );
                                                1 as libc::c_int != 0
                                            }) as libc::c_int;
                                        (*T).reason = 0x3 as libc::c_int;
                                        _glp_ios_proxy_heur(T);
                                        (*T).reason = 0 as libc::c_int;
                                        if is_branch_hopeful(T, p) == 0 {
                                            if (*(*T).parm).msg_lev >= 4 as libc::c_int {
                                                glp_printf(
                                                    b"Current branch became hopeless and can be pruned\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                );
                                            }
                                            current_block = 15395872843927266160;
                                            break;
                                        }
                                    }
                                    if (*(*T).parm).sr_heur != 0 {
                                        ((*T).reason == 0 as libc::c_int
                                            || {
                                                glp_assert_(
                                                    b"T->reason == 0\0" as *const u8 as *const libc::c_char,
                                                    b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                                    1326 as libc::c_int,
                                                );
                                                1 as libc::c_int != 0
                                            }) as libc::c_int;
                                        (*T).reason = 0x3 as libc::c_int;
                                        round_heur(T);
                                        (*T).reason = 0 as libc::c_int;
                                        if is_branch_hopeful(T, p) == 0 {
                                            if (*(*T).parm).msg_lev >= 4 as libc::c_int {
                                                glp_printf(
                                                    b"Current branch became hopeless and can be pruned\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                );
                                            }
                                            current_block = 15395872843927266160;
                                            break;
                                        }
                                    }
                                    (!((*T).local).is_null()
                                        || {
                                            glp_assert_(
                                                b"T->local != NULL\0" as *const u8 as *const libc::c_char,
                                                b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                                1340 as libc::c_int,
                                            );
                                            1 as libc::c_int != 0
                                        }) as libc::c_int;
                                    ((*(*T).local).m == 0 as libc::c_int
                                        || {
                                            glp_assert_(
                                                b"T->local->m == 0\0" as *const u8 as *const libc::c_char,
                                                b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                                1342 as libc::c_int,
                                            );
                                            1 as libc::c_int != 0
                                        }) as libc::c_int;
                                    if ((*(*T).parm).cb_func).is_some() {
                                        ((*T).reason == 0 as libc::c_int
                                            || {
                                                glp_assert_(
                                                    b"T->reason == 0\0" as *const u8 as *const libc::c_char,
                                                    b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                                    1350 as libc::c_int,
                                                );
                                                1 as libc::c_int != 0
                                            }) as libc::c_int;
                                        (*T).reason = 0x4 as libc::c_int;
                                        ((*(*T).parm).cb_func).unwrap()(T, (*(*T).parm).cb_info);
                                        (*T).reason = 0 as libc::c_int;
                                        if (*T).stop != 0 {
                                            ret = 0xd as libc::c_int;
                                            break '_loop;
                                        }
                                    }
                                    if (*(*T).curr).changed > 0 as libc::c_int {
                                        let mut degrad: libc::c_double = fabs(
                                            (*(*T).curr).lp_obj - old_obj,
                                        );
                                        if degrad < 1e-4f64 * (1.0f64 + fabs(old_obj)) {
                                            bad_cut += 1;
                                            bad_cut;
                                        } else {
                                            bad_cut = 0 as libc::c_int;
                                        }
                                    }
                                    old_obj = (*(*T).curr).lp_obj;
                                    if bad_cut == 0 as libc::c_int
                                        || root_done == 0 && bad_cut <= 3 as libc::c_int
                                    {
                                        if root_done == 0 || pred_p == 0 as libc::c_int {
                                            ((*T).reason == 0 as libc::c_int
                                                || {
                                                    glp_assert_(
                                                        b"T->reason == 0\0" as *const u8 as *const libc::c_char,
                                                        b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                                        1384 as libc::c_int,
                                                    );
                                                    1 as libc::c_int != 0
                                                }) as libc::c_int;
                                            (*T).reason = 0x4 as libc::c_int;
                                            generate_cuts(T);
                                            (*T).reason = 0 as libc::c_int;
                                        }
                                    }
                                    if (*(*T).local).m > 0 as libc::c_int {
                                        ((*T).reason == 0 as libc::c_int
                                            || {
                                                glp_assert_(
                                                    b"T->reason == 0\0" as *const u8 as *const libc::c_char,
                                                    b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                                    1396 as libc::c_int,
                                                );
                                                1 as libc::c_int != 0
                                            }) as libc::c_int;
                                        (*T).reason = 0x4 as libc::c_int;
                                        _glp_ios_process_cuts(T);
                                        (*T).reason = 0 as libc::c_int;
                                    }
                                    _glp_ios_clear_pool(T, (*T).local);
                                    if (*T).reopt != 0 {
                                        (*T).reopt = 0 as libc::c_int;
                                        (*(*T).curr).changed += 1;
                                        (*(*T).curr).changed;
                                    } else {
                                        remove_cuts(T);
                                        if (*(*T).parm).msg_lev >= 3 as libc::c_int
                                            && root_done == 0
                                        {
                                            display_cut_info(T);
                                        }
                                        if root_done == 0 {
                                            root_done = 1 as libc::c_int;
                                        }
                                        if !((*T).pcost).is_null() {
                                            _glp_ios_pcost_update(T);
                                        }
                                        ((*T).br_var == 0 as libc::c_int
                                            || {
                                                glp_assert_(
                                                    b"T->br_var == 0\0" as *const u8 as *const libc::c_char,
                                                    b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                                    1424 as libc::c_int,
                                                );
                                                1 as libc::c_int != 0
                                            }) as libc::c_int;
                                        ((*T).br_sel == 0 as libc::c_int
                                            || {
                                                glp_assert_(
                                                    b"T->br_sel == 0\0" as *const u8 as *const libc::c_char,
                                                    b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                                    1425 as libc::c_int,
                                                );
                                                1 as libc::c_int != 0
                                            }) as libc::c_int;
                                        if ((*(*T).parm).cb_func).is_some() {
                                            ((*T).reason == 0 as libc::c_int
                                                || {
                                                    glp_assert_(
                                                        b"T->reason == 0\0" as *const u8 as *const libc::c_char,
                                                        b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                                        1428 as libc::c_int,
                                                    );
                                                    1 as libc::c_int != 0
                                                }) as libc::c_int;
                                            ((*T).br_var == 0 as libc::c_int
                                                || {
                                                    glp_assert_(
                                                        b"T->br_var == 0\0" as *const u8 as *const libc::c_char,
                                                        b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                                        1429 as libc::c_int,
                                                    );
                                                    1 as libc::c_int != 0
                                                }) as libc::c_int;
                                            ((*T).br_sel == 0 as libc::c_int
                                                || {
                                                    glp_assert_(
                                                        b"T->br_sel == 0\0" as *const u8 as *const libc::c_char,
                                                        b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                                        1430 as libc::c_int,
                                                    );
                                                    1 as libc::c_int != 0
                                                }) as libc::c_int;
                                            (*T).reason = 0x5 as libc::c_int;
                                            ((*(*T).parm).cb_func).unwrap()(T, (*(*T).parm).cb_info);
                                            (*T).reason = 0 as libc::c_int;
                                            if (*T).stop != 0 {
                                                ret = 0xd as libc::c_int;
                                                break '_loop;
                                            }
                                        }
                                        if (*T).br_var == 0 as libc::c_int {
                                            (*T).br_var = _glp_ios_choose_var(T, &mut (*T).br_sel);
                                        }
                                        curr_p = (*(*T).curr).p;
                                        ret = branch_on(T, (*T).br_var, (*T).br_sel);
                                        (*T).br_sel = 0 as libc::c_int;
                                        (*T).br_var = (*T).br_sel;
                                        if ret == 0 as libc::c_int {
                                            pred_p = curr_p;
                                            continue '_loop;
                                        } else if ret == 1 as libc::c_int {
                                            (*(*T).curr).changed = 0 as libc::c_int;
                                            (*(*T).curr).solved = (*(*T).curr).changed;
                                        } else if ret == 2 as libc::c_int {
                                            current_block = 15395872843927266160;
                                            break;
                                        } else {
                                            current_block = 3818209998506676277;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            match current_block {
                4871270227279186910 => {
                    ((*T).reason == 0 as libc::c_int
                        || {
                            glp_assert_(
                                b"T->reason == 0\0" as *const u8 as *const libc::c_char,
                                b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                1247 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    (*T).reason = 0x2 as libc::c_int;
                    ((*(*T).parm).cb_func).unwrap()(T, (*(*T).parm).cb_info);
                    (*T).reason = 0 as libc::c_int;
                    if (*T).stop != 0 {
                        ret = 0xd as libc::c_int;
                        break;
                    } else {
                        current_block = 8880031775101799352;
                    }
                }
                3818209998506676277 => {
                    (ret != ret
                        || {
                            glp_assert_(
                                b"ret != ret\0" as *const u8 as *const libc::c_char,
                                b"draft/glpios03.c\0" as *const u8 as *const libc::c_char,
                                1469 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    current_block = 15395872843927266160;
                }
                _ => {}
            }
            match current_block {
                8880031775101799352 => {}
                _ => {}
            }
            if (*(*T).parm).msg_lev >= 4 as libc::c_int {
                glp_printf(
                    b"Node %d fathomed\n\0" as *const u8 as *const libc::c_char,
                    p,
                );
            }
            _glp_ios_freeze_node(T);
            _glp_ios_delete_node(T, p);
            if (*(*T).mip).mip_stat == 2 as libc::c_int {
                cleanup_the_tree(T);
            }
            pred_p = 0 as libc::c_int;
        }
    }
    if (*(*T).parm).msg_lev >= 2 as libc::c_int {
        show_progress(T, 0 as libc::c_int);
    }
    if !((*T).mir_gen).is_null() {
        glp_mir_free((*T).mir_gen);
        (*T).mir_gen = 0 as *mut glp_mir;
    }
    if !((*T).cov_gen).is_null() {
        glp_cov_free((*T).cov_gen);
        (*T).cov_gen = 0 as *mut glp_cov;
    }
    if !((*T).clq_gen).is_null() {
        glp_cfg_free((*T).clq_gen);
        (*T).clq_gen = 0 as *mut glp_cfg;
    }
    return ret;
}
