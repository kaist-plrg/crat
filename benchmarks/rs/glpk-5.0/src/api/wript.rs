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
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
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
pub unsafe extern "C" fn glp_write_ipt(
    mut P: *mut glp_prob,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut fp: *mut glp_file = 0 as *mut glp_file;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if fname.is_null() {
        (glp_error_(
            b"api/wript.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int,
        ))
            .unwrap()(
            b"glp_write_ipt: fname = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            fname,
        );
    }
    glp_printf(
        b"Writing interior-point solution to '%s'...\n\0" as *const u8
            as *const libc::c_char,
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
    } else {
        _glp_format(
            fp,
            b"c %-12s%s\n\0" as *const u8 as *const libc::c_char,
            b"Problem:\0" as *const u8 as *const libc::c_char,
            (if ((*P).name).is_null() {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                (*P).name as *const libc::c_char
            }),
        );
        count += 1;
        count;
        _glp_format(
            fp,
            b"c %-12s%d\n\0" as *const u8 as *const libc::c_char,
            b"Rows:\0" as *const u8 as *const libc::c_char,
            (*P).m,
        );
        count += 1;
        count;
        _glp_format(
            fp,
            b"c %-12s%d\n\0" as *const u8 as *const libc::c_char,
            b"Columns:\0" as *const u8 as *const libc::c_char,
            (*P).n,
        );
        count += 1;
        count;
        _glp_format(
            fp,
            b"c %-12s%d\n\0" as *const u8 as *const libc::c_char,
            b"Non-zeros:\0" as *const u8 as *const libc::c_char,
            (*P).nnz,
        );
        count += 1;
        count;
        match (*P).ipt_stat {
            5 => {
                s = b"OPTIMAL\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            3 => {
                s = b"INFEASIBLE (INTERMEDIATE)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            4 => {
                s = b"INFEASIBLE (FINAL)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            1 => {
                s = b"UNDEFINED\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            _ => {
                s = b"???\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
        }
        _glp_format(
            fp,
            b"c %-12s%s\n\0" as *const u8 as *const libc::c_char,
            b"Status:\0" as *const u8 as *const libc::c_char,
            s,
        );
        count += 1;
        count;
        match (*P).dir {
            1 => {
                s = b"MINimum\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            2 => {
                s = b"MAXimum\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            _ => {
                s = b"???\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
        }
        _glp_format(
            fp,
            b"c %-12s%s%s%.10g (%s)\n\0" as *const u8 as *const libc::c_char,
            b"Objective:\0" as *const u8 as *const libc::c_char,
            (if ((*P).obj).is_null() {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                (*P).obj as *const libc::c_char
            }),
            (if ((*P).obj).is_null() {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b" = \0" as *const u8 as *const libc::c_char
            }),
            (*P).ipt_obj,
            s,
        );
        count += 1;
        count;
        _glp_format(fp, b"c\n\0" as *const u8 as *const libc::c_char);
        count += 1;
        count;
        _glp_format(
            fp,
            b"s ipt %d %d \0" as *const u8 as *const libc::c_char,
            (*P).m,
            (*P).n,
        );
        count += 1;
        count;
        match (*P).ipt_stat {
            5 => {
                _glp_format(fp, b"o\0" as *const u8 as *const libc::c_char);
            }
            3 => {
                _glp_format(fp, b"i\0" as *const u8 as *const libc::c_char);
            }
            4 => {
                _glp_format(fp, b"n\0" as *const u8 as *const libc::c_char);
            }
            1 => {
                _glp_format(fp, b"u\0" as *const u8 as *const libc::c_char);
            }
            _ => {
                _glp_format(fp, b"?\0" as *const u8 as *const libc::c_char);
            }
        }
        _glp_format(
            fp,
            b" %.*g\n\0" as *const u8 as *const libc::c_char,
            15 as libc::c_int,
            (*P).ipt_obj,
        );
        i = 1 as libc::c_int;
        while i <= (*P).m {
            row = *((*P).row).offset(i as isize);
            _glp_format(
                fp,
                b"i %d %.*g %.*g\n\0" as *const u8 as *const libc::c_char,
                i,
                15 as libc::c_int,
                (*row).pval,
                15 as libc::c_int,
                (*row).dval,
            );
            count += 1;
            count;
            i += 1;
            i;
        }
        j = 1 as libc::c_int;
        while j <= (*P).n {
            col = *((*P).col).offset(j as isize);
            _glp_format(
                fp,
                b"j %d %.*g %.*g\n\0" as *const u8 as *const libc::c_char,
                j,
                15 as libc::c_int,
                (*col).pval,
                15 as libc::c_int,
                (*col).dval,
            );
            count += 1;
            count;
            j += 1;
            j;
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
