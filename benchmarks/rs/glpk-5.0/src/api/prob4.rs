use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_get_num_rows(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_num_cols(P: *mut glp_prob) -> libc::c_int;
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
pub unsafe extern "C" fn glp_set_rii(
    mut lp: *mut glp_prob,
    mut i: libc::c_int,
    mut rii: libc::c_double,
) {
    if !(1 as libc::c_int <= i && i <= (*lp).m) {
        (glp_error_(
            b"api/prob4.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_rii: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    if rii <= 0.0f64 {
        (glp_error_(
            b"api/prob4.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_rii: i = %d; rii = %g; invalid scale factor\n\0" as *const u8
                as *const libc::c_char,
            i,
            rii,
        );
    }
    if (*lp).valid != 0 && (**((*lp).row).offset(i as isize)).rii != rii {
        let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
        aij = (**((*lp).row).offset(i as isize)).ptr;
        while !aij.is_null() {
            if (*(*aij).col).stat == 1 as libc::c_int {
                (*lp).valid = 0 as libc::c_int;
                break;
            } else {
                aij = (*aij).r_next;
            }
        }
    }
    (**((*lp).row).offset(i as isize)).rii = rii;
}
pub unsafe extern "C" fn glp_set_sjj(
    mut lp: *mut glp_prob,
    mut j: libc::c_int,
    mut sjj: libc::c_double,
) {
    if !(1 as libc::c_int <= j && j <= (*lp).n) {
        (glp_error_(
            b"api/prob4.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_sjj: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    if sjj <= 0.0f64 {
        (glp_error_(
            b"api/prob4.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_sjj: j = %d; sjj = %g; invalid scale factor\n\0" as *const u8
                as *const libc::c_char,
            j,
            sjj,
        );
    }
    if (*lp).valid != 0 && (**((*lp).col).offset(j as isize)).sjj != sjj
        && (**((*lp).col).offset(j as isize)).stat == 1 as libc::c_int
    {
        (*lp).valid = 0 as libc::c_int;
    }
    (**((*lp).col).offset(j as isize)).sjj = sjj;
}
pub unsafe extern "C" fn glp_get_rii(
    mut lp: *mut glp_prob,
    mut i: libc::c_int,
) -> libc::c_double {
    if !(1 as libc::c_int <= i && i <= (*lp).m) {
        (glp_error_(
            b"api/prob4.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_rii: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    return (**((*lp).row).offset(i as isize)).rii;
}
pub unsafe extern "C" fn glp_get_sjj(
    mut lp: *mut glp_prob,
    mut j: libc::c_int,
) -> libc::c_double {
    if !(1 as libc::c_int <= j && j <= (*lp).n) {
        (glp_error_(
            b"api/prob4.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_sjj: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    return (**((*lp).col).offset(j as isize)).sjj;
}
pub unsafe extern "C" fn glp_unscale_prob(mut lp: *mut glp_prob) {
    let mut m: libc::c_int = glp_get_num_rows(lp);
    let mut n: libc::c_int = glp_get_num_cols(lp);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= m {
        glp_set_rii(lp, i, 1.0f64);
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= n {
        glp_set_sjj(lp, j, 1.0f64);
        j += 1;
        j;
    }
}
