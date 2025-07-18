use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
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
pub unsafe extern "C" fn glp_set_row_stat(
    mut lp: *mut glp_prob,
    mut i: libc::c_int,
    mut stat: libc::c_int,
) {
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    if !(1 as libc::c_int <= i && i <= (*lp).m) {
        (glp_error_(
            b"api/prob5.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_row_stat: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    if !(stat == 1 as libc::c_int || stat == 2 as libc::c_int || stat == 3 as libc::c_int
        || stat == 4 as libc::c_int || stat == 5 as libc::c_int)
    {
        (glp_error_(
            b"api/prob5.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_row_stat: i = %d; stat = %d; invalid status\n\0" as *const u8
                as *const libc::c_char,
            i,
            stat,
        );
    }
    row = *((*lp).row).offset(i as isize);
    if stat != 1 as libc::c_int {
        match (*row).type_0 {
            1 => {
                stat = 4 as libc::c_int;
            }
            2 => {
                stat = 2 as libc::c_int;
            }
            3 => {
                stat = 3 as libc::c_int;
            }
            4 => {
                if stat != 3 as libc::c_int {
                    stat = 2 as libc::c_int;
                }
            }
            5 => {
                stat = 5 as libc::c_int;
            }
            _ => {
                (row != row
                    || {
                        glp_assert_(
                            b"row != row\0" as *const u8 as *const libc::c_char,
                            b"api/prob5.c\0" as *const u8 as *const libc::c_char,
                            67 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
    }
    if (*row).stat == 1 as libc::c_int && stat != 1 as libc::c_int
        || (*row).stat != 1 as libc::c_int && stat == 1 as libc::c_int
    {
        (*lp).valid = 0 as libc::c_int;
    }
    (*row).stat = stat;
}
pub unsafe extern "C" fn glp_set_col_stat(
    mut lp: *mut glp_prob,
    mut j: libc::c_int,
    mut stat: libc::c_int,
) {
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    if !(1 as libc::c_int <= j && j <= (*lp).n) {
        (glp_error_(
            b"api/prob5.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_col_stat: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    if !(stat == 1 as libc::c_int || stat == 2 as libc::c_int || stat == 3 as libc::c_int
        || stat == 4 as libc::c_int || stat == 5 as libc::c_int)
    {
        (glp_error_(
            b"api/prob5.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_col_stat: j = %d; stat = %d; invalid status\n\0" as *const u8
                as *const libc::c_char,
            j,
            stat,
        );
    }
    col = *((*lp).col).offset(j as isize);
    if stat != 1 as libc::c_int {
        match (*col).type_0 {
            1 => {
                stat = 4 as libc::c_int;
            }
            2 => {
                stat = 2 as libc::c_int;
            }
            3 => {
                stat = 3 as libc::c_int;
            }
            4 => {
                if stat != 3 as libc::c_int {
                    stat = 2 as libc::c_int;
                }
            }
            5 => {
                stat = 5 as libc::c_int;
            }
            _ => {
                (col != col
                    || {
                        glp_assert_(
                            b"col != col\0" as *const u8 as *const libc::c_char,
                            b"api/prob5.c\0" as *const u8 as *const libc::c_char,
                            121 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
    }
    if (*col).stat == 1 as libc::c_int && stat != 1 as libc::c_int
        || (*col).stat != 1 as libc::c_int && stat == 1 as libc::c_int
    {
        (*lp).valid = 0 as libc::c_int;
    }
    (*col).stat = stat;
}
pub unsafe extern "C" fn glp_std_basis(mut lp: *mut glp_prob) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= (*lp).m {
        glp_set_row_stat(lp, i, 1 as libc::c_int);
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= (*lp).n {
        let mut col: *mut GLPCOL = *((*lp).col).offset(j as isize);
        if (*col).type_0 == 4 as libc::c_int && fabs((*col).lb) > fabs((*col).ub) {
            glp_set_col_stat(lp, j, 3 as libc::c_int);
        } else {
            glp_set_col_stat(lp, j, 2 as libc::c_int);
        }
        j += 1;
        j;
    }
}
