use ::libc;
extern "C" {
    pub type glp_file;
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn _glp_close(f: *mut glp_file) -> libc::c_int;
    fn _glp_format(f: *mut glp_file, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn _glp_ioerr(f: *mut glp_file) -> libc::c_int;
    fn _glp_open(name: *const libc::c_char, mode: *const libc::c_char) -> *mut glp_file;
    fn _glp_get_err_msg() -> *const libc::c_char;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_get_num_int(P: *mut glp_prob) -> libc::c_int;
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
pub unsafe extern "C" fn glp_write_prob(
    mut P: *mut glp_prob,
    mut flags: libc::c_int,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut fp: *mut glp_file = 0 as *mut glp_file;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut mip: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if flags != 0 as libc::c_int {
        (glp_error_(
            b"api/wrprob.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int,
        ))
            .unwrap()(
            b"glp_write_prob: flags = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            flags,
        );
    }
    if fname.is_null() {
        (glp_error_(
            b"api/wrprob.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
        ))
            .unwrap()(
            b"glp_write_prob: fname = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            fname,
        );
    }
    glp_printf(
        b"Writing problem data to '%s'...\n\0" as *const u8 as *const libc::c_char,
        fname,
    );
    fp = _glp_open(fname, b"w\0" as *const u8 as *const libc::c_char);
    count = 0 as libc::c_int;
    if fp.is_null() {
        glp_printf(
            b"Unable to create '%s' - %s\n\0" as *const u8 as *const libc::c_char,
            fname,
            _glp_get_err_msg(),
        );
        ret = 1 as libc::c_int;
    } else {
        mip = (glp_get_num_int(P) > 0 as libc::c_int) as libc::c_int;
        _glp_format(
            fp,
            b"p %s %s %d %d %d\n\0" as *const u8 as *const libc::c_char,
            (if mip == 0 {
                b"lp\0" as *const u8 as *const libc::c_char
            } else {
                b"mip\0" as *const u8 as *const libc::c_char
            }),
            (if (*P).dir == 1 as libc::c_int {
                b"min\0" as *const u8 as *const libc::c_char
            } else {
                (if (*P).dir == 2 as libc::c_int {
                    b"max\0" as *const u8 as *const libc::c_char
                } else {
                    b"???\0" as *const u8 as *const libc::c_char
                })
            }),
            (*P).m,
            (*P).n,
            (*P).nnz,
        );
        count += 1;
        count;
        if !((*P).name).is_null() {
            _glp_format(
                fp,
                b"n p %s\n\0" as *const u8 as *const libc::c_char,
                (*P).name,
            );
            count += 1;
            count;
        }
        if !((*P).obj).is_null() {
            _glp_format(fp, b"n z %s\n\0" as *const u8 as *const libc::c_char, (*P).obj);
            count += 1;
            count;
        }
        i = 1 as libc::c_int;
        while i <= (*P).m {
            row = *((*P).row).offset(i as isize);
            if !((*row).type_0 == 5 as libc::c_int && (*row).lb == 0.0f64) {
                _glp_format(fp, b"i %d \0" as *const u8 as *const libc::c_char, i);
                count += 1;
                count;
                if (*row).type_0 == 1 as libc::c_int {
                    _glp_format(fp, b"f\n\0" as *const u8 as *const libc::c_char);
                } else if (*row).type_0 == 2 as libc::c_int {
                    _glp_format(
                        fp,
                        b"l %.*g\n\0" as *const u8 as *const libc::c_char,
                        15 as libc::c_int,
                        (*row).lb,
                    );
                } else if (*row).type_0 == 3 as libc::c_int {
                    _glp_format(
                        fp,
                        b"u %.*g\n\0" as *const u8 as *const libc::c_char,
                        15 as libc::c_int,
                        (*row).ub,
                    );
                } else if (*row).type_0 == 4 as libc::c_int {
                    _glp_format(
                        fp,
                        b"d %.*g %.*g\n\0" as *const u8 as *const libc::c_char,
                        15 as libc::c_int,
                        (*row).lb,
                        15 as libc::c_int,
                        (*row).ub,
                    );
                } else if (*row).type_0 == 5 as libc::c_int {
                    _glp_format(
                        fp,
                        b"s %.*g\n\0" as *const u8 as *const libc::c_char,
                        15 as libc::c_int,
                        (*row).lb,
                    );
                } else {
                    (row != row
                        || {
                            glp_assert_(
                                b"row != row\0" as *const u8 as *const libc::c_char,
                                b"api/wrprob.c\0" as *const u8 as *const libc::c_char,
                                95 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
            }
            if !((*row).name).is_null() {
                _glp_format(
                    fp,
                    b"n i %d %s\n\0" as *const u8 as *const libc::c_char,
                    i,
                    (*row).name,
                );
                count += 1;
                count;
            }
            i += 1;
            i;
        }
        j = 1 as libc::c_int;
        while j <= (*P).n {
            col = *((*P).col).offset(j as isize);
            if !(mip == 0 && (*col).type_0 == 2 as libc::c_int && (*col).lb == 0.0f64) {
                if !(mip != 0 && (*col).kind == 2 as libc::c_int
                    && (*col).type_0 == 4 as libc::c_int && (*col).lb == 0.0f64
                    && (*col).ub == 1.0f64)
                {
                    _glp_format(fp, b"j %d \0" as *const u8 as *const libc::c_char, j);
                    count += 1;
                    count;
                    if mip != 0 {
                        if (*col).kind == 1 as libc::c_int {
                            _glp_format(fp, b"c \0" as *const u8 as *const libc::c_char);
                        } else if (*col).kind == 2 as libc::c_int {
                            _glp_format(fp, b"i \0" as *const u8 as *const libc::c_char);
                        } else {
                            (col != col
                                || {
                                    glp_assert_(
                                        b"col != col\0" as *const u8 as *const libc::c_char,
                                        b"api/wrprob.c\0" as *const u8 as *const libc::c_char,
                                        114 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                        }
                    }
                    if (*col).type_0 == 1 as libc::c_int {
                        _glp_format(fp, b"f\n\0" as *const u8 as *const libc::c_char);
                    } else if (*col).type_0 == 2 as libc::c_int {
                        _glp_format(
                            fp,
                            b"l %.*g\n\0" as *const u8 as *const libc::c_char,
                            15 as libc::c_int,
                            (*col).lb,
                        );
                    } else if (*col).type_0 == 3 as libc::c_int {
                        _glp_format(
                            fp,
                            b"u %.*g\n\0" as *const u8 as *const libc::c_char,
                            15 as libc::c_int,
                            (*col).ub,
                        );
                    } else if (*col).type_0 == 4 as libc::c_int {
                        _glp_format(
                            fp,
                            b"d %.*g %.*g\n\0" as *const u8 as *const libc::c_char,
                            15 as libc::c_int,
                            (*col).lb,
                            15 as libc::c_int,
                            (*col).ub,
                        );
                    } else if (*col).type_0 == 5 as libc::c_int {
                        _glp_format(
                            fp,
                            b"s %.*g\n\0" as *const u8 as *const libc::c_char,
                            15 as libc::c_int,
                            (*col).lb,
                        );
                    } else {
                        (col != col
                            || {
                                glp_assert_(
                                    b"col != col\0" as *const u8 as *const libc::c_char,
                                    b"api/wrprob.c\0" as *const u8 as *const libc::c_char,
                                    128 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                    }
                }
            }
            if !((*col).name).is_null() {
                _glp_format(
                    fp,
                    b"n j %d %s\n\0" as *const u8 as *const libc::c_char,
                    j,
                    (*col).name,
                );
                count += 1;
                count;
            }
            j += 1;
            j;
        }
        if (*P).c0 != 0.0f64 {
            _glp_format(
                fp,
                b"a 0 0 %.*g\n\0" as *const u8 as *const libc::c_char,
                15 as libc::c_int,
                (*P).c0,
            );
            count += 1;
            count;
        }
        j = 1 as libc::c_int;
        while j <= (*P).n {
            col = *((*P).col).offset(j as isize);
            if (*col).coef != 0.0f64 {
                _glp_format(
                    fp,
                    b"a 0 %d %.*g\n\0" as *const u8 as *const libc::c_char,
                    j,
                    15 as libc::c_int,
                    (*col).coef,
                );
                count += 1;
                count;
            }
            j += 1;
            j;
        }
        i = 1 as libc::c_int;
        while i <= (*P).m {
            row = *((*P).row).offset(i as isize);
            aij = (*row).ptr;
            while !aij.is_null() {
                _glp_format(
                    fp,
                    b"a %d %d %.*g\n\0" as *const u8 as *const libc::c_char,
                    i,
                    (*(*aij).col).j,
                    15 as libc::c_int,
                    (*aij).val,
                );
                count += 1;
                count;
                aij = (*aij).r_next;
            }
            i += 1;
            i;
        }
        _glp_format(fp, b"e o f\n\0" as *const u8 as *const libc::c_char);
        count += 1;
        count;
        if _glp_ioerr(fp) != 0 {
            glp_printf(
                b"Write error on '%s' - %s\n\0" as *const u8 as *const libc::c_char,
                fname,
                _glp_get_err_msg(),
            );
            ret = 1 as libc::c_int;
        } else {
            glp_printf(
                b"%d lines were written\n\0" as *const u8 as *const libc::c_char,
                count,
            );
            ret = 0 as libc::c_int;
        }
    }
    if !fp.is_null() {
        _glp_close(fp);
    }
    return ret;
}
