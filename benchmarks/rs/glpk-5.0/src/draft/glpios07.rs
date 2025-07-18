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
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_get_num_rows(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_num_cols(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_row_type(P: *mut glp_prob, i: libc::c_int) -> libc::c_int;
    fn glp_get_row_lb(P: *mut glp_prob, i: libc::c_int) -> libc::c_double;
    fn glp_get_row_ub(P: *mut glp_prob, i: libc::c_int) -> libc::c_double;
    fn glp_get_col_type(P: *mut glp_prob, j: libc::c_int) -> libc::c_int;
    fn glp_get_col_lb(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
    fn glp_get_col_ub(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
    fn glp_get_mat_row(
        P: *mut glp_prob,
        i: libc::c_int,
        ind: *mut libc::c_int,
        val: *mut libc::c_double,
    ) -> libc::c_int;
    fn glp_get_status(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_col_prim(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
    fn glp_get_col_kind(P: *mut glp_prob, j: libc::c_int) -> libc::c_int;
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
unsafe extern "C" fn cover2(
    mut n: libc::c_int,
    mut a: *mut libc::c_double,
    mut b: libc::c_double,
    mut u: libc::c_double,
    mut x: *mut libc::c_double,
    mut y: libc::c_double,
    mut cov: *mut libc::c_int,
    mut _alfa: *mut libc::c_double,
    mut _beta: *mut libc::c_double,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut try_0: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut eps: libc::c_double = 0.;
    let mut alfa: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut rmax: libc::c_double = 0.001f64;
    eps = 0.001f64 * (1.0f64 + fabs(b));
    i = 0 as libc::c_int + 1 as libc::c_int;
    's_9: while i <= n {
        j = i + 1 as libc::c_int;
        while j <= n {
            try_0 += 1;
            try_0;
            if try_0 > 1000 as libc::c_int {
                break 's_9;
            }
            if *a.offset(i as isize) + *a.offset(j as isize) + y > b + eps {
                temp = *a.offset(i as isize) + *a.offset(j as isize) - b;
                alfa = 1.0f64 / (temp + u);
                beta = 2.0f64 - alfa * temp;
                temp = *x.offset(i as isize) + *x.offset(j as isize) + alfa * y - beta;
                if rmax < temp {
                    rmax = temp;
                    *cov.offset(1 as libc::c_int as isize) = i;
                    *cov.offset(2 as libc::c_int as isize) = j;
                    *_alfa = alfa;
                    *_beta = beta;
                    ret = 1 as libc::c_int;
                }
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return ret;
}
unsafe extern "C" fn cover3(
    mut n: libc::c_int,
    mut a: *mut libc::c_double,
    mut b: libc::c_double,
    mut u: libc::c_double,
    mut x: *mut libc::c_double,
    mut y: libc::c_double,
    mut cov: *mut libc::c_int,
    mut _alfa: *mut libc::c_double,
    mut _beta: *mut libc::c_double,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut try_0: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut eps: libc::c_double = 0.;
    let mut alfa: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut rmax: libc::c_double = 0.001f64;
    eps = 0.001f64 * (1.0f64 + fabs(b));
    i = 0 as libc::c_int + 1 as libc::c_int;
    's_9: while i <= n {
        j = i + 1 as libc::c_int;
        while j <= n {
            k = j + 1 as libc::c_int;
            while k <= n {
                try_0 += 1;
                try_0;
                if try_0 > 1000 as libc::c_int {
                    break 's_9;
                }
                if *a.offset(i as isize) + *a.offset(j as isize) + *a.offset(k as isize)
                    + y > b + eps
                {
                    temp = *a.offset(i as isize) + *a.offset(j as isize)
                        + *a.offset(k as isize) - b;
                    alfa = 1.0f64 / (temp + u);
                    beta = 3.0f64 - alfa * temp;
                    temp = *x.offset(i as isize) + *x.offset(j as isize)
                        + *x.offset(k as isize) + alfa * y - beta;
                    if rmax < temp {
                        rmax = temp;
                        *cov.offset(1 as libc::c_int as isize) = i;
                        *cov.offset(2 as libc::c_int as isize) = j;
                        *cov.offset(3 as libc::c_int as isize) = k;
                        *_alfa = alfa;
                        *_beta = beta;
                        ret = 1 as libc::c_int;
                    }
                }
                k += 1;
                k;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return ret;
}
unsafe extern "C" fn cover4(
    mut n: libc::c_int,
    mut a: *mut libc::c_double,
    mut b: libc::c_double,
    mut u: libc::c_double,
    mut x: *mut libc::c_double,
    mut y: libc::c_double,
    mut cov: *mut libc::c_int,
    mut _alfa: *mut libc::c_double,
    mut _beta: *mut libc::c_double,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut try_0: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut eps: libc::c_double = 0.;
    let mut alfa: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut rmax: libc::c_double = 0.001f64;
    eps = 0.001f64 * (1.0f64 + fabs(b));
    i = 0 as libc::c_int + 1 as libc::c_int;
    's_9: while i <= n {
        j = i + 1 as libc::c_int;
        while j <= n {
            k = j + 1 as libc::c_int;
            while k <= n {
                l = k + 1 as libc::c_int;
                while l <= n {
                    try_0 += 1;
                    try_0;
                    if try_0 > 1000 as libc::c_int {
                        break 's_9;
                    }
                    if *a.offset(i as isize) + *a.offset(j as isize)
                        + *a.offset(k as isize) + *a.offset(l as isize) + y > b + eps
                    {
                        temp = *a.offset(i as isize) + *a.offset(j as isize)
                            + *a.offset(k as isize) + *a.offset(l as isize) - b;
                        alfa = 1.0f64 / (temp + u);
                        beta = 4.0f64 - alfa * temp;
                        temp = *x.offset(i as isize) + *x.offset(j as isize)
                            + *x.offset(k as isize) + *x.offset(l as isize) + alfa * y
                            - beta;
                        if rmax < temp {
                            rmax = temp;
                            *cov.offset(1 as libc::c_int as isize) = i;
                            *cov.offset(2 as libc::c_int as isize) = j;
                            *cov.offset(3 as libc::c_int as isize) = k;
                            *cov.offset(4 as libc::c_int as isize) = l;
                            *_alfa = alfa;
                            *_beta = beta;
                            ret = 1 as libc::c_int;
                        }
                    }
                    l += 1;
                    l;
                }
                k += 1;
                k;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return ret;
}
unsafe extern "C" fn cover(
    mut n: libc::c_int,
    mut a: *mut libc::c_double,
    mut b: libc::c_double,
    mut u: libc::c_double,
    mut x: *mut libc::c_double,
    mut y: libc::c_double,
    mut cov: *mut libc::c_int,
    mut alfa: *mut libc::c_double,
    mut beta: *mut libc::c_double,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    (n >= 2 as libc::c_int
        || {
            glp_assert_(
                b"n >= 2\0" as *const u8 as *const libc::c_char,
                b"draft/glpios07.c\0" as *const u8 as *const libc::c_char,
                261 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    j = 1 as libc::c_int;
    while j <= n {
        (*a.offset(j as isize) > 0.0f64
            || {
                glp_assert_(
                    b"a[j] > 0.0\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios07.c\0" as *const u8 as *const libc::c_char,
                    262 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        j += 1;
        j;
    }
    (b > -1e-5f64
        || {
            glp_assert_(
                b"b > -1e-5\0" as *const u8 as *const libc::c_char,
                b"draft/glpios07.c\0" as *const u8 as *const libc::c_char,
                264 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (u >= 0.0f64
        || {
            glp_assert_(
                b"u >= 0.0\0" as *const u8 as *const libc::c_char,
                b"draft/glpios07.c\0" as *const u8 as *const libc::c_char,
                268 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    j = 1 as libc::c_int;
    while j <= n {
        (0.0f64 <= *x.offset(j as isize) && *x.offset(j as isize) <= 1.0f64
            || {
                glp_assert_(
                    b"0.0 <= x[j] && x[j] <= 1.0\0" as *const u8 as *const libc::c_char,
                    b"draft/glpios07.c\0" as *const u8 as *const libc::c_char,
                    269 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        j += 1;
        j;
    }
    (0.0f64 <= y && y <= u
        || {
            glp_assert_(
                b"0.0 <= y && y <= u\0" as *const u8 as *const libc::c_char,
                b"draft/glpios07.c\0" as *const u8 as *const libc::c_char,
                270 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if cover2(n, a, b, u, x, y, cov, alfa, beta) != 0 {
        return 2 as libc::c_int;
    }
    if cover3(n, a, b, u, x, y, cov, alfa, beta) != 0 {
        return 3 as libc::c_int;
    }
    if cover4(n, a, b, u, x, y, cov, alfa, beta) != 0 {
        return 4 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn lpx_cover_cut(
    mut lp: *mut glp_prob,
    mut len: libc::c_int,
    mut ind: *mut libc::c_int,
    mut val: *mut libc::c_double,
    mut work: *mut libc::c_double,
) -> libc::c_int {
    let mut cov: [libc::c_int; 5] = [0; 5];
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nb: libc::c_int = 0;
    let mut newlen: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut f_min: libc::c_double = 0.;
    let mut f_max: libc::c_double = 0.;
    let mut alfa: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut x: *mut libc::c_double = work;
    let mut y: libc::c_double = 0.;
    newlen = 0 as libc::c_int;
    k = 1 as libc::c_int;
    while k <= len {
        j = *ind.offset(k as isize);
        if glp_get_col_type(lp, j) == 5 as libc::c_int {
            *val.offset(0 as libc::c_int as isize)
                -= *val.offset(k as isize) * glp_get_col_lb(lp, j);
        } else {
            newlen += 1;
            newlen;
            *ind.offset(newlen as isize) = *ind.offset(k as isize);
            *val.offset(newlen as isize) = *val.offset(k as isize);
        }
        k += 1;
        k;
    }
    len = newlen;
    nb = 0 as libc::c_int;
    k = 1 as libc::c_int;
    while k <= len {
        j = *ind.offset(k as isize);
        if glp_get_col_kind(lp, j) == 3 as libc::c_int {
            let mut ind_k: libc::c_int = 0;
            let mut val_k: libc::c_double = 0.;
            nb += 1;
            nb;
            ind_k = *ind.offset(nb as isize);
            val_k = *val.offset(nb as isize);
            *ind.offset(nb as isize) = *ind.offset(k as isize);
            *val.offset(nb as isize) = *val.offset(k as isize);
            *ind.offset(k as isize) = ind_k;
            *val.offset(k as isize) = val_k;
        }
        k += 1;
        k;
    }
    if nb < 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    f_max = 0.0f64;
    f_min = f_max;
    k = nb + 1 as libc::c_int;
    while k <= len {
        j = *ind.offset(k as isize);
        if glp_get_col_type(lp, j) != 4 as libc::c_int {
            return 0 as libc::c_int;
        }
        if *val.offset(k as isize) > 0.0f64 {
            f_min += *val.offset(k as isize) * glp_get_col_lb(lp, j);
            f_max += *val.offset(k as isize) * glp_get_col_ub(lp, j);
        } else {
            f_min += *val.offset(k as isize) * glp_get_col_ub(lp, j);
            f_max += *val.offset(k as isize) * glp_get_col_lb(lp, j);
        }
        k += 1;
        k;
    }
    u = f_max - f_min;
    y = 0.0f64;
    k = nb + 1 as libc::c_int;
    while k <= len {
        j = *ind.offset(k as isize);
        y += *val.offset(k as isize) * glp_get_col_prim(lp, j);
        k += 1;
        k;
    }
    y -= f_min;
    if y < 0.0f64 {
        y = 0.0f64;
    }
    if y > u {
        y = u;
    }
    *val.offset(0 as libc::c_int as isize) -= f_min;
    k = 1 as libc::c_int;
    while k <= nb {
        j = *ind.offset(k as isize);
        *x.offset(k as isize) = glp_get_col_prim(lp, j);
        if *x.offset(k as isize) < 0.0f64 {
            *x.offset(k as isize) = 0.0f64;
        }
        if *x.offset(k as isize) > 1.0f64 {
            *x.offset(k as isize) = 1.0f64;
        }
        k += 1;
        k;
    }
    k = 1 as libc::c_int;
    while k <= nb {
        if *val.offset(k as isize) < 0.0f64 {
            *ind.offset(k as isize) = -*ind.offset(k as isize);
            *val.offset(k as isize) = -*val.offset(k as isize);
            *val.offset(0 as libc::c_int as isize) += *val.offset(k as isize);
            *x.offset(k as isize) = 1.0f64 - *x.offset(k as isize);
        }
        k += 1;
        k;
    }
    r = cover(
        nb,
        val,
        *val.offset(0 as libc::c_int as isize),
        u,
        x,
        y,
        cov.as_mut_ptr(),
        &mut alfa,
        &mut beta,
    );
    if r == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    (2 as libc::c_int <= r && r <= 4 as libc::c_int
        || {
            glp_assert_(
                b"2 <= r && r <= 4\0" as *const u8 as *const libc::c_char,
                b"draft/glpios07.c\0" as *const u8 as *const libc::c_char,
                407 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    *ind.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    *val.offset(0 as libc::c_int as isize) = beta;
    j = 1 as libc::c_int;
    while j <= r {
        cov[j as usize] = *ind.offset(cov[j as usize] as isize);
        j += 1;
        j;
    }
    (r <= nb
        || {
            glp_assert_(
                b"r <= nb\0" as *const u8 as *const libc::c_char,
                b"draft/glpios07.c\0" as *const u8 as *const libc::c_char,
                416 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = 1 as libc::c_int;
    while k <= r {
        if cov[k as usize] > 0 as libc::c_int {
            *ind.offset(k as isize) = cov[k as usize];
            *val.offset(k as isize) = 1.0f64;
        } else {
            *ind.offset(k as isize) = -cov[k as usize];
            *val.offset(k as isize) = -1.0f64;
            *val.offset(0 as libc::c_int as isize) -= 1.0f64;
        }
        k += 1;
        k;
    }
    k = nb + 1 as libc::c_int;
    while k <= len {
        r += 1;
        r;
        *ind.offset(r as isize) = *ind.offset(k as isize);
        *val.offset(r as isize) = alfa * *val.offset(k as isize);
        k += 1;
        k;
    }
    *val.offset(0 as libc::c_int as isize) += alfa * f_min;
    (r <= len
        || {
            glp_assert_(
                b"r <= len\0" as *const u8 as *const libc::c_char,
                b"draft/glpios07.c\0" as *const u8 as *const libc::c_char,
                435 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    len = r;
    return len;
}
unsafe extern "C" fn lpx_eval_row(
    mut lp: *mut glp_prob,
    mut len: libc::c_int,
    mut ind: *mut libc::c_int,
    mut val: *mut libc::c_double,
) -> libc::c_double {
    let mut n: libc::c_int = glp_get_num_cols(lp);
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut sum: libc::c_double = 0.0f64;
    if len < 0 as libc::c_int {
        (glp_error_(
            b"draft/glpios07.c\0" as *const u8 as *const libc::c_char,
            475 as libc::c_int,
        ))
            .unwrap()(
            b"lpx_eval_row: len = %d; invalid row length\n\0" as *const u8
                as *const libc::c_char,
            len,
        );
    }
    k = 1 as libc::c_int;
    while k <= len {
        j = *ind.offset(k as isize);
        if !(1 as libc::c_int <= j && j <= n) {
            (glp_error_(
                b"draft/glpios07.c\0" as *const u8 as *const libc::c_char,
                479 as libc::c_int,
            ))
                .unwrap()(
                b"lpx_eval_row: j = %d; column number out of range\n\0" as *const u8
                    as *const libc::c_char,
                j,
            );
        }
        sum += *val.offset(k as isize) * glp_get_col_prim(lp, j);
        k += 1;
        k;
    }
    return sum;
}
pub unsafe extern "C" fn _glp_ios_cov_gen(mut tree: *mut glp_tree) {
    let mut prob: *mut glp_prob = (*tree).mip;
    let mut m: libc::c_int = glp_get_num_rows(prob);
    let mut n: libc::c_int = glp_get_num_cols(prob);
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut kase: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut r: libc::c_double = 0.;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut work: *mut libc::c_double = 0 as *mut libc::c_double;
    (glp_get_status(prob) == 5 as libc::c_int
        || {
            glp_assert_(
                b"glp_get_status(prob) == GLP_OPT\0" as *const u8 as *const libc::c_char,
                b"draft/glpios07.c\0" as *const u8 as *const libc::c_char,
                507 as libc::c_int,
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
    work = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    i = 1 as libc::c_int;
    while i <= m {
        let mut current_block_16: u64;
        kase = 1 as libc::c_int;
        while kase <= 2 as libc::c_int {
            type_0 = glp_get_row_type(prob, i);
            if kase == 1 as libc::c_int {
                if !(type_0 == 3 as libc::c_int || type_0 == 4 as libc::c_int) {
                    current_block_16 = 2473556513754201174;
                } else {
                    len = glp_get_mat_row(prob, i, ind, val);
                    *val.offset(0 as libc::c_int as isize) = glp_get_row_ub(prob, i);
                    current_block_16 = 17833034027772472439;
                }
            } else if !(type_0 == 2 as libc::c_int || type_0 == 4 as libc::c_int) {
                current_block_16 = 2473556513754201174;
            } else {
                len = glp_get_mat_row(prob, i, ind, val);
                k = 1 as libc::c_int;
                while k <= len {
                    *val.offset(k as isize) = -*val.offset(k as isize);
                    k += 1;
                    k;
                }
                *val.offset(0 as libc::c_int as isize) = -glp_get_row_lb(prob, i);
                current_block_16 = 17833034027772472439;
            }
            match current_block_16 {
                17833034027772472439 => {
                    len = lpx_cover_cut(prob, len, ind, val, work);
                    if !(len == 0 as libc::c_int) {
                        r = lpx_eval_row(prob, len, ind, val)
                            - *val.offset(0 as libc::c_int as isize);
                        if !(r < 1e-3f64) {
                            glp_ios_add_row(
                                tree,
                                0 as *const libc::c_char,
                                3 as libc::c_int,
                                0 as libc::c_int,
                                len,
                                ind as *const libc::c_int,
                                val as *const libc::c_double,
                                3 as libc::c_int,
                                *val.offset(0 as libc::c_int as isize),
                            );
                        }
                    }
                }
                _ => {}
            }
            kase += 1;
            kase;
        }
        i += 1;
        i;
    }
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
    glp_free(work as *mut libc::c_void);
}
