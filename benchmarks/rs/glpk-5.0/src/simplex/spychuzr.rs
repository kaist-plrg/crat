use ::libc;
extern "C" {
    pub type BFD;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn _glp_spx_eval_rho(lp: *mut SPXLP, i: libc::c_int, rho: *mut libc::c_double);
    fn _glp_spx_eval_tij(
        lp: *mut SPXLP,
        rho: *const libc::c_double,
        j: libc::c_int,
    ) -> libc::c_double;
    fn _glp_bfd_ftran(bfd: *mut BFD, x: *mut libc::c_double);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPYSE {
    pub valid: libc::c_int,
    pub refsp: *mut libc::c_char,
    pub gamma: *mut libc::c_double,
    pub work: *mut libc::c_double,
    pub u: FVS,
}
pub unsafe extern "C" fn _glp_spy_chuzr_sel(
    mut lp: *mut SPXLP,
    mut beta: *const libc::c_double,
    mut tol: libc::c_double,
    mut tol1: libc::c_double,
    mut list: *mut libc::c_int,
) -> libc::c_int {
    let mut m: libc::c_int = (*lp).m;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut lk: libc::c_double = 0.;
    let mut uk: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    num = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= m {
        k = *head.offset(i as isize);
        lk = *l.offset(k as isize);
        uk = *u.offset(k as isize);
        if *beta.offset(i as isize) < lk {
            eps = tol + tol1 * (if lk >= 0.0f64 { lk } else { -lk });
            if *beta.offset(i as isize) < lk - eps {
                num += 1;
                num;
                if !list.is_null() {
                    *list.offset(num as isize) = i;
                }
            }
        } else if *beta.offset(i as isize) > uk {
            eps = tol + tol1 * (if uk >= 0.0f64 { uk } else { -uk });
            if *beta.offset(i as isize) > uk + eps {
                num += 1;
                num;
                if !list.is_null() {
                    *list.offset(num as isize) = i;
                }
            }
        }
        i += 1;
        i;
    }
    return num;
}
pub unsafe extern "C" fn _glp_spy_chuzr_std(
    mut lp: *mut SPXLP,
    mut beta: *const libc::c_double,
    mut num: libc::c_int,
    mut list: *const libc::c_int,
) -> libc::c_int {
    let mut m: libc::c_int = (*lp).m;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut abs_ri: libc::c_double = 0.;
    let mut abs_rp: libc::c_double = 0.;
    ((0 as libc::c_int) < num && num <= m
        || {
            glp_assert_(
                b"0 < num && num <= m\0" as *const u8 as *const libc::c_char,
                b"simplex/spychuzr.c\0" as *const u8 as *const libc::c_char,
                135 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    p = 0 as libc::c_int;
    abs_rp = -1.0f64;
    t = 1 as libc::c_int;
    while t <= num {
        i = *list.offset(t as isize);
        k = *head.offset(i as isize);
        if *beta.offset(i as isize) < *l.offset(k as isize) {
            abs_ri = *l.offset(k as isize) - *beta.offset(i as isize);
        } else if *beta.offset(i as isize) > *u.offset(k as isize) {
            abs_ri = *beta.offset(i as isize) - *u.offset(k as isize);
        } else {
            (t != t
                || {
                    glp_assert_(
                        b"t != t\0" as *const u8 as *const libc::c_char,
                        b"simplex/spychuzr.c\0" as *const u8 as *const libc::c_char,
                        145 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        if abs_rp < abs_ri {
            p = i;
            abs_rp = abs_ri;
        }
        t += 1;
        t;
    }
    (p != 0 as libc::c_int
        || {
            glp_assert_(
                b"p != 0\0" as *const u8 as *const libc::c_char,
                b"simplex/spychuzr.c\0" as *const u8 as *const libc::c_char,
                149 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return p;
}
pub unsafe extern "C" fn _glp_spy_alloc_se(mut lp: *mut SPXLP, mut se: *mut SPYSE) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut i: libc::c_int = 0;
    (*se).valid = 0 as libc::c_int;
    (*se)
        .refsp = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    (*se)
        .gamma = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*se)
        .work = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*se).u.n = m;
    (*se).u.nnz = 0 as libc::c_int;
    (*se)
        .u
        .ind = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*se)
        .u
        .vec = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    i = 1 as libc::c_int;
    while i <= m {
        *((*se).u.vec).offset(i as isize) = 0.0f64;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn _glp_spy_reset_refsp(mut lp: *mut SPXLP, mut se: *mut SPYSE) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut refsp: *mut libc::c_char = (*se).refsp;
    let mut gamma: *mut libc::c_double = (*se).gamma;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    (*se).valid = 1 as libc::c_int;
    memset(
        &mut *refsp.offset(1 as libc::c_int as isize) as *mut libc::c_char
            as *mut libc::c_void,
        0 as libc::c_int,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    i = 1 as libc::c_int;
    while i <= m {
        k = *head.offset(i as isize);
        *refsp.offset(k as isize) = 1 as libc::c_int as libc::c_char;
        *gamma.offset(i as isize) = 1.0f64;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn _glp_spy_eval_gamma_i(
    mut lp: *mut SPXLP,
    mut se: *mut SPYSE,
    mut i: libc::c_int,
) -> libc::c_double {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut refsp: *mut libc::c_char = (*se).refsp;
    let mut rho: *mut libc::c_double = (*se).work;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut gamma_i: libc::c_double = 0.;
    let mut t_ij: libc::c_double = 0.;
    ((*se).valid != 0
        || {
            glp_assert_(
                b"se->valid\0" as *const u8 as *const libc::c_char,
                b"simplex/spychuzr.c\0" as *const u8 as *const libc::c_char,
                239 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= i && i <= m
        || {
            glp_assert_(
                b"1 <= i && i <= m\0" as *const u8 as *const libc::c_char,
                b"simplex/spychuzr.c\0" as *const u8 as *const libc::c_char,
                240 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = *head.offset(i as isize);
    gamma_i = if *refsp.offset(k as isize) as libc::c_int != 0 {
        1.0f64
    } else {
        0.0f64
    };
    _glp_spx_eval_rho(lp, i, rho);
    j = 1 as libc::c_int;
    while j <= n - m {
        k = *head.offset((m + j) as isize);
        if *refsp.offset(k as isize) != 0 {
            t_ij = _glp_spx_eval_tij(lp, rho as *const libc::c_double, j);
            gamma_i += t_ij * t_ij;
        }
        j += 1;
        j;
    }
    return gamma_i;
}
pub unsafe extern "C" fn _glp_spy_chuzr_pse(
    mut lp: *mut SPXLP,
    mut se: *mut SPYSE,
    mut beta: *const libc::c_double,
    mut num: libc::c_int,
    mut list: *const libc::c_int,
) -> libc::c_int {
    let mut m: libc::c_int = (*lp).m;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut gamma: *mut libc::c_double = (*se).gamma;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut best: libc::c_double = 0.;
    let mut ri: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    ((0 as libc::c_int) < num && num <= m
        || {
            glp_assert_(
                b"0 < num && num <= m\0" as *const u8 as *const libc::c_char,
                b"simplex/spychuzr.c\0" as *const u8 as *const libc::c_char,
                293 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    p = 0 as libc::c_int;
    best = -1.0f64;
    t = 1 as libc::c_int;
    while t <= num {
        i = *list.offset(t as isize);
        k = *head.offset(i as isize);
        if *beta.offset(i as isize) < *l.offset(k as isize) {
            ri = *l.offset(k as isize) - *beta.offset(i as isize);
        } else if *beta.offset(i as isize) > *u.offset(k as isize) {
            ri = *u.offset(k as isize) - *beta.offset(i as isize);
        } else {
            (t != t
                || {
                    glp_assert_(
                        b"t != t\0" as *const u8 as *const libc::c_char,
                        b"simplex/spychuzr.c\0" as *const u8 as *const libc::c_char,
                        303 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        if *gamma.offset(i as isize) < 2.2204460492503131e-16f64 {
            temp = 0.0f64;
        } else {
            temp = ri * ri / *gamma.offset(i as isize);
        }
        if best < temp {
            p = i;
            best = temp;
        }
        t += 1;
        t;
    }
    (p != 0 as libc::c_int
        || {
            glp_assert_(
                b"p != 0\0" as *const u8 as *const libc::c_char,
                b"simplex/spychuzr.c\0" as *const u8 as *const libc::c_char,
                312 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return p;
}
pub unsafe extern "C" fn _glp_spy_update_gamma(
    mut lp: *mut SPXLP,
    mut se: *mut SPYSE,
    mut p: libc::c_int,
    mut q: libc::c_int,
    mut trow: *const libc::c_double,
    mut tcol: *const libc::c_double,
) -> libc::c_double {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut refsp: *mut libc::c_char = (*se).refsp;
    let mut gamma: *mut libc::c_double = (*se).gamma;
    let mut u: *mut libc::c_double = (*se).work;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut gamma_p: libc::c_double = 0.;
    let mut delta_p: libc::c_double = 0.;
    let mut e: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut t1: libc::c_double = 0.;
    let mut t2: libc::c_double = 0.;
    ((*se).valid != 0
        || {
            glp_assert_(
                b"se->valid\0" as *const u8 as *const libc::c_char,
                b"simplex/spychuzr.c\0" as *const u8 as *const libc::c_char,
                360 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= p && p <= m
        || {
            glp_assert_(
                b"1 <= p && p <= m\0" as *const u8 as *const libc::c_char,
                b"simplex/spychuzr.c\0" as *const u8 as *const libc::c_char,
                361 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= q && q <= n - m
        || {
            glp_assert_(
                b"1 <= q && q <= n-m\0" as *const u8 as *const libc::c_char,
                b"simplex/spychuzr.c\0" as *const u8 as *const libc::c_char,
                362 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = *head.offset(p as isize);
    delta_p = if *refsp.offset(k as isize) as libc::c_int != 0 {
        1.0f64
    } else {
        0.0f64
    };
    gamma_p = delta_p;
    i = 1 as libc::c_int;
    while i <= m {
        *u.offset(i as isize) = 0.0f64;
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= n - m {
        k = *head.offset((m + j) as isize);
        if *refsp.offset(k as isize) as libc::c_int != 0
            && *trow.offset(j as isize) != 0.0f64
        {
            gamma_p += *trow.offset(j as isize) * *trow.offset(j as isize);
            ptr = *((*lp).A_ptr).offset(k as isize);
            end = *((*lp).A_ptr).offset((k + 1 as libc::c_int) as isize);
            while ptr < end {
                *u.offset(*((*lp).A_ind).offset(ptr as isize) as isize)
                    += *trow.offset(j as isize) * *((*lp).A_val).offset(ptr as isize);
                ptr += 1;
                ptr;
            }
        }
        j += 1;
        j;
    }
    _glp_bfd_ftran((*lp).bfd, u);
    e = fabs(gamma_p - *gamma.offset(p as isize)) / (1.0f64 + gamma_p);
    *gamma
        .offset(
            p as isize,
        ) = gamma_p / (*tcol.offset(p as isize) * *tcol.offset(p as isize));
    i = 1 as libc::c_int;
    while i <= m {
        if !(i == p) {
            r = *tcol.offset(i as isize) / *tcol.offset(p as isize);
            t1 = *gamma.offset(i as isize)
                + r * (r * gamma_p + *u.offset(i as isize) + *u.offset(i as isize));
            k = *head.offset(i as isize);
            t2 = (if *refsp.offset(k as isize) as libc::c_int != 0 {
                1.0f64
            } else {
                0.0f64
            }) + delta_p * r * r;
            *gamma.offset(i as isize) = if t1 >= t2 { t1 } else { t2 };
        }
        i += 1;
        i;
    }
    return e;
}
pub unsafe extern "C" fn _glp_spy_update_gamma_s(
    mut lp: *mut SPXLP,
    mut se: *mut SPYSE,
    mut p: libc::c_int,
    mut q: libc::c_int,
    mut trow: *const FVS,
    mut tcol: *const FVS,
) -> libc::c_double {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut refsp: *mut libc::c_char = (*se).refsp;
    let mut gamma: *mut libc::c_double = (*se).gamma;
    let mut u: *mut libc::c_double = (*se).work;
    let mut trow_nnz: libc::c_int = (*trow).nnz;
    let mut trow_ind: *mut libc::c_int = (*trow).ind;
    let mut trow_vec: *mut libc::c_double = (*trow).vec;
    let mut tcol_nnz: libc::c_int = (*tcol).nnz;
    let mut tcol_ind: *mut libc::c_int = (*tcol).ind;
    let mut tcol_vec: *mut libc::c_double = (*tcol).vec;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut gamma_p: libc::c_double = 0.;
    let mut delta_p: libc::c_double = 0.;
    let mut e: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut t1: libc::c_double = 0.;
    let mut t2: libc::c_double = 0.;
    ((*se).valid != 0
        || {
            glp_assert_(
                b"se->valid\0" as *const u8 as *const libc::c_char,
                b"simplex/spychuzr.c\0" as *const u8 as *const libc::c_char,
                419 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= p && p <= m
        || {
            glp_assert_(
                b"1 <= p && p <= m\0" as *const u8 as *const libc::c_char,
                b"simplex/spychuzr.c\0" as *const u8 as *const libc::c_char,
                420 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= q && q <= n - m
        || {
            glp_assert_(
                b"1 <= q && q <= n-m\0" as *const u8 as *const libc::c_char,
                b"simplex/spychuzr.c\0" as *const u8 as *const libc::c_char,
                421 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = *head.offset(p as isize);
    delta_p = if *refsp.offset(k as isize) as libc::c_int != 0 {
        1.0f64
    } else {
        0.0f64
    };
    gamma_p = delta_p;
    i = 1 as libc::c_int;
    while i <= m {
        *u.offset(i as isize) = 0.0f64;
        i += 1;
        i;
    }
    t = 1 as libc::c_int;
    while t <= trow_nnz {
        j = *trow_ind.offset(t as isize);
        k = *head.offset((m + j) as isize);
        if *refsp.offset(k as isize) != 0 {
            gamma_p += *trow_vec.offset(j as isize) * *trow_vec.offset(j as isize);
            ptr = *((*lp).A_ptr).offset(k as isize);
            end = *((*lp).A_ptr).offset((k + 1 as libc::c_int) as isize);
            while ptr < end {
                *u.offset(*((*lp).A_ind).offset(ptr as isize) as isize)
                    += *trow_vec.offset(j as isize)
                        * *((*lp).A_val).offset(ptr as isize);
                ptr += 1;
                ptr;
            }
        }
        t += 1;
        t;
    }
    _glp_bfd_ftran((*lp).bfd, u);
    e = fabs(gamma_p - *gamma.offset(p as isize)) / (1.0f64 + gamma_p);
    *gamma
        .offset(
            p as isize,
        ) = gamma_p / (*tcol_vec.offset(p as isize) * *tcol_vec.offset(p as isize));
    t = 1 as libc::c_int;
    while t <= tcol_nnz {
        i = *tcol_ind.offset(t as isize);
        if !(i == p) {
            r = *tcol_vec.offset(i as isize) / *tcol_vec.offset(p as isize);
            t1 = *gamma.offset(i as isize)
                + r * (r * gamma_p + *u.offset(i as isize) + *u.offset(i as isize));
            k = *head.offset(i as isize);
            t2 = (if *refsp.offset(k as isize) as libc::c_int != 0 {
                1.0f64
            } else {
                0.0f64
            }) + delta_p * r * r;
            *gamma.offset(i as isize) = if t1 >= t2 { t1 } else { t2 };
        }
        t += 1;
        t;
    }
    return e;
}
pub unsafe extern "C" fn _glp_spy_free_se(mut lp: *mut SPXLP, mut se: *mut SPYSE) {
    (lp == lp
        || {
            glp_assert_(
                b"lp == lp\0" as *const u8 as *const libc::c_char,
                b"simplex/spychuzr.c\0" as *const u8 as *const libc::c_char,
                470 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    glp_free((*se).refsp as *mut libc::c_void);
    glp_free((*se).gamma as *mut libc::c_void);
    glp_free((*se).work as *mut libc::c_void);
    glp_free((*se).u.ind as *mut libc::c_void);
    glp_free((*se).u.vec as *mut libc::c_void);
}
