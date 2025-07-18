use ::libc;
extern "C" {
    pub type glp_file;
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn _glp_get_err_msg() -> *const libc::c_char;
    fn _glp_open(name: *const libc::c_char, mode: *const libc::c_char) -> *mut glp_file;
    fn _glp_ioerr(f: *mut glp_file) -> libc::c_int;
    fn _glp_format(f: *mut glp_file, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn _glp_close(f: *mut glp_file) -> libc::c_int;
    fn glp_get_num_int(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_num_bin(P: *mut glp_prob) -> libc::c_int;
    fn glp_mip_status(P: *mut glp_prob) -> libc::c_int;
    fn glp_check_kkt(
        P: *mut glp_prob,
        sol: libc::c_int,
        cond: libc::c_int,
        ae_max: *mut libc::c_double,
        ae_ind: *mut libc::c_int,
        re_max: *mut libc::c_double,
        re_ind: *mut libc::c_int,
    );
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
pub unsafe extern "C" fn glp_print_mip(
    mut P: *mut glp_prob,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut fp: *mut glp_file = 0 as *mut glp_file;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut ae_ind: libc::c_int = 0;
    let mut re_ind: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut ae_max: libc::c_double = 0.;
    let mut re_max: libc::c_double = 0.;
    glp_printf(
        b"Writing MIP solution to '%s'...\n\0" as *const u8 as *const libc::c_char,
        fname,
    );
    fp = _glp_open(fname, b"w\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        glp_printf(
            b"Unable to create '%s' - %s\n\0" as *const u8 as *const libc::c_char,
            fname,
            _glp_get_err_msg(),
        );
        ret = 1 as libc::c_int;
    } else {
        _glp_format(
            fp,
            b"%-12s%s\n\0" as *const u8 as *const libc::c_char,
            b"Problem:\0" as *const u8 as *const libc::c_char,
            if ((*P).name).is_null() {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                (*P).name as *const libc::c_char
            },
        );
        _glp_format(
            fp,
            b"%-12s%d\n\0" as *const u8 as *const libc::c_char,
            b"Rows:\0" as *const u8 as *const libc::c_char,
            (*P).m,
        );
        _glp_format(
            fp,
            b"%-12s%d (%d integer, %d binary)\n\0" as *const u8 as *const libc::c_char,
            b"Columns:\0" as *const u8 as *const libc::c_char,
            (*P).n,
            glp_get_num_int(P),
            glp_get_num_bin(P),
        );
        _glp_format(
            fp,
            b"%-12s%d\n\0" as *const u8 as *const libc::c_char,
            b"Non-zeros:\0" as *const u8 as *const libc::c_char,
            (*P).nnz,
        );
        t = glp_mip_status(P);
        _glp_format(
            fp,
            b"%-12s%s\n\0" as *const u8 as *const libc::c_char,
            b"Status:\0" as *const u8 as *const libc::c_char,
            if t == 5 as libc::c_int {
                b"INTEGER OPTIMAL\0" as *const u8 as *const libc::c_char
            } else if t == 2 as libc::c_int {
                b"INTEGER NON-OPTIMAL\0" as *const u8 as *const libc::c_char
            } else if t == 4 as libc::c_int {
                b"INTEGER EMPTY\0" as *const u8 as *const libc::c_char
            } else if t == 1 as libc::c_int {
                b"INTEGER UNDEFINED\0" as *const u8 as *const libc::c_char
            } else {
                b"???\0" as *const u8 as *const libc::c_char
            },
        );
        _glp_format(
            fp,
            b"%-12s%s%s%.10g (%s)\n\0" as *const u8 as *const libc::c_char,
            b"Objective:\0" as *const u8 as *const libc::c_char,
            if ((*P).obj).is_null() {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                (*P).obj as *const libc::c_char
            },
            if ((*P).obj).is_null() {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b" = \0" as *const u8 as *const libc::c_char
            },
            (*P).mip_obj,
            if (*P).dir == 1 as libc::c_int {
                b"MINimum\0" as *const u8 as *const libc::c_char
            } else if (*P).dir == 2 as libc::c_int {
                b"MAXimum\0" as *const u8 as *const libc::c_char
            } else {
                b"???\0" as *const u8 as *const libc::c_char
            },
        );
        _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
        _glp_format(
            fp,
            b"   No.   Row name        Activity     Lower bound   Upper bound\n\0"
                as *const u8 as *const libc::c_char,
        );
        _glp_format(
            fp,
            b"------ ------------    ------------- ------------- -------------\n\0"
                as *const u8 as *const libc::c_char,
        );
        i = 1 as libc::c_int;
        while i <= (*P).m {
            row = *((*P).row).offset(i as isize);
            _glp_format(fp, b"%6d \0" as *const u8 as *const libc::c_char, i);
            if ((*row).name).is_null()
                || strlen((*row).name) <= 12 as libc::c_int as libc::c_ulong
            {
                _glp_format(
                    fp,
                    b"%-12s \0" as *const u8 as *const libc::c_char,
                    if ((*row).name).is_null() {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        (*row).name as *const libc::c_char
                    },
                );
            } else {
                _glp_format(
                    fp,
                    b"%s\n%20s\0" as *const u8 as *const libc::c_char,
                    (*row).name,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
            _glp_format(
                fp,
                b"%3s\0" as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
            );
            _glp_format(
                fp,
                b"%13.6g \0" as *const u8 as *const libc::c_char,
                if fabs((*row).mipx) <= 1e-9f64 { 0.0f64 } else { (*row).mipx },
            );
            if (*row).type_0 == 2 as libc::c_int || (*row).type_0 == 4 as libc::c_int
                || (*row).type_0 == 5 as libc::c_int
            {
                _glp_format(
                    fp,
                    b"%13.6g \0" as *const u8 as *const libc::c_char,
                    (*row).lb,
                );
            } else {
                _glp_format(
                    fp,
                    b"%13s \0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
            if (*row).type_0 == 3 as libc::c_int || (*row).type_0 == 4 as libc::c_int {
                _glp_format(
                    fp,
                    b"%13.6g \0" as *const u8 as *const libc::c_char,
                    (*row).ub,
                );
            } else {
                _glp_format(
                    fp,
                    b"%13s \0" as *const u8 as *const libc::c_char,
                    if (*row).type_0 == 5 as libc::c_int {
                        b"=\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                );
            }
            _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
            i += 1;
            i;
        }
        _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
        _glp_format(
            fp,
            b"   No. Column name       Activity     Lower bound   Upper bound\n\0"
                as *const u8 as *const libc::c_char,
        );
        _glp_format(
            fp,
            b"------ ------------    ------------- ------------- -------------\n\0"
                as *const u8 as *const libc::c_char,
        );
        j = 1 as libc::c_int;
        while j <= (*P).n {
            col = *((*P).col).offset(j as isize);
            _glp_format(fp, b"%6d \0" as *const u8 as *const libc::c_char, j);
            if ((*col).name).is_null()
                || strlen((*col).name) <= 12 as libc::c_int as libc::c_ulong
            {
                _glp_format(
                    fp,
                    b"%-12s \0" as *const u8 as *const libc::c_char,
                    if ((*col).name).is_null() {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        (*col).name as *const libc::c_char
                    },
                );
            } else {
                _glp_format(
                    fp,
                    b"%s\n%20s\0" as *const u8 as *const libc::c_char,
                    (*col).name,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
            _glp_format(
                fp,
                b"%s  \0" as *const u8 as *const libc::c_char,
                if (*col).kind == 1 as libc::c_int {
                    b" \0" as *const u8 as *const libc::c_char
                } else if (*col).kind == 2 as libc::c_int {
                    b"*\0" as *const u8 as *const libc::c_char
                } else {
                    b"?\0" as *const u8 as *const libc::c_char
                },
            );
            _glp_format(
                fp,
                b"%13.6g \0" as *const u8 as *const libc::c_char,
                if fabs((*col).mipx) <= 1e-9f64 { 0.0f64 } else { (*col).mipx },
            );
            if (*col).type_0 == 2 as libc::c_int || (*col).type_0 == 4 as libc::c_int
                || (*col).type_0 == 5 as libc::c_int
            {
                _glp_format(
                    fp,
                    b"%13.6g \0" as *const u8 as *const libc::c_char,
                    (*col).lb,
                );
            } else {
                _glp_format(
                    fp,
                    b"%13s \0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
            if (*col).type_0 == 3 as libc::c_int || (*col).type_0 == 4 as libc::c_int {
                _glp_format(
                    fp,
                    b"%13.6g \0" as *const u8 as *const libc::c_char,
                    (*col).ub,
                );
            } else {
                _glp_format(
                    fp,
                    b"%13s \0" as *const u8 as *const libc::c_char,
                    if (*col).type_0 == 5 as libc::c_int {
                        b"=\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                );
            }
            _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
            j += 1;
            j;
        }
        _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
        _glp_format(
            fp,
            b"Integer feasibility conditions:\n\0" as *const u8 as *const libc::c_char,
        );
        _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
        glp_check_kkt(
            P,
            3 as libc::c_int,
            1 as libc::c_int,
            &mut ae_max,
            &mut ae_ind,
            &mut re_max,
            &mut re_ind,
        );
        _glp_format(
            fp,
            b"KKT.PE: max.abs.err = %.2e on row %d\n\0" as *const u8
                as *const libc::c_char,
            ae_max,
            ae_ind,
        );
        _glp_format(
            fp,
            b"        max.rel.err = %.2e on row %d\n\0" as *const u8
                as *const libc::c_char,
            re_max,
            re_ind,
        );
        _glp_format(
            fp,
            b"%8s%s\n\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            if re_max <= 1e-9f64 {
                b"High quality\0" as *const u8 as *const libc::c_char
            } else if re_max <= 1e-6f64 {
                b"Medium quality\0" as *const u8 as *const libc::c_char
            } else if re_max <= 1e-3f64 {
                b"Low quality\0" as *const u8 as *const libc::c_char
            } else {
                b"SOLUTION IS WRONG\0" as *const u8 as *const libc::c_char
            },
        );
        _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
        glp_check_kkt(
            P,
            3 as libc::c_int,
            2 as libc::c_int,
            &mut ae_max,
            &mut ae_ind,
            &mut re_max,
            &mut re_ind,
        );
        _glp_format(
            fp,
            b"KKT.PB: max.abs.err = %.2e on %s %d\n\0" as *const u8
                as *const libc::c_char,
            ae_max,
            if ae_ind <= (*P).m {
                b"row\0" as *const u8 as *const libc::c_char
            } else {
                b"column\0" as *const u8 as *const libc::c_char
            },
            if ae_ind <= (*P).m { ae_ind } else { ae_ind - (*P).m },
        );
        _glp_format(
            fp,
            b"        max.rel.err = %.2e on %s %d\n\0" as *const u8
                as *const libc::c_char,
            re_max,
            if re_ind <= (*P).m {
                b"row\0" as *const u8 as *const libc::c_char
            } else {
                b"column\0" as *const u8 as *const libc::c_char
            },
            if re_ind <= (*P).m { re_ind } else { re_ind - (*P).m },
        );
        _glp_format(
            fp,
            b"%8s%s\n\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            if re_max <= 1e-9f64 {
                b"High quality\0" as *const u8 as *const libc::c_char
            } else if re_max <= 1e-6f64 {
                b"Medium quality\0" as *const u8 as *const libc::c_char
            } else if re_max <= 1e-3f64 {
                b"Low quality\0" as *const u8 as *const libc::c_char
            } else {
                b"SOLUTION IS INFEASIBLE\0" as *const u8 as *const libc::c_char
            },
        );
        _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
        _glp_format(fp, b"End of output\n\0" as *const u8 as *const libc::c_char);
        if _glp_ioerr(fp) != 0 {
            glp_printf(
                b"Write error on '%s' - %s\n\0" as *const u8 as *const libc::c_char,
                fname,
                _glp_get_err_msg(),
            );
            ret = 1 as libc::c_int;
        } else {
            ret = 0 as libc::c_int;
        }
    }
    if !fp.is_null() {
        _glp_close(fp);
    }
    return ret;
}
