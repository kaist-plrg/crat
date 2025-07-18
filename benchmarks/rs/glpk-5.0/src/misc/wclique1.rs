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
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vertex {
    pub i: libc::c_int,
    pub cw: libc::c_double,
}
unsafe extern "C" fn fcmp(
    mut xx: *const libc::c_void,
    mut yy: *const libc::c_void,
) -> libc::c_int {
    let mut x: *const vertex = xx as *const vertex;
    let mut y: *const vertex = yy as *const vertex;
    if (*x).cw > (*y).cw {
        return -(1 as libc::c_int);
    }
    if (*x).cw < (*y).cw {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_wclique1(
    mut n: libc::c_int,
    mut w: *const libc::c_double,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    mut info: *mut libc::c_void,
    mut c: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut v_list: *mut vertex = 0 as *mut vertex;
    let mut deg: libc::c_int = 0;
    let mut c_size: libc::c_int = 0;
    let mut d_size: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut kk: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut c_list: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut d_list: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut size: libc::c_int = 0 as libc::c_int;
    let mut c_wght: libc::c_double = 0.;
    let mut d_wght: libc::c_double = 0.;
    let mut sw: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut best: libc::c_double = 0.0f64;
    let mut d_flag: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut skip: *mut libc::c_char = 0 as *mut libc::c_char;
    (n >= 0 as libc::c_int
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const libc::c_char,
                b"misc/wclique1.c\0" as *const u8 as *const libc::c_char,
                80 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    i = 1 as libc::c_int;
    while i <= n {
        (*w.offset(i as isize) >= 0.0f64
            || {
                glp_assert_(
                    b"w[i] >= 0.0\0" as *const u8 as *const libc::c_char,
                    b"misc/wclique1.c\0" as *const u8 as *const libc::c_char,
                    82 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        i += 1;
        i;
    }
    if !(n == 0 as libc::c_int) {
        ind = glp_alloc(
            1 as libc::c_int + n,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        v_list = glp_alloc(
            1 as libc::c_int + n,
            ::std::mem::size_of::<vertex>() as libc::c_ulong as libc::c_int,
        ) as *mut vertex;
        c_list = glp_alloc(
            1 as libc::c_int + n,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        d_list = glp_alloc(
            1 as libc::c_int + n,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        d_flag = glp_alloc(
            1 as libc::c_int + n,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_char;
        skip = glp_alloc(
            1 as libc::c_int + n,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_char;
        sw = glp_alloc(
            1 as libc::c_int + n,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        i = 1 as libc::c_int;
        while i <= n {
            (*v_list.offset(i as isize)).i = i;
            (*v_list.offset(i as isize)).cw = *w.offset(i as isize);
            deg = func.unwrap()(info, i, ind);
            (0 as libc::c_int <= deg && deg < n
                || {
                    glp_assert_(
                        b"0 <= deg && deg < n\0" as *const u8 as *const libc::c_char,
                        b"misc/wclique1.c\0" as *const u8 as *const libc::c_char,
                        100 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            k = 1 as libc::c_int;
            while k <= deg {
                j = *ind.offset(k as isize);
                (1 as libc::c_int <= j && j <= n && j != i
                    || {
                        glp_assert_(
                            b"1 <= j && j <= n && j != i\0" as *const u8
                                as *const libc::c_char,
                            b"misc/wclique1.c\0" as *const u8 as *const libc::c_char,
                            103 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*v_list.offset(i as isize)).cw += *w.offset(j as isize);
                k += 1;
                k;
            }
            i += 1;
            i;
        }
        qsort(
            &mut *v_list.offset(1 as libc::c_int as isize) as *mut vertex
                as *mut libc::c_void,
            n as size_t,
            ::std::mem::size_of::<vertex>() as libc::c_ulong,
            Some(
                fcmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        memset(
            &mut *skip.offset(1 as libc::c_int as isize) as *mut libc::c_char
                as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong),
        );
        memset(
            &mut *d_flag.offset(1 as libc::c_int as isize) as *mut libc::c_char
                as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong),
        );
        l = 1 as libc::c_int;
        while l <= n {
            i = (*v_list.offset(l as isize)).i;
            if !(*skip.offset(i as isize) != 0) {
                c_size = 1 as libc::c_int;
                *c_list.offset(1 as libc::c_int as isize) = i;
                c_wght = *w.offset(i as isize);
                d_size = func.unwrap()(info, i, d_list);
                (0 as libc::c_int <= d_size && d_size < n
                    || {
                        glp_assert_(
                            b"0 <= d_size && d_size < n\0" as *const u8
                                as *const libc::c_char,
                            b"misc/wclique1.c\0" as *const u8 as *const libc::c_char,
                            127 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                d_wght = 0.0f64;
                k = 1 as libc::c_int;
                while k <= d_size {
                    j = *d_list.offset(k as isize);
                    (1 as libc::c_int <= j && j <= n && j != i
                        || {
                            glp_assert_(
                                b"1 <= j && j <= n && j != i\0" as *const u8
                                    as *const libc::c_char,
                                b"misc/wclique1.c\0" as *const u8 as *const libc::c_char,
                                131 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    (*d_flag.offset(j as isize) == 0
                        || {
                            glp_assert_(
                                b"!d_flag[j]\0" as *const u8 as *const libc::c_char,
                                b"misc/wclique1.c\0" as *const u8 as *const libc::c_char,
                                132 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    *d_flag.offset(j as isize) = 1 as libc::c_int as libc::c_char;
                    d_wght += *w.offset(j as isize);
                    k += 1;
                    k;
                }
                if !(c_wght + d_wght < best + 1e-5f64 * (1.0f64 + fabs(best))) {
                    k = 1 as libc::c_int;
                    while k <= d_size {
                        i = *d_list.offset(k as isize);
                        *sw.offset(i as isize) = *w.offset(i as isize);
                        deg = func.unwrap()(info, i, ind);
                        (0 as libc::c_int <= deg && deg < n
                            || {
                                glp_assert_(
                                    b"0 <= deg && deg < n\0" as *const u8
                                        as *const libc::c_char,
                                    b"misc/wclique1.c\0" as *const u8 as *const libc::c_char,
                                    148 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        kk = 1 as libc::c_int;
                        while kk <= deg {
                            j = *ind.offset(kk as isize);
                            (1 as libc::c_int <= j && j <= n && j != i
                                || {
                                    glp_assert_(
                                        b"1 <= j && j <= n && j != i\0" as *const u8
                                            as *const libc::c_char,
                                        b"misc/wclique1.c\0" as *const u8 as *const libc::c_char,
                                        151 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                            if *d_flag.offset(j as isize) != 0 {
                                *sw.offset(i as isize) += *w.offset(j as isize);
                            }
                            kk += 1;
                            kk;
                        }
                        k += 1;
                        k;
                    }
                    loop {
                        if !(d_size > 0 as libc::c_int) {
                            current_block = 13723035087248630346;
                            break;
                        }
                        if c_wght + d_wght < best + 1e-5f64 * (1.0f64 + fabs(best)) {
                            current_block = 17802063206392647197;
                            break;
                        }
                        i = *d_list.offset(1 as libc::c_int as isize);
                        k = 2 as libc::c_int;
                        while k <= d_size {
                            j = *d_list.offset(k as isize);
                            if *sw.offset(i as isize) < *sw.offset(j as isize) {
                                i = j;
                            }
                            k += 1;
                            k;
                        }
                        c_size += 1;
                        c_size;
                        *c_list.offset(c_size as isize) = i;
                        c_wght += *w.offset(i as isize);
                        deg = func.unwrap()(info, i, ind);
                        (0 as libc::c_int <= deg && deg < n
                            || {
                                glp_assert_(
                                    b"0 <= deg && deg < n\0" as *const u8
                                        as *const libc::c_char,
                                    b"misc/wclique1.c\0" as *const u8 as *const libc::c_char,
                                    175 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        k = 1 as libc::c_int;
                        while k <= deg {
                            j = *ind.offset(k as isize);
                            (1 as libc::c_int <= j && j <= n && j != i
                                || {
                                    glp_assert_(
                                        b"1 <= j && j <= n && j != i\0" as *const u8
                                            as *const libc::c_char,
                                        b"misc/wclique1.c\0" as *const u8 as *const libc::c_char,
                                        178 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                            if *d_flag.offset(j as isize) != 0 {
                                (*d_flag.offset(j as isize) as libc::c_int
                                    == 1 as libc::c_int
                                    || {
                                        glp_assert_(
                                            b"d_flag[j] == 1\0" as *const u8 as *const libc::c_char,
                                            b"misc/wclique1.c\0" as *const u8 as *const libc::c_char,
                                            181 as libc::c_int,
                                        );
                                        1 as libc::c_int != 0
                                    }) as libc::c_int;
                                *d_flag
                                    .offset(j as isize) = 2 as libc::c_int as libc::c_char;
                            }
                            k += 1;
                            k;
                        }
                        kk = d_size;
                        d_size = 0 as libc::c_int;
                        k = 1 as libc::c_int;
                        while k <= kk {
                            j = *d_list.offset(k as isize);
                            if *d_flag.offset(j as isize) as libc::c_int
                                == 1 as libc::c_int
                            {
                                *d_flag
                                    .offset(j as isize) = 0 as libc::c_int as libc::c_char;
                                d_wght -= *w.offset(j as isize);
                            } else if *d_flag.offset(j as isize) as libc::c_int
                                == 2 as libc::c_int
                            {
                                d_size += 1;
                                *d_list.offset(d_size as isize) = j;
                                *d_flag
                                    .offset(j as isize) = 1 as libc::c_int as libc::c_char;
                            } else {
                                (d_flag != d_flag
                                    || {
                                        glp_assert_(
                                            b"d_flag != d_flag\0" as *const u8 as *const libc::c_char,
                                            b"misc/wclique1.c\0" as *const u8 as *const libc::c_char,
                                            200 as libc::c_int,
                                        );
                                        1 as libc::c_int != 0
                                    }) as libc::c_int;
                            }
                            k += 1;
                            k;
                        }
                    }
                    match current_block {
                        17802063206392647197 => {}
                        _ => {
                            if best < c_wght {
                                best = c_wght;
                                size = c_size;
                                (1 as libc::c_int <= size && size <= n
                                    || {
                                        glp_assert_(
                                            b"1 <= size && size <= n\0" as *const u8
                                                as *const libc::c_char,
                                            b"misc/wclique1.c\0" as *const u8 as *const libc::c_char,
                                            207 as libc::c_int,
                                        );
                                        1 as libc::c_int != 0
                                    }) as libc::c_int;
                                memcpy(
                                    &mut *c.offset(1 as libc::c_int as isize)
                                        as *mut libc::c_int as *mut libc::c_void,
                                    &mut *c_list.offset(1 as libc::c_int as isize)
                                        as *mut libc::c_int as *const libc::c_void,
                                    (size as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                                        ),
                                );
                            }
                        }
                    }
                }
                k = 1 as libc::c_int;
                while k <= c_size {
                    *skip
                        .offset(
                            *c_list.offset(k as isize) as isize,
                        ) = 1 as libc::c_int as libc::c_char;
                    k += 1;
                    k;
                }
                k = 1 as libc::c_int;
                while k <= d_size {
                    *d_flag
                        .offset(
                            *d_list.offset(k as isize) as isize,
                        ) = 0 as libc::c_int as libc::c_char;
                    k += 1;
                    k;
                }
            }
            l += 1;
            l;
        }
        glp_free(ind as *mut libc::c_void);
        glp_free(v_list as *mut libc::c_void);
        glp_free(c_list as *mut libc::c_void);
        glp_free(d_list as *mut libc::c_void);
        glp_free(d_flag as *mut libc::c_void);
        glp_free(skip as *mut libc::c_void);
        glp_free(sw as *mut libc::c_void);
    }
    return size;
}
