use ::libc;
extern "C" {
    pub type BFD;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn _glp_bfd_btran(bfd: *mut BFD, x: *mut libc::c_double);
    fn _glp_bfd_factorize(
        bfd: *mut BFD,
        m: libc::c_int,
        col: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                libc::c_int,
                *mut libc::c_int,
                *mut libc::c_double,
            ) -> libc::c_int,
        >,
        info: *mut libc::c_void,
    ) -> libc::c_int;
    fn _glp_bfd_ftran(bfd: *mut BFD, x: *mut libc::c_double);
    fn _glp_fvs_clear_vec(x: *mut FVS);
    fn _glp_bfd_btran_s(bfd: *mut BFD, x: *mut FVS);
    fn _glp_bfd_update(
        bfd: *mut BFD,
        j: libc::c_int,
        len: libc::c_int,
        ind: *const libc::c_int,
        val: *const libc::c_double,
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FVS {
    pub n: libc::c_int,
    pub nnz: libc::c_int,
    pub ind: *mut libc::c_int,
    pub vec: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPXLP {
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub nnz: libc::c_int,
    pub A_ptr: *mut libc::c_int,
    pub A_ind: *mut libc::c_int,
    pub A_val: *mut libc::c_double,
    pub b: *mut libc::c_double,
    pub c: *mut libc::c_double,
    pub l: *mut libc::c_double,
    pub u: *mut libc::c_double,
    pub head: *mut libc::c_int,
    pub flag: *mut libc::c_char,
    pub valid: libc::c_int,
    pub bfd: *mut BFD,
}
unsafe extern "C" fn jth_col(
    mut info: *mut libc::c_void,
    mut j: libc::c_int,
    mut ind: *mut libc::c_int,
    mut val: *mut libc::c_double,
) -> libc::c_int {
    let mut lp: *mut SPXLP = info as *mut SPXLP;
    let mut m: libc::c_int = (*lp).m;
    let mut A_ptr: *mut libc::c_int = (*lp).A_ptr;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut k: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    (1 as libc::c_int <= j && j <= m
        || {
            glp_assert_(
                b"1 <= j && j <= m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                42 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = *head.offset(j as isize);
    ptr = *A_ptr.offset(k as isize);
    len = *A_ptr.offset((k + 1 as libc::c_int) as isize) - ptr;
    memcpy(
        &mut *ind.offset(1 as libc::c_int as isize) as *mut libc::c_int
            as *mut libc::c_void,
        &mut *((*lp).A_ind).offset(ptr as isize) as *mut libc::c_int
            as *const libc::c_void,
        (len as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    memcpy(
        &mut *val.offset(1 as libc::c_int as isize) as *mut libc::c_double
            as *mut libc::c_void,
        &mut *((*lp).A_val).offset(ptr as isize) as *mut libc::c_double
            as *const libc::c_void,
        (len as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    return len;
}
pub unsafe extern "C" fn _glp_spx_factorize(mut lp: *mut SPXLP) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = _glp_bfd_factorize(
        (*lp).bfd,
        (*lp).m,
        Some(
            jth_col
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_int,
                    *mut libc::c_int,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
        lp as *mut libc::c_void,
    );
    (*lp).valid = (ret == 0 as libc::c_int) as libc::c_int;
    return ret;
}
pub unsafe extern "C" fn _glp_spx_eval_beta(
    mut lp: *mut SPXLP,
    mut beta: *mut libc::c_double,
) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut A_ptr: *mut libc::c_int = (*lp).A_ptr;
    let mut A_ind: *mut libc::c_int = (*lp).A_ind;
    let mut A_val: *mut libc::c_double = (*lp).A_val;
    let mut b: *mut libc::c_double = (*lp).b;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut flag: *mut libc::c_char = (*lp).flag;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut fj: libc::c_double = 0.;
    let mut y: *mut libc::c_double = 0 as *mut libc::c_double;
    y = beta;
    memcpy(
        &mut *y.offset(1 as libc::c_int as isize) as *mut libc::c_double
            as *mut libc::c_void,
        &mut *b.offset(1 as libc::c_int as isize) as *mut libc::c_double
            as *const libc::c_void,
        (m as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    j = 1 as libc::c_int;
    while j <= n - m {
        k = *head.offset((m + j) as isize);
        fj = if *flag.offset(j as isize) as libc::c_int != 0 {
            *u.offset(k as isize)
        } else {
            *l.offset(k as isize)
        };
        if !(fj == 0.0f64 || fj == -1.7976931348623157e+308f64) {
            ptr = *A_ptr.offset(k as isize);
            end = *A_ptr.offset((k + 1 as libc::c_int) as isize);
            while ptr < end {
                *y.offset(*A_ind.offset(ptr as isize) as isize)
                    -= *A_val.offset(ptr as isize) * fj;
                ptr += 1;
                ptr;
            }
        }
        j += 1;
        j;
    }
    ((*lp).valid != 0
        || {
            glp_assert_(
                b"lp->valid\0" as *const u8 as *const libc::c_char,
                b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                122 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_bfd_ftran((*lp).bfd, beta);
}
pub unsafe extern "C" fn _glp_spx_eval_obj(
    mut lp: *mut SPXLP,
    mut beta: *const libc::c_double,
) -> libc::c_double {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut flag: *mut libc::c_char = (*lp).flag;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut fj: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    z = *c.offset(0 as libc::c_int as isize);
    i = 1 as libc::c_int;
    while i <= m {
        k = *head.offset(i as isize);
        z += *c.offset(k as isize) * *beta.offset(i as isize);
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= n - m {
        k = *head.offset((m + j) as isize);
        fj = if *flag.offset(j as isize) as libc::c_int != 0 {
            *u.offset(k as isize)
        } else {
            *l.offset(k as isize)
        };
        if !(fj == 0.0f64 || fj == -1.7976931348623157e+308f64) {
            z += *c.offset(k as isize) * fj;
        }
        j += 1;
        j;
    }
    return z;
}
pub unsafe extern "C" fn _glp_spx_eval_pi(
    mut lp: *mut SPXLP,
    mut pi: *mut libc::c_double,
) {
    let mut m: libc::c_int = (*lp).m;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut i: libc::c_int = 0;
    let mut cB: *mut libc::c_double = 0 as *mut libc::c_double;
    cB = pi;
    i = 1 as libc::c_int;
    while i <= m {
        *cB.offset(i as isize) = *c.offset(*head.offset(i as isize) as isize);
        i += 1;
        i;
    }
    _glp_bfd_btran((*lp).bfd, pi);
}
pub unsafe extern "C" fn _glp_spx_eval_dj(
    mut lp: *mut SPXLP,
    mut pi: *const libc::c_double,
    mut j: libc::c_int,
) -> libc::c_double {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut A_ptr: *mut libc::c_int = (*lp).A_ptr;
    let mut A_ind: *mut libc::c_int = (*lp).A_ind;
    let mut A_val: *mut libc::c_double = (*lp).A_val;
    let mut k: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut dj: libc::c_double = 0.;
    (1 as libc::c_int <= j && j <= n - m
        || {
            glp_assert_(
                b"1 <= j && j <= n-m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                237 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = *((*lp).head).offset((m + j) as isize);
    dj = *((*lp).c).offset(k as isize);
    ptr = *A_ptr.offset(k as isize);
    end = *A_ptr.offset((k + 1 as libc::c_int) as isize);
    while ptr < end {
        dj
            -= *A_val.offset(ptr as isize)
                * *pi.offset(*A_ind.offset(ptr as isize) as isize);
        ptr += 1;
        ptr;
    }
    return dj;
}
pub unsafe extern "C" fn _glp_spx_eval_tcol(
    mut lp: *mut SPXLP,
    mut j: libc::c_int,
    mut tcol: *mut libc::c_double,
) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut A_ptr: *mut libc::c_int = (*lp).A_ptr;
    let mut A_ind: *mut libc::c_int = (*lp).A_ind;
    let mut A_val: *mut libc::c_double = (*lp).A_val;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    (1 as libc::c_int <= j && j <= n - m
        || {
            glp_assert_(
                b"1 <= j && j <= n-m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                274 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = *head.offset((m + j) as isize);
    i = 1 as libc::c_int;
    while i <= m {
        *tcol.offset(i as isize) = 0.0f64;
        i += 1;
        i;
    }
    ptr = *A_ptr.offset(k as isize);
    end = *A_ptr.offset((k + 1 as libc::c_int) as isize);
    while ptr < end {
        *tcol
            .offset(*A_ind.offset(ptr as isize) as isize) = -*A_val.offset(ptr as isize);
        ptr += 1;
        ptr;
    }
    _glp_bfd_ftran((*lp).bfd, tcol);
}
pub unsafe extern "C" fn _glp_spx_eval_rho(
    mut lp: *mut SPXLP,
    mut i: libc::c_int,
    mut rho: *mut libc::c_double,
) {
    let mut m: libc::c_int = (*lp).m;
    let mut j: libc::c_int = 0;
    (1 as libc::c_int <= i && i <= m
        || {
            glp_assert_(
                b"1 <= i && i <= m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                306 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    j = 1 as libc::c_int;
    while j <= m {
        *rho.offset(j as isize) = 0.0f64;
        j += 1;
        j;
    }
    *rho.offset(i as isize) = 1.0f64;
    _glp_bfd_btran((*lp).bfd, rho);
}
pub unsafe extern "C" fn _glp_spx_eval_rho_s(
    mut lp: *mut SPXLP,
    mut i: libc::c_int,
    mut rho: *mut FVS,
) {
    let mut m: libc::c_int = (*lp).m;
    (1 as libc::c_int <= i && i <= m
        || {
            glp_assert_(
                b"1 <= i && i <= m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                319 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*rho).n == m
        || {
            glp_assert_(
                b"rho->n == m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                321 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_fvs_clear_vec(rho);
    (*rho).nnz = 1 as libc::c_int;
    *((*rho).ind).offset(1 as libc::c_int as isize) = i;
    *((*rho).vec).offset(i as isize) = 1.0f64;
    _glp_bfd_btran_s((*lp).bfd, rho);
}
pub unsafe extern "C" fn _glp_spx_eval_tij(
    mut lp: *mut SPXLP,
    mut rho: *const libc::c_double,
    mut j: libc::c_int,
) -> libc::c_double {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut A_ptr: *mut libc::c_int = (*lp).A_ptr;
    let mut A_ind: *mut libc::c_int = (*lp).A_ind;
    let mut A_val: *mut libc::c_double = (*lp).A_val;
    let mut k: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut tij: libc::c_double = 0.;
    (1 as libc::c_int <= j && j <= n - m
        || {
            glp_assert_(
                b"1 <= j && j <= n-m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                355 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = *((*lp).head).offset((m + j) as isize);
    tij = 0.0f64;
    ptr = *A_ptr.offset(k as isize);
    end = *A_ptr.offset((k + 1 as libc::c_int) as isize);
    while ptr < end {
        tij
            -= *A_val.offset(ptr as isize)
                * *rho.offset(*A_ind.offset(ptr as isize) as isize);
        ptr += 1;
        ptr;
    }
    return tij;
}
pub unsafe extern "C" fn _glp_spx_eval_trow(
    mut lp: *mut SPXLP,
    mut rho: *const libc::c_double,
    mut trow: *mut libc::c_double,
) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut j: libc::c_int = 0;
    j = 1 as libc::c_int;
    while j <= n - m {
        *trow.offset(j as isize) = _glp_spx_eval_tij(lp, rho, j);
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn _glp_spx_update_beta(
    mut lp: *mut SPXLP,
    mut beta: *mut libc::c_double,
    mut p: libc::c_int,
    mut p_flag: libc::c_int,
    mut q: libc::c_int,
    mut tcol: *const libc::c_double,
) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut flag: *mut libc::c_char = (*lp).flag;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut delta_p: libc::c_double = 0.;
    let mut delta_q: libc::c_double = 0.;
    if p < 0 as libc::c_int {
        (1 as libc::c_int <= q && q <= n - m
            || {
                glp_assert_(
                    b"1 <= q && q <= n-m\0" as *const u8 as *const libc::c_char,
                    b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                    468 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        k = *head.offset((m + q) as isize);
        (*l.offset(k as isize) != -1.7976931348623157e+308f64
            && *u.offset(k as isize) != 1.7976931348623157e+308f64
            && *l.offset(k as isize) != *u.offset(k as isize)
            || {
                glp_assert_(
                    b"l[k] != -DBL_MAX && u[k] != +DBL_MAX && l[k] != u[k]\0"
                        as *const u8 as *const libc::c_char,
                    b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                    471 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if *flag.offset(q as isize) != 0 {
            delta_q = *l.offset(k as isize) - *u.offset(k as isize);
        } else {
            delta_q = *u.offset(k as isize) - *l.offset(k as isize);
        }
    } else {
        (1 as libc::c_int <= p && p <= m
            || {
                glp_assert_(
                    b"1 <= p && p <= m\0" as *const u8 as *const libc::c_char,
                    b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                    484 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (1 as libc::c_int <= q && q <= n - m
            || {
                glp_assert_(
                    b"1 <= q && q <= n-m\0" as *const u8 as *const libc::c_char,
                    b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                    485 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        k = *head.offset(p as isize);
        if p_flag != 0 {
            (*l.offset(k as isize) != *u.offset(k as isize)
                && *u.offset(k as isize) != 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"l[k] != u[k] && u[k] != +DBL_MAX\0" as *const u8
                            as *const libc::c_char,
                        b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                        490 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            delta_p = *u.offset(k as isize) - *beta.offset(p as isize);
        } else if *l.offset(k as isize) == -1.7976931348623157e+308f64 {
            (*u.offset(k as isize) == 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"u[k] == +DBL_MAX\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                        495 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            delta_p = 0.0f64 - *beta.offset(p as isize);
        } else {
            delta_p = *l.offset(k as isize) - *beta.offset(p as isize);
        }
        delta_q = delta_p / *tcol.offset(p as isize);
        k = *head.offset((m + q) as isize);
        if *flag.offset(q as isize) != 0 {
            (*l.offset(k as isize) != *u.offset(k as isize)
                && *u.offset(k as isize) != 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"l[k] != u[k] && u[k] != +DBL_MAX\0" as *const u8
                            as *const libc::c_char,
                        b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                        509 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            *beta.offset(p as isize) = *u.offset(k as isize) + delta_q;
        } else if *l.offset(k as isize) == -1.7976931348623157e+308f64 {
            (*u.offset(k as isize) == 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"u[k] == +DBL_MAX\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                        514 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            *beta.offset(p as isize) = 0.0f64 + delta_q;
        } else {
            *beta.offset(p as isize) = *l.offset(k as isize) + delta_q;
        }
    }
    i = 1 as libc::c_int;
    while i <= m {
        if i != p {
            *beta.offset(i as isize) += *tcol.offset(i as isize) * delta_q;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn _glp_spx_update_beta_s(
    mut lp: *mut SPXLP,
    mut beta: *mut libc::c_double,
    mut p: libc::c_int,
    mut p_flag: libc::c_int,
    mut q: libc::c_int,
    mut tcol: *const FVS,
) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut flag: *mut libc::c_char = (*lp).flag;
    let mut nnz: libc::c_int = (*tcol).nnz;
    let mut ind: *mut libc::c_int = (*tcol).ind;
    let mut vec: *mut libc::c_double = (*tcol).vec;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut delta_p: libc::c_double = 0.;
    let mut delta_q: libc::c_double = 0.;
    ((*tcol).n == m
        || {
            glp_assert_(
                b"tcol->n == m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                546 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if p < 0 as libc::c_int {
        (1 as libc::c_int <= q && q <= n - m
            || {
                glp_assert_(
                    b"1 <= q && q <= n-m\0" as *const u8 as *const libc::c_char,
                    b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                    553 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        k = *head.offset((m + q) as isize);
        (*l.offset(k as isize) != -1.7976931348623157e+308f64
            && *u.offset(k as isize) != 1.7976931348623157e+308f64
            && *l.offset(k as isize) != *u.offset(k as isize)
            || {
                glp_assert_(
                    b"l[k] != -DBL_MAX && u[k] != +DBL_MAX && l[k] != u[k]\0"
                        as *const u8 as *const libc::c_char,
                    b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                    556 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if *flag.offset(q as isize) != 0 {
            delta_q = *l.offset(k as isize) - *u.offset(k as isize);
        } else {
            delta_q = *u.offset(k as isize) - *l.offset(k as isize);
        }
    } else {
        (1 as libc::c_int <= p && p <= m
            || {
                glp_assert_(
                    b"1 <= p && p <= m\0" as *const u8 as *const libc::c_char,
                    b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                    569 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (1 as libc::c_int <= q && q <= n - m
            || {
                glp_assert_(
                    b"1 <= q && q <= n-m\0" as *const u8 as *const libc::c_char,
                    b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                    570 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        k = *head.offset(p as isize);
        if p_flag != 0 {
            (*l.offset(k as isize) != *u.offset(k as isize)
                && *u.offset(k as isize) != 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"l[k] != u[k] && u[k] != +DBL_MAX\0" as *const u8
                            as *const libc::c_char,
                        b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                        575 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            delta_p = *u.offset(k as isize) - *beta.offset(p as isize);
        } else if *l.offset(k as isize) == -1.7976931348623157e+308f64 {
            (*u.offset(k as isize) == 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"u[k] == +DBL_MAX\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                        580 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            delta_p = 0.0f64 - *beta.offset(p as isize);
        } else {
            delta_p = *l.offset(k as isize) - *beta.offset(p as isize);
        }
        delta_q = delta_p / *vec.offset(p as isize);
        k = *head.offset((m + q) as isize);
        if *flag.offset(q as isize) != 0 {
            (*l.offset(k as isize) != *u.offset(k as isize)
                && *u.offset(k as isize) != 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"l[k] != u[k] && u[k] != +DBL_MAX\0" as *const u8
                            as *const libc::c_char,
                        b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                        594 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            *beta.offset(p as isize) = *u.offset(k as isize) + delta_q;
        } else if *l.offset(k as isize) == -1.7976931348623157e+308f64 {
            (*u.offset(k as isize) == 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"u[k] == +DBL_MAX\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                        599 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            *beta.offset(p as isize) = 0.0f64 + delta_q;
        } else {
            *beta.offset(p as isize) = *l.offset(k as isize) + delta_q;
        }
    }
    k = 1 as libc::c_int;
    while k <= nnz {
        i = *ind.offset(k as isize);
        if i != p {
            *beta.offset(i as isize) += *vec.offset(i as isize) * delta_q;
        }
        k += 1;
        k;
    }
}
pub unsafe extern "C" fn _glp_spx_update_d(
    mut lp: *mut SPXLP,
    mut d: *mut libc::c_double,
    mut p: libc::c_int,
    mut q: libc::c_int,
    mut trow: *const libc::c_double,
    mut tcol: *const libc::c_double,
) -> libc::c_double {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut dq: libc::c_double = 0.;
    let mut e: libc::c_double = 0.;
    (1 as libc::c_int <= p && p <= m
        || {
            glp_assert_(
                b"1 <= p && p <= m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                678 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= q && q <= n
        || {
            glp_assert_(
                b"1 <= q && q <= n\0" as *const u8 as *const libc::c_char,
                b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                679 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = *head.offset((m + q) as isize);
    dq = *c.offset(k as isize);
    i = 1 as libc::c_int;
    while i <= m {
        dq += *tcol.offset(i as isize) * *c.offset(*head.offset(i as isize) as isize);
        i += 1;
        i;
    }
    e = fabs(dq - *d.offset(q as isize)) / (1.0f64 + fabs(dq));
    dq /= *tcol.offset(p as isize);
    *d.offset(q as isize) = dq;
    j = 1 as libc::c_int;
    while j <= n - m {
        if j != q {
            *d.offset(j as isize) -= *trow.offset(j as isize) * dq;
        }
        j += 1;
        j;
    }
    return e;
}
pub unsafe extern "C" fn _glp_spx_update_d_s(
    mut lp: *mut SPXLP,
    mut d: *mut libc::c_double,
    mut p: libc::c_int,
    mut q: libc::c_int,
    mut trow: *const FVS,
    mut tcol: *const FVS,
) -> libc::c_double {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut trow_nnz: libc::c_int = (*trow).nnz;
    let mut trow_ind: *mut libc::c_int = (*trow).ind;
    let mut trow_vec: *mut libc::c_double = (*trow).vec;
    let mut tcol_nnz: libc::c_int = (*tcol).nnz;
    let mut tcol_ind: *mut libc::c_int = (*tcol).ind;
    let mut tcol_vec: *mut libc::c_double = (*tcol).vec;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut dq: libc::c_double = 0.;
    let mut e: libc::c_double = 0.;
    (1 as libc::c_int <= p && p <= m
        || {
            glp_assert_(
                b"1 <= p && p <= m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                714 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= q && q <= n
        || {
            glp_assert_(
                b"1 <= q && q <= n\0" as *const u8 as *const libc::c_char,
                b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                715 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*trow).n == n - m
        || {
            glp_assert_(
                b"trow->n == n-m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                716 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*tcol).n == m
        || {
            glp_assert_(
                b"tcol->n == m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                717 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = *head.offset((m + q) as isize);
    dq = *c.offset(k as isize);
    k = 1 as libc::c_int;
    while k <= tcol_nnz {
        i = *tcol_ind.offset(k as isize);
        dq
            += *tcol_vec.offset(i as isize)
                * *c.offset(*head.offset(i as isize) as isize);
        k += 1;
        k;
    }
    e = fabs(dq - *d.offset(q as isize)) / (1.0f64 + fabs(dq));
    dq /= *tcol_vec.offset(p as isize);
    *d.offset(q as isize) = dq;
    k = 1 as libc::c_int;
    while k <= trow_nnz {
        j = *trow_ind.offset(k as isize);
        if j != q {
            *d.offset(j as isize) -= *trow_vec.offset(j as isize) * dq;
        }
        k += 1;
        k;
    }
    return e;
}
pub unsafe extern "C" fn _glp_spx_change_basis(
    mut lp: *mut SPXLP,
    mut p: libc::c_int,
    mut p_flag: libc::c_int,
    mut q: libc::c_int,
) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut flag: *mut libc::c_char = (*lp).flag;
    let mut k: libc::c_int = 0;
    if p < 0 as libc::c_int {
        (1 as libc::c_int <= q && q <= n - m
            || {
                glp_assert_(
                    b"1 <= q && q <= n-m\0" as *const u8 as *const libc::c_char,
                    b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                    759 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        k = *head.offset((m + q) as isize);
        (*l.offset(k as isize) != -1.7976931348623157e+308f64
            && *u.offset(k as isize) != 1.7976931348623157e+308f64
            && *l.offset(k as isize) != *u.offset(k as isize)
            || {
                glp_assert_(
                    b"l[k] != -DBL_MAX && u[k] != +DBL_MAX && l[k] != u[k]\0"
                        as *const u8 as *const libc::c_char,
                    b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                    762 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        *flag
            .offset(
                q as isize,
            ) = (1 as libc::c_int - *flag.offset(q as isize) as libc::c_int)
            as libc::c_char;
    } else {
        (1 as libc::c_int <= p && p <= m
            || {
                glp_assert_(
                    b"1 <= p && p <= m\0" as *const u8 as *const libc::c_char,
                    b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                    768 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (p_flag == 0 as libc::c_int || p_flag == 1 as libc::c_int
            || {
                glp_assert_(
                    b"p_flag == 0 || p_flag == 1\0" as *const u8 as *const libc::c_char,
                    b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                    769 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (1 as libc::c_int <= q && q <= n - m
            || {
                glp_assert_(
                    b"1 <= q && q <= n-m\0" as *const u8 as *const libc::c_char,
                    b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                    770 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        k = *head.offset(p as isize);
        if p_flag != 0 {
            (*l.offset(k as isize) != *u.offset(k as isize)
                && *u.offset(k as isize) != 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"l[k] != u[k] && u[k] != +DBL_MAX\0" as *const u8
                            as *const libc::c_char,
                        b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                        774 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        *head.offset(p as isize) = *head.offset((m + q) as isize);
        *head.offset((m + q) as isize) = k;
        *((*lp).flag).offset(q as isize) = p_flag as libc::c_char;
    };
}
pub unsafe extern "C" fn _glp_spx_update_invb(
    mut lp: *mut SPXLP,
    mut i: libc::c_int,
    mut k: libc::c_int,
) -> libc::c_int {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut A_ptr: *mut libc::c_int = (*lp).A_ptr;
    let mut A_ind: *mut libc::c_int = (*lp).A_ind;
    let mut A_val: *mut libc::c_double = (*lp).A_val;
    let mut ptr: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    (1 as libc::c_int <= i && i <= m
        || {
            glp_assert_(
                b"1 <= i && i <= m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                808 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= k && k <= n
        || {
            glp_assert_(
                b"1 <= k && k <= n\0" as *const u8 as *const libc::c_char,
                b"simplex/spxlp.c\0" as *const u8 as *const libc::c_char,
                809 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ptr = *A_ptr.offset(k as isize);
    len = *A_ptr.offset((k + 1 as libc::c_int) as isize) - ptr;
    ret = _glp_bfd_update(
        (*lp).bfd,
        i,
        len,
        &mut *A_ind.offset((ptr - 1 as libc::c_int) as isize) as *mut libc::c_int
            as *const libc::c_int,
        &mut *A_val.offset((ptr - 1 as libc::c_int) as isize) as *mut libc::c_double
            as *const libc::c_double,
    );
    (*lp).valid = (ret == 0 as libc::c_int) as libc::c_int;
    return ret;
}
