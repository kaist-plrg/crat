use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn _glp_get_env_ptr() -> *mut ENV;
    fn glp_create_prob() -> *mut glp_prob;
    fn glp_delete_prob(P: *mut glp_prob);
    fn glp_get_mat_col(
        P: *mut glp_prob,
        j: libc::c_int,
        ind: *mut libc::c_int,
        val: *mut libc::c_double,
    ) -> libc::c_int;
    fn glp_scale_prob(P: *mut glp_prob, flags: libc::c_int);
    fn _glp_ipm_solve(P: *mut glp_prob, parm: *const glp_iptcp) -> libc::c_int;
    fn _glp_npp_create_wksp() -> *mut NPP;
    fn _glp_npp_load_prob(
        npp: *mut NPP,
        orig: *mut glp_prob,
        names: libc::c_int,
        sol: libc::c_int,
        scaling: libc::c_int,
    );
    fn _glp_npp_build_prob(npp: *mut NPP, prob: *mut glp_prob);
    fn _glp_npp_postprocess(npp: *mut NPP, prob: *mut glp_prob);
    fn _glp_npp_unload_sol(npp: *mut NPP, orig: *mut glp_prob);
    fn _glp_npp_delete_wksp(npp: *mut NPP);
    fn _glp_npp_free_row(npp: *mut NPP, p: *mut NPPROW);
    fn _glp_npp_geq_row(npp: *mut NPP, p: *mut NPPROW);
    fn _glp_npp_leq_row(npp: *mut NPP, p: *mut NPPROW);
    fn _glp_npp_free_col(npp: *mut NPP, q: *mut NPPCOL);
    fn _glp_npp_lbnd_col(npp: *mut NPP, q: *mut NPPCOL);
    fn _glp_npp_ubnd_col(npp: *mut NPP, q: *mut NPPCOL);
    fn _glp_npp_dbnd_col(npp: *mut NPP, q: *mut NPPCOL);
    fn _glp_npp_fixed_col(npp: *mut NPP, q: *mut NPPCOL);
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ENV {
    pub self_0: *mut ENV,
    pub term_buf: *mut libc::c_char,
    pub term_out: libc::c_int,
    pub term_hook: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int,
    >,
    pub term_info: *mut libc::c_void,
    pub tee_file: *mut FILE,
    pub err_st: libc::c_int,
    pub err_file: *const libc::c_char,
    pub err_line: libc::c_int,
    pub err_hook: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub err_info: *mut libc::c_void,
    pub err_buf: *mut libc::c_char,
    pub mem_limit: size_t,
    pub mem_ptr: *mut MBD,
    pub mem_count: libc::c_int,
    pub mem_cpeak: libc::c_int,
    pub mem_total: size_t,
    pub mem_tpeak: size_t,
    pub gmp_pool: *mut libc::c_void,
    pub gmp_size: libc::c_int,
    pub gmp_work: *mut libc::c_ushort,
    pub h_odbc: *mut libc::c_void,
    pub h_mysql: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MBD {
    pub size: size_t,
    pub self_0: *mut MBD,
    pub prev: *mut MBD,
    pub next: *mut MBD,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_iptcp {
    pub msg_lev: libc::c_int,
    pub ord_alg: libc::c_int,
    pub foo_bar: [libc::c_double; 48],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_prep {
    pub orig_dir: libc::c_int,
    pub orig_m: libc::c_int,
    pub orig_n: libc::c_int,
    pub orig_nnz: libc::c_int,
    pub pool: *mut DMP,
    pub name: *mut libc::c_char,
    pub obj: *mut libc::c_char,
    pub c0: libc::c_double,
    pub nrows: libc::c_int,
    pub ncols: libc::c_int,
    pub r_head: *mut NPPROW,
    pub r_tail: *mut NPPROW,
    pub c_head: *mut NPPCOL,
    pub c_tail: *mut NPPCOL,
    pub stack: *mut DMP,
    pub top: *mut NPPTSE,
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub nnz: libc::c_int,
    pub row_ref: *mut libc::c_int,
    pub col_ref: *mut libc::c_int,
    pub sol: libc::c_int,
    pub scaling: libc::c_int,
    pub p_stat: libc::c_int,
    pub d_stat: libc::c_int,
    pub t_stat: libc::c_int,
    pub i_stat: libc::c_int,
    pub r_stat: *mut libc::c_char,
    pub c_stat: *mut libc::c_char,
    pub r_pi: *mut libc::c_double,
    pub c_value: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPTSE {
    pub func: Option::<unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> libc::c_int>,
    pub info: *mut libc::c_void,
    pub link: *mut NPPTSE,
}
pub type NPP = glp_prep;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPCOL {
    pub j: libc::c_int,
    pub name: *mut libc::c_char,
    pub is_int: libc::c_char,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub coef: libc::c_double,
    pub ptr: *mut NPPAIJ,
    pub temp: libc::c_int,
    pub ll: C2RustUnnamed_0,
    pub uu: C2RustUnnamed,
    pub prev: *mut NPPCOL,
    pub next: *mut NPPCOL,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub uu: libc::c_double,
    pub neg: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ll: libc::c_double,
    pub pos: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPAIJ {
    pub row: *mut NPPROW,
    pub col: *mut NPPCOL,
    pub val: libc::c_double,
    pub r_prev: *mut NPPAIJ,
    pub r_next: *mut NPPAIJ,
    pub c_prev: *mut NPPAIJ,
    pub c_next: *mut NPPAIJ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPROW {
    pub i: libc::c_int,
    pub name: *mut libc::c_char,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut NPPAIJ,
    pub temp: libc::c_int,
    pub prev: *mut NPPROW,
    pub next: *mut NPPROW,
}
unsafe extern "C" fn transform(mut npp: *mut NPP) {
    let mut row: *mut NPPROW = 0 as *mut NPPROW;
    let mut prev_row: *mut NPPROW = 0 as *mut NPPROW;
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut prev_col: *mut NPPCOL = 0 as *mut NPPCOL;
    row = (*npp).r_tail;
    while !row.is_null() {
        prev_row = (*row).prev;
        if (*row).lb == -1.7976931348623157e+308f64
            && (*row).ub == 1.7976931348623157e+308f64
        {
            _glp_npp_free_row(npp, row);
        } else if (*row).lb == -1.7976931348623157e+308f64 {
            _glp_npp_leq_row(npp, row);
        } else if (*row).ub == 1.7976931348623157e+308f64 {
            _glp_npp_geq_row(npp, row);
        } else if (*row).lb != (*row).ub {
            if fabs((*row).lb) < fabs((*row).ub) {
                _glp_npp_geq_row(npp, row);
            } else {
                _glp_npp_leq_row(npp, row);
            }
        }
        row = prev_row;
    }
    col = (*npp).c_tail;
    while !col.is_null() {
        prev_col = (*col).prev;
        if (*col).lb == -1.7976931348623157e+308f64
            && (*col).ub == 1.7976931348623157e+308f64
        {
            _glp_npp_free_col(npp, col);
        } else if (*col).lb == -1.7976931348623157e+308f64 {
            _glp_npp_ubnd_col(npp, col);
        } else if (*col).ub == 1.7976931348623157e+308f64 {
            if (*col).lb != 0.0f64 {
                _glp_npp_lbnd_col(npp, col);
            }
        } else if (*col).lb != (*col).ub {
            if fabs((*col).lb) < fabs((*col).ub) {
                if (*col).lb != 0.0f64 {
                    _glp_npp_lbnd_col(npp, col);
                }
            } else {
                _glp_npp_ubnd_col(npp, col);
            }
            _glp_npp_dbnd_col(npp, col);
        } else {
            _glp_npp_fixed_col(npp, col);
        }
        col = prev_col;
    }
    row = (*npp).r_head;
    while !row.is_null() {
        ((*row).lb == (*row).ub
            || {
                glp_assert_(
                    b"row->lb == row->ub\0" as *const u8 as *const libc::c_char,
                    b"draft/glpapi08.c\0" as *const u8 as *const libc::c_char,
                    111 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        row = (*row).next;
    }
    col = (*npp).c_head;
    while !col.is_null() {
        ((*col).lb == 0.0f64 && (*col).ub == 1.7976931348623157e+308f64
            || {
                glp_assert_(
                    b"col->lb == 0.0 && col->ub == +DBL_MAX\0" as *const u8
                        as *const libc::c_char,
                    b"draft/glpapi08.c\0" as *const u8 as *const libc::c_char,
                    113 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        col = (*col).next;
    }
}
pub unsafe extern "C" fn glp_interior(
    mut P: *mut glp_prob,
    mut parm: *const glp_iptcp,
) -> libc::c_int {
    let mut current_block: u64;
    let mut _parm: glp_iptcp = glp_iptcp {
        msg_lev: 0,
        ord_alg: 0,
        foo_bar: [0.; 48],
    };
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut npp: *mut NPP = 0 as *mut NPP;
    let mut prob: *mut glp_prob = 0 as *mut glp_prob;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if parm.is_null() {
        glp_init_iptcp(&mut _parm);
        parm = &mut _parm;
    }
    if !((*parm).msg_lev == 0 as libc::c_int || (*parm).msg_lev == 1 as libc::c_int
        || (*parm).msg_lev == 2 as libc::c_int || (*parm).msg_lev == 3 as libc::c_int)
    {
        (glp_error_(
            b"draft/glpapi08.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
        ))
            .unwrap()(
            b"glp_interior: msg_lev = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).msg_lev,
        );
    }
    if !((*parm).ord_alg == 0 as libc::c_int || (*parm).ord_alg == 1 as libc::c_int
        || (*parm).ord_alg == 2 as libc::c_int || (*parm).ord_alg == 3 as libc::c_int)
    {
        (glp_error_(
            b"draft/glpapi08.c\0" as *const u8 as *const libc::c_char,
            137 as libc::c_int,
        ))
            .unwrap()(
            b"glp_interior: ord_alg = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).ord_alg,
        );
    }
    (*P).ipt_stat = 1 as libc::c_int;
    (*P).ipt_obj = 0.0f64;
    i = 1 as libc::c_int;
    loop {
        if !(i <= (*P).m) {
            current_block = 7976072742316086414;
            break;
        }
        row = *((*P).row).offset(i as isize);
        if (*row).type_0 == 4 as libc::c_int && (*row).lb >= (*row).ub {
            if (*parm).msg_lev >= 1 as libc::c_int {
                glp_printf(
                    b"glp_interior: row %d: lb = %g, ub = %g; incorrect bounds\n\0"
                        as *const u8 as *const libc::c_char,
                    i,
                    (*row).lb,
                    (*row).ub,
                );
            }
            ret = 0x4 as libc::c_int;
            current_block = 3853918948275168412;
            break;
        } else {
            i += 1;
            i;
        }
    }
    match current_block {
        7976072742316086414 => {
            j = 1 as libc::c_int;
            loop {
                if !(j <= (*P).n) {
                    current_block = 5783071609795492627;
                    break;
                }
                col = *((*P).col).offset(j as isize);
                if (*col).type_0 == 4 as libc::c_int && (*col).lb >= (*col).ub {
                    if (*parm).msg_lev >= 1 as libc::c_int {
                        glp_printf(
                            b"glp_interior: column %d: lb = %g, ub = %g; incorrect bounds\n\0"
                                as *const u8 as *const libc::c_char,
                            j,
                            (*col).lb,
                            (*col).ub,
                        );
                    }
                    ret = 0x4 as libc::c_int;
                    current_block = 3853918948275168412;
                    break;
                } else {
                    j += 1;
                    j;
                }
            }
            match current_block {
                3853918948275168412 => {}
                _ => {
                    if (*parm).msg_lev >= 3 as libc::c_int {
                        glp_printf(
                            b"Original LP has %d row(s), %d column(s), and %d non-zero(s)\n\0"
                                as *const u8 as *const libc::c_char,
                            (*P).m,
                            (*P).n,
                            (*P).nnz,
                        );
                    }
                    npp = _glp_npp_create_wksp();
                    _glp_npp_load_prob(
                        npp,
                        P,
                        0 as libc::c_int,
                        2 as libc::c_int,
                        1 as libc::c_int,
                    );
                    transform(npp);
                    prob = glp_create_prob();
                    _glp_npp_build_prob(npp, prob);
                    if (*parm).msg_lev >= 3 as libc::c_int {
                        glp_printf(
                            b"Working LP has %d row(s), %d column(s), and %d non-zero(s)\n\0"
                                as *const u8 as *const libc::c_char,
                            (*prob).m,
                            (*prob).n,
                            (*prob).nnz,
                        );
                    }
                    if !((*prob).m > 0 as libc::c_int && (*prob).n > 0 as libc::c_int) {
                        if (*parm).msg_lev >= 1 as libc::c_int {
                            glp_printf(
                                b"glp_interior: unable to solve empty problem\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        ret = 0x5 as libc::c_int;
                    } else {
                        let mut env: *mut ENV = _glp_get_env_ptr();
                        let mut term_out: libc::c_int = (*env).term_out;
                        (*env).term_out = 0 as libc::c_int;
                        glp_scale_prob(prob, 0x10 as libc::c_int);
                        (*env).term_out = term_out;
                        if (*parm).msg_lev >= 2 as libc::c_int
                            && (*prob).m >= 200 as libc::c_int
                        {
                            let mut len: libc::c_int = 0;
                            let mut cnt: libc::c_int = 0 as libc::c_int;
                            j = 1 as libc::c_int;
                            while j <= (*prob).n {
                                len = glp_get_mat_col(
                                    prob,
                                    j,
                                    0 as *mut libc::c_int,
                                    0 as *mut libc::c_double,
                                );
                                if len as libc::c_double
                                    >= 0.20f64 * (*prob).m as libc::c_double
                                {
                                    cnt += 1;
                                    cnt;
                                }
                                j += 1;
                                j;
                            }
                            if cnt == 1 as libc::c_int {
                                glp_printf(
                                    b"WARNING: PROBLEM HAS ONE DENSE COLUMN\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                            } else if cnt > 0 as libc::c_int {
                                glp_printf(
                                    b"WARNING: PROBLEM HAS %d DENSE COLUMNS\n\0" as *const u8
                                        as *const libc::c_char,
                                    cnt,
                                );
                            }
                        }
                        ret = _glp_ipm_solve(prob, parm);
                        _glp_npp_postprocess(npp, prob);
                        _glp_npp_unload_sol(npp, P);
                    }
                }
            }
        }
        _ => {}
    }
    if !npp.is_null() {
        _glp_npp_delete_wksp(npp);
    }
    if !prob.is_null() {
        glp_delete_prob(prob);
    }
    return ret;
}
pub unsafe extern "C" fn glp_init_iptcp(mut parm: *mut glp_iptcp) {
    (*parm).msg_lev = 3 as libc::c_int;
    (*parm).ord_alg = 2 as libc::c_int;
}
pub unsafe extern "C" fn glp_ipt_status(mut lp: *mut glp_prob) -> libc::c_int {
    let mut ipt_stat: libc::c_int = (*lp).ipt_stat;
    return ipt_stat;
}
pub unsafe extern "C" fn glp_ipt_obj_val(mut lp: *mut glp_prob) -> libc::c_double {
    let mut z: libc::c_double = 0.;
    z = (*lp).ipt_obj;
    return z;
}
pub unsafe extern "C" fn glp_ipt_row_prim(
    mut lp: *mut glp_prob,
    mut i: libc::c_int,
) -> libc::c_double {
    let mut pval: libc::c_double = 0.;
    if !(1 as libc::c_int <= i && i <= (*lp).m) {
        (glp_error_(
            b"draft/glpapi08.c\0" as *const u8 as *const libc::c_char,
            303 as libc::c_int,
        ))
            .unwrap()(
            b"glp_ipt_row_prim: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    pval = (**((*lp).row).offset(i as isize)).pval;
    return pval;
}
pub unsafe extern "C" fn glp_ipt_row_dual(
    mut lp: *mut glp_prob,
    mut i: libc::c_int,
) -> libc::c_double {
    let mut dval: libc::c_double = 0.;
    if !(1 as libc::c_int <= i && i <= (*lp).m) {
        (glp_error_(
            b"draft/glpapi08.c\0" as *const u8 as *const libc::c_char,
            328 as libc::c_int,
        ))
            .unwrap()(
            b"glp_ipt_row_dual: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    dval = (**((*lp).row).offset(i as isize)).dval;
    return dval;
}
pub unsafe extern "C" fn glp_ipt_col_prim(
    mut lp: *mut glp_prob,
    mut j: libc::c_int,
) -> libc::c_double {
    let mut pval: libc::c_double = 0.;
    if !(1 as libc::c_int <= j && j <= (*lp).n) {
        (glp_error_(
            b"draft/glpapi08.c\0" as *const u8 as *const libc::c_char,
            353 as libc::c_int,
        ))
            .unwrap()(
            b"glp_ipt_col_prim: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    pval = (**((*lp).col).offset(j as isize)).pval;
    return pval;
}
pub unsafe extern "C" fn glp_ipt_col_dual(
    mut lp: *mut glp_prob,
    mut j: libc::c_int,
) -> libc::c_double {
    let mut dval: libc::c_double = 0.;
    if !(1 as libc::c_int <= j && j <= (*lp).n) {
        (glp_error_(
            b"draft/glpapi08.c\0" as *const u8 as *const libc::c_char,
            378 as libc::c_int,
        ))
            .unwrap()(
            b"glp_ipt_col_dual: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    dval = (**((*lp).col).offset(j as isize)).dval;
    return dval;
}
