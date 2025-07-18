use ::libc;
extern "C" {
    pub type DMP;
    fn _glp_dmp_delete_pool(pool: *mut DMP);
    fn _glp_dmp_get_atom(pool: *mut DMP, size: libc::c_int) -> *mut libc::c_void;
    fn _glp_dmp_create_pool() -> *mut DMP;
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
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_cfg {
    pub n: libc::c_int,
    pub pos: *mut libc::c_int,
    pub neg: *mut libc::c_int,
    pub pool: *mut DMP,
    pub nv_max: libc::c_int,
    pub nv: libc::c_int,
    pub ref_0: *mut libc::c_int,
    pub vptr: *mut *mut CFGVLE,
    pub cptr: *mut *mut CFGCLE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CFGCLE {
    pub vptr: *mut CFGVLE,
    pub next: *mut CFGCLE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CFGVLE {
    pub v: libc::c_int,
    pub next: *mut CFGVLE,
}
pub type CFG = glp_cfg;
pub unsafe extern "C" fn _glp_cfg_create_graph(
    mut n: libc::c_int,
    mut nv_max: libc::c_int,
) -> *mut CFG {
    let mut G: *mut CFG = 0 as *mut CFG;
    (n >= 0 as libc::c_int
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const libc::c_char,
                b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                40 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (0 as libc::c_int <= nv_max && nv_max <= n + n
        || {
            glp_assert_(
                b"0 <= nv_max && nv_max <= n + n\0" as *const u8 as *const libc::c_char,
                b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                41 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    G = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<CFG>() as libc::c_ulong as libc::c_int,
    ) as *mut CFG;
    (*G).n = n;
    (*G)
        .pos = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    memset(
        &mut *((*G).pos).offset(1 as libc::c_int as isize) as *mut libc::c_int
            as *mut libc::c_void,
        0 as libc::c_int,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    (*G)
        .neg = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    memset(
        &mut *((*G).neg).offset(1 as libc::c_int as isize) as *mut libc::c_int
            as *mut libc::c_void,
        0 as libc::c_int,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    (*G).pool = _glp_dmp_create_pool();
    (*G).nv_max = nv_max;
    (*G).nv = 0 as libc::c_int;
    (*G)
        .ref_0 = glp_alloc(
        1 as libc::c_int + nv_max,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*G)
        .vptr = glp_alloc(
        1 as libc::c_int + nv_max,
        ::std::mem::size_of::<*mut CFGVLE>() as libc::c_ulong as libc::c_int,
    ) as *mut *mut CFGVLE;
    (*G)
        .cptr = glp_alloc(
        1 as libc::c_int + nv_max,
        ::std::mem::size_of::<*mut CFGCLE>() as libc::c_ulong as libc::c_int,
    ) as *mut *mut CFGCLE;
    return G;
}
unsafe extern "C" fn add_edge(mut G: *mut CFG, mut v: libc::c_int, mut w: libc::c_int) {
    let mut pool: *mut DMP = (*G).pool;
    let mut nv: libc::c_int = (*G).nv;
    let mut vptr: *mut *mut CFGVLE = (*G).vptr;
    let mut vle: *mut CFGVLE = 0 as *mut CFGVLE;
    (1 as libc::c_int <= v && v <= nv
        || {
            glp_assert_(
                b"1 <= v && v <= nv\0" as *const u8 as *const libc::c_char,
                b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                84 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= w && w <= nv
        || {
            glp_assert_(
                b"1 <= w && w <= nv\0" as *const u8 as *const libc::c_char,
                b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (v != w
        || {
            glp_assert_(
                b"v != w\0" as *const u8 as *const libc::c_char,
                b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                86 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    vle = _glp_dmp_get_atom(
        pool,
        ::std::mem::size_of::<CFGVLE>() as libc::c_ulong as libc::c_int,
    ) as *mut CFGVLE;
    (*vle).v = w;
    (*vle).next = *vptr.offset(v as isize);
    let ref mut fresh0 = *vptr.offset(v as isize);
    *fresh0 = vle;
    vle = _glp_dmp_get_atom(
        pool,
        ::std::mem::size_of::<CFGVLE>() as libc::c_ulong as libc::c_int,
    ) as *mut CFGVLE;
    (*vle).v = v;
    (*vle).next = *vptr.offset(w as isize);
    let ref mut fresh1 = *vptr.offset(w as isize);
    *fresh1 = vle;
}
pub unsafe extern "C" fn _glp_cfg_add_clique(
    mut G: *mut CFG,
    mut size: libc::c_int,
    mut ind: *const libc::c_int,
) {
    let mut n: libc::c_int = (*G).n;
    let mut pos: *mut libc::c_int = (*G).pos;
    let mut neg: *mut libc::c_int = (*G).neg;
    let mut pool: *mut DMP = (*G).pool;
    let mut nv_max: libc::c_int = (*G).nv_max;
    let mut ref_0: *mut libc::c_int = (*G).ref_0;
    let mut vptr: *mut *mut CFGVLE = (*G).vptr;
    let mut cptr: *mut *mut CFGCLE = (*G).cptr;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    (2 as libc::c_int <= size && size <= nv_max
        || {
            glp_assert_(
                b"2 <= size && size <= nv_max\0" as *const u8 as *const libc::c_char,
                b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                108 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = 1 as libc::c_int;
    while k <= size {
        j = *ind.offset(k as isize);
        if j > 0 as libc::c_int {
            (1 as libc::c_int <= j && j <= n
                || {
                    glp_assert_(
                        b"1 <= j && j <= n\0" as *const u8 as *const libc::c_char,
                        b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                        114 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if *pos.offset(j as isize) == 0 as libc::c_int {
                (*G).nv += 1;
                let ref mut fresh2 = *pos.offset(j as isize);
                *fresh2 = (*G).nv;
                v = *fresh2;
                (v <= nv_max
                    || {
                        glp_assert_(
                            b"v <= nv_max\0" as *const u8 as *const libc::c_char,
                            b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                            118 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                *ref_0.offset(v as isize) = j;
                let ref mut fresh3 = *vptr.offset(v as isize);
                *fresh3 = 0 as *mut CFGVLE;
                let ref mut fresh4 = *cptr.offset(v as isize);
                *fresh4 = 0 as *mut CFGCLE;
                if *neg.offset(j as isize) != 0 as libc::c_int {
                    add_edge(G, v, *neg.offset(j as isize));
                }
            }
        } else {
            j = -j;
            (1 as libc::c_int <= j && j <= n
                || {
                    glp_assert_(
                        b"1 <= j && j <= n\0" as *const u8 as *const libc::c_char,
                        b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                        131 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if *neg.offset(j as isize) == 0 as libc::c_int {
                (*G).nv += 1;
                let ref mut fresh5 = *neg.offset(j as isize);
                *fresh5 = (*G).nv;
                v = *fresh5;
                (v <= nv_max
                    || {
                        glp_assert_(
                            b"v <= nv_max\0" as *const u8 as *const libc::c_char,
                            b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                            135 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                *ref_0.offset(v as isize) = j;
                let ref mut fresh6 = *vptr.offset(v as isize);
                *fresh6 = 0 as *mut CFGVLE;
                let ref mut fresh7 = *cptr.offset(v as isize);
                *fresh7 = 0 as *mut CFGCLE;
                if *pos.offset(j as isize) != 0 as libc::c_int {
                    add_edge(G, v, *pos.offset(j as isize));
                }
            }
        }
        k += 1;
        k;
    }
    if size == 2 as libc::c_int {
        add_edge(
            G,
            if *ind.offset(1 as libc::c_int as isize) > 0 as libc::c_int {
                *pos.offset(*ind.offset(1 as libc::c_int as isize) as isize)
            } else {
                *neg.offset(-*ind.offset(1 as libc::c_int as isize) as isize)
            },
            if *ind.offset(2 as libc::c_int as isize) > 0 as libc::c_int {
                *pos.offset(*ind.offset(2 as libc::c_int as isize) as isize)
            } else {
                *neg.offset(-*ind.offset(2 as libc::c_int as isize) as isize)
            },
        );
    } else {
        let mut vp: *mut CFGVLE = 0 as *mut CFGVLE;
        let mut vle: *mut CFGVLE = 0 as *mut CFGVLE;
        let mut cle: *mut CFGCLE = 0 as *mut CFGCLE;
        vp = 0 as *mut CFGVLE;
        k = 1 as libc::c_int;
        while k <= size {
            vle = _glp_dmp_get_atom(
                pool,
                ::std::mem::size_of::<CFGVLE>() as libc::c_ulong as libc::c_int,
            ) as *mut CFGVLE;
            (*vle)
                .v = if *ind.offset(k as isize) > 0 as libc::c_int {
                *pos.offset(*ind.offset(k as isize) as isize)
            } else {
                *neg.offset(-*ind.offset(k as isize) as isize)
            };
            (*vle).next = vp;
            vp = vle;
            k += 1;
            k;
        }
        k = 1 as libc::c_int;
        while k <= size {
            cle = _glp_dmp_get_atom(
                pool,
                ::std::mem::size_of::<CFGCLE>() as libc::c_ulong as libc::c_int,
            ) as *mut CFGCLE;
            (*cle).vptr = vp;
            v = if *ind.offset(k as isize) > 0 as libc::c_int {
                *pos.offset(*ind.offset(k as isize) as isize)
            } else {
                *neg.offset(-*ind.offset(k as isize) as isize)
            };
            (*cle).next = *cptr.offset(v as isize);
            let ref mut fresh8 = *cptr.offset(v as isize);
            *fresh8 = cle;
            k += 1;
            k;
        }
    };
}
pub unsafe extern "C" fn _glp_cfg_get_adjacent(
    mut G: *mut CFG,
    mut v: libc::c_int,
    mut ind: *mut libc::c_int,
) -> libc::c_int {
    let mut nv: libc::c_int = (*G).nv;
    let mut ref_0: *mut libc::c_int = (*G).ref_0;
    let mut vptr: *mut *mut CFGVLE = (*G).vptr;
    let mut cptr: *mut *mut CFGCLE = (*G).cptr;
    let mut vle: *mut CFGVLE = 0 as *mut CFGVLE;
    let mut cle: *mut CFGCLE = 0 as *mut CFGCLE;
    let mut k: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    (1 as libc::c_int <= v && v <= nv
        || {
            glp_assert_(
                b"1 <= v && v <= nv\0" as *const u8 as *const libc::c_char,
                b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                193 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    len = 0 as libc::c_int;
    vle = *vptr.offset(v as isize);
    while !vle.is_null() {
        w = (*vle).v;
        (1 as libc::c_int <= w && w <= nv
            || {
                glp_assert_(
                    b"1 <= w && w <= nv\0" as *const u8 as *const libc::c_char,
                    b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                    198 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (w != v
            || {
                glp_assert_(
                    b"w != v\0" as *const u8 as *const libc::c_char,
                    b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                    199 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if *ref_0.offset(w as isize) > 0 as libc::c_int {
            len += 1;
            *ind.offset(len as isize) = w;
            *ref_0.offset(w as isize) = -*ref_0.offset(w as isize);
        }
        vle = (*vle).next;
    }
    cle = *cptr.offset(v as isize);
    while !cle.is_null() {
        vle = (*cle).vptr;
        while !vle.is_null() {
            w = (*vle).v;
            (1 as libc::c_int <= w && w <= nv
                || {
                    glp_assert_(
                        b"1 <= w && w <= nv\0" as *const u8 as *const libc::c_char,
                        b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                        210 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if w != v && *ref_0.offset(w as isize) > 0 as libc::c_int {
                len += 1;
                *ind.offset(len as isize) = w;
                *ref_0.offset(w as isize) = -*ref_0.offset(w as isize);
            }
            vle = (*vle).next;
        }
        cle = (*cle).next;
    }
    (1 as libc::c_int <= len && len < nv
        || {
            glp_assert_(
                b"1 <= len && len < nv\0" as *const u8 as *const libc::c_char,
                b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                217 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = 1 as libc::c_int;
    while k <= len {
        w = *ind.offset(k as isize);
        *ref_0.offset(w as isize) = -*ref_0.offset(w as isize);
        k += 1;
        k;
    }
    return len;
}
unsafe extern "C" fn intersection(
    mut d_len: libc::c_int,
    mut d_ind: *mut libc::c_int,
    mut d_pos: *mut libc::c_int,
    mut len: libc::c_int,
    mut ind: *const libc::c_int,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut new_len: libc::c_int = 0;
    t = 1 as libc::c_int;
    while t <= len {
        v = *ind.offset(t as isize);
        k = *d_pos.offset(v as isize);
        if k != 0 as libc::c_int {
            (*d_ind.offset(k as isize) == v
                || {
                    glp_assert_(
                        b"d_ind[k] == v\0" as *const u8 as *const libc::c_char,
                        b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                        269 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            *d_ind.offset(k as isize) = -v;
        }
        t += 1;
        t;
    }
    new_len = 0 as libc::c_int;
    k = 1 as libc::c_int;
    while k <= d_len {
        v = *d_ind.offset(k as isize);
        if v < 0 as libc::c_int {
            v = -v;
            new_len += 1;
            new_len;
            *d_ind.offset(new_len as isize) = v;
            *d_pos.offset(v as isize) = new_len;
        } else {
            *d_pos.offset(v as isize) = 0 as libc::c_int;
        }
        k += 1;
        k;
    }
    return new_len;
}
pub unsafe extern "C" fn _glp_cfg_expand_clique(
    mut G: *mut CFG,
    mut c_len: libc::c_int,
    mut c_ind: *mut libc::c_int,
) -> libc::c_int {
    let mut nv: libc::c_int = (*G).nv;
    let mut d_len: libc::c_int = 0;
    let mut d_ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut d_pos: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut len: libc::c_int = 0;
    let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut k: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    (0 as libc::c_int <= c_len && c_len <= nv
        || {
            glp_assert_(
                b"0 <= c_len && c_len <= nv\0" as *const u8 as *const libc::c_char,
                b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                298 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    d_ind = glp_alloc(
        1 as libc::c_int + nv,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    d_pos = glp_alloc(
        1 as libc::c_int + nv,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    ind = glp_alloc(
        1 as libc::c_int + nv,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    d_len = nv;
    k = 1 as libc::c_int;
    while k <= nv {
        let ref mut fresh9 = *d_pos.offset(k as isize);
        *fresh9 = k;
        *d_ind.offset(k as isize) = *fresh9;
        k += 1;
        k;
    }
    k = 1 as libc::c_int;
    while k <= c_len {
        v = *c_ind.offset(k as isize);
        (1 as libc::c_int <= v && v <= nv
            || {
                glp_assert_(
                    b"1 <= v && v <= nv\0" as *const u8 as *const libc::c_char,
                    b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                    311 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*d_pos.offset(v as isize) != 0 as libc::c_int
            || {
                glp_assert_(
                    b"d_pos[v] != 0\0" as *const u8 as *const libc::c_char,
                    b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                    313 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        len = _glp_cfg_get_adjacent(G, v, ind);
        d_len = intersection(d_len, d_ind, d_pos, len, ind as *const libc::c_int);
        (*d_pos.offset(v as isize) == 0 as libc::c_int
            || {
                glp_assert_(
                    b"d_pos[v] == 0\0" as *const u8 as *const libc::c_char,
                    b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                    319 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        k += 1;
        k;
    }
    while d_len > 0 as libc::c_int {
        v = *d_ind.offset(1 as libc::c_int as isize);
        (1 as libc::c_int <= v && v <= nv
            || {
                glp_assert_(
                    b"1 <= v && v <= nv\0" as *const u8 as *const libc::c_char,
                    b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                    325 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        c_len += 1;
        *c_ind.offset(c_len as isize) = v;
        len = _glp_cfg_get_adjacent(G, v, ind);
        d_len = intersection(d_len, d_ind, d_pos, len, ind as *const libc::c_int);
        (*d_pos.offset(v as isize) == 0 as libc::c_int
            || {
                glp_assert_(
                    b"d_pos[v] == 0\0" as *const u8 as *const libc::c_char,
                    b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                    334 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    glp_free(d_ind as *mut libc::c_void);
    glp_free(d_pos as *mut libc::c_void);
    glp_free(ind as *mut libc::c_void);
    return c_len;
}
pub unsafe extern "C" fn _glp_cfg_check_clique(
    mut G: *mut CFG,
    mut c_len: libc::c_int,
    mut c_ind: *const libc::c_int,
) {
    let mut nv: libc::c_int = (*G).nv;
    let mut k: libc::c_int = 0;
    let mut kk: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut flag: *mut libc::c_char = 0 as *mut libc::c_char;
    ind = glp_alloc(
        1 as libc::c_int + nv,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    flag = glp_alloc(
        1 as libc::c_int + nv,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    memset(
        &mut *flag.offset(1 as libc::c_int as isize) as *mut libc::c_char
            as *mut libc::c_void,
        0 as libc::c_int,
        nv as libc::c_ulong,
    );
    (c_len >= 0 as libc::c_int
        || {
            glp_assert_(
                b"c_len >= 0\0" as *const u8 as *const libc::c_char,
                b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                360 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = 1 as libc::c_int;
    while k <= c_len {
        v = *c_ind.offset(k as isize);
        (1 as libc::c_int <= v && v <= nv
            || {
                glp_assert_(
                    b"1 <= v && v <= nv\0" as *const u8 as *const libc::c_char,
                    b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                    364 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        len = _glp_cfg_get_adjacent(G, v, ind);
        kk = 1 as libc::c_int;
        while kk <= len {
            w = *ind.offset(kk as isize);
            (1 as libc::c_int <= w && w <= nv
                || {
                    glp_assert_(
                        b"1 <= w && w <= nv\0" as *const u8 as *const libc::c_char,
                        b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                        369 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (w != v
                || {
                    glp_assert_(
                        b"w != v\0" as *const u8 as *const libc::c_char,
                        b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                        370 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            *flag.offset(w as isize) = 1 as libc::c_int as libc::c_char;
            kk += 1;
            kk;
        }
        kk = 1 as libc::c_int;
        while kk <= c_len {
            w = *c_ind.offset(kk as isize);
            (1 as libc::c_int <= w && w <= nv
                || {
                    glp_assert_(
                        b"1 <= w && w <= nv\0" as *const u8 as *const libc::c_char,
                        b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                        377 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if w != v {
                (*flag.offset(w as isize) as libc::c_int != 0
                    || {
                        glp_assert_(
                            b"flag[w]\0" as *const u8 as *const libc::c_char,
                            b"intopt/cfg.c\0" as *const u8 as *const libc::c_char,
                            379 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            kk += 1;
            kk;
        }
        kk = 1 as libc::c_int;
        while kk <= len {
            *flag
                .offset(
                    *ind.offset(kk as isize) as isize,
                ) = 0 as libc::c_int as libc::c_char;
            kk += 1;
            kk;
        }
        k += 1;
        k;
    }
    glp_free(ind as *mut libc::c_void);
    glp_free(flag as *mut libc::c_void);
}
pub unsafe extern "C" fn _glp_cfg_delete_graph(mut G: *mut CFG) {
    glp_free((*G).pos as *mut libc::c_void);
    glp_free((*G).neg as *mut libc::c_void);
    _glp_dmp_delete_pool((*G).pool);
    glp_free((*G).ref_0 as *mut libc::c_void);
    glp_free((*G).vptr as *mut libc::c_void);
    glp_free((*G).cptr as *mut libc::c_void);
    glp_free(G as *mut libc::c_void);
}
