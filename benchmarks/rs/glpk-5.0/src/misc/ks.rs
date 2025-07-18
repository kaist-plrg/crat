use ::libc;
extern "C" {
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn _glp_mt1(
        n: libc::c_int,
        p: *mut libc::c_int,
        w: *mut libc::c_int,
        c: libc::c_int,
        x: *mut libc::c_int,
        jck: libc::c_int,
        xx: *mut libc::c_int,
        min: *mut libc::c_int,
        psign: *mut libc::c_int,
        wsign: *mut libc::c_int,
        zsign: *mut libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ks {
    pub orig_n: libc::c_int,
    pub n: libc::c_int,
    pub a: *mut libc::c_int,
    pub b: libc::c_int,
    pub c: *mut libc::c_int,
    pub c0: libc::c_int,
    pub x: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mt {
    pub j: libc::c_int,
    pub r: libc::c_float,
}
pub unsafe extern "C" fn _glp_ks_enum(
    mut n: libc::c_int,
    mut a: *const libc::c_int,
    mut b: libc::c_int,
    mut c: *const libc::c_int,
    mut x: *mut libc::c_char,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut z_best: libc::c_int = 0;
    let mut x_best: [libc::c_char; 41] = [0; 41];
    (0 as libc::c_int <= n && n <= 40 as libc::c_int
        || {
            glp_assert_(
                b"0 <= n && n <= N_MAX\0" as *const u8 as *const libc::c_char,
                b"misc/ks.c\0" as *const u8 as *const libc::c_char,
                62 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    memset(
        &mut *x.offset(1 as libc::c_int as isize) as *mut libc::c_char
            as *mut libc::c_void,
        0 as libc::c_int,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    z_best = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    '_loop: loop {
        z = 0 as libc::c_int;
        s = z;
        j = 1 as libc::c_int;
        while j <= n {
            if *x.offset(j as isize) != 0 {
                s += *a.offset(j as isize);
                z += *c.offset(j as isize);
            }
            j += 1;
            j;
        }
        if !(s > b) {
            if z_best < z {
                memcpy(
                    &mut *x_best.as_mut_ptr().offset(1 as libc::c_int as isize)
                        as *mut libc::c_char as *mut libc::c_void,
                    &mut *x.offset(1 as libc::c_int as isize) as *mut libc::c_char
                        as *const libc::c_void,
                    (n as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                );
                z_best = z;
            }
        }
        j = 1 as libc::c_int;
        loop {
            if !(j <= n) {
                break '_loop;
            }
            if *x.offset(j as isize) == 0 {
                *x.offset(j as isize) = 1 as libc::c_int as libc::c_char;
                break;
            } else {
                *x.offset(j as isize) = 0 as libc::c_int as libc::c_char;
                j += 1;
                j;
            }
        }
    }
    memcpy(
        &mut *x.offset(1 as libc::c_int as isize) as *mut libc::c_char
            as *mut libc::c_void,
        &mut *x_best.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut libc::c_char
            as *const libc::c_void,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    return z_best;
}
unsafe extern "C" fn reduce(
    n: libc::c_int,
    mut a: *const libc::c_int,
    mut b: libc::c_int,
    mut c: *const libc::c_int,
) -> *mut ks {
    let mut ks: *mut ks = 0 as *mut ks;
    let mut j: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    (n >= 0 as libc::c_int
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const libc::c_char,
                b"misc/ks.c\0" as *const u8 as *const libc::c_char,
                146 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ks = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<ks>() as libc::c_ulong as libc::c_int,
    ) as *mut ks;
    (*ks).orig_n = n;
    (*ks).n = 0 as libc::c_int;
    (*ks)
        .a = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    memcpy(
        &mut *((*ks).a).offset(1 as libc::c_int as isize) as *mut libc::c_int
            as *mut libc::c_void,
        &*a.offset(1 as libc::c_int as isize) as *const libc::c_int
            as *const libc::c_void,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    (*ks).b = b;
    (*ks)
        .c = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    memcpy(
        &mut *((*ks).c).offset(1 as libc::c_int as isize) as *mut libc::c_int
            as *mut libc::c_void,
        &*c.offset(1 as libc::c_int as isize) as *const libc::c_int
            as *const libc::c_void,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    (*ks).c0 = 0 as libc::c_int;
    (*ks)
        .x = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    j = 1 as libc::c_int;
    while j <= n {
        if *a.offset(j as isize) >= 0 as libc::c_int {
            *((*ks).x).offset(j as isize) = 0x10 as libc::c_int as libc::c_char;
        } else {
            *((*ks).x).offset(j as isize) = 0x11 as libc::c_int as libc::c_char;
            *((*ks).a).offset(j as isize) = -*((*ks).a).offset(j as isize);
            (*ks).b += *((*ks).a).offset(j as isize);
            (*ks).c0 += *((*ks).c).offset(j as isize);
            *((*ks).c).offset(j as isize) = -*((*ks).c).offset(j as isize);
        }
        j += 1;
        j;
    }
    if (*ks).b < 0 as libc::c_int {
        free_ks(ks);
        return 0 as *mut ks;
    }
    j = 1 as libc::c_int;
    while j <= n {
        if *((*ks).a).offset(j as isize) == 0 as libc::c_int {
            if *((*ks).c).offset(j as isize) <= 0 as libc::c_int {
                let ref mut fresh0 = *((*ks).x).offset(j as isize);
                *fresh0 = (*fresh0 as libc::c_int ^ 0x10 as libc::c_int) as libc::c_char;
            } else {
                let ref mut fresh1 = *((*ks).x).offset(j as isize);
                *fresh1 = (*fresh1 as libc::c_int ^ 0x11 as libc::c_int) as libc::c_char;
                (*ks).c0 += *((*ks).c).offset(j as isize);
            }
        } else if *((*ks).a).offset(j as isize) > (*ks).b
            || *((*ks).c).offset(j as isize) <= 0 as libc::c_int
        {
            let ref mut fresh2 = *((*ks).x).offset(j as isize);
            *fresh2 = (*fresh2 as libc::c_int ^ 0x10 as libc::c_int) as libc::c_char;
        } else {
            (*ks).n += 1;
            (*ks).n;
            *((*ks).a).offset((*ks).n as isize) = *((*ks).a).offset(j as isize);
            *((*ks).c).offset((*ks).n as isize) = *((*ks).c).offset(j as isize);
        }
        j += 1;
        j;
    }
    s = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while j <= (*ks).n {
        (1 as libc::c_int <= *((*ks).a).offset(j as isize)
            && *((*ks).a).offset(j as isize) <= (*ks).b
            || {
                glp_assert_(
                    b"1 <= ks->a[j] && ks->a[j] <= ks->b\0" as *const u8
                        as *const libc::c_char,
                    b"misc/ks.c\0" as *const u8 as *const libc::c_char,
                    213 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*((*ks).c).offset(j as isize) >= 1 as libc::c_int
            || {
                glp_assert_(
                    b"ks->c[j] >= 1\0" as *const u8 as *const libc::c_char,
                    b"misc/ks.c\0" as *const u8 as *const libc::c_char,
                    214 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        s += *((*ks).a).offset(j as isize);
        j += 1;
        j;
    }
    if s <= (*ks).b {
        j = 1 as libc::c_int;
        while j <= n {
            if *((*ks).x).offset(j as isize) as libc::c_int & 0x10 as libc::c_int != 0 {
                let ref mut fresh3 = *((*ks).x).offset(j as isize);
                *fresh3 = (*fresh3 as libc::c_int ^ 0x11 as libc::c_int) as libc::c_char;
            }
            j += 1;
            j;
        }
        j = 1 as libc::c_int;
        while j <= (*ks).n {
            (*ks).c0 += *((*ks).c).offset(j as isize);
            j += 1;
            j;
        }
        (*ks).n = 0 as libc::c_int;
    }
    ((*ks).n == 0 as libc::c_int || (*ks).n >= 2 as libc::c_int
        || {
            glp_assert_(
                b"ks->n == 0 || ks->n >= 2\0" as *const u8 as *const libc::c_char,
                b"misc/ks.c\0" as *const u8 as *const libc::c_char,
                230 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return ks;
}
unsafe extern "C" fn restore(mut ks: *mut ks, mut x: *mut libc::c_char) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    z = (*ks).c0;
    j = 1 as libc::c_int;
    k = 0 as libc::c_int;
    while j <= (*ks).orig_n {
        if *((*ks).x).offset(j as isize) as libc::c_int & 0x10 as libc::c_int != 0 {
            k += 1;
            k;
            (k <= (*ks).n
                || {
                    glp_assert_(
                        b"k <= ks->n\0" as *const u8 as *const libc::c_char,
                        b"misc/ks.c\0" as *const u8 as *const libc::c_char,
                        253 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*x.offset(k as isize) as libc::c_int == 0 as libc::c_int
                || *x.offset(k as isize) as libc::c_int == 1 as libc::c_int
                || {
                    glp_assert_(
                        b"x[k] == 0 || x[k] == 1\0" as *const u8 as *const libc::c_char,
                        b"misc/ks.c\0" as *const u8 as *const libc::c_char,
                        254 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if *((*ks).x).offset(j as isize) as libc::c_int & 1 as libc::c_int != 0 {
                *((*ks).x)
                    .offset(
                        j as isize,
                    ) = (1 as libc::c_int - *x.offset(k as isize) as libc::c_int)
                    as libc::c_char;
            } else {
                *((*ks).x).offset(j as isize) = *x.offset(k as isize);
            }
            if *x.offset(k as isize) != 0 {
                z += *((*ks).c).offset(k as isize);
            }
        }
        j += 1;
        j;
    }
    (k == (*ks).n
        || {
            glp_assert_(
                b"k == ks->n\0" as *const u8 as *const libc::c_char,
                b"misc/ks.c\0" as *const u8 as *const libc::c_char,
                263 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return z;
}
unsafe extern "C" fn free_ks(mut ks: *mut ks) {
    (!ks.is_null()
        || {
            glp_assert_(
                b"ks != NULL\0" as *const u8 as *const libc::c_char,
                b"misc/ks.c\0" as *const u8 as *const libc::c_char,
                274 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    glp_free((*ks).a as *mut libc::c_void);
    glp_free((*ks).c as *mut libc::c_void);
    glp_free((*ks).x as *mut libc::c_void);
    glp_free(ks as *mut libc::c_void);
}
unsafe extern "C" fn fcmp(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    if (*(p1 as *mut mt)).r > (*(p2 as *mut mt)).r {
        return -(1 as libc::c_int)
    } else if (*(p1 as *mut mt)).r < (*(p2 as *mut mt)).r {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn mt1a(
    mut n: libc::c_int,
    mut a: *const libc::c_int,
    mut b: libc::c_int,
    mut c: *const libc::c_int,
    mut x: *mut libc::c_char,
) -> libc::c_int {
    let mut mt: *mut mt = 0 as *mut mt;
    let mut j: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut x1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut xx: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut min: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut psign: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut wsign: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut zsign: *mut libc::c_int = 0 as *mut libc::c_int;
    (n >= 2 as libc::c_int
        || {
            glp_assert_(
                b"n >= 2\0" as *const u8 as *const libc::c_char,
                b"misc/ks.c\0" as *const u8 as *const libc::c_char,
                318 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    mt = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<mt>() as libc::c_ulong as libc::c_int,
    ) as *mut mt;
    p = glp_alloc(
        1 as libc::c_int + n + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    w = glp_alloc(
        1 as libc::c_int + n + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    x1 = glp_alloc(
        1 as libc::c_int + n + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    xx = glp_alloc(
        1 as libc::c_int + n + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    min = glp_alloc(
        1 as libc::c_int + n + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    psign = glp_alloc(
        1 as libc::c_int + n + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    wsign = glp_alloc(
        1 as libc::c_int + n + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    zsign = glp_alloc(
        1 as libc::c_int + n + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    j = 1 as libc::c_int;
    while j <= n {
        (*mt.offset(j as isize)).j = j;
        (*mt.offset(j as isize))
            .r = *c.offset(j as isize) as libc::c_float
            / *a.offset(j as isize) as libc::c_float;
        j += 1;
        j;
    }
    qsort(
        &mut *mt.offset(1 as libc::c_int as isize) as *mut mt as *mut libc::c_void,
        n as size_t,
        ::std::mem::size_of::<mt>() as libc::c_ulong,
        Some(
            fcmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    j = 1 as libc::c_int;
    while j <= n {
        *p.offset(j as isize) = *c.offset((*mt.offset(j as isize)).j as isize);
        *w.offset(j as isize) = *a.offset((*mt.offset(j as isize)).j as isize);
        j += 1;
        j;
    }
    z = _glp_mt1(n, p, w, b, x1, 1 as libc::c_int, xx, min, psign, wsign, zsign);
    (z >= 0 as libc::c_int
        || {
            glp_assert_(
                b"z >= 0\0" as *const u8 as *const libc::c_char,
                b"misc/ks.c\0" as *const u8 as *const libc::c_char,
                342 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    j = 1 as libc::c_int;
    while j <= n {
        (*x1.offset(j as isize) == 0 as libc::c_int
            || *x1.offset(j as isize) == 1 as libc::c_int
            || {
                glp_assert_(
                    b"x1[j] == 0 || x1[j] == 1\0" as *const u8 as *const libc::c_char,
                    b"misc/ks.c\0" as *const u8 as *const libc::c_char,
                    345 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        *x
            .offset(
                (*mt.offset(j as isize)).j as isize,
            ) = *x1.offset(j as isize) as libc::c_char;
        j += 1;
        j;
    }
    glp_free(mt as *mut libc::c_void);
    glp_free(p as *mut libc::c_void);
    glp_free(w as *mut libc::c_void);
    glp_free(x1 as *mut libc::c_void);
    glp_free(xx as *mut libc::c_void);
    glp_free(min as *mut libc::c_void);
    glp_free(psign as *mut libc::c_void);
    glp_free(wsign as *mut libc::c_void);
    glp_free(zsign as *mut libc::c_void);
    return z;
}
pub unsafe extern "C" fn _glp_ks_mt1(
    mut n: libc::c_int,
    mut a: *const libc::c_int,
    mut b: libc::c_int,
    mut c: *const libc::c_int,
    mut x: *mut libc::c_char,
) -> libc::c_int {
    let mut ks: *mut ks = 0 as *mut ks;
    let mut j: libc::c_int = 0;
    let mut s1: libc::c_int = 0;
    let mut s2: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    (n >= 0 as libc::c_int
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const libc::c_char,
                b"misc/ks.c\0" as *const u8 as *const libc::c_char,
                365 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ks = reduce(n, a, b, c);
    if ks.is_null() {
        return -(2147483647 as libc::c_int) - 1 as libc::c_int;
    }
    if (*ks).n > 0 as libc::c_int {
        mt1a(
            (*ks).n,
            (*ks).a as *const libc::c_int,
            (*ks).b,
            (*ks).c as *const libc::c_int,
            x,
        );
    }
    z = restore(ks, x);
    memcpy(
        &mut *x.offset(1 as libc::c_int as isize) as *mut libc::c_char
            as *mut libc::c_void,
        &mut *((*ks).x).offset(1 as libc::c_int as isize) as *mut libc::c_char
            as *const libc::c_void,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    free_ks(ks);
    s2 = 0 as libc::c_int;
    s1 = s2;
    j = 1 as libc::c_int;
    while j <= n {
        (*x.offset(j as isize) as libc::c_int == 0 as libc::c_int
            || *x.offset(j as isize) as libc::c_int == 1 as libc::c_int
            || {
                glp_assert_(
                    b"x[j] == 0 || x[j] == 1\0" as *const u8 as *const libc::c_char,
                    b"misc/ks.c\0" as *const u8 as *const libc::c_char,
                    382 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if *x.offset(j as isize) != 0 {
            s1 += *a.offset(j as isize);
            s2 += *c.offset(j as isize);
        }
        j += 1;
        j;
    }
    (s1 <= b
        || {
            glp_assert_(
                b"s1 <= b\0" as *const u8 as *const libc::c_char,
                b"misc/ks.c\0" as *const u8 as *const libc::c_char,
                386 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (s2 == z
        || {
            glp_assert_(
                b"s2 == z\0" as *const u8 as *const libc::c_char,
                b"misc/ks.c\0" as *const u8 as *const libc::c_char,
                387 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return z;
}
unsafe extern "C" fn greedy(
    mut n: libc::c_int,
    mut a: *const libc::c_int,
    mut b: libc::c_int,
    mut c: *const libc::c_int,
    mut x: *mut libc::c_char,
) -> libc::c_int {
    let mut mt: *mut mt = 0 as *mut mt;
    let mut j: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    (n >= 2 as libc::c_int
        || {
            glp_assert_(
                b"n >= 2\0" as *const u8 as *const libc::c_char,
                b"misc/ks.c\0" as *const u8 as *const libc::c_char,
                409 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    mt = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<mt>() as libc::c_ulong as libc::c_int,
    ) as *mut mt;
    j = 1 as libc::c_int;
    while j <= n {
        (*mt.offset(j as isize)).j = j;
        (*mt.offset(j as isize))
            .r = *c.offset(j as isize) as libc::c_float
            / *a.offset(j as isize) as libc::c_float;
        j += 1;
        j;
    }
    qsort(
        &mut *mt.offset(1 as libc::c_int as isize) as *mut mt as *mut libc::c_void,
        n as size_t,
        ::std::mem::size_of::<mt>() as libc::c_ulong,
        Some(
            fcmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    z = 0 as libc::c_int;
    s = z;
    j = 1 as libc::c_int;
    while j <= n {
        if s + *a.offset((*mt.offset(j as isize)).j as isize) > b {
            break;
        }
        *x
            .offset(
                (*mt.offset(j as isize)).j as isize,
            ) = 1 as libc::c_int as libc::c_char;
        s += *a.offset((*mt.offset(j as isize)).j as isize);
        z += *c.offset((*mt.offset(j as isize)).j as isize);
        j += 1;
        j;
    }
    j = j;
    while j <= n {
        *x
            .offset(
                (*mt.offset(j as isize)).j as isize,
            ) = 0 as libc::c_int as libc::c_char;
        j += 1;
        j;
    }
    glp_free(mt as *mut libc::c_void);
    return z;
}
pub unsafe extern "C" fn _glp_ks_greedy(
    mut n: libc::c_int,
    mut a: *const libc::c_int,
    mut b: libc::c_int,
    mut c: *const libc::c_int,
    mut x: *mut libc::c_char,
) -> libc::c_int {
    let mut ks: *mut ks = 0 as *mut ks;
    let mut j: libc::c_int = 0;
    let mut s1: libc::c_int = 0;
    let mut s2: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    (n >= 0 as libc::c_int
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const libc::c_char,
                b"misc/ks.c\0" as *const u8 as *const libc::c_char,
                438 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ks = reduce(n, a, b, c);
    if ks.is_null() {
        return -(2147483647 as libc::c_int) - 1 as libc::c_int;
    }
    if (*ks).n > 0 as libc::c_int {
        greedy(
            (*ks).n,
            (*ks).a as *const libc::c_int,
            (*ks).b,
            (*ks).c as *const libc::c_int,
            x,
        );
    }
    z = restore(ks, x);
    memcpy(
        &mut *x.offset(1 as libc::c_int as isize) as *mut libc::c_char
            as *mut libc::c_void,
        &mut *((*ks).x).offset(1 as libc::c_int as isize) as *mut libc::c_char
            as *const libc::c_void,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    free_ks(ks);
    s2 = 0 as libc::c_int;
    s1 = s2;
    j = 1 as libc::c_int;
    while j <= n {
        (*x.offset(j as isize) as libc::c_int == 0 as libc::c_int
            || *x.offset(j as isize) as libc::c_int == 1 as libc::c_int
            || {
                glp_assert_(
                    b"x[j] == 0 || x[j] == 1\0" as *const u8 as *const libc::c_char,
                    b"misc/ks.c\0" as *const u8 as *const libc::c_char,
                    455 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if *x.offset(j as isize) != 0 {
            s1 += *a.offset(j as isize);
            s2 += *c.offset(j as isize);
        }
        j += 1;
        j;
    }
    (s1 <= b
        || {
            glp_assert_(
                b"s1 <= b\0" as *const u8 as *const libc::c_char,
                b"misc/ks.c\0" as *const u8 as *const libc::c_char,
                459 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (s2 == z
        || {
            glp_assert_(
                b"s2 == z\0" as *const u8 as *const libc::c_char,
                b"misc/ks.c\0" as *const u8 as *const libc::c_char,
                460 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return z;
}
