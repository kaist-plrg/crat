use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
}
pub unsafe extern "C" fn _glp_triang(
    mut m: libc::c_int,
    mut n: libc::c_int,
    mut mat: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *mut libc::c_int,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    mut info: *mut libc::c_void,
    mut tol: libc::c_double,
    mut rn: *mut libc::c_int,
    mut cn: *mut libc::c_int,
) -> libc::c_int {
    let mut head: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut kk: libc::c_int = 0;
    let mut ks: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut len2: libc::c_int = 0;
    let mut next_j: libc::c_int = 0;
    let mut ns: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut cind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut cnt: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut list: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut prev: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut next: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut cval: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rval: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut big: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut flag: *mut libc::c_char = 0 as *mut libc::c_char;
    cind = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    cval = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    rind = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    rval = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    ptr = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    cnt = ptr;
    list = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    prev = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    next = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    big = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    flag = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    len = 0 as libc::c_int;
    while len <= m {
        *ptr.offset(len as isize) = 0 as libc::c_int;
        len += 1;
        len;
    }
    j = 1 as libc::c_int;
    while j <= n {
        len = mat.unwrap()(info, -j, cind, cval);
        (0 as libc::c_int <= len && len <= m
            || {
                glp_assert_(
                    b"0 <= len && len <= m\0" as *const u8 as *const libc::c_char,
                    b"misc/triang.c\0" as *const u8 as *const libc::c_char,
                    134 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        *next.offset(j as isize) = *ptr.offset(len as isize);
        *ptr.offset(len as isize) = j;
        *big.offset(j as isize) = 0.0f64;
        k = 1 as libc::c_int;
        while k <= len {
            if *big.offset(j as isize) < fabs(*cval.offset(k as isize)) {
                *big.offset(j as isize) = fabs(*cval.offset(k as isize));
            }
            k += 1;
            k;
        }
        j += 1;
        j;
    }
    head = 0 as libc::c_int;
    len = 0 as libc::c_int;
    while len <= m {
        j = *ptr.offset(len as isize);
        while j != 0 as libc::c_int {
            next_j = *next.offset(j as isize);
            *prev.offset(j as isize) = 0 as libc::c_int;
            *next.offset(j as isize) = head;
            if head != 0 as libc::c_int {
                *prev.offset(head as isize) = j;
            }
            head = j;
            j = next_j;
        }
        len += 1;
        len;
    }
    j = 1 as libc::c_int;
    while j <= n {
        *flag.offset(j as isize) = 1 as libc::c_int as libc::c_char;
        j += 1;
        j;
    }
    ns = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= m {
        let ref mut fresh0 = *cnt.offset(i as isize);
        *fresh0 = mat.unwrap()(info, i, rind, rval);
        len = *fresh0;
        (0 as libc::c_int <= len && len <= n
            || {
                glp_assert_(
                    b"0 <= len && len <= n\0" as *const u8 as *const libc::c_char,
                    b"misc/triang.c\0" as *const u8 as *const libc::c_char,
                    192 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if len == 1 as libc::c_int {
            j = *rind.offset(1 as libc::c_int as isize);
            (1 as libc::c_int <= j && j <= n
                || {
                    glp_assert_(
                        b"1 <= j && j <= n\0" as *const u8 as *const libc::c_char,
                        b"misc/triang.c\0" as *const u8 as *const libc::c_char,
                        196 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if *flag.offset(j as isize) as libc::c_int != 2 as libc::c_int {
                *flag.offset(j as isize) = 2 as libc::c_int as libc::c_char;
                ns += 1;
                *list.offset(ns as isize) = j;
            }
        }
        i += 1;
        i;
    }
    size = 0 as libc::c_int;
    while head != 0 as libc::c_int {
        if ns == 0 as libc::c_int {
            j = head;
            len = mat.unwrap()(info, -j, cind, cval);
            (0 as libc::c_int <= len && len <= m
                || {
                    glp_assert_(
                        b"0 <= len && len <= m\0" as *const u8 as *const libc::c_char,
                        b"misc/triang.c\0" as *const u8 as *const libc::c_char,
                        217 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        } else {
            let fresh1 = ns;
            ns = ns - 1;
            j = *list.offset(fresh1 as isize);
            (*flag.offset(j as isize) as libc::c_int == 2 as libc::c_int
                || {
                    glp_assert_(
                        b"flag[j] == 2\0" as *const u8 as *const libc::c_char,
                        b"misc/triang.c\0" as *const u8 as *const libc::c_char,
                        222 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            len = mat.unwrap()(info, -j, cind, cval);
            (0 as libc::c_int <= len && len <= m
                || {
                    glp_assert_(
                        b"0 <= len && len <= m\0" as *const u8 as *const libc::c_char,
                        b"misc/triang.c\0" as *const u8 as *const libc::c_char,
                        226 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            kk = 0 as libc::c_int;
            k = 1 as libc::c_int;
            while k <= len {
                i = *cind.offset(k as isize);
                (1 as libc::c_int <= i && i <= m
                    || {
                        glp_assert_(
                            b"1 <= i && i <= m\0" as *const u8 as *const libc::c_char,
                            b"misc/triang.c\0" as *const u8 as *const libc::c_char,
                            230 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                if *cnt.offset(i as isize) == 1 as libc::c_int {
                    if kk == 0 as libc::c_int
                        || fabs(*cval.offset(kk as isize))
                            < fabs(*cval.offset(k as isize))
                    {
                        kk = k;
                    }
                }
                k += 1;
                k;
            }
            (kk > 0 as libc::c_int
                || {
                    glp_assert_(
                        b"kk > 0\0" as *const u8 as *const libc::c_char,
                        b"misc/triang.c\0" as *const u8 as *const libc::c_char,
                        237 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if !(fabs(*cval.offset(kk as isize)) < tol * *big.offset(j as isize)) {
                size += 1;
                size;
                *rn.offset(size as isize) = *cind.offset(kk as isize);
                *cn.offset(size as isize) = j;
            }
        }
        (*flag.offset(j as isize) as libc::c_int != 0
            || {
                glp_assert_(
                    b"flag[j]\0" as *const u8 as *const libc::c_char,
                    b"misc/triang.c\0" as *const u8 as *const libc::c_char,
                    250 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        *flag.offset(j as isize) = 0 as libc::c_int as libc::c_char;
        if *prev.offset(j as isize) == 0 as libc::c_int {
            head = *next.offset(j as isize);
        } else {
            *next.offset(*prev.offset(j as isize) as isize) = *next.offset(j as isize);
        }
        if !(*next.offset(j as isize) == 0 as libc::c_int) {
            *prev.offset(*next.offset(j as isize) as isize) = *prev.offset(j as isize);
        }
        k = 1 as libc::c_int;
        while k <= len {
            i = *cind.offset(k as isize);
            (1 as libc::c_int <= i && i <= m
                || {
                    glp_assert_(
                        b"1 <= i && i <= m\0" as *const u8 as *const libc::c_char,
                        b"misc/triang.c\0" as *const u8 as *const libc::c_char,
                        263 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*cnt.offset(i as isize) > 0 as libc::c_int
                || {
                    glp_assert_(
                        b"cnt[i] > 0\0" as *const u8 as *const libc::c_char,
                        b"misc/triang.c\0" as *const u8 as *const libc::c_char,
                        264 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            let ref mut fresh2 = *cnt.offset(i as isize);
            *fresh2 -= 1;
            *fresh2;
            if *cnt.offset(i as isize) == 1 as libc::c_int {
                len2 = mat.unwrap()(info, i, rind, rval);
                (0 as libc::c_int <= len2 && len2 <= n
                    || {
                        glp_assert_(
                            b"0 <= len2 && len2 <= n\0" as *const u8
                                as *const libc::c_char,
                            b"misc/triang.c\0" as *const u8 as *const libc::c_char,
                            271 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                ks = 0 as libc::c_int;
                kk = 1 as libc::c_int;
                while kk <= len2 {
                    jj = *rind.offset(kk as isize);
                    (1 as libc::c_int <= jj && jj <= n
                        || {
                            glp_assert_(
                                b"1 <= jj && jj <= n\0" as *const u8 as *const libc::c_char,
                                b"misc/triang.c\0" as *const u8 as *const libc::c_char,
                                275 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    if *flag.offset(jj as isize) != 0 {
                        (ks == 0 as libc::c_int
                            || {
                                glp_assert_(
                                    b"ks == 0\0" as *const u8 as *const libc::c_char,
                                    b"misc/triang.c\0" as *const u8 as *const libc::c_char,
                                    277 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        ks = kk;
                    }
                    kk += 1;
                    kk;
                }
                (ks > 0 as libc::c_int
                    || {
                        glp_assert_(
                            b"ks > 0\0" as *const u8 as *const libc::c_char,
                            b"misc/triang.c\0" as *const u8 as *const libc::c_char,
                            281 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                jj = *rind.offset(ks as isize);
                if *flag.offset(jj as isize) as libc::c_int != 2 as libc::c_int {
                    *flag.offset(jj as isize) = 2 as libc::c_int as libc::c_char;
                    ns += 1;
                    *list.offset(ns as isize) = jj;
                }
            }
            k += 1;
            k;
        }
    }
    i = 1 as libc::c_int;
    while i <= m {
        (*cnt.offset(i as isize) == 0 as libc::c_int
            || {
                glp_assert_(
                    b"cnt[i] == 0\0" as *const u8 as *const libc::c_char,
                    b"misc/triang.c\0" as *const u8 as *const libc::c_char,
                    294 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        i += 1;
        i;
    }
    glp_free(cind as *mut libc::c_void);
    glp_free(cval as *mut libc::c_void);
    glp_free(rind as *mut libc::c_void);
    glp_free(rval as *mut libc::c_void);
    glp_free(ptr as *mut libc::c_void);
    glp_free(list as *mut libc::c_void);
    glp_free(prev as *mut libc::c_void);
    glp_free(next as *mut libc::c_void);
    glp_free(big as *mut libc::c_void);
    glp_free(flag as *mut libc::c_void);
    return size;
}
