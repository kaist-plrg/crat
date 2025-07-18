use ::libc;
extern "C" {
    pub type glp_file;
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn _glp_close(f: *mut glp_file) -> libc::c_int;
    fn _glp_format(f: *mut glp_file, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn _glp_ioerr(f: *mut glp_file) -> libc::c_int;
    fn _glp_open(name: *const libc::c_char, mode: *const libc::c_char) -> *mut glp_file;
    fn _glp_get_err_msg() -> *const libc::c_char;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn glp_check_cnfsat(P: *mut glp_prob) -> libc::c_int;
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
pub unsafe extern "C" fn glp_write_cnfsat(
    mut P: *mut glp_prob,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut fp: *mut glp_file = 0 as *mut glp_file;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0;
    let mut s: [libc::c_char; 50] = [0; 50];
    if glp_check_cnfsat(P) != 0 as libc::c_int {
        glp_printf(
            b"glp_write_cnfsat: problem object does not encode CNF-SAT instance\n\0"
                as *const u8 as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
    } else {
        glp_printf(
            b"Writing CNF-SAT problem data to '%s'...\n\0" as *const u8
                as *const libc::c_char,
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
                b"c %s\n\0" as *const u8 as *const libc::c_char,
                (if ((*P).name).is_null() {
                    b"unknown\0" as *const u8 as *const libc::c_char
                } else {
                    (*P).name as *const libc::c_char
                }),
            );
            count += 1;
            count;
            _glp_format(
                fp,
                b"p cnf %d %d\n\0" as *const u8 as *const libc::c_char,
                (*P).n,
                (*P).m,
            );
            count += 1;
            count;
            i = 1 as libc::c_int;
            while i <= (*P).m {
                len = 0 as libc::c_int;
                aij = (**((*P).row).offset(i as isize)).ptr;
                while !aij.is_null() {
                    j = (*(*aij).col).j;
                    if (*aij).val < 0.0f64 {
                        j = -j;
                    }
                    sprintf(
                        s.as_mut_ptr(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        j,
                    );
                    if len > 0 as libc::c_int
                        && ((len + 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_add(strlen(s.as_mut_ptr()))
                            > 72 as libc::c_int as libc::c_ulong
                    {
                        _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
                        count += 1;
                        count;
                        len = 0 as libc::c_int;
                    }
                    _glp_format(
                        fp,
                        b"%s%s\0" as *const u8 as *const libc::c_char,
                        if len == 0 as libc::c_int {
                            b"\0" as *const u8 as *const libc::c_char
                        } else {
                            b" \0" as *const u8 as *const libc::c_char
                        },
                        s.as_mut_ptr(),
                    );
                    if len > 0 as libc::c_int {
                        len += 1;
                        len;
                    }
                    len = (len as libc::c_ulong).wrapping_add(strlen(s.as_mut_ptr()))
                        as libc::c_int as libc::c_int;
                    aij = (*aij).r_next;
                }
                if len > 0 as libc::c_int
                    && len + 1 as libc::c_int + 1 as libc::c_int > 72 as libc::c_int
                {
                    _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
                    count += 1;
                    count;
                    len = 0 as libc::c_int;
                }
                _glp_format(
                    fp,
                    b"%s0\n\0" as *const u8 as *const libc::c_char,
                    (if len == 0 as libc::c_int {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b" \0" as *const u8 as *const libc::c_char
                    }),
                );
                count += 1;
                count;
                i += 1;
                i;
            }
            _glp_format(fp, b"c eof\n\0" as *const u8 as *const libc::c_char);
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
    }
    if !fp.is_null() {
        _glp_close(fp);
    }
    return ret;
}
