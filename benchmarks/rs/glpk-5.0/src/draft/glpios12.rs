use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_cfg;
    pub type glp_mir;
    pub type glp_cov;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
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
pub unsafe extern "C" fn _glp_ios_choose_node(mut T: *mut glp_tree) -> libc::c_int {
    let mut p: libc::c_int = 0;
    if (*(*T).parm).bt_tech == 1 as libc::c_int {
        (!((*T).tail).is_null()
            || {
                glp_assert_(
                    b"T->tail != NULL\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios12.c\0" as *const u8 as *const libc::c_char,
                    54 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        p = (*(*T).tail).p;
    } else if (*(*T).parm).bt_tech == 2 as libc::c_int {
        (!((*T).head).is_null()
            || {
                glp_assert_(
                    b"T->head != NULL\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios12.c\0" as *const u8 as *const libc::c_char,
                    59 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        p = (*(*T).head).p;
    } else if (*(*T).parm).bt_tech == 3 as libc::c_int {
        p = best_node(T);
    } else if (*(*T).parm).bt_tech == 4 as libc::c_int {
        if (*(*T).mip).mip_stat == 1 as libc::c_int {
            p = most_feas(T);
        } else {
            p = best_proj(T);
        }
    } else {
        (T != T
            || {
                glp_assert_(
                    b"T != T\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios12.c\0" as *const u8 as *const libc::c_char,
                    77 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    return p;
}
unsafe extern "C" fn most_feas(mut T: *mut glp_tree) -> libc::c_int {
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    let mut p: libc::c_int = 0;
    let mut best: libc::c_double = 0.;
    p = 0 as libc::c_int;
    best = 1.7976931348623157e+308f64;
    node = (*T).head;
    while !node.is_null() {
        (!((*node).up).is_null()
            || {
                glp_assert_(
                    b"node->up != NULL\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios12.c\0" as *const u8 as *const libc::c_char,
                    89 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if best > (*(*node).up).ii_sum {
            p = (*node).p;
            best = (*(*node).up).ii_sum;
        }
        node = (*node).next;
    }
    return p;
}
unsafe extern "C" fn best_proj(mut T: *mut glp_tree) -> libc::c_int {
    let mut root: *mut IOSNPD = 0 as *mut IOSNPD;
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    let mut p: libc::c_int = 0;
    let mut best: libc::c_double = 0.;
    let mut deg: libc::c_double = 0.;
    let mut obj: libc::c_double = 0.;
    ((*(*T).mip).mip_stat == 2 as libc::c_int
        || {
            glp_assert_(
                b"T->mip->mip_stat == GLP_FEAS\0" as *const u8 as *const libc::c_char,
                b"draft/glpios12.c\0" as *const u8 as *const libc::c_char,
                102 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    root = (*((*T).slot).offset(1 as libc::c_int as isize)).node;
    (!root.is_null()
        || {
            glp_assert_(
                b"root != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios12.c\0" as *const u8 as *const libc::c_char,
                105 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*root).ii_sum > 0.0f64
        || {
            glp_assert_(
                b"root->ii_sum > 0.0\0" as *const u8 as *const libc::c_char,
                b"draft/glpios12.c\0" as *const u8 as *const libc::c_char,
                108 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    deg = ((*(*T).mip).mip_obj - (*root).bound) / (*root).ii_sum;
    p = 0 as libc::c_int;
    best = 1.7976931348623157e+308f64;
    node = (*T).head;
    while !node.is_null() {
        (!((*node).up).is_null()
            || {
                glp_assert_(
                    b"node->up != NULL\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios12.c\0" as *const u8 as *const libc::c_char,
                    114 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        obj = (*(*node).up).bound + deg * (*(*node).up).ii_sum;
        if (*(*T).mip).dir == 2 as libc::c_int {
            obj = -obj;
        }
        if best > obj {
            p = (*node).p;
            best = obj;
        }
        node = (*node).next;
    }
    return p;
}
unsafe extern "C" fn best_node(mut T: *mut glp_tree) -> libc::c_int {
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    let mut best: *mut IOSNPD = 0 as *mut IOSNPD;
    let mut bound: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    match (*(*T).mip).dir {
        1 => {
            bound = 1.7976931348623157e+308f64;
            node = (*T).head;
            while !node.is_null() {
                if bound > (*node).bound {
                    bound = (*node).bound;
                }
                node = (*node).next;
            }
            (bound != 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"bound != +DBL_MAX\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios12.c\0" as *const u8 as *const libc::c_char,
                        135 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            eps = 1e-10f64 * (1.0f64 + fabs(bound));
            node = (*T).head;
            while !node.is_null() {
                if (*node).bound <= bound + eps {
                    (!((*node).up).is_null()
                        || {
                            glp_assert_(
                                b"node->up != NULL\0" as *const u8 as *const libc::c_char,
                                b"draft/glpios12.c\0" as *const u8 as *const libc::c_char,
                                139 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    if best.is_null() || (*(*best).up).ii_sum > (*(*node).up).ii_sum {
                        best = node;
                    }
                }
                node = (*node).next;
            }
        }
        2 => {
            bound = -1.7976931348623157e+308f64;
            node = (*T).head;
            while !node.is_null() {
                if bound < (*node).bound {
                    bound = (*node).bound;
                }
                node = (*node).next;
            }
            (bound != -1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"bound != -DBL_MAX\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios12.c\0" as *const u8 as *const libc::c_char,
                        153 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            eps = 1e-10f64 * (1.0f64 + fabs(bound));
            node = (*T).head;
            while !node.is_null() {
                if (*node).bound >= bound - eps {
                    (!((*node).up).is_null()
                        || {
                            glp_assert_(
                                b"node->up != NULL\0" as *const u8 as *const libc::c_char,
                                b"draft/glpios12.c\0" as *const u8 as *const libc::c_char,
                                157 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    if best.is_null() || (*(*best).up).ii_sum > (*(*node).up).ii_sum {
                        best = node;
                    }
                }
                node = (*node).next;
            }
        }
        _ => {
            (T != T
                || {
                    glp_assert_(
                        b"T != T\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios12.c\0" as *const u8 as *const libc::c_char,
                        168 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    (!best.is_null()
        || {
            glp_assert_(
                b"best != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios12.c\0" as *const u8 as *const libc::c_char,
                170 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return (*best).p;
}
