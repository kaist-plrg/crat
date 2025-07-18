use ::libc;
extern "C" {
    pub type glp_file;
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn _glp_dmx_end_of_line(csa: *mut DMX);
    fn _glp_dmx_read_field(csa: *mut DMX);
    fn _glp_dmx_read_designator(csa: *mut DMX);
    fn _glp_dmx_read_char(csa: *mut DMX);
    fn _glp_dmx_error(csa: *mut DMX, fmt: *const libc::c_char, _: ...);
    fn _glp_close(f: *mut glp_file) -> libc::c_int;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn _glp_get_err_msg() -> *const libc::c_char;
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn _glp_open(name: *const libc::c_char, mode: *const libc::c_char) -> *mut glp_file;
    fn _glp_str2int(str: *const libc::c_char, val: *mut libc::c_int) -> libc::c_int;
    fn glp_add_rows(P: *mut glp_prob, nrs: libc::c_int) -> libc::c_int;
    fn glp_add_cols(P: *mut glp_prob, ncs: libc::c_int) -> libc::c_int;
    fn glp_set_row_bnds(
        P: *mut glp_prob,
        i: libc::c_int,
        type_0: libc::c_int,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_set_mat_row(
        P: *mut glp_prob,
        i: libc::c_int,
        len: libc::c_int,
        ind: *const libc::c_int,
        val: *const libc::c_double,
    );
    fn glp_sort_matrix(P: *mut glp_prob);
    fn glp_erase_prob(P: *mut glp_prob);
    fn glp_set_col_kind(P: *mut glp_prob, j: libc::c_int, kind: libc::c_int);
}
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
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DMX {
    pub jump: jmp_buf,
    pub fname: *const libc::c_char,
    pub fp: *mut glp_file,
    pub count: libc::c_int,
    pub c: libc::c_int,
    pub field: [libc::c_char; 256],
    pub empty: libc::c_int,
    pub nonint: libc::c_int,
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
pub unsafe extern "C" fn glp_read_cnfsat(
    mut P: *mut glp_prob,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut _csa: DMX = DMX {
        jump: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        fname: 0 as *const libc::c_char,
        fp: 0 as *mut glp_file,
        count: 0,
        c: 0,
        field: [0; 256],
        empty: 0,
        nonint: 0,
    };
    let mut csa: *mut DMX = &mut _csa;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut neg: libc::c_int = 0;
    let mut rhs: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut map: *mut libc::c_char = 0 as *mut libc::c_char;
    if fname.is_null() {
        (glp_error_(
            b"api/rdcnf.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int,
        ))
            .unwrap()(
            b"glp_read_cnfsat: fname = %p; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            fname,
        );
    }
    glp_erase_prob(P);
    if _setjmp(((*csa).jump).as_mut_ptr()) != 0 {
        ret = 1 as libc::c_int;
    } else {
        (*csa).fname = fname;
        (*csa).fp = 0 as *mut glp_file;
        (*csa).count = 0 as libc::c_int;
        (*csa).c = '\n' as i32;
        (*csa).field[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        (*csa).nonint = 0 as libc::c_int;
        (*csa).empty = (*csa).nonint;
        glp_printf(
            b"Reading CNF-SAT problem data from '%s'...\n\0" as *const u8
                as *const libc::c_char,
            fname,
        );
        (*csa).fp = _glp_open(fname, b"r\0" as *const u8 as *const libc::c_char);
        if ((*csa).fp).is_null() {
            glp_printf(
                b"Unable to open '%s' - %s\n\0" as *const u8 as *const libc::c_char,
                fname,
                _glp_get_err_msg(),
            );
            longjmp(((*csa).jump).as_mut_ptr(), 1 as libc::c_int);
        }
        _glp_dmx_read_designator(csa);
        if strcmp(
            ((*csa).field).as_mut_ptr(),
            b"p\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            _glp_dmx_error(
                csa,
                b"problem line missing or invalid\0" as *const u8 as *const libc::c_char,
            );
        }
        _glp_dmx_read_field(csa);
        if strcmp(
            ((*csa).field).as_mut_ptr(),
            b"cnf\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            _glp_dmx_error(
                csa,
                b"wrong problem designator; 'cnf' expected\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        _glp_dmx_read_field(csa);
        if !(_glp_str2int(((*csa).field).as_mut_ptr(), &mut n) == 0 as libc::c_int
            && n >= 0 as libc::c_int)
        {
            _glp_dmx_error(
                csa,
                b"number of variables missing or invalid\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        _glp_dmx_read_field(csa);
        if !(_glp_str2int(((*csa).field).as_mut_ptr(), &mut m) == 0 as libc::c_int
            && m >= 0 as libc::c_int)
        {
            _glp_dmx_error(
                csa,
                b"number of clauses missing or invalid\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        glp_printf(
            b"Instance has %d variable%s and %d clause%s\n\0" as *const u8
                as *const libc::c_char,
            n,
            if n == 1 as libc::c_int {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"s\0" as *const u8 as *const libc::c_char
            },
            m,
            if m == 1 as libc::c_int {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"s\0" as *const u8 as *const libc::c_char
            },
        );
        _glp_dmx_end_of_line(csa);
        if m > 0 as libc::c_int {
            glp_add_rows(P, m);
        }
        if n > 0 as libc::c_int {
            glp_add_cols(P, n);
            j = 1 as libc::c_int;
            while j <= n {
                glp_set_col_kind(P, j, 3 as libc::c_int);
                j += 1;
                j;
            }
        }
        ind = glp_alloc(
            1 as libc::c_int + n,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        val = glp_alloc(
            1 as libc::c_int + n,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        map = glp_alloc(
            1 as libc::c_int + n,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_char;
        j = 1 as libc::c_int;
        while j <= n {
            *map.offset(j as isize) = 0 as libc::c_int as libc::c_char;
            j += 1;
            j;
        }
        i = 1 as libc::c_int;
        while i <= m {
            len = 0 as libc::c_int;
            rhs = 1 as libc::c_int;
            loop {
                while (*csa).c == ' ' as i32 || (*csa).c == '\n' as i32 {
                    _glp_dmx_read_char(csa);
                }
                _glp_dmx_read_field(csa);
                if _glp_str2int(((*csa).field).as_mut_ptr(), &mut j) != 0 as libc::c_int
                {
                    _glp_dmx_error(
                        csa,
                        b"variable number missing or invalid\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if j > 0 as libc::c_int {
                    neg = 0 as libc::c_int;
                } else {
                    if !(j < 0 as libc::c_int) {
                        break;
                    }
                    neg = 1 as libc::c_int;
                    j = -j;
                    rhs -= 1;
                    rhs;
                }
                if !(1 as libc::c_int <= j && j <= n) {
                    _glp_dmx_error(
                        csa,
                        b"variable number out of range\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if *map.offset(j as isize) != 0 {
                    _glp_dmx_error(
                        csa,
                        b"duplicate variable number\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                len += 1;
                len;
                *ind.offset(len as isize) = j;
                *val.offset(len as isize) = (if neg != 0 { -1.0f64 } else { 1.0f64 });
                *map.offset(j as isize) = 1 as libc::c_int as libc::c_char;
            }
            glp_set_row_bnds(P, i, 2 as libc::c_int, rhs as libc::c_double, 0.0f64);
            glp_set_mat_row(
                P,
                i,
                len,
                ind as *const libc::c_int,
                val as *const libc::c_double,
            );
            while len > 0 as libc::c_int {
                let fresh0 = len;
                len = len - 1;
                *map
                    .offset(
                        *ind.offset(fresh0 as isize) as isize,
                    ) = 0 as libc::c_int as libc::c_char;
            }
            i += 1;
            i;
        }
        glp_printf(
            b"%d lines were read\n\0" as *const u8 as *const libc::c_char,
            (*csa).count,
        );
        glp_sort_matrix(P);
    }
    if !((*csa).fp).is_null() {
        _glp_close((*csa).fp);
    }
    if !ind.is_null() {
        glp_free(ind as *mut libc::c_void);
    }
    if !val.is_null() {
        glp_free(val as *mut libc::c_void);
    }
    if !map.is_null() {
        glp_free(map as *mut libc::c_void);
    }
    if ret != 0 {
        glp_erase_prob(P);
    }
    return ret;
}
