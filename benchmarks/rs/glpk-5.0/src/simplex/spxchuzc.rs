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
    fn _glp_spx_eval_tcol(lp: *mut SPXLP, j: libc::c_int, tcol: *mut libc::c_double);
    fn _glp_bfd_btran(bfd: *mut BFD, x: *mut libc::c_double);
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
pub struct SPXSE {
    pub valid: libc::c_int,
    pub refsp: *mut libc::c_char,
    pub gamma: *mut libc::c_double,
    pub work: *mut libc::c_double,
}
pub unsafe extern "C" fn _glp_spx_chuzc_sel(
    mut lp: *mut SPXLP,
    mut d: *const libc::c_double,
    mut tol: libc::c_double,
    mut tol1: libc::c_double,
    mut list: *mut libc::c_int,
) -> libc::c_int {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut flag: *mut libc::c_char = (*lp).flag;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut ck: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    num = 0 as libc::c_int;
    let mut current_block_7: u64;
    j = 1 as libc::c_int;
    while j <= n - m {
        k = *head.offset((m + j) as isize);
        if !(*l.offset(k as isize) == *u.offset(k as isize)) {
            ck = *c.offset(k as isize);
            eps = tol + tol1 * (if ck >= 0.0f64 { ck } else { -ck });
            if *d.offset(j as isize) <= -eps {
                if *flag.offset(j as isize) != 0 {
                    current_block_7 = 7502529970979898288;
                } else {
                    current_block_7 = 17407779659766490442;
                }
            } else if *d.offset(j as isize) >= eps {
                if *flag.offset(j as isize) == 0
                    && *l.offset(k as isize) != -1.7976931348623157e+308f64
                {
                    current_block_7 = 7502529970979898288;
                } else {
                    current_block_7 = 17407779659766490442;
                }
            } else {
                current_block_7 = 7502529970979898288;
            }
            match current_block_7 {
                7502529970979898288 => {}
                _ => {
                    num += 1;
                    num;
                    if !list.is_null() {
                        *list.offset(num as isize) = j;
                    }
                }
            }
        }
        j += 1;
        j;
    }
    return num;
}
pub unsafe extern "C" fn _glp_spx_chuzc_std(
    mut lp: *mut SPXLP,
    mut d: *const libc::c_double,
    mut num: libc::c_int,
    mut list: *const libc::c_int,
) -> libc::c_int {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut j: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut abs_dj: libc::c_double = 0.;
    let mut abs_dq: libc::c_double = 0.;
    ((0 as libc::c_int) < num && num <= n - m
        || {
            glp_assert_(
                b"0 < num && num <= n-m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxchuzc.c\0" as *const u8 as *const libc::c_char,
                132 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    q = 0 as libc::c_int;
    abs_dq = -1.0f64;
    t = 1 as libc::c_int;
    while t <= num {
        j = *list.offset(t as isize);
        abs_dj = if *d.offset(j as isize) >= 0.0f64 {
            *d.offset(j as isize)
        } else {
            -*d.offset(j as isize)
        };
        if abs_dq < abs_dj {
            q = j;
            abs_dq = abs_dj;
        }
        t += 1;
        t;
    }
    (q != 0 as libc::c_int
        || {
            glp_assert_(
                b"q != 0\0" as *const u8 as *const libc::c_char,
                b"simplex/spxchuzc.c\0" as *const u8 as *const libc::c_char,
                140 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return q;
}
pub unsafe extern "C" fn _glp_spx_alloc_se(mut lp: *mut SPXLP, mut se: *mut SPXSE) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    (*se).valid = 0 as libc::c_int;
    (*se)
        .refsp = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    (*se)
        .gamma = glp_alloc(
        1 as libc::c_int + n - m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*se)
        .work = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
}
pub unsafe extern "C" fn _glp_spx_reset_refsp(mut lp: *mut SPXLP, mut se: *mut SPXSE) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut refsp: *mut libc::c_char = (*se).refsp;
    let mut gamma: *mut libc::c_double = (*se).gamma;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    (*se).valid = 1 as libc::c_int;
    memset(
        &mut *refsp.offset(1 as libc::c_int as isize) as *mut libc::c_char
            as *mut libc::c_void,
        0 as libc::c_int,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    j = 1 as libc::c_int;
    while j <= n - m {
        k = *head.offset((m + j) as isize);
        *refsp.offset(k as isize) = 1 as libc::c_int as libc::c_char;
        *gamma.offset(j as isize) = 1.0f64;
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn _glp_spx_eval_gamma_j(
    mut lp: *mut SPXLP,
    mut se: *mut SPXSE,
    mut j: libc::c_int,
) -> libc::c_double {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut refsp: *mut libc::c_char = (*se).refsp;
    let mut tcol: *mut libc::c_double = (*se).work;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut gamma_j: libc::c_double = 0.;
    ((*se).valid != 0
        || {
            glp_assert_(
                b"se->valid\0" as *const u8 as *const libc::c_char,
                b"simplex/spxchuzc.c\0" as *const u8 as *const libc::c_char,
                214 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= j && j <= n - m
        || {
            glp_assert_(
                b"1 <= j && j <= n-m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxchuzc.c\0" as *const u8 as *const libc::c_char,
                215 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = *head.offset((m + j) as isize);
    gamma_j = if *refsp.offset(k as isize) as libc::c_int != 0 {
        1.0f64
    } else {
        0.0f64
    };
    _glp_spx_eval_tcol(lp, j, tcol);
    i = 1 as libc::c_int;
    while i <= m {
        k = *head.offset(i as isize);
        if *refsp.offset(k as isize) != 0 {
            gamma_j += *tcol.offset(i as isize) * *tcol.offset(i as isize);
        }
        i += 1;
        i;
    }
    return gamma_j;
}
pub unsafe extern "C" fn _glp_spx_chuzc_pse(
    mut lp: *mut SPXLP,
    mut se: *mut SPXSE,
    mut d: *const libc::c_double,
    mut num: libc::c_int,
    mut list: *const libc::c_int,
) -> libc::c_int {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut gamma: *mut libc::c_double = (*se).gamma;
    let mut j: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut best: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    ((*se).valid != 0
        || {
            glp_assert_(
                b"se->valid\0" as *const u8 as *const libc::c_char,
                b"simplex/spxchuzc.c\0" as *const u8 as *const libc::c_char,
                258 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((0 as libc::c_int) < num && num <= n - m
        || {
            glp_assert_(
                b"0 < num && num <= n-m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxchuzc.c\0" as *const u8 as *const libc::c_char,
                259 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    q = 0 as libc::c_int;
    best = -1.0f64;
    t = 1 as libc::c_int;
    while t <= num {
        j = *list.offset(t as isize);
        if *gamma.offset(j as isize) < 2.2204460492503131e-16f64 {
            temp = 0.0f64;
        } else {
            temp = *d.offset(j as isize) * *d.offset(j as isize)
                / *gamma.offset(j as isize);
        }
        if best < temp {
            q = j;
            best = temp;
        }
        t += 1;
        t;
    }
    (q != 0 as libc::c_int
        || {
            glp_assert_(
                b"q != 0\0" as *const u8 as *const libc::c_char,
                b"simplex/spxchuzc.c\0" as *const u8 as *const libc::c_char,
                271 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return q;
}
pub unsafe extern "C" fn _glp_spx_update_gamma(
    mut lp: *mut SPXLP,
    mut se: *mut SPXSE,
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
    let mut gamma_q: libc::c_double = 0.;
    let mut delta_q: libc::c_double = 0.;
    let mut e: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut t1: libc::c_double = 0.;
    let mut t2: libc::c_double = 0.;
    ((*se).valid != 0
        || {
            glp_assert_(
                b"se->valid\0" as *const u8 as *const libc::c_char,
                b"simplex/spxchuzc.c\0" as *const u8 as *const libc::c_char,
                318 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= p && p <= m
        || {
            glp_assert_(
                b"1 <= p && p <= m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxchuzc.c\0" as *const u8 as *const libc::c_char,
                319 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= q && q <= n - m
        || {
            glp_assert_(
                b"1 <= q && q <= n-m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxchuzc.c\0" as *const u8 as *const libc::c_char,
                320 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = *head.offset((m + q) as isize);
    delta_q = if *refsp.offset(k as isize) as libc::c_int != 0 {
        1.0f64
    } else {
        0.0f64
    };
    gamma_q = delta_q;
    i = 1 as libc::c_int;
    while i <= m {
        k = *head.offset(i as isize);
        if *refsp.offset(k as isize) != 0 {
            gamma_q += *tcol.offset(i as isize) * *tcol.offset(i as isize);
            *u.offset(i as isize) = *tcol.offset(i as isize);
        } else {
            *u.offset(i as isize) = 0.0f64;
        }
        i += 1;
        i;
    }
    _glp_bfd_btran((*lp).bfd, u);
    e = fabs(gamma_q - *gamma.offset(q as isize)) / (1.0f64 + gamma_q);
    *gamma
        .offset(
            q as isize,
        ) = gamma_q / (*tcol.offset(p as isize) * *tcol.offset(p as isize));
    j = 1 as libc::c_int;
    while j <= n - m {
        if !(j == q) {
            if !(-1e-9f64 < *trow.offset(j as isize)
                && *trow.offset(j as isize) < 1e-9f64)
            {
                r = *trow.offset(j as isize) / *tcol.offset(p as isize);
                s = 0.0f64;
                k = *head.offset((m + j) as isize);
                ptr = *((*lp).A_ptr).offset(k as isize);
                end = *((*lp).A_ptr).offset((k + 1 as libc::c_int) as isize);
                while ptr < end {
                    s
                        += *((*lp).A_val).offset(ptr as isize)
                            * *u.offset(*((*lp).A_ind).offset(ptr as isize) as isize);
                    ptr += 1;
                    ptr;
                }
                t1 = *gamma.offset(j as isize) + r * (r * gamma_q + s + s);
                t2 = (if *refsp.offset(k as isize) as libc::c_int != 0 {
                    1.0f64
                } else {
                    0.0f64
                }) + delta_q * r * r;
                *gamma.offset(j as isize) = if t1 >= t2 { t1 } else { t2 };
            }
        }
        j += 1;
        j;
    }
    return e;
}
pub unsafe extern "C" fn _glp_spx_free_se(mut lp: *mut SPXLP, mut se: *mut SPXSE) {
    (lp == lp
        || {
            glp_assert_(
                b"lp == lp\0" as *const u8 as *const libc::c_char,
                b"simplex/spxchuzc.c\0" as *const u8 as *const libc::c_char,
                372 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    glp_free((*se).refsp as *mut libc::c_void);
    glp_free((*se).gamma as *mut libc::c_void);
    glp_free((*se).work as *mut libc::c_void);
}
