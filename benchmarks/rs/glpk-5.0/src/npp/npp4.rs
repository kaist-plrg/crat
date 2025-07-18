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
    fn _glp_dmp_get_atom(pool: *mut DMP, size: libc::c_int) -> *mut libc::c_void;
    fn _glp_dmp_free_atom(pool: *mut DMP, atom: *mut libc::c_void, size: libc::c_int);
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
    fn _glp_npp_erase_row(npp: *mut NPP, row: *mut NPPROW);
    fn _glp_npp_lbnd_col(npp: *mut NPP, q: *mut NPPCOL);
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
pub struct binarize {
    pub q: libc::c_int,
    pub j: libc::c_int,
    pub n: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elem {
    pub aj: libc::c_double,
    pub xj: *mut NPPCOL,
    pub next: *mut elem,
}
pub unsafe extern "C" fn _glp_npp_binarize_prob(mut npp: *mut NPP) -> libc::c_int {
    let mut info: *mut binarize = 0 as *mut binarize;
    let mut row: *mut NPPROW = 0 as *mut NPPROW;
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut bin: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut u: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut temp: libc::c_int = 0;
    let mut nfails: libc::c_int = 0;
    let mut nvars: libc::c_int = 0;
    let mut nbins: libc::c_int = 0;
    let mut nrows: libc::c_int = 0;
    nrows = 0 as libc::c_int;
    nbins = nrows;
    nvars = nbins;
    nfails = nvars;
    col = (*npp).c_tail;
    while !col.is_null() {
        if !((*col).is_int == 0) {
            if !((*col).lb == (*col).ub) {
                if !((*col).lb == 0.0f64 && (*col).ub == 1.0f64) {
                    if (*col).lb < -1e6f64 || (*col).ub > 1e6f64
                        || (*col).ub - (*col).lb > 4095.0f64
                    {
                        nfails += 1;
                        nfails;
                    } else {
                        nvars += 1;
                        nvars;
                        if (*col).lb != 0.0f64 {
                            _glp_npp_lbnd_col(npp, col);
                        }
                        ((*col).lb == 0.0f64
                            || {
                                glp_assert_(
                                    b"col->lb == 0.0\0" as *const u8 as *const libc::c_char,
                                    b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                                    159 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        u = (*col).ub as libc::c_int;
                        ((*col).ub == u as libc::c_double
                            || {
                                glp_assert_(
                                    b"col->ub == (double)u\0" as *const u8
                                        as *const libc::c_char,
                                    b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                                    161 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        if !(u == 1 as libc::c_int) {
                            n = 2 as libc::c_int;
                            temp = 4 as libc::c_int;
                            while u >= temp {
                                n += 1;
                                n;
                                temp += temp;
                            }
                            nbins += n;
                            info = _glp_npp_push_tse(
                                npp,
                                Some(
                                    rcv_binarize_prob
                                        as unsafe extern "C" fn(
                                            *mut NPP,
                                            *mut libc::c_void,
                                        ) -> libc::c_int,
                                ),
                                ::std::mem::size_of::<binarize>() as libc::c_ulong
                                    as libc::c_int,
                            ) as *mut binarize;
                            (*info).q = (*col).j;
                            (*info).j = 0 as libc::c_int;
                            (*info).n = n;
                            if u < temp - 1 as libc::c_int {
                                row = _glp_npp_add_row(npp);
                                nrows += 1;
                                nrows;
                                (*row).lb = -1.7976931348623157e+308f64;
                                (*row).ub = u as libc::c_double;
                            } else {
                                row = 0 as *mut NPPROW;
                            }
                            (*col).ub = 1.0f64;
                            if !row.is_null() {
                                _glp_npp_add_aij(npp, row, col, 1.0f64);
                            }
                            k = 1 as libc::c_int;
                            temp = 2 as libc::c_int;
                            while k < n {
                                bin = _glp_npp_add_col(npp);
                                (*bin).is_int = 1 as libc::c_int as libc::c_char;
                                (*bin).lb = 0.0f64;
                                (*bin).ub = 1.0f64;
                                (*bin).coef = temp as libc::c_double * (*col).coef;
                                if (*info).j == 0 as libc::c_int {
                                    (*info).j = (*bin).j;
                                } else {
                                    ((*info).j + (k - 1 as libc::c_int) == (*bin).j
                                        || {
                                            glp_assert_(
                                                b"info->j + (k-1) == bin->j\0" as *const u8
                                                    as *const libc::c_char,
                                                b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                                                201 as libc::c_int,
                                            );
                                            1 as libc::c_int != 0
                                        }) as libc::c_int;
                                }
                                aij = (*col).ptr;
                                while !aij.is_null() {
                                    _glp_npp_add_aij(
                                        npp,
                                        (*aij).row,
                                        bin,
                                        temp as libc::c_double * (*aij).val,
                                    );
                                    aij = (*aij).c_next;
                                }
                                k += 1;
                                k;
                                temp += temp;
                            }
                        }
                    }
                }
            }
        }
        col = (*col).prev;
    }
    if nvars > 0 as libc::c_int {
        glp_printf(
            b"%d integer variable(s) were replaced by %d binary ones\n\0" as *const u8
                as *const libc::c_char,
            nvars,
            nbins,
        );
    }
    if nrows > 0 as libc::c_int {
        glp_printf(
            b"%d row(s) were added due to binarization\n\0" as *const u8
                as *const libc::c_char,
            nrows,
        );
    }
    if nfails > 0 as libc::c_int {
        glp_printf(
            b"Binarization failed for %d integer variable(s)\n\0" as *const u8
                as *const libc::c_char,
            nfails,
        );
    }
    return nfails;
}
unsafe extern "C" fn rcv_binarize_prob(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> libc::c_int {
    let mut info: *mut binarize = _info as *mut binarize;
    let mut k: libc::c_int = 0;
    let mut temp: libc::c_int = 0;
    let mut sum: libc::c_double = 0.;
    sum = *((*npp).c_value).offset((*info).q as isize);
    k = 1 as libc::c_int;
    temp = 2 as libc::c_int;
    while k < (*info).n {
        sum
            += temp as libc::c_double
                * *((*npp).c_value)
                    .offset(((*info).j + (k - 1 as libc::c_int)) as isize);
        k += 1;
        k;
        temp += temp;
    }
    *((*npp).c_value).offset((*info).q as isize) = sum;
    return 0 as libc::c_int;
}
unsafe extern "C" fn copy_form(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
    mut s: libc::c_double,
) -> *mut elem {
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut ptr: *mut elem = 0 as *mut elem;
    let mut e: *mut elem = 0 as *mut elem;
    ptr = 0 as *mut elem;
    aij = (*row).ptr;
    while !aij.is_null() {
        e = _glp_dmp_get_atom(
            (*npp).pool,
            ::std::mem::size_of::<elem>() as libc::c_ulong as libc::c_int,
        ) as *mut elem;
        (*e).aj = s * (*aij).val;
        (*e).xj = (*aij).col;
        (*e).next = ptr;
        ptr = e;
        aij = (*aij).r_next;
    }
    return ptr;
}
unsafe extern "C" fn drop_form(mut npp: *mut NPP, mut ptr: *mut elem) {
    let mut e: *mut elem = 0 as *mut elem;
    while !ptr.is_null() {
        e = ptr;
        ptr = (*e).next;
        _glp_dmp_free_atom(
            (*npp).pool,
            e as *mut libc::c_void,
            ::std::mem::size_of::<elem>() as libc::c_ulong as libc::c_int,
        );
    }
}
pub unsafe extern "C" fn _glp_npp_is_packing(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> libc::c_int {
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut b: libc::c_int = 0;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const libc::c_char,
                b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                316 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if !((*row).lb == -1.7976931348623157e+308f64
        && (*row).ub != 1.7976931348623157e+308f64)
    {
        return 0 as libc::c_int;
    }
    b = 1 as libc::c_int;
    aij = (*row).ptr;
    while !aij.is_null() {
        col = (*aij).col;
        if !((*col).is_int as libc::c_int != 0 && (*col).lb == 0.0f64
            && (*col).ub == 1.0f64)
        {
            return 0 as libc::c_int;
        }
        if !((*aij).val == 1.0f64) {
            if (*aij).val == -1.0f64 {
                b -= 1;
                b;
            } else {
                return 0 as libc::c_int
            }
        }
        aij = (*aij).r_next;
    }
    if (*row).ub != b as libc::c_double {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn hidden_packing(
    mut npp: *mut NPP,
    mut ptr: *mut elem,
    mut _b: *mut libc::c_double,
) -> libc::c_int {
    let mut e: *mut elem = 0 as *mut elem;
    let mut ej: *mut elem = 0 as *mut elem;
    let mut ek: *mut elem = 0 as *mut elem;
    let mut neg: libc::c_int = 0;
    let mut b: libc::c_double = *_b;
    let mut eps: libc::c_double = 0.;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const libc::c_char,
                b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                435 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    e = ptr;
    while !e.is_null() {
        ((*e).aj != 0.0f64
            || {
                glp_assert_(
                    b"e->aj != 0.0\0" as *const u8 as *const libc::c_char,
                    b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                    438 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        ((*(*e).xj).is_int as libc::c_int != 0
            || {
                glp_assert_(
                    b"e->xj->is_int\0" as *const u8 as *const libc::c_char,
                    b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                    439 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        ((*(*e).xj).lb == 0.0f64 && (*(*e).xj).ub == 1.0f64
            || {
                glp_assert_(
                    b"e->xj->lb == 0.0 && e->xj->ub == 1.0\0" as *const u8
                        as *const libc::c_char,
                    b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                    440 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        e = (*e).next;
    }
    neg = 0 as libc::c_int;
    e = ptr;
    while !e.is_null() {
        if !((*e).aj == 1.0f64) {
            if !((*e).aj == -1.0f64) {
                break;
            }
            neg += 1;
            neg;
        }
        e = (*e).next;
    }
    if e.is_null() {
        if b == (1 as libc::c_int - neg) as libc::c_double {
            return 1 as libc::c_int;
        }
    }
    e = ptr;
    while !e.is_null() {
        if (*e).aj < 0 as libc::c_int as libc::c_double {
            b -= (*e).aj;
        }
        e = (*e).next;
    }
    e = ptr;
    while !e.is_null() {
        if fabs((*e).aj) > b {
            return 0 as libc::c_int;
        }
        e = (*e).next;
    }
    ej = 0 as *mut elem;
    e = ptr;
    while !e.is_null() {
        if ej.is_null() || fabs((*ej).aj) > fabs((*e).aj) {
            ej = e;
        }
        e = (*e).next;
    }
    (!ej.is_null()
        || {
            glp_assert_(
                b"ej != NULL\0" as *const u8 as *const libc::c_char,
                b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                473 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ek = 0 as *mut elem;
    e = ptr;
    while !e.is_null() {
        if e != ej {
            if ek.is_null() || fabs((*ek).aj) > fabs((*e).aj) {
                ek = e;
            }
        }
        e = (*e).next;
    }
    (!ek.is_null()
        || {
            glp_assert_(
                b"ek != NULL\0" as *const u8 as *const libc::c_char,
                b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                478 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    eps = 1e-3f64 + 1e-6f64 * fabs(b);
    if fabs((*ej).aj) + fabs((*ek).aj) <= b + eps {
        return 0 as libc::c_int;
    }
    b = 1.0f64;
    e = ptr;
    while !e.is_null() {
        if (*e).aj > 0.0f64 {
            (*e).aj = 1.0f64;
        } else {
            (*e).aj = -1.0f64;
            b -= 1.0f64;
        }
        e = (*e).next;
    }
    *_b = b;
    return 2 as libc::c_int;
}
pub unsafe extern "C" fn _glp_npp_hidden_packing(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> libc::c_int {
    let mut copy: *mut NPPROW = 0 as *mut NPPROW;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut ptr: *mut elem = 0 as *mut elem;
    let mut e: *mut elem = 0 as *mut elem;
    let mut kase: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut b: libc::c_double = 0.;
    ((*row).lb < (*row).ub
        || {
            glp_assert_(
                b"row->lb < row->ub\0" as *const u8 as *const libc::c_char,
                b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                504 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    let mut current_block_31: u64;
    kase = 0 as libc::c_int;
    while kase <= 1 as libc::c_int {
        if kase == 0 as libc::c_int {
            if (*row).ub == 1.7976931348623157e+308f64 {
                current_block_31 = 12675440807659640239;
            } else {
                ptr = copy_form(npp, row, 1.0f64);
                b = (*row).ub;
                current_block_31 = 11650488183268122163;
            }
        } else if (*row).lb == -1.7976931348623157e+308f64 {
            current_block_31 = 12675440807659640239;
        } else {
            ptr = copy_form(npp, row, -1.0f64);
            b = -(*row).lb;
            current_block_31 = 11650488183268122163;
        }
        match current_block_31 {
            11650488183268122163 => {
                ret = hidden_packing(npp, ptr, &mut b);
                (0 as libc::c_int <= ret && ret <= 2 as libc::c_int
                    || {
                        glp_assert_(
                            b"0 <= ret && ret <= 2\0" as *const u8
                                as *const libc::c_char,
                            b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                            520 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                if kase == 1 as libc::c_int && ret == 1 as libc::c_int
                    || ret == 2 as libc::c_int
                {
                    count += 1;
                    count;
                    if (*row).lb == -1.7976931348623157e+308f64
                        || (*row).ub == 1.7976931348623157e+308f64
                    {
                        copy = 0 as *mut NPPROW;
                    } else {
                        copy = _glp_npp_add_row(npp);
                        if kase == 0 as libc::c_int {
                            (*copy).lb = (*row).lb;
                            (*copy).ub = 1.7976931348623157e+308f64;
                        } else {
                            (*copy).lb = -1.7976931348623157e+308f64;
                            (*copy).ub = (*row).ub;
                        }
                        aij = (*row).ptr;
                        while !aij.is_null() {
                            _glp_npp_add_aij(npp, copy, (*aij).col, (*aij).val);
                            aij = (*aij).r_next;
                        }
                    }
                    _glp_npp_erase_row(npp, row);
                    (*row).lb = -1.7976931348623157e+308f64;
                    (*row).ub = b;
                    e = ptr;
                    while !e.is_null() {
                        _glp_npp_add_aij(npp, row, (*e).xj, (*e).aj);
                        e = (*e).next;
                    }
                    if !copy.is_null() {
                        row = copy;
                    }
                }
                drop_form(npp, ptr);
            }
            _ => {}
        }
        kase += 1;
        kase;
    }
    return count;
}
pub unsafe extern "C" fn _glp_npp_implied_packing(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
    mut which: libc::c_int,
    mut var: *mut *mut NPPCOL,
    mut set: *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ptr: *mut elem = 0 as *mut elem;
    let mut e: *mut elem = 0 as *mut elem;
    let mut i: *mut elem = 0 as *mut elem;
    let mut k: *mut elem = 0 as *mut elem;
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut b: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    if which == 0 as libc::c_int {
        ptr = copy_form(npp, row, -1.0f64);
        ((*row).lb != -1.7976931348623157e+308f64
            || {
                glp_assert_(
                    b"row->lb != -DBL_MAX\0" as *const u8 as *const libc::c_char,
                    b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                    721 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        b = -(*row).lb;
    } else if which == 1 as libc::c_int {
        ptr = copy_form(npp, row, 1.0f64);
        ((*row).ub != 1.7976931348623157e+308f64
            || {
                glp_assert_(
                    b"row->ub != +DBL_MAX\0" as *const u8 as *const libc::c_char,
                    b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                    726 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        b = (*row).ub;
    }
    e = ptr;
    loop {
        if e.is_null() {
            current_block = 5948590327928692120;
            break;
        }
        if !((*(*e).xj).is_int as libc::c_int != 0 && (*(*e).xj).lb == 0.0f64
            && (*(*e).xj).ub == 1.0f64)
        {
            if (*e).aj > 0.0f64 {
                if (*(*e).xj).lb == -1.7976931348623157e+308f64 {
                    current_block = 15854331283297482205;
                    break;
                }
                b -= (*e).aj * (*(*e).xj).lb;
            } else {
                if (*(*e).xj).ub == 1.7976931348623157e+308f64 {
                    current_block = 15854331283297482205;
                    break;
                }
                b -= (*e).aj * (*(*e).xj).ub;
            }
            (*e).aj = 0.0f64;
        }
        e = (*e).next;
    }
    match current_block {
        5948590327928692120 => {
            e = ptr;
            while !e.is_null() {
                if (*e).aj < 0.0f64 {
                    b -= (*e).aj;
                }
                e = (*e).next;
            }
            if !(b < 1e-3f64) {
                eps = 1e-3f64 + 1e-6f64 * b;
                k = 0 as *mut elem;
                i = k;
                e = ptr;
                while !e.is_null() {
                    if fabs((*e).aj) > 0.5f64 * (b + eps) {
                        len += 1;
                        let ref mut fresh0 = *var.offset(len as isize);
                        *fresh0 = (*e).xj;
                        *set
                            .offset(
                                len as isize,
                            ) = (if (*e).aj > 0.0f64 {
                            0 as libc::c_int
                        } else {
                            1 as libc::c_int
                        }) as libc::c_char;
                        if i.is_null() || fabs((*i).aj) > fabs((*e).aj) {
                            i = e;
                        }
                    } else if fabs((*e).aj) >= 1e-3f64 {
                        if k.is_null() || fabs((*k).aj) < fabs((*e).aj) {
                            k = e;
                        }
                    }
                    e = (*e).next;
                }
                if !i.is_null() && !k.is_null()
                    && fabs((*i).aj) + fabs((*k).aj) > b + eps
                {
                    len += 1;
                    let ref mut fresh1 = *var.offset(len as isize);
                    *fresh1 = (*k).xj;
                    *set
                        .offset(
                            len as isize,
                        ) = (if (*k).aj > 0.0f64 {
                        0 as libc::c_int
                    } else {
                        1 as libc::c_int
                    }) as libc::c_char;
                }
                if len < 2 as libc::c_int {
                    len = 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    drop_form(npp, ptr);
    return len;
}
pub unsafe extern "C" fn _glp_npp_is_covering(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> libc::c_int {
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut b: libc::c_int = 0;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const libc::c_char,
                b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                841 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if !((*row).lb != -1.7976931348623157e+308f64
        && (*row).ub == 1.7976931348623157e+308f64)
    {
        return 0 as libc::c_int;
    }
    b = 1 as libc::c_int;
    aij = (*row).ptr;
    while !aij.is_null() {
        col = (*aij).col;
        if !((*col).is_int as libc::c_int != 0 && (*col).lb == 0.0f64
            && (*col).ub == 1.0f64)
        {
            return 0 as libc::c_int;
        }
        if !((*aij).val == 1.0f64) {
            if (*aij).val == -1.0f64 {
                b -= 1;
                b;
            } else {
                return 0 as libc::c_int
            }
        }
        aij = (*aij).r_next;
    }
    if (*row).lb != b as libc::c_double {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn hidden_covering(
    mut npp: *mut NPP,
    mut ptr: *mut elem,
    mut _b: *mut libc::c_double,
) -> libc::c_int {
    let mut e: *mut elem = 0 as *mut elem;
    let mut neg: libc::c_int = 0;
    let mut b: libc::c_double = *_b;
    let mut eps: libc::c_double = 0.;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const libc::c_char,
                b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                938 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    e = ptr;
    while !e.is_null() {
        ((*e).aj != 0.0f64
            || {
                glp_assert_(
                    b"e->aj != 0.0\0" as *const u8 as *const libc::c_char,
                    b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                    941 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        ((*(*e).xj).is_int as libc::c_int != 0
            || {
                glp_assert_(
                    b"e->xj->is_int\0" as *const u8 as *const libc::c_char,
                    b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                    942 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        ((*(*e).xj).lb == 0.0f64 && (*(*e).xj).ub == 1.0f64
            || {
                glp_assert_(
                    b"e->xj->lb == 0.0 && e->xj->ub == 1.0\0" as *const u8
                        as *const libc::c_char,
                    b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                    943 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        e = (*e).next;
    }
    neg = 0 as libc::c_int;
    e = ptr;
    while !e.is_null() {
        if !((*e).aj == 1.0f64) {
            if !((*e).aj == -1.0f64) {
                break;
            }
            neg += 1;
            neg;
        }
        e = (*e).next;
    }
    if e.is_null() {
        if b == (1 as libc::c_int - neg) as libc::c_double {
            return 1 as libc::c_int;
        }
    }
    e = ptr;
    while !e.is_null() {
        if (*e).aj < 0 as libc::c_int as libc::c_double {
            b -= (*e).aj;
        }
        e = (*e).next;
    }
    if b < 1e-3f64 {
        return 0 as libc::c_int;
    }
    eps = 1e-9f64 + 1e-12f64 * fabs(b);
    e = ptr;
    while !e.is_null() {
        if fabs((*e).aj) < b - eps {
            return 0 as libc::c_int;
        }
        e = (*e).next;
    }
    b = 1.0f64;
    e = ptr;
    while !e.is_null() {
        if (*e).aj > 0.0f64 {
            (*e).aj = 1.0f64;
        } else {
            (*e).aj = -1.0f64;
            b -= 1.0f64;
        }
        e = (*e).next;
    }
    *_b = b;
    return 2 as libc::c_int;
}
pub unsafe extern "C" fn _glp_npp_hidden_covering(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> libc::c_int {
    let mut copy: *mut NPPROW = 0 as *mut NPPROW;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut ptr: *mut elem = 0 as *mut elem;
    let mut e: *mut elem = 0 as *mut elem;
    let mut kase: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut b: libc::c_double = 0.;
    ((*row).lb < (*row).ub
        || {
            glp_assert_(
                b"row->lb < row->ub\0" as *const u8 as *const libc::c_char,
                b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                997 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    let mut current_block_31: u64;
    kase = 0 as libc::c_int;
    while kase <= 1 as libc::c_int {
        if kase == 0 as libc::c_int {
            if (*row).lb == -1.7976931348623157e+308f64 {
                current_block_31 = 12675440807659640239;
            } else {
                ptr = copy_form(npp, row, 1.0f64);
                b = (*row).lb;
                current_block_31 = 11650488183268122163;
            }
        } else if (*row).ub == 1.7976931348623157e+308f64 {
            current_block_31 = 12675440807659640239;
        } else {
            ptr = copy_form(npp, row, -1.0f64);
            b = -(*row).ub;
            current_block_31 = 11650488183268122163;
        }
        match current_block_31 {
            11650488183268122163 => {
                ret = hidden_covering(npp, ptr, &mut b);
                (0 as libc::c_int <= ret && ret <= 2 as libc::c_int
                    || {
                        glp_assert_(
                            b"0 <= ret && ret <= 2\0" as *const u8
                                as *const libc::c_char,
                            b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                            1013 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                if kase == 1 as libc::c_int && ret == 1 as libc::c_int
                    || ret == 2 as libc::c_int
                {
                    count += 1;
                    count;
                    if (*row).lb == -1.7976931348623157e+308f64
                        || (*row).ub == 1.7976931348623157e+308f64
                    {
                        copy = 0 as *mut NPPROW;
                    } else {
                        copy = _glp_npp_add_row(npp);
                        if kase == 0 as libc::c_int {
                            (*copy).lb = -1.7976931348623157e+308f64;
                            (*copy).ub = (*row).ub;
                        } else {
                            (*copy).lb = (*row).lb;
                            (*copy).ub = 1.7976931348623157e+308f64;
                        }
                        aij = (*row).ptr;
                        while !aij.is_null() {
                            _glp_npp_add_aij(npp, copy, (*aij).col, (*aij).val);
                            aij = (*aij).r_next;
                        }
                    }
                    _glp_npp_erase_row(npp, row);
                    (*row).lb = b;
                    (*row).ub = 1.7976931348623157e+308f64;
                    e = ptr;
                    while !e.is_null() {
                        _glp_npp_add_aij(npp, row, (*e).xj, (*e).aj);
                        e = (*e).next;
                    }
                    if !copy.is_null() {
                        row = copy;
                    }
                }
                drop_form(npp, ptr);
            }
            _ => {}
        }
        kase += 1;
        kase;
    }
    return count;
}
pub unsafe extern "C" fn _glp_npp_is_partitioning(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> libc::c_int {
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut b: libc::c_int = 0;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const libc::c_char,
                b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                1113 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*row).lb != (*row).ub {
        return 0 as libc::c_int;
    }
    b = 1 as libc::c_int;
    aij = (*row).ptr;
    while !aij.is_null() {
        col = (*aij).col;
        if !((*col).is_int as libc::c_int != 0 && (*col).lb == 0.0f64
            && (*col).ub == 1.0f64)
        {
            return 0 as libc::c_int;
        }
        if !((*aij).val == 1.0f64) {
            if (*aij).val == -1.0f64 {
                b -= 1;
                b;
            } else {
                return 0 as libc::c_int
            }
        }
        aij = (*aij).r_next;
    }
    if (*row).lb != b as libc::c_double {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn reduce_ineq_coef(
    mut npp: *mut NPP,
    mut ptr: *mut elem,
    mut _b: *mut libc::c_double,
) -> libc::c_int {
    let mut current_block: u64;
    let mut e: *mut elem = 0 as *mut elem;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut h: libc::c_double = 0.;
    let mut inf_t: libc::c_double = 0.;
    let mut new_a: libc::c_double = 0.;
    let mut b: libc::c_double = *_b;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const libc::c_char,
                b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                1279 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    h = 0.0f64;
    e = ptr;
    loop {
        if e.is_null() {
            current_block = 1917311967535052937;
            break;
        }
        if (*e).aj > 0.0f64 {
            if (*(*e).xj).lb == -1.7976931348623157e+308f64 {
                current_block = 13121666071361358902;
                break;
            }
            h += (*e).aj * (*(*e).xj).lb;
        } else {
            if (*(*e).xj).ub == 1.7976931348623157e+308f64 {
                current_block = 13121666071361358902;
                break;
            }
            h += (*e).aj * (*(*e).xj).ub;
        }
        e = (*e).next;
    }
    match current_block {
        1917311967535052937 => {
            e = ptr;
            while !e.is_null() {
                if (*(*e).xj).is_int as libc::c_int != 0 && (*(*e).xj).lb == 0.0f64
                    && (*(*e).xj).ub == 1.0f64
                {
                    if (*e).aj > 0.0f64 {
                        inf_t = h;
                        if b - (*e).aj < inf_t && inf_t < b {
                            new_a = b - inf_t;
                            if new_a >= 1e-3f64
                                && (*e).aj - new_a >= 0.01f64 * (1.0f64 + (*e).aj)
                            {
                                (*e).aj = new_a;
                                count += 1;
                                count;
                            }
                        }
                    } else {
                        inf_t = h - (*e).aj;
                        if b < inf_t && inf_t < b - (*e).aj {
                            new_a = (*e).aj + (inf_t - b);
                            if new_a <= -1e-3f64
                                && new_a - (*e).aj >= 0.01f64 * (1.0f64 - (*e).aj)
                            {
                                (*e).aj = new_a;
                                h += inf_t - b;
                                b = inf_t;
                                count += 1;
                                count;
                            }
                        }
                    }
                }
                e = (*e).next;
            }
            *_b = b;
        }
        _ => {}
    }
    return count;
}
pub unsafe extern "C" fn _glp_npp_reduce_ineq_coef(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> libc::c_int {
    let mut copy: *mut NPPROW = 0 as *mut NPPROW;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut ptr: *mut elem = 0 as *mut elem;
    let mut e: *mut elem = 0 as *mut elem;
    let mut kase: libc::c_int = 0;
    let mut count: [libc::c_int; 2] = [0; 2];
    let mut b: libc::c_double = 0.;
    ((*row).lb < (*row).ub
        || {
            glp_assert_(
                b"row->lb < row->ub\0" as *const u8 as *const libc::c_char,
                b"npp/npp4.c\0" as *const u8 as *const libc::c_char,
                1348 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    count[1 as libc::c_int as usize] = 0 as libc::c_int;
    count[0 as libc::c_int as usize] = count[1 as libc::c_int as usize];
    let mut current_block_30: u64;
    kase = 0 as libc::c_int;
    while kase <= 1 as libc::c_int {
        if kase == 0 as libc::c_int {
            if (*row).lb == -1.7976931348623157e+308f64 {
                current_block_30 = 4644295000439058019;
            } else {
                ptr = copy_form(npp, row, 1.0f64);
                b = (*row).lb;
                current_block_30 = 7746791466490516765;
            }
        } else if (*row).ub == 1.7976931348623157e+308f64 {
            current_block_30 = 4644295000439058019;
        } else {
            ptr = copy_form(npp, row, -1.0f64);
            b = -(*row).ub;
            current_block_30 = 7746791466490516765;
        }
        match current_block_30 {
            7746791466490516765 => {
                count[kase as usize] = reduce_ineq_coef(npp, ptr, &mut b);
                if count[kase as usize] > 0 as libc::c_int {
                    if (*row).lb == -1.7976931348623157e+308f64
                        || (*row).ub == 1.7976931348623157e+308f64
                    {
                        copy = 0 as *mut NPPROW;
                    } else {
                        copy = _glp_npp_add_row(npp);
                        if kase == 0 as libc::c_int {
                            (*copy).lb = -1.7976931348623157e+308f64;
                            (*copy).ub = (*row).ub;
                        } else {
                            (*copy).lb = (*row).lb;
                            (*copy).ub = 1.7976931348623157e+308f64;
                        }
                        aij = (*row).ptr;
                        while !aij.is_null() {
                            _glp_npp_add_aij(npp, copy, (*aij).col, (*aij).val);
                            aij = (*aij).r_next;
                        }
                    }
                    _glp_npp_erase_row(npp, row);
                    (*row).lb = b;
                    (*row).ub = 1.7976931348623157e+308f64;
                    e = ptr;
                    while !e.is_null() {
                        _glp_npp_add_aij(npp, row, (*e).xj, (*e).aj);
                        e = (*e).next;
                    }
                    if !copy.is_null() {
                        row = copy;
                    }
                }
                drop_form(npp, ptr);
            }
            _ => {}
        }
        kase += 1;
        kase;
    }
    return count[0 as libc::c_int as usize] + count[1 as libc::c_int as usize];
}
