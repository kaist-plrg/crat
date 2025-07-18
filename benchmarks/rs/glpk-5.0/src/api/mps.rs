use ::libc;
extern "C" {
    pub type glp_file;
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_vprintf(fmt: *const libc::c_char, arg: ::std::ffi::VaList);
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn _glp_get_err_msg() -> *const libc::c_char;
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn _glp_open(name: *const libc::c_char, mode: *const libc::c_char) -> *mut glp_file;
    fn _glp_ioerr(f: *mut glp_file) -> libc::c_int;
    fn _glp_getc(f: *mut glp_file) -> libc::c_int;
    fn _glp_format(f: *mut glp_file, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn _glp_close(f: *mut glp_file) -> libc::c_int;
    fn _glp_str2num(str: *const libc::c_char, val: *mut libc::c_double) -> libc::c_int;
    fn _glp_strspx(str: *mut libc::c_char) -> *mut libc::c_char;
    fn _glp_strtrim(str: *mut libc::c_char) -> *mut libc::c_char;
    fn glp_set_prob_name(P: *mut glp_prob, name: *const libc::c_char);
    fn glp_set_obj_name(P: *mut glp_prob, name: *const libc::c_char);
    fn glp_add_rows(P: *mut glp_prob, nrs: libc::c_int) -> libc::c_int;
    fn glp_add_cols(P: *mut glp_prob, ncs: libc::c_int) -> libc::c_int;
    fn glp_set_row_name(P: *mut glp_prob, i: libc::c_int, name: *const libc::c_char);
    fn glp_set_col_name(P: *mut glp_prob, j: libc::c_int, name: *const libc::c_char);
    fn glp_set_row_bnds(
        P: *mut glp_prob,
        i: libc::c_int,
        type_0: libc::c_int,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_set_col_bnds(
        P: *mut glp_prob,
        j: libc::c_int,
        type_0: libc::c_int,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_set_obj_coef(P: *mut glp_prob, j: libc::c_int, coef: libc::c_double);
    fn glp_set_mat_col(
        P: *mut glp_prob,
        j: libc::c_int,
        len: libc::c_int,
        ind: *const libc::c_int,
        val: *const libc::c_double,
    );
    fn glp_sort_matrix(P: *mut glp_prob);
    fn glp_del_rows(P: *mut glp_prob, nrs: libc::c_int, num: *const libc::c_int);
    fn glp_erase_prob(P: *mut glp_prob);
    fn glp_create_index(P: *mut glp_prob);
    fn glp_find_row(P: *mut glp_prob, name: *const libc::c_char) -> libc::c_int;
    fn glp_find_col(P: *mut glp_prob, name: *const libc::c_char) -> libc::c_int;
    fn glp_delete_index(P: *mut glp_prob);
    fn glp_set_col_kind(P: *mut glp_prob, j: libc::c_int, kind: libc::c_int);
    fn glp_get_num_int(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_num_bin(P: *mut glp_prob) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type va_list = __builtin_va_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_mpscp {
    pub blank: libc::c_int,
    pub obj_name: *mut libc::c_char,
    pub tol_mps: libc::c_double,
    pub foo_bar: [libc::c_double; 17],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csa {
    pub P: *mut glp_prob,
    pub deck: libc::c_int,
    pub parm: *const glp_mpscp,
    pub fname: *const libc::c_char,
    pub fp: *mut glp_file,
    pub jump: jmp_buf,
    pub recno: libc::c_int,
    pub recpos: libc::c_int,
    pub c: libc::c_int,
    pub fldno: libc::c_int,
    pub field: [libc::c_char; 256],
    pub w80: libc::c_int,
    pub wef: libc::c_int,
    pub obj_row: libc::c_int,
    pub work1: *mut libc::c_void,
    pub work2: *mut libc::c_void,
    pub work3: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csa1 {
    pub P: *mut glp_prob,
    pub deck: libc::c_int,
    pub parm: *const glp_mpscp,
    pub field: [libc::c_char; 256],
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub unsafe extern "C" fn glp_init_mpscp(mut parm: *mut glp_mpscp) {
    (*parm).blank = '\0' as i32;
    (*parm).obj_name = 0 as *mut libc::c_char;
    (*parm).tol_mps = 1e-12f64;
}
unsafe extern "C" fn check_parm(
    mut func: *const libc::c_char,
    mut parm: *const glp_mpscp,
) {
    if !(0 as libc::c_int <= (*parm).blank && (*parm).blank <= 0xff as libc::c_int)
        || !((*parm).blank == '\0' as i32
            || *(*__ctype_b_loc()).offset((*parm).blank as isize) as libc::c_int
                & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0)
    {
        (glp_error_(
            b"api/mps.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
        ))
            .unwrap()(
            b"%s: blank = 0x%02X; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            func,
            (*parm).blank,
        );
    }
    if !(((*parm).obj_name).is_null()
        || strlen((*parm).obj_name) <= 255 as libc::c_int as libc::c_ulong)
    {
        (glp_error_(
            b"api/mps.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int,
        ))
            .unwrap()(
            b"%s: obj_name = \"%.12s...\"; parameter too long\n\0" as *const u8
                as *const libc::c_char,
            func,
            (*parm).obj_name,
        );
    }
    if !(0.0f64 <= (*parm).tol_mps && (*parm).tol_mps < 1.0f64) {
        (glp_error_(
            b"api/mps.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
        ))
            .unwrap()(
            b"%s: tol_mps = %g; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            func,
            (*parm).tol_mps,
        );
    }
}
unsafe extern "C" fn error(
    mut csa_0: *mut csa,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut arg: ::std::ffi::VaListImpl;
    glp_printf(
        b"%s:%d: \0" as *const u8 as *const libc::c_char,
        (*csa_0).fname,
        (*csa_0).recno,
    );
    arg = args.clone();
    glp_vprintf(fmt, arg.as_va_list());
    longjmp(((*csa_0).jump).as_mut_ptr(), 1 as libc::c_int);
}
unsafe extern "C" fn warning(
    mut csa_0: *mut csa,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut arg: ::std::ffi::VaListImpl;
    glp_printf(
        b"%s:%d: warning: \0" as *const u8 as *const libc::c_char,
        (*csa_0).fname,
        (*csa_0).recno,
    );
    arg = args.clone();
    glp_vprintf(fmt, arg.as_va_list());
}
unsafe extern "C" fn read_char(mut csa_0: *mut csa) {
    let mut current_block: u64;
    let mut c: libc::c_int = 0;
    if (*csa_0).c == '\n' as i32 {
        (*csa_0).recno += 1;
        (*csa_0).recno;
        (*csa_0).recpos = 0 as libc::c_int;
    }
    (*csa_0).recpos += 1;
    (*csa_0).recpos;
    loop {
        c = _glp_getc((*csa_0).fp);
        if c < 0 as libc::c_int {
            if _glp_ioerr((*csa_0).fp) != 0 {
                error(
                    csa_0,
                    b"read error - %s\n\0" as *const u8 as *const libc::c_char,
                    _glp_get_err_msg(),
                );
            } else if (*csa_0).c == '\n' as i32 {
                error(
                    csa_0,
                    b"unexpected end of file\n\0" as *const u8 as *const libc::c_char,
                );
            } else {
                warning(
                    csa_0,
                    b"missing final end of line\n\0" as *const u8 as *const libc::c_char,
                );
                c = '\n' as i32;
            }
            current_block = 11194104282611034094;
            break;
        } else {
            if c == '\n' as i32 {
                current_block = 11194104282611034094;
                break;
            }
            if (*csa_0).c == '\r' as i32 {
                c = '\r' as i32;
                current_block = 7911617268370197842;
                break;
            } else if (*csa_0).deck != 0 && c == '\r' as i32 {
                (*csa_0).c = '\r' as i32;
            } else if c == ' ' as i32 {
                current_block = 11194104282611034094;
                break;
            } else {
                current_block = 13056961889198038528;
                break;
            }
        }
    }
    match current_block {
        13056961889198038528 => {
            if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                if (*csa_0).deck != 0 {
                    current_block = 7911617268370197842;
                } else {
                    current_block = 2370887241019905314;
                }
            } else {
                if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                    & _IScntrl as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    error(
                        csa_0,
                        b"invalid control character 0x%02X\n\0" as *const u8
                            as *const libc::c_char,
                        c,
                    );
                }
                current_block = 11194104282611034094;
            }
        }
        _ => {}
    }
    match current_block {
        7911617268370197842 => {
            error(
                csa_0,
                b"in fixed MPS format white-space character 0x%02X is not allowed\n\0"
                    as *const u8 as *const libc::c_char,
                c,
            );
            current_block = 2370887241019905314;
        }
        _ => {}
    }
    match current_block {
        2370887241019905314 => {
            c = ' ' as i32;
        }
        _ => {}
    }
    if (*csa_0).deck != 0 && (*csa_0).recpos == 81 as libc::c_int && c != '\n' as i32
        && (*csa_0).w80 < 1 as libc::c_int
    {
        warning(
            csa_0,
            b"in fixed MPS format record must not be longer than 80 characters\n\0"
                as *const u8 as *const libc::c_char,
        );
        (*csa_0).w80 += 1;
        (*csa_0).w80;
    }
    (*csa_0).c = c;
}
unsafe extern "C" fn indicator(
    mut csa_0: *mut csa,
    mut name: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    (*csa_0).fldno = 0 as libc::c_int;
    loop {
        ((*csa_0).c == '\n' as i32
            || {
                glp_assert_(
                    b"csa->c == '\\n'\0" as *const u8 as *const libc::c_char,
                    b"api/mps.c\0" as *const u8 as *const libc::c_char,
                    210 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        read_char(csa_0);
        if (*csa_0).c == ' ' as i32 || (*csa_0).c == '\n' as i32 {
            ret = 0 as libc::c_int;
            break;
        } else if (*csa_0).c == '*' as i32 {
            while (*csa_0).c != '\n' as i32 {
                read_char(csa_0);
            }
        } else {
            let mut len: libc::c_int = 0 as libc::c_int;
            while (*csa_0).c != ' ' as i32 && (*csa_0).c != '\n' as i32
                && len < 12 as libc::c_int
            {
                let fresh0 = len;
                len = len + 1;
                (*csa_0).field[fresh0 as usize] = (*csa_0).c as libc::c_char;
                read_char(csa_0);
            }
            (*csa_0).field[len as usize] = '\0' as i32 as libc::c_char;
            if !(strcmp(
                ((*csa_0).field).as_mut_ptr(),
                b"NAME\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    ((*csa_0).field).as_mut_ptr(),
                    b"ROWS\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    ((*csa_0).field).as_mut_ptr(),
                    b"COLUMNS\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    ((*csa_0).field).as_mut_ptr(),
                    b"RHS\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    ((*csa_0).field).as_mut_ptr(),
                    b"RANGES\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    ((*csa_0).field).as_mut_ptr(),
                    b"BOUNDS\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    ((*csa_0).field).as_mut_ptr(),
                    b"ENDATA\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int)
            {
                error(
                    csa_0,
                    b"invalid indicator record\n\0" as *const u8 as *const libc::c_char,
                );
            }
            if name == 0 {
                while (*csa_0).c != '\n' as i32 {
                    read_char(csa_0);
                }
            }
            ret = 1 as libc::c_int;
            break;
        }
    }
    return ret;
}
unsafe extern "C" fn read_field(mut csa_0: *mut csa) {
    (*csa_0).fldno += 1;
    (*csa_0).fldno;
    if (*csa_0).deck != 0 {
        let mut beg: libc::c_int = 0;
        let mut end: libc::c_int = 0;
        let mut pos: libc::c_int = 0;
        if (*csa_0).fldno == 1 as libc::c_int {
            beg = 2 as libc::c_int;
            end = 3 as libc::c_int;
        } else if (*csa_0).fldno == 2 as libc::c_int {
            beg = 5 as libc::c_int;
            end = 12 as libc::c_int;
        } else if (*csa_0).fldno == 3 as libc::c_int {
            beg = 15 as libc::c_int;
            end = 22 as libc::c_int;
        } else if (*csa_0).fldno == 4 as libc::c_int {
            beg = 25 as libc::c_int;
            end = 36 as libc::c_int;
        } else if (*csa_0).fldno == 5 as libc::c_int {
            beg = 40 as libc::c_int;
            end = 47 as libc::c_int;
        } else if (*csa_0).fldno == 6 as libc::c_int {
            beg = 50 as libc::c_int;
            end = 61 as libc::c_int;
        } else {
            (csa_0 != csa_0
                || {
                    glp_assert_(
                        b"csa != csa\0" as *const u8 as *const libc::c_char,
                        b"api/mps.c\0" as *const u8 as *const libc::c_char,
                        267 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        if (*csa_0).c != '\n' as i32 {
            pos = (*csa_0).recpos;
            while (*csa_0).recpos < beg {
                if !((*csa_0).c == ' ' as i32) {
                    if (*csa_0).c == '\n' as i32 {
                        break;
                    }
                    error(
                        csa_0,
                        b"in fixed MPS format positions %d-%d must be blank\n\0"
                            as *const u8 as *const libc::c_char,
                        pos,
                        beg - 1 as libc::c_int,
                    );
                }
                read_char(csa_0);
            }
        }
        if ((*csa_0).fldno == 3 as libc::c_int || (*csa_0).fldno == 5 as libc::c_int)
            && (*csa_0).c == '$' as i32
        {
            while (*csa_0).c != '\n' as i32 {
                read_char(csa_0);
            }
        }
        pos = beg;
        while pos <= end {
            if (*csa_0).c == '\n' as i32 {
                break;
            }
            (*csa_0).field[(pos - beg) as usize] = (*csa_0).c as libc::c_char;
            read_char(csa_0);
            pos += 1;
            pos;
        }
        (*csa_0).field[(pos - beg) as usize] = '\0' as i32 as libc::c_char;
        _glp_strtrim(((*csa_0).field).as_mut_ptr());
        if (*csa_0).fldno == 6 as libc::c_int && (*csa_0).c != '\n' as i32 {
            while (*csa_0).recpos <= 72 as libc::c_int {
                if !((*csa_0).c == ' ' as i32) {
                    if (*csa_0).c == '\n' as i32 {
                        break;
                    }
                    error(
                        csa_0,
                        b"in fixed MPS format positions 62-72 must be blank\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                read_char(csa_0);
            }
            while (*csa_0).c != '\n' as i32 {
                read_char(csa_0);
            }
        }
    } else {
        let mut len: libc::c_int = 0;
        while (*csa_0).c == ' ' as i32 {
            read_char(csa_0);
        }
        if (*csa_0).c == '$' as i32 {
            while (*csa_0).c != '\n' as i32 {
                read_char(csa_0);
            }
        }
        len = 0 as libc::c_int;
        while !((*csa_0).c == ' ' as i32 || (*csa_0).c == '\n' as i32) {
            if len == 255 as libc::c_int {
                let fresh1 = (*csa_0).fldno;
                (*csa_0).fldno = (*csa_0).fldno + 1;
                error(
                    csa_0,
                    b"length of field %d exceeds 255 characters\n\0" as *const u8
                        as *const libc::c_char,
                    fresh1,
                );
            }
            let fresh2 = len;
            len = len + 1;
            (*csa_0).field[fresh2 as usize] = (*csa_0).c as libc::c_char;
            read_char(csa_0);
        }
        (*csa_0).field[len as usize] = '\0' as i32 as libc::c_char;
        if (*csa_0).fldno == 6 as libc::c_int {
            while (*csa_0).c == ' ' as i32 {
                read_char(csa_0);
            }
            if (*csa_0).c != '$' as i32 && (*csa_0).c != '\n' as i32
                && (*csa_0).wef < 1 as libc::c_int
            {
                warning(
                    csa_0,
                    b"some extra field(s) detected beyond field 6; field(s) ignored\n\0"
                        as *const u8 as *const libc::c_char,
                );
                (*csa_0).wef += 1;
                (*csa_0).wef;
            }
            while (*csa_0).c != '\n' as i32 {
                read_char(csa_0);
            }
        }
    };
}
unsafe extern "C" fn patch_name(mut csa_0: *mut csa, mut name: *mut libc::c_char) {
    let mut blank: libc::c_int = (*(*csa_0).parm).blank;
    if blank == '\0' as i32 {
        _glp_strspx(name);
    } else {
        while *name as libc::c_int != '\0' as i32 {
            if *name as libc::c_int == ' ' as i32 {
                *name = blank as libc::c_char;
            }
            name = name.offset(1);
            name;
        }
    };
}
unsafe extern "C" fn read_number(mut csa_0: *mut csa) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    read_field(csa_0);
    ((*csa_0).fldno == 4 as libc::c_int || (*csa_0).fldno == 6 as libc::c_int
        || {
            glp_assert_(
                b"csa->fldno == 4 || csa->fldno == 6\0" as *const u8
                    as *const libc::c_char,
                b"api/mps.c\0" as *const u8 as *const libc::c_char,
                370 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*csa_0).field[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
        error(
            csa_0,
            b"missing numeric value in field %d\n\0" as *const u8 as *const libc::c_char,
            (*csa_0).fldno,
        );
    }
    s = ((*csa_0).field).as_mut_ptr();
    while *s as libc::c_int == ' ' as i32 {
        s = s.offset(1);
        s;
    }
    if _glp_str2num(s, &mut x) != 0 as libc::c_int {
        error(
            csa_0,
            b"cannot convert '%s' to floating-point number\n\0" as *const u8
                as *const libc::c_char,
            s,
        );
    }
    return x;
}
unsafe extern "C" fn skip_field(mut csa_0: *mut csa) {
    read_field(csa_0);
    if (*csa_0).field[0 as libc::c_int as usize] as libc::c_int != '\0' as i32 {
        error(
            csa_0,
            b"field %d must be blank\n\0" as *const u8 as *const libc::c_char,
            (*csa_0).fldno,
        );
    }
}
unsafe extern "C" fn read_name(mut csa_0: *mut csa) {
    if !(indicator(csa_0, 1 as libc::c_int) != 0
        && strcmp(
            ((*csa_0).field).as_mut_ptr(),
            b"NAME\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int)
    {
        error(
            csa_0,
            b"missing NAME indicator record\n\0" as *const u8 as *const libc::c_char,
        );
    }
    (*csa_0).fldno = 2 as libc::c_int;
    read_field(csa_0);
    patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
    if (*csa_0).field[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
        warning(
            csa_0,
            b"missing model name in field 3\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        glp_set_prob_name((*csa_0).P, ((*csa_0).field).as_mut_ptr());
    }
    while (*csa_0).c != '\n' as i32 {
        read_char(csa_0);
    }
}
unsafe extern "C" fn read_rows(mut csa_0: *mut csa) {
    let mut i: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    while !(indicator(csa_0, 0 as libc::c_int) != 0) {
        read_field(csa_0);
        _glp_strspx(((*csa_0).field).as_mut_ptr());
        if strcmp(
            ((*csa_0).field).as_mut_ptr(),
            b"N\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            type_0 = 1 as libc::c_int;
        } else if strcmp(
            ((*csa_0).field).as_mut_ptr(),
            b"G\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            type_0 = 2 as libc::c_int;
        } else if strcmp(
            ((*csa_0).field).as_mut_ptr(),
            b"L\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            type_0 = 3 as libc::c_int;
        } else if strcmp(
            ((*csa_0).field).as_mut_ptr(),
            b"E\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            type_0 = 5 as libc::c_int;
        } else if (*csa_0).field[0 as libc::c_int as usize] as libc::c_int == '\0' as i32
        {
            error(
                csa_0,
                b"missing row type in field 1\n\0" as *const u8 as *const libc::c_char,
            );
        } else {
            error(
                csa_0,
                b"invalid row type in field 1\n\0" as *const u8 as *const libc::c_char,
            );
        }
        read_field(csa_0);
        patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
        if (*csa_0).field[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
            error(
                csa_0,
                b"missing row name in field 2\n\0" as *const u8 as *const libc::c_char,
            );
        }
        if glp_find_row((*csa_0).P, ((*csa_0).field).as_mut_ptr()) != 0 as libc::c_int {
            error(
                csa_0,
                b"row '%s' multiply specified\n\0" as *const u8 as *const libc::c_char,
                ((*csa_0).field).as_mut_ptr(),
            );
        }
        i = glp_add_rows((*csa_0).P, 1 as libc::c_int);
        glp_set_row_name((*csa_0).P, i, ((*csa_0).field).as_mut_ptr());
        glp_set_row_bnds((*csa_0).P, i, type_0, 0.0f64, 0.0f64);
        skip_field(csa_0);
        skip_field(csa_0);
        skip_field(csa_0);
        skip_field(csa_0);
    }
}
unsafe extern "C" fn read_columns(mut csa_0: *mut csa) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut kind: libc::c_int = 1 as libc::c_int;
    let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut aij: libc::c_double = 0.;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut name: [libc::c_char; 256] = [0; 256];
    let mut flag: *mut libc::c_char = 0 as *mut libc::c_char;
    ind = glp_alloc(
        1 as libc::c_int + (*(*csa_0).P).m,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*csa_0).work1 = ind as *mut libc::c_void;
    val = glp_alloc(
        1 as libc::c_int + (*(*csa_0).P).m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*csa_0).work2 = val as *mut libc::c_void;
    flag = glp_alloc(
        1 as libc::c_int + (*(*csa_0).P).m,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    (*csa_0).work3 = flag as *mut libc::c_void;
    memset(
        &mut *flag.offset(1 as libc::c_int as isize) as *mut libc::c_char
            as *mut libc::c_void,
        0 as libc::c_int,
        (*(*csa_0).P).m as libc::c_ulong,
    );
    j = 0 as libc::c_int;
    len = 0 as libc::c_int;
    while !(indicator(csa_0, 0 as libc::c_int) != 0) {
        if (*csa_0).deck != 0 {
            read_field(csa_0);
            if (*csa_0).field[0 as libc::c_int as usize] as libc::c_int != '\0' as i32 {
                error(
                    csa_0,
                    b"field 1 must be blank\n\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            (*csa_0).fldno += 1;
            (*csa_0).fldno;
        }
        read_field(csa_0);
        patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
        strcpy(name.as_mut_ptr(), ((*csa_0).field).as_mut_ptr());
        read_field(csa_0);
        patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
        if strcmp(
            ((*csa_0).field).as_mut_ptr(),
            b"'MARKER'\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            if (*csa_0).deck != 0 {
                read_field(csa_0);
                if (*csa_0).field[0 as libc::c_int as usize] as libc::c_int
                    != '\0' as i32
                {
                    error(
                        csa_0,
                        b"field 4 must be blank\n\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else {
                (*csa_0).fldno += 1;
                (*csa_0).fldno;
            }
            read_field(csa_0);
            patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
            if strcmp(
                ((*csa_0).field).as_mut_ptr(),
                b"'INTORG'\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                kind = 2 as libc::c_int;
            } else if strcmp(
                ((*csa_0).field).as_mut_ptr(),
                b"'INTEND'\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                kind = 1 as libc::c_int;
            } else if (*csa_0).field[0 as libc::c_int as usize] as libc::c_int
                == '\0' as i32
            {
                error(
                    csa_0,
                    b"missing keyword in field 5\n\0" as *const u8 as *const libc::c_char,
                );
            } else {
                error(
                    csa_0,
                    b"invalid keyword in field 5\n\0" as *const u8 as *const libc::c_char,
                );
            }
            skip_field(csa_0);
        } else {
            if name[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
                if j == 0 as libc::c_int {
                    error(
                        csa_0,
                        b"missing column name in field 2\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else if j != 0 as libc::c_int
                && strcmp(
                    name.as_mut_ptr(),
                    (**((*(*csa_0).P).col).offset(j as isize)).name,
                ) == 0 as libc::c_int
            {
                (j != 0 as libc::c_int
                    || {
                        glp_assert_(
                            b"j != 0\0" as *const u8 as *const libc::c_char,
                            b"api/mps.c\0" as *const u8 as *const libc::c_char,
                            503 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            } else {
                if j != 0 as libc::c_int {
                    glp_set_mat_col(
                        (*csa_0).P,
                        j,
                        len,
                        ind as *const libc::c_int,
                        val as *const libc::c_double,
                    );
                    while len > 0 as libc::c_int {
                        let fresh3 = len;
                        len = len - 1;
                        *flag
                            .offset(
                                *ind.offset(fresh3 as isize) as isize,
                            ) = 0 as libc::c_int as libc::c_char;
                    }
                }
                if glp_find_col((*csa_0).P, name.as_mut_ptr()) != 0 as libc::c_int {
                    error(
                        csa_0,
                        b"column '%s' multiply specified\n\0" as *const u8
                            as *const libc::c_char,
                        name.as_mut_ptr(),
                    );
                }
                j = glp_add_cols((*csa_0).P, 1 as libc::c_int);
                glp_set_col_name((*csa_0).P, j, name.as_mut_ptr());
                glp_set_col_kind((*csa_0).P, j, kind);
                if kind == 1 as libc::c_int {
                    glp_set_col_bnds((*csa_0).P, j, 2 as libc::c_int, 0.0f64, 0.0f64);
                } else if kind == 2 as libc::c_int {
                    glp_set_col_bnds((*csa_0).P, j, 4 as libc::c_int, 0.0f64, 1.0f64);
                } else {
                    (kind != kind
                        || {
                            glp_assert_(
                                b"kind != kind\0" as *const u8 as *const libc::c_char,
                                b"api/mps.c\0" as *const u8 as *const libc::c_char,
                                522 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
            }
            let mut current_block_66: u64;
            f = 3 as libc::c_int;
            while f <= 5 as libc::c_int {
                if f == 3 as libc::c_int {
                    if (*csa_0).field[0 as libc::c_int as usize] as libc::c_int
                        == '\0' as i32
                    {
                        error(
                            csa_0,
                            b"missing row name in field 3\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    current_block_66 = 5892776923941496671;
                } else {
                    read_field(csa_0);
                    patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
                    if (*csa_0).field[0 as libc::c_int as usize] as libc::c_int
                        == '\0' as i32
                    {
                        skip_field(csa_0);
                        current_block_66 = 9441801433784995173;
                    } else {
                        current_block_66 = 5892776923941496671;
                    }
                }
                match current_block_66 {
                    5892776923941496671 => {
                        i = glp_find_row((*csa_0).P, ((*csa_0).field).as_mut_ptr());
                        if i == 0 as libc::c_int {
                            error(
                                csa_0,
                                b"row '%s' not found\n\0" as *const u8
                                    as *const libc::c_char,
                                ((*csa_0).field).as_mut_ptr(),
                            );
                        }
                        if *flag.offset(i as isize) != 0 {
                            error(
                                csa_0,
                                b"duplicate coefficient in row '%s'\n\0" as *const u8
                                    as *const libc::c_char,
                                ((*csa_0).field).as_mut_ptr(),
                            );
                        }
                        aij = read_number(csa_0);
                        if fabs(aij) < (*(*csa_0).parm).tol_mps {
                            aij = 0.0f64;
                        }
                        len += 1;
                        len;
                        *ind.offset(len as isize) = i;
                        *val.offset(len as isize) = aij;
                        *flag.offset(i as isize) = 1 as libc::c_int as libc::c_char;
                    }
                    _ => {}
                }
                f += 2 as libc::c_int;
            }
        }
    }
    if j != 0 as libc::c_int {
        glp_set_mat_col(
            (*csa_0).P,
            j,
            len,
            ind as *const libc::c_int,
            val as *const libc::c_double,
        );
    }
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
    glp_free(flag as *mut libc::c_void);
    (*csa_0).work3 = 0 as *mut libc::c_void;
    (*csa_0).work2 = (*csa_0).work3;
    (*csa_0).work1 = (*csa_0).work2;
}
unsafe extern "C" fn read_rhs(mut csa_0: *mut csa) {
    let mut i: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut rhs: libc::c_double = 0.;
    let mut name: [libc::c_char; 256] = [0; 256];
    let mut flag: *mut libc::c_char = 0 as *mut libc::c_char;
    flag = glp_alloc(
        1 as libc::c_int + (*(*csa_0).P).m,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    (*csa_0).work3 = flag as *mut libc::c_void;
    memset(
        &mut *flag.offset(1 as libc::c_int as isize) as *mut libc::c_char
            as *mut libc::c_void,
        0 as libc::c_int,
        (*(*csa_0).P).m as libc::c_ulong,
    );
    v = 0 as libc::c_int;
    while !(indicator(csa_0, 0 as libc::c_int) != 0) {
        if (*csa_0).deck != 0 {
            read_field(csa_0);
            if (*csa_0).field[0 as libc::c_int as usize] as libc::c_int != '\0' as i32 {
                error(
                    csa_0,
                    b"field 1 must be blank\n\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            (*csa_0).fldno += 1;
            (*csa_0).fldno;
        }
        read_field(csa_0);
        patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
        let mut current_block_17: u64;
        if (*csa_0).field[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
            if v == 0 as libc::c_int {
                warning(
                    csa_0,
                    b"missing RHS vector name in field 2\n\0" as *const u8
                        as *const libc::c_char,
                );
                current_block_17 = 17863905675527378800;
            } else {
                current_block_17 = 7175849428784450219;
            }
        } else if v != 0 as libc::c_int
            && strcmp(((*csa_0).field).as_mut_ptr(), name.as_mut_ptr())
                == 0 as libc::c_int
        {
            (v != 0 as libc::c_int
                || {
                    glp_assert_(
                        b"v != 0\0" as *const u8 as *const libc::c_char,
                        b"api/mps.c\0" as *const u8 as *const libc::c_char,
                        592 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            current_block_17 = 7175849428784450219;
        } else {
            current_block_17 = 17863905675527378800;
        }
        match current_block_17 {
            17863905675527378800 => {
                if v != 0 as libc::c_int {
                    error(
                        csa_0,
                        b"multiple RHS vectors not supported\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                v += 1;
                v;
                strcpy(name.as_mut_ptr(), ((*csa_0).field).as_mut_ptr());
            }
            _ => {}
        }
        let mut current_block_39: u64;
        f = 3 as libc::c_int;
        while f <= 5 as libc::c_int {
            read_field(csa_0);
            patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
            if (*csa_0).field[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
                if f == 3 as libc::c_int {
                    error(
                        csa_0,
                        b"missing row name in field 3\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    current_block_39 = 4068382217303356765;
                } else {
                    skip_field(csa_0);
                    current_block_39 = 11584701595673473500;
                }
            } else {
                current_block_39 = 4068382217303356765;
            }
            match current_block_39 {
                4068382217303356765 => {
                    i = glp_find_row((*csa_0).P, ((*csa_0).field).as_mut_ptr());
                    if i == 0 as libc::c_int {
                        error(
                            csa_0,
                            b"row '%s' not found\n\0" as *const u8
                                as *const libc::c_char,
                            ((*csa_0).field).as_mut_ptr(),
                        );
                    }
                    if *flag.offset(i as isize) != 0 {
                        error(
                            csa_0,
                            b"duplicate right-hand side for row '%s'\n\0" as *const u8
                                as *const libc::c_char,
                            ((*csa_0).field).as_mut_ptr(),
                        );
                    }
                    rhs = read_number(csa_0);
                    if fabs(rhs) < (*(*csa_0).parm).tol_mps {
                        rhs = 0.0f64;
                    }
                    type_0 = (**((*(*csa_0).P).row).offset(i as isize)).type_0;
                    if type_0 == 1 as libc::c_int {
                        if i == (*csa_0).obj_row {
                            glp_set_obj_coef((*csa_0).P, 0 as libc::c_int, rhs);
                        } else if rhs != 0.0f64 {
                            warning(
                                csa_0,
                                b"non-zero right-hand side for free row '%s' ignored\n\0"
                                    as *const u8 as *const libc::c_char,
                                (**((*(*csa_0).P).row).offset(i as isize)).name,
                            );
                        }
                    } else {
                        glp_set_row_bnds((*csa_0).P, i, type_0, rhs, rhs);
                    }
                    *flag.offset(i as isize) = 1 as libc::c_int as libc::c_char;
                }
                _ => {}
            }
            f += 2 as libc::c_int;
        }
    }
    glp_free(flag as *mut libc::c_void);
    (*csa_0).work3 = 0 as *mut libc::c_void;
}
unsafe extern "C" fn read_ranges(mut csa_0: *mut csa) {
    let mut i: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut rhs: libc::c_double = 0.;
    let mut rng: libc::c_double = 0.;
    let mut name: [libc::c_char; 256] = [0; 256];
    let mut flag: *mut libc::c_char = 0 as *mut libc::c_char;
    flag = glp_alloc(
        1 as libc::c_int + (*(*csa_0).P).m,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    (*csa_0).work3 = flag as *mut libc::c_void;
    memset(
        &mut *flag.offset(1 as libc::c_int as isize) as *mut libc::c_char
            as *mut libc::c_void,
        0 as libc::c_int,
        (*(*csa_0).P).m as libc::c_ulong,
    );
    v = 0 as libc::c_int;
    while !(indicator(csa_0, 0 as libc::c_int) != 0) {
        if (*csa_0).deck != 0 {
            read_field(csa_0);
            if (*csa_0).field[0 as libc::c_int as usize] as libc::c_int != '\0' as i32 {
                error(
                    csa_0,
                    b"field 1 must be blank\n\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            (*csa_0).fldno += 1;
            (*csa_0).fldno;
        }
        read_field(csa_0);
        patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
        let mut current_block_17: u64;
        if (*csa_0).field[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
            if v == 0 as libc::c_int {
                warning(
                    csa_0,
                    b"missing RANGES vector name in field 2\n\0" as *const u8
                        as *const libc::c_char,
                );
                current_block_17 = 10318958094777065518;
            } else {
                current_block_17 = 7175849428784450219;
            }
        } else if v != 0 as libc::c_int
            && strcmp(((*csa_0).field).as_mut_ptr(), name.as_mut_ptr())
                == 0 as libc::c_int
        {
            (v != 0 as libc::c_int
                || {
                    glp_assert_(
                        b"v != 0\0" as *const u8 as *const libc::c_char,
                        b"api/mps.c\0" as *const u8 as *const libc::c_char,
                        672 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            current_block_17 = 7175849428784450219;
        } else {
            current_block_17 = 10318958094777065518;
        }
        match current_block_17 {
            10318958094777065518 => {
                if v != 0 as libc::c_int {
                    error(
                        csa_0,
                        b"multiple RANGES vectors not supported\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                v += 1;
                v;
                strcpy(name.as_mut_ptr(), ((*csa_0).field).as_mut_ptr());
            }
            _ => {}
        }
        let mut current_block_50: u64;
        f = 3 as libc::c_int;
        while f <= 5 as libc::c_int {
            read_field(csa_0);
            patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
            if (*csa_0).field[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
                if f == 3 as libc::c_int {
                    error(
                        csa_0,
                        b"missing row name in field 3\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    current_block_50 = 4068382217303356765;
                } else {
                    skip_field(csa_0);
                    current_block_50 = 11584701595673473500;
                }
            } else {
                current_block_50 = 4068382217303356765;
            }
            match current_block_50 {
                4068382217303356765 => {
                    i = glp_find_row((*csa_0).P, ((*csa_0).field).as_mut_ptr());
                    if i == 0 as libc::c_int {
                        error(
                            csa_0,
                            b"row '%s' not found\n\0" as *const u8
                                as *const libc::c_char,
                            ((*csa_0).field).as_mut_ptr(),
                        );
                    }
                    if *flag.offset(i as isize) != 0 {
                        error(
                            csa_0,
                            b"duplicate range for row '%s'\n\0" as *const u8
                                as *const libc::c_char,
                            ((*csa_0).field).as_mut_ptr(),
                        );
                    }
                    rng = read_number(csa_0);
                    if fabs(rng) < (*(*csa_0).parm).tol_mps {
                        rng = 0.0f64;
                    }
                    type_0 = (**((*(*csa_0).P).row).offset(i as isize)).type_0;
                    if type_0 == 1 as libc::c_int {
                        warning(
                            csa_0,
                            b"range for free row '%s' ignored\n\0" as *const u8
                                as *const libc::c_char,
                            (**((*(*csa_0).P).row).offset(i as isize)).name,
                        );
                    } else if type_0 == 2 as libc::c_int {
                        rhs = (**((*(*csa_0).P).row).offset(i as isize)).lb;
                        glp_set_row_bnds(
                            (*csa_0).P,
                            i,
                            if rng == 0.0f64 {
                                5 as libc::c_int
                            } else {
                                4 as libc::c_int
                            },
                            rhs,
                            rhs + fabs(rng),
                        );
                    } else if type_0 == 3 as libc::c_int {
                        rhs = (**((*(*csa_0).P).row).offset(i as isize)).ub;
                        glp_set_row_bnds(
                            (*csa_0).P,
                            i,
                            if rng == 0.0f64 {
                                5 as libc::c_int
                            } else {
                                4 as libc::c_int
                            },
                            rhs - fabs(rng),
                            rhs,
                        );
                    } else if type_0 == 5 as libc::c_int {
                        rhs = (**((*(*csa_0).P).row).offset(i as isize)).lb;
                        if rng > 0.0f64 {
                            glp_set_row_bnds(
                                (*csa_0).P,
                                i,
                                4 as libc::c_int,
                                rhs,
                                rhs + rng,
                            );
                        } else if rng < 0.0f64 {
                            glp_set_row_bnds(
                                (*csa_0).P,
                                i,
                                4 as libc::c_int,
                                rhs + rng,
                                rhs,
                            );
                        }
                    } else {
                        (type_0 != type_0
                            || {
                                glp_assert_(
                                    b"type != type\0" as *const u8 as *const libc::c_char,
                                    b"api/mps.c\0" as *const u8 as *const libc::c_char,
                                    732 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                    }
                    *flag.offset(i as isize) = 1 as libc::c_int as libc::c_char;
                }
                _ => {}
            }
            f += 2 as libc::c_int;
        }
    }
    glp_free(flag as *mut libc::c_void);
    (*csa_0).work3 = 0 as *mut libc::c_void;
}
unsafe extern "C" fn read_bounds(mut csa_0: *mut csa) {
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut j: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut data: libc::c_int = 0;
    let mut bnd: libc::c_double = 0.;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    let mut type_0: [libc::c_char; 3] = [0; 3];
    let mut name: [libc::c_char; 256] = [0; 256];
    let mut flag: *mut libc::c_char = 0 as *mut libc::c_char;
    flag = glp_alloc(
        1 as libc::c_int + (*(*csa_0).P).n,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    (*csa_0).work3 = flag as *mut libc::c_void;
    memset(
        &mut *flag.offset(1 as libc::c_int as isize) as *mut libc::c_char
            as *mut libc::c_void,
        0 as libc::c_int,
        (*(*csa_0).P).n as libc::c_ulong,
    );
    v = 0 as libc::c_int;
    while !(indicator(csa_0, 0 as libc::c_int) != 0) {
        read_field(csa_0);
        if strcmp(
            ((*csa_0).field).as_mut_ptr(),
            b"LO\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            mask = 0x1 as libc::c_int;
            data = 1 as libc::c_int;
        } else if strcmp(
            ((*csa_0).field).as_mut_ptr(),
            b"UP\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            mask = 0x10 as libc::c_int;
            data = 1 as libc::c_int;
        } else if strcmp(
            ((*csa_0).field).as_mut_ptr(),
            b"FX\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            mask = 0x11 as libc::c_int;
            data = 1 as libc::c_int;
        } else if strcmp(
            ((*csa_0).field).as_mut_ptr(),
            b"FR\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            mask = 0x11 as libc::c_int;
            data = 0 as libc::c_int;
        } else if strcmp(
            ((*csa_0).field).as_mut_ptr(),
            b"MI\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            mask = 0x1 as libc::c_int;
            data = 0 as libc::c_int;
        } else if strcmp(
            ((*csa_0).field).as_mut_ptr(),
            b"PL\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            mask = 0x10 as libc::c_int;
            data = 0 as libc::c_int;
        } else if strcmp(
            ((*csa_0).field).as_mut_ptr(),
            b"LI\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            mask = 0x1 as libc::c_int;
            data = 1 as libc::c_int;
        } else if strcmp(
            ((*csa_0).field).as_mut_ptr(),
            b"UI\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            mask = 0x10 as libc::c_int;
            data = 1 as libc::c_int;
        } else if strcmp(
            ((*csa_0).field).as_mut_ptr(),
            b"BV\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            mask = 0x11 as libc::c_int;
            data = 0 as libc::c_int;
        } else if (*csa_0).field[0 as libc::c_int as usize] as libc::c_int == '\0' as i32
        {
            error(
                csa_0,
                b"missing bound type in field 1\n\0" as *const u8 as *const libc::c_char,
            );
        } else {
            error(
                csa_0,
                b"invalid bound type in field 1\n\0" as *const u8 as *const libc::c_char,
            );
        }
        strcpy(type_0.as_mut_ptr(), ((*csa_0).field).as_mut_ptr());
        read_field(csa_0);
        patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
        let mut current_block_34: u64;
        if (*csa_0).field[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
            if v == 0 as libc::c_int {
                warning(
                    csa_0,
                    b"missing BOUNDS vector name in field 2\n\0" as *const u8
                        as *const libc::c_char,
                );
                current_block_34 = 9875467767015216156;
            } else {
                current_block_34 = 9520865839495247062;
            }
        } else if v != 0 as libc::c_int
            && strcmp(((*csa_0).field).as_mut_ptr(), name.as_mut_ptr())
                == 0 as libc::c_int
        {
            (v != 0 as libc::c_int
                || {
                    glp_assert_(
                        b"v != 0\0" as *const u8 as *const libc::c_char,
                        b"api/mps.c\0" as *const u8 as *const libc::c_char,
                        790 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            current_block_34 = 9520865839495247062;
        } else {
            current_block_34 = 9875467767015216156;
        }
        match current_block_34 {
            9875467767015216156 => {
                if v != 0 as libc::c_int {
                    error(
                        csa_0,
                        b"multiple BOUNDS vectors not supported\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                v += 1;
                v;
                strcpy(name.as_mut_ptr(), ((*csa_0).field).as_mut_ptr());
            }
            _ => {}
        }
        read_field(csa_0);
        patch_name(csa_0, ((*csa_0).field).as_mut_ptr());
        if (*csa_0).field[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
            error(
                csa_0,
                b"missing column name in field 3\n\0" as *const u8 as *const libc::c_char,
            );
        }
        j = glp_find_col((*csa_0).P, ((*csa_0).field).as_mut_ptr());
        if j == 0 as libc::c_int {
            error(
                csa_0,
                b"column '%s' not found\n\0" as *const u8 as *const libc::c_char,
                ((*csa_0).field).as_mut_ptr(),
            );
        }
        if *flag.offset(j as isize) as libc::c_int & mask == 0x1 as libc::c_int {
            error(
                csa_0,
                b"duplicate lower bound for column '%s'\n\0" as *const u8
                    as *const libc::c_char,
                ((*csa_0).field).as_mut_ptr(),
            );
        }
        if *flag.offset(j as isize) as libc::c_int & mask == 0x10 as libc::c_int {
            error(
                csa_0,
                b"duplicate upper bound for column '%s'\n\0" as *const u8
                    as *const libc::c_char,
                ((*csa_0).field).as_mut_ptr(),
            );
        }
        (*flag.offset(j as isize) as libc::c_int & mask == 0 as libc::c_int
            || {
                glp_assert_(
                    b"(flag[j] & mask) == 0x00\0" as *const u8 as *const libc::c_char,
                    b"api/mps.c\0" as *const u8 as *const libc::c_char,
                    812 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if data != 0 {
            bnd = read_number(csa_0);
            if fabs(bnd) < (*(*csa_0).parm).tol_mps {
                bnd = 0.0f64;
            }
        } else {
            read_field(csa_0);
            bnd = 0.0f64;
        }
        col = *((*(*csa_0).P).col).offset(j as isize);
        if (*col).type_0 == 1 as libc::c_int {
            lb = -1.7976931348623157e+308f64;
            ub = 1.7976931348623157e+308f64;
        } else if (*col).type_0 == 2 as libc::c_int {
            lb = (*col).lb;
            ub = 1.7976931348623157e+308f64;
        } else if (*col).type_0 == 3 as libc::c_int {
            lb = -1.7976931348623157e+308f64;
            ub = (*col).ub;
        } else if (*col).type_0 == 4 as libc::c_int {
            lb = (*col).lb;
            ub = (*col).ub;
        } else if (*col).type_0 == 5 as libc::c_int {
            ub = (*col).lb;
            lb = ub;
        } else {
            (col != col
                || {
                    glp_assert_(
                        b"col != col\0" as *const u8 as *const libc::c_char,
                        b"api/mps.c\0" as *const u8 as *const libc::c_char,
                        833 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        if strcmp(type_0.as_mut_ptr(), b"LO\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            lb = bnd;
        } else if strcmp(
            type_0.as_mut_ptr(),
            b"UP\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            ub = bnd;
        } else if strcmp(
            type_0.as_mut_ptr(),
            b"FX\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            ub = bnd;
            lb = ub;
        } else if strcmp(
            type_0.as_mut_ptr(),
            b"FR\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            lb = -1.7976931348623157e+308f64;
            ub = 1.7976931348623157e+308f64;
        } else if strcmp(
            type_0.as_mut_ptr(),
            b"MI\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            lb = -1.7976931348623157e+308f64;
        } else if strcmp(
            type_0.as_mut_ptr(),
            b"PL\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            ub = 1.7976931348623157e+308f64;
        } else if strcmp(
            type_0.as_mut_ptr(),
            b"LI\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            glp_set_col_kind((*csa_0).P, j, 2 as libc::c_int);
            lb = ceil(bnd);
            if *flag.offset(j as isize) as libc::c_int & 0x10 as libc::c_int == 0 {
                ub = 1.7976931348623157e+308f64;
            }
        } else if strcmp(
            type_0.as_mut_ptr(),
            b"UI\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            glp_set_col_kind((*csa_0).P, j, 2 as libc::c_int);
            ub = floor(bnd);
        } else if strcmp(
            type_0.as_mut_ptr(),
            b"BV\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            glp_set_col_kind((*csa_0).P, j, 2 as libc::c_int);
            lb = 0.0f64;
            ub = 1.0f64;
        } else {
            (type_0.as_mut_ptr() != type_0.as_mut_ptr()
                || {
                    glp_assert_(
                        b"type != type\0" as *const u8 as *const libc::c_char,
                        b"api/mps.c\0" as *const u8 as *const libc::c_char,
                        866 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        if lb == -1.7976931348623157e+308f64 && ub == 1.7976931348623157e+308f64 {
            glp_set_col_bnds((*csa_0).P, j, 1 as libc::c_int, lb, ub);
        } else if ub == 1.7976931348623157e+308f64 {
            glp_set_col_bnds((*csa_0).P, j, 2 as libc::c_int, lb, ub);
        } else if lb == -1.7976931348623157e+308f64 {
            glp_set_col_bnds((*csa_0).P, j, 3 as libc::c_int, lb, ub);
        } else if lb != ub {
            glp_set_col_bnds((*csa_0).P, j, 4 as libc::c_int, lb, ub);
        } else {
            glp_set_col_bnds((*csa_0).P, j, 5 as libc::c_int, lb, ub);
        }
        let ref mut fresh4 = *flag.offset(j as isize);
        *fresh4 = (*fresh4 as libc::c_int | mask as libc::c_char as libc::c_int)
            as libc::c_char;
        skip_field(csa_0);
        skip_field(csa_0);
    }
    glp_free(flag as *mut libc::c_void);
    (*csa_0).work3 = 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn glp_read_mps(
    mut P: *mut glp_prob,
    mut fmt: libc::c_int,
    mut parm: *const glp_mpscp,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut _parm: glp_mpscp = glp_mpscp {
        blank: 0,
        obj_name: 0 as *mut libc::c_char,
        tol_mps: 0.,
        foo_bar: [0.; 17],
    };
    let mut _csa: csa = csa {
        P: 0 as *mut glp_prob,
        deck: 0,
        parm: 0 as *const glp_mpscp,
        fname: 0 as *const libc::c_char,
        fp: 0 as *mut glp_file,
        jump: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        recno: 0,
        recpos: 0,
        c: 0,
        fldno: 0,
        field: [0; 256],
        w80: 0,
        wef: 0,
        obj_row: 0,
        work1: 0 as *mut libc::c_void,
        work2: 0 as *mut libc::c_void,
        work3: 0 as *mut libc::c_void,
    };
    let mut csa_0: *mut csa = &mut _csa;
    let mut ret: libc::c_int = 0;
    glp_printf(
        b"Reading problem data from '%s'...\n\0" as *const u8 as *const libc::c_char,
        fname,
    );
    if !(fmt == 1 as libc::c_int || fmt == 2 as libc::c_int) {
        (glp_error_(
            b"api/mps.c\0" as *const u8 as *const libc::c_char,
            897 as libc::c_int,
        ))
            .unwrap()(
            b"glp_read_mps: fmt = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            fmt,
        );
    }
    if parm.is_null() {
        glp_init_mpscp(&mut _parm);
        parm = &mut _parm;
    }
    check_parm(b"glp_read_mps\0" as *const u8 as *const libc::c_char, parm);
    (*csa_0).P = P;
    (*csa_0).deck = (fmt == 1 as libc::c_int) as libc::c_int;
    (*csa_0).parm = parm;
    (*csa_0).fname = fname;
    (*csa_0).fp = 0 as *mut glp_file;
    if _setjmp(((*csa_0).jump).as_mut_ptr()) != 0 {
        ret = 1 as libc::c_int;
    } else {
        (*csa_0).recpos = 0 as libc::c_int;
        (*csa_0).recno = (*csa_0).recpos;
        (*csa_0).c = '\n' as i32;
        (*csa_0).fldno = 0 as libc::c_int;
        (*csa_0).field[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        (*csa_0).wef = 0 as libc::c_int;
        (*csa_0).w80 = (*csa_0).wef;
        (*csa_0).obj_row = 0 as libc::c_int;
        (*csa_0).work3 = 0 as *mut libc::c_void;
        (*csa_0).work2 = (*csa_0).work3;
        (*csa_0).work1 = (*csa_0).work2;
        glp_erase_prob(P);
        glp_create_index(P);
        (*csa_0).fp = _glp_open(fname, b"r\0" as *const u8 as *const libc::c_char);
        if ((*csa_0).fp).is_null() {
            glp_printf(
                b"Unable to open '%s' - %s\n\0" as *const u8 as *const libc::c_char,
                fname,
                _glp_get_err_msg(),
            );
            ret = 1 as libc::c_int;
        } else {
            read_name(csa_0);
            if !((*P).name).is_null() {
                glp_printf(
                    b"Problem: %s\n\0" as *const u8 as *const libc::c_char,
                    (*P).name,
                );
            }
            if !(indicator(csa_0, 0 as libc::c_int) != 0
                && strcmp(
                    ((*csa_0).field).as_mut_ptr(),
                    b"ROWS\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int)
            {
                error(
                    csa_0,
                    b"missing ROWS indicator record\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            read_rows(csa_0);
            if ((*parm).obj_name).is_null()
                || *((*parm).obj_name).offset(0 as libc::c_int as isize) as libc::c_int
                    == '\0' as i32
            {
                let mut i: libc::c_int = 0;
                i = 1 as libc::c_int;
                while i <= (*P).m {
                    if (**((*P).row).offset(i as isize)).type_0 == 1 as libc::c_int {
                        (*csa_0).obj_row = i;
                        break;
                    } else {
                        i += 1;
                        i;
                    }
                }
                if (*csa_0).obj_row == 0 as libc::c_int {
                    warning(
                        csa_0,
                        b"unable to determine objective row\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else {
                let mut i_0: libc::c_int = 0;
                i_0 = 1 as libc::c_int;
                while i_0 <= (*P).m {
                    (!((**((*P).row).offset(i_0 as isize)).name).is_null()
                        || {
                            glp_assert_(
                                b"P->row[i]->name != NULL\0" as *const u8
                                    as *const libc::c_char,
                                b"api/mps.c\0" as *const u8 as *const libc::c_char,
                                954 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    if strcmp((*parm).obj_name, (**((*P).row).offset(i_0 as isize)).name)
                        == 0 as libc::c_int
                    {
                        (*csa_0).obj_row = i_0;
                        break;
                    } else {
                        i_0 += 1;
                        i_0;
                    }
                }
                if (*csa_0).obj_row == 0 as libc::c_int {
                    error(
                        csa_0,
                        b"objective row '%s' not found\n\0" as *const u8
                            as *const libc::c_char,
                        (*parm).obj_name,
                    );
                }
            }
            if (*csa_0).obj_row != 0 as libc::c_int {
                glp_set_obj_name(
                    P,
                    (**((*P).row).offset((*csa_0).obj_row as isize)).name,
                );
                glp_printf(
                    b"Objective: %s\n\0" as *const u8 as *const libc::c_char,
                    (*P).obj,
                );
            }
            if strcmp(
                ((*csa_0).field).as_mut_ptr(),
                b"COLUMNS\0" as *const u8 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                error(
                    csa_0,
                    b"missing COLUMNS indicator record\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            read_columns(csa_0);
            if (*csa_0).obj_row != 0 as libc::c_int {
                let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
                aij = (**((*P).row).offset((*csa_0).obj_row as isize)).ptr;
                while !aij.is_null() {
                    glp_set_obj_coef(P, (*(*aij).col).j, (*aij).val);
                    aij = (*aij).r_next;
                }
            }
            if strcmp(
                ((*csa_0).field).as_mut_ptr(),
                b"RHS\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                read_rhs(csa_0);
            }
            if strcmp(
                ((*csa_0).field).as_mut_ptr(),
                b"RANGES\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                read_ranges(csa_0);
            }
            if strcmp(
                ((*csa_0).field).as_mut_ptr(),
                b"BOUNDS\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                read_bounds(csa_0);
            }
            if strcmp(
                ((*csa_0).field).as_mut_ptr(),
                b"ENDATA\0" as *const u8 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                error(
                    csa_0,
                    b"invalid use of %s indicator record\n\0" as *const u8
                        as *const libc::c_char,
                    ((*csa_0).field).as_mut_ptr(),
                );
            }
            glp_printf(
                b"%d row%s, %d column%s, %d non-zero%s\n\0" as *const u8
                    as *const libc::c_char,
                (*P).m,
                if (*P).m == 1 as libc::c_int {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    b"s\0" as *const u8 as *const libc::c_char
                },
                (*P).n,
                if (*P).n == 1 as libc::c_int {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    b"s\0" as *const u8 as *const libc::c_char
                },
                (*P).nnz,
                if (*P).nnz == 1 as libc::c_int {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    b"s\0" as *const u8 as *const libc::c_char
                },
            );
            if glp_get_num_int(P) > 0 as libc::c_int {
                let mut ni: libc::c_int = glp_get_num_int(P);
                let mut nb: libc::c_int = glp_get_num_bin(P);
                if ni == 1 as libc::c_int {
                    if nb == 0 as libc::c_int {
                        glp_printf(
                            b"One variable is integer\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        glp_printf(
                            b"One variable is binary\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                } else {
                    glp_printf(
                        b"%d integer variables, \0" as *const u8 as *const libc::c_char,
                        ni,
                    );
                    if nb == 0 as libc::c_int {
                        glp_printf(b"none\0" as *const u8 as *const libc::c_char);
                    } else if nb == 1 as libc::c_int {
                        glp_printf(b"one\0" as *const u8 as *const libc::c_char);
                    } else if nb == ni {
                        glp_printf(b"all\0" as *const u8 as *const libc::c_char);
                    } else {
                        glp_printf(b"%d\0" as *const u8 as *const libc::c_char, nb);
                    }
                    glp_printf(
                        b" of which %s binary\n\0" as *const u8 as *const libc::c_char,
                        if nb == 1 as libc::c_int {
                            b"is\0" as *const u8 as *const libc::c_char
                        } else {
                            b"are\0" as *const u8 as *const libc::c_char
                        },
                    );
                }
            }
            glp_printf(
                b"%d records were read\n\0" as *const u8 as *const libc::c_char,
                (*csa_0).recno,
            );
            let mut i_1: libc::c_int = 0;
            let mut nrs: libc::c_int = 0;
            let mut num: *mut libc::c_int = 0 as *mut libc::c_int;
            num = glp_alloc(
                1 as libc::c_int + (*P).m,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
            ) as *mut libc::c_int;
            nrs = 0 as libc::c_int;
            i_1 = 1 as libc::c_int;
            while i_1 <= (*P).m {
                if (**((*P).row).offset(i_1 as isize)).type_0 == 1 as libc::c_int {
                    nrs += 1;
                    *num.offset(nrs as isize) = i_1;
                }
                i_1 += 1;
                i_1;
            }
            if nrs > 0 as libc::c_int {
                glp_del_rows(P, nrs, num as *const libc::c_int);
                if nrs == 1 as libc::c_int {
                    glp_printf(
                        b"One free row was removed\n\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    glp_printf(
                        b"%d free rows were removed\n\0" as *const u8
                            as *const libc::c_char,
                        nrs,
                    );
                }
            }
            glp_free(num as *mut libc::c_void);
            glp_delete_index(P);
            glp_sort_matrix(P);
            ret = 0 as libc::c_int;
        }
    }
    if !((*csa_0).fp).is_null() {
        _glp_close((*csa_0).fp);
    }
    if !((*csa_0).work1).is_null() {
        glp_free((*csa_0).work1);
    }
    if !((*csa_0).work2).is_null() {
        glp_free((*csa_0).work2);
    }
    if !((*csa_0).work3).is_null() {
        glp_free((*csa_0).work3);
    }
    if ret != 0 as libc::c_int {
        glp_erase_prob(P);
    }
    return ret;
}
unsafe extern "C" fn mps_name(mut csa1: *mut csa1) -> *mut libc::c_char {
    let mut f: *mut libc::c_char = 0 as *mut libc::c_char;
    if ((*(*csa1).P).name).is_null() {
        (*csa1).field[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    } else if (*csa1).deck != 0 {
        strncpy(
            ((*csa1).field).as_mut_ptr(),
            (*(*csa1).P).name,
            8 as libc::c_int as libc::c_ulong,
        );
        (*csa1).field[8 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    } else {
        strcpy(((*csa1).field).as_mut_ptr(), (*(*csa1).P).name);
    }
    f = ((*csa1).field).as_mut_ptr();
    while *f as libc::c_int != '\0' as i32 {
        if *f as libc::c_int == ' ' as i32 {
            *f = '_' as i32 as libc::c_char;
        }
        f = f.offset(1);
        f;
    }
    return ((*csa1).field).as_mut_ptr();
}
unsafe extern "C" fn row_name(
    mut csa1: *mut csa1,
    mut i: libc::c_int,
) -> *mut libc::c_char {
    let mut f: *mut libc::c_char = 0 as *mut libc::c_char;
    (0 as libc::c_int <= i && i <= (*(*csa1).P).m
        || {
            glp_assert_(
                b"0 <= i && i <= csa->P->m\0" as *const u8 as *const libc::c_char,
                b"api/mps.c\0" as *const u8 as *const libc::c_char,
                1126 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if i == 0 as libc::c_int
        || ((**((*(*csa1).P).row).offset(i as isize)).name).is_null()
        || (*csa1).deck != 0
            && strlen((**((*(*csa1).P).row).offset(i as isize)).name)
                > 8 as libc::c_int as libc::c_ulong
    {
        sprintf(
            ((*csa1).field).as_mut_ptr(),
            b"R%07d\0" as *const u8 as *const libc::c_char,
            i,
        );
    } else {
        strcpy(
            ((*csa1).field).as_mut_ptr(),
            (**((*(*csa1).P).row).offset(i as isize)).name,
        );
        f = ((*csa1).field).as_mut_ptr();
        while *f as libc::c_int != '\0' as i32 {
            if *f as libc::c_int == ' ' as i32 {
                *f = '_' as i32 as libc::c_char;
            }
            f = f.offset(1);
            f;
        }
    }
    return ((*csa1).field).as_mut_ptr();
}
unsafe extern "C" fn col_name(
    mut csa1: *mut csa1,
    mut j: libc::c_int,
) -> *mut libc::c_char {
    let mut f: *mut libc::c_char = 0 as *mut libc::c_char;
    (1 as libc::c_int <= j && j <= (*(*csa1).P).n
        || {
            glp_assert_(
                b"1 <= j && j <= csa->P->n\0" as *const u8 as *const libc::c_char,
                b"api/mps.c\0" as *const u8 as *const libc::c_char,
                1141 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if ((**((*(*csa1).P).col).offset(j as isize)).name).is_null()
        || (*csa1).deck != 0
            && strlen((**((*(*csa1).P).col).offset(j as isize)).name)
                > 8 as libc::c_int as libc::c_ulong
    {
        sprintf(
            ((*csa1).field).as_mut_ptr(),
            b"C%07d\0" as *const u8 as *const libc::c_char,
            j,
        );
    } else {
        strcpy(
            ((*csa1).field).as_mut_ptr(),
            (**((*(*csa1).P).col).offset(j as isize)).name,
        );
        f = ((*csa1).field).as_mut_ptr();
        while *f as libc::c_int != '\0' as i32 {
            if *f as libc::c_int == ' ' as i32 {
                *f = '_' as i32 as libc::c_char;
            }
            f = f.offset(1);
            f;
        }
    }
    return ((*csa1).field).as_mut_ptr();
}
unsafe extern "C" fn mps_numb(
    mut csa1: *mut csa1,
    mut val: libc::c_double,
) -> *mut libc::c_char {
    let mut dig: libc::c_int = 0;
    let mut exp: *mut libc::c_char = 0 as *mut libc::c_char;
    dig = 12 as libc::c_int;
    while dig >= 6 as libc::c_int {
        if val != 0.0f64 && fabs(val) < 0.002f64 {
            sprintf(
                ((*csa1).field).as_mut_ptr(),
                b"%.*E\0" as *const u8 as *const libc::c_char,
                dig - 1 as libc::c_int,
                val,
            );
        } else {
            sprintf(
                ((*csa1).field).as_mut_ptr(),
                b"%.*G\0" as *const u8 as *const libc::c_char,
                dig,
                val,
            );
        }
        exp = strchr(((*csa1).field).as_mut_ptr(), 'E' as i32);
        if !exp.is_null() {
            sprintf(
                exp.offset(1 as libc::c_int as isize),
                b"%d\0" as *const u8 as *const libc::c_char,
                atoi(exp.offset(1 as libc::c_int as isize)),
            );
        }
        if strlen(((*csa1).field).as_mut_ptr()) <= 12 as libc::c_int as libc::c_ulong {
            break;
        }
        dig -= 1;
        dig;
    }
    (strlen(((*csa1).field).as_mut_ptr()) <= 12 as libc::c_int as libc::c_ulong
        || {
            glp_assert_(
                b"strlen(csa->field) <= 12\0" as *const u8 as *const libc::c_char,
                b"api/mps.c\0" as *const u8 as *const libc::c_char,
                1167 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return ((*csa1).field).as_mut_ptr();
}
pub unsafe extern "C" fn glp_write_mps(
    mut P: *mut glp_prob,
    mut fmt: libc::c_int,
    mut parm: *const glp_mpscp,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut _parm: glp_mpscp = glp_mpscp {
        blank: 0,
        obj_name: 0 as *mut libc::c_char,
        tol_mps: 0.,
        foo_bar: [0.; 17],
    };
    let mut _csa: csa1 = csa1 {
        P: 0 as *mut glp_prob,
        deck: 0,
        parm: 0 as *const glp_mpscp,
        field: [0; 256],
    };
    let mut csa1: *mut csa1 = &mut _csa;
    let mut fp: *mut glp_file = 0 as *mut glp_file;
    let mut out_obj: libc::c_int = 0;
    let mut one_col: libc::c_int = 0 as libc::c_int;
    let mut empty: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut recno: libc::c_int = 0;
    let mut marker: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut gap: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    glp_printf(
        b"Writing problem data to '%s'...\n\0" as *const u8 as *const libc::c_char,
        fname,
    );
    if !(fmt == 1 as libc::c_int || fmt == 2 as libc::c_int) {
        (glp_error_(
            b"api/mps.c\0" as *const u8 as *const libc::c_char,
            1181 as libc::c_int,
        ))
            .unwrap()(
            b"glp_write_mps: fmt = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            fmt,
        );
    }
    if parm.is_null() {
        glp_init_mpscp(&mut _parm);
        parm = &mut _parm;
    }
    check_parm(b"glp_write_mps\0" as *const u8 as *const libc::c_char, parm);
    (*csa1).P = P;
    (*csa1).deck = (fmt == 1 as libc::c_int) as libc::c_int;
    (*csa1).parm = parm;
    fp = _glp_open(fname, b"w\0" as *const u8 as *const libc::c_char);
    recno = 0 as libc::c_int;
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
            b"* %-*s%s\n\0" as *const u8 as *const libc::c_char,
            (if ((*P).name).is_null() { 1 as libc::c_int } else { 12 as libc::c_int }),
            b"Problem:\0" as *const u8 as *const libc::c_char,
            (if ((*P).name).is_null() {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                (*P).name as *const libc::c_char
            }),
        );
        recno += 1;
        recno;
        _glp_format(
            fp,
            b"* %-12s%s\n\0" as *const u8 as *const libc::c_char,
            b"Class:\0" as *const u8 as *const libc::c_char,
            (if glp_get_num_int(P) == 0 as libc::c_int {
                b"LP\0" as *const u8 as *const libc::c_char
            } else {
                b"MIP\0" as *const u8 as *const libc::c_char
            }),
        );
        recno += 1;
        recno;
        _glp_format(
            fp,
            b"* %-12s%d\n\0" as *const u8 as *const libc::c_char,
            b"Rows:\0" as *const u8 as *const libc::c_char,
            (*P).m,
        );
        recno += 1;
        recno;
        if glp_get_num_int(P) == 0 as libc::c_int {
            _glp_format(
                fp,
                b"* %-12s%d\n\0" as *const u8 as *const libc::c_char,
                b"Columns:\0" as *const u8 as *const libc::c_char,
                (*P).n,
            );
            recno += 1;
            recno;
        } else {
            _glp_format(
                fp,
                b"* %-12s%d (%d integer, %d binary)\n\0" as *const u8
                    as *const libc::c_char,
                b"Columns:\0" as *const u8 as *const libc::c_char,
                (*P).n,
                glp_get_num_int(P),
                glp_get_num_bin(P),
            );
            recno += 1;
            recno;
        }
        _glp_format(
            fp,
            b"* %-12s%d\n\0" as *const u8 as *const libc::c_char,
            b"Non-zeros:\0" as *const u8 as *const libc::c_char,
            (*P).nnz,
        );
        recno += 1;
        recno;
        _glp_format(
            fp,
            b"* %-12s%s\n\0" as *const u8 as *const libc::c_char,
            b"Format:\0" as *const u8 as *const libc::c_char,
            (if (*csa1).deck != 0 {
                b"Fixed MPS\0" as *const u8 as *const libc::c_char
            } else {
                b"Free MPS\0" as *const u8 as *const libc::c_char
            }),
        );
        recno += 1;
        recno;
        let fresh5 = recno;
        recno = recno + 1;
        _glp_format(fp, b"*\n\0" as *const u8 as *const libc::c_char, fresh5);
        _glp_format(
            fp,
            b"NAME%*s%s\n\0" as *const u8 as *const libc::c_char,
            (if ((*P).name).is_null() {
                0 as libc::c_int
            } else {
                (if (*csa1).deck != 0 { 10 as libc::c_int } else { 1 as libc::c_int })
            }),
            b"\0" as *const u8 as *const libc::c_char,
            mps_name(csa1),
        );
        recno += 1;
        recno;
        out_obj = 1 as libc::c_int;
        i = 1 as libc::c_int;
        while i <= (*P).m {
            if (**((*P).row).offset(i as isize)).type_0 == 1 as libc::c_int {
                out_obj = 0 as libc::c_int;
                break;
            } else {
                i += 1;
                i;
            }
        }
        _glp_format(fp, b"ROWS\n\0" as *const u8 as *const libc::c_char);
        recno += 1;
        recno;
        i = if out_obj != 0 { 0 as libc::c_int } else { 1 as libc::c_int };
        while i <= (*P).m {
            let mut type_0: libc::c_int = 0;
            type_0 = if i == 0 as libc::c_int {
                1 as libc::c_int
            } else {
                (**((*P).row).offset(i as isize)).type_0
            };
            if type_0 == 1 as libc::c_int {
                type_0 = 'N' as i32;
            } else if type_0 == 2 as libc::c_int {
                type_0 = 'G' as i32;
            } else if type_0 == 3 as libc::c_int {
                type_0 = 'L' as i32;
            } else if type_0 == 4 as libc::c_int || type_0 == 5 as libc::c_int {
                type_0 = 'E' as i32;
            } else {
                (type_0 != type_0
                    || {
                        glp_assert_(
                            b"type != type\0" as *const u8 as *const libc::c_char,
                            b"api/mps.c\0" as *const u8 as *const libc::c_char,
                            1241 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            _glp_format(
                fp,
                b" %c%*s%s\n\0" as *const u8 as *const libc::c_char,
                type_0,
                (if (*csa1).deck != 0 { 2 as libc::c_int } else { 1 as libc::c_int }),
                b"\0" as *const u8 as *const libc::c_char,
                row_name(csa1, i),
            );
            recno += 1;
            recno;
            i += 1;
            i;
        }
        _glp_format(fp, b"COLUMNS\n\0" as *const u8 as *const libc::c_char);
        recno += 1;
        recno;
        marker = 0 as libc::c_int;
        j = 1 as libc::c_int;
        while j <= (*P).n {
            let mut cj: GLPAIJ = GLPAIJ {
                row: 0 as *mut GLPROW,
                col: 0 as *mut GLPCOL,
                val: 0.,
                r_prev: 0 as *mut GLPAIJ,
                r_next: 0 as *mut GLPAIJ,
                c_prev: 0 as *mut GLPAIJ,
                c_next: 0 as *mut GLPAIJ,
            };
            let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
            let mut kind: libc::c_int = 0;
            kind = (**((*P).col).offset(j as isize)).kind;
            if kind == 1 as libc::c_int {
                if marker % 2 as libc::c_int == 1 as libc::c_int {
                    marker += 1;
                    marker;
                    _glp_format(
                        fp,
                        b"%*sM%07d%*s'MARKER'%*s'INTEND'\n\0" as *const u8
                            as *const libc::c_char,
                        (if (*csa1).deck != 0 {
                            4 as libc::c_int
                        } else {
                            1 as libc::c_int
                        }),
                        b"\0" as *const u8 as *const libc::c_char,
                        marker,
                        (if (*csa1).deck != 0 {
                            2 as libc::c_int
                        } else {
                            1 as libc::c_int
                        }),
                        b"\0" as *const u8 as *const libc::c_char,
                        (if (*csa1).deck != 0 {
                            17 as libc::c_int
                        } else {
                            1 as libc::c_int
                        }),
                        b"\0" as *const u8 as *const libc::c_char,
                    );
                    recno += 1;
                    recno;
                }
            } else if kind == 2 as libc::c_int {
                if marker % 2 as libc::c_int == 0 as libc::c_int {
                    marker += 1;
                    marker;
                    _glp_format(
                        fp,
                        b"%*sM%07d%*s'MARKER'%*s'INTORG'\n\0" as *const u8
                            as *const libc::c_char,
                        (if (*csa1).deck != 0 {
                            4 as libc::c_int
                        } else {
                            1 as libc::c_int
                        }),
                        b"\0" as *const u8 as *const libc::c_char,
                        marker,
                        (if (*csa1).deck != 0 {
                            2 as libc::c_int
                        } else {
                            1 as libc::c_int
                        }),
                        b"\0" as *const u8 as *const libc::c_char,
                        (if (*csa1).deck != 0 {
                            17 as libc::c_int
                        } else {
                            1 as libc::c_int
                        }),
                        b"\0" as *const u8 as *const libc::c_char,
                    );
                    recno += 1;
                    recno;
                }
            } else {
                (kind != kind
                    || {
                        glp_assert_(
                            b"kind != kind\0" as *const u8 as *const libc::c_char,
                            b"api/mps.c\0" as *const u8 as *const libc::c_char,
                            1271 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            if out_obj != 0 && (**((*P).col).offset(j as isize)).coef != 0.0f64 {
                aij = &mut cj;
                (*aij).row = 0 as *mut GLPROW;
                (*aij).val = (**((*P).col).offset(j as isize)).coef;
                (*aij).c_next = (**((*P).col).offset(j as isize)).ptr;
            } else {
                aij = (**((*P).col).offset(j as isize)).ptr;
            }
            if aij.is_null() {
                empty += 1;
                empty;
                _glp_format(
                    fp,
                    b"%*s%-*s\0" as *const u8 as *const libc::c_char,
                    if (*csa1).deck != 0 { 4 as libc::c_int } else { 1 as libc::c_int },
                    b"\0" as *const u8 as *const libc::c_char,
                    if (*csa1).deck != 0 { 8 as libc::c_int } else { 1 as libc::c_int },
                    col_name(csa1, j),
                );
                ((*P).m > 0 as libc::c_int
                    || {
                        glp_assert_(
                            b"P->m > 0\0" as *const u8 as *const libc::c_char,
                            b"api/mps.c\0" as *const u8 as *const libc::c_char,
                            1288 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                _glp_format(
                    fp,
                    b"%*s%-*s\0" as *const u8 as *const libc::c_char,
                    if (*csa1).deck != 0 { 2 as libc::c_int } else { 1 as libc::c_int },
                    b"\0" as *const u8 as *const libc::c_char,
                    if (*csa1).deck != 0 { 8 as libc::c_int } else { 1 as libc::c_int },
                    row_name(csa1, 1 as libc::c_int),
                );
                _glp_format(
                    fp,
                    b"%*s0%*s$ empty column\n\0" as *const u8 as *const libc::c_char,
                    (if (*csa1).deck != 0 {
                        13 as libc::c_int
                    } else {
                        1 as libc::c_int
                    }),
                    b"\0" as *const u8 as *const libc::c_char,
                    (if (*csa1).deck != 0 {
                        3 as libc::c_int
                    } else {
                        1 as libc::c_int
                    }),
                    b"\0" as *const u8 as *const libc::c_char,
                );
                recno += 1;
                recno;
            }
            count = 0 as libc::c_int;
            aij = aij;
            while !aij.is_null() {
                if one_col != 0 || count % 2 as libc::c_int == 0 as libc::c_int {
                    _glp_format(
                        fp,
                        b"%*s%-*s\0" as *const u8 as *const libc::c_char,
                        if (*csa1).deck != 0 {
                            4 as libc::c_int
                        } else {
                            1 as libc::c_int
                        },
                        b"\0" as *const u8 as *const libc::c_char,
                        if (*csa1).deck != 0 {
                            8 as libc::c_int
                        } else {
                            1 as libc::c_int
                        },
                        col_name(csa1, j),
                    );
                }
                gap = if one_col != 0 || count % 2 as libc::c_int == 0 as libc::c_int {
                    2 as libc::c_int
                } else {
                    3 as libc::c_int
                };
                _glp_format(
                    fp,
                    b"%*s%-*s\0" as *const u8 as *const libc::c_char,
                    if (*csa1).deck != 0 { gap } else { 1 as libc::c_int },
                    b"\0" as *const u8 as *const libc::c_char,
                    if (*csa1).deck != 0 { 8 as libc::c_int } else { 1 as libc::c_int },
                    row_name(
                        csa1,
                        if ((*aij).row).is_null() {
                            0 as libc::c_int
                        } else {
                            (*(*aij).row).i
                        },
                    ),
                );
                _glp_format(
                    fp,
                    b"%*s%*s\0" as *const u8 as *const libc::c_char,
                    (if (*csa1).deck != 0 {
                        2 as libc::c_int
                    } else {
                        1 as libc::c_int
                    }),
                    b"\0" as *const u8 as *const libc::c_char,
                    (if (*csa1).deck != 0 {
                        12 as libc::c_int
                    } else {
                        1 as libc::c_int
                    }),
                    mps_numb(csa1, (*aij).val),
                );
                count += 1;
                count;
                if one_col != 0 || count % 2 as libc::c_int == 0 as libc::c_int {
                    _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
                    recno += 1;
                    recno;
                }
                aij = (*aij).c_next;
            }
            if !(one_col != 0 || count % 2 as libc::c_int == 0 as libc::c_int) {
                _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
                recno += 1;
                recno;
            }
            j += 1;
            j;
        }
        if marker % 2 as libc::c_int == 1 as libc::c_int {
            marker += 1;
            marker;
            _glp_format(
                fp,
                b"%*sM%07d%*s'MARKER'%*s'INTEND'\n\0" as *const u8
                    as *const libc::c_char,
                (if (*csa1).deck != 0 { 4 as libc::c_int } else { 1 as libc::c_int }),
                b"\0" as *const u8 as *const libc::c_char,
                marker,
                (if (*csa1).deck != 0 { 2 as libc::c_int } else { 1 as libc::c_int }),
                b"\0" as *const u8 as *const libc::c_char,
                (if (*csa1).deck != 0 { 17 as libc::c_int } else { 1 as libc::c_int }),
                b"\0" as *const u8 as *const libc::c_char,
            );
            recno += 1;
            recno;
        }
        if empty > 0 as libc::c_int {
            glp_printf(
                b"Warning: problem has %d empty column(s)\n\0" as *const u8
                    as *const libc::c_char,
                empty,
            );
        }
        _glp_format(fp, b"RHS\n\0" as *const u8 as *const libc::c_char);
        recno += 1;
        recno;
        count = 0 as libc::c_int;
        i = if out_obj != 0 { 0 as libc::c_int } else { 1 as libc::c_int };
        while i <= (*P).m {
            let mut type_1: libc::c_int = 0;
            let mut rhs: libc::c_double = 0.;
            if i == 0 as libc::c_int {
                rhs = (*P).c0;
            } else {
                type_1 = (**((*P).row).offset(i as isize)).type_0;
                if type_1 == 1 as libc::c_int {
                    rhs = 0.0f64;
                } else if type_1 == 2 as libc::c_int {
                    rhs = (**((*P).row).offset(i as isize)).lb;
                } else if type_1 == 3 as libc::c_int {
                    rhs = (**((*P).row).offset(i as isize)).ub;
                } else if type_1 == 4 as libc::c_int || type_1 == 5 as libc::c_int {
                    rhs = (**((*P).row).offset(i as isize)).lb;
                } else {
                    (type_1 != type_1
                        || {
                            glp_assert_(
                                b"type != type\0" as *const u8 as *const libc::c_char,
                                b"api/mps.c\0" as *const u8 as *const libc::c_char,
                                1344 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
            }
            if rhs != 0.0f64 {
                if one_col != 0 || count % 2 as libc::c_int == 0 as libc::c_int {
                    _glp_format(
                        fp,
                        b"%*s%-*s\0" as *const u8 as *const libc::c_char,
                        if (*csa1).deck != 0 {
                            4 as libc::c_int
                        } else {
                            1 as libc::c_int
                        },
                        b"\0" as *const u8 as *const libc::c_char,
                        if (*csa1).deck != 0 {
                            8 as libc::c_int
                        } else {
                            1 as libc::c_int
                        },
                        b"RHS1\0" as *const u8 as *const libc::c_char,
                    );
                }
                gap = if one_col != 0 || count % 2 as libc::c_int == 0 as libc::c_int {
                    2 as libc::c_int
                } else {
                    3 as libc::c_int
                };
                _glp_format(
                    fp,
                    b"%*s%-*s\0" as *const u8 as *const libc::c_char,
                    if (*csa1).deck != 0 { gap } else { 1 as libc::c_int },
                    b"\0" as *const u8 as *const libc::c_char,
                    if (*csa1).deck != 0 { 8 as libc::c_int } else { 1 as libc::c_int },
                    row_name(csa1, i),
                );
                _glp_format(
                    fp,
                    b"%*s%*s\0" as *const u8 as *const libc::c_char,
                    (if (*csa1).deck != 0 {
                        2 as libc::c_int
                    } else {
                        1 as libc::c_int
                    }),
                    b"\0" as *const u8 as *const libc::c_char,
                    (if (*csa1).deck != 0 {
                        12 as libc::c_int
                    } else {
                        1 as libc::c_int
                    }),
                    mps_numb(csa1, rhs),
                );
                count += 1;
                count;
                if one_col != 0 || count % 2 as libc::c_int == 0 as libc::c_int {
                    _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
                    recno += 1;
                    recno;
                }
            }
            i += 1;
            i;
        }
        if !(one_col != 0 || count % 2 as libc::c_int == 0 as libc::c_int) {
            _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
            recno += 1;
            recno;
        }
        i = (*P).m;
        while i >= 1 as libc::c_int {
            if (**((*P).row).offset(i as isize)).type_0 == 4 as libc::c_int {
                break;
            }
            i -= 1;
            i;
        }
        if !(i == 0 as libc::c_int) {
            _glp_format(fp, b"RANGES\n\0" as *const u8 as *const libc::c_char);
            recno += 1;
            recno;
            count = 0 as libc::c_int;
            i = 1 as libc::c_int;
            while i <= (*P).m {
                if (**((*P).row).offset(i as isize)).type_0 == 4 as libc::c_int {
                    if one_col != 0 || count % 2 as libc::c_int == 0 as libc::c_int {
                        _glp_format(
                            fp,
                            b"%*s%-*s\0" as *const u8 as *const libc::c_char,
                            if (*csa1).deck != 0 {
                                4 as libc::c_int
                            } else {
                                1 as libc::c_int
                            },
                            b"\0" as *const u8 as *const libc::c_char,
                            if (*csa1).deck != 0 {
                                8 as libc::c_int
                            } else {
                                1 as libc::c_int
                            },
                            b"RNG1\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    gap = if one_col != 0 || count % 2 as libc::c_int == 0 as libc::c_int
                    {
                        2 as libc::c_int
                    } else {
                        3 as libc::c_int
                    };
                    _glp_format(
                        fp,
                        b"%*s%-*s\0" as *const u8 as *const libc::c_char,
                        if (*csa1).deck != 0 { gap } else { 1 as libc::c_int },
                        b"\0" as *const u8 as *const libc::c_char,
                        if (*csa1).deck != 0 {
                            8 as libc::c_int
                        } else {
                            1 as libc::c_int
                        },
                        row_name(csa1, i),
                    );
                    _glp_format(
                        fp,
                        b"%*s%*s\0" as *const u8 as *const libc::c_char,
                        (if (*csa1).deck != 0 {
                            2 as libc::c_int
                        } else {
                            1 as libc::c_int
                        }),
                        b"\0" as *const u8 as *const libc::c_char,
                        (if (*csa1).deck != 0 {
                            12 as libc::c_int
                        } else {
                            1 as libc::c_int
                        }),
                        mps_numb(
                            csa1,
                            (**((*P).row).offset(i as isize)).ub
                                - (**((*P).row).offset(i as isize)).lb,
                        ),
                    );
                    count += 1;
                    count;
                    if one_col != 0 || count % 2 as libc::c_int == 0 as libc::c_int {
                        _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
                        recno += 1;
                        recno;
                    }
                }
                i += 1;
                i;
            }
            if !(one_col != 0 || count % 2 as libc::c_int == 0 as libc::c_int) {
                _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
                recno += 1;
                recno;
            }
        }
        j = (*P).n;
        while j >= 1 as libc::c_int {
            if !((**((*P).col).offset(j as isize)).kind == 1 as libc::c_int
                && (**((*P).col).offset(j as isize)).type_0 == 2 as libc::c_int
                && (**((*P).col).offset(j as isize)).lb == 0.0f64)
            {
                break;
            }
            j -= 1;
            j;
        }
        if !(j == 0 as libc::c_int) {
            _glp_format(fp, b"BOUNDS\n\0" as *const u8 as *const libc::c_char);
            recno += 1;
            recno;
            j = 1 as libc::c_int;
            while j <= (*P).n {
                let mut type_2: libc::c_int = 0;
                let mut data: [libc::c_int; 2] = [0; 2];
                let mut bnd: [libc::c_double; 2] = [0.; 2];
                let mut spec: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
                spec[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
                spec[0 as libc::c_int as usize] = spec[1 as libc::c_int as usize];
                type_2 = (**((*P).col).offset(j as isize)).type_0;
                if type_2 == 1 as libc::c_int {
                    spec[0 as libc::c_int
                        as usize] = b"FR\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    data[0 as libc::c_int as usize] = 0 as libc::c_int;
                } else if type_2 == 2 as libc::c_int {
                    if (**((*P).col).offset(j as isize)).lb != 0.0f64 {
                        spec[0 as libc::c_int
                            as usize] = b"LO\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                        data[0 as libc::c_int as usize] = 1 as libc::c_int;
                        bnd[0 as libc::c_int
                            as usize] = (**((*P).col).offset(j as isize)).lb;
                    }
                    if (**((*P).col).offset(j as isize)).kind == 2 as libc::c_int {
                        spec[1 as libc::c_int
                            as usize] = b"PL\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                        data[1 as libc::c_int as usize] = 0 as libc::c_int;
                    }
                } else if type_2 == 3 as libc::c_int {
                    spec[0 as libc::c_int
                        as usize] = b"MI\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    data[0 as libc::c_int as usize] = 0 as libc::c_int;
                    spec[1 as libc::c_int
                        as usize] = b"UP\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    data[1 as libc::c_int as usize] = 1 as libc::c_int;
                    bnd[1 as libc::c_int
                        as usize] = (**((*P).col).offset(j as isize)).ub;
                } else if type_2 == 4 as libc::c_int {
                    if (**((*P).col).offset(j as isize)).lb != 0.0f64 {
                        spec[0 as libc::c_int
                            as usize] = b"LO\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                        data[0 as libc::c_int as usize] = 1 as libc::c_int;
                        bnd[0 as libc::c_int
                            as usize] = (**((*P).col).offset(j as isize)).lb;
                    }
                    spec[1 as libc::c_int
                        as usize] = b"UP\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    data[1 as libc::c_int as usize] = 1 as libc::c_int;
                    bnd[1 as libc::c_int
                        as usize] = (**((*P).col).offset(j as isize)).ub;
                } else if type_2 == 5 as libc::c_int {
                    spec[0 as libc::c_int
                        as usize] = b"FX\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    data[0 as libc::c_int as usize] = 1 as libc::c_int;
                    bnd[0 as libc::c_int
                        as usize] = (**((*P).col).offset(j as isize)).lb;
                } else {
                    (type_2 != type_2
                        || {
                            glp_assert_(
                                b"type != type\0" as *const u8 as *const libc::c_char,
                                b"api/mps.c\0" as *const u8 as *const libc::c_char,
                                1420 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
                i = 0 as libc::c_int;
                while i <= 1 as libc::c_int {
                    if !(spec[i as usize]).is_null() {
                        _glp_format(
                            fp,
                            b" %s %-*s%*s%-*s\0" as *const u8 as *const libc::c_char,
                            spec[i as usize],
                            if (*csa1).deck != 0 {
                                8 as libc::c_int
                            } else {
                                1 as libc::c_int
                            },
                            b"BND1\0" as *const u8 as *const libc::c_char,
                            if (*csa1).deck != 0 {
                                2 as libc::c_int
                            } else {
                                1 as libc::c_int
                            },
                            b"\0" as *const u8 as *const libc::c_char,
                            if (*csa1).deck != 0 {
                                8 as libc::c_int
                            } else {
                                1 as libc::c_int
                            },
                            col_name(csa1, j),
                        );
                        if data[i as usize] != 0 {
                            _glp_format(
                                fp,
                                b"%*s%*s\0" as *const u8 as *const libc::c_char,
                                if (*csa1).deck != 0 {
                                    2 as libc::c_int
                                } else {
                                    1 as libc::c_int
                                },
                                b"\0" as *const u8 as *const libc::c_char,
                                if (*csa1).deck != 0 {
                                    12 as libc::c_int
                                } else {
                                    1 as libc::c_int
                                },
                                mps_numb(csa1, bnd[i as usize]),
                            );
                        }
                        _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
                        recno += 1;
                        recno;
                    }
                    i += 1;
                    i;
                }
                j += 1;
                j;
            }
        }
        _glp_format(fp, b"ENDATA\n\0" as *const u8 as *const libc::c_char);
        recno += 1;
        recno;
        if _glp_ioerr(fp) != 0 {
            glp_printf(
                b"Write error on '%s' - %s\n\0" as *const u8 as *const libc::c_char,
                fname,
                _glp_get_err_msg(),
            );
            ret = 1 as libc::c_int;
        } else {
            glp_printf(
                b"%d records were written\n\0" as *const u8 as *const libc::c_char,
                recno,
            );
            ret = 0 as libc::c_int;
        }
    }
    if !fp.is_null() {
        _glp_close(fp);
    }
    return ret;
}
