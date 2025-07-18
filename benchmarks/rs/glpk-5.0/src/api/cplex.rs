use ::libc;
extern "C" {
    pub type glp_file;
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn _glp_close(f: *mut glp_file) -> libc::c_int;
    fn _glp_format(f: *mut glp_file, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn _glp_getc(f: *mut glp_file) -> libc::c_int;
    fn _glp_ioerr(f: *mut glp_file) -> libc::c_int;
    fn _glp_open(name: *const libc::c_char, mode: *const libc::c_char) -> *mut glp_file;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn _glp_get_err_msg() -> *const libc::c_char;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_vprintf(fmt: *const libc::c_char, arg: ::std::ffi::VaList);
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn _glp_str2num(str: *const libc::c_char, val: *mut libc::c_double) -> libc::c_int;
    fn glp_set_obj_name(P: *mut glp_prob, name: *const libc::c_char);
    fn glp_set_obj_dir(P: *mut glp_prob, dir: libc::c_int);
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
    fn glp_set_mat_row(
        P: *mut glp_prob,
        i: libc::c_int,
        len: libc::c_int,
        ind: *const libc::c_int,
        val: *const libc::c_double,
    );
    fn glp_sort_matrix(P: *mut glp_prob);
    fn glp_erase_prob(P: *mut glp_prob);
    fn glp_get_obj_name(P: *mut glp_prob) -> *const libc::c_char;
    fn glp_get_row_name(P: *mut glp_prob, i: libc::c_int) -> *const libc::c_char;
    fn glp_get_col_name(P: *mut glp_prob, j: libc::c_int) -> *const libc::c_char;
    fn glp_create_index(P: *mut glp_prob);
    fn glp_find_row(P: *mut glp_prob, name: *const libc::c_char) -> libc::c_int;
    fn glp_find_col(P: *mut glp_prob, name: *const libc::c_char) -> libc::c_int;
    fn glp_delete_index(P: *mut glp_prob);
    fn glp_get_num_bin(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_num_int(P: *mut glp_prob) -> libc::c_int;
    fn glp_set_col_kind(P: *mut glp_prob, j: libc::c_int, kind: libc::c_int);
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
pub type __int32_t = libc::c_int;
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
pub struct glp_cpxcp {
    pub foo_bar: [libc::c_double; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csa {
    pub P: *mut glp_prob,
    pub parm: *const glp_cpxcp,
    pub fname: *const libc::c_char,
    pub fp: *mut glp_file,
    pub jump: jmp_buf,
    pub count: libc::c_int,
    pub c: libc::c_int,
    pub token: libc::c_int,
    pub image: [libc::c_char; 256],
    pub imlen: libc::c_int,
    pub value: libc::c_double,
    pub n_max: libc::c_int,
    pub ind: *mut libc::c_int,
    pub val: *mut libc::c_double,
    pub flag: *mut libc::c_char,
    pub lb: *mut libc::c_double,
    pub ub: *mut libc::c_double,
    pub lb_warn: libc::c_int,
    pub ub_warn: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csa1 {
    pub P: *mut glp_prob,
    pub parm: *const glp_cpxcp,
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
pub unsafe extern "C" fn glp_init_cpxcp(mut parm: *mut glp_cpxcp) {
    (!parm.is_null()
        || {
            glp_assert_(
                b"parm != NULL\0" as *const u8 as *const libc::c_char,
                b"api/cplex.c\0" as *const u8 as *const libc::c_char,
                45 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
}
unsafe extern "C" fn check_parm(
    mut func: *const libc::c_char,
    mut parm: *const glp_cpxcp,
) {
    (!func.is_null()
        || {
            glp_assert_(
                b"func != NULL\0" as *const u8 as *const libc::c_char,
                b"api/cplex.c\0" as *const u8 as *const libc::c_char,
                51 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!parm.is_null()
        || {
            glp_assert_(
                b"parm != NULL\0" as *const u8 as *const libc::c_char,
                b"api/cplex.c\0" as *const u8 as *const libc::c_char,
                52 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
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
        (*csa_0).count,
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
        (*csa_0).count,
    );
    arg = args.clone();
    glp_vprintf(fmt, arg.as_va_list());
}
unsafe extern "C" fn read_char(mut csa_0: *mut csa) {
    let mut c: libc::c_int = 0;
    ((*csa_0).c != -(1 as libc::c_int)
        || {
            glp_assert_(
                b"csa->c != EOF\0" as *const u8 as *const libc::c_char,
                b"api/cplex.c\0" as *const u8 as *const libc::c_char,
                170 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*csa_0).c == '\n' as i32 {
        (*csa_0).count += 1;
        (*csa_0).count;
    }
    c = _glp_getc((*csa_0).fp);
    if c < 0 as libc::c_int {
        if _glp_ioerr((*csa_0).fp) != 0 {
            error(
                csa_0,
                b"read error - %s\n\0" as *const u8 as *const libc::c_char,
                _glp_get_err_msg(),
            );
        } else if (*csa_0).c == '\n' as i32 {
            (*csa_0).count -= 1;
            (*csa_0).count;
            c = -(1 as libc::c_int);
        } else {
            warning(
                csa_0,
                b"missing final end of line\n\0" as *const u8 as *const libc::c_char,
            );
            c = '\n' as i32;
        }
    } else if !(c == '\n' as i32) {
        if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            c = ' ' as i32;
        } else if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
            & _IScntrl as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            error(
                csa_0,
                b"invalid control character 0x%02X\n\0" as *const u8
                    as *const libc::c_char,
                c,
            );
        }
    }
    (*csa_0).c = c;
}
unsafe extern "C" fn add_char(mut csa_0: *mut csa) {
    if (*csa_0).imlen as libc::c_ulong
        == (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        error(
            csa_0,
            b"token '%.15s...' too long\n\0" as *const u8 as *const libc::c_char,
            ((*csa_0).image).as_mut_ptr(),
        );
    }
    let fresh0 = (*csa_0).imlen;
    (*csa_0).imlen = (*csa_0).imlen + 1;
    (*csa_0).image[fresh0 as usize] = (*csa_0).c as libc::c_char;
    (*csa_0).image[(*csa_0).imlen as usize] = '\0' as i32 as libc::c_char;
    read_char(csa_0);
}
unsafe extern "C" fn the_same(
    mut s1: *mut libc::c_char,
    mut s2: *mut libc::c_char,
) -> libc::c_int {
    while *s1 as libc::c_int != '\0' as i32 {
        if ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *s1 as libc::c_uchar as libc::c_int;
                    __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    });
                } else {
                    __res = tolower(*s1 as libc::c_uchar as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(*s1 as libc::c_uchar as libc::c_int as isize);
            }
            __res
        })
            != ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *s2 as libc::c_uchar as libc::c_int;
                        __res = (if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        });
                    } else {
                        __res = tolower(*s2 as libc::c_uchar as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(*s2 as libc::c_uchar as libc::c_int as isize);
                }
                __res
            })
        {
            return 0 as libc::c_int;
        }
        s1 = s1.offset(1);
        s1;
        s2 = s2.offset(1);
        s2;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn scan_token(mut csa_0: *mut csa) {
    let mut current_block: u64;
    let mut flag: libc::c_int = 0;
    (*csa_0).token = -(1 as libc::c_int);
    (*csa_0).image[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    (*csa_0).imlen = 0 as libc::c_int;
    (*csa_0).value = 0.0f64;
    loop {
        flag = 0 as libc::c_int;
        while (*csa_0).c == ' ' as i32 {
            read_char(csa_0);
        }
        if (*csa_0).c == -(1 as libc::c_int) {
            (*csa_0).token = 0 as libc::c_int;
            current_block = 15456862084301247793;
            break;
        } else if (*csa_0).c == '\n' as i32 {
            read_char(csa_0);
            if !(*(*__ctype_b_loc()).offset((*csa_0).c as isize) as libc::c_int
                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0)
            {
                continue;
            }
            flag = 1 as libc::c_int;
            current_block = 12599329904712511516;
            break;
        } else if (*csa_0).c == '\\' as i32 {
            while (*csa_0).c != '\n' as i32 {
                read_char(csa_0);
            }
        } else if *(*__ctype_b_loc()).offset((*csa_0).c as isize) as libc::c_int
            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
            || (*csa_0).c != '.' as i32
                && !(strchr(
                    b"!\"#$%&()/,.;?@_`'{}|~\0" as *const u8 as *const libc::c_char,
                    (*csa_0).c,
                ))
                    .is_null()
        {
            current_block = 12599329904712511516;
            break;
        } else {
            current_block = 3512920355445576850;
            break;
        }
    }
    match current_block {
        12599329904712511516 => {
            (*csa_0).token = 0x9 as libc::c_int;
            while *(*__ctype_b_loc()).offset((*csa_0).c as isize) as libc::c_int
                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                || !(strchr(
                    b"!\"#$%&()/,.;?@_`'{}|~\0" as *const u8 as *const libc::c_char,
                    (*csa_0).c,
                ))
                    .is_null()
            {
                add_char(csa_0);
            }
            if flag != 0 {
                if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"minimize\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ) != 0
                {
                    (*csa_0).token = 0x1 as libc::c_int;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"minimum\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
                {
                    (*csa_0).token = 0x1 as libc::c_int;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"min\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
                {
                    (*csa_0).token = 0x1 as libc::c_int;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"maximize\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ) != 0
                {
                    (*csa_0).token = 0x2 as libc::c_int;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"maximum\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
                {
                    (*csa_0).token = 0x2 as libc::c_int;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"max\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
                {
                    (*csa_0).token = 0x2 as libc::c_int;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"subject\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
                {
                    if (*csa_0).c == ' ' as i32 {
                        read_char(csa_0);
                        if ({
                            let mut __res: libc::c_int = 0;
                            if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                > 1 as libc::c_int as libc::c_ulong
                            {
                                if 0 != 0 {
                                    let mut __c: libc::c_int = (*csa_0).c;
                                    __res = (if __c < -(128 as libc::c_int)
                                        || __c > 255 as libc::c_int
                                    {
                                        __c
                                    } else {
                                        *(*__ctype_tolower_loc()).offset(__c as isize)
                                    });
                                } else {
                                    __res = tolower((*csa_0).c);
                                }
                            } else {
                                __res = *(*__ctype_tolower_loc())
                                    .offset((*csa_0).c as isize);
                            }
                            __res
                        }) == 't' as i32
                        {
                            (*csa_0).token = 0x3 as libc::c_int;
                            let fresh1 = (*csa_0).imlen;
                            (*csa_0).imlen = (*csa_0).imlen + 1;
                            (*csa_0).image[fresh1 as usize] = ' ' as i32 as libc::c_char;
                            (*csa_0)
                                .image[(*csa_0).imlen
                                as usize] = '\0' as i32 as libc::c_char;
                            add_char(csa_0);
                            if ({
                                let mut __res: libc::c_int = 0;
                                if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    > 1 as libc::c_int as libc::c_ulong
                                {
                                    if 0 != 0 {
                                        let mut __c: libc::c_int = (*csa_0).c;
                                        __res = (if __c < -(128 as libc::c_int)
                                            || __c > 255 as libc::c_int
                                        {
                                            __c
                                        } else {
                                            *(*__ctype_tolower_loc()).offset(__c as isize)
                                        });
                                    } else {
                                        __res = tolower((*csa_0).c);
                                    }
                                } else {
                                    __res = *(*__ctype_tolower_loc())
                                        .offset((*csa_0).c as isize);
                                }
                                __res
                            }) != 'o' as i32
                            {
                                error(
                                    csa_0,
                                    b"keyword 'subject to' incomplete\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            add_char(csa_0);
                            if *(*__ctype_b_loc()).offset((*csa_0).c as isize)
                                as libc::c_int
                                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                            {
                                error(
                                    csa_0,
                                    b"keyword '%s%c...' not recognized\n\0" as *const u8
                                        as *const libc::c_char,
                                    ((*csa_0).image).as_mut_ptr(),
                                    (*csa_0).c,
                                );
                            }
                        }
                    }
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"such\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
                {
                    if (*csa_0).c == ' ' as i32 {
                        read_char(csa_0);
                        if ({
                            let mut __res: libc::c_int = 0;
                            if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                > 1 as libc::c_int as libc::c_ulong
                            {
                                if 0 != 0 {
                                    let mut __c: libc::c_int = (*csa_0).c;
                                    __res = (if __c < -(128 as libc::c_int)
                                        || __c > 255 as libc::c_int
                                    {
                                        __c
                                    } else {
                                        *(*__ctype_tolower_loc()).offset(__c as isize)
                                    });
                                } else {
                                    __res = tolower((*csa_0).c);
                                }
                            } else {
                                __res = *(*__ctype_tolower_loc())
                                    .offset((*csa_0).c as isize);
                            }
                            __res
                        }) == 't' as i32
                        {
                            (*csa_0).token = 0x3 as libc::c_int;
                            let fresh2 = (*csa_0).imlen;
                            (*csa_0).imlen = (*csa_0).imlen + 1;
                            (*csa_0).image[fresh2 as usize] = ' ' as i32 as libc::c_char;
                            (*csa_0)
                                .image[(*csa_0).imlen
                                as usize] = '\0' as i32 as libc::c_char;
                            add_char(csa_0);
                            if ({
                                let mut __res: libc::c_int = 0;
                                if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    > 1 as libc::c_int as libc::c_ulong
                                {
                                    if 0 != 0 {
                                        let mut __c: libc::c_int = (*csa_0).c;
                                        __res = (if __c < -(128 as libc::c_int)
                                            || __c > 255 as libc::c_int
                                        {
                                            __c
                                        } else {
                                            *(*__ctype_tolower_loc()).offset(__c as isize)
                                        });
                                    } else {
                                        __res = tolower((*csa_0).c);
                                    }
                                } else {
                                    __res = *(*__ctype_tolower_loc())
                                        .offset((*csa_0).c as isize);
                                }
                                __res
                            }) != 'h' as i32
                            {
                                current_block = 14914615493902279771;
                            } else {
                                current_block = 2480299350034459858;
                            }
                            loop {
                                match current_block {
                                    14914615493902279771 => {
                                        error(
                                            csa_0,
                                            b"keyword 'such that' incomplete\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        current_block = 2480299350034459858;
                                    }
                                    _ => {
                                        add_char(csa_0);
                                        if ({
                                            let mut __res: libc::c_int = 0;
                                            if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                                > 1 as libc::c_int as libc::c_ulong
                                            {
                                                if 0 != 0 {
                                                    let mut __c: libc::c_int = (*csa_0).c;
                                                    __res = (if __c < -(128 as libc::c_int)
                                                        || __c > 255 as libc::c_int
                                                    {
                                                        __c
                                                    } else {
                                                        *(*__ctype_tolower_loc()).offset(__c as isize)
                                                    });
                                                } else {
                                                    __res = tolower((*csa_0).c);
                                                }
                                            } else {
                                                __res = *(*__ctype_tolower_loc())
                                                    .offset((*csa_0).c as isize);
                                            }
                                            __res
                                        }) != 'a' as i32
                                        {
                                            current_block = 14914615493902279771;
                                            continue;
                                        }
                                        add_char(csa_0);
                                        if ({
                                            let mut __res: libc::c_int = 0;
                                            if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                                > 1 as libc::c_int as libc::c_ulong
                                            {
                                                if 0 != 0 {
                                                    let mut __c: libc::c_int = (*csa_0).c;
                                                    __res = (if __c < -(128 as libc::c_int)
                                                        || __c > 255 as libc::c_int
                                                    {
                                                        __c
                                                    } else {
                                                        *(*__ctype_tolower_loc()).offset(__c as isize)
                                                    });
                                                } else {
                                                    __res = tolower((*csa_0).c);
                                                }
                                            } else {
                                                __res = *(*__ctype_tolower_loc())
                                                    .offset((*csa_0).c as isize);
                                            }
                                            __res
                                        }) != 't' as i32
                                        {
                                            current_block = 14914615493902279771;
                                        } else {
                                            break;
                                        }
                                    }
                                }
                            }
                            add_char(csa_0);
                            if *(*__ctype_b_loc()).offset((*csa_0).c as isize)
                                as libc::c_int
                                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                            {
                                error(
                                    csa_0,
                                    b"keyword '%s%c...' not recognized\n\0" as *const u8
                                        as *const libc::c_char,
                                    ((*csa_0).image).as_mut_ptr(),
                                    (*csa_0).c,
                                );
                            }
                        }
                    }
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"st\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
                {
                    (*csa_0).token = 0x3 as libc::c_int;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"s.t.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
                {
                    (*csa_0).token = 0x3 as libc::c_int;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"st.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
                {
                    (*csa_0).token = 0x3 as libc::c_int;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"bounds\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
                {
                    (*csa_0).token = 0x4 as libc::c_int;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"bound\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
                {
                    (*csa_0).token = 0x4 as libc::c_int;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"general\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
                {
                    (*csa_0).token = 0x5 as libc::c_int;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"generals\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ) != 0
                {
                    (*csa_0).token = 0x5 as libc::c_int;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"gen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
                {
                    (*csa_0).token = 0x5 as libc::c_int;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"integer\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
                {
                    (*csa_0).token = 0x6 as libc::c_int;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"integers\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ) != 0
                {
                    (*csa_0).token = 0x6 as libc::c_int;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"int\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
                {
                    (*csa_0).token = 0x6 as libc::c_int;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"binary\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
                {
                    (*csa_0).token = 0x7 as libc::c_int;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"binaries\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ) != 0
                {
                    (*csa_0).token = 0x7 as libc::c_int;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"bin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
                {
                    (*csa_0).token = 0x7 as libc::c_int;
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"end\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
                {
                    (*csa_0).token = 0x8 as libc::c_int;
                }
            }
        }
        3512920355445576850 => {
            if *(*__ctype_b_loc()).offset((*csa_0).c as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                || (*csa_0).c == '.' as i32
            {
                (*csa_0).token = 0xa as libc::c_int;
                while *(*__ctype_b_loc()).offset((*csa_0).c as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    add_char(csa_0);
                }
                if (*csa_0).c == '.' as i32 {
                    add_char(csa_0);
                    if (*csa_0).imlen == 1 as libc::c_int
                        && *(*__ctype_b_loc()).offset((*csa_0).c as isize) as libc::c_int
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            == 0
                    {
                        error(
                            csa_0,
                            b"invalid use of decimal point\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    while *(*__ctype_b_loc()).offset((*csa_0).c as isize) as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        add_char(csa_0);
                    }
                }
                if (*csa_0).c == 'e' as i32 || (*csa_0).c == 'E' as i32 {
                    add_char(csa_0);
                    if (*csa_0).c == '+' as i32 || (*csa_0).c == '-' as i32 {
                        add_char(csa_0);
                    }
                    if *(*__ctype_b_loc()).offset((*csa_0).c as isize) as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                    {
                        error(
                            csa_0,
                            b"numeric constant '%s' incomplete\n\0" as *const u8
                                as *const libc::c_char,
                            ((*csa_0).image).as_mut_ptr(),
                        );
                    }
                    while *(*__ctype_b_loc()).offset((*csa_0).c as isize) as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        add_char(csa_0);
                    }
                }
                if _glp_str2num(((*csa_0).image).as_mut_ptr(), &mut (*csa_0).value) != 0
                {
                    error(
                        csa_0,
                        b"numeric constant '%s' out of range\n\0" as *const u8
                            as *const libc::c_char,
                        ((*csa_0).image).as_mut_ptr(),
                    );
                }
            } else if (*csa_0).c == '+' as i32 {
                (*csa_0).token = 0xb as libc::c_int;
                add_char(csa_0);
            } else if (*csa_0).c == '-' as i32 {
                (*csa_0).token = 0xc as libc::c_int;
                add_char(csa_0);
            } else if (*csa_0).c == ':' as i32 {
                (*csa_0).token = 0xd as libc::c_int;
                add_char(csa_0);
            } else if (*csa_0).c == '<' as i32 {
                (*csa_0).token = 0xe as libc::c_int;
                add_char(csa_0);
                if (*csa_0).c == '=' as i32 {
                    add_char(csa_0);
                }
            } else if (*csa_0).c == '>' as i32 {
                (*csa_0).token = 0xf as libc::c_int;
                add_char(csa_0);
                if (*csa_0).c == '=' as i32 {
                    add_char(csa_0);
                }
            } else if (*csa_0).c == '=' as i32 {
                (*csa_0).token = 0x10 as libc::c_int;
                add_char(csa_0);
                if (*csa_0).c == '<' as i32 {
                    (*csa_0).token = 0xe as libc::c_int;
                    add_char(csa_0);
                } else if (*csa_0).c == '>' as i32 {
                    (*csa_0).token = 0xf as libc::c_int;
                    add_char(csa_0);
                }
            } else {
                error(
                    csa_0,
                    b"character '%c' not recognized\n\0" as *const u8
                        as *const libc::c_char,
                    (*csa_0).c,
                );
            }
        }
        _ => {}
    }
    while (*csa_0).c == ' ' as i32 {
        read_char(csa_0);
    }
}
unsafe extern "C" fn find_col(
    mut csa_0: *mut csa,
    mut name: *mut libc::c_char,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    j = glp_find_col((*csa_0).P, name);
    if j == 0 as libc::c_int {
        j = glp_add_cols((*csa_0).P, 1 as libc::c_int);
        glp_set_col_name((*csa_0).P, j, name);
        if (*csa_0).n_max < j {
            let mut n_max: libc::c_int = (*csa_0).n_max;
            let mut ind: *mut libc::c_int = (*csa_0).ind;
            let mut val: *mut libc::c_double = (*csa_0).val;
            let mut flag: *mut libc::c_char = (*csa_0).flag;
            let mut lb: *mut libc::c_double = (*csa_0).lb;
            let mut ub: *mut libc::c_double = (*csa_0).ub;
            (*csa_0).n_max += (*csa_0).n_max;
            (*csa_0)
                .ind = glp_alloc(
                1 as libc::c_int + (*csa_0).n_max,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
            ) as *mut libc::c_int;
            memcpy(
                &mut *((*csa_0).ind).offset(1 as libc::c_int as isize)
                    as *mut libc::c_int as *mut libc::c_void,
                &mut *ind.offset(1 as libc::c_int as isize) as *mut libc::c_int
                    as *const libc::c_void,
                (n_max as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
            glp_free(ind as *mut libc::c_void);
            (*csa_0)
                .val = glp_alloc(
                1 as libc::c_int + (*csa_0).n_max,
                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
            ) as *mut libc::c_double;
            memcpy(
                &mut *((*csa_0).val).offset(1 as libc::c_int as isize)
                    as *mut libc::c_double as *mut libc::c_void,
                &mut *val.offset(1 as libc::c_int as isize) as *mut libc::c_double
                    as *const libc::c_void,
                (n_max as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            );
            glp_free(val as *mut libc::c_void);
            (*csa_0)
                .flag = glp_alloc(
                1 as libc::c_int + (*csa_0).n_max,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
            ) as *mut libc::c_char;
            memset(
                &mut *((*csa_0).flag).offset(1 as libc::c_int as isize)
                    as *mut libc::c_char as *mut libc::c_void,
                0 as libc::c_int,
                ((*csa_0).n_max as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            memcpy(
                &mut *((*csa_0).flag).offset(1 as libc::c_int as isize)
                    as *mut libc::c_char as *mut libc::c_void,
                &mut *flag.offset(1 as libc::c_int as isize) as *mut libc::c_char
                    as *const libc::c_void,
                (n_max as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            glp_free(flag as *mut libc::c_void);
            (*csa_0)
                .lb = glp_alloc(
                1 as libc::c_int + (*csa_0).n_max,
                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
            ) as *mut libc::c_double;
            memcpy(
                &mut *((*csa_0).lb).offset(1 as libc::c_int as isize)
                    as *mut libc::c_double as *mut libc::c_void,
                &mut *lb.offset(1 as libc::c_int as isize) as *mut libc::c_double
                    as *const libc::c_void,
                (n_max as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            );
            glp_free(lb as *mut libc::c_void);
            (*csa_0)
                .ub = glp_alloc(
                1 as libc::c_int + (*csa_0).n_max,
                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
            ) as *mut libc::c_double;
            memcpy(
                &mut *((*csa_0).ub).offset(1 as libc::c_int as isize)
                    as *mut libc::c_double as *mut libc::c_void,
                &mut *ub.offset(1 as libc::c_int as isize) as *mut libc::c_double
                    as *const libc::c_void,
                (n_max as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            );
            glp_free(ub as *mut libc::c_void);
        }
        *((*csa_0).lb).offset(j as isize) = 1.7976931348623157e+308f64;
        *((*csa_0).ub).offset(j as isize) = -1.7976931348623157e+308f64;
    }
    return j;
}
unsafe extern "C" fn parse_linear_form(mut csa_0: *mut csa) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut newlen: libc::c_int = 0;
    let mut s: libc::c_double = 0.;
    let mut coef: libc::c_double = 0.;
    loop {
        if (*csa_0).token == 0xb as libc::c_int {
            s = 1.0f64;
            scan_token(csa_0);
        } else if (*csa_0).token == 0xc as libc::c_int {
            s = -1.0f64;
            scan_token(csa_0);
        } else {
            s = 1.0f64;
        }
        if (*csa_0).token == 0xa as libc::c_int {
            coef = (*csa_0).value;
            scan_token(csa_0);
        } else {
            coef = 1.0f64;
        }
        if (*csa_0).token != 0x9 as libc::c_int {
            error(
                csa_0,
                b"missing variable name\n\0" as *const u8 as *const libc::c_char,
            );
        }
        j = find_col(csa_0, ((*csa_0).image).as_mut_ptr());
        if *((*csa_0).flag).offset(j as isize) != 0 {
            error(
                csa_0,
                b"multiple use of variable '%s' not allowed\n\0" as *const u8
                    as *const libc::c_char,
                ((*csa_0).image).as_mut_ptr(),
            );
        }
        len += 1;
        len;
        *((*csa_0).ind).offset(len as isize) = j;
        *((*csa_0).val).offset(len as isize) = s * coef;
        *((*csa_0).flag).offset(j as isize) = 1 as libc::c_int as libc::c_char;
        scan_token(csa_0);
        if !((*csa_0).token == 0xb as libc::c_int
            || (*csa_0).token == 0xc as libc::c_int)
        {
            break;
        }
    }
    k = 1 as libc::c_int;
    while k <= len {
        *((*csa_0).flag)
            .offset(
                *((*csa_0).ind).offset(k as isize) as isize,
            ) = 0 as libc::c_int as libc::c_char;
        k += 1;
        k;
    }
    newlen = 0 as libc::c_int;
    k = 1 as libc::c_int;
    while k <= len {
        if *((*csa_0).val).offset(k as isize) != 0.0f64 {
            newlen += 1;
            newlen;
            *((*csa_0).ind).offset(newlen as isize) = *((*csa_0).ind).offset(k as isize);
            *((*csa_0).val).offset(newlen as isize) = *((*csa_0).val).offset(k as isize);
        }
        k += 1;
        k;
    }
    return newlen;
}
unsafe extern "C" fn parse_objective(mut csa_0: *mut csa) {
    let mut k: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if (*csa_0).token == 0x1 as libc::c_int {
        glp_set_obj_dir((*csa_0).P, 1 as libc::c_int);
    } else if (*csa_0).token == 0x2 as libc::c_int {
        glp_set_obj_dir((*csa_0).P, 2 as libc::c_int);
    } else {
        (csa_0 != csa_0
            || {
                glp_assert_(
                    b"csa != csa\0" as *const u8 as *const libc::c_char,
                    b"api/cplex.c\0" as *const u8 as *const libc::c_char,
                    502 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    scan_token(csa_0);
    if (*csa_0).token == 0x9 as libc::c_int && (*csa_0).c == ':' as i32 {
        glp_set_obj_name((*csa_0).P, ((*csa_0).image).as_mut_ptr());
        scan_token(csa_0);
        ((*csa_0).token == 0xd as libc::c_int
            || {
                glp_assert_(
                    b"csa->token == T_COLON\0" as *const u8 as *const libc::c_char,
                    b"api/cplex.c\0" as *const u8 as *const libc::c_char,
                    509 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        scan_token(csa_0);
    } else {
        glp_set_obj_name((*csa_0).P, b"obj\0" as *const u8 as *const libc::c_char);
    }
    len = parse_linear_form(csa_0);
    k = 1 as libc::c_int;
    while k <= len {
        glp_set_obj_coef(
            (*csa_0).P,
            *((*csa_0).ind).offset(k as isize),
            *((*csa_0).val).offset(k as isize),
        );
        k += 1;
        k;
    }
}
unsafe extern "C" fn parse_constraints(mut csa_0: *mut csa) {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut s: libc::c_double = 0.;
    ((*csa_0).token == 0x3 as libc::c_int
        || {
            glp_assert_(
                b"csa->token == T_SUBJECT_TO\0" as *const u8 as *const libc::c_char,
                b"api/cplex.c\0" as *const u8 as *const libc::c_char,
                543 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    scan_token(csa_0);
    loop {
        i = glp_add_rows((*csa_0).P, 1 as libc::c_int);
        if (*csa_0).token == 0x9 as libc::c_int && (*csa_0).c == ':' as i32 {
            if glp_find_row((*csa_0).P, ((*csa_0).image).as_mut_ptr())
                != 0 as libc::c_int
            {
                error(
                    csa_0,
                    b"constraint '%s' multiply defined\n\0" as *const u8
                        as *const libc::c_char,
                    ((*csa_0).image).as_mut_ptr(),
                );
            }
            glp_set_row_name((*csa_0).P, i, ((*csa_0).image).as_mut_ptr());
            scan_token(csa_0);
            ((*csa_0).token == 0xd as libc::c_int
                || {
                    glp_assert_(
                        b"csa->token == T_COLON\0" as *const u8 as *const libc::c_char,
                        b"api/cplex.c\0" as *const u8 as *const libc::c_char,
                        555 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            scan_token(csa_0);
        } else {
            let mut name: [libc::c_char; 50] = [0; 50];
            sprintf(
                name.as_mut_ptr(),
                b"r.%d\0" as *const u8 as *const libc::c_char,
                (*csa_0).count,
            );
            glp_set_row_name((*csa_0).P, i, name.as_mut_ptr());
        }
        len = parse_linear_form(csa_0);
        glp_set_mat_row(
            (*csa_0).P,
            i,
            len,
            (*csa_0).ind as *const libc::c_int,
            (*csa_0).val as *const libc::c_double,
        );
        if (*csa_0).token == 0xe as libc::c_int {
            type_0 = 3 as libc::c_int;
            scan_token(csa_0);
        } else if (*csa_0).token == 0xf as libc::c_int {
            type_0 = 2 as libc::c_int;
            scan_token(csa_0);
        } else if (*csa_0).token == 0x10 as libc::c_int {
            type_0 = 5 as libc::c_int;
            scan_token(csa_0);
        } else {
            error(
                csa_0,
                b"missing constraint sense\n\0" as *const u8 as *const libc::c_char,
            );
        }
        if (*csa_0).token == 0xb as libc::c_int {
            s = 1.0f64;
            scan_token(csa_0);
        } else if (*csa_0).token == 0xc as libc::c_int {
            s = -1.0f64;
            scan_token(csa_0);
        } else {
            s = 1.0f64;
        }
        if (*csa_0).token != 0xa as libc::c_int {
            error(
                csa_0,
                b"missing right-hand side\n\0" as *const u8 as *const libc::c_char,
            );
        }
        glp_set_row_bnds((*csa_0).P, i, type_0, s * (*csa_0).value, s * (*csa_0).value);
        if !((*csa_0).c == '\n' as i32 || (*csa_0).c == -(1 as libc::c_int)) {
            error(
                csa_0,
                b"invalid symbol(s) beyond right-hand side\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        scan_token(csa_0);
        if !((*csa_0).token == 0xb as libc::c_int || (*csa_0).token == 0xc as libc::c_int
            || (*csa_0).token == 0xa as libc::c_int
            || (*csa_0).token == 0x9 as libc::c_int)
        {
            break;
        }
    };
}
unsafe extern "C" fn set_lower_bound(
    mut csa_0: *mut csa,
    mut j: libc::c_int,
    mut lb: libc::c_double,
) {
    if *((*csa_0).lb).offset(j as isize) != 1.7976931348623157e+308f64
        && (*csa_0).lb_warn == 0
    {
        warning(
            csa_0,
            b"lower bound of variable '%s' redefined\n\0" as *const u8
                as *const libc::c_char,
            glp_get_col_name((*csa_0).P, j),
        );
        (*csa_0).lb_warn = 1 as libc::c_int;
    }
    *((*csa_0).lb).offset(j as isize) = lb;
}
unsafe extern "C" fn set_upper_bound(
    mut csa_0: *mut csa,
    mut j: libc::c_int,
    mut ub: libc::c_double,
) {
    if *((*csa_0).ub).offset(j as isize) != -1.7976931348623157e+308f64
        && (*csa_0).ub_warn == 0
    {
        warning(
            csa_0,
            b"upper bound of variable '%s' redefined\n\0" as *const u8
                as *const libc::c_char,
            glp_get_col_name((*csa_0).P, j),
        );
        (*csa_0).ub_warn = 1 as libc::c_int;
    }
    *((*csa_0).ub).offset(j as isize) = ub;
}
unsafe extern "C" fn parse_bounds(mut csa_0: *mut csa) {
    let mut j: libc::c_int = 0;
    let mut lb_flag: libc::c_int = 0;
    let mut lb: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    ((*csa_0).token == 0x4 as libc::c_int
        || {
            glp_assert_(
                b"csa->token == T_BOUNDS\0" as *const u8 as *const libc::c_char,
                b"api/cplex.c\0" as *const u8 as *const libc::c_char,
                641 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    scan_token(csa_0);
    while (*csa_0).token == 0xb as libc::c_int || (*csa_0).token == 0xc as libc::c_int
        || (*csa_0).token == 0xa as libc::c_int || (*csa_0).token == 0x9 as libc::c_int
    {
        if (*csa_0).token == 0xb as libc::c_int || (*csa_0).token == 0xc as libc::c_int {
            lb_flag = 1 as libc::c_int;
            s = if (*csa_0).token == 0xb as libc::c_int { 1.0f64 } else { -1.0f64 };
            scan_token(csa_0);
            if (*csa_0).token == 0xa as libc::c_int {
                lb = s * (*csa_0).value;
                scan_token(csa_0);
            } else if the_same(
                ((*csa_0).image).as_mut_ptr(),
                b"infinity\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
                || the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"inf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
            {
                if s > 0.0f64 {
                    error(
                        csa_0,
                        b"invalid use of '+inf' as lower bound\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                lb = -1.7976931348623157e+308f64;
                scan_token(csa_0);
            } else {
                error(
                    csa_0,
                    b"missing lower bound\n\0" as *const u8 as *const libc::c_char,
                );
            }
        } else if (*csa_0).token == 0xa as libc::c_int {
            lb_flag = 1 as libc::c_int;
            lb = (*csa_0).value;
            scan_token(csa_0);
        } else {
            lb_flag = 0 as libc::c_int;
        }
        if lb_flag != 0 {
            if (*csa_0).token != 0xe as libc::c_int {
                error(
                    csa_0,
                    b"missing '<', '<=', or '=<' after lower bound\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            scan_token(csa_0);
        }
        if (*csa_0).token != 0x9 as libc::c_int {
            error(
                csa_0,
                b"missing variable name\n\0" as *const u8 as *const libc::c_char,
            );
        }
        j = find_col(csa_0, ((*csa_0).image).as_mut_ptr());
        if lb_flag != 0 {
            set_lower_bound(csa_0, j, lb);
        }
        scan_token(csa_0);
        if (*csa_0).token == 0xe as libc::c_int {
            scan_token(csa_0);
            if (*csa_0).token == 0xb as libc::c_int
                || (*csa_0).token == 0xc as libc::c_int
            {
                s = if (*csa_0).token == 0xb as libc::c_int { 1.0f64 } else { -1.0f64 };
                scan_token(csa_0);
                if (*csa_0).token == 0xa as libc::c_int {
                    set_upper_bound(csa_0, j, s * (*csa_0).value);
                    scan_token(csa_0);
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"infinity\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ) != 0
                    || the_same(
                        ((*csa_0).image).as_mut_ptr(),
                        b"inf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ) != 0
                {
                    if s < 0.0f64 {
                        error(
                            csa_0,
                            b"invalid use of '-inf' as upper bound\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    set_upper_bound(csa_0, j, 1.7976931348623157e+308f64);
                    scan_token(csa_0);
                } else {
                    error(
                        csa_0,
                        b"missing upper bound\n\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else if (*csa_0).token == 0xa as libc::c_int {
                set_upper_bound(csa_0, j, (*csa_0).value);
                scan_token(csa_0);
            } else {
                error(
                    csa_0,
                    b"missing upper bound\n\0" as *const u8 as *const libc::c_char,
                );
            }
        } else if (*csa_0).token == 0xf as libc::c_int {
            if lb_flag != 0 {
                error(
                    csa_0,
                    b"invalid bound definition\n\0" as *const u8 as *const libc::c_char,
                );
            }
            scan_token(csa_0);
            if (*csa_0).token == 0xb as libc::c_int
                || (*csa_0).token == 0xc as libc::c_int
            {
                s = if (*csa_0).token == 0xb as libc::c_int { 1.0f64 } else { -1.0f64 };
                scan_token(csa_0);
                if (*csa_0).token == 0xa as libc::c_int {
                    set_lower_bound(csa_0, j, s * (*csa_0).value);
                    scan_token(csa_0);
                } else if the_same(
                    ((*csa_0).image).as_mut_ptr(),
                    b"infinity\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ) != 0
                    || the_same(
                        ((*csa_0).image).as_mut_ptr(),
                        b"inf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ) == 0 as libc::c_int
                {
                    if s > 0.0f64 {
                        error(
                            csa_0,
                            b"invalid use of '+inf' as lower bound\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    set_lower_bound(csa_0, j, -1.7976931348623157e+308f64);
                    scan_token(csa_0);
                } else {
                    error(
                        csa_0,
                        b"missing lower bound\n\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else if (*csa_0).token == 0xa as libc::c_int {
                set_lower_bound(csa_0, j, (*csa_0).value);
                scan_token(csa_0);
            } else {
                error(
                    csa_0,
                    b"missing lower bound\n\0" as *const u8 as *const libc::c_char,
                );
            }
        } else if (*csa_0).token == 0x10 as libc::c_int {
            if lb_flag != 0 {
                error(
                    csa_0,
                    b"invalid bound definition\n\0" as *const u8 as *const libc::c_char,
                );
            }
            scan_token(csa_0);
            if (*csa_0).token == 0xb as libc::c_int
                || (*csa_0).token == 0xc as libc::c_int
            {
                s = if (*csa_0).token == 0xb as libc::c_int { 1.0f64 } else { -1.0f64 };
                scan_token(csa_0);
                if (*csa_0).token == 0xa as libc::c_int {
                    set_lower_bound(csa_0, j, s * (*csa_0).value);
                    set_upper_bound(csa_0, j, s * (*csa_0).value);
                    scan_token(csa_0);
                } else {
                    error(
                        csa_0,
                        b"missing fixed value\n\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else if (*csa_0).token == 0xa as libc::c_int {
                set_lower_bound(csa_0, j, (*csa_0).value);
                set_upper_bound(csa_0, j, (*csa_0).value);
                scan_token(csa_0);
            } else {
                error(
                    csa_0,
                    b"missing fixed value\n\0" as *const u8 as *const libc::c_char,
                );
            }
        } else if the_same(
            ((*csa_0).image).as_mut_ptr(),
            b"free\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
            if lb_flag != 0 {
                error(
                    csa_0,
                    b"invalid bound definition\n\0" as *const u8 as *const libc::c_char,
                );
            }
            set_lower_bound(csa_0, j, -1.7976931348623157e+308f64);
            set_upper_bound(csa_0, j, 1.7976931348623157e+308f64);
            scan_token(csa_0);
        } else if lb_flag == 0 {
            error(
                csa_0,
                b"invalid bound definition\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
}
unsafe extern "C" fn parse_integer(mut csa_0: *mut csa) {
    let mut j: libc::c_int = 0;
    let mut binary: libc::c_int = 0;
    if (*csa_0).token == 0x5 as libc::c_int {
        binary = 0 as libc::c_int;
        scan_token(csa_0);
    } else if (*csa_0).token == 0x6 as libc::c_int {
        binary = 0 as libc::c_int;
        scan_token(csa_0);
    } else if (*csa_0).token == 0x7 as libc::c_int {
        binary = 1 as libc::c_int;
        scan_token(csa_0);
    } else {
        (csa_0 != csa_0
            || {
                glp_assert_(
                    b"csa != csa\0" as *const u8 as *const libc::c_char,
                    b"api/cplex.c\0" as *const u8 as *const libc::c_char,
                    817 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    while (*csa_0).token == 0x9 as libc::c_int {
        j = find_col(csa_0, ((*csa_0).image).as_mut_ptr());
        glp_set_col_kind((*csa_0).P, j, 2 as libc::c_int);
        if binary != 0 {
            set_lower_bound(
                csa_0,
                j,
                if *((*csa_0).lb).offset(j as isize) == 1.7976931348623157e+308f64 {
                    0.0f64
                } else {
                    *((*csa_0).lb).offset(j as isize)
                },
            );
            set_upper_bound(
                csa_0,
                j,
                if *((*csa_0).ub).offset(j as isize) == -1.7976931348623157e+308f64 {
                    1.0f64
                } else {
                    *((*csa_0).ub).offset(j as isize)
                },
            );
        }
        scan_token(csa_0);
    }
}
pub unsafe extern "C" fn glp_read_lp(
    mut P: *mut glp_prob,
    mut parm: *const glp_cpxcp,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut _parm: glp_cpxcp = glp_cpxcp { foo_bar: [0.; 20] };
    let mut _csa: csa = csa {
        P: 0 as *mut glp_prob,
        parm: 0 as *const glp_cpxcp,
        fname: 0 as *const libc::c_char,
        fp: 0 as *mut glp_file,
        jump: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        count: 0,
        c: 0,
        token: 0,
        image: [0; 256],
        imlen: 0,
        value: 0.,
        n_max: 0,
        ind: 0 as *mut libc::c_int,
        val: 0 as *mut libc::c_double,
        flag: 0 as *mut libc::c_char,
        lb: 0 as *mut libc::c_double,
        ub: 0 as *mut libc::c_double,
        lb_warn: 0,
        ub_warn: 0,
    };
    let mut csa_0: *mut csa = &mut _csa;
    let mut ret: libc::c_int = 0;
    glp_printf(
        b"Reading problem data from '%s'...\n\0" as *const u8 as *const libc::c_char,
        fname,
    );
    if parm.is_null() {
        glp_init_cpxcp(&mut _parm);
        parm = &mut _parm;
    }
    check_parm(b"glp_read_lp\0" as *const u8 as *const libc::c_char, parm);
    (*csa_0).P = P;
    (*csa_0).parm = parm;
    (*csa_0).fname = fname;
    (*csa_0).fp = 0 as *mut glp_file;
    if _setjmp(((*csa_0).jump).as_mut_ptr()) != 0 {
        ret = 1 as libc::c_int;
    } else {
        (*csa_0).count = 0 as libc::c_int;
        (*csa_0).c = '\n' as i32;
        (*csa_0).token = 0 as libc::c_int;
        (*csa_0).image[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        (*csa_0).imlen = 0 as libc::c_int;
        (*csa_0).value = 0.0f64;
        (*csa_0).n_max = 100 as libc::c_int;
        (*csa_0)
            .ind = glp_alloc(
            1 as libc::c_int + (*csa_0).n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*csa_0)
            .val = glp_alloc(
            1 as libc::c_int + (*csa_0).n_max,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        (*csa_0)
            .flag = glp_alloc(
            1 as libc::c_int + (*csa_0).n_max,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_char;
        memset(
            &mut *((*csa_0).flag).offset(1 as libc::c_int as isize) as *mut libc::c_char
                as *mut libc::c_void,
            0 as libc::c_int,
            ((*csa_0).n_max as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
        (*csa_0)
            .lb = glp_alloc(
            1 as libc::c_int + (*csa_0).n_max,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        (*csa_0)
            .ub = glp_alloc(
            1 as libc::c_int + (*csa_0).n_max,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        (*csa_0).ub_warn = 0 as libc::c_int;
        (*csa_0).lb_warn = (*csa_0).ub_warn;
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
            scan_token(csa_0);
            if !((*csa_0).token == 0x1 as libc::c_int
                || (*csa_0).token == 0x2 as libc::c_int)
            {
                error(
                    csa_0,
                    b"'minimize' or 'maximize' keyword missing\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            parse_objective(csa_0);
            if (*csa_0).token != 0x3 as libc::c_int {
                error(
                    csa_0,
                    b"constraints section missing\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            parse_constraints(csa_0);
            if (*csa_0).token == 0x4 as libc::c_int {
                parse_bounds(csa_0);
            }
            while (*csa_0).token == 0x5 as libc::c_int
                || (*csa_0).token == 0x6 as libc::c_int
                || (*csa_0).token == 0x7 as libc::c_int
            {
                parse_integer(csa_0);
            }
            if (*csa_0).token == 0x8 as libc::c_int {
                scan_token(csa_0);
            } else if (*csa_0).token == 0 as libc::c_int {
                warning(
                    csa_0,
                    b"keyword 'end' missing\n\0" as *const u8 as *const libc::c_char,
                );
            } else {
                error(
                    csa_0,
                    b"symbol '%s' in wrong position\n\0" as *const u8
                        as *const libc::c_char,
                    ((*csa_0).image).as_mut_ptr(),
                );
            }
            if (*csa_0).token != 0 as libc::c_int {
                error(
                    csa_0,
                    b"extra symbol(s) detected beyond 'end'\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            let mut j: libc::c_int = 0;
            let mut type_0: libc::c_int = 0;
            let mut lb: libc::c_double = 0.;
            let mut ub: libc::c_double = 0.;
            j = 1 as libc::c_int;
            while j <= (*P).n {
                lb = *((*csa_0).lb).offset(j as isize);
                ub = *((*csa_0).ub).offset(j as isize);
                if lb == 1.7976931348623157e+308f64 {
                    lb = 0.0f64;
                }
                if ub == -1.7976931348623157e+308f64 {
                    ub = 1.7976931348623157e+308f64;
                }
                if lb == -1.7976931348623157e+308f64 && ub == 1.7976931348623157e+308f64
                {
                    type_0 = 1 as libc::c_int;
                } else if ub == 1.7976931348623157e+308f64 {
                    type_0 = 2 as libc::c_int;
                } else if lb == -1.7976931348623157e+308f64 {
                    type_0 = 3 as libc::c_int;
                } else if lb != ub {
                    type_0 = 4 as libc::c_int;
                } else {
                    type_0 = 5 as libc::c_int;
                }
                glp_set_col_bnds((*csa_0).P, j, type_0, lb, ub);
                j += 1;
                j;
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
                b"%d lines were read\n\0" as *const u8 as *const libc::c_char,
                (*csa_0).count,
            );
            glp_delete_index(P);
            glp_sort_matrix(P);
            ret = 0 as libc::c_int;
        }
    }
    if !((*csa_0).fp).is_null() {
        _glp_close((*csa_0).fp);
    }
    glp_free((*csa_0).ind as *mut libc::c_void);
    glp_free((*csa_0).val as *mut libc::c_void);
    glp_free((*csa_0).flag as *mut libc::c_void);
    glp_free((*csa_0).lb as *mut libc::c_void);
    glp_free((*csa_0).ub as *mut libc::c_void);
    if ret != 0 as libc::c_int {
        glp_erase_prob(P);
    }
    return ret;
}
unsafe extern "C" fn check_name(mut name: *mut libc::c_char) -> libc::c_int {
    if *name as libc::c_int == '.' as i32 {
        return 1 as libc::c_int;
    }
    if *(*__ctype_b_loc()).offset(*name as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        return 1 as libc::c_int;
    }
    while *name != 0 {
        if *(*__ctype_b_loc()).offset(*name as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
            == 0
            && (strchr(
                b"!\"#$%&()/,.;?@_`'{}|~\0" as *const u8 as *const libc::c_char,
                *name as libc::c_uchar as libc::c_int,
            ))
                .is_null()
        {
            return 1 as libc::c_int;
        }
        name = name.offset(1);
        name;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn adjust_name(mut name: *mut libc::c_char) {
    while *name != 0 {
        if *name as libc::c_int == ' ' as i32 {
            *name = '_' as i32 as libc::c_char;
        } else if *name as libc::c_int == '-' as i32 {
            *name = '~' as i32 as libc::c_char;
        } else if *name as libc::c_int == '[' as i32 {
            *name = '(' as i32 as libc::c_char;
        } else if *name as libc::c_int == ']' as i32 {
            *name = ')' as i32 as libc::c_char;
        }
        name = name.offset(1);
        name;
    }
}
unsafe extern "C" fn row_name(
    mut csa1: *mut csa1,
    mut i: libc::c_int,
    mut rname: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if i == 0 as libc::c_int {
        name = glp_get_obj_name((*csa1).P);
    } else {
        name = glp_get_row_name((*csa1).P, i);
    }
    if !name.is_null() {
        strcpy(rname, name);
        adjust_name(rname);
        if !(check_name(rname) != 0) {
            return rname;
        }
    }
    if i == 0 as libc::c_int {
        strcpy(rname, b"obj\0" as *const u8 as *const libc::c_char);
    } else {
        sprintf(rname, b"r_%d\0" as *const u8 as *const libc::c_char, i);
    }
    return rname;
}
unsafe extern "C" fn col_name(
    mut csa1: *mut csa1,
    mut j: libc::c_int,
    mut cname: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    name = glp_get_col_name((*csa1).P, j);
    if !name.is_null() {
        strcpy(cname, name);
        adjust_name(cname);
        if !(check_name(cname) != 0) {
            return cname;
        }
    }
    let mut col: *mut GLPCOL = *((*(*csa1).P).col).offset(j as isize);
    if (*col).type_0 == 5 as libc::c_int {
        sprintf(cname, b"s_%d\0" as *const u8 as *const libc::c_char, j);
    } else if (*col).kind == 1 as libc::c_int {
        sprintf(cname, b"x_%d\0" as *const u8 as *const libc::c_char, j);
    } else if !((*col).lb == 0 as libc::c_int as libc::c_double
        && (*col).ub == 1 as libc::c_int as libc::c_double)
    {
        sprintf(cname, b"y_%d\0" as *const u8 as *const libc::c_char, j);
    } else {
        sprintf(cname, b"z_%d\0" as *const u8 as *const libc::c_char, j);
    }
    return cname;
}
pub unsafe extern "C" fn glp_write_lp(
    mut P: *mut glp_prob,
    mut parm: *const glp_cpxcp,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut _parm: glp_cpxcp = glp_cpxcp { foo_bar: [0.; 20] };
    let mut _csa: csa1 = csa1 {
        P: 0 as *mut glp_prob,
        parm: 0 as *const glp_cpxcp,
    };
    let mut csa1: *mut csa1 = &mut _csa;
    let mut fp: *mut glp_file = 0 as *mut glp_file;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut line: [libc::c_char; 1001] = [0; 1001];
    let mut term: [libc::c_char; 501] = [0; 501];
    let mut name: [libc::c_char; 256] = [0; 256];
    glp_printf(
        b"Writing problem data to '%s'...\n\0" as *const u8 as *const libc::c_char,
        fname,
    );
    if parm.is_null() {
        glp_init_cpxcp(&mut _parm);
        parm = &mut _parm;
    }
    check_parm(b"glp_write_lp\0" as *const u8 as *const libc::c_char, parm);
    (*csa1).P = P;
    (*csa1).parm = parm;
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
        _glp_format(
            fp,
            b"\\* Problem: %s *\\\n\0" as *const u8 as *const libc::c_char,
            (if ((*P).name).is_null() {
                b"Unknown\0" as *const u8 as *const libc::c_char
            } else {
                (*P).name as *const libc::c_char
            }),
        );
        count += 1;
        count;
        _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
        count += 1;
        count;
        if !((*P).m > 0 as libc::c_int && (*P).n > 0 as libc::c_int) {
            glp_printf(
                b"Warning: problem has no rows/columns\n\0" as *const u8
                    as *const libc::c_char,
            );
            _glp_format(
                fp,
                b"\\* WARNING: PROBLEM HAS NO ROWS/COLUMNS *\\\n\0" as *const u8
                    as *const libc::c_char,
            );
            count += 1;
            count;
            _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
            count += 1;
            count;
        } else {
            if (*P).dir == 1 as libc::c_int {
                _glp_format(fp, b"Minimize\n\0" as *const u8 as *const libc::c_char);
                count += 1;
                count;
            } else if (*P).dir == 2 as libc::c_int {
                _glp_format(fp, b"Maximize\n\0" as *const u8 as *const libc::c_char);
                count += 1;
                count;
            } else {
                (P != P
                    || {
                        glp_assert_(
                            b"P != P\0" as *const u8 as *const libc::c_char,
                            b"api/cplex.c\0" as *const u8 as *const libc::c_char,
                            1136 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            row_name(csa1, 0 as libc::c_int, name.as_mut_ptr());
            sprintf(
                line.as_mut_ptr(),
                b" %s:\0" as *const u8 as *const libc::c_char,
                name.as_mut_ptr(),
            );
            len = 0 as libc::c_int;
            j = 1 as libc::c_int;
            while j <= (*P).n {
                col = *((*P).col).offset(j as isize);
                if (*col).coef != 0.0f64 || ((*col).ptr).is_null() {
                    len += 1;
                    len;
                    col_name(csa1, j, name.as_mut_ptr());
                    if (*col).coef == 0.0f64 {
                        sprintf(
                            term.as_mut_ptr(),
                            b" + 0 %s\0" as *const u8 as *const libc::c_char,
                            name.as_mut_ptr(),
                        );
                    } else if (*col).coef == 1.0f64 {
                        sprintf(
                            term.as_mut_ptr(),
                            b" + %s\0" as *const u8 as *const libc::c_char,
                            name.as_mut_ptr(),
                        );
                    } else if (*col).coef == -1.0f64 {
                        sprintf(
                            term.as_mut_ptr(),
                            b" - %s\0" as *const u8 as *const libc::c_char,
                            name.as_mut_ptr(),
                        );
                    } else if (*col).coef > 0.0f64 {
                        sprintf(
                            term.as_mut_ptr(),
                            b" + %.*g %s\0" as *const u8 as *const libc::c_char,
                            15 as libc::c_int,
                            (*col).coef,
                            name.as_mut_ptr(),
                        );
                    } else {
                        sprintf(
                            term.as_mut_ptr(),
                            b" - %.*g %s\0" as *const u8 as *const libc::c_char,
                            15 as libc::c_int,
                            -(*col).coef,
                            name.as_mut_ptr(),
                        );
                    }
                    if (strlen(line.as_mut_ptr()))
                        .wrapping_add(strlen(term.as_mut_ptr()))
                        > 72 as libc::c_int as libc::c_ulong
                    {
                        _glp_format(
                            fp,
                            b"%s\n\0" as *const u8 as *const libc::c_char,
                            line.as_mut_ptr(),
                        );
                        line[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                        count += 1;
                        count;
                    }
                    strcat(line.as_mut_ptr(), term.as_mut_ptr());
                }
                j += 1;
                j;
            }
            if len == 0 as libc::c_int {
                sprintf(
                    term.as_mut_ptr(),
                    b" 0 %s\0" as *const u8 as *const libc::c_char,
                    col_name(csa1, 1 as libc::c_int, name.as_mut_ptr()),
                );
                strcat(line.as_mut_ptr(), term.as_mut_ptr());
            }
            _glp_format(
                fp,
                b"%s\n\0" as *const u8 as *const libc::c_char,
                line.as_mut_ptr(),
            );
            count += 1;
            count;
            if (*P).c0 != 0.0f64 {
                _glp_format(
                    fp,
                    b"\\* constant term = %.*g *\\\n\0" as *const u8
                        as *const libc::c_char,
                    15 as libc::c_int,
                    (*P).c0,
                );
                count += 1;
                count;
            }
            _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
            count += 1;
            count;
            _glp_format(fp, b"Subject To\n\0" as *const u8 as *const libc::c_char);
            count += 1;
            count;
            i = 1 as libc::c_int;
            while i <= (*P).m {
                row = *((*P).row).offset(i as isize);
                if !((*row).type_0 == 1 as libc::c_int) {
                    row_name(csa1, i, name.as_mut_ptr());
                    sprintf(
                        line.as_mut_ptr(),
                        b" %s:\0" as *const u8 as *const libc::c_char,
                        name.as_mut_ptr(),
                    );
                    aij = (*row).ptr;
                    while !aij.is_null() {
                        col_name(csa1, (*(*aij).col).j, name.as_mut_ptr());
                        if (*aij).val == 1.0f64 {
                            sprintf(
                                term.as_mut_ptr(),
                                b" + %s\0" as *const u8 as *const libc::c_char,
                                name.as_mut_ptr(),
                            );
                        } else if (*aij).val == -1.0f64 {
                            sprintf(
                                term.as_mut_ptr(),
                                b" - %s\0" as *const u8 as *const libc::c_char,
                                name.as_mut_ptr(),
                            );
                        } else if (*aij).val > 0.0f64 {
                            sprintf(
                                term.as_mut_ptr(),
                                b" + %.*g %s\0" as *const u8 as *const libc::c_char,
                                15 as libc::c_int,
                                (*aij).val,
                                name.as_mut_ptr(),
                            );
                        } else {
                            sprintf(
                                term.as_mut_ptr(),
                                b" - %.*g %s\0" as *const u8 as *const libc::c_char,
                                15 as libc::c_int,
                                -(*aij).val,
                                name.as_mut_ptr(),
                            );
                        }
                        if (strlen(line.as_mut_ptr()))
                            .wrapping_add(strlen(term.as_mut_ptr()))
                            > 72 as libc::c_int as libc::c_ulong
                        {
                            _glp_format(
                                fp,
                                b"%s\n\0" as *const u8 as *const libc::c_char,
                                line.as_mut_ptr(),
                            );
                            line[0 as libc::c_int
                                as usize] = '\0' as i32 as libc::c_char;
                            count += 1;
                            count;
                        }
                        strcat(line.as_mut_ptr(), term.as_mut_ptr());
                        aij = (*aij).r_next;
                    }
                    if (*row).type_0 == 4 as libc::c_int {
                        sprintf(
                            term.as_mut_ptr(),
                            b" - ~r_%d\0" as *const u8 as *const libc::c_char,
                            i,
                        );
                        if (strlen(line.as_mut_ptr()))
                            .wrapping_add(strlen(term.as_mut_ptr()))
                            > 72 as libc::c_int as libc::c_ulong
                        {
                            _glp_format(
                                fp,
                                b"%s\n\0" as *const u8 as *const libc::c_char,
                                line.as_mut_ptr(),
                            );
                            line[0 as libc::c_int
                                as usize] = '\0' as i32 as libc::c_char;
                            count += 1;
                            count;
                        }
                        strcat(line.as_mut_ptr(), term.as_mut_ptr());
                    } else if ((*row).ptr).is_null() {
                        sprintf(
                            term.as_mut_ptr(),
                            b" 0 %s\0" as *const u8 as *const libc::c_char,
                            col_name(csa1, 1 as libc::c_int, name.as_mut_ptr()),
                        );
                        strcat(line.as_mut_ptr(), term.as_mut_ptr());
                    }
                    if (*row).type_0 == 2 as libc::c_int {
                        sprintf(
                            term.as_mut_ptr(),
                            b" >= %.*g\0" as *const u8 as *const libc::c_char,
                            15 as libc::c_int,
                            (*row).lb,
                        );
                    } else if (*row).type_0 == 3 as libc::c_int {
                        sprintf(
                            term.as_mut_ptr(),
                            b" <= %.*g\0" as *const u8 as *const libc::c_char,
                            15 as libc::c_int,
                            (*row).ub,
                        );
                    } else if (*row).type_0 == 4 as libc::c_int
                        || (*row).type_0 == 5 as libc::c_int
                    {
                        sprintf(
                            term.as_mut_ptr(),
                            b" = %.*g\0" as *const u8 as *const libc::c_char,
                            15 as libc::c_int,
                            (*row).lb,
                        );
                    } else {
                        (row != row
                            || {
                                glp_assert_(
                                    b"row != row\0" as *const u8 as *const libc::c_char,
                                    b"api/cplex.c\0" as *const u8 as *const libc::c_char,
                                    1212 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                    }
                    if (strlen(line.as_mut_ptr()))
                        .wrapping_add(strlen(term.as_mut_ptr()))
                        > 72 as libc::c_int as libc::c_ulong
                    {
                        _glp_format(
                            fp,
                            b"%s\n\0" as *const u8 as *const libc::c_char,
                            line.as_mut_ptr(),
                        );
                        line[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                        count += 1;
                        count;
                    }
                    strcat(line.as_mut_ptr(), term.as_mut_ptr());
                    _glp_format(
                        fp,
                        b"%s\n\0" as *const u8 as *const libc::c_char,
                        line.as_mut_ptr(),
                    );
                    count += 1;
                    count;
                }
                i += 1;
                i;
            }
            _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
            count += 1;
            count;
            flag = 0 as libc::c_int;
            i = 1 as libc::c_int;
            while i <= (*P).m {
                row = *((*P).row).offset(i as isize);
                if !((*row).type_0 != 4 as libc::c_int) {
                    if flag == 0 {
                        _glp_format(
                            fp,
                            b"Bounds\n\0" as *const u8 as *const libc::c_char,
                        );
                        flag = 1 as libc::c_int;
                        count += 1;
                        count;
                    }
                    _glp_format(
                        fp,
                        b" 0 <= ~r_%d <= %.*g\n\0" as *const u8 as *const libc::c_char,
                        i,
                        15 as libc::c_int,
                        (*row).ub - (*row).lb,
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
                if !((*col).type_0 == 2 as libc::c_int && (*col).lb == 0.0f64) {
                    if flag == 0 {
                        _glp_format(
                            fp,
                            b"Bounds\n\0" as *const u8 as *const libc::c_char,
                        );
                        flag = 1 as libc::c_int;
                        count += 1;
                        count;
                    }
                    col_name(csa1, j, name.as_mut_ptr());
                    if (*col).type_0 == 1 as libc::c_int {
                        _glp_format(
                            fp,
                            b" %s free\n\0" as *const u8 as *const libc::c_char,
                            name.as_mut_ptr(),
                        );
                        count += 1;
                        count;
                    } else if (*col).type_0 == 2 as libc::c_int {
                        _glp_format(
                            fp,
                            b" %s >= %.*g\n\0" as *const u8 as *const libc::c_char,
                            name.as_mut_ptr(),
                            15 as libc::c_int,
                            (*col).lb,
                        );
                        count += 1;
                        count;
                    } else if (*col).type_0 == 3 as libc::c_int {
                        _glp_format(
                            fp,
                            b" -Inf <= %s <= %.*g\n\0" as *const u8
                                as *const libc::c_char,
                            name.as_mut_ptr(),
                            15 as libc::c_int,
                            (*col).ub,
                        );
                        count += 1;
                        count;
                    } else if (*col).type_0 == 4 as libc::c_int {
                        _glp_format(
                            fp,
                            b" %.*g <= %s <= %.*g\n\0" as *const u8
                                as *const libc::c_char,
                            15 as libc::c_int,
                            (*col).lb,
                            name.as_mut_ptr(),
                            15 as libc::c_int,
                            (*col).ub,
                        );
                        count += 1;
                        count;
                    } else if (*col).type_0 == 5 as libc::c_int {
                        _glp_format(
                            fp,
                            b" %s = %.*g\n\0" as *const u8 as *const libc::c_char,
                            name.as_mut_ptr(),
                            15 as libc::c_int,
                            (*col).lb,
                        );
                        count += 1;
                        count;
                    } else {
                        (col != col
                            || {
                                glp_assert_(
                                    b"col != col\0" as *const u8 as *const libc::c_char,
                                    b"api/cplex.c\0" as *const u8 as *const libc::c_char,
                                    1250 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                    }
                }
                j += 1;
                j;
            }
            if flag != 0 {
                _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
                count += 1;
                count;
            }
            flag = 0 as libc::c_int;
            j = 1 as libc::c_int;
            while j <= (*P).n {
                col = *((*P).col).offset(j as isize);
                if !((*col).kind == 1 as libc::c_int) {
                    ((*col).kind == 2 as libc::c_int
                        || {
                            glp_assert_(
                                b"col->kind == GLP_IV\0" as *const u8
                                    as *const libc::c_char,
                                b"api/cplex.c\0" as *const u8 as *const libc::c_char,
                                1258 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    if flag == 0 {
                        _glp_format(
                            fp,
                            b"Generals\n\0" as *const u8 as *const libc::c_char,
                        );
                        flag = 1 as libc::c_int;
                        count += 1;
                        count;
                    }
                    _glp_format(
                        fp,
                        b" %s\n\0" as *const u8 as *const libc::c_char,
                        col_name(csa1, j, name.as_mut_ptr()),
                    );
                    count += 1;
                    count;
                }
                j += 1;
                j;
            }
            if flag != 0 {
                _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
                count += 1;
                count;
            }
        }
        _glp_format(fp, b"End\n\0" as *const u8 as *const libc::c_char);
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
