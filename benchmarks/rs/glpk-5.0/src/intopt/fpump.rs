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
    fn floor(_: libc::c_double) -> libc::c_double;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_time() -> libc::c_double;
    fn glp_difftime(t1: libc::c_double, t0: libc::c_double) -> libc::c_double;
    fn glp_create_prob() -> *mut glp_prob;
    fn glp_add_rows(P: *mut glp_prob, nrs: libc::c_int) -> libc::c_int;
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
    fn glp_copy_prob(dest: *mut glp_prob, prob: *mut glp_prob, names: libc::c_int);
    fn glp_delete_prob(P: *mut glp_prob);
    fn glp_simplex(P: *mut glp_prob, parm: *const glp_smcp) -> libc::c_int;
    fn glp_init_smcp(parm: *mut glp_smcp);
    fn glp_get_status(P: *mut glp_prob) -> libc::c_int;
    fn glp_ios_heur_sol(T: *mut glp_tree, x: *const libc::c_double) -> libc::c_int;
    fn _glp_ios_is_hopeful(tree: *mut glp_tree, bound: libc::c_double) -> libc::c_int;
    fn _glp_rng_create_rand() -> *mut RNG;
    fn _glp_rng_delete_rand(rand: *mut RNG);
    fn _glp_rng_uniform(
        rand: *mut RNG,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub struct RNG {
    pub A: [libc::c_int; 56],
    pub fptr: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VAR {
    pub j: libc::c_int,
    pub x: libc::c_int,
    pub d: libc::c_double,
}
unsafe extern "C" fn fcmp(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> libc::c_int {
    let mut vx: *const VAR = x as *const VAR;
    let mut vy: *const VAR = y as *const VAR;
    if (*vx).d > (*vy).d {
        return -(1 as libc::c_int)
    } else if (*vx).d < (*vy).d {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
pub unsafe extern "C" fn _glp_ios_feas_pump(mut T: *mut glp_tree) {
    let mut current_block: u64;
    let mut P: *mut glp_prob = (*T).mip;
    let mut n: libc::c_int = (*P).n;
    let mut lp: *mut glp_prob = 0 as *mut glp_prob;
    let mut var: *mut VAR = 0 as *mut VAR;
    let mut rand: *mut RNG = 0 as *mut RNG;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
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
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut new_x: libc::c_int = 0;
    let mut nfail: libc::c_int = 0;
    let mut npass: libc::c_int = 0;
    let mut nv: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut stalling: libc::c_int = 0;
    let mut dist: libc::c_double = 0.;
    let mut tol: libc::c_double = 0.;
    (glp_get_status(P) == 5 as libc::c_int
        || {
            glp_assert_(
                b"glp_get_status(P) == GLP_OPT\0" as *const u8 as *const libc::c_char,
                b"intopt/fpump.c\0" as *const u8 as *const libc::c_char,
                77 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*(*T).curr).level == 0 as libc::c_int && (*(*T).curr).solved == 1 as libc::c_int
    {
        nv = 0 as libc::c_int;
        j = 1 as libc::c_int;
        loop {
            if !(j <= n) {
                current_block = 1054647088692577877;
                break;
            }
            col = *((*P).col).offset(j as isize);
            if !((*col).kind == 1 as libc::c_int) {
                if !((*col).type_0 == 5 as libc::c_int) {
                    ((*col).kind == 2 as libc::c_int
                        || {
                            glp_assert_(
                                b"col->kind == GLP_IV\0" as *const u8
                                    as *const libc::c_char,
                                b"intopt/fpump.c\0" as *const u8 as *const libc::c_char,
                                89 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    if (*col).type_0 == 4 as libc::c_int && (*col).lb == 0.0f64
                        && (*col).ub == 1.0f64
                    {
                        nv += 1;
                        nv;
                    } else {
                        if (*(*T).parm).msg_lev >= 3 as libc::c_int {
                            glp_printf(
                                b"FPUMP heuristic cannot be applied due to general integer variables\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        current_block = 12327285569713290383;
                        break;
                    }
                }
            }
            j += 1;
            j;
        }
        match current_block {
            12327285569713290383 => {}
            _ => {
                if !(nv == 0 as libc::c_int) {
                    if (*(*T).parm).msg_lev >= 3 as libc::c_int {
                        glp_printf(
                            b"Applying FPUMP heuristic...\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    var = glp_alloc(
                        1 as libc::c_int + nv,
                        ::std::mem::size_of::<VAR>() as libc::c_ulong as libc::c_int,
                    ) as *mut VAR;
                    k = 0 as libc::c_int;
                    j = 1 as libc::c_int;
                    while j <= n {
                        col = *((*P).col).offset(j as isize);
                        if (*col).kind == 2 as libc::c_int
                            && (*col).type_0 == 4 as libc::c_int
                        {
                            k += 1;
                            (*var.offset(k as isize)).j = j;
                        }
                        j += 1;
                        j;
                    }
                    (k == nv
                        || {
                            glp_assert_(
                                b"k == nv\0" as *const u8 as *const libc::c_char,
                                b"intopt/fpump.c\0" as *const u8 as *const libc::c_char,
                                114 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    lp = glp_create_prob();
                    '_more: loop {
                        glp_copy_prob(lp, P, 0 as libc::c_int);
                        if (*P).mip_stat == 2 as libc::c_int {
                            let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
                            let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
                            let mut bnd: libc::c_double = 0.;
                            glp_add_rows(lp, 1 as libc::c_int);
                            ind = glp_alloc(
                                1 as libc::c_int + n,
                                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int,
                            ) as *mut libc::c_int;
                            val = glp_alloc(
                                1 as libc::c_int + n,
                                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                                    as libc::c_int,
                            ) as *mut libc::c_double;
                            j = 1 as libc::c_int;
                            while j <= n {
                                *ind.offset(j as isize) = j;
                                *val
                                    .offset(
                                        j as isize,
                                    ) = (**((*P).col).offset(j as isize)).coef;
                                j += 1;
                                j;
                            }
                            glp_set_mat_row(
                                lp,
                                (*lp).m,
                                n,
                                ind as *const libc::c_int,
                                val as *const libc::c_double,
                            );
                            glp_free(ind as *mut libc::c_void);
                            glp_free(val as *mut libc::c_void);
                            bnd = 0.1f64 * (*P).obj_val + 0.9f64 * (*P).mip_obj;
                            if (*P).dir == 1 as libc::c_int {
                                glp_set_row_bnds(
                                    lp,
                                    (*lp).m,
                                    3 as libc::c_int,
                                    0.0f64,
                                    bnd - (*P).c0,
                                );
                            } else if (*P).dir == 2 as libc::c_int {
                                glp_set_row_bnds(
                                    lp,
                                    (*lp).m,
                                    2 as libc::c_int,
                                    bnd - (*P).c0,
                                    0.0f64,
                                );
                            } else {
                                (P != P
                                    || {
                                        glp_assert_(
                                            b"P != P\0" as *const u8 as *const libc::c_char,
                                            b"intopt/fpump.c\0" as *const u8 as *const libc::c_char,
                                            160 as libc::c_int,
                                        );
                                        1 as libc::c_int != 0
                                    }) as libc::c_int;
                            }
                        }
                        npass = 0 as libc::c_int;
                        k = 1 as libc::c_int;
                        while k <= nv {
                            (*var.offset(k as isize)).x = -(1 as libc::c_int);
                            k += 1;
                            k;
                        }
                        '_pass: loop {
                            npass += 1;
                            npass;
                            if (*(*T).parm).msg_lev >= 3 as libc::c_int {
                                glp_printf(
                                    b"Pass %d\n\0" as *const u8 as *const libc::c_char,
                                    npass,
                                );
                            }
                            dist = 1.7976931348623157e+308f64;
                            nfail = 0 as libc::c_int;
                            if npass > 1 as libc::c_int {
                                let mut rho: libc::c_double = 0.;
                                let mut temp: libc::c_double = 0.;
                                if rand.is_null() {
                                    rand = _glp_rng_create_rand();
                                }
                                k = 1 as libc::c_int;
                                while k <= nv {
                                    j = (*var.offset(k as isize)).j;
                                    col = *((*lp).col).offset(j as isize);
                                    rho = _glp_rng_uniform(rand, -0.3f64, 0.7f64);
                                    if rho < 0.0f64 {
                                        rho = 0.0f64;
                                    }
                                    temp = fabs(
                                        (*var.offset(k as isize)).x as libc::c_double - (*col).prim,
                                    );
                                    if temp + rho > 0.5f64 {
                                        (*var.offset(k as isize))
                                            .x = 1 as libc::c_int - (*var.offset(k as isize)).x;
                                    }
                                    k += 1;
                                    k;
                                }
                                current_block = 15724842116480786263;
                            } else {
                                current_block = 11224097781814090116;
                            }
                            loop {
                                match current_block {
                                    11224097781814090116 => {
                                        stalling = 1 as libc::c_int;
                                        k = 1 as libc::c_int;
                                        while k <= nv {
                                            col = *((*lp).col)
                                                .offset((*var.offset(k as isize)).j as isize);
                                            if (*col).prim < 0.5f64 {
                                                new_x = 0 as libc::c_int;
                                            } else {
                                                new_x = 1 as libc::c_int;
                                            }
                                            if (*var.offset(k as isize)).x != new_x {
                                                stalling = 0 as libc::c_int;
                                                (*var.offset(k as isize)).x = new_x;
                                            }
                                            k += 1;
                                            k;
                                        }
                                        if stalling != 0 {
                                            k = 1 as libc::c_int;
                                            while k <= nv {
                                                col = *((*lp).col)
                                                    .offset((*var.offset(k as isize)).j as isize);
                                                (*var.offset(k as isize))
                                                    .d = fabs(
                                                    (*col).prim - (*var.offset(k as isize)).x as libc::c_double,
                                                );
                                                k += 1;
                                                k;
                                            }
                                            qsort(
                                                &mut *var.offset(1 as libc::c_int as isize) as *mut VAR
                                                    as *mut libc::c_void,
                                                nv as size_t,
                                                ::std::mem::size_of::<VAR>() as libc::c_ulong,
                                                Some(
                                                    fcmp
                                                        as unsafe extern "C" fn(
                                                            *const libc::c_void,
                                                            *const libc::c_void,
                                                        ) -> libc::c_int,
                                                ),
                                            );
                                            k = 1 as libc::c_int;
                                            while k <= nv {
                                                if k >= 5 as libc::c_int
                                                    && (*var.offset(k as isize)).d < 0.35f64
                                                    || k >= 10 as libc::c_int
                                                {
                                                    break;
                                                }
                                                (*var.offset(k as isize))
                                                    .x = 1 as libc::c_int - (*var.offset(k as isize)).x;
                                                k += 1;
                                                k;
                                            }
                                        }
                                        current_block = 15724842116480786263;
                                    }
                                    _ => {
                                        if (*(*T).parm).tm_lim < 2147483647 as libc::c_int
                                            && ((*(*T).parm).tm_lim - 1 as libc::c_int)
                                                as libc::c_double
                                                <= 1000.0f64 * glp_difftime(glp_time(), (*T).tm_beg)
                                        {
                                            break '_more;
                                        }
                                        (*lp).dir = 1 as libc::c_int;
                                        (*lp).c0 = 0.0f64;
                                        j = 1 as libc::c_int;
                                        while j <= n {
                                            (**((*lp).col).offset(j as isize)).coef = 0.0f64;
                                            j += 1;
                                            j;
                                        }
                                        k = 1 as libc::c_int;
                                        while k <= nv {
                                            j = (*var.offset(k as isize)).j;
                                            if (*var.offset(k as isize)).x == 0 as libc::c_int {
                                                (**((*lp).col).offset(j as isize)).coef = 1.0f64;
                                            } else {
                                                (**((*lp).col).offset(j as isize)).coef = -1.0f64;
                                                (*lp).c0 += 1.0f64;
                                            }
                                            k += 1;
                                            k;
                                        }
                                        glp_init_smcp(&mut parm);
                                        if (*(*T).parm).msg_lev <= 1 as libc::c_int {
                                            parm.msg_lev = (*(*T).parm).msg_lev;
                                        } else if (*(*T).parm).msg_lev <= 3 as libc::c_int {
                                            parm.msg_lev = 2 as libc::c_int;
                                            parm.out_dly = 10000 as libc::c_int;
                                        }
                                        ret = glp_simplex(lp, &mut parm);
                                        if ret != 0 as libc::c_int {
                                            if (*(*T).parm).msg_lev >= 1 as libc::c_int {
                                                glp_printf(
                                                    b"Warning: glp_simplex returned %d\n\0" as *const u8
                                                        as *const libc::c_char,
                                                    ret,
                                                );
                                            }
                                            break '_more;
                                        } else {
                                            ret = glp_get_status(lp);
                                            if ret != 5 as libc::c_int {
                                                if (*(*T).parm).msg_lev >= 1 as libc::c_int {
                                                    glp_printf(
                                                        b"Warning: glp_get_status returned %d\n\0" as *const u8
                                                            as *const libc::c_char,
                                                        ret,
                                                    );
                                                }
                                                break '_more;
                                            } else {
                                                if (*(*T).parm).msg_lev >= 4 as libc::c_int {
                                                    glp_printf(
                                                        b"delta = %g\n\0" as *const u8 as *const libc::c_char,
                                                        (*lp).obj_val,
                                                    );
                                                }
                                                tol = 0.3f64 * (*(*T).parm).tol_int;
                                                k = 1 as libc::c_int;
                                                while k <= nv {
                                                    col = *((*lp).col)
                                                        .offset((*var.offset(k as isize)).j as isize);
                                                    if tol < (*col).prim && (*col).prim < 1.0f64 - tol {
                                                        break;
                                                    }
                                                    k += 1;
                                                    k;
                                                }
                                                if k > nv {
                                                    let mut x: *mut libc::c_double = glp_alloc(
                                                        1 as libc::c_int + n,
                                                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                                                            as libc::c_int,
                                                    ) as *mut libc::c_double;
                                                    j = 1 as libc::c_int;
                                                    while j <= n {
                                                        *x
                                                            .offset(
                                                                j as isize,
                                                            ) = (**((*lp).col).offset(j as isize)).prim;
                                                        if (**((*P).col).offset(j as isize)).kind
                                                            == 2 as libc::c_int
                                                        {
                                                            *x
                                                                .offset(j as isize) = floor(*x.offset(j as isize) + 0.5f64);
                                                        }
                                                        j += 1;
                                                        j;
                                                    }
                                                    (*lp).c0 = (*P).c0;
                                                    (*lp).dir = (*P).dir;
                                                    k = 1 as libc::c_int;
                                                    while k <= nv {
                                                        glp_set_col_bnds(
                                                            lp,
                                                            (*var.offset(k as isize)).j,
                                                            5 as libc::c_int,
                                                            *x.offset((*var.offset(k as isize)).j as isize),
                                                            0.0f64,
                                                        );
                                                        k += 1;
                                                        k;
                                                    }
                                                    j = 1 as libc::c_int;
                                                    while j <= n {
                                                        (**((*lp).col).offset(j as isize))
                                                            .coef = (**((*P).col).offset(j as isize)).coef;
                                                        j += 1;
                                                        j;
                                                    }
                                                    ret = glp_simplex(lp, &mut parm);
                                                    if ret != 0 as libc::c_int {
                                                        if (*(*T).parm).msg_lev >= 1 as libc::c_int {
                                                            glp_printf(
                                                                b"Warning: glp_simplex returned %d\n\0" as *const u8
                                                                    as *const libc::c_char,
                                                                ret,
                                                            );
                                                        }
                                                        glp_free(x as *mut libc::c_void);
                                                        break '_more;
                                                    } else {
                                                        ret = glp_get_status(lp);
                                                        if ret != 5 as libc::c_int {
                                                            if (*(*T).parm).msg_lev >= 1 as libc::c_int {
                                                                glp_printf(
                                                                    b"Warning: glp_get_status returned %d\n\0" as *const u8
                                                                        as *const libc::c_char,
                                                                    ret,
                                                                );
                                                            }
                                                            glp_free(x as *mut libc::c_void);
                                                            break '_more;
                                                        } else {
                                                            j = 1 as libc::c_int;
                                                            while j <= n {
                                                                if (**((*P).col).offset(j as isize)).kind
                                                                    != 2 as libc::c_int
                                                                {
                                                                    *x
                                                                        .offset(
                                                                            j as isize,
                                                                        ) = (**((*lp).col).offset(j as isize)).prim;
                                                                }
                                                                j += 1;
                                                                j;
                                                            }
                                                            ret = glp_ios_heur_sol(T, x as *const libc::c_double);
                                                            glp_free(x as *mut libc::c_void);
                                                            if ret == 0 as libc::c_int {
                                                                if _glp_ios_is_hopeful(T, (*(*T).curr).bound) != 0 {
                                                                    break '_pass;
                                                                } else {
                                                                    break '_more;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                if dist == 1.7976931348623157e+308f64
                                                    || (*lp).obj_val <= dist - 1e-6f64 * (1.0f64 + dist)
                                                {
                                                    nfail = 0 as libc::c_int;
                                                    dist = (*lp).obj_val;
                                                } else {
                                                    nfail += 1;
                                                    nfail;
                                                }
                                                if nfail < 3 as libc::c_int {
                                                    current_block = 11224097781814090116;
                                                    continue;
                                                }
                                                if npass < 5 as libc::c_int {
                                                    break;
                                                } else {
                                                    break '_more;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !lp.is_null() {
        glp_delete_prob(lp);
    }
    if !var.is_null() {
        glp_free(var as *mut libc::c_void);
    }
    if !rand.is_null() {
        _glp_rng_delete_rand(rand);
    }
}
