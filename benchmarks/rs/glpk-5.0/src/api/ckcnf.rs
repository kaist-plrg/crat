use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
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
pub unsafe extern "C" fn glp_check_cnfsat(mut P: *mut glp_prob) -> libc::c_int {
    let mut m: libc::c_int = (*P).m;
    let mut n: libc::c_int = (*P).n;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut neg: libc::c_int = 0;
    j = 1 as libc::c_int;
    while j <= n {
        col = *((*P).col).offset(j as isize);
        if !((*col).kind == 2 as libc::c_int && (*col).type_0 == 4 as libc::c_int
            && (*col).lb == 0.0f64 && (*col).ub == 1.0f64)
        {
            return 1 as libc::c_int;
        }
        j += 1;
        j;
    }
    if (*P).c0 != 0.0f64 {
        return 2 as libc::c_int;
    }
    j = 1 as libc::c_int;
    while j <= n {
        col = *((*P).col).offset(j as isize);
        if (*col).coef != 0.0f64 {
            return 3 as libc::c_int;
        }
        j += 1;
        j;
    }
    i = 1 as libc::c_int;
    while i <= m {
        row = *((*P).row).offset(i as isize);
        if (*row).type_0 != 2 as libc::c_int {
            return 4 as libc::c_int;
        }
        neg = 0 as libc::c_int;
        aij = (*row).ptr;
        while !aij.is_null() {
            if !((*aij).val == 1.0f64) {
                if (*aij).val == -1.0f64 {
                    neg += 1;
                    neg;
                } else {
                    return 5 as libc::c_int
                }
            }
            aij = (*aij).r_next;
        }
        if (*row).lb != (1 as libc::c_int - neg) as libc::c_double {
            return 6 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
