use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_cfg;
    pub type glp_mir;
    pub type glp_cov;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn _glp_ios_add_row(
        tree: *mut glp_tree,
        pool: *mut IOSPOOL,
        name: *const libc::c_char,
        klass: libc::c_int,
        flags: libc::c_int,
        len: libc::c_int,
        ind: *const libc::c_int,
        val: *const libc::c_double,
        type_0: libc::c_int,
        rhs: libc::c_double,
    ) -> libc::c_int;
    fn _glp_ios_relative_gap(tree: *mut glp_tree) -> libc::c_double;
    fn _glp_ios_best_node(tree: *mut glp_tree) -> libc::c_int;
    fn _glp_ios_process_sol(T: *mut glp_tree);
    fn _glp_ios_clear_pool(tree: *mut glp_tree, pool: *mut IOSPOOL);
    fn _glp_ios_del_row(tree: *mut glp_tree, pool: *mut IOSPOOL, i: libc::c_int);
}
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
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
pub struct glp_attr {
    pub level: libc::c_int,
    pub origin: libc::c_int,
    pub klass: libc::c_int,
    pub foo_bar: [libc::c_double; 7],
}
pub unsafe extern "C" fn glp_ios_reason(mut tree: *mut glp_tree) -> libc::c_int {
    return (*tree).reason;
}
pub unsafe extern "C" fn glp_ios_get_prob(mut tree: *mut glp_tree) -> *mut glp_prob {
    return (*tree).mip;
}
pub unsafe extern "C" fn glp_ios_tree_size(
    mut tree: *mut glp_tree,
    mut a_cnt: *mut libc::c_int,
    mut n_cnt: *mut libc::c_int,
    mut t_cnt: *mut libc::c_int,
) {
    if !a_cnt.is_null() {
        *a_cnt = (*tree).a_cnt;
    }
    if !n_cnt.is_null() {
        *n_cnt = (*tree).n_cnt;
    }
    if !t_cnt.is_null() {
        *t_cnt = (*tree).t_cnt;
    }
}
pub unsafe extern "C" fn glp_ios_curr_node(mut tree: *mut glp_tree) -> libc::c_int {
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    node = (*tree).curr;
    return if node.is_null() { 0 as libc::c_int } else { (*node).p };
}
pub unsafe extern "C" fn glp_ios_next_node(
    mut tree: *mut glp_tree,
    mut p: libc::c_int,
) -> libc::c_int {
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    if p == 0 as libc::c_int {
        node = (*tree).head;
    } else {
        let mut current_block_6: u64;
        if !(1 as libc::c_int <= p && p <= (*tree).nslots) {
            current_block_6 = 2993789950729859404;
        } else {
            current_block_6 = 17179679302217393232;
        }
        loop {
            match current_block_6 {
                2993789950729859404 => {
                    (glp_error_(
                        b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
                        163 as libc::c_int,
                    ))
                        .unwrap()(
                        b"glp_ios_next_node: p = %d; invalid subproblem reference number\n\0"
                            as *const u8 as *const libc::c_char,
                        p,
                    );
                    current_block_6 = 17179679302217393232;
                }
                _ => {
                    node = (*((*tree).slot).offset(p as isize)).node;
                    if node.is_null() {
                        current_block_6 = 2993789950729859404;
                    } else {
                        break;
                    }
                }
            }
        }
        if (*node).count != 0 as libc::c_int {
            (glp_error_(
                b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
                169 as libc::c_int,
            ))
                .unwrap()(
                b"glp_ios_next_node: p = %d; subproblem not in the active list\n\0"
                    as *const u8 as *const libc::c_char,
                p,
            );
        }
        node = (*node).next;
    }
    return if node.is_null() { 0 as libc::c_int } else { (*node).p };
}
pub unsafe extern "C" fn glp_ios_prev_node(
    mut tree: *mut glp_tree,
    mut p: libc::c_int,
) -> libc::c_int {
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    if p == 0 as libc::c_int {
        node = (*tree).tail;
    } else {
        let mut current_block_6: u64;
        if !(1 as libc::c_int <= p && p <= (*tree).nslots) {
            current_block_6 = 16934468337314461219;
        } else {
            current_block_6 = 17179679302217393232;
        }
        loop {
            match current_block_6 {
                16934468337314461219 => {
                    (glp_error_(
                        b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
                        210 as libc::c_int,
                    ))
                        .unwrap()(
                        b"glp_ios_prev_node: p = %d; invalid subproblem reference number\n\0"
                            as *const u8 as *const libc::c_char,
                        p,
                    );
                    current_block_6 = 17179679302217393232;
                }
                _ => {
                    node = (*((*tree).slot).offset(p as isize)).node;
                    if node.is_null() {
                        current_block_6 = 16934468337314461219;
                    } else {
                        break;
                    }
                }
            }
        }
        if (*node).count != 0 as libc::c_int {
            (glp_error_(
                b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
                216 as libc::c_int,
            ))
                .unwrap()(
                b"glp_ios_prev_node: p = %d; subproblem not in the active list\n\0"
                    as *const u8 as *const libc::c_char,
                p,
            );
        }
        node = (*node).prev;
    }
    return if node.is_null() { 0 as libc::c_int } else { (*node).p };
}
pub unsafe extern "C" fn glp_ios_up_node(
    mut tree: *mut glp_tree,
    mut p: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    if !(1 as libc::c_int <= p && p <= (*tree).nslots) {
        current_block = 11113636336352276289;
    } else {
        current_block = 16559507199688588974;
    }
    loop {
        match current_block {
            11113636336352276289 => {
                (glp_error_(
                    b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
                    246 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_ios_up_node: p = %d; invalid subproblem reference number\n\0"
                        as *const u8 as *const libc::c_char,
                    p,
                );
                current_block = 16559507199688588974;
            }
            _ => {
                node = (*((*tree).slot).offset(p as isize)).node;
                if node.is_null() {
                    current_block = 11113636336352276289;
                } else {
                    break;
                }
            }
        }
    }
    node = (*node).up;
    return if node.is_null() { 0 as libc::c_int } else { (*node).p };
}
pub unsafe extern "C" fn glp_ios_node_level(
    mut tree: *mut glp_tree,
    mut p: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    if !(1 as libc::c_int <= p && p <= (*tree).nslots) {
        current_block = 5925342424383658646;
    } else {
        current_block = 16559507199688588974;
    }
    loop {
        match current_block {
            5925342424383658646 => {
                (glp_error_(
                    b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
                    276 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_ios_node_level: p = %d; invalid subproblem reference number\n\0"
                        as *const u8 as *const libc::c_char,
                    p,
                );
                current_block = 16559507199688588974;
            }
            _ => {
                node = (*((*tree).slot).offset(p as isize)).node;
                if node.is_null() {
                    current_block = 5925342424383658646;
                } else {
                    break;
                }
            }
        }
    }
    return (*node).level;
}
pub unsafe extern "C" fn glp_ios_node_bound(
    mut tree: *mut glp_tree,
    mut p: libc::c_int,
) -> libc::c_double {
    let mut current_block: u64;
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    if !(1 as libc::c_int <= p && p <= (*tree).nslots) {
        current_block = 2188145222343300284;
    } else {
        current_block = 16559507199688588974;
    }
    loop {
        match current_block {
            2188145222343300284 => {
                (glp_error_(
                    b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
                    319 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_ios_node_bound: p = %d; invalid subproblem reference number\n\0"
                        as *const u8 as *const libc::c_char,
                    p,
                );
                current_block = 16559507199688588974;
            }
            _ => {
                node = (*((*tree).slot).offset(p as isize)).node;
                if node.is_null() {
                    current_block = 2188145222343300284;
                } else {
                    break;
                }
            }
        }
    }
    return (*node).bound;
}
pub unsafe extern "C" fn glp_ios_best_node(mut tree: *mut glp_tree) -> libc::c_int {
    return _glp_ios_best_node(tree);
}
pub unsafe extern "C" fn glp_ios_mip_gap(mut tree: *mut glp_tree) -> libc::c_double {
    return _glp_ios_relative_gap(tree);
}
pub unsafe extern "C" fn glp_ios_node_data(
    mut tree: *mut glp_tree,
    mut p: libc::c_int,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    if !(1 as libc::c_int <= p && p <= (*tree).nslots) {
        current_block = 11475374934439804317;
    } else {
        current_block = 16559507199688588974;
    }
    loop {
        match current_block {
            11475374934439804317 => {
                (glp_error_(
                    b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
                    416 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_ios_node_level: p = %d; invalid subproblem reference number\n\0"
                        as *const u8 as *const libc::c_char,
                    p,
                );
                current_block = 16559507199688588974;
            }
            _ => {
                node = (*((*tree).slot).offset(p as isize)).node;
                if node.is_null() {
                    current_block = 11475374934439804317;
                } else {
                    break;
                }
            }
        }
    }
    return (*node).data;
}
pub unsafe extern "C" fn glp_ios_row_attr(
    mut tree: *mut glp_tree,
    mut i: libc::c_int,
    mut attr: *mut glp_attr,
) {
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    if !(1 as libc::c_int <= i && i <= (*(*tree).mip).m) {
        (glp_error_(
            b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
            441 as libc::c_int,
        ))
            .unwrap()(
            b"glp_ios_row_attr: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    row = *((*(*tree).mip).row).offset(i as isize);
    (*attr).level = (*row).level;
    (*attr).origin = (*row).origin as libc::c_int;
    (*attr).klass = (*row).klass as libc::c_int;
}
pub unsafe extern "C" fn glp_ios_pool_size(mut tree: *mut glp_tree) -> libc::c_int {
    if (*tree).reason != 0x4 as libc::c_int {
        (glp_error_(
            b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
            455 as libc::c_int,
        ))
            .unwrap()(
            b"glp_ios_pool_size: operation not allowed\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    (!((*tree).local).is_null()
        || {
            glp_assert_(
                b"tree->local != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
                456 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return (*(*tree).local).m;
}
pub unsafe extern "C" fn glp_ios_add_row(
    mut tree: *mut glp_tree,
    mut name: *const libc::c_char,
    mut klass: libc::c_int,
    mut flags: libc::c_int,
    mut len: libc::c_int,
    mut ind: *const libc::c_int,
    mut val: *const libc::c_double,
    mut type_0: libc::c_int,
    mut rhs: libc::c_double,
) -> libc::c_int {
    let mut num: libc::c_int = 0;
    if (*tree).reason != 0x4 as libc::c_int {
        (glp_error_(
            b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
            472 as libc::c_int,
        ))
            .unwrap()(
            b"glp_ios_add_row: operation not allowed\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    (!((*tree).local).is_null()
        || {
            glp_assert_(
                b"tree->local != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
                473 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    num = _glp_ios_add_row(
        tree,
        (*tree).local,
        name,
        klass,
        flags,
        len,
        ind,
        val,
        type_0,
        rhs,
    );
    return num;
}
pub unsafe extern "C" fn glp_ios_del_row(mut tree: *mut glp_tree, mut i: libc::c_int) {
    if (*tree).reason != 0x4 as libc::c_int {
        (glp_error_(
            b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
            484 as libc::c_int,
        ))
            .unwrap()(
            b"glp_ios_del_row: operation not allowed\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    _glp_ios_del_row(tree, (*tree).local, i);
}
pub unsafe extern "C" fn glp_ios_clear_pool(mut tree: *mut glp_tree) {
    if (*tree).reason != 0x4 as libc::c_int {
        (glp_error_(
            b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
            494 as libc::c_int,
        ))
            .unwrap()(
            b"glp_ios_clear_pool: operation not allowed\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    _glp_ios_clear_pool(tree, (*tree).local);
}
pub unsafe extern "C" fn glp_ios_can_branch(
    mut tree: *mut glp_tree,
    mut j: libc::c_int,
) -> libc::c_int {
    if !(1 as libc::c_int <= j && j <= (*(*tree).mip).n) {
        (glp_error_(
            b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
            515 as libc::c_int,
        ))
            .unwrap()(
            b"glp_ios_can_branch: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    return *((*tree).non_int).offset(j as isize) as libc::c_int;
}
pub unsafe extern "C" fn glp_ios_branch_upon(
    mut tree: *mut glp_tree,
    mut j: libc::c_int,
    mut sel: libc::c_int,
) {
    if !(1 as libc::c_int <= j && j <= (*(*tree).mip).n) {
        (glp_error_(
            b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
            546 as libc::c_int,
        ))
            .unwrap()(
            b"glp_ios_branch_upon: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    if !(sel == 1 as libc::c_int || sel == 2 as libc::c_int || sel == 0 as libc::c_int) {
        (glp_error_(
            b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
            550 as libc::c_int,
        ))
            .unwrap()(
            b"glp_ios_branch_upon: sel = %d: invalid branch selection flag\n\0"
                as *const u8 as *const libc::c_char,
            sel,
        );
    }
    if *((*tree).non_int).offset(j as isize) == 0 {
        (glp_error_(
            b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
            553 as libc::c_int,
        ))
            .unwrap()(
            b"glp_ios_branch_upon: j = %d; variable cannot be used to branch upon\n\0"
                as *const u8 as *const libc::c_char,
            j,
        );
    }
    if (*tree).br_var != 0 as libc::c_int {
        (glp_error_(
            b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
            556 as libc::c_int,
        ))
            .unwrap()(
            b"glp_ios_branch_upon: branching variable already chosen\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*tree).br_var = j;
    (*tree).br_sel = sel;
}
pub unsafe extern "C" fn glp_ios_select_node(
    mut tree: *mut glp_tree,
    mut p: libc::c_int,
) {
    let mut current_block: u64;
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    if !(1 as libc::c_int <= p && p <= (*tree).nslots) {
        current_block = 12300177230703411789;
    } else {
        current_block = 16559507199688588974;
    }
    loop {
        match current_block {
            12300177230703411789 => {
                (glp_error_(
                    b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
                    583 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_ios_select_node: p = %d; invalid subproblem reference number\n\0"
                        as *const u8 as *const libc::c_char,
                    p,
                );
                current_block = 16559507199688588974;
            }
            _ => {
                node = (*((*tree).slot).offset(p as isize)).node;
                if node.is_null() {
                    current_block = 12300177230703411789;
                } else {
                    break;
                }
            }
        }
    }
    if (*node).count != 0 as libc::c_int {
        (glp_error_(
            b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
            589 as libc::c_int,
        ))
            .unwrap()(
            b"glp_ios_select_node: p = %d; subproblem not in the active list\n\0"
                as *const u8 as *const libc::c_char,
            p,
        );
    }
    if (*tree).next_p != 0 as libc::c_int {
        (glp_error_(
            b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
            593 as libc::c_int,
        ))
            .unwrap()(
            b"glp_ios_select_node: subproblem already selected\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*tree).next_p = p;
}
pub unsafe extern "C" fn glp_ios_heur_sol(
    mut tree: *mut glp_tree,
    mut x: *const libc::c_double,
) -> libc::c_int {
    let mut mip: *mut glp_prob = (*tree).mip;
    let mut m: libc::c_int = (*tree).orig_m;
    let mut n: libc::c_int = (*tree).n;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut obj: libc::c_double = 0.;
    ((*mip).m >= m
        || {
            glp_assert_(
                b"mip->m >= m\0" as *const u8 as *const libc::c_char,
                b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
                638 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*mip).n == n
        || {
            glp_assert_(
                b"mip->n == n\0" as *const u8 as *const libc::c_char,
                b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
                639 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    obj = (*mip).c0;
    j = 1 as libc::c_int;
    while j <= n {
        let mut col: *mut GLPCOL = *((*mip).col).offset(j as isize);
        if (*col).kind == 2 as libc::c_int {
            if *x.offset(j as isize) != floor(*x.offset(j as isize)) {
                return 1 as libc::c_int;
            }
        }
        obj += (*col).coef * *x.offset(j as isize);
        j += 1;
        j;
    }
    if (*mip).mip_stat == 2 as libc::c_int {
        match (*mip).dir {
            1 => {
                if obj >= (*(*tree).mip).mip_obj {
                    return 1 as libc::c_int;
                }
            }
            2 => {
                if obj <= (*(*tree).mip).mip_obj {
                    return 1 as libc::c_int;
                }
            }
            _ => {
                (mip != mip
                    || {
                        glp_assert_(
                            b"mip != mip\0" as *const u8 as *const libc::c_char,
                            b"draft/glpapi13.c\0" as *const u8 as *const libc::c_char,
                            662 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
    }
    if (*(*tree).parm).msg_lev >= 2 as libc::c_int {
        glp_printf(
            b"Solution found by heuristic: %.12g\n\0" as *const u8
                as *const libc::c_char,
            obj,
        );
    }
    (*mip).mip_stat = 2 as libc::c_int;
    (*mip).mip_obj = obj;
    j = 1 as libc::c_int;
    while j <= n {
        (**((*mip).col).offset(j as isize)).mipx = *x.offset(j as isize);
        j += 1;
        j;
    }
    i = 1 as libc::c_int;
    while i <= m {
        let mut row: *mut GLPROW = *((*mip).row).offset(i as isize);
        let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
        (*row).mipx = 0.0f64;
        aij = (*row).ptr;
        while !aij.is_null() {
            (*row).mipx += (*aij).val * (*(*aij).col).mipx;
            aij = (*aij).r_next;
        }
        i += 1;
        i;
    }
    _glp_ios_process_sol(tree);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn glp_ios_terminate(mut tree: *mut glp_tree) {
    if (*(*tree).parm).msg_lev >= 4 as libc::c_int {
        glp_printf(
            b"The search is prematurely terminated due to application request\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    (*tree).stop = 1 as libc::c_int;
}
