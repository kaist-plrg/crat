use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
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
pub unsafe extern "C" fn glp_get_prob_name(
    mut lp: *mut glp_prob,
) -> *const libc::c_char {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    name = (*lp).name;
    return name;
}
pub unsafe extern "C" fn glp_get_obj_name(mut lp: *mut glp_prob) -> *const libc::c_char {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    name = (*lp).obj;
    return name;
}
pub unsafe extern "C" fn glp_get_obj_dir(mut lp: *mut glp_prob) -> libc::c_int {
    let mut dir: libc::c_int = (*lp).dir;
    return dir;
}
pub unsafe extern "C" fn glp_get_num_rows(mut lp: *mut glp_prob) -> libc::c_int {
    let mut m: libc::c_int = (*lp).m;
    return m;
}
pub unsafe extern "C" fn glp_get_num_cols(mut lp: *mut glp_prob) -> libc::c_int {
    let mut n: libc::c_int = (*lp).n;
    return n;
}
pub unsafe extern "C" fn glp_get_row_name(
    mut lp: *mut glp_prob,
    mut i: libc::c_int,
) -> *const libc::c_char {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(1 as libc::c_int <= i && i <= (*lp).m) {
        (glp_error_(
            b"api/prob2.c\0" as *const u8 as *const libc::c_char,
            146 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_row_name: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    name = (**((*lp).row).offset(i as isize)).name;
    return name;
}
pub unsafe extern "C" fn glp_get_col_name(
    mut lp: *mut glp_prob,
    mut j: libc::c_int,
) -> *const libc::c_char {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(1 as libc::c_int <= j && j <= (*lp).n) {
        (glp_error_(
            b"api/prob2.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_col_name: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    name = (**((*lp).col).offset(j as isize)).name;
    return name;
}
pub unsafe extern "C" fn glp_get_row_type(
    mut lp: *mut glp_prob,
    mut i: libc::c_int,
) -> libc::c_int {
    if !(1 as libc::c_int <= i && i <= (*lp).m) {
        (glp_error_(
            b"api/prob2.c\0" as *const u8 as *const libc::c_char,
            198 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_row_type: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    return (**((*lp).row).offset(i as isize)).type_0;
}
pub unsafe extern "C" fn glp_get_row_lb(
    mut lp: *mut glp_prob,
    mut i: libc::c_int,
) -> libc::c_double {
    let mut lb: libc::c_double = 0.;
    if !(1 as libc::c_int <= i && i <= (*lp).m) {
        (glp_error_(
            b"api/prob2.c\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_row_lb: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    match (**((*lp).row).offset(i as isize)).type_0 {
        1 | 3 => {
            lb = -1.7976931348623157e+308f64;
        }
        2 | 4 | 5 => {
            lb = (**((*lp).row).offset(i as isize)).lb;
        }
        _ => {
            (lp != lp
                || {
                    glp_assert_(
                        b"lp != lp\0" as *const u8 as *const libc::c_char,
                        b"api/prob2.c\0" as *const u8 as *const libc::c_char,
                        231 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    return lb;
}
pub unsafe extern "C" fn glp_get_row_ub(
    mut lp: *mut glp_prob,
    mut i: libc::c_int,
) -> libc::c_double {
    let mut ub: libc::c_double = 0.;
    if !(1 as libc::c_int <= i && i <= (*lp).m) {
        (glp_error_(
            b"api/prob2.c\0" as *const u8 as *const libc::c_char,
            254 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_row_ub: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    match (**((*lp).row).offset(i as isize)).type_0 {
        1 | 2 => {
            ub = 1.7976931348623157e+308f64;
        }
        3 | 4 | 5 => {
            ub = (**((*lp).row).offset(i as isize)).ub;
        }
        _ => {
            (lp != lp
                || {
                    glp_assert_(
                        b"lp != lp\0" as *const u8 as *const libc::c_char,
                        b"api/prob2.c\0" as *const u8 as *const libc::c_char,
                        264 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    return ub;
}
pub unsafe extern "C" fn glp_get_col_type(
    mut lp: *mut glp_prob,
    mut j: libc::c_int,
) -> libc::c_int {
    if !(1 as libc::c_int <= j && j <= (*lp).n) {
        (glp_error_(
            b"api/prob2.c\0" as *const u8 as *const libc::c_char,
            291 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_col_type: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    return (**((*lp).col).offset(j as isize)).type_0;
}
pub unsafe extern "C" fn glp_get_col_lb(
    mut lp: *mut glp_prob,
    mut j: libc::c_int,
) -> libc::c_double {
    let mut lb: libc::c_double = 0.;
    if !(1 as libc::c_int <= j && j <= (*lp).n) {
        (glp_error_(
            b"api/prob2.c\0" as *const u8 as *const libc::c_char,
            314 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_col_lb: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    match (**((*lp).col).offset(j as isize)).type_0 {
        1 | 3 => {
            lb = -1.7976931348623157e+308f64;
        }
        2 | 4 | 5 => {
            lb = (**((*lp).col).offset(j as isize)).lb;
        }
        _ => {
            (lp != lp
                || {
                    glp_assert_(
                        b"lp != lp\0" as *const u8 as *const libc::c_char,
                        b"api/prob2.c\0" as *const u8 as *const libc::c_char,
                        325 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    return lb;
}
pub unsafe extern "C" fn glp_get_col_ub(
    mut lp: *mut glp_prob,
    mut j: libc::c_int,
) -> libc::c_double {
    let mut ub: libc::c_double = 0.;
    if !(1 as libc::c_int <= j && j <= (*lp).n) {
        (glp_error_(
            b"api/prob2.c\0" as *const u8 as *const libc::c_char,
            348 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_col_ub: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    match (**((*lp).col).offset(j as isize)).type_0 {
        1 | 2 => {
            ub = 1.7976931348623157e+308f64;
        }
        3 | 4 | 5 => {
            ub = (**((*lp).col).offset(j as isize)).ub;
        }
        _ => {
            (lp != lp
                || {
                    glp_assert_(
                        b"lp != lp\0" as *const u8 as *const libc::c_char,
                        b"api/prob2.c\0" as *const u8 as *const libc::c_char,
                        359 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    return ub;
}
pub unsafe extern "C" fn glp_get_obj_coef(
    mut lp: *mut glp_prob,
    mut j: libc::c_int,
) -> libc::c_double {
    if !(0 as libc::c_int <= j && j <= (*lp).n) {
        (glp_error_(
            b"api/prob2.c\0" as *const u8 as *const libc::c_char,
            383 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_obj_coef: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    return if j == 0 as libc::c_int {
        (*lp).c0
    } else {
        (**((*lp).col).offset(j as isize)).coef
    };
}
pub unsafe extern "C" fn glp_get_num_nz(mut lp: *mut glp_prob) -> libc::c_int {
    let mut nnz: libc::c_int = (*lp).nnz;
    return nnz;
}
pub unsafe extern "C" fn glp_get_mat_row(
    mut lp: *mut glp_prob,
    mut i: libc::c_int,
    mut ind: *mut libc::c_int,
    mut val: *mut libc::c_double,
) -> libc::c_int {
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut len: libc::c_int = 0;
    if !(1 as libc::c_int <= i && i <= (*lp).m) {
        (glp_error_(
            b"api/prob2.c\0" as *const u8 as *const libc::c_char,
            436 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_mat_row: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    len = 0 as libc::c_int;
    aij = (**((*lp).row).offset(i as isize)).ptr;
    while !aij.is_null() {
        len += 1;
        len;
        if !ind.is_null() {
            *ind.offset(len as isize) = (*(*aij).col).j;
        }
        if !val.is_null() {
            *val.offset(len as isize) = (*aij).val;
        }
        aij = (*aij).r_next;
    }
    (len <= (*lp).n
        || {
            glp_assert_(
                b"len <= lp->n\0" as *const u8 as *const libc::c_char,
                b"api/prob2.c\0" as *const u8 as *const libc::c_char,
                444 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return len;
}
pub unsafe extern "C" fn glp_get_mat_col(
    mut lp: *mut glp_prob,
    mut j: libc::c_int,
    mut ind: *mut libc::c_int,
    mut val: *mut libc::c_double,
) -> libc::c_int {
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut len: libc::c_int = 0;
    if !(1 as libc::c_int <= j && j <= (*lp).n) {
        (glp_error_(
            b"api/prob2.c\0" as *const u8 as *const libc::c_char,
            477 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_mat_col: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    len = 0 as libc::c_int;
    aij = (**((*lp).col).offset(j as isize)).ptr;
    while !aij.is_null() {
        len += 1;
        len;
        if !ind.is_null() {
            *ind.offset(len as isize) = (*(*aij).row).i;
        }
        if !val.is_null() {
            *val.offset(len as isize) = (*aij).val;
        }
        aij = (*aij).c_next;
    }
    (len <= (*lp).m
        || {
            glp_assert_(
                b"len <= lp->m\0" as *const u8 as *const libc::c_char,
                b"api/prob2.c\0" as *const u8 as *const libc::c_char,
                485 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return len;
}
