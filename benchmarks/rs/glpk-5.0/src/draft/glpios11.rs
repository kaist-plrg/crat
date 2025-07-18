use ::libc;
extern "C" {
    pub type BFD;
    pub type AVL;
    pub type AVLNODE;
    pub type glp_cfg;
    pub type glp_mir;
    pub type glp_cov;
    pub type DMP;
    fn glp_add_rows(P: *mut glp_prob, nrs: libc::c_int) -> libc::c_int;
    fn glp_set_row_name(P: *mut glp_prob, i: libc::c_int, name: *const libc::c_char);
    fn glp_set_row_bnds(
        P: *mut glp_prob,
        i: libc::c_int,
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
    fn glp_transform_row(
        P: *mut glp_prob,
        len: libc::c_int,
        ind: *mut libc::c_int,
        val: *mut libc::c_double,
    ) -> libc::c_int;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn _glp_analyze_row(
        P: *mut glp_prob,
        len: libc::c_int,
        ind: *const libc::c_int,
        val: *const libc::c_double,
        type_0: libc::c_int,
        rhs: libc::c_double,
        eps: libc::c_double,
        _piv: *mut libc::c_int,
        _x: *mut libc::c_double,
        _dx: *mut libc::c_double,
        _y: *mut libc::c_double,
        _dy: *mut libc::c_double,
        _dz: *mut libc::c_double,
    ) -> libc::c_int;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type IOSCUT = GLPROW;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct info {
    pub cut: *mut IOSCUT,
    pub flag: libc::c_char,
    pub eff: libc::c_double,
    pub deg: libc::c_double,
}
unsafe extern "C" fn fcmp(
    mut arg1: *const libc::c_void,
    mut arg2: *const libc::c_void,
) -> libc::c_int {
    let mut info1: *const info = arg1 as *const info;
    let mut info2: *const info = arg2 as *const info;
    if (*info1).deg == 0.0f64 && (*info2).deg == 0.0f64 {
        if (*info1).eff > (*info2).eff {
            return -(1 as libc::c_int);
        }
        if (*info1).eff < (*info2).eff {
            return 1 as libc::c_int;
        }
    } else {
        if (*info1).deg > (*info2).deg {
            return -(1 as libc::c_int);
        }
        if (*info1).deg < (*info2).deg {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_ios_process_cuts(mut T: *mut glp_tree) {
    let mut pool: *mut IOSPOOL = 0 as *mut IOSPOOL;
    let mut cut: *mut IOSCUT = 0 as *mut IOSCUT;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut info: *mut info = 0 as *mut info;
    let mut k: libc::c_int = 0;
    let mut kk: libc::c_int = 0;
    let mut max_cuts: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut work: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rhs: libc::c_double = 0.;
    (!((*T).curr).is_null()
        || {
            glp_assert_(
                b"T->curr != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios11.c\0" as *const u8 as *const libc::c_char,
                90 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    pool = (*T).local;
    (!pool.is_null()
        || {
            glp_assert_(
                b"pool != NULL\0" as *const u8 as *const libc::c_char,
                b"draft/glpios11.c\0" as *const u8 as *const libc::c_char,
                93 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*pool).m > 0 as libc::c_int
        || {
            glp_assert_(
                b"pool->m > 0\0" as *const u8 as *const libc::c_char,
                b"draft/glpios11.c\0" as *const u8 as *const libc::c_char,
                94 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    info = glp_alloc(
        1 as libc::c_int + (*pool).m,
        ::std::mem::size_of::<info>() as libc::c_ulong as libc::c_int,
    ) as *mut info;
    ind = glp_alloc(
        1 as libc::c_int + (*T).n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    val = glp_alloc(
        1 as libc::c_int + (*T).n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    work = glp_alloc(
        1 as libc::c_int + (*T).n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    k = 1 as libc::c_int;
    while k <= (*T).n {
        *work.offset(k as isize) = 0.0f64;
        k += 1;
        k;
    }
    k = 1 as libc::c_int;
    while k <= (*pool).m {
        let ref mut fresh0 = (*info.offset(k as isize)).cut;
        *fresh0 = *((*pool).row).offset(k as isize);
        (*info.offset(k as isize)).flag = 0 as libc::c_int as libc::c_char;
        k += 1;
        k;
    }
    k = 1 as libc::c_int;
    while k <= (*pool).m {
        let mut temp: libc::c_double = 0.;
        let mut dy: libc::c_double = 0.;
        let mut dz: libc::c_double = 0.;
        cut = (*info.offset(k as isize)).cut;
        len = 0 as libc::c_int;
        temp = 0.0f64;
        aij = (*cut).ptr;
        while !aij.is_null() {
            (1 as libc::c_int <= (*(*aij).col).j && (*(*aij).col).j <= (*T).n
                || {
                    glp_assert_(
                        b"1 <= aij->col->j && aij->col->j <= T->n\0" as *const u8
                            as *const libc::c_char,
                        b"draft/glpios11.c\0" as *const u8 as *const libc::c_char,
                        112 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            len += 1;
            len;
            *ind.offset(len as isize) = (*(*aij).col).j;
            *val.offset(len as isize) = (*aij).val;
            temp += (*aij).val * (*aij).val;
            aij = (*aij).r_next;
        }
        if temp < 2.2204460492503131e-16f64 * 2.2204460492503131e-16f64 {
            temp = 2.2204460492503131e-16f64;
        }
        len = glp_transform_row((*T).mip, len, ind, val);
        match (*cut).type_0 {
            2 => {
                rhs = (*cut).lb;
            }
            3 => {
                rhs = (*cut).ub;
            }
            _ => {
                (cut != cut
                    || {
                        glp_assert_(
                            b"cut != cut\0" as *const u8 as *const libc::c_char,
                            b"draft/glpios11.c\0" as *const u8 as *const libc::c_char,
                            126 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        ret = _glp_analyze_row(
            (*T).mip,
            len,
            ind as *const libc::c_int,
            val as *const libc::c_double,
            (*cut).type_0,
            rhs,
            1e-9f64,
            0 as *mut libc::c_int,
            0 as *mut libc::c_double,
            0 as *mut libc::c_double,
            0 as *mut libc::c_double,
            &mut dy,
            &mut dz,
        );
        if ret == 0 as libc::c_int {
            (*info.offset(k as isize)).eff = fabs(dy) / sqrt(temp);
            if (*(*T).mip).dir == 1 as libc::c_int {
                if dz < 0.0f64 {
                    dz = 0.0f64;
                }
                (*info.offset(k as isize)).deg = dz;
            } else {
                if dz > 0.0f64 {
                    dz = 0.0f64;
                }
                (*info.offset(k as isize)).deg = -dz;
            }
        } else if ret == 1 as libc::c_int {
            let ref mut fresh1 = (*info.offset(k as isize)).deg;
            *fresh1 = 0.0f64;
            (*info.offset(k as isize)).eff = *fresh1;
        } else if ret == 2 as libc::c_int {
            (*info.offset(k as isize)).eff = 1.0f64;
            (*info.offset(k as isize)).deg = 1.7976931348623157e+308f64;
        } else {
            (ret != ret
                || {
                    glp_assert_(
                        b"ret != ret\0" as *const u8 as *const libc::c_char,
                        b"draft/glpios11.c\0" as *const u8 as *const libc::c_char,
                        156 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        if (*info.offset(k as isize)).deg < 0.01f64 {
            (*info.offset(k as isize)).deg = 0.0f64;
        }
        k += 1;
        k;
    }
    qsort(
        &mut *info.offset(1 as libc::c_int as isize) as *mut info as *mut libc::c_void,
        (*pool).m as size_t,
        ::std::mem::size_of::<info>() as libc::c_ulong,
        Some(
            fcmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    max_cuts = if (*(*T).curr).level == 0 as libc::c_int {
        90 as libc::c_int
    } else {
        10 as libc::c_int
    };
    if max_cuts > (*pool).m {
        max_cuts = (*pool).m;
    }
    k = 1 as libc::c_int;
    while k <= max_cuts {
        let mut i: libc::c_int = 0;
        let mut len_0: libc::c_int = 0;
        if !((*info.offset(k as isize)).deg < 0.01f64
            && (*info.offset(k as isize)).eff < 0.01f64)
        {
            kk = 1 as libc::c_int;
            while kk < k {
                if (*info.offset(kk as isize)).flag != 0 {
                    if parallel(
                        (*info.offset(k as isize)).cut,
                        (*info.offset(kk as isize)).cut,
                        work,
                    ) > 0.90f64
                    {
                        break;
                    }
                }
                kk += 1;
                kk;
            }
            if !(kk < k) {
                cut = (*info.offset(k as isize)).cut;
                (*info.offset(k as isize)).flag = 1 as libc::c_int as libc::c_char;
                i = glp_add_rows((*T).mip, 1 as libc::c_int);
                if !((*cut).name).is_null() {
                    glp_set_row_name((*T).mip, i, (*cut).name);
                }
                ((**((*(*T).mip).row).offset(i as isize)).origin as libc::c_int
                    == 2 as libc::c_int
                    || {
                        glp_assert_(
                            b"T->mip->row[i]->origin == GLP_RF_CUT\0" as *const u8
                                as *const libc::c_char,
                            b"draft/glpios11.c\0" as *const u8 as *const libc::c_char,
                            192 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (**((*(*T).mip).row).offset(i as isize)).klass = (*cut).klass;
                len_0 = 0 as libc::c_int;
                aij = (*cut).ptr;
                while !aij.is_null() {
                    len_0 += 1;
                    len_0;
                    *ind.offset(len_0 as isize) = (*(*aij).col).j;
                    *val.offset(len_0 as isize) = (*aij).val;
                    aij = (*aij).r_next;
                }
                glp_set_mat_row(
                    (*T).mip,
                    i,
                    len_0,
                    ind as *const libc::c_int,
                    val as *const libc::c_double,
                );
                match (*cut).type_0 {
                    2 => {
                        rhs = (*cut).lb;
                    }
                    3 => {
                        rhs = (*cut).ub;
                    }
                    _ => {
                        (cut != cut
                            || {
                                glp_assert_(
                                    b"cut != cut\0" as *const u8 as *const libc::c_char,
                                    b"draft/glpios11.c\0" as *const u8 as *const libc::c_char,
                                    201 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                    }
                }
                glp_set_row_bnds((*T).mip, i, (*cut).type_0, rhs, rhs);
            }
        }
        k += 1;
        k;
    }
    glp_free(info as *mut libc::c_void);
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
    glp_free(work as *mut libc::c_void);
}
unsafe extern "C" fn parallel(
    mut a: *mut IOSCUT,
    mut b: *mut IOSCUT,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut s: libc::c_double = 0.0f64;
    let mut sa: libc::c_double = 0.0f64;
    let mut sb: libc::c_double = 0.0f64;
    let mut temp: libc::c_double = 0.;
    aij = (*a).ptr;
    while !aij.is_null() {
        *work.offset((*(*aij).col).j as isize) = (*aij).val;
        sa += (*aij).val * (*aij).val;
        aij = (*aij).r_next;
    }
    aij = (*b).ptr;
    while !aij.is_null() {
        s += *work.offset((*(*aij).col).j as isize) * (*aij).val;
        sb += (*aij).val * (*aij).val;
        aij = (*aij).r_next;
    }
    aij = (*a).ptr;
    while !aij.is_null() {
        *work.offset((*(*aij).col).j as isize) = 0.0f64;
        aij = (*aij).r_next;
    }
    temp = sqrt(sa) * sqrt(sb);
    if temp < 2.2204460492503131e-16f64 * 2.2204460492503131e-16f64 {
        temp = 2.2204460492503131e-16f64;
    }
    return s / temp;
}
