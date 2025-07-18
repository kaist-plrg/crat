use ::libc;
extern "C" {
    pub type DMP;
    pub type glp_tree;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn _glp_npp_activate_row(npp: *mut NPP, row: *mut NPPROW);
    fn _glp_npp_deactivate_row(npp: *mut NPP, row: *mut NPPROW);
    fn _glp_npp_activate_col(npp: *mut NPP, col: *mut NPPCOL);
    fn _glp_npp_deactivate_col(npp: *mut NPP, col: *mut NPPCOL);
    fn _glp_npp_reduce_ineq_coef(npp: *mut NPP, row: *mut NPPROW) -> libc::c_int;
    fn _glp_npp_hidden_covering(npp: *mut NPP, row: *mut NPPROW) -> libc::c_int;
    fn _glp_npp_hidden_packing(npp: *mut NPP, row: *mut NPPROW) -> libc::c_int;
    fn _glp_npp_binarize_prob(npp: *mut NPP) -> libc::c_int;
    fn _glp_npp_implied_bounds(npp: *mut NPP, p: *mut NPPROW);
    fn _glp_npp_inactive_bound(npp: *mut NPP, p: *mut NPPROW, which: libc::c_int);
    fn _glp_npp_analyze_row(npp: *mut NPP, p: *mut NPPROW) -> libc::c_int;
    fn _glp_npp_forcing_row(
        npp: *mut NPP,
        p: *mut NPPROW,
        at: libc::c_int,
    ) -> libc::c_int;
    fn _glp_npp_implied_free(npp: *mut NPP, q: *mut NPPCOL) -> libc::c_int;
    fn _glp_npp_implied_slack(npp: *mut NPP, q: *mut NPPCOL);
    fn _glp_npp_ineq_singlet(npp: *mut NPP, p: *mut NPPROW) -> libc::c_int;
    fn _glp_npp_implied_upper(
        npp: *mut NPP,
        q: *mut NPPCOL,
        u: libc::c_double,
    ) -> libc::c_int;
    fn _glp_npp_implied_lower(
        npp: *mut NPP,
        q: *mut NPPCOL,
        l: libc::c_double,
    ) -> libc::c_int;
    fn _glp_npp_eq_singlet(npp: *mut NPP, p: *mut NPPROW) -> libc::c_int;
    fn _glp_npp_empty_col(npp: *mut NPP, q: *mut NPPCOL) -> libc::c_int;
    fn _glp_npp_empty_row(npp: *mut NPP, p: *mut NPPROW) -> libc::c_int;
    fn _glp_npp_make_fixed(npp: *mut NPP, q: *mut NPPCOL) -> libc::c_int;
    fn _glp_npp_make_equality(npp: *mut NPP, p: *mut NPPROW) -> libc::c_int;
    fn _glp_npp_fixed_col(npp: *mut NPP, q: *mut NPPCOL);
    fn _glp_npp_free_row(npp: *mut NPP, p: *mut NPPROW);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_smcp {
    pub msg_lev: libc::c_int,
    pub meth: libc::c_int,
    pub pricing: libc::c_int,
    pub r_test: libc::c_int,
    pub tol_bnd: libc::c_double,
    pub tol_dj: libc::c_double,
    pub tol_piv: libc::c_double,
    pub obj_ll: libc::c_double,
    pub obj_ul: libc::c_double,
    pub it_lim: libc::c_int,
    pub tm_lim: libc::c_int,
    pub out_frq: libc::c_int,
    pub out_dly: libc::c_int,
    pub presolve: libc::c_int,
    pub excl: libc::c_int,
    pub shift: libc::c_int,
    pub aorn: libc::c_int,
    pub foo_bar: [libc::c_double; 33],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_iocp {
    pub msg_lev: libc::c_int,
    pub br_tech: libc::c_int,
    pub bt_tech: libc::c_int,
    pub tol_int: libc::c_double,
    pub tol_obj: libc::c_double,
    pub tm_lim: libc::c_int,
    pub out_frq: libc::c_int,
    pub out_dly: libc::c_int,
    pub cb_func: Option::<unsafe extern "C" fn(*mut glp_tree, *mut libc::c_void) -> ()>,
    pub cb_info: *mut libc::c_void,
    pub cb_size: libc::c_int,
    pub pp_tech: libc::c_int,
    pub mip_gap: libc::c_double,
    pub mir_cuts: libc::c_int,
    pub gmi_cuts: libc::c_int,
    pub cov_cuts: libc::c_int,
    pub clq_cuts: libc::c_int,
    pub presolve: libc::c_int,
    pub binarize: libc::c_int,
    pub fp_heur: libc::c_int,
    pub ps_heur: libc::c_int,
    pub ps_tm_lim: libc::c_int,
    pub sr_heur: libc::c_int,
    pub use_sol: libc::c_int,
    pub save_sol: *const libc::c_char,
    pub alien: libc::c_int,
    pub flip: libc::c_int,
    pub foo_bar: [libc::c_double; 23],
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
pub unsafe extern "C" fn _glp_npp_clean_prob(mut npp: *mut NPP) {
    let mut row: *mut NPPROW = 0 as *mut NPPROW;
    let mut next_row: *mut NPPROW = 0 as *mut NPPROW;
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut next_col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut ret: libc::c_int = 0;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const libc::c_char,
                b"npp/npp5.c\0" as *const u8 as *const libc::c_char,
                59 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    row = (*npp).r_head;
    while !row.is_null() {
        next_row = (*row).next;
        if (*row).lb == -1.7976931348623157e+308f64
            && (*row).ub == 1.7976931348623157e+308f64
        {
            _glp_npp_free_row(npp, row);
        }
        row = next_row;
    }
    row = (*npp).r_head;
    while !row.is_null() {
        next_row = (*row).next;
        if (*row).lb != -1.7976931348623157e+308f64
            && (*row).ub != 1.7976931348623157e+308f64 && (*row).lb < (*row).ub
        {
            ret = _glp_npp_make_equality(npp, row);
            if !(ret == 0 as libc::c_int) {
                if !(ret == 1 as libc::c_int) {
                    (ret != ret
                        || {
                            glp_assert_(
                                b"ret != ret\0" as *const u8 as *const libc::c_char,
                                b"npp/npp5.c\0" as *const u8 as *const libc::c_char,
                                87 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
            }
        }
        row = next_row;
    }
    col = (*npp).c_head;
    while !col.is_null() {
        next_col = (*col).next;
        if (*col).lb == (*col).ub {
            _glp_npp_fixed_col(npp, col);
        }
        col = next_col;
    }
    col = (*npp).c_head;
    while !col.is_null() {
        next_col = (*col).next;
        if (*col).lb != -1.7976931348623157e+308f64
            && (*col).ub != 1.7976931348623157e+308f64 && (*col).lb < (*col).ub
        {
            ret = _glp_npp_make_fixed(npp, col);
            if !(ret == 0 as libc::c_int) {
                if ret == 1 as libc::c_int {
                    _glp_npp_fixed_col(npp, col);
                }
            }
        }
        col = next_col;
    }
}
pub unsafe extern "C" fn _glp_npp_process_row(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
    mut hard: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut next_aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut aaa: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut ret: libc::c_int = 0;
    (!((*row).lb == -1.7976931348623157e+308f64
        && (*row).ub == 1.7976931348623157e+308f64)
        || {
            glp_assert_(
                b"!(row->lb == -DBL_MAX && row->ub == +DBL_MAX)\0" as *const u8
                    as *const libc::c_char,
                b"npp/npp5.c\0" as *const u8 as *const libc::c_char,
                175 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if ((*row).ptr).is_null() {
        ret = _glp_npp_empty_row(npp, row);
        if ret == 0 as libc::c_int {
            return 0 as libc::c_int
        } else if ret == 1 as libc::c_int {
            return 0xa as libc::c_int
        } else {
            (ret != ret
                || {
                    glp_assert_(
                        b"ret != ret\0" as *const u8 as *const libc::c_char,
                        b"npp/npp5.c\0" as *const u8 as *const libc::c_char,
                        192 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    if ((*(*row).ptr).r_next).is_null() {
        col = (*(*row).ptr).col;
        if (*row).lb == (*row).ub {
            ret = _glp_npp_eq_singlet(npp, row);
            if ret == 0 as libc::c_int {
                aij = (*col).ptr;
                while !aij.is_null() {
                    _glp_npp_activate_row(npp, (*aij).row);
                    aij = (*aij).c_next;
                }
                _glp_npp_fixed_col(npp, col);
                return 0 as libc::c_int;
            } else if ret == 1 as libc::c_int || ret == 2 as libc::c_int {
                return 0xa as libc::c_int
            } else {
                (ret != ret
                    || {
                        glp_assert_(
                            b"ret != ret\0" as *const u8 as *const libc::c_char,
                            b"npp/npp5.c\0" as *const u8 as *const libc::c_char,
                            218 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        } else {
            ret = _glp_npp_ineq_singlet(npp, row);
            if 0 as libc::c_int <= ret && ret <= 3 as libc::c_int {
                _glp_npp_activate_col(npp, col);
                if ret >= 2 as libc::c_int {
                    aij = (*col).ptr;
                    while !aij.is_null() {
                        _glp_npp_activate_row(npp, (*aij).row);
                        aij = (*aij).c_next;
                    }
                }
                if ret == 3 as libc::c_int {
                    _glp_npp_fixed_col(npp, col);
                }
                return 0 as libc::c_int;
            } else if ret == 4 as libc::c_int {
                return 0xa as libc::c_int
            } else {
                (ret != ret
                    || {
                        glp_assert_(
                            b"ret != ret\0" as *const u8 as *const libc::c_char,
                            b"npp/npp5.c\0" as *const u8 as *const libc::c_char,
                            253 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
    }
    ret = _glp_npp_analyze_row(npp, row);
    (0 as libc::c_int <= ret && ret <= 0xff as libc::c_int
        || {
            glp_assert_(
                b"0x00 <= ret && ret <= 0xFF\0" as *const u8 as *const libc::c_char,
                b"npp/npp5.c\0" as *const u8 as *const libc::c_char,
                289 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if ret == 0x33 as libc::c_int {
        return 0xa as libc::c_int;
    }
    if ret & 0xf as libc::c_int == 0 as libc::c_int {
        if (*row).lb != -1.7976931348623157e+308f64 {
            _glp_npp_inactive_bound(npp, row, 0 as libc::c_int);
        }
        current_block = 5159818223158340697;
    } else if ret & 0xf as libc::c_int == 0x1 as libc::c_int {
        current_block = 5159818223158340697;
    } else if ret & 0xf as libc::c_int == 0x2 as libc::c_int {
        if _glp_npp_forcing_row(npp, row, 0 as libc::c_int) == 0 as libc::c_int {
            current_block = 11725818755224855140;
        } else {
            current_block = 5159818223158340697;
        }
    } else {
        (ret != ret
            || {
                glp_assert_(
                    b"ret != ret\0" as *const u8 as *const libc::c_char,
                    b"npp/npp5.c\0" as *const u8 as *const libc::c_char,
                    338 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        current_block = 5159818223158340697;
    }
    match current_block {
        5159818223158340697 => {
            if ret & 0xf0 as libc::c_int == 0 as libc::c_int {
                if (*row).ub != 1.7976931348623157e+308f64 {
                    _glp_npp_inactive_bound(npp, row, 1 as libc::c_int);
                }
                current_block = 15669289850109000831;
            } else if ret & 0xf0 as libc::c_int == 0x10 as libc::c_int {
                current_block = 15669289850109000831;
            } else if ret & 0xf0 as libc::c_int == 0x20 as libc::c_int {
                if _glp_npp_forcing_row(npp, row, 1 as libc::c_int) == 0 as libc::c_int {
                    current_block = 11725818755224855140;
                } else {
                    current_block = 15669289850109000831;
                }
            } else {
                (ret != ret
                    || {
                        glp_assert_(
                            b"ret != ret\0" as *const u8 as *const libc::c_char,
                            b"npp/npp5.c\0" as *const u8 as *const libc::c_char,
                            362 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                current_block = 15669289850109000831;
            }
            match current_block {
                11725818755224855140 => {}
                _ => {
                    if (*row).lb == -1.7976931348623157e+308f64
                        && (*row).ub == 1.7976931348623157e+308f64
                    {
                        aij = (*row).ptr;
                        while !aij.is_null() {
                            _glp_npp_activate_col(npp, (*aij).col);
                            aij = (*aij).r_next;
                        }
                        _glp_npp_free_row(npp, row);
                        return 0 as libc::c_int;
                    }
                    if (*npp).sol == 3 as libc::c_int && hard != 0 {
                        if _glp_npp_improve_bounds(npp, row, 1 as libc::c_int)
                            < 0 as libc::c_int
                        {
                            return 0xa as libc::c_int;
                        }
                    }
                    return 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    aij = (*row).ptr;
    while !aij.is_null() {
        col = (*aij).col;
        next_aij = (*aij).r_next;
        aaa = (*col).ptr;
        while !aaa.is_null() {
            _glp_npp_activate_row(npp, (*aaa).row);
            aaa = (*aaa).c_next;
        }
        _glp_npp_fixed_col(npp, col);
        aij = next_aij;
    }
    _glp_npp_free_row(npp, row);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_npp_improve_bounds(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
    mut flag: libc::c_int,
) -> libc::c_int {
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut next_aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut aaa: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut kase: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    ((*npp).sol == 3 as libc::c_int
        || {
            glp_assert_(
                b"npp->sol == GLP_MIP\0" as *const u8 as *const libc::c_char,
                b"npp/npp5.c\0" as *const u8 as *const libc::c_char,
                424 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!((*row).lb == -1.7976931348623157e+308f64
        && (*row).ub == 1.7976931348623157e+308f64)
        || {
            glp_assert_(
                b"!(row->lb == -DBL_MAX && row->ub == +DBL_MAX)\0" as *const u8
                    as *const libc::c_char,
                b"npp/npp5.c\0" as *const u8 as *const libc::c_char,
                426 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_npp_implied_bounds(npp, row);
    aij = (*row).ptr;
    while !aij.is_null() {
        col = (*aij).col;
        next_aij = (*aij).r_next;
        let mut current_block_24: u64;
        kase = 0 as libc::c_int;
        while kase <= 1 as libc::c_int {
            lb = (*col).lb;
            ub = (*col).ub;
            if kase == 0 as libc::c_int {
                if (*col).ll.ll == -1.7976931348623157e+308f64 {
                    current_block_24 = 10879442775620481940;
                } else {
                    ret = _glp_npp_implied_lower(npp, col, (*col).ll.ll);
                    current_block_24 = 3512920355445576850;
                }
            } else if (*col).uu.uu == 1.7976931348623157e+308f64 {
                current_block_24 = 10879442775620481940;
            } else {
                ret = _glp_npp_implied_upper(npp, col, (*col).uu.uu);
                current_block_24 = 3512920355445576850;
            }
            match current_block_24 {
                3512920355445576850 => {
                    if ret == 0 as libc::c_int || ret == 1 as libc::c_int {
                        (*col).lb = lb;
                        (*col).ub = ub;
                    } else if ret == 2 as libc::c_int || ret == 3 as libc::c_int {
                        count += 1;
                        count;
                        if flag != 0 {
                            aaa = (*col).ptr;
                            while !aaa.is_null() {
                                if (*aaa).row != row {
                                    _glp_npp_activate_row(npp, (*aaa).row);
                                }
                                aaa = (*aaa).c_next;
                            }
                        }
                        if ret == 3 as libc::c_int {
                            _glp_npp_fixed_col(npp, col);
                            break;
                        }
                    } else if ret == 4 as libc::c_int {
                        return -(1 as libc::c_int)
                    } else {
                        (ret != ret
                            || {
                                glp_assert_(
                                    b"ret != ret\0" as *const u8 as *const libc::c_char,
                                    b"npp/npp5.c\0" as *const u8 as *const libc::c_char,
                                    480 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                    }
                }
                _ => {}
            }
            kase += 1;
            kase;
        }
        aij = next_aij;
    }
    return count;
}
pub unsafe extern "C" fn _glp_npp_process_col(
    mut npp: *mut NPP,
    mut col: *mut NPPCOL,
) -> libc::c_int {
    let mut row: *mut NPPROW = 0 as *mut NPPROW;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut ret: libc::c_int = 0;
    ((*col).lb < (*col).ub
        || {
            glp_assert_(
                b"col->lb < col->ub\0" as *const u8 as *const libc::c_char,
                b"npp/npp5.c\0" as *const u8 as *const libc::c_char,
                526 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if ((*col).ptr).is_null() {
        ret = _glp_npp_empty_col(npp, col);
        if ret == 0 as libc::c_int {
            return 0 as libc::c_int
        } else if ret == 1 as libc::c_int {
            return 0xb as libc::c_int
        } else {
            (ret != ret
                || {
                    glp_assert_(
                        b"ret != ret\0" as *const u8 as *const libc::c_char,
                        b"npp/npp5.c\0" as *const u8 as *const libc::c_char,
                        543 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    if ((*(*col).ptr).c_next).is_null() {
        row = (*(*col).ptr).row;
        let mut current_block_28: u64;
        if (*row).lb == (*row).ub {
            if (*col).is_int == 0 {
                current_block_28 = 1653761772223795703;
            } else {
                current_block_28 = 4775909272756257391;
            }
        } else if (*col).is_int == 0 {
            ret = _glp_npp_implied_free(npp, col);
            if ret == 0 as libc::c_int {
                current_block_28 = 1653761772223795703;
            } else {
                if !(ret == 1 as libc::c_int) {
                    if ret == 2 as libc::c_int {
                        return 0xb as libc::c_int;
                    }
                }
                current_block_28 = 4775909272756257391;
            }
        } else {
            current_block_28 = 4775909272756257391;
        }
        match current_block_28 {
            1653761772223795703 => {
                _glp_npp_implied_slack(npp, col);
                if (*row).lb == -1.7976931348623157e+308f64
                    && (*row).ub == 1.7976931348623157e+308f64
                {
                    aij = (*row).ptr;
                    while !aij.is_null() {
                        _glp_npp_activate_col(npp, (*aij).col);
                        aij = (*aij).r_next;
                    }
                    _glp_npp_free_row(npp, row);
                } else {
                    _glp_npp_activate_row(npp, row);
                }
                return 0 as libc::c_int;
            }
            _ => {}
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_npp_process_prob(
    mut npp: *mut NPP,
    mut hard: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut row: *mut NPPROW = 0 as *mut NPPROW;
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut processing: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    _glp_npp_clean_prob(npp);
    row = (*npp).r_head;
    while !row.is_null() {
        (*row).temp = 1 as libc::c_int;
        row = (*row).next;
    }
    col = (*npp).c_head;
    while !col.is_null() {
        (*col).temp = 1 as libc::c_int;
        col = (*col).next;
    }
    processing = 1 as libc::c_int;
    's_41: loop {
        if !(processing != 0) {
            current_block = 1109700713171191020;
            break;
        }
        processing = 0 as libc::c_int;
        loop {
            row = (*npp).r_head;
            if row.is_null() || (*row).temp == 0 {
                break;
            }
            _glp_npp_deactivate_row(npp, row);
            ret = _glp_npp_process_row(npp, row, hard);
            if ret != 0 as libc::c_int {
                current_block = 434799920685049853;
                break 's_41;
            }
            processing = 1 as libc::c_int;
        }
        loop {
            col = (*npp).c_head;
            if col.is_null() || (*col).temp == 0 {
                break;
            }
            _glp_npp_deactivate_col(npp, col);
            ret = _glp_npp_process_col(npp, col);
            if ret != 0 as libc::c_int {
                current_block = 434799920685049853;
                break 's_41;
            }
            processing = 1 as libc::c_int;
        }
    }
    match current_block {
        1109700713171191020 => {
            if (*npp).sol == 3 as libc::c_int && hard == 0 {
                row = (*npp).r_head;
                loop {
                    if row.is_null() {
                        current_block = 14359455889292382949;
                        break;
                    }
                    if _glp_npp_improve_bounds(npp, row, 0 as libc::c_int)
                        < 0 as libc::c_int
                    {
                        ret = 0xa as libc::c_int;
                        current_block = 434799920685049853;
                        break;
                    } else {
                        row = (*row).next;
                    }
                }
            } else {
                current_block = 14359455889292382949;
            }
            match current_block {
                434799920685049853 => {}
                _ => {
                    ret = 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    (ret == 0 as libc::c_int || ret == 0xa as libc::c_int || ret == 0xb as libc::c_int
        || {
            glp_assert_(
                b"ret == 0 || ret == GLP_ENOPFS || ret == GLP_ENODFS\0" as *const u8
                    as *const libc::c_char,
                b"npp/npp5.c\0" as *const u8 as *const libc::c_char,
                697 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return ret;
}
pub unsafe extern "C" fn _glp_npp_simplex(
    mut npp: *mut NPP,
    mut parm: *const glp_smcp,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ((*npp).sol == 1 as libc::c_int
        || {
            glp_assert_(
                b"npp->sol == GLP_SOL\0" as *const u8 as *const libc::c_char,
                b"npp/npp5.c\0" as *const u8 as *const libc::c_char,
                709 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (parm == parm
        || {
            glp_assert_(
                b"parm == parm\0" as *const u8 as *const libc::c_char,
                b"npp/npp5.c\0" as *const u8 as *const libc::c_char,
                710 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ret = _glp_npp_process_prob(npp, 0 as libc::c_int);
    return ret;
}
pub unsafe extern "C" fn _glp_npp_integer(
    mut npp: *mut NPP,
    mut parm: *const glp_iocp,
) -> libc::c_int {
    let mut row: *mut NPPROW = 0 as *mut NPPROW;
    let mut prev_row: *mut NPPROW = 0 as *mut NPPROW;
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut count: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    ((*npp).sol == 3 as libc::c_int
        || {
            glp_assert_(
                b"npp->sol == GLP_MIP\0" as *const u8 as *const libc::c_char,
                b"npp/npp5.c\0" as *const u8 as *const libc::c_char,
                723 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (parm == parm
        || {
            glp_assert_(
                b"parm == parm\0" as *const u8 as *const libc::c_char,
                b"npp/npp5.c\0" as *const u8 as *const libc::c_char,
                724 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ret = _glp_npp_process_prob(npp, 1 as libc::c_int);
    if !(ret != 0 as libc::c_int) {
        if (*parm).binarize != 0 {
            _glp_npp_binarize_prob(npp);
        }
        count = 0 as libc::c_int;
        row = (*npp).r_tail;
        while !row.is_null() {
            prev_row = (*row).prev;
            if !((*row).lb == -1.7976931348623157e+308f64
                && (*row).ub == 1.7976931348623157e+308f64)
            {
                if !((*row).lb == (*row).ub) {
                    if !(((*row).ptr).is_null() || ((*(*row).ptr).r_next).is_null()) {
                        aij = (*row).ptr;
                        while !aij.is_null() {
                            col = (*aij).col;
                            if !((*col).is_int as libc::c_int != 0 && (*col).lb == 0.0f64
                                && (*col).ub == 1.0f64)
                            {
                                break;
                            }
                            aij = (*aij).r_next;
                        }
                        if aij.is_null() {
                            count += _glp_npp_hidden_packing(npp, row);
                        }
                    }
                }
            }
            row = prev_row;
        }
        if count > 0 as libc::c_int {
            glp_printf(
                b"%d hidden packing inequaliti(es) were detected\n\0" as *const u8
                    as *const libc::c_char,
                count,
            );
        }
        count = 0 as libc::c_int;
        row = (*npp).r_tail;
        while !row.is_null() {
            prev_row = (*row).prev;
            if !((*row).lb == -1.7976931348623157e+308f64
                && (*row).ub == 1.7976931348623157e+308f64)
            {
                if !((*row).lb == (*row).ub) {
                    if !(((*row).ptr).is_null() || ((*(*row).ptr).r_next).is_null()
                        || ((*(*(*row).ptr).r_next).r_next).is_null())
                    {
                        aij = (*row).ptr;
                        while !aij.is_null() {
                            col = (*aij).col;
                            if !((*col).is_int as libc::c_int != 0 && (*col).lb == 0.0f64
                                && (*col).ub == 1.0f64)
                            {
                                break;
                            }
                            aij = (*aij).r_next;
                        }
                        if aij.is_null() {
                            count += _glp_npp_hidden_covering(npp, row);
                        }
                    }
                }
            }
            row = prev_row;
        }
        if count > 0 as libc::c_int {
            glp_printf(
                b"%d hidden covering inequaliti(es) were detected\n\0" as *const u8
                    as *const libc::c_char,
                count,
            );
        }
        count = 0 as libc::c_int;
        row = (*npp).r_tail;
        while !row.is_null() {
            prev_row = (*row).prev;
            if !((*row).lb == (*row).ub) {
                count += _glp_npp_reduce_ineq_coef(npp, row);
            }
            row = prev_row;
        }
        if count > 0 as libc::c_int {
            glp_printf(
                b"%d constraint coefficient(s) were reduced\n\0" as *const u8
                    as *const libc::c_char,
                count,
            );
        }
        ret = 0 as libc::c_int;
    }
    return ret;
}
