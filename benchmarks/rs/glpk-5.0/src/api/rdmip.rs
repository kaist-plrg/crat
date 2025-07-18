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
    fn _glp_dmx_error(csa: *mut DMX, fmt: *const libc::c_char, _: ...);
    fn _glp_close(f: *mut glp_file) -> libc::c_int;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn _glp_open(name: *const libc::c_char, mode: *const libc::c_char) -> *mut glp_file;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn _glp_get_err_msg() -> *const libc::c_char;
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn _glp_str2int(str: *const libc::c_char, val: *mut libc::c_int) -> libc::c_int;
    fn _glp_str2num(str: *const libc::c_char, val: *mut libc::c_double) -> libc::c_int;
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
pub unsafe extern "C" fn glp_read_mip(
    mut P: *mut glp_prob,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut dmx_: DMX = DMX {
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
    let mut dmx: *mut DMX = &mut dmx_;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut sst: libc::c_int = 0;
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut stat: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut obj: libc::c_double = 0.;
    let mut prim: *mut libc::c_double = 0 as *mut libc::c_double;
    if fname.is_null() {
        (glp_error_(
            b"api/rdmip.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int,
        ))
            .unwrap()(
            b"glp_read_mip: fname = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            fname,
        );
    }
    if !(_setjmp(((*dmx).jump).as_mut_ptr()) != 0) {
        (*dmx).fname = fname;
        (*dmx).fp = 0 as *mut glp_file;
        (*dmx).count = 0 as libc::c_int;
        (*dmx).c = '\n' as i32;
        (*dmx).field[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        (*dmx).nonint = 0 as libc::c_int;
        (*dmx).empty = (*dmx).nonint;
        glp_printf(
            b"Reading MIP solution from '%s'...\n\0" as *const u8 as *const libc::c_char,
            fname,
        );
        (*dmx).fp = _glp_open(fname, b"r\0" as *const u8 as *const libc::c_char);
        if ((*dmx).fp).is_null() {
            glp_printf(
                b"Unable to open '%s' - %s\n\0" as *const u8 as *const libc::c_char,
                fname,
                _glp_get_err_msg(),
            );
        } else {
            _glp_dmx_read_designator(dmx);
            if strcmp(
                ((*dmx).field).as_mut_ptr(),
                b"s\0" as *const u8 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                _glp_dmx_error(
                    dmx,
                    b"solution line missing or invalid\0" as *const u8
                        as *const libc::c_char,
                );
            }
            _glp_dmx_read_field(dmx);
            if strcmp(
                ((*dmx).field).as_mut_ptr(),
                b"mip\0" as *const u8 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                _glp_dmx_error(
                    dmx,
                    b"wrong solution designator; 'mip' expected\0" as *const u8
                        as *const libc::c_char,
                );
            }
            _glp_dmx_read_field(dmx);
            if !(_glp_str2int(((*dmx).field).as_mut_ptr(), &mut m) == 0 as libc::c_int
                && m >= 0 as libc::c_int)
            {
                _glp_dmx_error(
                    dmx,
                    b"number of rows missing or invalid\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if m != (*P).m {
                _glp_dmx_error(
                    dmx,
                    b"number of rows mismatch\0" as *const u8 as *const libc::c_char,
                );
            }
            _glp_dmx_read_field(dmx);
            if !(_glp_str2int(((*dmx).field).as_mut_ptr(), &mut n) == 0 as libc::c_int
                && n >= 0 as libc::c_int)
            {
                _glp_dmx_error(
                    dmx,
                    b"number of columns missing or invalid\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if n != (*P).n {
                _glp_dmx_error(
                    dmx,
                    b"number of columns mismatch\0" as *const u8 as *const libc::c_char,
                );
            }
            _glp_dmx_read_field(dmx);
            if strcmp(
                ((*dmx).field).as_mut_ptr(),
                b"o\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                sst = 5 as libc::c_int;
            } else if strcmp(
                ((*dmx).field).as_mut_ptr(),
                b"f\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                sst = 2 as libc::c_int;
            } else if strcmp(
                ((*dmx).field).as_mut_ptr(),
                b"n\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                sst = 4 as libc::c_int;
            } else if strcmp(
                ((*dmx).field).as_mut_ptr(),
                b"u\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                sst = 1 as libc::c_int;
            } else {
                _glp_dmx_error(
                    dmx,
                    b"solution status missing or invalid\0" as *const u8
                        as *const libc::c_char,
                );
            }
            _glp_dmx_read_field(dmx);
            if _glp_str2num(((*dmx).field).as_mut_ptr(), &mut obj) != 0 as libc::c_int {
                _glp_dmx_error(
                    dmx,
                    b"objective value missing or invalid\0" as *const u8
                        as *const libc::c_char,
                );
            }
            _glp_dmx_end_of_line(dmx);
            stat = glp_alloc(
                1 as libc::c_int + m + n,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
            ) as *mut libc::c_char;
            k = 1 as libc::c_int;
            while k <= m + n {
                *stat.offset(k as isize) = '?' as i32 as libc::c_char;
                k += 1;
                k;
            }
            prim = glp_alloc(
                1 as libc::c_int + m + n,
                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
            ) as *mut libc::c_double;
            loop {
                _glp_dmx_read_designator(dmx);
                if strcmp(
                    ((*dmx).field).as_mut_ptr(),
                    b"i\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    _glp_dmx_read_field(dmx);
                    if _glp_str2int(((*dmx).field).as_mut_ptr(), &mut i)
                        != 0 as libc::c_int
                    {
                        _glp_dmx_error(
                            dmx,
                            b"row number missing or invalid\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    if !(1 as libc::c_int <= i && i <= m) {
                        _glp_dmx_error(
                            dmx,
                            b"row number out of range\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    if *stat.offset(i as isize) as libc::c_int != '?' as i32 {
                        _glp_dmx_error(
                            dmx,
                            b"duplicate row solution descriptor\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    *stat.offset(i as isize) = 1 as libc::c_int as libc::c_char;
                    _glp_dmx_read_field(dmx);
                    if _glp_str2num(
                        ((*dmx).field).as_mut_ptr(),
                        &mut *prim.offset(i as isize),
                    ) != 0 as libc::c_int
                    {
                        _glp_dmx_error(
                            dmx,
                            b"row value missing or invalid\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    _glp_dmx_end_of_line(dmx);
                } else if strcmp(
                    ((*dmx).field).as_mut_ptr(),
                    b"j\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    _glp_dmx_read_field(dmx);
                    if _glp_str2int(((*dmx).field).as_mut_ptr(), &mut j)
                        != 0 as libc::c_int
                    {
                        _glp_dmx_error(
                            dmx,
                            b"column number missing or invalid\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    if !(1 as libc::c_int <= j && j <= n) {
                        _glp_dmx_error(
                            dmx,
                            b"column number out of range\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    if *stat.offset((m + j) as isize) as libc::c_int != '?' as i32 {
                        _glp_dmx_error(
                            dmx,
                            b"duplicate column solution descriptor\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    *stat.offset((m + j) as isize) = 1 as libc::c_int as libc::c_char;
                    _glp_dmx_read_field(dmx);
                    if _glp_str2num(
                        ((*dmx).field).as_mut_ptr(),
                        &mut *prim.offset((m + j) as isize),
                    ) != 0 as libc::c_int
                    {
                        _glp_dmx_error(
                            dmx,
                            b"column value missing or invalid\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    _glp_dmx_end_of_line(dmx);
                } else {
                    if strcmp(
                        ((*dmx).field).as_mut_ptr(),
                        b"e\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        break;
                    }
                    _glp_dmx_error(
                        dmx,
                        b"line designator missing or invalid\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                _glp_dmx_end_of_line(dmx);
            }
            k = 1 as libc::c_int;
            while k <= m + n {
                if *stat.offset(k as isize) as libc::c_int == '?' as i32 {
                    _glp_dmx_error(
                        dmx,
                        b"incomplete MIP solution\0" as *const u8 as *const libc::c_char,
                    );
                }
                k += 1;
                k;
            }
            (*P).mip_stat = sst;
            (*P).mip_obj = obj;
            i = 1 as libc::c_int;
            while i <= m {
                (**((*P).row).offset(i as isize)).mipx = *prim.offset(i as isize);
                i += 1;
                i;
            }
            j = 1 as libc::c_int;
            while j <= n {
                (**((*P).col).offset(j as isize)).mipx = *prim.offset((m + j) as isize);
                j += 1;
                j;
            }
            glp_printf(
                b"%d lines were read\n\0" as *const u8 as *const libc::c_char,
                (*dmx).count,
            );
            ret = 0 as libc::c_int;
        }
    }
    if !((*dmx).fp).is_null() {
        _glp_close((*dmx).fp);
    }
    if !stat.is_null() {
        glp_free(stat as *mut libc::c_void);
    }
    if !prim.is_null() {
        glp_free(prim as *mut libc::c_void);
    }
    return ret;
}
