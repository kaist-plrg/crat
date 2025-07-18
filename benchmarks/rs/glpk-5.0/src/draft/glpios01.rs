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
    fn glp_time() -> libc::c_double;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_create_prob() -> *mut glp_prob;
    fn glp_add_rows(P: *mut glp_prob, nrs: libc::c_int) -> libc::c_int;
    fn glp_add_cols(P: *mut glp_prob, ncs: libc::c_int) -> libc::c_int;
    fn glp_set_row_name(P: *mut glp_prob, i: libc::c_int, name: *const libc::c_char);
    fn glp_set_row_bnds(
        P: *mut glp_prob,
        i: libc::c_int,
        type_0: libc::c_int,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_set_col_bnds(
        P: *mut glp_prob,
        j: libc::c_int,
        type_0: libc::c_int,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_set_mat_row(
        P: *mut glp_prob,
        i: libc::c_int,
        len: libc::c_int,
        ind: *const libc::c_int,
        val: *const libc::c_double,
    );
    fn glp_del_rows(P: *mut glp_prob, nrs: libc::c_int, num: *const libc::c_int);
    fn glp_delete_prob(P: *mut glp_prob);
    fn glp_get_row_name(P: *mut glp_prob, i: libc::c_int) -> *const libc::c_char;
    fn glp_get_mat_row(
        P: *mut glp_prob,
        i: libc::c_int,
        ind: *mut libc::c_int,
        val: *mut libc::c_double,
    ) -> libc::c_int;
    fn glp_set_rii(P: *mut glp_prob, i: libc::c_int, rii: libc::c_double);
    fn glp_set_row_stat(P: *mut glp_prob, i: libc::c_int, stat: libc::c_int);
    fn glp_set_col_stat(P: *mut glp_prob, j: libc::c_int, stat: libc::c_int);
    fn glp_adv_basis(P: *mut glp_prob, flags: libc::c_int);
    fn glp_simplex(P: *mut glp_prob, parm: *const glp_smcp) -> libc::c_int;
    fn glp_init_smcp(parm: *mut glp_smcp);
    fn glp_get_status(P: *mut glp_prob) -> libc::c_int;
    fn glp_write_mip(P: *mut glp_prob, fname: *const libc::c_char) -> libc::c_int;
    fn glp_bf_exists(P: *mut glp_prob) -> libc::c_int;
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
    fn _glp_dmp_delete_pool(pool: *mut DMP);
    fn _glp_dmp_free_atom(pool: *mut DMP, atom: *mut libc::c_void, size: libc::c_int);
    fn _glp_dmp_get_atom(pool: *mut DMP, size: libc::c_int) -> *mut libc::c_void;
    fn _glp_dmp_create_pool() -> *mut DMP;
    fn _glp_ios_pcost_free(tree: *mut glp_tree);
    fn _glp_gcdn(n: libc::c_int, x: *mut libc::c_int) -> libc::c_int;
    fn _glp_npp_postprocess(npp: *mut NPP, prob: *mut glp_prob);
    fn _glp_npp_unload_sol(npp: *mut NPP, orig: *mut glp_prob);
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
pub struct glp_prep {
    pub orig_dir: libc::c_int,
    pub orig_m: libc::c_int,
    pub orig_n: libc::c_int,
    pub orig_nnz: libc::c_int,
    pub pool: *mut DMP,
    pub name: *mut libc::c_char,
    pub obj: *mut libc::c_char,
    pub c0: libc::c_double,
    pub nrows: libc::c_int,
    pub ncols: libc::c_int,
    pub r_head: *mut NPPROW,
    pub r_tail: *mut NPPROW,
    pub c_head: *mut NPPCOL,
    pub c_tail: *mut NPPCOL,
    pub stack: *mut DMP,
    pub top: *mut NPPTSE,
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub nnz: libc::c_int,
    pub row_ref: *mut libc::c_int,
    pub col_ref: *mut libc::c_int,
    pub sol: libc::c_int,
    pub scaling: libc::c_int,
    pub p_stat: libc::c_int,
    pub d_stat: libc::c_int,
    pub t_stat: libc::c_int,
    pub i_stat: libc::c_int,
    pub r_stat: *mut libc::c_char,
    pub c_stat: *mut libc::c_char,
    pub r_pi: *mut libc::c_double,
    pub c_value: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPTSE {
    pub func: Option::<unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> libc::c_int>,
    pub info: *mut libc::c_void,
    pub link: *mut NPPTSE,
}
pub type NPP = glp_prep;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPCOL {
    pub j: libc::c_int,
    pub name: *mut libc::c_char,
    pub is_int: libc::c_char,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub coef: libc::c_double,
    pub ptr: *mut NPPAIJ,
    pub temp: libc::c_int,
    pub ll: C2RustUnnamed_0,
    pub uu: C2RustUnnamed,
    pub prev: *mut NPPCOL,
    pub next: *mut NPPCOL,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub uu: libc::c_double,
    pub neg: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ll: libc::c_double,
    pub pos: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPAIJ {
    pub row: *mut NPPROW,
    pub col: *mut NPPCOL,
    pub val: libc::c_double,
    pub r_prev: *mut NPPAIJ,
    pub r_next: *mut NPPAIJ,
    pub c_prev: *mut NPPAIJ,
    pub c_next: *mut NPPAIJ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPROW {
    pub i: libc::c_int,
    pub name: *mut libc::c_char,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut NPPAIJ,
    pub temp: libc::c_int,
    pub prev: *mut NPPROW,
    pub next: *mut NPPROW,
}
pub type IOSCUT = GLPROW;
unsafe extern "C" fn lpx_eval_tab_row(
    mut lp: *mut glp_prob,
    mut k: libc::c_int,
    mut ind: *mut libc::c_int,
    mut val: *mut libc::c_double,
) -> libc::c_int {
    return glp_eval_tab_row(lp, k, ind, val);
}
unsafe extern "C" fn lpx_dual_ratio_test(
    mut lp: *mut glp_prob,
    mut len: libc::c_int,
    mut ind: *const libc::c_int,
    mut val: *const libc::c_double,
    mut how: libc::c_int,
    mut tol: libc::c_double,
) -> libc::c_int {
    let mut piv: libc::c_int = 0;
    piv = glp_dual_rtest(lp, len, ind, val, how, tol);
    (0 as libc::c_int <= piv && piv <= len
        || {
            glp_assert_(
                b"0 <= piv && piv <= len\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                37 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return if piv == 0 as libc::c_int {
        0 as libc::c_int
    } else {
        *ind.offset(piv as isize)
    };
}
pub unsafe extern "C" fn _glp_ios_create_tree(
    mut mip: *mut glp_prob,
    mut parm: *const glp_iocp,
) -> *mut glp_tree {
    let mut m: libc::c_int = (*mip).m;
    let mut n: libc::c_int = (*mip).n;
    let mut tree: *mut glp_tree = 0 as *mut glp_tree;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    (((*mip).tree).is_null()
        || {
            glp_assert_(
                b"mip->tree == NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                70 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    tree = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<glp_tree>() as libc::c_ulong as libc::c_int,
    ) as *mut glp_tree;
    (*mip).tree = tree;
    (*tree).pool = _glp_dmp_create_pool();
    (*tree).n = n;
    (*tree).orig_m = m;
    (*tree)
        .orig_type = glp_alloc(
        1 as libc::c_int + m + n,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_uchar;
    (*tree)
        .orig_lb = glp_alloc(
        1 as libc::c_int + m + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*tree)
        .orig_ub = glp_alloc(
        1 as libc::c_int + m + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*tree)
        .orig_stat = glp_alloc(
        1 as libc::c_int + m + n,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_uchar;
    (*tree)
        .orig_prim = glp_alloc(
        1 as libc::c_int + m + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*tree)
        .orig_dual = glp_alloc(
        1 as libc::c_int + m + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    i = 1 as libc::c_int;
    while i <= m {
        let mut row: *mut GLPROW = *((*mip).row).offset(i as isize);
        *((*tree).orig_type)
            .offset(i as isize) = (*row).type_0 as libc::c_char as libc::c_uchar;
        *((*tree).orig_lb).offset(i as isize) = (*row).lb;
        *((*tree).orig_ub).offset(i as isize) = (*row).ub;
        *((*tree).orig_stat)
            .offset(i as isize) = (*row).stat as libc::c_char as libc::c_uchar;
        *((*tree).orig_prim).offset(i as isize) = (*row).prim;
        *((*tree).orig_dual).offset(i as isize) = (*row).dual;
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= n {
        let mut col: *mut GLPCOL = *((*mip).col).offset(j as isize);
        *((*tree).orig_type)
            .offset((m + j) as isize) = (*col).type_0 as libc::c_char as libc::c_uchar;
        *((*tree).orig_lb).offset((m + j) as isize) = (*col).lb;
        *((*tree).orig_ub).offset((m + j) as isize) = (*col).ub;
        *((*tree).orig_stat)
            .offset((m + j) as isize) = (*col).stat as libc::c_char as libc::c_uchar;
        *((*tree).orig_prim).offset((m + j) as isize) = (*col).prim;
        *((*tree).orig_dual).offset((m + j) as isize) = (*col).dual;
        j += 1;
        j;
    }
    (*tree).orig_obj = (*mip).obj_val;
    (*tree).nslots = 0 as libc::c_int;
    (*tree).avail = 0 as libc::c_int;
    (*tree).slot = 0 as *mut IOSLOT;
    (*tree).tail = 0 as *mut IOSNPD;
    (*tree).head = (*tree).tail;
    (*tree).t_cnt = 0 as libc::c_int;
    (*tree).n_cnt = (*tree).t_cnt;
    (*tree).a_cnt = (*tree).n_cnt;
    (*tree).root_m = 0 as libc::c_int;
    (*tree).root_type = 0 as *mut libc::c_uchar;
    (*tree).root_ub = 0 as *mut libc::c_double;
    (*tree).root_lb = (*tree).root_ub;
    (*tree).root_stat = 0 as *mut libc::c_uchar;
    (*tree).curr = 0 as *mut IOSNPD;
    (*tree).mip = mip;
    (*tree)
        .non_int = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_uchar;
    memset(
        &mut *((*tree).non_int).offset(1 as libc::c_int as isize) as *mut libc::c_uchar
            as *mut libc::c_void,
        0 as libc::c_int,
        n as libc::c_ulong,
    );
    (*tree).pred_max = 0 as libc::c_int;
    (*tree).pred_m = (*tree).pred_max;
    (*tree).pred_type = 0 as *mut libc::c_uchar;
    (*tree).pred_ub = 0 as *mut libc::c_double;
    (*tree).pred_lb = (*tree).pred_ub;
    (*tree).pred_stat = 0 as *mut libc::c_uchar;
    (*tree).local = _glp_ios_create_pool(tree);
    (*tree).cov_gen = 0 as *mut glp_cov;
    (*tree).mir_gen = 0 as *mut glp_mir;
    (*tree).clq_gen = 0 as *mut glp_cfg;
    (*tree).pcost = 0 as *mut libc::c_void;
    (*tree)
        .iwrk = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*tree)
        .dwrk = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*tree).parm = parm;
    (*tree).tm_beg = glp_time();
    (*tree).tm_lag = 0.0f64;
    (*tree).sol_cnt = 0 as libc::c_int;
    (*tree).P = 0 as *mut libc::c_void;
    (*tree).npp = 0 as *mut libc::c_void;
    (*tree).save_sol = (*parm).save_sol;
    (*tree).save_cnt = 0 as libc::c_int;
    (*tree).reason = 0 as libc::c_int;
    (*tree).reopt = 0 as libc::c_int;
    (*tree).reinv = 0 as libc::c_int;
    (*tree).br_var = 0 as libc::c_int;
    (*tree).br_sel = 0 as libc::c_int;
    (*tree).child = 0 as libc::c_int;
    (*tree).next_p = 0 as libc::c_int;
    (*tree).stop = 0 as libc::c_int;
    new_node(tree, 0 as *mut IOSNPD);
    return tree;
}
pub unsafe extern "C" fn _glp_ios_revive_node(
    mut tree: *mut glp_tree,
    mut p: libc::c_int,
) {
    let mut mip: *mut glp_prob = (*tree).mip;
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    let mut root: *mut IOSNPD = 0 as *mut IOSNPD;
    (1 as libc::c_int <= p && p <= (*tree).nslots
        || {
            glp_assert_(
                b"1 <= p && p <= tree->nslots\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                204 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    node = (*((*tree).slot).offset(p as isize)).node;
    (!node.is_null()
        || {
            glp_assert_(
                b"node != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                206 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*node).count == 0 as libc::c_int
        || {
            glp_assert_(
                b"node->count == 0\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                208 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (((*tree).curr).is_null()
        || {
            glp_assert_(
                b"tree->curr == NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                210 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*tree).curr = node;
    root = (*((*tree).slot).offset(1 as libc::c_int as isize)).node;
    (!root.is_null()
        || {
            glp_assert_(
                b"root != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                216 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if !(node == root) {
        ((*mip).m == (*tree).root_m
            || {
                glp_assert_(
                    b"mip->m == tree->root_m\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                    221 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*node).temp = 0 as *mut IOSNPD;
        node = node;
        while !node.is_null() {
            if ((*node).up).is_null() {
                (node == root
                    || {
                        glp_assert_(
                            b"node == root\0" as *const u8 as *const libc::c_char,
                            b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                            226 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            } else {
                (*(*node).up).temp = node;
            }
            node = (*node).up;
        }
        node = root;
        while !node.is_null() {
            let mut m: libc::c_int = (*mip).m;
            let mut n: libc::c_int = (*mip).n;
            if ((*node).temp).is_null() {
                let mut i: libc::c_int = 0;
                let mut j: libc::c_int = 0;
                (*tree).pred_m = m;
                if (*tree).pred_max < m + n {
                    let mut new_size: libc::c_int = m + n + 100 as libc::c_int;
                    if !((*tree).pred_type).is_null() {
                        glp_free((*tree).pred_type as *mut libc::c_void);
                    }
                    if !((*tree).pred_lb).is_null() {
                        glp_free((*tree).pred_lb as *mut libc::c_void);
                    }
                    if !((*tree).pred_ub).is_null() {
                        glp_free((*tree).pred_ub as *mut libc::c_void);
                    }
                    if !((*tree).pred_stat).is_null() {
                        glp_free((*tree).pred_stat as *mut libc::c_void);
                    }
                    (*tree).pred_max = new_size;
                    (*tree)
                        .pred_type = glp_alloc(
                        1 as libc::c_int + new_size,
                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                            as libc::c_int,
                    ) as *mut libc::c_uchar;
                    (*tree)
                        .pred_lb = glp_alloc(
                        1 as libc::c_int + new_size,
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                            as libc::c_int,
                    ) as *mut libc::c_double;
                    (*tree)
                        .pred_ub = glp_alloc(
                        1 as libc::c_int + new_size,
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                            as libc::c_int,
                    ) as *mut libc::c_double;
                    (*tree)
                        .pred_stat = glp_alloc(
                        1 as libc::c_int + new_size,
                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                            as libc::c_int,
                    ) as *mut libc::c_uchar;
                }
                i = 1 as libc::c_int;
                while i <= m {
                    let mut row: *mut GLPROW = *((*mip).row).offset(i as isize);
                    *((*tree).pred_type)
                        .offset(
                            i as isize,
                        ) = (*row).type_0 as libc::c_char as libc::c_uchar;
                    *((*tree).pred_lb).offset(i as isize) = (*row).lb;
                    *((*tree).pred_ub).offset(i as isize) = (*row).ub;
                    *((*tree).pred_stat)
                        .offset(
                            i as isize,
                        ) = (*row).stat as libc::c_char as libc::c_uchar;
                    i += 1;
                    i;
                }
                j = 1 as libc::c_int;
                while j <= n {
                    let mut col: *mut GLPCOL = *((*mip).col).offset(j as isize);
                    *((*tree).pred_type)
                        .offset(
                            ((*mip).m + j) as isize,
                        ) = (*col).type_0 as libc::c_char as libc::c_uchar;
                    *((*tree).pred_lb).offset(((*mip).m + j) as isize) = (*col).lb;
                    *((*tree).pred_ub).offset(((*mip).m + j) as isize) = (*col).ub;
                    *((*tree).pred_stat)
                        .offset(
                            ((*mip).m + j) as isize,
                        ) = (*col).stat as libc::c_char as libc::c_uchar;
                    j += 1;
                    j;
                }
            }
            let mut b: *mut IOSBND = 0 as *mut IOSBND;
            b = (*node).b_ptr;
            while !b.is_null() {
                if (*b).k <= m {
                    glp_set_row_bnds(
                        mip,
                        (*b).k,
                        (*b).type_0 as libc::c_int,
                        (*b).lb,
                        (*b).ub,
                    );
                } else {
                    glp_set_col_bnds(
                        mip,
                        (*b).k - m,
                        (*b).type_0 as libc::c_int,
                        (*b).lb,
                        (*b).ub,
                    );
                }
                b = (*b).next;
            }
            let mut s: *mut IOSTAT = 0 as *mut IOSTAT;
            s = (*node).s_ptr;
            while !s.is_null() {
                if (*s).k <= m {
                    glp_set_row_stat(mip, (*s).k, (*s).stat as libc::c_int);
                } else {
                    glp_set_col_stat(mip, (*s).k - m, (*s).stat as libc::c_int);
                }
                s = (*s).next;
            }
            if !((*node).r_ptr).is_null() {
                let mut r: *mut IOSROW = 0 as *mut IOSROW;
                let mut a: *mut IOSAIJ = 0 as *mut IOSAIJ;
                let mut i_0: libc::c_int = 0;
                let mut len: libc::c_int = 0;
                let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
                let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
                ind = glp_alloc(
                    1 as libc::c_int + n,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
                ) as *mut libc::c_int;
                val = glp_alloc(
                    1 as libc::c_int + n,
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                        as libc::c_int,
                ) as *mut libc::c_double;
                r = (*node).r_ptr;
                while !r.is_null() {
                    i_0 = glp_add_rows(mip, 1 as libc::c_int);
                    glp_set_row_name(mip, i_0, (*r).name);
                    ((**((*mip).row).offset(i_0 as isize)).level == 0 as libc::c_int
                        || {
                            glp_assert_(
                                b"mip->row[i]->level == 0\0" as *const u8
                                    as *const libc::c_char,
                                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                                301 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    (**((*mip).row).offset(i_0 as isize)).level = (*node).level;
                    (**((*mip).row).offset(i_0 as isize)).origin = (*r).origin;
                    (**((*mip).row).offset(i_0 as isize)).klass = (*r).klass;
                    glp_set_row_bnds(
                        mip,
                        i_0,
                        (*r).type_0 as libc::c_int,
                        (*r).lb,
                        (*r).ub,
                    );
                    len = 0 as libc::c_int;
                    a = (*r).ptr;
                    while !a.is_null() {
                        len += 1;
                        len;
                        *ind.offset(len as isize) = (*a).j;
                        *val.offset(len as isize) = (*a).val;
                        a = (*a).next;
                    }
                    glp_set_mat_row(
                        mip,
                        i_0,
                        len,
                        ind as *const libc::c_int,
                        val as *const libc::c_double,
                    );
                    glp_set_rii(mip, i_0, (*r).rii);
                    glp_set_row_stat(mip, i_0, (*r).stat as libc::c_int);
                    r = (*r).next;
                }
                glp_free(ind as *mut libc::c_void);
                glp_free(val as *mut libc::c_void);
            }
            node = (*node).temp;
        }
        node = (*tree).curr;
        while !((*node).b_ptr).is_null() {
            let mut b_0: *mut IOSBND = 0 as *mut IOSBND;
            b_0 = (*node).b_ptr;
            (*node).b_ptr = (*b_0).next;
            _glp_dmp_free_atom(
                (*tree).pool,
                b_0 as *mut libc::c_void,
                ::std::mem::size_of::<IOSBND>() as libc::c_ulong as libc::c_int,
            );
        }
        while !((*node).s_ptr).is_null() {
            let mut s_0: *mut IOSTAT = 0 as *mut IOSTAT;
            s_0 = (*node).s_ptr;
            (*node).s_ptr = (*s_0).next;
            _glp_dmp_free_atom(
                (*tree).pool,
                s_0 as *mut libc::c_void,
                ::std::mem::size_of::<IOSTAT>() as libc::c_ulong as libc::c_int,
            );
        }
        while !((*node).r_ptr).is_null() {
            let mut r_0: *mut IOSROW = 0 as *mut IOSROW;
            r_0 = (*node).r_ptr;
            (*node).r_ptr = (*r_0).next;
            (((*r_0).name).is_null()
                || {
                    glp_assert_(
                        b"r->name == NULL\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                        349 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            while !((*r_0).ptr).is_null() {
                let mut a_0: *mut IOSAIJ = 0 as *mut IOSAIJ;
                a_0 = (*r_0).ptr;
                (*r_0).ptr = (*a_0).next;
                _glp_dmp_free_atom(
                    (*tree).pool,
                    a_0 as *mut libc::c_void,
                    ::std::mem::size_of::<IOSAIJ>() as libc::c_ulong as libc::c_int,
                );
            }
            _glp_dmp_free_atom(
                (*tree).pool,
                r_0 as *mut libc::c_void,
                ::std::mem::size_of::<IOSROW>() as libc::c_ulong as libc::c_int,
            );
        }
    }
}
pub unsafe extern "C" fn _glp_ios_freeze_node(mut tree: *mut glp_tree) {
    let mut mip: *mut glp_prob = (*tree).mip;
    let mut m: libc::c_int = (*mip).m;
    let mut n: libc::c_int = (*mip).n;
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    node = (*tree).curr;
    (!node.is_null()
        || {
            glp_assert_(
                b"node != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                383 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if ((*node).up).is_null() {
        let mut k: libc::c_int = 0;
        ((*node).p == 1 as libc::c_int
            || {
                glp_assert_(
                    b"node->p == 1\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                    387 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        ((*tree).root_m == 0 as libc::c_int
            || {
                glp_assert_(
                    b"tree->root_m == 0\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                    388 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (((*tree).root_type).is_null()
            || {
                glp_assert_(
                    b"tree->root_type == NULL\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                    389 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (((*tree).root_lb).is_null()
            || {
                glp_assert_(
                    b"tree->root_lb == NULL\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                    390 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (((*tree).root_ub).is_null()
            || {
                glp_assert_(
                    b"tree->root_ub == NULL\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                    391 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (((*tree).root_stat).is_null()
            || {
                glp_assert_(
                    b"tree->root_stat == NULL\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                    392 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*tree).root_m = m;
        (*tree)
            .root_type = glp_alloc(
            1 as libc::c_int + m + n,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_uchar;
        (*tree)
            .root_lb = glp_alloc(
            1 as libc::c_int + m + n,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        (*tree)
            .root_ub = glp_alloc(
            1 as libc::c_int + m + n,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        (*tree)
            .root_stat = glp_alloc(
            1 as libc::c_int + m + n,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_uchar;
        k = 1 as libc::c_int;
        while k <= m + n {
            if k <= m {
                let mut row: *mut GLPROW = *((*mip).row).offset(k as isize);
                *((*tree).root_type)
                    .offset(k as isize) = (*row).type_0 as libc::c_char as libc::c_uchar;
                *((*tree).root_lb).offset(k as isize) = (*row).lb;
                *((*tree).root_ub).offset(k as isize) = (*row).ub;
                *((*tree).root_stat)
                    .offset(k as isize) = (*row).stat as libc::c_char as libc::c_uchar;
            } else {
                let mut col: *mut GLPCOL = *((*mip).col).offset((k - m) as isize);
                *((*tree).root_type)
                    .offset(k as isize) = (*col).type_0 as libc::c_char as libc::c_uchar;
                *((*tree).root_lb).offset(k as isize) = (*col).lb;
                *((*tree).root_ub).offset(k as isize) = (*col).ub;
                *((*tree).root_stat)
                    .offset(k as isize) = (*col).stat as libc::c_char as libc::c_uchar;
            }
            k += 1;
            k;
        }
    } else {
        let mut root_m: libc::c_int = (*tree).root_m;
        let mut pred_m: libc::c_int = (*tree).pred_m;
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut k_0: libc::c_int = 0;
        (pred_m <= m
            || {
                glp_assert_(
                    b"pred_m <= m\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                    420 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (((*node).b_ptr).is_null()
            || {
                glp_assert_(
                    b"node->b_ptr == NULL\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                    423 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (((*node).s_ptr).is_null()
            || {
                glp_assert_(
                    b"node->s_ptr == NULL\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                    424 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        k_0 = 1 as libc::c_int;
        while k_0 <= pred_m + n {
            let mut pred_type: libc::c_int = 0;
            let mut pred_stat: libc::c_int = 0;
            let mut type_0: libc::c_int = 0;
            let mut stat: libc::c_int = 0;
            let mut pred_lb: libc::c_double = 0.;
            let mut pred_ub: libc::c_double = 0.;
            let mut lb: libc::c_double = 0.;
            let mut ub: libc::c_double = 0.;
            pred_type = *((*tree).pred_type).offset(k_0 as isize) as libc::c_int;
            pred_lb = *((*tree).pred_lb).offset(k_0 as isize);
            pred_ub = *((*tree).pred_ub).offset(k_0 as isize);
            pred_stat = *((*tree).pred_stat).offset(k_0 as isize) as libc::c_int;
            if k_0 <= pred_m {
                let mut row_0: *mut GLPROW = *((*mip).row).offset(k_0 as isize);
                type_0 = (*row_0).type_0;
                lb = (*row_0).lb;
                ub = (*row_0).ub;
                stat = (*row_0).stat;
            } else {
                let mut col_0: *mut GLPCOL = *((*mip).col)
                    .offset((k_0 - pred_m) as isize);
                type_0 = (*col_0).type_0;
                lb = (*col_0).lb;
                ub = (*col_0).ub;
                stat = (*col_0).stat;
            }
            if !(pred_type == type_0 && pred_lb == lb && pred_ub == ub) {
                let mut b: *mut IOSBND = 0 as *mut IOSBND;
                b = _glp_dmp_get_atom(
                    (*tree).pool,
                    ::std::mem::size_of::<IOSBND>() as libc::c_ulong as libc::c_int,
                ) as *mut IOSBND;
                (*b).k = k_0;
                (*b).type_0 = type_0 as libc::c_uchar;
                (*b).lb = lb;
                (*b).ub = ub;
                (*b).next = (*node).b_ptr;
                (*node).b_ptr = b;
            }
            if pred_stat != stat {
                let mut s: *mut IOSTAT = 0 as *mut IOSTAT;
                s = _glp_dmp_get_atom(
                    (*tree).pool,
                    ::std::mem::size_of::<IOSTAT>() as libc::c_ulong as libc::c_int,
                ) as *mut IOSTAT;
                (*s).k = k_0;
                (*s).stat = stat as libc::c_uchar;
                (*s).next = (*node).s_ptr;
                (*node).s_ptr = s;
            }
            k_0 += 1;
            k_0;
        }
        (((*node).r_ptr).is_null()
            || {
                glp_assert_(
                    b"node->r_ptr == NULL\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                    470 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if pred_m < m {
            let mut i_0: libc::c_int = 0;
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
            i_0 = m;
            while i_0 > pred_m {
                let mut row_1: *mut GLPROW = *((*mip).row).offset(i_0 as isize);
                let mut r: *mut IOSROW = 0 as *mut IOSROW;
                let mut name: *const libc::c_char = 0 as *const libc::c_char;
                r = _glp_dmp_get_atom(
                    (*tree).pool,
                    ::std::mem::size_of::<IOSROW>() as libc::c_ulong as libc::c_int,
                ) as *mut IOSROW;
                name = glp_get_row_name(mip, i_0);
                if name.is_null() {
                    (*r).name = 0 as *mut libc::c_char;
                } else {
                    (*r)
                        .name = _glp_dmp_get_atom(
                        (*tree).pool,
                        (strlen(name)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as libc::c_int,
                    ) as *mut libc::c_char;
                    strcpy((*r).name, name);
                }
                (*r).origin = (*row_1).origin;
                (*r).klass = (*row_1).klass;
                (*r).type_0 = (*row_1).type_0 as libc::c_uchar;
                (*r).lb = (*row_1).lb;
                (*r).ub = (*row_1).ub;
                (*r).ptr = 0 as *mut IOSAIJ;
                len = glp_get_mat_row(mip, i_0, ind, val);
                k_0 = 1 as libc::c_int;
                while k_0 <= len {
                    let mut a: *mut IOSAIJ = 0 as *mut IOSAIJ;
                    a = _glp_dmp_get_atom(
                        (*tree).pool,
                        ::std::mem::size_of::<IOSAIJ>() as libc::c_ulong as libc::c_int,
                    ) as *mut IOSAIJ;
                    (*a).j = *ind.offset(k_0 as isize);
                    (*a).val = *val.offset(k_0 as isize);
                    (*a).next = (*r).ptr;
                    (*r).ptr = a;
                    k_0 += 1;
                    k_0;
                }
                (*r).rii = (*row_1).rii;
                (*r).stat = (*row_1).stat as libc::c_uchar;
                (*r).next = (*node).r_ptr;
                (*node).r_ptr = r;
                i_0 -= 1;
                i_0;
            }
            glp_free(ind as *mut libc::c_void);
            glp_free(val as *mut libc::c_void);
        }
        if m != root_m {
            let mut nrs: libc::c_int = 0;
            let mut num: *mut libc::c_int = 0 as *mut libc::c_int;
            nrs = m - root_m;
            (nrs > 0 as libc::c_int
                || {
                    glp_assert_(
                        b"nrs > 0\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                        517 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            num = glp_alloc(
                1 as libc::c_int + nrs,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
            ) as *mut libc::c_int;
            i = 1 as libc::c_int;
            while i <= nrs {
                *num.offset(i as isize) = root_m + i;
                i += 1;
                i;
            }
            glp_del_rows(mip, nrs, num as *const libc::c_int);
            glp_free(num as *mut libc::c_void);
        }
        m = (*mip).m;
        (m == root_m
            || {
                glp_assert_(
                    b"m == root_m\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                    526 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        i = 1 as libc::c_int;
        while i <= m {
            glp_set_row_bnds(
                mip,
                i,
                *((*tree).root_type).offset(i as isize) as libc::c_int,
                *((*tree).root_lb).offset(i as isize),
                *((*tree).root_ub).offset(i as isize),
            );
            glp_set_row_stat(
                mip,
                i,
                *((*tree).root_stat).offset(i as isize) as libc::c_int,
            );
            i += 1;
            i;
        }
        j = 1 as libc::c_int;
        while j <= n {
            glp_set_col_bnds(
                mip,
                j,
                *((*tree).root_type).offset((m + j) as isize) as libc::c_int,
                *((*tree).root_lb).offset((m + j) as isize),
                *((*tree).root_ub).offset((m + j) as isize),
            );
            glp_set_col_stat(
                mip,
                j,
                *((*tree).root_stat).offset((m + j) as isize) as libc::c_int,
            );
            j += 1;
            j;
        }
    }
    (*tree).curr = 0 as *mut IOSNPD;
}
unsafe extern "C" fn get_slot(mut tree: *mut glp_tree) -> libc::c_int {
    let mut p: libc::c_int = 0;
    if (*tree).avail == 0 as libc::c_int {
        let mut nslots: libc::c_int = (*tree).nslots;
        let mut save: *mut IOSLOT = (*tree).slot;
        if nslots == 0 as libc::c_int {
            (*tree).nslots = 20 as libc::c_int;
        } else {
            (*tree).nslots = nslots + nslots;
            ((*tree).nslots > nslots
                || {
                    glp_assert_(
                        b"tree->nslots > nslots\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                        582 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        (*tree)
            .slot = glp_alloc(
            1 as libc::c_int + (*tree).nslots,
            ::std::mem::size_of::<IOSLOT>() as libc::c_ulong as libc::c_int,
        ) as *mut IOSLOT;
        if !save.is_null() {
            memcpy(
                &mut *((*tree).slot).offset(1 as libc::c_int as isize) as *mut IOSLOT
                    as *mut libc::c_void,
                &mut *save.offset(1 as libc::c_int as isize) as *mut IOSLOT
                    as *const libc::c_void,
                (nslots as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<IOSLOT>() as libc::c_ulong),
            );
            glp_free(save as *mut libc::c_void);
        }
        p = (*tree).nslots;
        while p > nslots {
            let ref mut fresh0 = (*((*tree).slot).offset(p as isize)).node;
            *fresh0 = 0 as *mut IOSNPD;
            (*((*tree).slot).offset(p as isize)).next = (*tree).avail;
            (*tree).avail = p;
            p -= 1;
            p;
        }
    }
    p = (*tree).avail;
    (*tree).avail = (*((*tree).slot).offset(p as isize)).next;
    (((*((*tree).slot).offset(p as isize)).node).is_null()
        || {
            glp_assert_(
                b"tree->slot[p].node == NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                599 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*((*tree).slot).offset(p as isize)).next = 0 as libc::c_int;
    return p;
}
unsafe extern "C" fn new_node(
    mut tree: *mut glp_tree,
    mut parent: *mut IOSNPD,
) -> *mut IOSNPD {
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    let mut p: libc::c_int = 0;
    p = get_slot(tree);
    node = _glp_dmp_get_atom(
        (*tree).pool,
        ::std::mem::size_of::<IOSNPD>() as libc::c_ulong as libc::c_int,
    ) as *mut IOSNPD;
    let ref mut fresh1 = (*((*tree).slot).offset(p as isize)).node;
    *fresh1 = node;
    (*node).p = p;
    (*node).up = parent;
    (*node)
        .level = if parent.is_null() {
        0 as libc::c_int
    } else {
        (*parent).level + 1 as libc::c_int
    };
    (*node).count = 0 as libc::c_int;
    (*node).b_ptr = 0 as *mut IOSBND;
    (*node).s_ptr = 0 as *mut IOSTAT;
    (*node).r_ptr = 0 as *mut IOSROW;
    (*node).solved = 0 as libc::c_int;
    (*node)
        .lp_obj = if parent.is_null() {
        if (*(*tree).mip).dir == 1 as libc::c_int {
            -1.7976931348623157e+308f64
        } else {
            1.7976931348623157e+308f64
        }
    } else {
        (*parent).lp_obj
    };
    (*node)
        .bound = if parent.is_null() {
        if (*(*tree).mip).dir == 1 as libc::c_int {
            -1.7976931348623157e+308f64
        } else {
            1.7976931348623157e+308f64
        }
    } else {
        (*parent).bound
    };
    (*node).br_var = 0 as libc::c_int;
    (*node).br_val = 0.0f64;
    (*node).ii_cnt = 0 as libc::c_int;
    (*node).ii_sum = 0.0f64;
    (*node).changed = 0 as libc::c_int;
    if (*(*tree).parm).cb_size == 0 as libc::c_int {
        (*node).data = 0 as *mut libc::c_void;
    } else {
        (*node).data = _glp_dmp_get_atom((*tree).pool, (*(*tree).parm).cb_size);
        memset((*node).data, 0 as libc::c_int, (*(*tree).parm).cb_size as libc::c_ulong);
    }
    (*node).temp = 0 as *mut IOSNPD;
    (*node).prev = (*tree).tail;
    (*node).next = 0 as *mut IOSNPD;
    if ((*tree).head).is_null() {
        (*tree).head = node;
    } else {
        (*(*tree).tail).next = node;
    }
    (*tree).tail = node;
    (*tree).a_cnt += 1;
    (*tree).a_cnt;
    (*tree).n_cnt += 1;
    (*tree).n_cnt;
    (*tree).t_cnt += 1;
    (*tree).t_cnt;
    if parent.is_null() {
        (p == 1 as libc::c_int
            || {
                glp_assert_(
                    b"p == 1\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                    657 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    } else {
        (*parent).count += 1;
        (*parent).count;
    }
    return node;
}
pub unsafe extern "C" fn _glp_ios_clone_node(
    mut tree: *mut glp_tree,
    mut p: libc::c_int,
    mut nnn: libc::c_int,
    mut ref_0: *mut libc::c_int,
) {
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    let mut k: libc::c_int = 0;
    (1 as libc::c_int <= p && p <= (*tree).nslots
        || {
            glp_assert_(
                b"1 <= p && p <= tree->nslots\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                667 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    node = (*((*tree).slot).offset(p as isize)).node;
    (!node.is_null()
        || {
            glp_assert_(
                b"node != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                669 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*node).count == 0 as libc::c_int
        || {
            glp_assert_(
                b"node->count == 0\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                671 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*tree).curr != node
        || {
            glp_assert_(
                b"tree->curr != node\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                673 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if ((*node).prev).is_null() {
        (*tree).head = (*node).next;
    } else {
        (*(*node).prev).next = (*node).next;
    }
    if ((*node).next).is_null() {
        (*tree).tail = (*node).prev;
    } else {
        (*(*node).next).prev = (*node).prev;
    }
    (*node).next = 0 as *mut IOSNPD;
    (*node).prev = (*node).next;
    (*tree).a_cnt -= 1;
    (*tree).a_cnt;
    (nnn > 0 as libc::c_int
        || {
            glp_assert_(
                b"nnn > 0\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                687 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = 1 as libc::c_int;
    while k <= nnn {
        *ref_0.offset(k as isize) = (*new_node(tree, node)).p;
        k += 1;
        k;
    }
}
pub unsafe extern "C" fn _glp_ios_delete_node(
    mut tree: *mut glp_tree,
    mut p: libc::c_int,
) {
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    let mut temp: *mut IOSNPD = 0 as *mut IOSNPD;
    (1 as libc::c_int <= p && p <= (*tree).nslots
        || {
            glp_assert_(
                b"1 <= p && p <= tree->nslots\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                716 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    node = (*((*tree).slot).offset(p as isize)).node;
    (!node.is_null()
        || {
            glp_assert_(
                b"node != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                718 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*node).count == 0 as libc::c_int
        || {
            glp_assert_(
                b"node->count == 0\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                720 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*tree).curr != node
        || {
            glp_assert_(
                b"tree->curr != node\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                722 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if ((*node).prev).is_null() {
        (*tree).head = (*node).next;
    } else {
        (*(*node).prev).next = (*node).next;
    }
    if ((*node).next).is_null() {
        (*tree).tail = (*node).prev;
    } else {
        (*(*node).next).prev = (*node).prev;
    }
    (*node).next = 0 as *mut IOSNPD;
    (*node).prev = (*node).next;
    (*tree).a_cnt -= 1;
    (*tree).a_cnt;
    loop {
        let mut b: *mut IOSBND = 0 as *mut IOSBND;
        while !((*node).b_ptr).is_null() {
            b = (*node).b_ptr;
            (*node).b_ptr = (*b).next;
            _glp_dmp_free_atom(
                (*tree).pool,
                b as *mut libc::c_void,
                ::std::mem::size_of::<IOSBND>() as libc::c_ulong as libc::c_int,
            );
        }
        let mut s: *mut IOSTAT = 0 as *mut IOSTAT;
        while !((*node).s_ptr).is_null() {
            s = (*node).s_ptr;
            (*node).s_ptr = (*s).next;
            _glp_dmp_free_atom(
                (*tree).pool,
                s as *mut libc::c_void,
                ::std::mem::size_of::<IOSTAT>() as libc::c_ulong as libc::c_int,
            );
        }
        while !((*node).r_ptr).is_null() {
            let mut r: *mut IOSROW = 0 as *mut IOSROW;
            r = (*node).r_ptr;
            if !((*r).name).is_null() {
                _glp_dmp_free_atom(
                    (*tree).pool,
                    (*r).name as *mut libc::c_void,
                    (strlen((*r).name)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as libc::c_int,
                );
            }
            while !((*r).ptr).is_null() {
                let mut a: *mut IOSAIJ = 0 as *mut IOSAIJ;
                a = (*r).ptr;
                (*r).ptr = (*a).next;
                _glp_dmp_free_atom(
                    (*tree).pool,
                    a as *mut libc::c_void,
                    ::std::mem::size_of::<IOSAIJ>() as libc::c_ulong as libc::c_int,
                );
            }
            (*node).r_ptr = (*r).next;
            _glp_dmp_free_atom(
                (*tree).pool,
                r as *mut libc::c_void,
                ::std::mem::size_of::<IOSROW>() as libc::c_ulong as libc::c_int,
            );
        }
        if (*(*tree).parm).cb_size == 0 as libc::c_int {
            (((*node).data).is_null()
                || {
                    glp_assert_(
                        b"node->data == NULL\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                        777 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        } else {
            _glp_dmp_free_atom((*tree).pool, (*node).data, (*(*tree).parm).cb_size);
        }
        p = (*node).p;
        ((*((*tree).slot).offset(p as isize)).node == node
            || {
                glp_assert_(
                    b"tree->slot[p].node == node\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                    782 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        let ref mut fresh2 = (*((*tree).slot).offset(p as isize)).node;
        *fresh2 = 0 as *mut IOSNPD;
        (*((*tree).slot).offset(p as isize)).next = (*tree).avail;
        (*tree).avail = p;
        temp = (*node).up;
        _glp_dmp_free_atom(
            (*tree).pool,
            node as *mut libc::c_void,
            ::std::mem::size_of::<IOSNPD>() as libc::c_ulong as libc::c_int,
        );
        (*tree).n_cnt -= 1;
        (*tree).n_cnt;
        node = temp;
        if node.is_null() {
            break;
        }
        ((*node).count > 0 as libc::c_int
            || {
                glp_assert_(
                    b"node->count > 0\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                    796 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*node).count -= 1;
        (*node).count;
        if !((*node).count == 0 as libc::c_int) {
            break;
        }
    };
}
pub unsafe extern "C" fn _glp_ios_delete_tree(mut tree: *mut glp_tree) {
    let mut mip: *mut glp_prob = (*tree).mip;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut m: libc::c_int = (*mip).m;
    let mut n: libc::c_int = (*mip).n;
    ((*mip).tree == tree
        || {
            glp_assert_(
                b"mip->tree == tree\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                829 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if m != (*tree).orig_m {
        let mut nrs: libc::c_int = 0;
        let mut num: *mut libc::c_int = 0 as *mut libc::c_int;
        nrs = m - (*tree).orig_m;
        (nrs > 0 as libc::c_int
            || {
                glp_assert_(
                    b"nrs > 0\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                    834 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        num = glp_alloc(
            1 as libc::c_int + nrs,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        i = 1 as libc::c_int;
        while i <= nrs {
            *num.offset(i as isize) = (*tree).orig_m + i;
            i += 1;
            i;
        }
        glp_del_rows(mip, nrs, num as *const libc::c_int);
        glp_free(num as *mut libc::c_void);
    }
    m = (*tree).orig_m;
    (m == (*tree).orig_m
        || {
            glp_assert_(
                b"m == tree->orig_m\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                842 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (n == (*tree).n
        || {
            glp_assert_(
                b"n == tree->n\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                843 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    i = 1 as libc::c_int;
    while i <= m {
        glp_set_row_bnds(
            mip,
            i,
            *((*tree).orig_type).offset(i as isize) as libc::c_int,
            *((*tree).orig_lb).offset(i as isize),
            *((*tree).orig_ub).offset(i as isize),
        );
        glp_set_row_stat(mip, i, *((*tree).orig_stat).offset(i as isize) as libc::c_int);
        (**((*mip).row).offset(i as isize))
            .prim = *((*tree).orig_prim).offset(i as isize);
        (**((*mip).row).offset(i as isize))
            .dual = *((*tree).orig_dual).offset(i as isize);
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= n {
        glp_set_col_bnds(
            mip,
            j,
            *((*tree).orig_type).offset((m + j) as isize) as libc::c_int,
            *((*tree).orig_lb).offset((m + j) as isize),
            *((*tree).orig_ub).offset((m + j) as isize),
        );
        glp_set_col_stat(
            mip,
            j,
            *((*tree).orig_stat).offset((m + j) as isize) as libc::c_int,
        );
        (**((*mip).col).offset(j as isize))
            .prim = *((*tree).orig_prim).offset((m + j) as isize);
        (**((*mip).col).offset(j as isize))
            .dual = *((*tree).orig_dual).offset((m + j) as isize);
        j += 1;
        j;
    }
    (*mip).dbs_stat = 2 as libc::c_int;
    (*mip).pbs_stat = (*mip).dbs_stat;
    (*mip).obj_val = (*tree).orig_obj;
    (!((*tree).local).is_null()
        || {
            glp_assert_(
                b"tree->local != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                861 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_ios_delete_pool(tree, (*tree).local);
    _glp_dmp_delete_pool((*tree).pool);
    glp_free((*tree).orig_type as *mut libc::c_void);
    glp_free((*tree).orig_lb as *mut libc::c_void);
    glp_free((*tree).orig_ub as *mut libc::c_void);
    glp_free((*tree).orig_stat as *mut libc::c_void);
    glp_free((*tree).orig_prim as *mut libc::c_void);
    glp_free((*tree).orig_dual as *mut libc::c_void);
    glp_free((*tree).slot as *mut libc::c_void);
    if !((*tree).root_type).is_null() {
        glp_free((*tree).root_type as *mut libc::c_void);
    }
    if !((*tree).root_lb).is_null() {
        glp_free((*tree).root_lb as *mut libc::c_void);
    }
    if !((*tree).root_ub).is_null() {
        glp_free((*tree).root_ub as *mut libc::c_void);
    }
    if !((*tree).root_stat).is_null() {
        glp_free((*tree).root_stat as *mut libc::c_void);
    }
    glp_free((*tree).non_int as *mut libc::c_void);
    if !((*tree).pcost).is_null() {
        _glp_ios_pcost_free(tree);
    }
    glp_free((*tree).iwrk as *mut libc::c_void);
    glp_free((*tree).dwrk as *mut libc::c_void);
    if !((*tree).pred_type).is_null() {
        glp_free((*tree).pred_type as *mut libc::c_void);
    }
    if !((*tree).pred_lb).is_null() {
        glp_free((*tree).pred_lb as *mut libc::c_void);
    }
    if !((*tree).pred_ub).is_null() {
        glp_free((*tree).pred_ub as *mut libc::c_void);
    }
    if !((*tree).pred_stat).is_null() {
        glp_free((*tree).pred_stat as *mut libc::c_void);
    }
    (((*tree).mir_gen).is_null()
        || {
            glp_assert_(
                b"tree->mir_gen == NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                894 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (((*tree).clq_gen).is_null()
        || {
            glp_assert_(
                b"tree->clq_gen == NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                895 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    glp_free(tree as *mut libc::c_void);
    (*mip).tree = 0 as *mut glp_tree;
}
pub unsafe extern "C" fn _glp_ios_eval_degrad(
    mut tree: *mut glp_tree,
    mut j: libc::c_int,
    mut dn: *mut libc::c_double,
    mut up: *mut libc::c_double,
) {
    let mut mip: *mut glp_prob = (*tree).mip;
    let mut m: libc::c_int = (*mip).m;
    let mut n: libc::c_int = (*mip).n;
    let mut len: libc::c_int = 0;
    let mut kase: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut stat: libc::c_int = 0;
    let mut alfa: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut gamma: libc::c_double = 0.;
    let mut delta: libc::c_double = 0.;
    let mut dz: libc::c_double = 0.;
    let mut ind: *mut libc::c_int = (*tree).iwrk;
    let mut val: *mut libc::c_double = (*tree).dwrk;
    (glp_get_status(mip) == 5 as libc::c_int
        || {
            glp_assert_(
                b"glp_get_status(mip) == GLP_OPT\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                927 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (glp_bf_exists(mip) != 0
        || {
            glp_assert_(
                b"glp_bf_exists(mip)\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                929 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= j && j <= n
        || {
            glp_assert_(
                b"1 <= j && j <= n\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                932 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    beta = (**((*mip).col).offset(j as isize)).prim;
    len = lpx_eval_tab_row(mip, m + j, ind, val);
    kase = -(1 as libc::c_int);
    while kase <= 1 as libc::c_int {
        k = lpx_dual_ratio_test(
            mip,
            len,
            ind as *const libc::c_int,
            val as *const libc::c_double,
            kase,
            1e-9f64,
        );
        if k == 0 as libc::c_int {
            if (*mip).dir == 1 as libc::c_int {
                if kase < 0 as libc::c_int {
                    *dn = 1.7976931348623157e+308f64;
                } else {
                    *up = 1.7976931348623157e+308f64;
                }
            } else if (*mip).dir == 2 as libc::c_int {
                if kase < 0 as libc::c_int {
                    *dn = -1.7976931348623157e+308f64;
                } else {
                    *up = -1.7976931348623157e+308f64;
                }
            } else {
                (mip != mip
                    || {
                        glp_assert_(
                            b"mip != mip\0" as *const u8 as *const libc::c_char,
                            b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                            969 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        } else {
            (1 as libc::c_int <= k && k <= m + n
                || {
                    glp_assert_(
                        b"1 <= k && k <= m+n\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                        972 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
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
                        b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                        980 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            alfa = *val.offset(t as isize);
            if k <= m {
                stat = (**((*mip).row).offset(k as isize)).stat;
                gamma = (**((*mip).row).offset(k as isize)).dual;
            } else {
                stat = (**((*mip).col).offset((k - m) as isize)).stat;
                gamma = (**((*mip).col).offset((k - m) as isize)).dual;
            }
            (stat == 2 as libc::c_int || stat == 3 as libc::c_int
                || stat == 4 as libc::c_int
                || {
                    glp_assert_(
                        b"stat == GLP_NL || stat == GLP_NU || stat == GLP_NF\0"
                            as *const u8 as *const libc::c_char,
                        b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                        992 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if (*mip).dir == 1 as libc::c_int {
                if stat == 2 as libc::c_int && gamma < 0.0f64
                    || stat == 3 as libc::c_int && gamma > 0.0f64
                    || stat == 4 as libc::c_int
                {
                    gamma = 0.0f64;
                }
            } else if (*mip).dir == 2 as libc::c_int {
                if stat == 2 as libc::c_int && gamma > 0.0f64
                    || stat == 3 as libc::c_int && gamma < 0.0f64
                    || stat == 4 as libc::c_int
                {
                    gamma = 0.0f64;
                }
            } else {
                (mip != mip
                    || {
                        glp_assert_(
                            b"mip != mip\0" as *const u8 as *const libc::c_char,
                            b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                            1007 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            delta = (if kase < 0 as libc::c_int { floor(beta) } else { ceil(beta) })
                - beta;
            delta /= alfa;
            dz = gamma * delta;
            if (*mip).dir == 1 as libc::c_int {
                (dz >= 0.0f64
                    || {
                        glp_assert_(
                            b"dz >= 0.0\0" as *const u8 as *const libc::c_char,
                            b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                            1018 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            } else if (*mip).dir == 2 as libc::c_int {
                (dz <= 0.0f64
                    || {
                        glp_assert_(
                            b"dz <= 0.0\0" as *const u8 as *const libc::c_char,
                            b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                            1020 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            } else {
                (mip != mip
                    || {
                        glp_assert_(
                            b"mip != mip\0" as *const u8 as *const libc::c_char,
                            b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                            1022 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            if kase < 0 as libc::c_int {
                *dn = (*mip).obj_val + dz;
            } else {
                *up = (*mip).obj_val + dz;
            }
        }
        kase += 2 as libc::c_int;
    }
}
pub unsafe extern "C" fn _glp_ios_round_bound(
    mut tree: *mut glp_tree,
    mut bound: libc::c_double,
) -> libc::c_double {
    let mut current_block: u64;
    let mut mip: *mut glp_prob = (*tree).mip;
    let mut n: libc::c_int = (*mip).n;
    let mut d: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nn: libc::c_int = 0;
    let mut c: *mut libc::c_int = (*tree).iwrk;
    let mut s: libc::c_double = 0.;
    let mut h: libc::c_double = 0.;
    nn = 0 as libc::c_int;
    s = (*mip).c0;
    d = 0 as libc::c_int;
    j = 1 as libc::c_int;
    loop {
        if !(j <= n) {
            current_block = 12800627514080957624;
            break;
        }
        let mut col: *mut GLPCOL = *((*mip).col).offset(j as isize);
        if !((*col).coef == 0.0f64) {
            if (*col).type_0 == 5 as libc::c_int {
                s += (*col).coef * (*col).prim;
            } else {
                if (*col).kind != 2 as libc::c_int {
                    current_block = 16194969828456462931;
                    break;
                }
                if (*col).coef != floor((*col).coef) {
                    current_block = 16194969828456462931;
                    break;
                }
                if fabs((*col).coef) <= 2147483647 as libc::c_int as libc::c_double {
                    nn += 1;
                    *c.offset(nn as isize) = fabs((*col).coef) as libc::c_int;
                } else {
                    d = 1 as libc::c_int;
                }
            }
        }
        j += 1;
        j;
    }
    match current_block {
        12800627514080957624 => {
            if d == 0 as libc::c_int {
                if nn == 0 as libc::c_int {
                    current_block = 16194969828456462931;
                } else {
                    d = _glp_gcdn(nn, c);
                    current_block = 12039483399334584727;
                }
            } else {
                current_block = 12039483399334584727;
            }
            match current_block {
                16194969828456462931 => {}
                _ => {
                    (d > 0 as libc::c_int
                        || {
                            glp_assert_(
                                b"d > 0\0" as *const u8 as *const libc::c_char,
                                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                                1120 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    if (*mip).dir == 1 as libc::c_int {
                        if bound != 1.7976931348623157e+308f64 {
                            h = (bound - s) / d as libc::c_double;
                            if h >= floor(h) + 0.001f64 {
                                h = ceil(h);
                                bound = d as libc::c_double * h + s;
                            }
                        }
                    } else if (*mip).dir == 2 as libc::c_int {
                        if bound != -1.7976931348623157e+308f64 {
                            h = (bound - s) / d as libc::c_double;
                            if h <= ceil(h) - 0.001f64 {
                                h = floor(h);
                                bound = d as libc::c_double * h + s;
                            }
                        }
                    } else {
                        (mip != mip
                            || {
                                glp_assert_(
                                    b"mip != mip\0" as *const u8 as *const libc::c_char,
                                    b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                                    1145 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                    }
                }
            }
        }
        _ => {}
    }
    return bound;
}
pub unsafe extern "C" fn _glp_ios_is_hopeful(
    mut tree: *mut glp_tree,
    mut bound: libc::c_double,
) -> libc::c_int {
    let mut mip: *mut glp_prob = (*tree).mip;
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut eps: libc::c_double = 0.;
    if (*mip).mip_stat == 2 as libc::c_int {
        eps = (*(*tree).parm).tol_obj * (1.0f64 + fabs((*mip).mip_obj));
        match (*mip).dir {
            1 => {
                if bound >= (*mip).mip_obj - eps {
                    ret = 0 as libc::c_int;
                }
            }
            2 => {
                if bound <= (*mip).mip_obj + eps {
                    ret = 0 as libc::c_int;
                }
            }
            _ => {
                (mip != mip
                    || {
                        glp_assert_(
                            b"mip != mip\0" as *const u8 as *const libc::c_char,
                            b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                            1185 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
    } else {
        match (*mip).dir {
            1 => {
                if bound == 1.7976931348623157e+308f64 {
                    ret = 0 as libc::c_int;
                }
            }
            2 => {
                if bound == -1.7976931348623157e+308f64 {
                    ret = 0 as libc::c_int;
                }
            }
            _ => {
                (mip != mip
                    || {
                        glp_assert_(
                            b"mip != mip\0" as *const u8 as *const libc::c_char,
                            b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                            1197 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
    }
    return ret;
}
pub unsafe extern "C" fn _glp_ios_best_node(mut tree: *mut glp_tree) -> libc::c_int {
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    let mut best: *mut IOSNPD = 0 as *mut IOSNPD;
    match (*(*tree).mip).dir {
        1 => {
            node = (*tree).head;
            while !node.is_null() {
                if best.is_null() || (*best).bound > (*node).bound {
                    best = node;
                }
                node = (*node).next;
            }
        }
        2 => {
            node = (*tree).head;
            while !node.is_null() {
                if best.is_null() || (*best).bound < (*node).bound {
                    best = node;
                }
                node = (*node).next;
            }
        }
        _ => {
            (tree != tree
                || {
                    glp_assert_(
                        b"tree != tree\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                        1244 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    return if best.is_null() { 0 as libc::c_int } else { (*best).p };
}
pub unsafe extern "C" fn _glp_ios_relative_gap(
    mut tree: *mut glp_tree,
) -> libc::c_double {
    let mut mip: *mut glp_prob = (*tree).mip;
    let mut p: libc::c_int = 0;
    let mut best_mip: libc::c_double = 0.;
    let mut best_bnd: libc::c_double = 0.;
    let mut gap: libc::c_double = 0.;
    if (*mip).mip_stat == 2 as libc::c_int {
        best_mip = (*mip).mip_obj;
        p = _glp_ios_best_node(tree);
        if p == 0 as libc::c_int {
            gap = 0.0f64;
        } else {
            best_bnd = (*(*((*tree).slot).offset(p as isize)).node).bound;
            gap = fabs(best_mip - best_bnd)
                / (fabs(best_mip) + 2.2204460492503131e-16f64);
        }
    } else {
        gap = 1.7976931348623157e+308f64;
    }
    return gap;
}
pub unsafe extern "C" fn _glp_ios_solve_node(mut tree: *mut glp_tree) -> libc::c_int {
    let mut mip: *mut glp_prob = (*tree).mip;
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
    (!((*tree).curr).is_null()
        || {
            glp_assert_(
                b"tree->curr != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                1322 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    glp_init_smcp(&mut parm);
    match (*(*tree).parm).msg_lev {
        0 => {
            parm.msg_lev = 0 as libc::c_int;
        }
        1 => {
            parm.msg_lev = 1 as libc::c_int;
        }
        2 | 3 => {
            parm.msg_lev = 2 as libc::c_int;
        }
        4 => {
            parm.msg_lev = 3 as libc::c_int;
        }
        _ => {
            (tree != tree
                || {
                    glp_assert_(
                        b"tree != tree\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                        1336 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    parm.meth = 2 as libc::c_int;
    if (*(*tree).parm).flip != 0 {
        parm.r_test = 0x33 as libc::c_int;
    }
    if (*(*tree).parm).tm_lim < 2147483647 as libc::c_int {
        parm
            .tm_lim = ((*(*tree).parm).tm_lim as libc::c_double
            - (glp_time() - (*tree).tm_beg)) as libc::c_int;
    }
    if parm.tm_lim < 0 as libc::c_int {
        parm.tm_lim = 0 as libc::c_int;
    }
    if (*(*tree).parm).msg_lev < 4 as libc::c_int {
        parm.out_dly = (*(*tree).parm).out_dly;
    } else {
        parm.out_dly = 0 as libc::c_int;
    }
    if (*mip).mip_stat == 2 as libc::c_int {
        match (*(*tree).mip).dir {
            1 => {
                parm.obj_ul = (*mip).mip_obj;
            }
            2 => {
                parm.obj_ll = (*mip).mip_obj;
            }
            _ => {
                (mip != mip
                    || {
                        glp_assert_(
                            b"mip != mip\0" as *const u8 as *const libc::c_char,
                            b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                            1363 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
    }
    ret = glp_simplex(mip, &mut parm);
    if ret == 0x5 as libc::c_int {
        glp_adv_basis(mip, 0 as libc::c_int);
        ret = glp_simplex(mip, &mut parm);
    }
    (*(*tree).curr).solved += 1;
    (*(*tree).curr).solved;
    return ret;
}
pub unsafe extern "C" fn _glp_ios_create_pool(mut tree: *mut glp_tree) -> *mut IOSPOOL {
    let mut pool: *mut IOSPOOL = 0 as *mut IOSPOOL;
    pool = glp_create_prob();
    if (*(*tree).mip).n != 0 {
        glp_add_cols(pool, (*(*tree).mip).n);
    }
    return pool;
}
pub unsafe extern "C" fn _glp_ios_add_row(
    mut tree: *mut glp_tree,
    mut pool: *mut IOSPOOL,
    mut name: *const libc::c_char,
    mut klass: libc::c_int,
    mut flags: libc::c_int,
    mut len: libc::c_int,
    mut ind: *const libc::c_int,
    mut val: *const libc::c_double,
    mut type_0: libc::c_int,
    mut rhs: libc::c_double,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = glp_add_rows(pool, 1 as libc::c_int);
    glp_set_row_name(pool, i, name);
    (**((*pool).row).offset(i as isize)).klass = klass as libc::c_uchar;
    (flags == 0 as libc::c_int
        || {
            glp_assert_(
                b"flags == 0\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                1424 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    glp_set_mat_row(pool, i, len, ind, val);
    glp_set_row_bnds(pool, i, type_0, rhs, rhs);
    return i;
}
pub unsafe extern "C" fn _glp_ios_find_row(
    mut pool: *mut IOSPOOL,
    mut i: libc::c_int,
) -> *mut IOSCUT {
    (0 as libc::c_int != 0
        || {
            glp_assert_(
                b"0\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                1493 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    panic!();
}
pub unsafe extern "C" fn _glp_ios_del_row(
    mut tree: *mut glp_tree,
    mut pool: *mut IOSPOOL,
    mut i: libc::c_int,
) {
    (0 as libc::c_int != 0
        || {
            glp_assert_(
                b"0\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                1552 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
}
pub unsafe extern "C" fn _glp_ios_clear_pool(
    mut tree: *mut glp_tree,
    mut pool: *mut IOSPOOL,
) {
    if (*pool).m > 0 as libc::c_int {
        let mut i: libc::c_int = 0;
        let mut num: *mut libc::c_int = 0 as *mut libc::c_int;
        num = glp_alloc(
            1 as libc::c_int + (*pool).m,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        i = 1 as libc::c_int;
        while i <= (*pool).m {
            *num.offset(i as isize) = i;
            i += 1;
            i;
        }
        glp_del_rows(pool, (*pool).m, num as *const libc::c_int);
        glp_free(num as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn _glp_ios_delete_pool(
    mut tree: *mut glp_tree,
    mut pool: *mut IOSPOOL,
) {
    (!pool.is_null()
        || {
            glp_assert_(
                b"pool != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                1639 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    glp_delete_prob(pool);
}
pub unsafe extern "C" fn _glp_ios_process_sol(mut T: *mut glp_tree) {
    if !((*T).npp).is_null() {
        _glp_npp_postprocess((*T).npp as *mut NPP, (*T).mip);
        _glp_npp_unload_sol((*T).npp as *mut NPP, (*T).P as *mut glp_prob);
    }
    (!((*T).P).is_null()
        || {
            glp_assert_(
                b"T->P != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios01.c\0" as *const u8 as *const libc::c_char,
                1664 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if !((*T).save_sol).is_null() {
        let mut fn_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut mark: *mut libc::c_char = 0 as *mut libc::c_char;
        fn_0 = glp_alloc(
            (strlen((*T).save_sol)).wrapping_add(50 as libc::c_int as libc::c_ulong)
                as libc::c_int,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_char;
        mark = strrchr((*T).save_sol, '*' as i32);
        if mark.is_null() {
            strcpy(fn_0, (*T).save_sol);
        } else {
            memcpy(
                fn_0 as *mut libc::c_void,
                (*T).save_sol as *const libc::c_void,
                mark.offset_from((*T).save_sol) as libc::c_long as libc::c_ulong,
            );
            *fn_0
                .offset(
                    mark.offset_from((*T).save_sol) as libc::c_long as isize,
                ) = '\0' as i32 as libc::c_char;
            (*T).save_cnt += 1;
            sprintf(
                fn_0.offset(strlen(fn_0) as isize),
                b"%03d\0" as *const u8 as *const libc::c_char,
                (*T).save_cnt,
            );
            strcat(fn_0, &mut *mark.offset(1 as libc::c_int as isize));
        }
        glp_write_mip((*T).P as *mut glp_prob, fn_0);
        glp_free(fn_0 as *mut libc::c_void);
    }
}
