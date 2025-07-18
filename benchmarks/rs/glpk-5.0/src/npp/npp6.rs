use ::libc;
extern "C" {
    pub type DMP;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn _glp_dmp_free_atom(pool: *mut DMP, atom: *mut libc::c_void, size: libc::c_int);
    fn _glp_dmp_get_atom(pool: *mut DMP, size: libc::c_int) -> *mut libc::c_void;
    fn _glp_npp_push_tse(
        npp: *mut NPP,
        func: Option::<unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> libc::c_int>,
        size: libc::c_int,
    ) -> *mut libc::c_void;
    fn _glp_npp_del_row(npp: *mut NPP, row: *mut NPPROW);
    fn _glp_npp_del_col(npp: *mut NPP, col: *mut NPPCOL);
    fn _glp_npp_del_aij(npp: *mut NPP, aij: *mut NPPAIJ);
    fn _glp_npp_row_nnz(npp: *mut NPP, row: *mut NPPROW) -> libc::c_int;
    fn _glp_npp_add_aij(
        npp: *mut NPP,
        row: *mut NPPROW,
        col: *mut NPPCOL,
        val: libc::c_double,
    ) -> *mut NPPAIJ;
    fn _glp_npp_add_col(npp: *mut NPP) -> *mut NPPCOL;
    fn _glp_npp_add_row(npp: *mut NPP) -> *mut NPPROW;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sat_fixed_col {
    pub q: libc::c_int,
    pub s: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPLIT {
    pub col: *mut NPPCOL,
    pub neg: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPLSE {
    pub lit: NPPLIT,
    pub next: *mut NPPLSE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPSED {
    pub x: NPPLIT,
    pub y: NPPLIT,
    pub z: NPPLIT,
    pub s: *mut NPPCOL,
    pub c: *mut NPPCOL,
}
pub unsafe extern "C" fn _glp_npp_sat_free_row(mut npp: *mut NPP, mut p: *mut NPPROW) {
    ((*p).lb == -1.7976931348623157e+308f64 && (*p).ub == 1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"p->lb == -DBL_MAX && p->ub == +DBL_MAX\0" as *const u8
                    as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                38 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_npp_del_row(npp, p);
}
pub unsafe extern "C" fn _glp_npp_sat_fixed_col(
    mut npp: *mut NPP,
    mut q: *mut NPPCOL,
) -> libc::c_int {
    let mut info: *mut sat_fixed_col = 0 as *mut sat_fixed_col;
    let mut i: *mut NPPROW = 0 as *mut NPPROW;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut temp: libc::c_int = 0;
    ((*q).lb == (*q).ub
        || {
            glp_assert_(
                b"q->lb == q->ub\0" as *const u8 as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                97 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    info = _glp_npp_push_tse(
        npp,
        Some(
            rcv_sat_fixed_col
                as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> libc::c_int,
        ),
        ::std::mem::size_of::<sat_fixed_col>() as libc::c_ulong as libc::c_int,
    ) as *mut sat_fixed_col;
    (*info).q = (*q).j;
    (*info).s = (*q).lb as libc::c_int;
    ((*info).s as libc::c_double == (*q).lb
        || {
            glp_assert_(
                b"(double)info->s == q->lb\0" as *const u8 as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                103 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if !((*info).s == 0 as libc::c_int) {
        aij = (*q).ptr;
        while !aij.is_null() {
            i = (*aij).row;
            if (*i).lb != -1.7976931348623157e+308f64 {
                (*i).lb -= (*aij).val * (*info).s as libc::c_double;
                temp = (*i).lb as libc::c_int;
                if temp as libc::c_double != (*i).lb {
                    return 1 as libc::c_int;
                }
            }
            if (*i).ub != 1.7976931348623157e+308f64 {
                (*i).ub -= (*aij).val * (*info).s as libc::c_double;
                temp = (*i).ub as libc::c_int;
                if temp as libc::c_double != (*i).ub {
                    return 2 as libc::c_int;
                }
            }
            aij = (*aij).c_next;
        }
    }
    _glp_npp_del_col(npp, q);
    return 0 as libc::c_int;
}
unsafe extern "C" fn rcv_sat_fixed_col(
    mut npp: *mut NPP,
    mut info_: *mut libc::c_void,
) -> libc::c_int {
    let mut info: *mut sat_fixed_col = info_ as *mut sat_fixed_col;
    *((*npp).c_value).offset((*info).q as isize) = (*info).s as libc::c_double;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_npp_sat_is_bin_comb(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> libc::c_int {
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                144 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    aij = (*row).ptr;
    while !aij.is_null() {
        if !((*aij).val == 1.0f64 || (*aij).val == -1.0f64) {
            return 0 as libc::c_int;
        }
        col = (*aij).col;
        if !((*col).is_int as libc::c_int != 0 && (*col).lb == 0.0f64
            && (*col).ub == 1.0f64)
        {
            return 0 as libc::c_int;
        }
        aij = (*aij).r_next;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn _glp_npp_sat_num_pos_coef(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> libc::c_int {
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut num: libc::c_int = 0 as libc::c_int;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                164 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    aij = (*row).ptr;
    while !aij.is_null() {
        if (*aij).val > 0.0f64 {
            num += 1;
            num;
        }
        aij = (*aij).r_next;
    }
    return num;
}
pub unsafe extern "C" fn _glp_npp_sat_num_neg_coef(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> libc::c_int {
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut num: libc::c_int = 0 as libc::c_int;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                181 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    aij = (*row).ptr;
    while !aij.is_null() {
        if (*aij).val < 0.0f64 {
            num += 1;
            num;
        }
        aij = (*aij).r_next;
    }
    return num;
}
pub unsafe extern "C" fn _glp_npp_sat_is_cover_ineq(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> libc::c_int {
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                223 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*row).lb != -1.7976931348623157e+308f64
        && (*row).ub == 1.7976931348623157e+308f64
    {
        if _glp_npp_sat_is_bin_comb(npp, row) != 0 {
            if (*row).lb
                == 1.0f64 - _glp_npp_sat_num_neg_coef(npp, row) as libc::c_double
            {
                return 1 as libc::c_int;
            }
        }
    } else if (*row).lb == -1.7976931348623157e+308f64
        && (*row).ub != 1.7976931348623157e+308f64
    {
        if _glp_npp_sat_is_bin_comb(npp, row) != 0 {
            if (*row).ub
                == _glp_npp_sat_num_pos_coef(npp, row) as libc::c_double - 1.0f64
            {
                return 2 as libc::c_int;
            }
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_npp_sat_is_pack_ineq(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> libc::c_int {
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                282 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*row).lb == -1.7976931348623157e+308f64
        && (*row).ub != 1.7976931348623157e+308f64
    {
        if _glp_npp_sat_is_bin_comb(npp, row) != 0 {
            if (*row).ub
                == 1.0f64 - _glp_npp_sat_num_neg_coef(npp, row) as libc::c_double
            {
                return 1 as libc::c_int;
            }
        }
    } else if (*row).lb != -1.7976931348623157e+308f64
        && (*row).ub == 1.7976931348623157e+308f64
    {
        if _glp_npp_sat_is_bin_comb(npp, row) != 0 {
            if (*row).lb
                == _glp_npp_sat_num_pos_coef(npp, row) as libc::c_double - 1.0f64
            {
                return 2 as libc::c_int;
            }
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_npp_sat_is_partn_eq(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> libc::c_int {
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                341 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*row).lb == (*row).ub {
        if _glp_npp_sat_is_bin_comb(npp, row) != 0 {
            if (*row).lb
                == 1.0f64 - _glp_npp_sat_num_neg_coef(npp, row) as libc::c_double
            {
                return 1 as libc::c_int;
            }
            if (*row).ub
                == _glp_npp_sat_num_pos_coef(npp, row) as libc::c_double - 1.0f64
            {
                return 2 as libc::c_int;
            }
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_npp_sat_reverse_row(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> libc::c_int {
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut temp: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut old_lb: libc::c_double = 0.;
    let mut old_ub: libc::c_double = 0.;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                379 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    aij = (*row).ptr;
    while !aij.is_null() {
        (*aij).val = -(*aij).val;
        temp = (*aij).val as libc::c_int;
        if temp as libc::c_double != (*aij).val {
            ret = 1 as libc::c_int;
        }
        aij = (*aij).r_next;
    }
    old_lb = (*row).lb;
    old_ub = (*row).ub;
    if old_ub == 1.7976931348623157e+308f64 {
        (*row).lb = -1.7976931348623157e+308f64;
    } else {
        (*row).lb = -old_ub;
        temp = (*row).lb as libc::c_int;
        if temp as libc::c_double != (*row).lb {
            ret = 2 as libc::c_int;
        }
    }
    if old_lb == -1.7976931348623157e+308f64 {
        (*row).ub = 1.7976931348623157e+308f64;
    } else {
        (*row).ub = -old_lb;
        temp = (*row).ub as libc::c_int;
        if temp as libc::c_double != (*row).ub {
            ret = 3 as libc::c_int;
        }
    }
    return ret;
}
pub unsafe extern "C" fn _glp_npp_sat_split_pack(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
    mut nlit: libc::c_int,
) -> *mut NPPROW {
    let mut rrr: *mut NPPROW = 0 as *mut NPPROW;
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut k: libc::c_int = 0;
    (_glp_npp_sat_is_pack_ineq(npp, row) == 1 as libc::c_int
        || {
            glp_assert_(
                b"npp_sat_is_pack_ineq(npp, row) == 1\0" as *const u8
                    as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                439 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((0 as libc::c_int) < nlit && nlit < _glp_npp_row_nnz(npp, row)
        || {
            glp_assert_(
                b"0 < nlit && nlit < npp_row_nnz(npp, row)\0" as *const u8
                    as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                442 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    rrr = _glp_npp_add_row(npp);
    (*rrr).lb = -1.7976931348623157e+308f64;
    (*rrr).ub = 1.0f64;
    k = 1 as libc::c_int;
    while k <= nlit {
        aij = (*row).ptr;
        (!aij.is_null()
            || {
                glp_assert_(
                    b"aij != NULL\0" as *const u8 as *const libc::c_char,
                    b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                    450 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        _glp_npp_add_aij(npp, rrr, (*aij).col, (*aij).val);
        if (*aij).val < 0.0f64 {
            (*rrr).ub -= 1.0f64;
            (*row).ub += 1.0f64;
        }
        _glp_npp_del_aij(npp, aij);
        k += 1;
        k;
    }
    col = _glp_npp_add_col(npp);
    (*col).is_int = 1 as libc::c_int as libc::c_char;
    (*col).lb = 0.0f64;
    (*col).ub = 1.0f64;
    _glp_npp_add_aij(npp, rrr, col, -1.0f64);
    (*rrr).ub -= 1.0f64;
    _glp_npp_add_aij(npp, row, col, 1.0f64);
    return rrr;
}
pub unsafe extern "C" fn _glp_npp_sat_encode_pack(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) {
    let mut rrr: *mut NPPROW = 0 as *mut NPPROW;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut aik: *mut NPPAIJ = 0 as *mut NPPAIJ;
    (_glp_npp_sat_is_pack_ineq(npp, row) == 1 as libc::c_int
        || {
            glp_assert_(
                b"npp_sat_is_pack_ineq(npp, row) == 1\0" as *const u8
                    as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                496 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    aij = (*row).ptr;
    while !aij.is_null() {
        aik = (*aij).r_next;
        while !aik.is_null() {
            rrr = _glp_npp_add_row(npp);
            (*rrr).lb = -1.7976931348623157e+308f64;
            (*rrr).ub = 1.0f64;
            _glp_npp_add_aij(npp, rrr, (*aij).col, (*aij).val);
            if (*aij).val < 0.0f64 {
                (*rrr).ub -= 1.0f64;
            }
            _glp_npp_add_aij(npp, rrr, (*aik).col, (*aik).val);
            if (*aik).val < 0.0f64 {
                (*rrr).ub -= 1.0f64;
            }
            _glp_npp_sat_reverse_row(npp, rrr);
            (_glp_npp_sat_is_cover_ineq(npp, rrr) == 1 as libc::c_int
                || {
                    glp_assert_(
                        b"npp_sat_is_cover_ineq(npp, rrr) == 1\0" as *const u8
                            as *const libc::c_char,
                        b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                        513 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            aik = (*aik).r_next;
        }
        aij = (*aij).r_next;
    }
    _glp_npp_del_row(npp, row);
}
pub unsafe extern "C" fn _glp_npp_sat_encode_sum2(
    mut npp: *mut NPP,
    mut set: *mut NPPLSE,
    mut sed: *mut NPPSED,
) {
    let mut row: *mut NPPROW = 0 as *mut NPPROW;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    (!set.is_null()
        || {
            glp_assert_(
                b"set != NULL\0" as *const u8 as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                543 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!((*set).next).is_null()
        || {
            glp_assert_(
                b"set->next != NULL\0" as *const u8 as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                544 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (((*(*set).next).next).is_null()
        || {
            glp_assert_(
                b"set->next->next == NULL\0" as *const u8 as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                545 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*sed).x = (*set).lit;
    ((*sed).x.neg == 0 as libc::c_int || (*sed).x.neg == 1 as libc::c_int
        || {
            glp_assert_(
                b"sed->x.neg == 0 || sed->x.neg == 1\0" as *const u8
                    as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                547 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*sed).y = (*(*set).next).lit;
    ((*sed).y.neg == 0 as libc::c_int || (*sed).y.neg == 1 as libc::c_int
        || {
            glp_assert_(
                b"sed->y.neg == 0 || sed->y.neg == 1\0" as *const u8
                    as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                549 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*sed).z.col = 0 as *mut NPPCOL;
    (*sed).z.neg = 0 as libc::c_int;
    (*sed).s = _glp_npp_add_col(npp);
    (*(*sed).s).is_int = 1 as libc::c_int as libc::c_char;
    (*(*sed).s).lb = 0.0f64;
    (*(*sed).s).ub = 1.0f64;
    x = 0 as libc::c_int;
    while x <= 1 as libc::c_int {
        y = 0 as libc::c_int;
        while y <= 1 as libc::c_int {
            s = 0 as libc::c_int;
            while s <= 1 as libc::c_int {
                if (x + y) % 2 as libc::c_int != s {
                    row = _glp_npp_add_row(npp);
                    (*row).lb = 1.0f64;
                    (*row).ub = 1.7976931348623157e+308f64;
                    if x == (*sed).x.neg {
                        _glp_npp_add_aij(npp, row, (*sed).x.col, 1.0f64);
                    } else {
                        _glp_npp_add_aij(npp, row, (*sed).x.col, -1.0f64);
                        (*row).lb -= 1.0f64;
                    }
                    if y == (*sed).y.neg {
                        _glp_npp_add_aij(npp, row, (*sed).y.col, 1.0f64);
                    } else {
                        _glp_npp_add_aij(npp, row, (*sed).y.col, -1.0f64);
                        (*row).lb -= 1.0f64;
                    }
                    if s == 0 as libc::c_int {
                        _glp_npp_add_aij(npp, row, (*sed).s, 1.0f64);
                    } else {
                        _glp_npp_add_aij(npp, row, (*sed).s, -1.0f64);
                        (*row).lb -= 1.0f64;
                    }
                }
                s += 1;
                s;
            }
            y += 1;
            y;
        }
        x += 1;
        x;
    }
    (*sed).c = _glp_npp_add_col(npp);
    (*(*sed).c).is_int = 1 as libc::c_int as libc::c_char;
    (*(*sed).c).lb = 0.0f64;
    (*(*sed).c).ub = 1.0f64;
    x = 0 as libc::c_int;
    while x <= 1 as libc::c_int {
        y = 0 as libc::c_int;
        while y <= 1 as libc::c_int {
            c = 0 as libc::c_int;
            while c <= 1 as libc::c_int {
                if (x + y) / 2 as libc::c_int != c {
                    row = _glp_npp_add_row(npp);
                    (*row).lb = 1.0f64;
                    (*row).ub = 1.7976931348623157e+308f64;
                    if x == (*sed).x.neg {
                        _glp_npp_add_aij(npp, row, (*sed).x.col, 1.0f64);
                    } else {
                        _glp_npp_add_aij(npp, row, (*sed).x.col, -1.0f64);
                        (*row).lb -= 1.0f64;
                    }
                    if y == (*sed).y.neg {
                        _glp_npp_add_aij(npp, row, (*sed).y.col, 1.0f64);
                    } else {
                        _glp_npp_add_aij(npp, row, (*sed).y.col, -1.0f64);
                        (*row).lb -= 1.0f64;
                    }
                    if c == 0 as libc::c_int {
                        _glp_npp_add_aij(npp, row, (*sed).c, 1.0f64);
                    } else {
                        _glp_npp_add_aij(npp, row, (*sed).c, -1.0f64);
                        (*row).lb -= 1.0f64;
                    }
                }
                c += 1;
                c;
            }
            y += 1;
            y;
        }
        x += 1;
        x;
    }
}
pub unsafe extern "C" fn _glp_npp_sat_encode_sum3(
    mut npp: *mut NPP,
    mut set: *mut NPPLSE,
    mut sed: *mut NPPSED,
) {
    let mut row: *mut NPPROW = 0 as *mut NPPROW;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    (!set.is_null()
        || {
            glp_assert_(
                b"set != NULL\0" as *const u8 as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                642 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!((*set).next).is_null()
        || {
            glp_assert_(
                b"set->next != NULL\0" as *const u8 as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                643 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!((*(*set).next).next).is_null()
        || {
            glp_assert_(
                b"set->next->next != NULL\0" as *const u8 as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                644 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*sed).x = (*set).lit;
    ((*sed).x.neg == 0 as libc::c_int || (*sed).x.neg == 1 as libc::c_int
        || {
            glp_assert_(
                b"sed->x.neg == 0 || sed->x.neg == 1\0" as *const u8
                    as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                646 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*sed).y = (*(*set).next).lit;
    ((*sed).y.neg == 0 as libc::c_int || (*sed).y.neg == 1 as libc::c_int
        || {
            glp_assert_(
                b"sed->y.neg == 0 || sed->y.neg == 1\0" as *const u8
                    as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                648 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*sed).z = (*(*(*set).next).next).lit;
    ((*sed).z.neg == 0 as libc::c_int || (*sed).z.neg == 1 as libc::c_int
        || {
            glp_assert_(
                b"sed->z.neg == 0 || sed->z.neg == 1\0" as *const u8
                    as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                650 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*sed).s = _glp_npp_add_col(npp);
    (*(*sed).s).is_int = 1 as libc::c_int as libc::c_char;
    (*(*sed).s).lb = 0.0f64;
    (*(*sed).s).ub = 1.0f64;
    x = 0 as libc::c_int;
    while x <= 1 as libc::c_int {
        y = 0 as libc::c_int;
        while y <= 1 as libc::c_int {
            z = 0 as libc::c_int;
            while z <= 1 as libc::c_int {
                s = 0 as libc::c_int;
                while s <= 1 as libc::c_int {
                    if (x + y + z) % 2 as libc::c_int != s {
                        row = _glp_npp_add_row(npp);
                        (*row).lb = 1.0f64;
                        (*row).ub = 1.7976931348623157e+308f64;
                        if x == (*sed).x.neg {
                            _glp_npp_add_aij(npp, row, (*sed).x.col, 1.0f64);
                        } else {
                            _glp_npp_add_aij(npp, row, (*sed).x.col, -1.0f64);
                            (*row).lb -= 1.0f64;
                        }
                        if y == (*sed).y.neg {
                            _glp_npp_add_aij(npp, row, (*sed).y.col, 1.0f64);
                        } else {
                            _glp_npp_add_aij(npp, row, (*sed).y.col, -1.0f64);
                            (*row).lb -= 1.0f64;
                        }
                        if z == (*sed).z.neg {
                            _glp_npp_add_aij(npp, row, (*sed).z.col, 1.0f64);
                        } else {
                            _glp_npp_add_aij(npp, row, (*sed).z.col, -1.0f64);
                            (*row).lb -= 1.0f64;
                        }
                        if s == 0 as libc::c_int {
                            _glp_npp_add_aij(npp, row, (*sed).s, 1.0f64);
                        } else {
                            _glp_npp_add_aij(npp, row, (*sed).s, -1.0f64);
                            (*row).lb -= 1.0f64;
                        }
                    }
                    s += 1;
                    s;
                }
                z += 1;
                z;
            }
            y += 1;
            y;
        }
        x += 1;
        x;
    }
    (*sed).c = _glp_npp_add_col(npp);
    (*(*sed).c).is_int = 1 as libc::c_int as libc::c_char;
    (*(*sed).c).lb = 0.0f64;
    (*(*sed).c).ub = 1.0f64;
    x = 0 as libc::c_int;
    while x <= 1 as libc::c_int {
        y = 0 as libc::c_int;
        while y <= 1 as libc::c_int {
            z = 0 as libc::c_int;
            while z <= 1 as libc::c_int {
                c = 0 as libc::c_int;
                while c <= 1 as libc::c_int {
                    if (x + y + z) / 2 as libc::c_int != c {
                        row = _glp_npp_add_row(npp);
                        (*row).lb = 1.0f64;
                        (*row).ub = 1.7976931348623157e+308f64;
                        if x == (*sed).x.neg {
                            _glp_npp_add_aij(npp, row, (*sed).x.col, 1.0f64);
                        } else {
                            _glp_npp_add_aij(npp, row, (*sed).x.col, -1.0f64);
                            (*row).lb -= 1.0f64;
                        }
                        if y == (*sed).y.neg {
                            _glp_npp_add_aij(npp, row, (*sed).y.col, 1.0f64);
                        } else {
                            _glp_npp_add_aij(npp, row, (*sed).y.col, -1.0f64);
                            (*row).lb -= 1.0f64;
                        }
                        if z == (*sed).z.neg {
                            _glp_npp_add_aij(npp, row, (*sed).z.col, 1.0f64);
                        } else {
                            _glp_npp_add_aij(npp, row, (*sed).z.col, -1.0f64);
                            (*row).lb -= 1.0f64;
                        }
                        if c == 0 as libc::c_int {
                            _glp_npp_add_aij(npp, row, (*sed).c, 1.0f64);
                        } else {
                            _glp_npp_add_aij(npp, row, (*sed).c, -1.0f64);
                            (*row).lb -= 1.0f64;
                        }
                    }
                    c += 1;
                    c;
                }
                z += 1;
                z;
            }
            y += 1;
            y;
        }
        x += 1;
        x;
    }
}
unsafe extern "C" fn remove_lse(
    mut npp: *mut NPP,
    mut set: *mut NPPLSE,
    mut col: *mut NPPCOL,
) -> *mut NPPLSE {
    let mut lse: *mut NPPLSE = 0 as *mut NPPLSE;
    let mut prev: *mut NPPLSE = 0 as *mut NPPLSE;
    lse = set;
    while !lse.is_null() {
        if (*lse).lit.col == col {
            break;
        }
        prev = lse;
        lse = (*lse).next;
    }
    (!lse.is_null()
        || {
            glp_assert_(
                b"lse != NULL\0" as *const u8 as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                825 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if prev.is_null() {
        set = (*lse).next;
    } else {
        (*prev).next = (*lse).next;
    }
    _glp_dmp_free_atom(
        (*npp).pool,
        lse as *mut libc::c_void,
        ::std::mem::size_of::<NPPLSE>() as libc::c_ulong as libc::c_int,
    );
    return set;
}
pub unsafe extern "C" fn _glp_npp_sat_encode_sum_ax(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
    mut y: *mut NPPLIT,
) -> libc::c_int {
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut set: [*mut NPPLSE; 32] = [0 as *mut NPPLSE; 32];
    let mut lse: *mut NPPLSE = 0 as *mut NPPLSE;
    let mut sed: NPPSED = NPPSED {
        x: NPPLIT {
            col: 0 as *mut NPPCOL,
            neg: 0,
        },
        y: NPPLIT {
            col: 0 as *mut NPPCOL,
            neg: 0,
        },
        z: NPPLIT {
            col: 0 as *mut NPPCOL,
            neg: 0,
        },
        s: 0 as *mut NPPCOL,
        c: 0 as *mut NPPCOL,
    };
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut temp: libc::c_int = 0;
    let mut sum: libc::c_double = 0.;
    sum = 0.0f64;
    aij = (*row).ptr;
    while !aij.is_null() {
        sum += fabs((*aij).val);
        aij = (*aij).r_next;
    }
    temp = sum as libc::c_int;
    if temp as libc::c_double != sum {
        return -(1 as libc::c_int);
    }
    n = 0 as libc::c_int;
    while temp > 0 as libc::c_int {
        n += 1;
        n;
        temp >>= 1 as libc::c_int;
    }
    (0 as libc::c_int <= n && n <= 31 as libc::c_int
        || {
            glp_assert_(
                b"0 <= n && n <= NBIT_MAX\0" as *const u8 as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                849 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = 1 as libc::c_int;
    while k <= n {
        set[k as usize] = 0 as *mut NPPLSE;
        k += 1;
        k;
    }
    aij = (*row).ptr;
    while !aij.is_null() {
        temp = fabs((*aij).val) as libc::c_int;
        (temp as libc::c_double == fabs((*aij).val)
            || {
                glp_assert_(
                    b"(int)temp == fabs(aij->val)\0" as *const u8 as *const libc::c_char,
                    b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                    856 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        k = 1 as libc::c_int;
        while temp > 0 as libc::c_int {
            if temp & 1 as libc::c_int != 0 {
                (k <= n
                    || {
                        glp_assert_(
                            b"k <= n\0" as *const u8 as *const libc::c_char,
                            b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                            859 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                lse = _glp_dmp_get_atom(
                    (*npp).pool,
                    ::std::mem::size_of::<NPPLSE>() as libc::c_ulong as libc::c_int,
                ) as *mut NPPLSE;
                (*lse).lit.col = (*aij).col;
                (*lse)
                    .lit
                    .neg = if (*aij).val > 0.0f64 {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                };
                (*lse).next = set[k as usize];
                set[k as usize] = lse;
            }
            k += 1;
            k;
            temp >>= 1 as libc::c_int;
        }
        aij = (*aij).r_next;
    }
    k = 1 as libc::c_int;
    while k <= n {
        loop {
            if (set[k as usize]).is_null() {
                let ref mut fresh0 = (*y.offset(k as isize)).col;
                *fresh0 = 0 as *mut NPPCOL;
                (*y.offset(k as isize)).neg = 0 as libc::c_int;
                break;
            } else if ((*set[k as usize]).next).is_null() {
                *y.offset(k as isize) = (*set[k as usize]).lit;
                _glp_dmp_free_atom(
                    (*npp).pool,
                    set[k as usize] as *mut libc::c_void,
                    ::std::mem::size_of::<NPPLSE>() as libc::c_ulong as libc::c_int,
                );
                break;
            } else {
                if ((*(*set[k as usize]).next).next).is_null() {
                    _glp_npp_sat_encode_sum2(npp, set[k as usize], &mut sed);
                } else {
                    _glp_npp_sat_encode_sum3(npp, set[k as usize], &mut sed);
                    set[k as usize] = remove_lse(npp, set[k as usize], sed.z.col);
                }
                set[k as usize] = remove_lse(npp, set[k as usize], sed.y.col);
                set[k as usize] = remove_lse(npp, set[k as usize], sed.x.col);
                lse = _glp_dmp_get_atom(
                    (*npp).pool,
                    ::std::mem::size_of::<NPPLSE>() as libc::c_ulong as libc::c_int,
                ) as *mut NPPLSE;
                (*lse).lit.col = sed.s;
                (*lse).lit.neg = 0 as libc::c_int;
                (*lse).next = set[k as usize];
                set[k as usize] = lse;
                (k < n
                    || {
                        glp_assert_(
                            b"k < n\0" as *const u8 as *const libc::c_char,
                            b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                            907 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                lse = _glp_dmp_get_atom(
                    (*npp).pool,
                    ::std::mem::size_of::<NPPLSE>() as libc::c_ulong as libc::c_int,
                ) as *mut NPPLSE;
                (*lse).lit.col = sed.c;
                (*lse).lit.neg = 0 as libc::c_int;
                (*lse).next = set[(k + 1 as libc::c_int) as usize];
                set[(k + 1 as libc::c_int) as usize] = lse;
            }
        }
        k += 1;
        k;
    }
    return n;
}
pub unsafe extern "C" fn _glp_npp_sat_normalize_clause(
    mut npp: *mut NPP,
    mut size: libc::c_int,
    mut lit: *mut NPPLIT,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut new_size: libc::c_int = 0;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                931 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (size >= 0 as libc::c_int
        || {
            glp_assert_(
                b"size >= 0\0" as *const u8 as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                932 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    new_size = 0 as libc::c_int;
    k = 1 as libc::c_int;
    while k <= size {
        let mut current_block_8: u64;
        j = 1 as libc::c_int;
        loop {
            if !(j <= new_size) {
                current_block_8 = 13109137661213826276;
                break;
            }
            if (*lit.offset(k as isize)).col == (*lit.offset(j as isize)).col {
                if (*lit.offset(k as isize)).neg == (*lit.offset(j as isize)).neg {
                    current_block_8 = 5399440093318478209;
                    break;
                }
                return -(1 as libc::c_int);
            } else {
                j += 1;
                j;
            }
        }
        match current_block_8 {
            13109137661213826276 => {
                new_size += 1;
                *lit.offset(new_size as isize) = *lit.offset(k as isize);
            }
            _ => {}
        }
        k += 1;
        k;
    }
    return new_size;
}
pub unsafe extern "C" fn _glp_npp_sat_encode_clause(
    mut npp: *mut NPP,
    mut size: libc::c_int,
    mut lit: *mut NPPLIT,
) -> *mut NPPROW {
    let mut row: *mut NPPROW = 0 as *mut NPPROW;
    let mut k: libc::c_int = 0;
    (size >= 1 as libc::c_int
        || {
            glp_assert_(
                b"size >= 1\0" as *const u8 as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                980 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    row = _glp_npp_add_row(npp);
    (*row).lb = 1.0f64;
    (*row).ub = 1.7976931348623157e+308f64;
    k = 1 as libc::c_int;
    while k <= size {
        (!((*lit.offset(k as isize)).col).is_null()
            || {
                glp_assert_(
                    b"lit[k].col != NULL\0" as *const u8 as *const libc::c_char,
                    b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                    984 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if (*lit.offset(k as isize)).neg == 0 as libc::c_int {
            _glp_npp_add_aij(npp, row, (*lit.offset(k as isize)).col, 1.0f64);
        } else if (*lit.offset(k as isize)).neg == 1 as libc::c_int {
            _glp_npp_add_aij(npp, row, (*lit.offset(k as isize)).col, -1.0f64);
            (*row).lb -= 1.0f64;
        } else {
            (lit != lit
                || {
                    glp_assert_(
                        b"lit != lit\0" as *const u8 as *const libc::c_char,
                        b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                        992 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        k += 1;
        k;
    }
    return row;
}
pub unsafe extern "C" fn _glp_npp_sat_encode_geq(
    mut npp: *mut NPP,
    mut n: libc::c_int,
    mut y: *mut NPPLIT,
    mut rhs: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut lit: [NPPLIT; 32] = [NPPLIT {
        col: 0 as *mut NPPCOL,
        neg: 0,
    }; 32];
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut temp: libc::c_int = 0;
    let mut b: [libc::c_int; 32] = [0; 32];
    (0 as libc::c_int <= n && n <= 31 as libc::c_int
        || {
            glp_assert_(
                b"0 <= n && n <= NBIT_MAX\0" as *const u8 as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                1059 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if rhs < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    k = 1 as libc::c_int;
    temp = rhs;
    while k <= n {
        b[k as usize] = temp & 1 as libc::c_int;
        k += 1;
        k;
        temp >>= 1 as libc::c_int;
    }
    if temp != 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    k = 1 as libc::c_int;
    while k <= n {
        size = 0 as libc::c_int;
        if !(b[k as usize] == 0 as libc::c_int) {
            if ((*y.offset(k as isize)).col).is_null() {
                ((*y.offset(k as isize)).neg == 0 as libc::c_int
                    || {
                        glp_assert_(
                            b"y[k].neg == 0\0" as *const u8 as *const libc::c_char,
                            b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                            1081 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            } else {
                size += 1;
                lit[size as usize] = *y.offset(k as isize);
            }
            j = k + 1 as libc::c_int;
            loop {
                if !(j <= n) {
                    current_block = 16203760046146113240;
                    break;
                }
                if ((*y.offset(j as isize)).col).is_null() {
                    ((*y.offset(j as isize)).neg == 0 as libc::c_int
                        || {
                            glp_assert_(
                                b"y[j].neg == 0\0" as *const u8 as *const libc::c_char,
                                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                                1090 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    if !(b[j as usize] == 0 as libc::c_int) {
                        current_block = 14523784380283086299;
                        break;
                    }
                } else {
                    size += 1;
                    lit[size as usize] = *y.offset(j as isize);
                    if b[j as usize] != 0 as libc::c_int {
                        lit[size as usize]
                            .neg = 1 as libc::c_int - lit[size as usize].neg;
                    }
                }
                j += 1;
                j;
            }
            match current_block {
                14523784380283086299 => {}
                _ => {
                    size = _glp_npp_sat_normalize_clause(npp, size, lit.as_mut_ptr());
                    if !(size < 0 as libc::c_int) {
                        if size == 0 as libc::c_int {
                            return 2 as libc::c_int;
                        }
                        _glp_npp_sat_encode_clause(npp, size, lit.as_mut_ptr());
                    }
                }
            }
        }
        k += 1;
        k;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_npp_sat_encode_leq(
    mut npp: *mut NPP,
    mut n: libc::c_int,
    mut y: *mut NPPLIT,
    mut rhs: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut lit: [NPPLIT; 32] = [NPPLIT {
        col: 0 as *mut NPPCOL,
        neg: 0,
    }; 32];
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut temp: libc::c_int = 0;
    let mut b: [libc::c_int; 32] = [0; 32];
    (0 as libc::c_int <= n && n <= 31 as libc::c_int
        || {
            glp_assert_(
                b"0 <= n && n <= NBIT_MAX\0" as *const u8 as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                1186 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if rhs < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    k = 1 as libc::c_int;
    temp = rhs;
    while k <= n {
        b[k as usize] = temp & 1 as libc::c_int;
        k += 1;
        k;
        temp >>= 1 as libc::c_int;
    }
    if temp != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    k = 1 as libc::c_int;
    while k <= n {
        size = 0 as libc::c_int;
        if !(b[k as usize] == 1 as libc::c_int) {
            if ((*y.offset(k as isize)).col).is_null() {
                ((*y.offset(k as isize)).neg == 0 as libc::c_int
                    || {
                        glp_assert_(
                            b"y[k].neg == 0\0" as *const u8 as *const libc::c_char,
                            b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                            1208 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            } else {
                size += 1;
                lit[size as usize] = *y.offset(k as isize);
                lit[size as usize].neg = 1 as libc::c_int - lit[size as usize].neg;
                j = k + 1 as libc::c_int;
                loop {
                    if !(j <= n) {
                        current_block = 7172762164747879670;
                        break;
                    }
                    if ((*y.offset(j as isize)).col).is_null() {
                        ((*y.offset(j as isize)).neg == 0 as libc::c_int
                            || {
                                glp_assert_(
                                    b"y[j].neg == 0\0" as *const u8 as *const libc::c_char,
                                    b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                                    1219 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        if !(b[j as usize] == 0 as libc::c_int) {
                            current_block = 14523784380283086299;
                            break;
                        }
                    } else {
                        size += 1;
                        lit[size as usize] = *y.offset(j as isize);
                        if b[j as usize] != 0 as libc::c_int {
                            lit[size as usize]
                                .neg = 1 as libc::c_int - lit[size as usize].neg;
                        }
                    }
                    j += 1;
                    j;
                }
                match current_block {
                    14523784380283086299 => {}
                    _ => {
                        size = _glp_npp_sat_normalize_clause(
                            npp,
                            size,
                            lit.as_mut_ptr(),
                        );
                        if !(size < 0 as libc::c_int) {
                            if size == 0 as libc::c_int {
                                return 2 as libc::c_int;
                            }
                            _glp_npp_sat_encode_clause(npp, size, lit.as_mut_ptr());
                        }
                    }
                }
            }
        }
        k += 1;
        k;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_npp_sat_encode_row(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> libc::c_int {
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut y: [NPPLIT; 32] = [NPPLIT {
        col: 0 as *mut NPPCOL,
        neg: 0,
    }; 32];
    let mut n: libc::c_int = 0;
    let mut rhs: libc::c_int = 0;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    (!((*row).lb == -1.7976931348623157e+308f64
        && (*row).ub == 1.7976931348623157e+308f64)
        || {
            glp_assert_(
                b"!(row->lb == -DBL_MAX && row->ub == +DBL_MAX)\0" as *const u8
                    as *const libc::c_char,
                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                1327 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    lb = (*row).lb;
    ub = (*row).ub;
    aij = (*row).ptr;
    while !aij.is_null() {
        if (*aij).val < 0.0f64 {
            if lb != -1.7976931348623157e+308f64 {
                lb -= (*aij).val;
            }
            if ub != -1.7976931348623157e+308f64 {
                ub -= (*aij).val;
            }
        }
        aij = (*aij).r_next;
    }
    n = _glp_npp_sat_encode_sum_ax(npp, row, y.as_mut_ptr());
    if n < 0 as libc::c_int {
        return 2 as libc::c_int;
    }
    if lb != -1.7976931348623157e+308f64 {
        rhs = lb as libc::c_int;
        if rhs as libc::c_double != lb {
            return 2 as libc::c_int;
        }
        if _glp_npp_sat_encode_geq(npp, n, y.as_mut_ptr(), rhs) != 0 as libc::c_int {
            return 1 as libc::c_int;
        }
    }
    if ub != 1.7976931348623157e+308f64 {
        rhs = ub as libc::c_int;
        if rhs as libc::c_double != ub {
            return 2 as libc::c_int;
        }
        if _glp_npp_sat_encode_leq(npp, n, y.as_mut_ptr(), rhs) != 0 as libc::c_int {
            return 1 as libc::c_int;
        }
    }
    _glp_npp_del_row(npp, row);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_npp_sat_encode_prob(mut npp: *mut NPP) -> libc::c_int {
    let mut current_block: u64;
    let mut row: *mut NPPROW = 0 as *mut NPPROW;
    let mut next_row: *mut NPPROW = 0 as *mut NPPROW;
    let mut prev_row: *mut NPPROW = 0 as *mut NPPROW;
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut next_col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut cover: libc::c_int = 0 as libc::c_int;
    let mut pack: libc::c_int = 0 as libc::c_int;
    let mut partn: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0;
    row = (*npp).r_head;
    while !row.is_null() {
        next_row = (*row).next;
        if (*row).lb == -1.7976931348623157e+308f64
            && (*row).ub == 1.7976931348623157e+308f64
        {
            _glp_npp_sat_free_row(npp, row);
        }
        row = next_row;
    }
    col = (*npp).c_head;
    while !col.is_null() {
        next_col = (*col).next;
        if (*col).lb == (*col).ub {
            (_glp_npp_sat_fixed_col(npp, col) == 0 as libc::c_int
                || {
                    glp_assert_(
                        b"npp_sat_fixed_col(npp, col) == 0\0" as *const u8
                            as *const libc::c_char,
                        b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                        1394 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        col = next_col;
    }
    col = (*npp).c_head;
    while !col.is_null() {
        ((*col).is_int as libc::c_int != 0 && (*col).lb == 0.0f64 && (*col).ub == 1.0f64
            || {
                glp_assert_(
                    b"col->is_int && col->lb == 0.0 && col->ub == 1.0\0" as *const u8
                        as *const libc::c_char,
                    b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                    1398 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        col = (*col).next;
    }
    row = (*npp).r_tail;
    loop {
        if row.is_null() {
            current_block = 2520131295878969859;
            break;
        }
        prev_row = (*row).prev;
        ret = _glp_npp_sat_is_cover_ineq(npp, row);
        if ret != 0 as libc::c_int {
            cover += 1;
            cover;
            if ret == 2 as libc::c_int {
                (_glp_npp_sat_reverse_row(npp, row) == 0 as libc::c_int
                    || {
                        glp_assert_(
                            b"npp_sat_reverse_row(npp, row) == 0\0" as *const u8
                                as *const libc::c_char,
                            b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                            1411 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                ret = _glp_npp_sat_is_cover_ineq(npp, row);
            }
            (ret == 1 as libc::c_int
                || {
                    glp_assert_(
                        b"ret == 1\0" as *const u8 as *const libc::c_char,
                        b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                        1414 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        } else {
            ret = _glp_npp_sat_is_partn_eq(npp, row);
            if ret != 0 as libc::c_int {
                let mut cov: *mut NPPROW = 0 as *mut NPPROW;
                let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
                partn += 1;
                partn;
                if ret == 2 as libc::c_int {
                    (_glp_npp_sat_reverse_row(npp, row) == 0 as libc::c_int
                        || {
                            glp_assert_(
                                b"npp_sat_reverse_row(npp, row) == 0\0" as *const u8
                                    as *const libc::c_char,
                                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                                1425 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    ret = _glp_npp_sat_is_partn_eq(npp, row);
                }
                (ret == 1 as libc::c_int
                    || {
                        glp_assert_(
                            b"ret == 1\0" as *const u8 as *const libc::c_char,
                            b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                            1428 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                cov = _glp_npp_add_row(npp);
                (*cov).lb = (*row).lb;
                (*cov).ub = 1.7976931348623157e+308f64;
                aij = (*row).ptr;
                while !aij.is_null() {
                    _glp_npp_add_aij(npp, cov, (*aij).col, (*aij).val);
                    aij = (*aij).r_next;
                }
                (_glp_npp_sat_is_cover_ineq(npp, cov) == 1 as libc::c_int
                    || {
                        glp_assert_(
                            b"npp_sat_is_cover_ineq(npp, cov) == 1\0" as *const u8
                                as *const libc::c_char,
                            b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                            1435 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*row).lb = -1.7976931348623157e+308f64;
                (_glp_npp_sat_is_pack_ineq(npp, row) == 1 as libc::c_int
                    || {
                        glp_assert_(
                            b"npp_sat_is_pack_ineq(npp, row) == 1\0" as *const u8
                                as *const libc::c_char,
                            b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                            1439 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                pack -= 1;
                pack;
            }
            ret = _glp_npp_sat_is_pack_ineq(npp, row);
            if ret != 0 as libc::c_int {
                let mut rrr: *mut NPPROW = 0 as *mut NPPROW;
                let mut nlit: libc::c_int = 0;
                let mut desired_nlit: libc::c_int = 4 as libc::c_int;
                pack += 1;
                pack;
                if ret == 2 as libc::c_int {
                    (_glp_npp_sat_reverse_row(npp, row) == 0 as libc::c_int
                        || {
                            glp_assert_(
                                b"npp_sat_reverse_row(npp, row) == 0\0" as *const u8
                                    as *const libc::c_char,
                                b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                                1451 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    ret = _glp_npp_sat_is_pack_ineq(npp, row);
                }
                (ret == 1 as libc::c_int
                    || {
                        glp_assert_(
                            b"ret == 1\0" as *const u8 as *const libc::c_char,
                            b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                            1454 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                loop {
                    nlit = _glp_npp_row_nnz(npp, row);
                    if nlit <= desired_nlit {
                        break;
                    }
                    rrr = _glp_npp_sat_split_pack(
                        npp,
                        row,
                        desired_nlit - 1 as libc::c_int,
                    );
                    _glp_npp_sat_encode_pack(npp, rrr);
                }
                _glp_npp_sat_encode_pack(npp, row);
            } else {
                ret = _glp_npp_sat_encode_row(npp, row);
                if !(ret == 0 as libc::c_int) {
                    if ret == 1 as libc::c_int {
                        ret = 0xa as libc::c_int;
                    } else if ret == 2 as libc::c_int {
                        ret = 0x13 as libc::c_int;
                    } else {
                        (ret != ret
                            || {
                                glp_assert_(
                                    b"ret != ret\0" as *const u8 as *const libc::c_char,
                                    b"npp/npp6.c\0" as *const u8 as *const libc::c_char,
                                    1484 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                    }
                }
                if ret != 0 as libc::c_int {
                    current_block = 15759306406029391826;
                    break;
                }
            }
        }
        row = prev_row;
    }
    match current_block {
        2520131295878969859 => {
            ret = 0 as libc::c_int;
            if cover != 0 as libc::c_int {
                glp_printf(
                    b"%d covering inequalities\n\0" as *const u8 as *const libc::c_char,
                    cover,
                );
            }
            if pack != 0 as libc::c_int {
                glp_printf(
                    b"%d packing inequalities\n\0" as *const u8 as *const libc::c_char,
                    pack,
                );
            }
            if partn != 0 as libc::c_int {
                glp_printf(
                    b"%d partitioning equalities\n\0" as *const u8
                        as *const libc::c_char,
                    partn,
                );
            }
        }
        _ => {}
    }
    return ret;
}
