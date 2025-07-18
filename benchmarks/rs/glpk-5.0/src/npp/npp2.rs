use ::libc;
extern "C" {
    pub type DMP;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn _glp_dmp_get_atom(pool: *mut DMP, size: libc::c_int) -> *mut libc::c_void;
    fn _glp_npp_add_row(npp: *mut NPP) -> *mut NPPROW;
    fn _glp_npp_add_col(npp: *mut NPP) -> *mut NPPCOL;
    fn _glp_npp_add_aij(
        npp: *mut NPP,
        row: *mut NPPROW,
        col: *mut NPPCOL,
        val: libc::c_double,
    ) -> *mut NPPAIJ;
    fn _glp_npp_push_tse(
        npp: *mut NPP,
        func: Option::<unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> libc::c_int>,
        size: libc::c_int,
    ) -> *mut libc::c_void;
    fn _glp_npp_del_row(npp: *mut NPP, row: *mut NPPROW);
    fn _glp_npp_del_col(npp: *mut NPP, col: *mut NPPCOL);
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
pub struct NPPLFE {
    pub ref_0: libc::c_int,
    pub val: libc::c_double,
    pub next: *mut NPPLFE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct free_row {
    pub p: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ineq_row {
    pub p: libc::c_int,
    pub s: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct free_col {
    pub q: libc::c_int,
    pub s: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bnd_col {
    pub q: libc::c_int,
    pub bnd: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dbnd_col {
    pub q: libc::c_int,
    pub s: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fixed_col {
    pub q: libc::c_int,
    pub s: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct make_equality {
    pub p: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct make_fixed {
    pub q: libc::c_int,
    pub c: libc::c_double,
    pub ptr: *mut NPPLFE,
}
pub unsafe extern "C" fn _glp_npp_free_row(mut npp: *mut NPP, mut p: *mut NPPROW) {
    let mut info: *mut free_row = 0 as *mut free_row;
    ((*p).lb == -1.7976931348623157e+308f64 && (*p).ub == 1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"p->lb == -DBL_MAX && p->ub == +DBL_MAX\0" as *const u8
                    as *const libc::c_char,
                b"npp/npp2.c\0" as *const u8 as *const libc::c_char,
                79 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    info = _glp_npp_push_tse(
        npp,
        Some(
            rcv_free_row
                as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> libc::c_int,
        ),
        ::std::mem::size_of::<free_row>() as libc::c_ulong as libc::c_int,
    ) as *mut free_row;
    (*info).p = (*p).i;
    _glp_npp_del_row(npp, p);
}
unsafe extern "C" fn rcv_free_row(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> libc::c_int {
    let mut info: *mut free_row = _info as *mut free_row;
    if (*npp).sol == 1 as libc::c_int {
        *((*npp).r_stat).offset((*info).p as isize) = 1 as libc::c_int as libc::c_char;
    }
    if (*npp).sol != 3 as libc::c_int {
        *((*npp).r_pi).offset((*info).p as isize) = 0.0f64;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_npp_geq_row(mut npp: *mut NPP, mut p: *mut NPPROW) {
    let mut info: *mut ineq_row = 0 as *mut ineq_row;
    let mut s: *mut NPPCOL = 0 as *mut NPPCOL;
    ((*p).lb != -1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"p->lb != -DBL_MAX\0" as *const u8 as *const libc::c_char,
                b"npp/npp2.c\0" as *const u8 as *const libc::c_char,
                196 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*p).lb < (*p).ub
        || {
            glp_assert_(
                b"p->lb < p->ub\0" as *const u8 as *const libc::c_char,
                b"npp/npp2.c\0" as *const u8 as *const libc::c_char,
                197 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    s = _glp_npp_add_col(npp);
    (*s).lb = 0.0f64;
    (*s)
        .ub = if (*p).ub == 1.7976931348623157e+308f64 {
        1.7976931348623157e+308f64
    } else {
        (*p).ub - (*p).lb
    };
    _glp_npp_add_aij(npp, p, s, -1.0f64);
    info = _glp_npp_push_tse(
        npp,
        Some(
            rcv_geq_row
                as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> libc::c_int,
        ),
        ::std::mem::size_of::<ineq_row>() as libc::c_ulong as libc::c_int,
    ) as *mut ineq_row;
    (*info).p = (*p).i;
    (*info).s = (*s).j;
    (*p).ub = (*p).lb;
}
unsafe extern "C" fn rcv_geq_row(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> libc::c_int {
    let mut info: *mut ineq_row = _info as *mut ineq_row;
    if (*npp).sol == 1 as libc::c_int {
        if *((*npp).r_stat).offset((*info).p as isize) as libc::c_int == 1 as libc::c_int
        {
            if *((*npp).c_stat).offset((*info).s as isize) as libc::c_int
                == 1 as libc::c_int
            {
                return 1 as libc::c_int
            } else if *((*npp).c_stat).offset((*info).s as isize) as libc::c_int
                == 2 as libc::c_int
                || *((*npp).c_stat).offset((*info).s as isize) as libc::c_int
                    == 3 as libc::c_int
            {
                *((*npp).r_stat)
                    .offset((*info).p as isize) = 1 as libc::c_int as libc::c_char;
            } else {
                return 1 as libc::c_int
            }
        } else if *((*npp).r_stat).offset((*info).p as isize) as libc::c_int
            == 5 as libc::c_int
        {
            if *((*npp).c_stat).offset((*info).s as isize) as libc::c_int
                == 1 as libc::c_int
            {
                *((*npp).r_stat)
                    .offset((*info).p as isize) = 1 as libc::c_int as libc::c_char;
            } else if *((*npp).c_stat).offset((*info).s as isize) as libc::c_int
                == 2 as libc::c_int
            {
                *((*npp).r_stat)
                    .offset((*info).p as isize) = 2 as libc::c_int as libc::c_char;
            } else if *((*npp).c_stat).offset((*info).s as isize) as libc::c_int
                == 3 as libc::c_int
            {
                *((*npp).r_stat)
                    .offset((*info).p as isize) = 3 as libc::c_int as libc::c_char;
            } else {
                return 1 as libc::c_int
            }
        } else {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_npp_leq_row(mut npp: *mut NPP, mut p: *mut NPPROW) {
    let mut info: *mut ineq_row = 0 as *mut ineq_row;
    let mut s: *mut NPPCOL = 0 as *mut NPPCOL;
    ((*p).ub != 1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"p->ub != +DBL_MAX\0" as *const u8 as *const libc::c_char,
                b"npp/npp2.c\0" as *const u8 as *const libc::c_char,
                340 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*p).lb < (*p).ub
        || {
            glp_assert_(
                b"p->lb < p->ub\0" as *const u8 as *const libc::c_char,
                b"npp/npp2.c\0" as *const u8 as *const libc::c_char,
                341 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    s = _glp_npp_add_col(npp);
    (*s).lb = 0.0f64;
    (*s)
        .ub = if (*p).lb == -1.7976931348623157e+308f64 {
        1.7976931348623157e+308f64
    } else {
        (*p).ub - (*p).lb
    };
    _glp_npp_add_aij(npp, p, s, 1.0f64);
    info = _glp_npp_push_tse(
        npp,
        Some(
            rcv_leq_row
                as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> libc::c_int,
        ),
        ::std::mem::size_of::<ineq_row>() as libc::c_ulong as libc::c_int,
    ) as *mut ineq_row;
    (*info).p = (*p).i;
    (*info).s = (*s).j;
    (*p).lb = (*p).ub;
}
unsafe extern "C" fn rcv_leq_row(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> libc::c_int {
    let mut info: *mut ineq_row = _info as *mut ineq_row;
    if (*npp).sol == 1 as libc::c_int {
        if *((*npp).r_stat).offset((*info).p as isize) as libc::c_int == 1 as libc::c_int
        {
            if *((*npp).c_stat).offset((*info).s as isize) as libc::c_int
                == 1 as libc::c_int
            {
                return 1 as libc::c_int
            } else if *((*npp).c_stat).offset((*info).s as isize) as libc::c_int
                == 2 as libc::c_int
                || *((*npp).c_stat).offset((*info).s as isize) as libc::c_int
                    == 3 as libc::c_int
            {
                *((*npp).r_stat)
                    .offset((*info).p as isize) = 1 as libc::c_int as libc::c_char;
            } else {
                return 1 as libc::c_int
            }
        } else if *((*npp).r_stat).offset((*info).p as isize) as libc::c_int
            == 5 as libc::c_int
        {
            if *((*npp).c_stat).offset((*info).s as isize) as libc::c_int
                == 1 as libc::c_int
            {
                *((*npp).r_stat)
                    .offset((*info).p as isize) = 1 as libc::c_int as libc::c_char;
            } else if *((*npp).c_stat).offset((*info).s as isize) as libc::c_int
                == 2 as libc::c_int
            {
                *((*npp).r_stat)
                    .offset((*info).p as isize) = 3 as libc::c_int as libc::c_char;
            } else if *((*npp).c_stat).offset((*info).s as isize) as libc::c_int
                == 3 as libc::c_int
            {
                *((*npp).r_stat)
                    .offset((*info).p as isize) = 2 as libc::c_int as libc::c_char;
            } else {
                return 1 as libc::c_int
            }
        } else {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_npp_free_col(mut npp: *mut NPP, mut q: *mut NPPCOL) {
    let mut info: *mut free_col = 0 as *mut free_col;
    let mut s: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    ((*q).lb == -1.7976931348623157e+308f64 && (*q).ub == 1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"q->lb == -DBL_MAX && q->ub == +DBL_MAX\0" as *const u8
                    as *const libc::c_char,
                b"npp/npp2.c\0" as *const u8 as *const libc::c_char,
                492 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*q).lb = 0.0f64;
    (*q).ub = 1.7976931348623157e+308f64;
    s = _glp_npp_add_col(npp);
    (*s).is_int = (*q).is_int;
    (*s).lb = 0.0f64;
    (*s).ub = 1.7976931348623157e+308f64;
    (*s).coef = -(*q).coef;
    aij = (*q).ptr;
    while !aij.is_null() {
        _glp_npp_add_aij(npp, (*aij).row, s, -(*aij).val);
        aij = (*aij).c_next;
    }
    info = _glp_npp_push_tse(
        npp,
        Some(
            rcv_free_col
                as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> libc::c_int,
        ),
        ::std::mem::size_of::<free_col>() as libc::c_ulong as libc::c_int,
    ) as *mut free_col;
    (*info).q = (*q).j;
    (*info).s = (*s).j;
}
unsafe extern "C" fn rcv_free_col(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> libc::c_int {
    let mut info: *mut free_col = _info as *mut free_col;
    if (*npp).sol == 1 as libc::c_int {
        if *((*npp).c_stat).offset((*info).q as isize) as libc::c_int == 1 as libc::c_int
        {
            if *((*npp).c_stat).offset((*info).s as isize) as libc::c_int
                == 1 as libc::c_int
            {
                return 1 as libc::c_int
            } else if *((*npp).c_stat).offset((*info).s as isize) as libc::c_int
                == 2 as libc::c_int
            {
                *((*npp).c_stat)
                    .offset((*info).q as isize) = 1 as libc::c_int as libc::c_char;
            } else {
                return -(1 as libc::c_int)
            }
        } else if *((*npp).c_stat).offset((*info).q as isize) as libc::c_int
            == 2 as libc::c_int
        {
            if *((*npp).c_stat).offset((*info).s as isize) as libc::c_int
                == 1 as libc::c_int
            {
                *((*npp).c_stat)
                    .offset((*info).q as isize) = 1 as libc::c_int as libc::c_char;
            } else if *((*npp).c_stat).offset((*info).s as isize) as libc::c_int
                == 2 as libc::c_int
            {
                *((*npp).c_stat)
                    .offset((*info).q as isize) = 4 as libc::c_int as libc::c_char;
            } else {
                return -(1 as libc::c_int)
            }
        } else {
            return -(1 as libc::c_int)
        }
    }
    *((*npp).c_value).offset((*info).q as isize)
        -= *((*npp).c_value).offset((*info).s as isize);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_npp_lbnd_col(mut npp: *mut NPP, mut q: *mut NPPCOL) {
    let mut info: *mut bnd_col = 0 as *mut bnd_col;
    let mut i: *mut NPPROW = 0 as *mut NPPROW;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    ((*q).lb != 0.0f64
        || {
            glp_assert_(
                b"q->lb != 0.0\0" as *const u8 as *const libc::c_char,
                b"npp/npp2.c\0" as *const u8 as *const libc::c_char,
                650 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*q).lb != -1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"q->lb != -DBL_MAX\0" as *const u8 as *const libc::c_char,
                b"npp/npp2.c\0" as *const u8 as *const libc::c_char,
                651 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*q).lb < (*q).ub
        || {
            glp_assert_(
                b"q->lb < q->ub\0" as *const u8 as *const libc::c_char,
                b"npp/npp2.c\0" as *const u8 as *const libc::c_char,
                652 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    info = _glp_npp_push_tse(
        npp,
        Some(
            rcv_lbnd_col
                as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> libc::c_int,
        ),
        ::std::mem::size_of::<bnd_col>() as libc::c_ulong as libc::c_int,
    ) as *mut bnd_col;
    (*info).q = (*q).j;
    (*info).bnd = (*q).lb;
    (*npp).c0 += (*q).coef * (*q).lb;
    aij = (*q).ptr;
    while !aij.is_null() {
        i = (*aij).row;
        if (*i).lb == (*i).ub {
            (*i).lb -= (*aij).val * (*q).lb;
            (*i).ub = (*i).lb;
        } else {
            if (*i).lb != -1.7976931348623157e+308f64 {
                (*i).lb -= (*aij).val * (*q).lb;
            }
            if (*i).ub != 1.7976931348623157e+308f64 {
                (*i).ub -= (*aij).val * (*q).lb;
            }
        }
        aij = (*aij).c_next;
    }
    if (*q).ub != 1.7976931348623157e+308f64 {
        (*q).ub -= (*q).lb;
    }
    (*q).lb = 0.0f64;
}
unsafe extern "C" fn rcv_lbnd_col(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> libc::c_int {
    let mut info: *mut bnd_col = _info as *mut bnd_col;
    if (*npp).sol == 1 as libc::c_int {
        if *((*npp).c_stat).offset((*info).q as isize) as libc::c_int == 1 as libc::c_int
            || *((*npp).c_stat).offset((*info).q as isize) as libc::c_int
                == 2 as libc::c_int
            || *((*npp).c_stat).offset((*info).q as isize) as libc::c_int
                == 3 as libc::c_int
        {
            *((*npp).c_stat)
                .offset(
                    (*info).q as isize,
                ) = *((*npp).c_stat).offset((*info).q as isize);
        } else {
            return 1 as libc::c_int
        }
    }
    *((*npp).c_value)
        .offset(
            (*info).q as isize,
        ) = (*info).bnd + *((*npp).c_value).offset((*info).q as isize);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_npp_ubnd_col(mut npp: *mut NPP, mut q: *mut NPPCOL) {
    let mut info: *mut bnd_col = 0 as *mut bnd_col;
    let mut i: *mut NPPROW = 0 as *mut NPPROW;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    ((*q).ub != 1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"q->ub != +DBL_MAX\0" as *const u8 as *const libc::c_char,
                b"npp/npp2.c\0" as *const u8 as *const libc::c_char,
                818 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*q).lb < (*q).ub
        || {
            glp_assert_(
                b"q->lb < q->ub\0" as *const u8 as *const libc::c_char,
                b"npp/npp2.c\0" as *const u8 as *const libc::c_char,
                819 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    info = _glp_npp_push_tse(
        npp,
        Some(
            rcv_ubnd_col
                as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> libc::c_int,
        ),
        ::std::mem::size_of::<bnd_col>() as libc::c_ulong as libc::c_int,
    ) as *mut bnd_col;
    (*info).q = (*q).j;
    (*info).bnd = (*q).ub;
    (*npp).c0 += (*q).coef * (*q).ub;
    (*q).coef = -(*q).coef;
    aij = (*q).ptr;
    while !aij.is_null() {
        i = (*aij).row;
        if (*i).lb == (*i).ub {
            (*i).lb -= (*aij).val * (*q).ub;
            (*i).ub = (*i).lb;
        } else {
            if (*i).lb != -1.7976931348623157e+308f64 {
                (*i).lb -= (*aij).val * (*q).ub;
            }
            if (*i).ub != 1.7976931348623157e+308f64 {
                (*i).ub -= (*aij).val * (*q).ub;
            }
        }
        (*aij).val = -(*aij).val;
        aij = (*aij).c_next;
    }
    if (*q).lb != -1.7976931348623157e+308f64 {
        (*q).ub -= (*q).lb;
    } else {
        (*q).ub = 1.7976931348623157e+308f64;
    }
    (*q).lb = 0.0f64;
}
unsafe extern "C" fn rcv_ubnd_col(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> libc::c_int {
    let mut info: *mut bnd_col = _info as *mut bnd_col;
    if (*npp).sol == 1 as libc::c_int {
        if *((*npp).c_stat).offset((*info).q as isize) as libc::c_int == 1 as libc::c_int
        {
            *((*npp).c_stat)
                .offset((*info).q as isize) = 1 as libc::c_int as libc::c_char;
        } else if *((*npp).c_stat).offset((*info).q as isize) as libc::c_int
            == 2 as libc::c_int
        {
            *((*npp).c_stat)
                .offset((*info).q as isize) = 3 as libc::c_int as libc::c_char;
        } else if *((*npp).c_stat).offset((*info).q as isize) as libc::c_int
            == 3 as libc::c_int
        {
            *((*npp).c_stat)
                .offset((*info).q as isize) = 2 as libc::c_int as libc::c_char;
        } else {
            return 1 as libc::c_int
        }
    }
    *((*npp).c_value)
        .offset(
            (*info).q as isize,
        ) = (*info).bnd - *((*npp).c_value).offset((*info).q as isize);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_npp_dbnd_col(mut npp: *mut NPP, mut q: *mut NPPCOL) {
    let mut info: *mut dbnd_col = 0 as *mut dbnd_col;
    let mut p: *mut NPPROW = 0 as *mut NPPROW;
    let mut s: *mut NPPCOL = 0 as *mut NPPCOL;
    ((*q).lb == 0.0f64
        || {
            glp_assert_(
                b"q->lb == 0.0\0" as *const u8 as *const libc::c_char,
                b"npp/npp2.c\0" as *const u8 as *const libc::c_char,
                966 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*q).ub > 0.0f64
        || {
            glp_assert_(
                b"q->ub > 0.0\0" as *const u8 as *const libc::c_char,
                b"npp/npp2.c\0" as *const u8 as *const libc::c_char,
                967 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*q).ub != 1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"q->ub != +DBL_MAX\0" as *const u8 as *const libc::c_char,
                b"npp/npp2.c\0" as *const u8 as *const libc::c_char,
                968 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    s = _glp_npp_add_col(npp);
    (*s).is_int = (*q).is_int;
    (*s).lb = 0.0f64;
    (*s).ub = 1.7976931348623157e+308f64;
    p = _glp_npp_add_row(npp);
    (*p).ub = (*q).ub;
    (*p).lb = (*p).ub;
    _glp_npp_add_aij(npp, p, q, 1.0f64);
    _glp_npp_add_aij(npp, p, s, 1.0f64);
    info = _glp_npp_push_tse(
        npp,
        Some(
            rcv_dbnd_col
                as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> libc::c_int,
        ),
        ::std::mem::size_of::<dbnd_col>() as libc::c_ulong as libc::c_int,
    ) as *mut dbnd_col;
    (*info).q = (*q).j;
    (*info).s = (*s).j;
    (*q).ub = 1.7976931348623157e+308f64;
}
unsafe extern "C" fn rcv_dbnd_col(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> libc::c_int {
    let mut info: *mut dbnd_col = _info as *mut dbnd_col;
    if (*npp).sol == 1 as libc::c_int {
        if *((*npp).c_stat).offset((*info).q as isize) as libc::c_int == 1 as libc::c_int
        {
            if *((*npp).c_stat).offset((*info).s as isize) as libc::c_int
                == 1 as libc::c_int
            {
                *((*npp).c_stat)
                    .offset((*info).q as isize) = 1 as libc::c_int as libc::c_char;
            } else if *((*npp).c_stat).offset((*info).s as isize) as libc::c_int
                == 2 as libc::c_int
            {
                *((*npp).c_stat)
                    .offset((*info).q as isize) = 3 as libc::c_int as libc::c_char;
            } else {
                return 1 as libc::c_int
            }
        } else if *((*npp).c_stat).offset((*info).q as isize) as libc::c_int
            == 2 as libc::c_int
        {
            if *((*npp).c_stat).offset((*info).s as isize) as libc::c_int
                == 1 as libc::c_int
                || *((*npp).c_stat).offset((*info).s as isize) as libc::c_int
                    == 2 as libc::c_int
            {
                *((*npp).c_stat)
                    .offset((*info).q as isize) = 2 as libc::c_int as libc::c_char;
            } else {
                return 1 as libc::c_int
            }
        } else {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_npp_fixed_col(mut npp: *mut NPP, mut q: *mut NPPCOL) {
    let mut info: *mut fixed_col = 0 as *mut fixed_col;
    let mut i: *mut NPPROW = 0 as *mut NPPROW;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    ((*q).lb == (*q).ub
        || {
            glp_assert_(
                b"q->lb == q->ub\0" as *const u8 as *const libc::c_char,
                b"npp/npp2.c\0" as *const u8 as *const libc::c_char,
                1110 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    info = _glp_npp_push_tse(
        npp,
        Some(
            rcv_fixed_col
                as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> libc::c_int,
        ),
        ::std::mem::size_of::<fixed_col>() as libc::c_ulong as libc::c_int,
    ) as *mut fixed_col;
    (*info).q = (*q).j;
    (*info).s = (*q).lb;
    (*npp).c0 += (*q).coef * (*q).lb;
    aij = (*q).ptr;
    while !aij.is_null() {
        i = (*aij).row;
        if (*i).lb == (*i).ub {
            (*i).lb -= (*aij).val * (*q).lb;
            (*i).ub = (*i).lb;
        } else {
            if (*i).lb != -1.7976931348623157e+308f64 {
                (*i).lb -= (*aij).val * (*q).lb;
            }
            if (*i).ub != 1.7976931348623157e+308f64 {
                (*i).ub -= (*aij).val * (*q).lb;
            }
        }
        aij = (*aij).c_next;
    }
    _glp_npp_del_col(npp, q);
}
unsafe extern "C" fn rcv_fixed_col(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> libc::c_int {
    let mut info: *mut fixed_col = _info as *mut fixed_col;
    if (*npp).sol == 1 as libc::c_int {
        *((*npp).c_stat).offset((*info).q as isize) = 5 as libc::c_int as libc::c_char;
    }
    *((*npp).c_value).offset((*info).q as isize) = (*info).s;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_npp_make_equality(
    mut npp: *mut NPP,
    mut p: *mut NPPROW,
) -> libc::c_int {
    let mut info: *mut make_equality = 0 as *mut make_equality;
    let mut b: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    let mut nint: libc::c_double = 0.;
    ((*p).lb != -1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"p->lb != -DBL_MAX\0" as *const u8 as *const libc::c_char,
                b"npp/npp2.c\0" as *const u8 as *const libc::c_char,
                1229 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*p).ub != 1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"p->ub != +DBL_MAX\0" as *const u8 as *const libc::c_char,
                b"npp/npp2.c\0" as *const u8 as *const libc::c_char,
                1230 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*p).lb < (*p).ub
        || {
            glp_assert_(
                b"p->lb < p->ub\0" as *const u8 as *const libc::c_char,
                b"npp/npp2.c\0" as *const u8 as *const libc::c_char,
                1231 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    eps = 1e-9f64 + 1e-12f64 * fabs((*p).lb);
    if (*p).ub - (*p).lb > eps {
        return 0 as libc::c_int;
    }
    info = _glp_npp_push_tse(
        npp,
        Some(
            rcv_make_equality
                as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> libc::c_int,
        ),
        ::std::mem::size_of::<make_equality>() as libc::c_ulong as libc::c_int,
    ) as *mut make_equality;
    (*info).p = (*p).i;
    b = 0.5f64 * ((*p).ub + (*p).lb);
    nint = floor(b + 0.5f64);
    if fabs(b - nint) <= eps {
        b = nint;
    }
    (*p).ub = b;
    (*p).lb = (*p).ub;
    return 1 as libc::c_int;
}
unsafe extern "C" fn rcv_make_equality(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> libc::c_int {
    let mut info: *mut make_equality = _info as *mut make_equality;
    if (*npp).sol == 1 as libc::c_int {
        if *((*npp).r_stat).offset((*info).p as isize) as libc::c_int == 1 as libc::c_int
        {
            *((*npp).r_stat)
                .offset((*info).p as isize) = 1 as libc::c_int as libc::c_char;
        } else if *((*npp).r_stat).offset((*info).p as isize) as libc::c_int
            == 5 as libc::c_int
        {
            if *((*npp).r_pi).offset((*info).p as isize) >= 0.0f64 {
                *((*npp).r_stat)
                    .offset((*info).p as isize) = 2 as libc::c_int as libc::c_char;
            } else {
                *((*npp).r_stat)
                    .offset((*info).p as isize) = 3 as libc::c_int as libc::c_char;
            }
        } else {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_npp_make_fixed(
    mut npp: *mut NPP,
    mut q: *mut NPPCOL,
) -> libc::c_int {
    let mut info: *mut make_fixed = 0 as *mut make_fixed;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut lfe: *mut NPPLFE = 0 as *mut NPPLFE;
    let mut s: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    let mut nint: libc::c_double = 0.;
    ((*q).lb != -1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"q->lb != -DBL_MAX\0" as *const u8 as *const libc::c_char,
                b"npp/npp2.c\0" as *const u8 as *const libc::c_char,
                1371 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*q).ub != 1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"q->ub != +DBL_MAX\0" as *const u8 as *const libc::c_char,
                b"npp/npp2.c\0" as *const u8 as *const libc::c_char,
                1372 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*q).lb < (*q).ub
        || {
            glp_assert_(
                b"q->lb < q->ub\0" as *const u8 as *const libc::c_char,
                b"npp/npp2.c\0" as *const u8 as *const libc::c_char,
                1373 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    eps = 1e-9f64 + 1e-12f64 * fabs((*q).lb);
    if (*q).ub - (*q).lb > eps {
        return 0 as libc::c_int;
    }
    info = _glp_npp_push_tse(
        npp,
        Some(
            rcv_make_fixed
                as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> libc::c_int,
        ),
        ::std::mem::size_of::<make_fixed>() as libc::c_ulong as libc::c_int,
    ) as *mut make_fixed;
    (*info).q = (*q).j;
    (*info).c = (*q).coef;
    (*info).ptr = 0 as *mut NPPLFE;
    if (*npp).sol == 1 as libc::c_int {
        aij = (*q).ptr;
        while !aij.is_null() {
            lfe = _glp_dmp_get_atom(
                (*npp).stack,
                ::std::mem::size_of::<NPPLFE>() as libc::c_ulong as libc::c_int,
            ) as *mut NPPLFE;
            (*lfe).ref_0 = (*(*aij).row).i;
            (*lfe).val = (*aij).val;
            (*lfe).next = (*info).ptr;
            (*info).ptr = lfe;
            aij = (*aij).c_next;
        }
    }
    s = 0.5f64 * ((*q).ub + (*q).lb);
    nint = floor(s + 0.5f64);
    if fabs(s - nint) <= eps {
        s = nint;
    }
    (*q).ub = s;
    (*q).lb = (*q).ub;
    return 1 as libc::c_int;
}
unsafe extern "C" fn rcv_make_fixed(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> libc::c_int {
    let mut info: *mut make_fixed = _info as *mut make_fixed;
    let mut lfe: *mut NPPLFE = 0 as *mut NPPLFE;
    let mut lambda: libc::c_double = 0.;
    if (*npp).sol == 1 as libc::c_int {
        if *((*npp).c_stat).offset((*info).q as isize) as libc::c_int == 1 as libc::c_int
        {
            *((*npp).c_stat)
                .offset((*info).q as isize) = 1 as libc::c_int as libc::c_char;
        } else if *((*npp).c_stat).offset((*info).q as isize) as libc::c_int
            == 5 as libc::c_int
        {
            lambda = (*info).c;
            lfe = (*info).ptr;
            while !lfe.is_null() {
                lambda -= (*lfe).val * *((*npp).r_pi).offset((*lfe).ref_0 as isize);
                lfe = (*lfe).next;
            }
            if lambda >= 0.0f64 {
                *((*npp).c_stat)
                    .offset((*info).q as isize) = 2 as libc::c_int as libc::c_char;
            } else {
                *((*npp).c_stat)
                    .offset((*info).q as isize) = 3 as libc::c_int as libc::c_char;
            }
        } else {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
