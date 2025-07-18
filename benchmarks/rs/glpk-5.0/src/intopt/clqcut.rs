use ::libc;
extern "C" {
    pub type DMP;
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type glp_tree;
    fn _glp_cfg_find_clique(
        P: *mut libc::c_void,
        G: *mut CFG,
        ind: *mut libc::c_int,
        sum: *mut libc::c_double,
    ) -> libc::c_int;
    fn _glp_cfg_expand_clique(
        G: *mut CFG,
        c_len: libc::c_int,
        c_ind: *mut libc::c_int,
    ) -> libc::c_int;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_cfg {
    pub n: libc::c_int,
    pub pos: *mut libc::c_int,
    pub neg: *mut libc::c_int,
    pub pool: *mut DMP,
    pub nv_max: libc::c_int,
    pub nv: libc::c_int,
    pub ref_0: *mut libc::c_int,
    pub vptr: *mut *mut CFGVLE,
    pub cptr: *mut *mut CFGCLE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CFGCLE {
    pub vptr: *mut CFGVLE,
    pub next: *mut CFGCLE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CFGVLE {
    pub v: libc::c_int,
    pub next: *mut CFGVLE,
}
pub type CFG = glp_cfg;
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
pub unsafe extern "C" fn glp_clq_cut(
    mut P: *mut glp_prob,
    mut G: *mut glp_cfg,
    mut ind: *mut libc::c_int,
    mut val: *mut libc::c_double,
) -> libc::c_int {
    let mut n: libc::c_int = (*P).n;
    let mut pos: *mut libc::c_int = (*G).pos;
    let mut neg: *mut libc::c_int = (*G).neg;
    let mut nv: libc::c_int = (*G).nv;
    let mut ref_0: *mut libc::c_int = (*G).ref_0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut rhs: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    ((*G).n == n
        || {
            glp_assert_(
                b"G->n == n\0" as *const u8 as *const libc::c_char,
                b"intopt/clqcut.c\0" as *const u8 as *const libc::c_char,
                65 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    len = _glp_cfg_find_clique(P as *mut libc::c_void, G, ind, &mut sum);
    if sum < 1.07f64 {
        return 0 as libc::c_int;
    }
    len = _glp_cfg_expand_clique(G, len, ind);
    rhs = 1.0f64;
    j = 1 as libc::c_int;
    while j <= n {
        *val.offset(j as isize) = 0.0f64;
        j += 1;
        j;
    }
    k = 1 as libc::c_int;
    while k <= len {
        v = *ind.offset(k as isize);
        (1 as libc::c_int <= v && v <= nv
            || {
                glp_assert_(
                    b"1 <= v && v <= nv\0" as *const u8 as *const libc::c_char,
                    b"intopt/clqcut.c\0" as *const u8 as *const libc::c_char,
                    89 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        j = *ref_0.offset(v as isize);
        (1 as libc::c_int <= j && j <= n
            || {
                glp_assert_(
                    b"1 <= j && j <= n\0" as *const u8 as *const libc::c_char,
                    b"intopt/clqcut.c\0" as *const u8 as *const libc::c_char,
                    92 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if *pos.offset(j as isize) == v {
            if (**((*P).col).offset(j as isize)).type_0 == 5 as libc::c_int {
                rhs -= (**((*P).col).offset(j as isize)).prim;
            } else {
                *val.offset(j as isize) += 1.0f64;
            }
        } else if *neg.offset(j as isize) == v {
            if (**((*P).col).offset(j as isize)).type_0 == 5 as libc::c_int {
                rhs -= 1.0f64 - (**((*P).col).offset(j as isize)).prim;
            } else {
                *val.offset(j as isize) -= 1.0f64;
                rhs -= 1.0f64;
            }
        } else {
            (v != v
                || {
                    glp_assert_(
                        b"v != v\0" as *const u8 as *const libc::c_char,
                        b"intopt/clqcut.c\0" as *const u8 as *const libc::c_char,
                        117 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        k += 1;
        k;
    }
    len = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while j <= n {
        if *val.offset(j as isize) != 0.0f64 {
            len += 1;
            len;
            *ind.offset(len as isize) = j;
            *val.offset(len as isize) = *val.offset(j as isize);
        }
        j += 1;
        j;
    }
    *ind.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    *val.offset(0 as libc::c_int as isize) = rhs;
    return len;
}
