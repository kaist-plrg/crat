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
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
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
    fn glp_get_row_lb(P: *mut glp_prob, i: libc::c_int) -> libc::c_double;
    fn glp_get_row_ub(P: *mut glp_prob, i: libc::c_int) -> libc::c_double;
    fn glp_get_col_lb(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
    fn glp_get_col_ub(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
    fn glp_get_row_stat(P: *mut glp_prob, i: libc::c_int) -> libc::c_int;
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
pub struct f_info {
    pub j_min: libc::c_int,
    pub j_max: libc::c_int,
    pub f_min: libc::c_double,
    pub f_max: libc::c_double,
}
unsafe extern "C" fn prepare_row_info(
    mut n: libc::c_int,
    mut a: *const libc::c_double,
    mut l: *const libc::c_double,
    mut u: *const libc::c_double,
    mut f: *mut f_info,
) {
    let mut j: libc::c_int = 0;
    let mut j_min: libc::c_int = 0;
    let mut j_max: libc::c_int = 0;
    let mut f_min: libc::c_double = 0.;
    let mut f_max: libc::c_double = 0.;
    (n >= 0 as libc::c_int
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const libc::c_char,
                b"draft/glpios02.c\0" as *const u8 as *const libc::c_char,
                104 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    f_min = 0.0f64;
    j_min = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while j <= n {
        if *a.offset(j as isize) > 0.0f64 {
            if *l.offset(j as isize) == -1.7976931348623157e+308f64 {
                if j_min == 0 as libc::c_int {
                    j_min = j;
                } else {
                    f_min = -1.7976931348623157e+308f64;
                    j_min = 0 as libc::c_int;
                    break;
                }
            } else {
                f_min += *a.offset(j as isize) * *l.offset(j as isize);
            }
        } else if *a.offset(j as isize) < 0.0f64 {
            if *u.offset(j as isize) == 1.7976931348623157e+308f64 {
                if j_min == 0 as libc::c_int {
                    j_min = j;
                } else {
                    f_min = -1.7976931348623157e+308f64;
                    j_min = 0 as libc::c_int;
                    break;
                }
            } else {
                f_min += *a.offset(j as isize) * *u.offset(j as isize);
            }
        } else {
            (a != a
                || {
                    glp_assert_(
                        b"a != a\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios02.c\0" as *const u8 as *const libc::c_char,
                        133 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        j += 1;
        j;
    }
    (*f).f_min = f_min;
    (*f).j_min = j_min;
    f_max = 0.0f64;
    j_max = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while j <= n {
        if *a.offset(j as isize) > 0.0f64 {
            if *u.offset(j as isize) == 1.7976931348623157e+308f64 {
                if j_max == 0 as libc::c_int {
                    j_max = j;
                } else {
                    f_max = 1.7976931348623157e+308f64;
                    j_max = 0 as libc::c_int;
                    break;
                }
            } else {
                f_max += *a.offset(j as isize) * *u.offset(j as isize);
            }
        } else if *a.offset(j as isize) < 0.0f64 {
            if *l.offset(j as isize) == -1.7976931348623157e+308f64 {
                if j_max == 0 as libc::c_int {
                    j_max = j;
                } else {
                    f_max = 1.7976931348623157e+308f64;
                    j_max = 0 as libc::c_int;
                    break;
                }
            } else {
                f_max += *a.offset(j as isize) * *l.offset(j as isize);
            }
        } else {
            (a != a
                || {
                    glp_assert_(
                        b"a != a\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios02.c\0" as *const u8 as *const libc::c_char,
                        164 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        j += 1;
        j;
    }
    (*f).f_max = f_max;
    (*f).j_max = j_max;
}
unsafe extern "C" fn row_implied_bounds(
    mut f: *const f_info,
    mut LL: *mut libc::c_double,
    mut UU: *mut libc::c_double,
) {
    *LL = if (*f).j_min == 0 as libc::c_int {
        (*f).f_min
    } else {
        -1.7976931348623157e+308f64
    };
    *UU = if (*f).j_max == 0 as libc::c_int {
        (*f).f_max
    } else {
        1.7976931348623157e+308f64
    };
}
unsafe extern "C" fn col_implied_bounds(
    mut f: *const f_info,
    mut n: libc::c_int,
    mut a: *const libc::c_double,
    mut L: libc::c_double,
    mut U: libc::c_double,
    mut l: *const libc::c_double,
    mut u: *const libc::c_double,
    mut k: libc::c_int,
    mut ll: *mut libc::c_double,
    mut uu: *mut libc::c_double,
) {
    let mut ilb: libc::c_double = 0.;
    let mut iub: libc::c_double = 0.;
    (n >= 0 as libc::c_int
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const libc::c_char,
                b"draft/glpios02.c\0" as *const u8 as *const libc::c_char,
                299 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= k && k <= n
        || {
            glp_assert_(
                b"1 <= k && k <= n\0" as *const u8 as *const libc::c_char,
                b"draft/glpios02.c\0" as *const u8 as *const libc::c_char,
                300 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if L == -1.7976931348623157e+308f64 || (*f).f_max == 1.7976931348623157e+308f64 {
        ilb = -1.7976931348623157e+308f64;
    } else if (*f).j_max == 0 as libc::c_int {
        if *a.offset(k as isize) > 0.0f64 {
            (*u.offset(k as isize) != 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"u[k] != +DBL_MAX\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios02.c\0" as *const u8 as *const libc::c_char,
                        306 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            ilb = L - ((*f).f_max - *a.offset(k as isize) * *u.offset(k as isize));
        } else if *a.offset(k as isize) < 0.0f64 {
            (*l.offset(k as isize) != -1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"l[k] != -DBL_MAX\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios02.c\0" as *const u8 as *const libc::c_char,
                        310 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            ilb = L - ((*f).f_max - *a.offset(k as isize) * *l.offset(k as isize));
        } else {
            (a != a
                || {
                    glp_assert_(
                        b"a != a\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios02.c\0" as *const u8 as *const libc::c_char,
                        314 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    } else if (*f).j_max == k {
        ilb = L - (*f).f_max;
    } else {
        ilb = -1.7976931348623157e+308f64;
    }
    if U == 1.7976931348623157e+308f64 || (*f).f_min == -1.7976931348623157e+308f64 {
        iub = 1.7976931348623157e+308f64;
    } else if (*f).j_min == 0 as libc::c_int {
        if *a.offset(k as isize) > 0.0f64 {
            (*l.offset(k as isize) != -1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"l[k] != -DBL_MAX\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios02.c\0" as *const u8 as *const libc::c_char,
                        325 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            iub = U - ((*f).f_min - *a.offset(k as isize) * *l.offset(k as isize));
        } else if *a.offset(k as isize) < 0.0f64 {
            (*u.offset(k as isize) != 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"u[k] != +DBL_MAX\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios02.c\0" as *const u8 as *const libc::c_char,
                        329 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            iub = U - ((*f).f_min - *a.offset(k as isize) * *u.offset(k as isize));
        } else {
            (a != a
                || {
                    glp_assert_(
                        b"a != a\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios02.c\0" as *const u8 as *const libc::c_char,
                        333 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    } else if (*f).j_min == k {
        iub = U - (*f).f_min;
    } else {
        iub = 1.7976931348623157e+308f64;
    }
    if fabs(*a.offset(k as isize)) < 1e-6f64 {
        *ll = -1.7976931348623157e+308f64;
        *uu = 1.7976931348623157e+308f64;
    } else if *a.offset(k as isize) > 0.0f64 {
        *ll = if ilb == -1.7976931348623157e+308f64 {
            -1.7976931348623157e+308f64
        } else {
            ilb / *a.offset(k as isize)
        };
        *uu = if iub == 1.7976931348623157e+308f64 {
            1.7976931348623157e+308f64
        } else {
            iub / *a.offset(k as isize)
        };
    } else if *a.offset(k as isize) < 0.0f64 {
        *ll = if iub == 1.7976931348623157e+308f64 {
            -1.7976931348623157e+308f64
        } else {
            iub / *a.offset(k as isize)
        };
        *uu = if ilb == -1.7976931348623157e+308f64 {
            1.7976931348623157e+308f64
        } else {
            ilb / *a.offset(k as isize)
        };
    } else {
        (a != a
            || {
                glp_assert_(
                    b"a != a\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios02.c\0" as *const u8 as *const libc::c_char,
                    357 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    };
}
unsafe extern "C" fn check_row_bounds(
    mut f: *const f_info,
    mut L_: *mut libc::c_double,
    mut U_: *mut libc::c_double,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut L: libc::c_double = *L_;
    let mut U: libc::c_double = *U_;
    let mut LL: libc::c_double = 0.;
    let mut UU: libc::c_double = 0.;
    row_implied_bounds(f, &mut LL, &mut UU);
    if L != -1.7976931348623157e+308f64 {
        let mut eps: libc::c_double = 1e-3f64 * (1.0f64 + fabs(L));
        if UU < L - eps {
            ret = 1 as libc::c_int;
            current_block = 6824992841857643189;
        } else {
            current_block = 820271813250567934;
        }
    } else {
        current_block = 820271813250567934;
    }
    match current_block {
        820271813250567934 => {
            if U != 1.7976931348623157e+308f64 {
                let mut eps_0: libc::c_double = 1e-3f64 * (1.0f64 + fabs(U));
                if LL > U + eps_0 {
                    ret = 1 as libc::c_int;
                    current_block = 6824992841857643189;
                } else {
                    current_block = 13513818773234778473;
                }
            } else {
                current_block = 13513818773234778473;
            }
            match current_block {
                6824992841857643189 => {}
                _ => {
                    if L != -1.7976931348623157e+308f64 {
                        let mut eps_1: libc::c_double = 1e-12f64 * (1.0f64 + fabs(L));
                        if LL > L - eps_1 {
                            *L_ = -1.7976931348623157e+308f64;
                        }
                    }
                    if U != 1.7976931348623157e+308f64 {
                        let mut eps_2: libc::c_double = 1e-12f64 * (1.0f64 + fabs(U));
                        if UU < U + eps_2 {
                            *U_ = 1.7976931348623157e+308f64;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn check_col_bounds(
    mut f: *const f_info,
    mut n: libc::c_int,
    mut a: *const libc::c_double,
    mut L: libc::c_double,
    mut U: libc::c_double,
    mut l: *const libc::c_double,
    mut u: *const libc::c_double,
    mut flag: libc::c_int,
    mut j: libc::c_int,
    mut _lj: *mut libc::c_double,
    mut _uj: *mut libc::c_double,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut lj: libc::c_double = 0.;
    let mut uj: libc::c_double = 0.;
    let mut ll: libc::c_double = 0.;
    let mut uu: libc::c_double = 0.;
    (n >= 0 as libc::c_int
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const libc::c_char,
                b"draft/glpios02.c\0" as *const u8 as *const libc::c_char,
                457 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= j && j <= n
        || {
            glp_assert_(
                b"1 <= j && j <= n\0" as *const u8 as *const libc::c_char,
                b"draft/glpios02.c\0" as *const u8 as *const libc::c_char,
                458 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    lj = *l.offset(j as isize);
    uj = *u.offset(j as isize);
    col_implied_bounds(f, n, a, L, U, l, u, j, &mut ll, &mut uu);
    if flag != 0 {
        if ll != -1.7976931348623157e+308f64 {
            ll = if ll - floor(ll) < 1e-3f64 { floor(ll) } else { ceil(ll) };
        }
        if uu != 1.7976931348623157e+308f64 {
            uu = if ceil(uu) - uu < 1e-3f64 { ceil(uu) } else { floor(uu) };
        }
    }
    if lj != -1.7976931348623157e+308f64 {
        let mut eps: libc::c_double = 1e-3f64 * (1.0f64 + fabs(lj));
        if uu < lj - eps {
            ret = 1 as libc::c_int;
            current_block = 5195975951074984801;
        } else {
            current_block = 12209867499936983673;
        }
    } else {
        current_block = 12209867499936983673;
    }
    match current_block {
        12209867499936983673 => {
            if uj != 1.7976931348623157e+308f64 {
                let mut eps_0: libc::c_double = 1e-3f64 * (1.0f64 + fabs(uj));
                if ll > uj + eps_0 {
                    ret = 1 as libc::c_int;
                    current_block = 5195975951074984801;
                } else {
                    current_block = 17407779659766490442;
                }
            } else {
                current_block = 17407779659766490442;
            }
            match current_block {
                5195975951074984801 => {}
                _ => {
                    if ll != -1.7976931348623157e+308f64 {
                        let mut eps_1: libc::c_double = 1e-3f64 * (1.0f64 + fabs(ll));
                        if lj < ll - eps_1 {
                            lj = ll;
                        }
                    }
                    if uu != 1.7976931348623157e+308f64 {
                        let mut eps_2: libc::c_double = 1e-3f64 * (1.0f64 + fabs(uu));
                        if uj > uu + eps_2 {
                            uj = uu;
                        }
                    }
                    if !(lj == -1.7976931348623157e+308f64
                        || uj == 1.7976931348623157e+308f64)
                    {
                        let mut t1: libc::c_double = fabs(lj);
                        let mut t2: libc::c_double = fabs(uj);
                        let mut eps_3: libc::c_double = 1e-10f64
                            * (1.0f64 + (if t1 <= t2 { t1 } else { t2 }));
                        if lj > uj - eps_3 {
                            if lj == *l.offset(j as isize) {
                                uj = lj;
                            } else if uj == *u.offset(j as isize) {
                                lj = uj;
                            } else if t1 <= t2 {
                                uj = lj;
                            } else {
                                lj = uj;
                            }
                        }
                    }
                    *_lj = lj;
                    *_uj = uj;
                }
            }
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn check_efficiency(
    mut flag: libc::c_int,
    mut l: libc::c_double,
    mut u: libc::c_double,
    mut ll: libc::c_double,
    mut uu: libc::c_double,
) -> libc::c_int {
    let mut eff: libc::c_int = 0 as libc::c_int;
    if l < ll {
        if flag != 0 || l == -1.7976931348623157e+308f64 {
            eff += 1;
            eff;
        } else {
            let mut r: libc::c_double = 0.;
            if u == 1.7976931348623157e+308f64 {
                r = 1.0f64 + fabs(l);
            } else {
                r = 1.0f64 + (u - l);
            }
            if ll - l >= 0.25f64 * r {
                eff += 1;
                eff;
            }
        }
    }
    if u > uu {
        if flag != 0 || u == 1.7976931348623157e+308f64 {
            eff += 1;
            eff;
        } else {
            let mut r_0: libc::c_double = 0.;
            if l == -1.7976931348623157e+308f64 {
                r_0 = 1.0f64 + fabs(u);
            } else {
                r_0 = 1.0f64 + (u - l);
            }
            if u - uu >= 0.25f64 * r_0 {
                eff += 1;
                eff;
            }
        }
    }
    return eff;
}
unsafe extern "C" fn basic_preprocessing(
    mut mip: *mut glp_prob,
    mut L: *mut libc::c_double,
    mut U: *mut libc::c_double,
    mut l: *mut libc::c_double,
    mut u: *mut libc::c_double,
    mut nrs: libc::c_int,
    mut num: *const libc::c_int,
    mut max_pass: libc::c_int,
) -> libc::c_int {
    let mut m: libc::c_int = (*mip).m;
    let mut n: libc::c_int = (*mip).n;
    let mut f: f_info = f_info {
        j_min: 0,
        j_max: 0,
        f_min: 0.,
        f_max: 0.,
    };
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut list: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut mark: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pass: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut lb: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut ub: *mut libc::c_double = 0 as *mut libc::c_double;
    (0 as libc::c_int <= nrs && nrs <= m + 1 as libc::c_int
        || {
            glp_assert_(
                b"0 <= nrs && nrs <= m+1\0" as *const u8 as *const libc::c_char,
                b"draft/glpios02.c\0" as *const u8 as *const libc::c_char,
                608 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (max_pass > 0 as libc::c_int
        || {
            glp_assert_(
                b"max_pass > 0\0" as *const u8 as *const libc::c_char,
                b"draft/glpios02.c\0" as *const u8 as *const libc::c_char,
                609 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ind = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    list = glp_alloc(
        1 as libc::c_int + m + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    mark = glp_alloc(
        1 as libc::c_int + m + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    memset(
        &mut *mark.offset(0 as libc::c_int as isize) as *mut libc::c_int
            as *mut libc::c_void,
        0 as libc::c_int,
        ((m + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    pass = glp_alloc(
        1 as libc::c_int + m + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    memset(
        &mut *pass.offset(0 as libc::c_int as isize) as *mut libc::c_int
            as *mut libc::c_void,
        0 as libc::c_int,
        ((m + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    val = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    lb = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    ub = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    size = 0 as libc::c_int;
    k = 1 as libc::c_int;
    while k <= nrs {
        i = *num.offset(k as isize);
        (0 as libc::c_int <= i && i <= m
            || {
                glp_assert_(
                    b"0 <= i && i <= m\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios02.c\0" as *const u8 as *const libc::c_char,
                    624 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*mark.offset(i as isize) == 0
            || {
                glp_assert_(
                    b"!mark[i]\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios02.c\0" as *const u8 as *const libc::c_char,
                    626 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        size += 1;
        *list.offset(size as isize) = i;
        *mark.offset(i as isize) = 1 as libc::c_int;
        k += 1;
        k;
    }
    (size == nrs
        || {
            glp_assert_(
                b"size == nrs\0" as *const u8 as *const libc::c_char,
                b"draft/glpios02.c\0" as *const u8 as *const libc::c_char,
                629 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    's_79: while size > 0 as libc::c_int {
        let fresh0 = size;
        size = size - 1;
        i = *list.offset(fresh0 as isize);
        *mark.offset(i as isize) = 0 as libc::c_int;
        let ref mut fresh1 = *pass.offset(i as isize);
        *fresh1 += 1;
        *fresh1;
        if *L.offset(i as isize) == -1.7976931348623157e+308f64
            && *U.offset(i as isize) == 1.7976931348623157e+308f64
        {
            continue;
        }
        len = 0 as libc::c_int;
        if i == 0 as libc::c_int {
            j = 1 as libc::c_int;
            while j <= n {
                let mut col: *mut GLPCOL = *((*mip).col).offset(j as isize);
                if (*col).coef != 0.0f64 {
                    len += 1;
                    len;
                    *ind.offset(len as isize) = j;
                    *val.offset(len as isize) = (*col).coef;
                }
                j += 1;
                j;
            }
        } else {
            let mut row: *mut GLPROW = *((*mip).row).offset(i as isize);
            let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
            aij = (*row).ptr;
            while !aij.is_null() {
                len += 1;
                len;
                *ind.offset(len as isize) = (*(*aij).col).j;
                *val.offset(len as isize) = (*aij).val;
                aij = (*aij).r_next;
            }
        }
        k = 1 as libc::c_int;
        while k <= len {
            j = *ind.offset(k as isize);
            *lb.offset(k as isize) = *l.offset(j as isize);
            *ub.offset(k as isize) = *u.offset(j as isize);
            k += 1;
            k;
        }
        prepare_row_info(
            len,
            val as *const libc::c_double,
            lb as *const libc::c_double,
            ub as *const libc::c_double,
            &mut f,
        );
        if check_row_bounds(
            &mut f,
            &mut *L.offset(i as isize),
            &mut *U.offset(i as isize),
        ) != 0
        {
            ret = 1 as libc::c_int;
            break;
        } else {
            if *L.offset(i as isize) == -1.7976931348623157e+308f64
                && *U.offset(i as isize) == 1.7976931348623157e+308f64
            {
                continue;
            }
            k = 1 as libc::c_int;
            while k <= len {
                let mut col_0: *mut GLPCOL = 0 as *mut GLPCOL;
                let mut flag: libc::c_int = 0;
                let mut eff: libc::c_int = 0;
                let mut ll: libc::c_double = 0.;
                let mut uu: libc::c_double = 0.;
                j = *ind.offset(k as isize);
                col_0 = *((*mip).col).offset(j as isize);
                flag = ((*col_0).kind != 1 as libc::c_int) as libc::c_int;
                if check_col_bounds(
                    &mut f,
                    len,
                    val as *const libc::c_double,
                    *L.offset(i as isize),
                    *U.offset(i as isize),
                    lb as *const libc::c_double,
                    ub as *const libc::c_double,
                    flag,
                    k,
                    &mut ll,
                    &mut uu,
                ) != 0
                {
                    ret = 1 as libc::c_int;
                    break 's_79;
                } else {
                    eff = check_efficiency(
                        flag,
                        *l.offset(j as isize),
                        *u.offset(j as isize),
                        ll,
                        uu,
                    );
                    *l.offset(j as isize) = ll;
                    *u.offset(j as isize) = uu;
                    if eff > 0 as libc::c_int {
                        let mut aij_0: *mut GLPAIJ = 0 as *mut GLPAIJ;
                        aij_0 = (*col_0).ptr;
                        while !aij_0.is_null() {
                            let mut ii: libc::c_int = (*(*aij_0).row).i;
                            if !(*pass.offset(ii as isize) >= max_pass) {
                                if !(*L.offset(ii as isize) == -1.7976931348623157e+308f64
                                    && *U.offset(ii as isize) == 1.7976931348623157e+308f64)
                                {
                                    if *mark.offset(ii as isize) == 0 as libc::c_int {
                                        (size <= m
                                            || {
                                                glp_assert_(
                                                    b"size <= m\0" as *const u8 as *const libc::c_char,
                                                    b"draft/glpios02.c\0" as *const u8 as *const libc::c_char,
                                                    699 as libc::c_int,
                                                );
                                                1 as libc::c_int != 0
                                            }) as libc::c_int;
                                        size += 1;
                                        *list.offset(size as isize) = ii;
                                        *mark.offset(ii as isize) = 1 as libc::c_int;
                                    }
                                }
                            }
                            aij_0 = (*aij_0).c_next;
                        }
                    }
                    k += 1;
                    k;
                }
            }
        }
    }
    glp_free(ind as *mut libc::c_void);
    glp_free(list as *mut libc::c_void);
    glp_free(mark as *mut libc::c_void);
    glp_free(pass as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
    glp_free(lb as *mut libc::c_void);
    glp_free(ub as *mut libc::c_void);
    return ret;
}
pub unsafe extern "C" fn _glp_ios_preprocess_node(
    mut tree: *mut glp_tree,
    mut max_pass: libc::c_int,
) -> libc::c_int {
    let mut mip: *mut glp_prob = (*tree).mip;
    let mut m: libc::c_int = (*mip).m;
    let mut n: libc::c_int = (*mip).n;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nrs: libc::c_int = 0;
    let mut num: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut L: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut U: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut l: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut u: *mut libc::c_double = 0 as *mut libc::c_double;
    (!((*tree).curr).is_null()
        || {
            glp_assert_(
                b"tree->curr != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios02.c\0" as *const u8 as *const libc::c_char,
                744 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    L = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    U = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    match (*mip).mip_stat {
        1 => {
            *L.offset(0 as libc::c_int as isize) = -1.7976931348623157e+308f64;
            *U.offset(0 as libc::c_int as isize) = 1.7976931348623157e+308f64;
        }
        2 => {
            match (*mip).dir {
                1 => {
                    *L.offset(0 as libc::c_int as isize) = -1.7976931348623157e+308f64;
                    *U.offset(0 as libc::c_int as isize) = (*mip).mip_obj - (*mip).c0;
                }
                2 => {
                    *L.offset(0 as libc::c_int as isize) = (*mip).mip_obj - (*mip).c0;
                    *U.offset(0 as libc::c_int as isize) = 1.7976931348623157e+308f64;
                }
                _ => {
                    (mip != mip
                        || {
                            glp_assert_(
                                b"mip != mip\0" as *const u8 as *const libc::c_char,
                                b"draft/glpios02.c\0" as *const u8 as *const libc::c_char,
                                761 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
            }
        }
        _ => {
            (mip != mip
                || {
                    glp_assert_(
                        b"mip != mip\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios02.c\0" as *const u8 as *const libc::c_char,
                        765 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    i = 1 as libc::c_int;
    while i <= m {
        *L.offset(i as isize) = glp_get_row_lb(mip, i);
        *U.offset(i as isize) = glp_get_row_ub(mip, i);
        i += 1;
        i;
    }
    l = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    u = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    j = 1 as libc::c_int;
    while j <= n {
        *l.offset(j as isize) = glp_get_col_lb(mip, j);
        *u.offset(j as isize) = glp_get_col_ub(mip, j);
        j += 1;
        j;
    }
    nrs = m + 1 as libc::c_int;
    num = glp_alloc(
        1 as libc::c_int + nrs,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    i = 1 as libc::c_int;
    while i <= nrs {
        *num.offset(i as isize) = i - 1 as libc::c_int;
        i += 1;
        i;
    }
    if basic_preprocessing(mip, L, U, l, u, nrs, num as *const libc::c_int, max_pass)
        != 0
    {
        ret = 1 as libc::c_int;
    } else {
        i = 1 as libc::c_int;
        while i <= m {
            if glp_get_row_stat(mip, i) == 1 as libc::c_int {
                if *L.offset(i as isize) == -1.7976931348623157e+308f64
                    && *U.offset(i as isize) == 1.7976931348623157e+308f64
                {
                    glp_set_row_bnds(mip, i, 1 as libc::c_int, 0.0f64, 0.0f64);
                } else if *U.offset(i as isize) == 1.7976931348623157e+308f64 {
                    glp_set_row_bnds(
                        mip,
                        i,
                        2 as libc::c_int,
                        *L.offset(i as isize),
                        0.0f64,
                    );
                } else if *L.offset(i as isize) == -1.7976931348623157e+308f64 {
                    glp_set_row_bnds(
                        mip,
                        i,
                        3 as libc::c_int,
                        0.0f64,
                        *U.offset(i as isize),
                    );
                }
            }
            i += 1;
            i;
        }
        j = 1 as libc::c_int;
        while j <= n {
            let mut type_0: libc::c_int = 0;
            if *l.offset(j as isize) == -1.7976931348623157e+308f64
                && *u.offset(j as isize) == 1.7976931348623157e+308f64
            {
                type_0 = 1 as libc::c_int;
            } else if *u.offset(j as isize) == 1.7976931348623157e+308f64 {
                type_0 = 2 as libc::c_int;
            } else if *l.offset(j as isize) == -1.7976931348623157e+308f64 {
                type_0 = 3 as libc::c_int;
            } else if *l.offset(j as isize) != *u.offset(j as isize) {
                type_0 = 4 as libc::c_int;
            } else {
                type_0 = 5 as libc::c_int;
            }
            glp_set_col_bnds(
                mip,
                j,
                type_0,
                *l.offset(j as isize),
                *u.offset(j as isize),
            );
            j += 1;
            j;
        }
    }
    glp_free(L as *mut libc::c_void);
    glp_free(U as *mut libc::c_void);
    glp_free(l as *mut libc::c_void);
    glp_free(u as *mut libc::c_void);
    glp_free(num as *mut libc::c_void);
    return ret;
}
