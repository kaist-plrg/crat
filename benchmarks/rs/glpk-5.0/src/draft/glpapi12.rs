use ::libc;
extern "C" {
    pub type BFD;
    pub type AVL;
    pub type AVLNODE;
    pub type glp_tree;
    pub type DMP;
    fn glp_get_obj_dir(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_num_rows(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_num_cols(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_row_type(P: *mut glp_prob, i: libc::c_int) -> libc::c_int;
    fn glp_get_row_lb(P: *mut glp_prob, i: libc::c_int) -> libc::c_double;
    fn glp_get_row_ub(P: *mut glp_prob, i: libc::c_int) -> libc::c_double;
    fn glp_get_col_type(P: *mut glp_prob, j: libc::c_int) -> libc::c_int;
    fn glp_get_col_lb(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
    fn glp_get_col_ub(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
    fn glp_get_mat_col(
        P: *mut glp_prob,
        j: libc::c_int,
        ind: *mut libc::c_int,
        val: *mut libc::c_double,
    ) -> libc::c_int;
    fn glp_get_prim_stat(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_dual_stat(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_row_stat(P: *mut glp_prob, i: libc::c_int) -> libc::c_int;
    fn glp_get_row_prim(P: *mut glp_prob, i: libc::c_int) -> libc::c_double;
    fn glp_get_row_dual(P: *mut glp_prob, i: libc::c_int) -> libc::c_double;
    fn glp_get_col_stat(P: *mut glp_prob, j: libc::c_int) -> libc::c_int;
    fn glp_get_col_prim(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
    fn glp_get_col_dual(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn _glp_bfd_ftran(bfd: *mut BFD, x: *mut libc::c_double);
    fn _glp_bfd_get_bfcp(bfd: *mut BFD, parm: *mut libc::c_void);
    fn _glp_bfd_set_bfcp(bfd: *mut BFD, parm: *const libc::c_void);
    fn _glp_bfd_create_it() -> *mut BFD;
    fn _glp_bfd_factorize(
        bfd: *mut BFD,
        m: libc::c_int,
        col: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                libc::c_int,
                *mut libc::c_int,
                *mut libc::c_double,
            ) -> libc::c_int,
        >,
        info: *mut libc::c_void,
    ) -> libc::c_int;
    fn _glp_bfd_btran(bfd: *mut BFD, x: *mut libc::c_double);
    fn _glp_bfd_get_count(bfd: *mut BFD) -> libc::c_int;
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
pub struct glp_bfcp {
    pub msg_lev: libc::c_int,
    pub type_0: libc::c_int,
    pub lu_size: libc::c_int,
    pub piv_tol: libc::c_double,
    pub piv_lim: libc::c_int,
    pub suhl: libc::c_int,
    pub eps_tol: libc::c_double,
    pub max_gro: libc::c_double,
    pub nfs_max: libc::c_int,
    pub upd_tol: libc::c_double,
    pub nrs_max: libc::c_int,
    pub rs_size: libc::c_int,
    pub foo_bar: [libc::c_double; 38],
}
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
pub unsafe extern "C" fn glp_bf_exists(mut lp: *mut glp_prob) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = ((*lp).m == 0 as libc::c_int || (*lp).valid != 0) as libc::c_int;
    return ret;
}
unsafe extern "C" fn b_col(
    mut info: *mut libc::c_void,
    mut j: libc::c_int,
    mut ind: *mut libc::c_int,
    mut val: *mut libc::c_double,
) -> libc::c_int {
    let mut lp: *mut glp_prob = info as *mut glp_prob;
    let mut m: libc::c_int = (*lp).m;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut k: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    (1 as libc::c_int <= j && j <= m
        || {
            glp_assert_(
                b"1 <= j && j <= m\0" as *const u8 as *const libc::c_char,
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                82 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = *((*lp).head).offset(j as isize);
    if k <= m {
        len = 1 as libc::c_int;
        *ind.offset(1 as libc::c_int as isize) = k;
        *val.offset(1 as libc::c_int as isize) = 1.0f64;
    } else {
        len = 0 as libc::c_int;
        aij = (**((*lp).col).offset((k - m) as isize)).ptr;
        while !aij.is_null() {
            len += 1;
            len;
            *ind.offset(len as isize) = (*(*aij).row).i;
            *val
                .offset(
                    len as isize,
                ) = -(*(*aij).row).rii * (*aij).val * (*(*aij).col).sjj;
            aij = (*aij).c_next;
        }
    }
    return len;
}
pub unsafe extern "C" fn glp_factorize(mut lp: *mut glp_prob) -> libc::c_int {
    let mut current_block: u64;
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut row: *mut *mut GLPROW = (*lp).row;
    let mut col: *mut *mut GLPCOL = (*lp).col;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut stat: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    (*lp).valid = 0 as libc::c_int;
    j = 0 as libc::c_int;
    k = 1 as libc::c_int;
    loop {
        if !(k <= m + n) {
            current_block = 5689001924483802034;
            break;
        }
        if k <= m {
            stat = (**row.offset(k as isize)).stat;
            (**row.offset(k as isize)).bind = 0 as libc::c_int;
        } else {
            stat = (**col.offset((k - m) as isize)).stat;
            (**col.offset((k - m) as isize)).bind = 0 as libc::c_int;
        }
        if stat == 1 as libc::c_int {
            j += 1;
            j;
            if j > m {
                ret = 0x1 as libc::c_int;
                current_block = 7350059504414396885;
                break;
            } else {
                *head.offset(j as isize) = k;
                if k <= m {
                    (**row.offset(k as isize)).bind = j;
                } else {
                    (**col.offset((k - m) as isize)).bind = j;
                }
            }
        }
        k += 1;
        k;
    }
    match current_block {
        5689001924483802034 => {
            if j < m {
                ret = 0x1 as libc::c_int;
            } else {
                if m > 0 as libc::c_int {
                    if ((*lp).bfd).is_null() {
                        (*lp).bfd = _glp_bfd_create_it();
                    }
                    match _glp_bfd_factorize(
                        (*lp).bfd,
                        m,
                        Some(
                            b_col
                                as unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    libc::c_int,
                                    *mut libc::c_int,
                                    *mut libc::c_double,
                                ) -> libc::c_int,
                        ),
                        lp as *mut libc::c_void,
                    ) {
                        0 => {
                            current_block = 16924917904204750491;
                        }
                        1 => {
                            ret = 0x2 as libc::c_int;
                            current_block = 7350059504414396885;
                        }
                        2 => {
                            ret = 0x3 as libc::c_int;
                            current_block = 7350059504414396885;
                        }
                        _ => {
                            (lp != lp
                                || {
                                    glp_assert_(
                                        b"lp != lp\0" as *const u8 as *const libc::c_char,
                                        b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                                        166 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                            current_block = 16924917904204750491;
                        }
                    }
                    match current_block {
                        7350059504414396885 => {}
                        _ => {
                            (*lp).valid = 1 as libc::c_int;
                            current_block = 17788412896529399552;
                        }
                    }
                } else {
                    current_block = 17788412896529399552;
                }
                match current_block {
                    7350059504414396885 => {}
                    _ => {
                        ret = 0 as libc::c_int;
                    }
                }
            }
        }
        _ => {}
    }
    return ret;
}
pub unsafe extern "C" fn glp_bf_updated(mut lp: *mut glp_prob) -> libc::c_int {
    let mut cnt: libc::c_int = 0;
    if !((*lp).m == 0 as libc::c_int || (*lp).valid != 0) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int,
        ))
            .unwrap()(
            b"glp_bf_update: basis factorization does not exist\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    cnt = if (*lp).m == 0 as libc::c_int {
        0 as libc::c_int
    } else {
        _glp_bfd_get_count((*lp).bfd)
    };
    return cnt;
}
pub unsafe extern "C" fn glp_get_bfcp(mut P: *mut glp_prob, mut parm: *mut glp_bfcp) {
    if ((*P).bfd).is_null() {
        (*P).bfd = _glp_bfd_create_it();
    }
    _glp_bfd_get_bfcp((*P).bfd, parm as *mut libc::c_void);
}
pub unsafe extern "C" fn glp_set_bfcp(mut P: *mut glp_prob, mut parm: *const glp_bfcp) {
    if ((*P).bfd).is_null() {
        (*P).bfd = _glp_bfd_create_it();
    }
    if !parm.is_null() {
        if !((*parm).type_0 == 0 as libc::c_int + 0x1 as libc::c_int
            || (*parm).type_0 == 0 as libc::c_int + 0x2 as libc::c_int
            || (*parm).type_0 == 0 as libc::c_int + 0x3 as libc::c_int
            || (*parm).type_0 == 0x10 as libc::c_int + 0x2 as libc::c_int
            || (*parm).type_0 == 0x10 as libc::c_int + 0x3 as libc::c_int)
        {
            (glp_error_(
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                261 as libc::c_int,
            ))
                .unwrap()(
                b"glp_set_bfcp: type = 0x%02X; invalid parameter\n\0" as *const u8
                    as *const libc::c_char,
                (*parm).type_0,
            );
        }
        if !(0.0f64 < (*parm).piv_tol && (*parm).piv_tol < 1.0f64) {
            (glp_error_(
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                264 as libc::c_int,
            ))
                .unwrap()(
                b"glp_set_bfcp: piv_tol = %g; invalid parameter\n\0" as *const u8
                    as *const libc::c_char,
                (*parm).piv_tol,
            );
        }
        if (*parm).piv_lim < 1 as libc::c_int {
            (glp_error_(
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                267 as libc::c_int,
            ))
                .unwrap()(
                b"glp_set_bfcp: piv_lim = %d; invalid parameter\n\0" as *const u8
                    as *const libc::c_char,
                (*parm).piv_lim,
            );
        }
        if !((*parm).suhl == 1 as libc::c_int || (*parm).suhl == 0 as libc::c_int) {
            (glp_error_(
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                270 as libc::c_int,
            ))
                .unwrap()(
                b"glp_set_bfcp: suhl = %d; invalid parameter\n\0" as *const u8
                    as *const libc::c_char,
                (*parm).suhl,
            );
        }
        if !(0.0f64 <= (*parm).eps_tol && (*parm).eps_tol <= 1e-6f64) {
            (glp_error_(
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                273 as libc::c_int,
            ))
                .unwrap()(
                b"glp_set_bfcp: eps_tol = %g; invalid parameter\n\0" as *const u8
                    as *const libc::c_char,
                (*parm).eps_tol,
            );
        }
        if !(1 as libc::c_int <= (*parm).nfs_max
            && (*parm).nfs_max <= 32767 as libc::c_int)
        {
            (glp_error_(
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                276 as libc::c_int,
            ))
                .unwrap()(
                b"glp_set_bfcp: nfs_max = %d; invalid parameter\n\0" as *const u8
                    as *const libc::c_char,
                (*parm).nfs_max,
            );
        }
        if !(1 as libc::c_int <= (*parm).nrs_max
            && (*parm).nrs_max <= 32767 as libc::c_int)
        {
            (glp_error_(
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                279 as libc::c_int,
            ))
                .unwrap()(
                b"glp_set_bfcp: nrs_max = %d; invalid parameter\n\0" as *const u8
                    as *const libc::c_char,
                (*parm).nrs_max,
            );
        }
    }
    _glp_bfd_set_bfcp((*P).bfd, parm as *const libc::c_void);
}
pub unsafe extern "C" fn glp_get_bhead(
    mut lp: *mut glp_prob,
    mut k: libc::c_int,
) -> libc::c_int {
    if !((*lp).m == 0 as libc::c_int || (*lp).valid != 0) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            310 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_bhead: basis factorization does not exist\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(1 as libc::c_int <= k && k <= (*lp).m) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_bhead: k = %d; index out of range\n\0" as *const u8
                as *const libc::c_char,
            k,
        );
    }
    return *((*lp).head).offset(k as isize);
}
pub unsafe extern "C" fn glp_get_row_bind(
    mut lp: *mut glp_prob,
    mut i: libc::c_int,
) -> libc::c_int {
    if !((*lp).m == 0 as libc::c_int || (*lp).valid != 0) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            335 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_row_bind: basis factorization does not exist\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(1 as libc::c_int <= i && i <= (*lp).m) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            338 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_row_bind: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    return (**((*lp).row).offset(i as isize)).bind;
}
pub unsafe extern "C" fn glp_get_col_bind(
    mut lp: *mut glp_prob,
    mut j: libc::c_int,
) -> libc::c_int {
    if !((*lp).m == 0 as libc::c_int || (*lp).valid != 0) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            362 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_col_bind: basis factorization does not exist\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(1 as libc::c_int <= j && j <= (*lp).n) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            365 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_col_bind: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    return (**((*lp).col).offset(j as isize)).bind;
}
pub unsafe extern "C" fn glp_ftran(mut lp: *mut glp_prob, mut x: *mut libc::c_double) {
    let mut m: libc::c_int = (*lp).m;
    let mut row: *mut *mut GLPROW = (*lp).row;
    let mut col: *mut *mut GLPCOL = (*lp).col;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if !(m == 0 as libc::c_int || (*lp).valid != 0) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            432 as libc::c_int,
        ))
            .unwrap()(
            b"glp_ftran: basis factorization does not exist\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    i = 1 as libc::c_int;
    while i <= m {
        *x.offset(i as isize) *= (**row.offset(i as isize)).rii;
        i += 1;
        i;
    }
    if m > 0 as libc::c_int {
        _glp_bfd_ftran((*lp).bfd, x);
    }
    i = 1 as libc::c_int;
    while i <= m {
        k = *((*lp).head).offset(i as isize);
        if k <= m {
            *x.offset(i as isize) /= (**row.offset(k as isize)).rii;
        } else {
            *x.offset(i as isize) *= (**col.offset((k - m) as isize)).sjj;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn glp_btran(mut lp: *mut glp_prob, mut x: *mut libc::c_double) {
    let mut m: libc::c_int = (*lp).m;
    let mut row: *mut *mut GLPROW = (*lp).row;
    let mut col: *mut *mut GLPCOL = (*lp).col;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if !(m == 0 as libc::c_int || (*lp).valid != 0) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            482 as libc::c_int,
        ))
            .unwrap()(
            b"glp_btran: basis factorization does not exist\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    i = 1 as libc::c_int;
    while i <= m {
        k = *((*lp).head).offset(i as isize);
        if k <= m {
            *x.offset(i as isize) /= (**row.offset(k as isize)).rii;
        } else {
            *x.offset(i as isize) *= (**col.offset((k - m) as isize)).sjj;
        }
        i += 1;
        i;
    }
    if m > 0 as libc::c_int {
        _glp_bfd_btran((*lp).bfd, x);
    }
    i = 1 as libc::c_int;
    while i <= m {
        *x.offset(i as isize) *= (**row.offset(i as isize)).rii;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn glp_warm_up(mut P: *mut glp_prob) -> libc::c_int {
    let mut current_block: u64;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut stat: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut eps: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut work: *mut libc::c_double = 0 as *mut libc::c_double;
    (*P).dbs_stat = 1 as libc::c_int;
    (*P).pbs_stat = (*P).dbs_stat;
    (*P).obj_val = 0.0f64;
    (*P).some = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= (*P).m {
        row = *((*P).row).offset(i as isize);
        (*row).dual = 0.0f64;
        (*row).prim = (*row).dual;
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= (*P).n {
        col = *((*P).col).offset(j as isize);
        (*col).dual = 0.0f64;
        (*col).prim = (*col).dual;
        j += 1;
        j;
    }
    if glp_bf_exists(P) == 0 {
        ret = glp_factorize(P);
        if ret != 0 as libc::c_int {
            current_block = 15695542331815434033;
        } else {
            current_block = 17860125682698302841;
        }
    } else {
        current_block = 17860125682698302841;
    }
    match current_block {
        17860125682698302841 => {
            work = glp_alloc(
                1 as libc::c_int + (*P).m,
                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
            ) as *mut libc::c_double;
            i = 1 as libc::c_int;
            while i <= (*P).m {
                *work.offset(i as isize) = 0.0f64;
                i += 1;
                i;
            }
            i = 1 as libc::c_int;
            while i <= (*P).m {
                row = *((*P).row).offset(i as isize);
                if !((*row).stat == 1 as libc::c_int) {
                    if (*row).stat == 2 as libc::c_int {
                        (*row).prim = (*row).lb;
                    } else if (*row).stat == 3 as libc::c_int {
                        (*row).prim = (*row).ub;
                    } else if (*row).stat == 4 as libc::c_int {
                        (*row).prim = 0.0f64;
                    } else if (*row).stat == 5 as libc::c_int {
                        (*row).prim = (*row).lb;
                    } else {
                        (row != row
                            || {
                                glp_assert_(
                                    b"row != row\0" as *const u8 as *const libc::c_char,
                                    b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                                    575 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                    }
                    *work.offset(i as isize) -= (*row).prim;
                }
                i += 1;
                i;
            }
            j = 1 as libc::c_int;
            while j <= (*P).n {
                col = *((*P).col).offset(j as isize);
                if !((*col).stat == 1 as libc::c_int) {
                    if (*col).stat == 2 as libc::c_int {
                        (*col).prim = (*col).lb;
                    } else if (*col).stat == 3 as libc::c_int {
                        (*col).prim = (*col).ub;
                    } else if (*col).stat == 4 as libc::c_int {
                        (*col).prim = 0.0f64;
                    } else if (*col).stat == 5 as libc::c_int {
                        (*col).prim = (*col).lb;
                    } else {
                        (col != col
                            || {
                                glp_assert_(
                                    b"col != col\0" as *const u8 as *const libc::c_char,
                                    b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                                    592 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                    }
                    if (*col).prim != 0.0f64 {
                        aij = (*col).ptr;
                        while !aij.is_null() {
                            *work.offset((*(*aij).row).i as isize)
                                += (*aij).val * (*col).prim;
                            aij = (*aij).c_next;
                        }
                    }
                }
                j += 1;
                j;
            }
            glp_ftran(P, work);
            (*P).pbs_stat = 2 as libc::c_int;
            i = 1 as libc::c_int;
            while i <= (*P).m {
                row = *((*P).row).offset(i as isize);
                if !((*row).stat != 1 as libc::c_int) {
                    (*row).prim = *work.offset((*row).bind as isize);
                    type_0 = (*row).type_0;
                    if type_0 == 2 as libc::c_int || type_0 == 4 as libc::c_int
                        || type_0 == 5 as libc::c_int
                    {
                        eps = 1e-6f64 + 1e-9f64 * fabs((*row).lb);
                        if (*row).prim < (*row).lb - eps {
                            (*P).pbs_stat = 3 as libc::c_int;
                        }
                    }
                    if type_0 == 3 as libc::c_int || type_0 == 4 as libc::c_int
                        || type_0 == 5 as libc::c_int
                    {
                        eps = 1e-6f64 + 1e-9f64 * fabs((*row).ub);
                        if (*row).prim > (*row).ub + eps {
                            (*P).pbs_stat = 3 as libc::c_int;
                        }
                    }
                }
                i += 1;
                i;
            }
            j = 1 as libc::c_int;
            while j <= (*P).n {
                col = *((*P).col).offset(j as isize);
                if !((*col).stat != 1 as libc::c_int) {
                    (*col).prim = *work.offset((*col).bind as isize);
                    type_0 = (*col).type_0;
                    if type_0 == 2 as libc::c_int || type_0 == 4 as libc::c_int
                        || type_0 == 5 as libc::c_int
                    {
                        eps = 1e-6f64 + 1e-9f64 * fabs((*col).lb);
                        if (*col).prim < (*col).lb - eps {
                            (*P).pbs_stat = 3 as libc::c_int;
                        }
                    }
                    if type_0 == 3 as libc::c_int || type_0 == 4 as libc::c_int
                        || type_0 == 5 as libc::c_int
                    {
                        eps = 1e-6f64 + 1e-9f64 * fabs((*col).ub);
                        if (*col).prim > (*col).ub + eps {
                            (*P).pbs_stat = 3 as libc::c_int;
                        }
                    }
                }
                j += 1;
                j;
            }
            (*P).obj_val = (*P).c0;
            j = 1 as libc::c_int;
            while j <= (*P).n {
                col = *((*P).col).offset(j as isize);
                (*P).obj_val += (*col).coef * (*col).prim;
                j += 1;
                j;
            }
            i = 1 as libc::c_int;
            while i <= (*P).m {
                *work.offset(i as isize) = 0.0f64;
                i += 1;
                i;
            }
            j = 1 as libc::c_int;
            while j <= (*P).n {
                col = *((*P).col).offset(j as isize);
                if (*col).stat == 1 as libc::c_int {
                    *work.offset((*col).bind as isize) = (*col).coef;
                }
                j += 1;
                j;
            }
            glp_btran(P, work);
            (*P).dbs_stat = 2 as libc::c_int;
            i = 1 as libc::c_int;
            while i <= (*P).m {
                row = *((*P).row).offset(i as isize);
                if (*row).stat == 1 as libc::c_int {
                    (*row).dual = 0.0f64;
                } else {
                    (*row).dual = -*work.offset(i as isize);
                    stat = (*row).stat;
                    temp = if (*P).dir == 1 as libc::c_int {
                        (*row).dual
                    } else {
                        -(*row).dual
                    };
                    if (stat == 4 as libc::c_int || stat == 2 as libc::c_int)
                        && temp < -1e-5f64
                        || (stat == 4 as libc::c_int || stat == 3 as libc::c_int)
                            && temp > 1e-5f64
                    {
                        (*P).dbs_stat = 3 as libc::c_int;
                    }
                }
                i += 1;
                i;
            }
            j = 1 as libc::c_int;
            while j <= (*P).n {
                col = *((*P).col).offset(j as isize);
                if (*col).stat == 1 as libc::c_int {
                    (*col).dual = 0.0f64;
                } else {
                    (*col).dual = (*col).coef;
                    aij = (*col).ptr;
                    while !aij.is_null() {
                        (*col).dual
                            += (*aij).val * *work.offset((*(*aij).row).i as isize);
                        aij = (*aij).c_next;
                    }
                    stat = (*col).stat;
                    temp = if (*P).dir == 1 as libc::c_int {
                        (*col).dual
                    } else {
                        -(*col).dual
                    };
                    if (stat == 4 as libc::c_int || stat == 2 as libc::c_int)
                        && temp < -1e-5f64
                        || (stat == 4 as libc::c_int || stat == 3 as libc::c_int)
                            && temp > 1e-5f64
                    {
                        (*P).dbs_stat = 3 as libc::c_int;
                    }
                }
                j += 1;
                j;
            }
            glp_free(work as *mut libc::c_void);
            ret = 0 as libc::c_int;
        }
        _ => {}
    }
    return ret;
}
pub unsafe extern "C" fn glp_eval_tab_row(
    mut lp: *mut glp_prob,
    mut k: libc::c_int,
    mut ind: *mut libc::c_int,
    mut val: *mut libc::c_double,
) -> libc::c_int {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut i: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut lll: libc::c_int = 0;
    let mut iii: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut alfa: libc::c_double = 0.;
    let mut rho: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut vvv: *mut libc::c_double = 0 as *mut libc::c_double;
    if !(m == 0 as libc::c_int || (*lp).valid != 0) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            808 as libc::c_int,
        ))
            .unwrap()(
            b"glp_eval_tab_row: basis factorization does not exist\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(1 as libc::c_int <= k && k <= m + n) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            811 as libc::c_int,
        ))
            .unwrap()(
            b"glp_eval_tab_row: k = %d; variable number out of range\0" as *const u8
                as *const libc::c_char,
            k,
        );
    }
    if k <= m {
        i = glp_get_row_bind(lp, k);
    } else {
        i = glp_get_col_bind(lp, k - m);
    }
    if i == 0 as libc::c_int {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            819 as libc::c_int,
        ))
            .unwrap()(
            b"glp_eval_tab_row: k = %d; variable must be basic\0" as *const u8
                as *const libc::c_char,
            k,
        );
    }
    (1 as libc::c_int <= i && i <= m
        || {
            glp_assert_(
                b"1 <= i && i <= m\0" as *const u8 as *const libc::c_char,
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                820 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    rho = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    iii = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    vvv = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    t = 1 as libc::c_int;
    while t <= m {
        *rho.offset(t as isize) = 0.0f64;
        t += 1;
        t;
    }
    *rho.offset(i as isize) = 1.0f64;
    glp_btran(lp, rho);
    len = 0 as libc::c_int;
    let mut current_block_27: u64;
    k = 1 as libc::c_int;
    while k <= m + n {
        if k <= m {
            if glp_get_row_stat(lp, k) == 1 as libc::c_int {
                current_block_27 = 12039483399334584727;
            } else {
                alfa = -*rho.offset(k as isize);
                current_block_27 = 2232869372362427478;
            }
        } else if glp_get_col_stat(lp, k - m) == 1 as libc::c_int {
            current_block_27 = 12039483399334584727;
        } else {
            lll = glp_get_mat_col(lp, k - m, iii, vvv);
            alfa = 0.0f64;
            t = 1 as libc::c_int;
            while t <= lll {
                alfa
                    += *rho.offset(*iii.offset(t as isize) as isize)
                        * *vvv.offset(t as isize);
                t += 1;
                t;
            }
            current_block_27 = 2232869372362427478;
        }
        match current_block_27 {
            2232869372362427478 => {
                if alfa != 0.0f64 {
                    len += 1;
                    len;
                    *ind.offset(len as isize) = k;
                    *val.offset(len as isize) = alfa;
                }
            }
            _ => {}
        }
        k += 1;
        k;
    }
    (len <= n
        || {
            glp_assert_(
                b"len <= n\0" as *const u8 as *const libc::c_char,
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                850 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    glp_free(rho as *mut libc::c_void);
    glp_free(iii as *mut libc::c_void);
    glp_free(vvv as *mut libc::c_void);
    return len;
}
pub unsafe extern "C" fn glp_eval_tab_col(
    mut lp: *mut glp_prob,
    mut k: libc::c_int,
    mut ind: *mut libc::c_int,
    mut val: *mut libc::c_double,
) -> libc::c_int {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut t: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut stat: libc::c_int = 0;
    let mut col: *mut libc::c_double = 0 as *mut libc::c_double;
    if !(m == 0 as libc::c_int || (*lp).valid != 0) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            923 as libc::c_int,
        ))
            .unwrap()(
            b"glp_eval_tab_col: basis factorization does not exist\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(1 as libc::c_int <= k && k <= m + n) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            926 as libc::c_int,
        ))
            .unwrap()(
            b"glp_eval_tab_col: k = %d; variable number out of range\0" as *const u8
                as *const libc::c_char,
            k,
        );
    }
    if k <= m {
        stat = glp_get_row_stat(lp, k);
    } else {
        stat = glp_get_col_stat(lp, k - m);
    }
    if stat == 1 as libc::c_int {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            933 as libc::c_int,
        ))
            .unwrap()(
            b"glp_eval_tab_col: k = %d; variable must be non-basic\0" as *const u8
                as *const libc::c_char,
            k,
        );
    }
    col = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    t = 1 as libc::c_int;
    while t <= m {
        *col.offset(t as isize) = 0.0f64;
        t += 1;
        t;
    }
    if k <= m {
        *col.offset(k as isize) = -1.0f64;
    } else {
        len = glp_get_mat_col(lp, k - m, ind, val);
        t = 1 as libc::c_int;
        while t <= len {
            *col.offset(*ind.offset(t as isize) as isize) = *val.offset(t as isize);
            t += 1;
            t;
        }
    }
    glp_ftran(lp, col);
    len = 0 as libc::c_int;
    t = 1 as libc::c_int;
    while t <= m {
        if *col.offset(t as isize) != 0.0f64 {
            len += 1;
            len;
            *ind.offset(len as isize) = glp_get_bhead(lp, t);
            *val.offset(len as isize) = *col.offset(t as isize);
        }
        t += 1;
        t;
    }
    glp_free(col as *mut libc::c_void);
    return len;
}
pub unsafe extern "C" fn glp_transform_row(
    mut P: *mut glp_prob,
    mut len: libc::c_int,
    mut ind: *mut libc::c_int,
    mut val: *mut libc::c_double,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut lll: libc::c_int = 0;
    let mut iii: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut alfa: libc::c_double = 0.;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut aB: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rho: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut vvv: *mut libc::c_double = 0 as *mut libc::c_double;
    if glp_bf_exists(P) == 0 {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            1050 as libc::c_int,
        ))
            .unwrap()(
            b"glp_transform_row: basis factorization does not exist \n\0" as *const u8
                as *const libc::c_char,
        );
    }
    m = glp_get_num_rows(P);
    n = glp_get_num_cols(P);
    a = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    j = 1 as libc::c_int;
    while j <= n {
        *a.offset(j as isize) = 0.0f64;
        j += 1;
        j;
    }
    if !(0 as libc::c_int <= len && len <= n) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            1058 as libc::c_int,
        ))
            .unwrap()(
            b"glp_transform_row: len = %d; invalid row length\n\0" as *const u8
                as *const libc::c_char,
            len,
        );
    }
    t = 1 as libc::c_int;
    while t <= len {
        j = *ind.offset(t as isize);
        if !(1 as libc::c_int <= j && j <= n) {
            (glp_error_(
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                1063 as libc::c_int,
            ))
                .unwrap()(
                b"glp_transform_row: ind[%d] = %d; column index out of range\n\0"
                    as *const u8 as *const libc::c_char,
                t,
                j,
            );
        }
        if *val.offset(t as isize) == 0.0f64 {
            (glp_error_(
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                1066 as libc::c_int,
            ))
                .unwrap()(
                b"glp_transform_row: val[%d] = 0; zero coefficient not allowed\n\0"
                    as *const u8 as *const libc::c_char,
                t,
            );
        }
        if *a.offset(j as isize) != 0.0f64 {
            (glp_error_(
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                1069 as libc::c_int,
            ))
                .unwrap()(
                b"glp_transform_row: ind[%d] = %d; duplicate column indices not allowed\n\0"
                    as *const u8 as *const libc::c_char,
                t,
                j,
            );
        }
        *a.offset(j as isize) = *val.offset(t as isize);
        t += 1;
        t;
    }
    aB = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    i = 1 as libc::c_int;
    while i <= m {
        k = glp_get_bhead(P, i);
        (1 as libc::c_int <= k && k <= m + n
            || {
                glp_assert_(
                    b"1 <= k && k <= m+n\0" as *const u8 as *const libc::c_char,
                    b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                    1078 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        *aB
            .offset(
                i as isize,
            ) = if k <= m { 0.0f64 } else { *a.offset((k - m) as isize) };
        i += 1;
        i;
    }
    rho = aB;
    glp_btran(P, rho);
    len = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= m {
        if glp_get_row_stat(P, i) != 1 as libc::c_int {
            alfa = -*rho.offset(i as isize);
            if alfa != 0.0f64 {
                len += 1;
                len;
                *ind.offset(len as isize) = i;
                *val.offset(len as isize) = alfa;
            }
        }
        i += 1;
        i;
    }
    iii = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    vvv = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    j = 1 as libc::c_int;
    while j <= n {
        if glp_get_col_stat(P, j) != 1 as libc::c_int {
            alfa = *a.offset(j as isize);
            lll = glp_get_mat_col(P, j, iii, vvv);
            t = 1 as libc::c_int;
            while t <= lll {
                alfa
                    += *vvv.offset(t as isize)
                        * *rho.offset(*iii.offset(t as isize) as isize);
                t += 1;
                t;
            }
            if alfa != 0.0f64 {
                len += 1;
                len;
                *ind.offset(len as isize) = m + j;
                *val.offset(len as isize) = alfa;
            }
        }
        j += 1;
        j;
    }
    (len <= n
        || {
            glp_assert_(
                b"len <= n\0" as *const u8 as *const libc::c_char,
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                1110 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    glp_free(iii as *mut libc::c_void);
    glp_free(vvv as *mut libc::c_void);
    glp_free(aB as *mut libc::c_void);
    glp_free(a as *mut libc::c_void);
    return len;
}
pub unsafe extern "C" fn glp_transform_col(
    mut P: *mut glp_prob,
    mut len: libc::c_int,
    mut ind: *mut libc::c_int,
    mut val: *mut libc::c_double,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut alfa: *mut libc::c_double = 0 as *mut libc::c_double;
    if glp_bf_exists(P) == 0 {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            1190 as libc::c_int,
        ))
            .unwrap()(
            b"glp_transform_col: basis factorization does not exist \n\0" as *const u8
                as *const libc::c_char,
        );
    }
    m = glp_get_num_rows(P);
    a = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    i = 1 as libc::c_int;
    while i <= m {
        *a.offset(i as isize) = 0.0f64;
        i += 1;
        i;
    }
    if !(0 as libc::c_int <= len && len <= m) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            1197 as libc::c_int,
        ))
            .unwrap()(
            b"glp_transform_col: len = %d; invalid column length\n\0" as *const u8
                as *const libc::c_char,
            len,
        );
    }
    t = 1 as libc::c_int;
    while t <= len {
        i = *ind.offset(t as isize);
        if !(1 as libc::c_int <= i && i <= m) {
            (glp_error_(
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                1202 as libc::c_int,
            ))
                .unwrap()(
                b"glp_transform_col: ind[%d] = %d; row index out of range\n\0"
                    as *const u8 as *const libc::c_char,
                t,
                i,
            );
        }
        if *val.offset(t as isize) == 0.0f64 {
            (glp_error_(
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                1205 as libc::c_int,
            ))
                .unwrap()(
                b"glp_transform_col: val[%d] = 0; zero coefficient not allowed\n\0"
                    as *const u8 as *const libc::c_char,
                t,
            );
        }
        if *a.offset(i as isize) != 0.0f64 {
            (glp_error_(
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                1208 as libc::c_int,
            ))
                .unwrap()(
                b"glp_transform_col: ind[%d] = %d; duplicate row indices not allowed\n\0"
                    as *const u8 as *const libc::c_char,
                t,
                i,
            );
        }
        *a.offset(i as isize) = *val.offset(t as isize);
        t += 1;
        t;
    }
    alfa = a;
    glp_ftran(P, alfa);
    len = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= m {
        if *alfa.offset(i as isize) != 0.0f64 {
            len += 1;
            len;
            *ind.offset(len as isize) = glp_get_bhead(P, i);
            *val.offset(len as isize) = *alfa.offset(i as isize);
        }
        i += 1;
        i;
    }
    glp_free(a as *mut libc::c_void);
    return len;
}
pub unsafe extern "C" fn glp_prim_rtest(
    mut P: *mut glp_prob,
    mut len: libc::c_int,
    mut ind: *const libc::c_int,
    mut val: *const libc::c_double,
    mut dir: libc::c_int,
    mut eps: libc::c_double,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut piv: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut stat: libc::c_int = 0;
    let mut alfa: libc::c_double = 0.;
    let mut big: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut teta: libc::c_double = 0.;
    if glp_get_prim_stat(P) != 2 as libc::c_int {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            1290 as libc::c_int,
        ))
            .unwrap()(
            b"glp_prim_rtest: basic solution is not primal feasible \n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(dir == 1 as libc::c_int || dir == -(1 as libc::c_int)) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            1293 as libc::c_int,
        ))
            .unwrap()(
            b"glp_prim_rtest: dir = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            dir,
        );
    }
    if !(0.0f64 < eps && eps < 1.0f64) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            1295 as libc::c_int,
        ))
            .unwrap()(
            b"glp_prim_rtest: eps = %g; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            eps,
        );
    }
    m = glp_get_num_rows(P);
    n = glp_get_num_cols(P);
    piv = 0 as libc::c_int;
    teta = 1.7976931348623157e+308f64;
    big = 0.0f64;
    let mut current_block_36: u64;
    t = 1 as libc::c_int;
    while t <= len {
        k = *ind.offset(t as isize);
        if !(1 as libc::c_int <= k && k <= m + n) {
            (glp_error_(
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                1305 as libc::c_int,
            ))
                .unwrap()(
                b"glp_prim_rtest: ind[%d] = %d; variable number out of range\n\0"
                    as *const u8 as *const libc::c_char,
                t,
                k,
            );
        }
        if k <= m {
            type_0 = glp_get_row_type(P, k);
            lb = glp_get_row_lb(P, k);
            ub = glp_get_row_ub(P, k);
            stat = glp_get_row_stat(P, k);
            beta = glp_get_row_prim(P, k);
        } else {
            type_0 = glp_get_col_type(P, k - m);
            lb = glp_get_col_lb(P, k - m);
            ub = glp_get_col_ub(P, k - m);
            stat = glp_get_col_stat(P, k - m);
            beta = glp_get_col_prim(P, k - m);
        }
        if stat != 1 as libc::c_int {
            (glp_error_(
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                1324 as libc::c_int,
            ))
                .unwrap()(
                b"glp_prim_rtest: ind[%d] = %d; non-basic variable not allowed\n\0"
                    as *const u8 as *const libc::c_char,
                t,
                k,
            );
        }
        alfa = if dir > 0 as libc::c_int {
            *val.offset(t as isize)
        } else {
            -*val.offset(t as isize)
        };
        if !(type_0 == 1 as libc::c_int) {
            if type_0 == 2 as libc::c_int {
                current_block_36 = 10043043949733653460;
            } else {
                if type_0 == 3 as libc::c_int {
                    current_block_36 = 14763689060501151050;
                } else if type_0 == 4 as libc::c_int {
                    if alfa < 0.0f64 {
                        current_block_36 = 10043043949733653460;
                    } else {
                        current_block_36 = 14763689060501151050;
                    }
                } else if type_0 == 5 as libc::c_int {
                    if -eps < alfa && alfa < eps {
                        current_block_36 = 10886091980245723256;
                    } else {
                        temp = 0.0f64;
                        current_block_36 = 14832935472441733737;
                    }
                } else {
                    (type_0 != type_0
                        || {
                            glp_assert_(
                                b"type != type\0" as *const u8 as *const libc::c_char,
                                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                                1356 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    current_block_36 = 14832935472441733737;
                }
                match current_block_36 {
                    14832935472441733737 => {}
                    10886091980245723256 => {}
                    10043043949733653460 => {}
                    _ => {
                        if alfa < eps {
                            current_block_36 = 10886091980245723256;
                        } else {
                            temp = (ub - beta) / alfa;
                            current_block_36 = 14832935472441733737;
                        }
                    }
                }
            }
            match current_block_36 {
                10886091980245723256 => {}
                _ => {
                    match current_block_36 {
                        10043043949733653460 => {
                            if alfa > -eps {
                                current_block_36 = 10886091980245723256;
                            } else {
                                temp = (lb - beta) / alfa;
                                current_block_36 = 14832935472441733737;
                            }
                        }
                        _ => {}
                    }
                    match current_block_36 {
                        10886091980245723256 => {}
                        _ => {
                            if temp < 0.0f64 {
                                temp = 0.0f64;
                            }
                            if teta > temp || teta == temp && big < fabs(alfa) {
                                piv = t;
                                teta = temp;
                                big = fabs(alfa);
                            }
                        }
                    }
                }
            }
        }
        t += 1;
        t;
    }
    return piv;
}
pub unsafe extern "C" fn glp_dual_rtest(
    mut P: *mut glp_prob,
    mut len: libc::c_int,
    mut ind: *const libc::c_int,
    mut val: *const libc::c_double,
    mut dir: libc::c_int,
    mut eps: libc::c_double,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut piv: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut stat: libc::c_int = 0;
    let mut alfa: libc::c_double = 0.;
    let mut big: libc::c_double = 0.;
    let mut cost: libc::c_double = 0.;
    let mut obj: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut teta: libc::c_double = 0.;
    if glp_get_dual_stat(P) != 2 as libc::c_int {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            1432 as libc::c_int,
        ))
            .unwrap()(
            b"glp_dual_rtest: basic solution is not dual feasible\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(dir == 1 as libc::c_int || dir == -(1 as libc::c_int)) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            1435 as libc::c_int,
        ))
            .unwrap()(
            b"glp_dual_rtest: dir = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            dir,
        );
    }
    if !(0.0f64 < eps && eps < 1.0f64) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            1437 as libc::c_int,
        ))
            .unwrap()(
            b"glp_dual_rtest: eps = %g; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            eps,
        );
    }
    m = glp_get_num_rows(P);
    n = glp_get_num_cols(P);
    obj = if glp_get_obj_dir(P) == 1 as libc::c_int { 1.0f64 } else { -1.0f64 };
    piv = 0 as libc::c_int;
    teta = 1.7976931348623157e+308f64;
    big = 0.0f64;
    let mut current_block_31: u64;
    t = 1 as libc::c_int;
    while t <= len {
        k = *ind.offset(t as isize);
        if !(1 as libc::c_int <= k && k <= m + n) {
            (glp_error_(
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                1449 as libc::c_int,
            ))
                .unwrap()(
                b"glp_dual_rtest: ind[%d] = %d; variable number out of range\n\0"
                    as *const u8 as *const libc::c_char,
                t,
                k,
            );
        }
        if k <= m {
            stat = glp_get_row_stat(P, k);
            cost = glp_get_row_dual(P, k);
        } else {
            stat = glp_get_col_stat(P, k - m);
            cost = glp_get_col_dual(P, k - m);
        }
        if stat == 1 as libc::c_int {
            (glp_error_(
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                1462 as libc::c_int,
            ))
                .unwrap()(
                b"glp_dual_rtest: ind[%d] = %d; basic variable not allowed\n\0"
                    as *const u8 as *const libc::c_char,
                t,
                k,
            );
        }
        alfa = if dir > 0 as libc::c_int {
            *val.offset(t as isize)
        } else {
            -*val.offset(t as isize)
        };
        if stat == 2 as libc::c_int {
            if alfa < eps {
                current_block_31 = 8515828400728868193;
            } else {
                temp = obj * cost / alfa;
                current_block_31 = 11636175345244025579;
            }
        } else if stat == 3 as libc::c_int {
            if alfa > -eps {
                current_block_31 = 8515828400728868193;
            } else {
                temp = obj * cost / alfa;
                current_block_31 = 11636175345244025579;
            }
        } else if stat == 4 as libc::c_int {
            if -eps < alfa && alfa < eps {
                current_block_31 = 8515828400728868193;
            } else {
                temp = 0.0f64;
                current_block_31 = 11636175345244025579;
            }
        } else if stat == 5 as libc::c_int {
            current_block_31 = 8515828400728868193;
        } else {
            (stat != stat
                || {
                    glp_assert_(
                        b"stat != stat\0" as *const u8 as *const libc::c_char,
                        b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                        1490 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            current_block_31 = 11636175345244025579;
        }
        match current_block_31 {
            11636175345244025579 => {
                if temp < 0.0f64 {
                    temp = 0.0f64;
                }
                if teta > temp || teta == temp && big < fabs(alfa) {
                    piv = t;
                    teta = temp;
                    big = fabs(alfa);
                }
            }
            _ => {}
        }
        t += 1;
        t;
    }
    return piv;
}
pub unsafe extern "C" fn _glp_analyze_row(
    mut P: *mut glp_prob,
    mut len: libc::c_int,
    mut ind: *const libc::c_int,
    mut val: *const libc::c_double,
    mut type_0: libc::c_int,
    mut rhs: libc::c_double,
    mut eps: libc::c_double,
    mut _piv: *mut libc::c_int,
    mut _x: *mut libc::c_double,
    mut _dx: *mut libc::c_double,
    mut _y: *mut libc::c_double,
    mut _dy: *mut libc::c_double,
    mut _dz: *mut libc::c_double,
) -> libc::c_int {
    let mut current_block: u64;
    let mut t: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut piv: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut x: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut dz: libc::c_double = 0.;
    if (*P).pbs_stat == 1 as libc::c_int {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            1589 as libc::c_int,
        ))
            .unwrap()(
            b"glp_analyze_row: primal basic solution components are undefined\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if (*P).dbs_stat != 2 as libc::c_int {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            1592 as libc::c_int,
        ))
            .unwrap()(
            b"glp_analyze_row: basic solution is not dual feasible\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(0 as libc::c_int <= len && len <= (*P).n) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            1597 as libc::c_int,
        ))
            .unwrap()(
            b"glp_analyze_row: len = %d; invalid row length\n\0" as *const u8
                as *const libc::c_char,
            len,
        );
    }
    y = 0.0f64;
    t = 1 as libc::c_int;
    while t <= len {
        k = *ind.offset(t as isize);
        if !(1 as libc::c_int <= k && k <= (*P).m + (*P).n) {
            (glp_error_(
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                1603 as libc::c_int,
            ))
                .unwrap()(
                b"glp_analyze_row: ind[%d] = %d; row/column index out of range\n\0"
                    as *const u8 as *const libc::c_char,
                t,
                k,
            );
        }
        if k <= (*P).m {
            if (**((*P).row).offset(k as isize)).stat == 1 as libc::c_int {
                (glp_error_(
                    b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                    1608 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_analyze_row: ind[%d] = %d; basic auxiliary variable is not allowed\n\0"
                        as *const u8 as *const libc::c_char,
                    t,
                    k,
                );
            }
            x = (**((*P).row).offset(k as isize)).prim;
        } else {
            if (**((*P).col).offset((k - (*P).m) as isize)).stat == 1 as libc::c_int {
                (glp_error_(
                    b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                    1615 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_analyze_row: ind[%d] = %d; basic structural variable is not allowed\n\0"
                        as *const u8 as *const libc::c_char,
                    t,
                    k,
                );
            }
            x = (**((*P).col).offset((k - (*P).m) as isize)).prim;
        }
        y += *val.offset(t as isize) * x;
        t += 1;
        t;
    }
    if type_0 == 2 as libc::c_int {
        if y >= rhs {
            ret = 1 as libc::c_int;
            current_block = 5803690297361441364;
        } else {
            dir = 1 as libc::c_int;
            current_block = 15345278821338558188;
        }
    } else if type_0 == 3 as libc::c_int {
        if y <= rhs {
            ret = 1 as libc::c_int;
            current_block = 5803690297361441364;
        } else {
            dir = -(1 as libc::c_int);
            current_block = 15345278821338558188;
        }
    } else {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            1642 as libc::c_int,
        ))
            .unwrap()(
            b"glp_analyze_row: type = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            type_0,
        );
        current_block = 15345278821338558188;
    }
    match current_block {
        15345278821338558188 => {
            dy = rhs - y;
            piv = glp_dual_rtest(P, len, ind, val, dir, eps);
            if piv == 0 as libc::c_int {
                ret = 2 as libc::c_int;
            } else {
                k = *ind.offset(piv as isize);
                (1 as libc::c_int <= k && k <= (*P).m + (*P).n
                    || {
                        glp_assert_(
                            b"1 <= k && k <= P->m+P->n\0" as *const u8
                                as *const libc::c_char,
                            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                            1656 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                if k <= (*P).m {
                    x = (**((*P).row).offset(k as isize)).prim;
                } else {
                    x = (**((*P).col).offset((k - (*P).m) as isize)).prim;
                }
                (*val.offset(piv as isize) != 0.0f64
                    || {
                        glp_assert_(
                            b"val[piv] != 0.0\0" as *const u8 as *const libc::c_char,
                            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                            1663 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                dx = dy / *val.offset(piv as isize);
                if k <= (*P).m {
                    dz = (**((*P).row).offset(k as isize)).dual * dx;
                } else {
                    dz = (**((*P).col).offset((k - (*P).m) as isize)).dual * dx;
                }
                if !_piv.is_null() {
                    *_piv = piv;
                }
                if !_x.is_null() {
                    *_x = x;
                }
                if !_dx.is_null() {
                    *_dx = dx;
                }
                if !_y.is_null() {
                    *_y = y;
                }
                if !_dy.is_null() {
                    *_dy = dy;
                }
                if !_dz.is_null() {
                    *_dz = dz;
                }
            }
        }
        _ => {}
    }
    return ret;
}
pub unsafe extern "C" fn glp_analyze_bound(
    mut P: *mut glp_prob,
    mut k: libc::c_int,
    mut value1: *mut libc::c_double,
    mut var1: *mut libc::c_int,
    mut value2: *mut libc::c_double,
    mut var2: *mut libc::c_int,
) {
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut stat: libc::c_int = 0;
    let mut kase: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut piv: libc::c_int = 0;
    let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut x: libc::c_double = 0.;
    let mut new_x: libc::c_double = 0.;
    let mut ll: libc::c_double = 0.;
    let mut uu: libc::c_double = 0.;
    let mut xx: libc::c_double = 0.;
    let mut delta: libc::c_double = 0.;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    m = (*P).m;
    n = (*P).n;
    if !((*P).pbs_stat == 2 as libc::c_int && (*P).dbs_stat == 2 as libc::c_int) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            1807 as libc::c_int,
        ))
            .unwrap()(
            b"glp_analyze_bound: optimal basic solution required\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(m == 0 as libc::c_int || (*P).valid != 0) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            1809 as libc::c_int,
        ))
            .unwrap()(
            b"glp_analyze_bound: basis factorization required\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(1 as libc::c_int <= k && k <= m + n) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            1811 as libc::c_int,
        ))
            .unwrap()(
            b"glp_analyze_bound: k = %d; variable number out of range\n\0" as *const u8
                as *const libc::c_char,
            k,
        );
    }
    if k <= m {
        row = *((*P).row).offset(k as isize);
        stat = (*row).stat;
        x = (*row).prim;
    } else {
        col = *((*P).col).offset((k - m) as isize);
        stat = (*col).stat;
        x = (*col).prim;
    }
    if stat == 1 as libc::c_int {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            1826 as libc::c_int,
        ))
            .unwrap()(
            b"glp_analyze_bound: k = %d; basic variable not allowed \n\0" as *const u8
                as *const libc::c_char,
            k,
        );
    }
    ind = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    val = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    len = glp_eval_tab_col(P, k, ind, val);
    (0 as libc::c_int <= len && len <= m
        || {
            glp_assert_(
                b"0 <= len && len <= m\0" as *const u8 as *const libc::c_char,
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                1834 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    kase = -(1 as libc::c_int);
    while kase <= 1 as libc::c_int {
        piv = glp_prim_rtest(
            P,
            len,
            ind as *const libc::c_int,
            val as *const libc::c_double,
            kase,
            1e-9f64,
        );
        if piv == 0 as libc::c_int {
            p = 0 as libc::c_int;
            new_x = if kase < 0 as libc::c_int {
                -1.7976931348623157e+308f64
            } else {
                1.7976931348623157e+308f64
            };
        } else {
            (1 as libc::c_int <= piv && piv <= len
                || {
                    glp_assert_(
                        b"1 <= piv && piv <= len\0" as *const u8 as *const libc::c_char,
                        b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                        1850 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            p = *ind.offset(piv as isize);
            if p <= m {
                row = *((*P).row).offset(p as isize);
                ll = glp_get_row_lb(P, (*row).i);
                uu = glp_get_row_ub(P, (*row).i);
                stat = (*row).stat;
                xx = (*row).prim;
            } else {
                col = *((*P).col).offset((p - m) as isize);
                ll = glp_get_col_lb(P, (*col).j);
                uu = glp_get_col_ub(P, (*col).j);
                stat = (*col).stat;
                xx = (*col).prim;
            }
            (stat == 1 as libc::c_int
                || {
                    glp_assert_(
                        b"stat == GLP_BS\0" as *const u8 as *const libc::c_char,
                        b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                        1866 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if kase < 0 as libc::c_int && *val.offset(piv as isize) > 0.0f64
                || kase > 0 as libc::c_int && *val.offset(piv as isize) < 0.0f64
            {
                (ll != -1.7976931348623157e+308f64
                    || {
                        glp_assert_(
                            b"ll != -DBL_MAX\0" as *const u8 as *const libc::c_char,
                            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                            1871 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                delta = ll - xx;
            } else {
                (uu != 1.7976931348623157e+308f64
                    || {
                        glp_assert_(
                            b"uu != +DBL_MAX\0" as *const u8 as *const libc::c_char,
                            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                            1876 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                delta = uu - xx;
            }
            (*val.offset(piv as isize) != 0.0f64
                || {
                    glp_assert_(
                        b"val[piv] != 0.0\0" as *const u8 as *const libc::c_char,
                        b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                        1882 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            new_x = x + delta / *val.offset(piv as isize);
        }
        if kase < 0 as libc::c_int {
            if !value1.is_null() {
                *value1 = new_x;
            }
            if !var1.is_null() {
                *var1 = p;
            }
        } else {
            if !value2.is_null() {
                *value2 = new_x;
            }
            if !var2.is_null() {
                *var2 = p;
            }
        }
        kase += 2 as libc::c_int;
    }
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
}
pub unsafe extern "C" fn glp_analyze_coef(
    mut P: *mut glp_prob,
    mut k: libc::c_int,
    mut coef1: *mut libc::c_double,
    mut var1: *mut libc::c_int,
    mut value1: *mut libc::c_double,
    mut coef2: *mut libc::c_double,
    mut var2: *mut libc::c_int,
    mut value2: *mut libc::c_double,
) {
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut stat: libc::c_int = 0;
    let mut kase: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut clen: libc::c_int = 0;
    let mut cpiv: libc::c_int = 0;
    let mut rlen: libc::c_int = 0;
    let mut rpiv: libc::c_int = 0;
    let mut cind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    let mut coef: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut lim_coef: libc::c_double = 0.;
    let mut new_x: libc::c_double = 0.;
    let mut d: libc::c_double = 0.;
    let mut delta: libc::c_double = 0.;
    let mut ll: libc::c_double = 0.;
    let mut uu: libc::c_double = 0.;
    let mut xx: libc::c_double = 0.;
    let mut rval: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut cval: *mut libc::c_double = 0 as *mut libc::c_double;
    m = (*P).m;
    n = (*P).n;
    if !((*P).pbs_stat == 2 as libc::c_int && (*P).dbs_stat == 2 as libc::c_int) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            1974 as libc::c_int,
        ))
            .unwrap()(
            b"glp_analyze_coef: optimal basic solution required\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(m == 0 as libc::c_int || (*P).valid != 0) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            1976 as libc::c_int,
        ))
            .unwrap()(
            b"glp_analyze_coef: basis factorization required\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(1 as libc::c_int <= k && k <= m + n) {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            1978 as libc::c_int,
        ))
            .unwrap()(
            b"glp_analyze_coef: k = %d; variable number out of range\n\0" as *const u8
                as *const libc::c_char,
            k,
        );
    }
    if k <= m {
        row = *((*P).row).offset(k as isize);
        type_0 = (*row).type_0;
        lb = (*row).lb;
        ub = (*row).ub;
        coef = 0.0f64;
        stat = (*row).stat;
        x = (*row).prim;
    } else {
        col = *((*P).col).offset((k - m) as isize);
        type_0 = (*col).type_0;
        lb = (*col).lb;
        ub = (*col).ub;
        coef = (*col).coef;
        stat = (*col).stat;
        x = (*col).prim;
    }
    if stat != 1 as libc::c_int {
        (glp_error_(
            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
            2001 as libc::c_int,
        ))
            .unwrap()(
            b"glp_analyze_coef: k = %d; non-basic variable not allowed\n\0" as *const u8
                as *const libc::c_char,
            k,
        );
    }
    cind = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    cval = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    rind = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    rval = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    rlen = glp_eval_tab_row(P, k, rind, rval);
    (0 as libc::c_int <= rlen && rlen <= n
        || {
            glp_assert_(
                b"0 <= rlen && rlen <= n\0" as *const u8 as *const libc::c_char,
                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                2011 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    kase = -(1 as libc::c_int);
    while kase <= 1 as libc::c_int {
        if (*P).dir == 1 as libc::c_int {
            dir = -kase;
        } else if (*P).dir == 2 as libc::c_int {
            dir = kase;
        } else {
            (P != P
                || {
                    glp_assert_(
                        b"P != P\0" as *const u8 as *const libc::c_char,
                        b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                        2024 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        rpiv = glp_dual_rtest(
            P,
            rlen,
            rind as *const libc::c_int,
            rval as *const libc::c_double,
            dir,
            1e-9f64,
        );
        if rpiv == 0 as libc::c_int {
            lim_coef = if kase < 0 as libc::c_int {
                -1.7976931348623157e+308f64
            } else {
                1.7976931348623157e+308f64
            };
            q = 0 as libc::c_int;
            new_x = x;
        } else {
            (1 as libc::c_int <= rpiv && rpiv <= rlen
                || {
                    glp_assert_(
                        b"1 <= rpiv && rpiv <= rlen\0" as *const u8
                            as *const libc::c_char,
                        b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                        2039 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            q = *rind.offset(rpiv as isize);
            (1 as libc::c_int <= q && q <= m + n
                || {
                    glp_assert_(
                        b"1 <= q && q <= m+n\0" as *const u8 as *const libc::c_char,
                        b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                        2041 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if q <= m {
                row = *((*P).row).offset(q as isize);
                stat = (*row).stat;
                d = (*row).dual;
            } else {
                col = *((*P).col).offset((q - m) as isize);
                stat = (*col).stat;
                d = (*col).dual;
            }
            (*rval.offset(rpiv as isize) != 0.0f64
                || {
                    glp_assert_(
                        b"rval[rpiv] != 0.0\0" as *const u8 as *const libc::c_char,
                        b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                        2055 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            delta = -d / *rval.offset(rpiv as isize);
            lim_coef = coef + delta;
            if kase < 0 as libc::c_int && *rval.offset(rpiv as isize) > 0.0f64
                || kase > 0 as libc::c_int && *rval.offset(rpiv as isize) < 0.0f64
            {
                dir = 1 as libc::c_int;
            } else {
                dir = -(1 as libc::c_int);
            }
            if (*P).dir == 2 as libc::c_int {
                dir = -dir;
            }
            if dir > 0 as libc::c_int {
                (stat == 2 as libc::c_int || stat == 4 as libc::c_int
                    || {
                        glp_assert_(
                            b"stat == GLP_NL || stat == GLP_NF\0" as *const u8
                                as *const libc::c_char,
                            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                            2082 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            } else {
                (stat == 3 as libc::c_int || stat == 4 as libc::c_int
                    || {
                        glp_assert_(
                            b"stat == GLP_NU || stat == GLP_NF\0" as *const u8
                                as *const libc::c_char,
                            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                            2084 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            clen = glp_eval_tab_col(P, q, cind, cval);
            if k <= m {
                row = *((*P).row).offset(k as isize);
                (*row).type_0 = 1 as libc::c_int;
                (*row).ub = 0.0f64;
                (*row).lb = (*row).ub;
            } else {
                col = *((*P).col).offset((k - m) as isize);
                (*col).type_0 = 1 as libc::c_int;
                (*col).ub = 0.0f64;
                (*col).lb = (*col).ub;
            }
            cpiv = glp_prim_rtest(
                P,
                clen,
                cind as *const libc::c_int,
                cval as *const libc::c_double,
                dir,
                1e-9f64,
            );
            if k <= m {
                row = *((*P).row).offset(k as isize);
                (*row).type_0 = type_0;
                (*row).lb = lb;
                (*row).ub = ub;
            } else {
                col = *((*P).col).offset((k - m) as isize);
                (*col).type_0 = type_0;
                (*col).lb = lb;
                (*col).ub = ub;
            }
            if cpiv == 0 as libc::c_int {
                if dir < 0 as libc::c_int && *rval.offset(rpiv as isize) > 0.0f64
                    || dir > 0 as libc::c_int && *rval.offset(rpiv as isize) < 0.0f64
                {
                    new_x = -1.7976931348623157e+308f64;
                } else {
                    new_x = 1.7976931348623157e+308f64;
                }
            } else {
                (1 as libc::c_int <= cpiv && cpiv <= clen
                    || {
                        glp_assert_(
                            b"1 <= cpiv && cpiv <= clen\0" as *const u8
                                as *const libc::c_char,
                            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                            2128 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                p = *cind.offset(cpiv as isize);
                (1 as libc::c_int <= p && p <= m + n
                    || {
                        glp_assert_(
                            b"1 <= p && p <= m+n\0" as *const u8 as *const libc::c_char,
                            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                            2130 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (p != k
                    || {
                        glp_assert_(
                            b"p != k\0" as *const u8 as *const libc::c_char,
                            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                            2131 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                if p <= m {
                    row = *((*P).row).offset(p as isize);
                    ((*row).stat == 1 as libc::c_int
                        || {
                            glp_assert_(
                                b"row->stat == GLP_BS\0" as *const u8
                                    as *const libc::c_char,
                                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                                2134 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    ll = glp_get_row_lb(P, (*row).i);
                    uu = glp_get_row_ub(P, (*row).i);
                    xx = (*row).prim;
                } else {
                    col = *((*P).col).offset((p - m) as isize);
                    ((*col).stat == 1 as libc::c_int
                        || {
                            glp_assert_(
                                b"col->stat == GLP_BS\0" as *const u8
                                    as *const libc::c_char,
                                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                                2141 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    ll = glp_get_col_lb(P, (*col).j);
                    uu = glp_get_col_ub(P, (*col).j);
                    xx = (*col).prim;
                }
                if dir < 0 as libc::c_int && *cval.offset(cpiv as isize) > 0.0f64
                    || dir > 0 as libc::c_int && *cval.offset(cpiv as isize) < 0.0f64
                {
                    (ll != -1.7976931348623157e+308f64
                        || {
                            glp_assert_(
                                b"ll != -DBL_MAX\0" as *const u8 as *const libc::c_char,
                                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                                2150 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    delta = ll - xx;
                } else {
                    (uu != 1.7976931348623157e+308f64
                        || {
                            glp_assert_(
                                b"uu != +DBL_MAX\0" as *const u8 as *const libc::c_char,
                                b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                                2155 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    delta = uu - xx;
                }
                (*cval.offset(cpiv as isize) != 0.0f64
                    || {
                        glp_assert_(
                            b"cval[cpiv] != 0.0\0" as *const u8 as *const libc::c_char,
                            b"draft/glpapi12.c\0" as *const u8 as *const libc::c_char,
                            2160 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                new_x = x
                    + *rval.offset(rpiv as isize) / *cval.offset(cpiv as isize) * delta;
            }
        }
        if kase < 0 as libc::c_int {
            if !coef1.is_null() {
                *coef1 = lim_coef;
            }
            if !var1.is_null() {
                *var1 = q;
            }
            if !value1.is_null() {
                *value1 = new_x;
            }
        } else {
            if !coef2.is_null() {
                *coef2 = lim_coef;
            }
            if !var2.is_null() {
                *var2 = q;
            }
            if !value2.is_null() {
                *value2 = new_x;
            }
        }
        kase += 2 as libc::c_int;
    }
    glp_free(cind as *mut libc::c_void);
    glp_free(cval as *mut libc::c_void);
    glp_free(rind as *mut libc::c_void);
    glp_free(rval as *mut libc::c_void);
}
