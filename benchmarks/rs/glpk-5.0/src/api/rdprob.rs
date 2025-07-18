use ::libc;
extern "C" {
    pub type glp_file;
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
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
    fn _glp_close(f: *mut glp_file) -> libc::c_int;
    fn _glp_dmx_error(csa: *mut DMX, fmt: *const libc::c_char, _: ...);
    fn _glp_dmx_read_designator(csa: *mut DMX);
    fn _glp_dmx_read_field(csa: *mut DMX);
    fn _glp_dmx_end_of_line(csa: *mut DMX);
    fn _glp_str2int(str: *const libc::c_char, val: *mut libc::c_int) -> libc::c_int;
    fn _glp_str2num(str: *const libc::c_char, val: *mut libc::c_double) -> libc::c_int;
    fn glp_set_prob_name(P: *mut glp_prob, name: *const libc::c_char);
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
    fn glp_load_matrix(
        P: *mut glp_prob,
        ne: libc::c_int,
        ia: *const libc::c_int,
        ja: *const libc::c_int,
        ar: *const libc::c_double,
    );
    fn glp_check_dup(
        m: libc::c_int,
        n: libc::c_int,
        ne: libc::c_int,
        ia: *const libc::c_int,
        ja: *const libc::c_int,
    ) -> libc::c_int;
    fn glp_sort_matrix(P: *mut glp_prob);
    fn glp_erase_prob(P: *mut glp_prob);
    fn glp_set_col_kind(P: *mut glp_prob, j: libc::c_int, kind: libc::c_int);
    fn glp_get_num_int(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_num_bin(P: *mut glp_prob) -> libc::c_int;
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
pub unsafe extern "C" fn glp_read_prob(
    mut P: *mut glp_prob,
    mut flags: libc::c_int,
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
    let mut mip: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut nnz: libc::c_int = 0;
    let mut ne: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut kind: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut ln: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ia: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ja: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut ar: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cf: *mut libc::c_char = 0 as *mut libc::c_char;
    if flags != 0 as libc::c_int {
        (glp_error_(
            b"api/rdprob.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
        ))
            .unwrap()(
            b"glp_read_prob: flags = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            flags,
        );
    }
    if fname.is_null() {
        (glp_error_(
            b"api/rdprob.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int,
        ))
            .unwrap()(
            b"glp_read_prob: fname = %d; invalid parameter\n\0" as *const u8
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
            b"Reading problem data from '%s'...\n\0" as *const u8 as *const libc::c_char,
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
            b"lp\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            mip = 0 as libc::c_int;
        } else if strcmp(
            ((*csa).field).as_mut_ptr(),
            b"mip\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            mip = 1 as libc::c_int;
        } else {
            _glp_dmx_error(
                csa,
                b"wrong problem designator; 'lp' or 'mip' expected\0" as *const u8
                    as *const libc::c_char,
            );
        }
        _glp_dmx_read_field(csa);
        if strcmp(
            ((*csa).field).as_mut_ptr(),
            b"min\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            glp_set_obj_dir(P, 1 as libc::c_int);
        } else if strcmp(
            ((*csa).field).as_mut_ptr(),
            b"max\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            glp_set_obj_dir(P, 2 as libc::c_int);
        } else {
            _glp_dmx_error(
                csa,
                b"objective sense missing or invalid\0" as *const u8
                    as *const libc::c_char,
            );
        }
        _glp_dmx_read_field(csa);
        if !(_glp_str2int(((*csa).field).as_mut_ptr(), &mut m) == 0 as libc::c_int
            && m >= 0 as libc::c_int)
        {
            _glp_dmx_error(
                csa,
                b"number of rows missing or invalid\0" as *const u8
                    as *const libc::c_char,
            );
        }
        _glp_dmx_read_field(csa);
        if !(_glp_str2int(((*csa).field).as_mut_ptr(), &mut n) == 0 as libc::c_int
            && n >= 0 as libc::c_int)
        {
            _glp_dmx_error(
                csa,
                b"number of columns missing or invalid\0" as *const u8
                    as *const libc::c_char,
            );
        }
        _glp_dmx_read_field(csa);
        if !(_glp_str2int(((*csa).field).as_mut_ptr(), &mut nnz) == 0 as libc::c_int
            && nnz >= 0 as libc::c_int)
        {
            _glp_dmx_error(
                csa,
                b"number of constraint coefficients missing or invalid\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if m > 0 as libc::c_int {
            glp_add_rows(P, m);
            i = 1 as libc::c_int;
            while i <= m {
                glp_set_row_bnds(P, i, 5 as libc::c_int, 0.0f64, 0.0f64);
                i += 1;
                i;
            }
        }
        if n > 0 as libc::c_int {
            glp_add_cols(P, n);
            j = 1 as libc::c_int;
            while j <= n {
                if mip == 0 {
                    glp_set_col_bnds(P, j, 2 as libc::c_int, 0.0f64, 0.0f64);
                } else {
                    glp_set_col_kind(P, j, 3 as libc::c_int);
                }
                j += 1;
                j;
            }
        }
        _glp_dmx_end_of_line(csa);
        rf = glp_alloc(
            1 as libc::c_int + m,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_char;
        memset(
            rf as *mut libc::c_void,
            0 as libc::c_int,
            (1 as libc::c_int + m) as libc::c_ulong,
        );
        cf = glp_alloc(
            1 as libc::c_int + n,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_char;
        memset(
            cf as *mut libc::c_void,
            0 as libc::c_int,
            (1 as libc::c_int + n) as libc::c_ulong,
        );
        ln = glp_alloc(
            1 as libc::c_int + nnz,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        ia = glp_alloc(
            1 as libc::c_int + nnz,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        ja = glp_alloc(
            1 as libc::c_int + nnz,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        ar = glp_alloc(
            1 as libc::c_int + nnz,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        ne = 0 as libc::c_int;
        loop {
            _glp_dmx_read_designator(csa);
            if strcmp(
                ((*csa).field).as_mut_ptr(),
                b"i\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                _glp_dmx_read_field(csa);
                if _glp_str2int(((*csa).field).as_mut_ptr(), &mut i) != 0 as libc::c_int
                {
                    _glp_dmx_error(
                        csa,
                        b"row number missing or invalid\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if !(1 as libc::c_int <= i && i <= m) {
                    _glp_dmx_error(
                        csa,
                        b"row number out of range\0" as *const u8 as *const libc::c_char,
                    );
                }
                _glp_dmx_read_field(csa);
                if strcmp(
                    ((*csa).field).as_mut_ptr(),
                    b"f\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    type_0 = 1 as libc::c_int;
                } else if strcmp(
                    ((*csa).field).as_mut_ptr(),
                    b"l\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    type_0 = 2 as libc::c_int;
                } else if strcmp(
                    ((*csa).field).as_mut_ptr(),
                    b"u\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    type_0 = 3 as libc::c_int;
                } else if strcmp(
                    ((*csa).field).as_mut_ptr(),
                    b"d\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    type_0 = 4 as libc::c_int;
                } else if strcmp(
                    ((*csa).field).as_mut_ptr(),
                    b"s\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    type_0 = 5 as libc::c_int;
                } else {
                    _glp_dmx_error(
                        csa,
                        b"row type missing or invalid\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if type_0 == 2 as libc::c_int || type_0 == 4 as libc::c_int
                    || type_0 == 5 as libc::c_int
                {
                    _glp_dmx_read_field(csa);
                    if _glp_str2num(((*csa).field).as_mut_ptr(), &mut lb)
                        != 0 as libc::c_int
                    {
                        _glp_dmx_error(
                            csa,
                            b"row lower bound/fixed value missing or invalid\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                } else {
                    lb = 0.0f64;
                }
                if type_0 == 3 as libc::c_int || type_0 == 4 as libc::c_int {
                    _glp_dmx_read_field(csa);
                    if _glp_str2num(((*csa).field).as_mut_ptr(), &mut ub)
                        != 0 as libc::c_int
                    {
                        _glp_dmx_error(
                            csa,
                            b"row upper bound missing or invalid\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                } else {
                    ub = 0.0f64;
                }
                if *rf.offset(i as isize) as libc::c_int & 0x1 as libc::c_int != 0 {
                    _glp_dmx_error(
                        csa,
                        b"duplicate row descriptor\0" as *const u8 as *const libc::c_char,
                    );
                }
                glp_set_row_bnds(P, i, type_0, lb, ub);
                let ref mut fresh0 = *rf.offset(i as isize);
                *fresh0 = (*fresh0 as libc::c_int | 0x1 as libc::c_int) as libc::c_char;
            } else if strcmp(
                ((*csa).field).as_mut_ptr(),
                b"j\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                let mut current_block_140: u64;
                _glp_dmx_read_field(csa);
                if _glp_str2int(((*csa).field).as_mut_ptr(), &mut j) != 0 as libc::c_int
                {
                    _glp_dmx_error(
                        csa,
                        b"column number missing or invalid\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if !(1 as libc::c_int <= j && j <= n) {
                    _glp_dmx_error(
                        csa,
                        b"column number out of range\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if mip == 0 {
                    kind = 1 as libc::c_int;
                    current_block_140 = 6540614962658479183;
                } else {
                    _glp_dmx_read_field(csa);
                    if strcmp(
                        ((*csa).field).as_mut_ptr(),
                        b"c\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        kind = 1 as libc::c_int;
                        current_block_140 = 6540614962658479183;
                    } else if strcmp(
                        ((*csa).field).as_mut_ptr(),
                        b"i\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        kind = 2 as libc::c_int;
                        current_block_140 = 6540614962658479183;
                    } else if strcmp(
                        ((*csa).field).as_mut_ptr(),
                        b"b\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        kind = 2 as libc::c_int;
                        type_0 = 4 as libc::c_int;
                        lb = 0.0f64;
                        ub = 1.0f64;
                        current_block_140 = 11090474849799129000;
                    } else {
                        _glp_dmx_error(
                            csa,
                            b"column kind missing or invalid\0" as *const u8
                                as *const libc::c_char,
                        );
                        current_block_140 = 6540614962658479183;
                    }
                }
                match current_block_140 {
                    6540614962658479183 => {
                        _glp_dmx_read_field(csa);
                        if strcmp(
                            ((*csa).field).as_mut_ptr(),
                            b"f\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            type_0 = 1 as libc::c_int;
                        } else if strcmp(
                            ((*csa).field).as_mut_ptr(),
                            b"l\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            type_0 = 2 as libc::c_int;
                        } else if strcmp(
                            ((*csa).field).as_mut_ptr(),
                            b"u\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            type_0 = 3 as libc::c_int;
                        } else if strcmp(
                            ((*csa).field).as_mut_ptr(),
                            b"d\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            type_0 = 4 as libc::c_int;
                        } else if strcmp(
                            ((*csa).field).as_mut_ptr(),
                            b"s\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            type_0 = 5 as libc::c_int;
                        } else {
                            _glp_dmx_error(
                                csa,
                                b"column type missing or invalid\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        if type_0 == 2 as libc::c_int || type_0 == 4 as libc::c_int
                            || type_0 == 5 as libc::c_int
                        {
                            _glp_dmx_read_field(csa);
                            if _glp_str2num(((*csa).field).as_mut_ptr(), &mut lb)
                                != 0 as libc::c_int
                            {
                                _glp_dmx_error(
                                    csa,
                                    b"column lower bound/fixed value missing or invalid\0"
                                        as *const u8 as *const libc::c_char,
                                );
                            }
                        } else {
                            lb = 0.0f64;
                        }
                        if type_0 == 3 as libc::c_int || type_0 == 4 as libc::c_int {
                            _glp_dmx_read_field(csa);
                            if _glp_str2num(((*csa).field).as_mut_ptr(), &mut ub)
                                != 0 as libc::c_int
                            {
                                _glp_dmx_error(
                                    csa,
                                    b"column upper bound missing or invalid\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                        } else {
                            ub = 0.0f64;
                        }
                    }
                    _ => {}
                }
                if *cf.offset(j as isize) as libc::c_int & 0x1 as libc::c_int != 0 {
                    _glp_dmx_error(
                        csa,
                        b"duplicate column descriptor\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                glp_set_col_kind(P, j, kind);
                glp_set_col_bnds(P, j, type_0, lb, ub);
                let ref mut fresh1 = *cf.offset(j as isize);
                *fresh1 = (*fresh1 as libc::c_int | 0x1 as libc::c_int) as libc::c_char;
            } else if strcmp(
                ((*csa).field).as_mut_ptr(),
                b"a\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                _glp_dmx_read_field(csa);
                if _glp_str2int(((*csa).field).as_mut_ptr(), &mut i) != 0 as libc::c_int
                {
                    _glp_dmx_error(
                        csa,
                        b"row number missing or invalid\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if !(0 as libc::c_int <= i && i <= m) {
                    _glp_dmx_error(
                        csa,
                        b"row number out of range\0" as *const u8 as *const libc::c_char,
                    );
                }
                _glp_dmx_read_field(csa);
                if _glp_str2int(((*csa).field).as_mut_ptr(), &mut j) != 0 as libc::c_int
                {
                    _glp_dmx_error(
                        csa,
                        b"column number missing or invalid\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if !((if i == 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                }) <= j && j <= n)
                {
                    _glp_dmx_error(
                        csa,
                        b"column number out of range\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                _glp_dmx_read_field(csa);
                if i == 0 as libc::c_int {
                    if _glp_str2num(((*csa).field).as_mut_ptr(), &mut temp)
                        != 0 as libc::c_int
                    {
                        _glp_dmx_error(
                            csa,
                            b"objective %s missing or invalid\0" as *const u8
                                as *const libc::c_char,
                            if j == 0 as libc::c_int {
                                b"constant term\0" as *const u8 as *const libc::c_char
                            } else {
                                b"coefficient\0" as *const u8 as *const libc::c_char
                            },
                        );
                    }
                    if *cf.offset(j as isize) as libc::c_int & 0x10 as libc::c_int != 0 {
                        _glp_dmx_error(
                            csa,
                            b"duplicate objective %s\0" as *const u8
                                as *const libc::c_char,
                            if j == 0 as libc::c_int {
                                b"constant term\0" as *const u8 as *const libc::c_char
                            } else {
                                b"coefficient\0" as *const u8 as *const libc::c_char
                            },
                        );
                    }
                    glp_set_obj_coef(P, j, temp);
                    let ref mut fresh2 = *cf.offset(j as isize);
                    *fresh2 = (*fresh2 as libc::c_int | 0x10 as libc::c_int)
                        as libc::c_char;
                } else {
                    if _glp_str2num(((*csa).field).as_mut_ptr(), &mut temp)
                        != 0 as libc::c_int
                    {
                        _glp_dmx_error(
                            csa,
                            b"constraint coefficient missing or invalid\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    if ne == nnz {
                        _glp_dmx_error(
                            csa,
                            b"too many constraint coefficient descriptors\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    ne += 1;
                    *ln.offset(ne as isize) = (*csa).count;
                    *ia.offset(ne as isize) = i;
                    *ja.offset(ne as isize) = j;
                    *ar.offset(ne as isize) = temp;
                }
            } else if strcmp(
                ((*csa).field).as_mut_ptr(),
                b"n\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                _glp_dmx_read_field(csa);
                if strcmp(
                    ((*csa).field).as_mut_ptr(),
                    b"p\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    _glp_dmx_read_field(csa);
                    if !((*P).name).is_null() {
                        _glp_dmx_error(
                            csa,
                            b"duplicate problem name\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    glp_set_prob_name(P, ((*csa).field).as_mut_ptr());
                } else if strcmp(
                    ((*csa).field).as_mut_ptr(),
                    b"z\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    _glp_dmx_read_field(csa);
                    if !((*P).obj).is_null() {
                        _glp_dmx_error(
                            csa,
                            b"duplicate objective name\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    glp_set_obj_name(P, ((*csa).field).as_mut_ptr());
                } else if strcmp(
                    ((*csa).field).as_mut_ptr(),
                    b"i\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    _glp_dmx_read_field(csa);
                    if _glp_str2int(((*csa).field).as_mut_ptr(), &mut i)
                        != 0 as libc::c_int
                    {
                        _glp_dmx_error(
                            csa,
                            b"row number missing or invalid\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    if !(1 as libc::c_int <= i && i <= m) {
                        _glp_dmx_error(
                            csa,
                            b"row number out of range\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    _glp_dmx_read_field(csa);
                    if !((**((*P).row).offset(i as isize)).name).is_null() {
                        _glp_dmx_error(
                            csa,
                            b"duplicate row name\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    glp_set_row_name(P, i, ((*csa).field).as_mut_ptr());
                } else if strcmp(
                    ((*csa).field).as_mut_ptr(),
                    b"j\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    _glp_dmx_read_field(csa);
                    if _glp_str2int(((*csa).field).as_mut_ptr(), &mut j)
                        != 0 as libc::c_int
                    {
                        _glp_dmx_error(
                            csa,
                            b"column number missing or invalid\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    if !(1 as libc::c_int <= j && j <= n) {
                        _glp_dmx_error(
                            csa,
                            b"column number out of range\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    _glp_dmx_read_field(csa);
                    if !((**((*P).col).offset(j as isize)).name).is_null() {
                        _glp_dmx_error(
                            csa,
                            b"duplicate column name\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    glp_set_col_name(P, j, ((*csa).field).as_mut_ptr());
                } else {
                    _glp_dmx_error(
                        csa,
                        b"object designator missing or invalid\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else {
                if strcmp(
                    ((*csa).field).as_mut_ptr(),
                    b"e\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    break;
                }
                _glp_dmx_error(
                    csa,
                    b"line designator missing or invalid\0" as *const u8
                        as *const libc::c_char,
                );
            }
            _glp_dmx_end_of_line(csa);
        }
        if ne < nnz {
            _glp_dmx_error(
                csa,
                b"too few constraint coefficient descriptors\0" as *const u8
                    as *const libc::c_char,
            );
        }
        (ne == nnz
            || {
                glp_assert_(
                    b"ne == nnz\0" as *const u8 as *const libc::c_char,
                    b"api/rdprob.c\0" as *const u8 as *const libc::c_char,
                    322 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        k = glp_check_dup(m, n, ne, ia as *const libc::c_int, ja as *const libc::c_int);
        (0 as libc::c_int <= k && k <= nnz
            || {
                glp_assert_(
                    b"0 <= k && k <= nnz\0" as *const u8 as *const libc::c_char,
                    b"api/rdprob.c\0" as *const u8 as *const libc::c_char,
                    324 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if k > 0 as libc::c_int {
            (*csa).count = *ln.offset(k as isize);
            _glp_dmx_error(
                csa,
                b"duplicate constraint coefficient\0" as *const u8 as *const libc::c_char,
            );
        }
        glp_load_matrix(
            P,
            ne,
            ia as *const libc::c_int,
            ja as *const libc::c_int,
            ar as *const libc::c_double,
        );
        if !((*P).name).is_null() {
            glp_printf(
                b"Problem: %s\n\0" as *const u8 as *const libc::c_char,
                (*P).name,
            );
        }
        if !((*P).obj).is_null() {
            glp_printf(
                b"Objective: %s\n\0" as *const u8 as *const libc::c_char,
                (*P).obj,
            );
        }
        glp_printf(
            b"%d row%s, %d column%s, %d non-zero%s\n\0" as *const u8
                as *const libc::c_char,
            m,
            if m == 1 as libc::c_int {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"s\0" as *const u8 as *const libc::c_char
            },
            n,
            if n == 1 as libc::c_int {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"s\0" as *const u8 as *const libc::c_char
            },
            nnz,
            if nnz == 1 as libc::c_int {
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
                        b"One variable is binary\n\0" as *const u8 as *const libc::c_char,
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
            (*csa).count,
        );
        glp_sort_matrix(P);
        ret = 0 as libc::c_int;
    }
    if !((*csa).fp).is_null() {
        _glp_close((*csa).fp);
    }
    if !rf.is_null() {
        glp_free(rf as *mut libc::c_void);
    }
    if !cf.is_null() {
        glp_free(cf as *mut libc::c_void);
    }
    if !ln.is_null() {
        glp_free(ln as *mut libc::c_void);
    }
    if !ia.is_null() {
        glp_free(ia as *mut libc::c_void);
    }
    if !ja.is_null() {
        glp_free(ja as *mut libc::c_void);
    }
    if !ar.is_null() {
        glp_free(ar as *mut libc::c_void);
    }
    if ret != 0 {
        glp_erase_prob(P);
    }
    return ret;
}
