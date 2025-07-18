use ::libc;
extern "C" {
    pub type DMP;
    pub type AVLNODE;
    pub type BFD;
    pub type AVL;
    pub type glp_tree;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn _glp_cfg_get_adjacent(
        G: *mut CFG,
        v: libc::c_int,
        ind: *mut libc::c_int,
    ) -> libc::c_int;
    fn _glp_cfg_add_clique(G: *mut CFG, size: libc::c_int, ind: *const libc::c_int);
    fn _glp_cfg_create_graph(n: libc::c_int, nv_max: libc::c_int) -> *mut CFG;
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
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_get_mat_row(
        P: *mut glp_prob,
        i: libc::c_int,
        ind: *mut libc::c_int,
        val: *mut libc::c_double,
    ) -> libc::c_int;
    fn glp_get_num_bin(P: *mut glp_prob) -> libc::c_int;
    fn _glp_wclique(
        n: libc::c_int,
        w: *const libc::c_int,
        a: *const libc::c_uchar,
        ind: *mut libc::c_int,
    ) -> libc::c_int;
    fn _glp_wclique1(
        n: libc::c_int,
        w: *const libc::c_double,
        func_0: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                libc::c_int,
                *mut libc::c_int,
            ) -> libc::c_int,
        >,
        info: *mut libc::c_void,
        c: *mut libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct term {
    pub ind: libc::c_int,
    pub val: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPROW {
    pub i: libc::c_int,
    pub name: *mut libc::c_char,
    pub node: *mut AVLNODE,
    pub level: libc::c_int,
    pub origin: libc::c_uchar,
    pub klass: libc::c_uchar,
    pub type_0: libc::c_int,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut GLPAIJ,
    pub rii: libc::c_double,
    pub stat: libc::c_int,
    pub bind: libc::c_int,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
    pub pval: libc::c_double,
    pub dval: libc::c_double,
    pub mipx: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPAIJ {
    pub row: *mut GLPROW,
    pub col: *mut GLPCOL,
    pub val: libc::c_double,
    pub r_prev: *mut GLPAIJ,
    pub r_next: *mut GLPAIJ,
    pub c_prev: *mut GLPAIJ,
    pub c_next: *mut GLPAIJ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPCOL {
    pub j: libc::c_int,
    pub name: *mut libc::c_char,
    pub node: *mut AVLNODE,
    pub kind: libc::c_int,
    pub type_0: libc::c_int,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub coef: libc::c_double,
    pub ptr: *mut GLPAIJ,
    pub sjj: libc::c_double,
    pub stat: libc::c_int,
    pub bind: libc::c_int,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
    pub pval: libc::c_double,
    pub dval: libc::c_double,
    pub mipx: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_prob {
    pub pool: *mut DMP,
    pub tree: *mut glp_tree,
    pub name: *mut libc::c_char,
    pub obj: *mut libc::c_char,
    pub dir: libc::c_int,
    pub c0: libc::c_double,
    pub m_max: libc::c_int,
    pub n_max: libc::c_int,
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub nnz: libc::c_int,
    pub row: *mut *mut GLPROW,
    pub col: *mut *mut GLPCOL,
    pub r_tree: *mut AVL,
    pub c_tree: *mut AVL,
    pub valid: libc::c_int,
    pub head: *mut libc::c_int,
    pub bfd: *mut BFD,
    pub pbs_stat: libc::c_int,
    pub dbs_stat: libc::c_int,
    pub obj_val: libc::c_double,
    pub it_cnt: libc::c_int,
    pub some: libc::c_int,
    pub ipt_stat: libc::c_int,
    pub ipt_obj: libc::c_double,
    pub mip_stat: libc::c_int,
    pub mip_obj: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csa {
    pub P: *mut glp_prob,
    pub G: *mut CFG,
    pub ind: *mut libc::c_int,
    pub nn: libc::c_int,
    pub vtoi: *mut libc::c_int,
    pub itov: *mut libc::c_int,
    pub wgt: *mut libc::c_double,
}
unsafe extern "C" fn fcmp(
    mut e1: *const libc::c_void,
    mut e2: *const libc::c_void,
) -> libc::c_int {
    let mut t1: *const term = e1 as *const term;
    let mut t2: *const term = e2 as *const term;
    if (*t1).val > (*t2).val {
        return -(1 as libc::c_int)
    } else if (*t1).val < (*t2).val {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn analyze_ineq(
    mut P: *mut glp_prob,
    mut G: *mut CFG,
    mut len: libc::c_int,
    mut ind: *mut libc::c_int,
    mut val: *mut libc::c_double,
    mut rhs: libc::c_double,
    mut t: *mut term,
) {
    let mut current_block: u64;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut kk: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut new_len: libc::c_int = 0;
    new_len = 0 as libc::c_int;
    k = 1 as libc::c_int;
    loop {
        if !(k <= len) {
            current_block = 17860125682698302841;
            break;
        }
        j = *ind.offset(k as isize);
        if (**((*P).col).offset(j as isize)).kind == 2 as libc::c_int
            && (**((*P).col).offset(j as isize)).type_0 == 4 as libc::c_int
            && (**((*P).col).offset(j as isize)).lb == 0.0f64
            && (**((*P).col).offset(j as isize)).ub == 1.0f64
        {
            new_len += 1;
            new_len;
            *ind.offset(new_len as isize) = j;
            *val.offset(new_len as isize) = *val.offset(k as isize);
        } else if *val.offset(k as isize) > 0.0f64 {
            type_0 = (**((*P).col).offset(j as isize)).type_0;
            if type_0 == 1 as libc::c_int || type_0 == 3 as libc::c_int {
                current_block = 1990015240104409014;
                break;
            } else {
                rhs -= *val.offset(k as isize) * (**((*P).col).offset(j as isize)).lb;
            }
        } else {
            type_0 = (**((*P).col).offset(j as isize)).type_0;
            if type_0 == 1 as libc::c_int || type_0 == 2 as libc::c_int {
                current_block = 1990015240104409014;
                break;
            } else {
                rhs -= *val.offset(k as isize) * (**((*P).col).offset(j as isize)).ub;
            }
        }
        k += 1;
        k;
    }
    match current_block {
        17860125682698302841 => {
            len = new_len;
            if !(len <= 1 as libc::c_int) {
                k = 1 as libc::c_int;
                while k <= len {
                    if *val.offset(k as isize) < 0.0f64 {
                        *ind.offset(k as isize) = -*ind.offset(k as isize);
                        *val.offset(k as isize) = -*val.offset(k as isize);
                        rhs += *val.offset(k as isize);
                    }
                    k += 1;
                    k;
                }
                rhs += 0.001f64 * (1.0f64 + fabs(rhs));
                p = 0 as libc::c_int;
                k = 1 as libc::c_int;
                while k <= len {
                    if p == 0 as libc::c_int
                        || *val.offset(p as isize) > *val.offset(k as isize)
                    {
                        p = k;
                    }
                    k += 1;
                    k;
                }
                q = 0 as libc::c_int;
                k = 1 as libc::c_int;
                while k <= len {
                    if k != p
                        && (q == 0 as libc::c_int
                            || *val.offset(q as isize) > *val.offset(k as isize))
                    {
                        q = k;
                    }
                    k += 1;
                    k;
                }
                (p != 0 as libc::c_int && q != 0 as libc::c_int && p != q
                    || {
                        glp_assert_(
                            b"p != 0 && q != 0 && p != q\0" as *const u8
                                as *const libc::c_char,
                            b"intopt/cfg1.c\0" as *const u8 as *const libc::c_char,
                            284 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                if *val.offset(p as isize) + *val.offset(q as isize) > rhs {
                    _glp_cfg_add_clique(G, len, ind as *const libc::c_int);
                } else {
                    p = 0 as libc::c_int;
                    k = 1 as libc::c_int;
                    while k <= len {
                        if p == 0 as libc::c_int
                            || *val.offset(p as isize) < *val.offset(k as isize)
                        {
                            p = k;
                        }
                        k += 1;
                        k;
                    }
                    q = 0 as libc::c_int;
                    k = 1 as libc::c_int;
                    while k <= len {
                        if k != p
                            && (q == 0 as libc::c_int
                                || *val.offset(q as isize) < *val.offset(k as isize))
                        {
                            q = k;
                        }
                        k += 1;
                        k;
                    }
                    (p != 0 as libc::c_int && q != 0 as libc::c_int && p != q
                        || {
                            glp_assert_(
                                b"p != 0 && q != 0 && p != q\0" as *const u8
                                    as *const libc::c_char,
                                b"intopt/cfg1.c\0" as *const u8 as *const libc::c_char,
                                303 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    if !(*val.offset(p as isize) + *val.offset(q as isize) <= rhs) {
                        (len >= 3 as libc::c_int
                            || {
                                glp_assert_(
                                    b"len >= 3\0" as *const u8 as *const libc::c_char,
                                    b"intopt/cfg1.c\0" as *const u8 as *const libc::c_char,
                                    310 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        k = 1 as libc::c_int;
                        while k <= len {
                            (*t.offset(k as isize)).ind = *ind.offset(k as isize);
                            (*t.offset(k as isize)).val = *val.offset(k as isize);
                            k += 1;
                            k;
                        }
                        qsort(
                            &mut *t.offset(1 as libc::c_int as isize) as *mut term
                                as *mut libc::c_void,
                            len as size_t,
                            ::std::mem::size_of::<term>() as libc::c_ulong,
                            Some(
                                fcmp
                                    as unsafe extern "C" fn(
                                        *const libc::c_void,
                                        *const libc::c_void,
                                    ) -> libc::c_int,
                            ),
                        );
                        k = 1 as libc::c_int;
                        while k <= len {
                            *ind.offset(k as isize) = (*t.offset(k as isize)).ind;
                            *val.offset(k as isize) = (*t.offset(k as isize)).val;
                            k += 1;
                            k;
                        }
                        (*val.offset(1 as libc::c_int as isize)
                            + *val.offset(2 as libc::c_int as isize) > rhs
                            || {
                                glp_assert_(
                                    b"val[1] + val[2] > rhs\0" as *const u8
                                        as *const libc::c_char,
                                    b"intopt/cfg1.c\0" as *const u8 as *const libc::c_char,
                                    324 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        (*val.offset((len - 1 as libc::c_int) as isize)
                            + *val.offset(len as isize) <= rhs
                            || {
                                glp_assert_(
                                    b"val[len-1] + val[len] <= rhs\0" as *const u8
                                        as *const libc::c_char,
                                    b"intopt/cfg1.c\0" as *const u8 as *const libc::c_char,
                                    325 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        p = 2 as libc::c_int;
                        while p < len {
                            if *val.offset(p as isize)
                                + *val.offset((p + 1 as libc::c_int) as isize) <= rhs
                            {
                                break;
                            }
                            p += 1;
                            p;
                        }
                        (p < len
                            || {
                                glp_assert_(
                                    b"p < len\0" as *const u8 as *const libc::c_char,
                                    b"intopt/cfg1.c\0" as *const u8 as *const libc::c_char,
                                    331 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        _glp_cfg_add_clique(G, p, ind as *const libc::c_int);
                        k = 1 as libc::c_int;
                        while k <= p {
                            kk = p;
                            while kk <= len {
                                if k != kk
                                    && *val.offset(k as isize) + *val.offset(kk as isize) > rhs
                                {
                                    let mut iii: [libc::c_int; 3] = [0; 3];
                                    iii[1 as libc::c_int as usize] = *ind.offset(k as isize);
                                    iii[2 as libc::c_int as usize] = *ind.offset(kk as isize);
                                    _glp_cfg_add_clique(
                                        G,
                                        2 as libc::c_int,
                                        iii.as_mut_ptr() as *const libc::c_int,
                                    );
                                }
                                kk += 1;
                                kk;
                            }
                            k += 1;
                            k;
                        }
                    }
                }
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn _glp_cfg_build_graph(mut P_: *mut libc::c_void) -> *mut CFG {
    let mut P: *mut glp_prob = P_ as *mut glp_prob;
    let mut m: libc::c_int = (*P).m;
    let mut n: libc::c_int = (*P).n;
    let mut G: *mut CFG = 0 as *mut CFG;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut t: *mut term = 0 as *mut term;
    G = _glp_cfg_create_graph(n, 2 as libc::c_int * glp_get_num_bin(P));
    ind = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    val = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    t = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<term>() as libc::c_ulong as libc::c_int,
    ) as *mut term;
    i = 1 as libc::c_int;
    while i <= m {
        type_0 = (**((*P).row).offset(i as isize)).type_0;
        if type_0 == 2 as libc::c_int || type_0 == 4 as libc::c_int
            || type_0 == 5 as libc::c_int
        {
            len = glp_get_mat_row(P, i, ind, val);
            k = 1 as libc::c_int;
            while k <= len {
                *val.offset(k as isize) = -*val.offset(k as isize);
                k += 1;
                k;
            }
            analyze_ineq(P, G, len, ind, val, -(**((*P).row).offset(i as isize)).lb, t);
        }
        if type_0 == 3 as libc::c_int || type_0 == 4 as libc::c_int
            || type_0 == 5 as libc::c_int
        {
            len = glp_get_mat_row(P, i, ind, val);
            analyze_ineq(P, G, len, ind, val, (**((*P).row).offset(i as isize)).ub, t);
        }
        i += 1;
        i;
    }
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
    glp_free(t as *mut libc::c_void);
    return G;
}
unsafe extern "C" fn build_subgraph(mut csa: *mut csa) {
    let mut P: *mut glp_prob = (*csa).P;
    let mut n: libc::c_int = (*P).n;
    let mut G: *mut CFG = (*csa).G;
    let mut ind: *mut libc::c_int = (*csa).ind;
    let mut pos: *mut libc::c_int = (*G).pos;
    let mut neg: *mut libc::c_int = (*G).neg;
    let mut nv: libc::c_int = (*G).nv;
    let mut ref_0: *mut libc::c_int = (*G).ref_0;
    let mut vtoi: *mut libc::c_int = (*csa).vtoi;
    let mut itov: *mut libc::c_int = (*csa).itov;
    let mut wgt: *mut libc::c_double = (*csa).wgt;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut nn: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut z: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    nn = 0 as libc::c_int;
    v = 1 as libc::c_int;
    while v <= nv {
        j = *ref_0.offset(v as isize);
        (1 as libc::c_int <= j && j <= n
            || {
                glp_assert_(
                    b"1 <= j && j <= n\0" as *const u8 as *const libc::c_char,
                    b"intopt/cfg1.c\0" as *const u8 as *const libc::c_char,
                    477 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if *pos.offset(j as isize) == v {
            z = (**((*P).col).offset(j as isize)).prim;
        } else if *neg.offset(j as isize) == v {
            z = 1.0f64 - (**((*P).col).offset(j as isize)).prim;
        } else {
            (v != v
                || {
                    glp_assert_(
                        b"v != v\0" as *const u8 as *const libc::c_char,
                        b"intopt/cfg1.c\0" as *const u8 as *const libc::c_char,
                        487 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        if z < 0.001f64 {
            *vtoi.offset(v as isize) = 0 as libc::c_int;
        } else {
            sum = z;
            len = _glp_cfg_get_adjacent(G, v, ind);
            k = 1 as libc::c_int;
            while k <= len {
                w = *ind.offset(k as isize);
                (w != v
                    || {
                        glp_assert_(
                            b"w != v\0" as *const u8 as *const libc::c_char,
                            b"intopt/cfg1.c\0" as *const u8 as *const libc::c_char,
                            501 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                j = *ref_0.offset(w as isize);
                (1 as libc::c_int <= j && j <= n
                    || {
                        glp_assert_(
                            b"1 <= j && j <= n\0" as *const u8 as *const libc::c_char,
                            b"intopt/cfg1.c\0" as *const u8 as *const libc::c_char,
                            504 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                if *pos.offset(j as isize) == w {
                    sum += (**((*P).col).offset(j as isize)).prim;
                } else if *neg.offset(j as isize) == w {
                    sum += 1.0f64 - (**((*P).col).offset(j as isize)).prim;
                } else {
                    (w != w
                        || {
                            glp_assert_(
                                b"w != w\0" as *const u8 as *const libc::c_char,
                                b"intopt/cfg1.c\0" as *const u8 as *const libc::c_char,
                                510 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
                k += 1;
                k;
            }
            if sum < 1.010f64 {
                *vtoi.offset(v as isize) = 0 as libc::c_int;
            } else {
                nn += 1;
                nn;
                *vtoi.offset(v as isize) = nn;
                *itov.offset(nn as isize) = v;
                *wgt.offset(nn as isize) = z;
            }
        }
        v += 1;
        v;
    }
    (*csa).nn = nn;
}
unsafe extern "C" fn sub_adjacent(
    mut csa: *mut csa,
    mut i: libc::c_int,
    mut adj: *mut libc::c_int,
) -> libc::c_int {
    let mut G: *mut CFG = (*csa).G;
    let mut nv: libc::c_int = (*G).nv;
    let mut ind: *mut libc::c_int = (*csa).ind;
    let mut nn: libc::c_int = (*csa).nn;
    let mut vtoi: *mut libc::c_int = (*csa).vtoi;
    let mut itov: *mut libc::c_int = (*csa).itov;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut len1: libc::c_int = 0;
    (1 as libc::c_int <= i && i <= nn
        || {
            glp_assert_(
                b"1 <= i && i <= nn\0" as *const u8 as *const libc::c_char,
                b"intopt/cfg1.c\0" as *const u8 as *const libc::c_char,
                541 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    v = *itov.offset(i as isize);
    len1 = _glp_cfg_get_adjacent(G, v, ind);
    len = 0 as libc::c_int;
    k = 1 as libc::c_int;
    while k <= len1 {
        w = *ind.offset(k as isize);
        (1 as libc::c_int <= w && w <= nv && w != v
            || {
                glp_assert_(
                    b"1 <= w && w <= nv && w != v\0" as *const u8 as *const libc::c_char,
                    b"intopt/cfg1.c\0" as *const u8 as *const libc::c_char,
                    551 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        j = *vtoi.offset(w as isize);
        if j != 0 as libc::c_int {
            (1 as libc::c_int <= j && j <= nn && j != i
                || {
                    glp_assert_(
                        b"1 <= j && j <= nn && j != i\0" as *const u8
                            as *const libc::c_char,
                        b"intopt/cfg1.c\0" as *const u8 as *const libc::c_char,
                        555 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            len += 1;
            *adj.offset(len as isize) = j;
        }
        k += 1;
        k;
    }
    return len;
}
unsafe extern "C" fn find_clique(
    mut csa: *mut csa,
    mut c_ind: *mut libc::c_int,
) -> libc::c_int {
    let mut nn: libc::c_int = (*csa).nn;
    let mut wgt: *mut libc::c_double = (*csa).wgt;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut ne: libc::c_int = 0;
    let mut nb: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut iwt: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut a: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    (nn >= 2 as libc::c_int
        || {
            glp_assert_(
                b"nn >= 2\0" as *const u8 as *const libc::c_char,
                b"intopt/cfg1.c\0" as *const u8 as *const libc::c_char,
                569 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ind = glp_alloc(
        1 as libc::c_int + nn,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    ne = nn * (nn - 1 as libc::c_int) / 2 as libc::c_int;
    nb = (ne + (8 as libc::c_int - 1 as libc::c_int)) / 8 as libc::c_int;
    a = glp_alloc(
        nb,
        ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_uchar;
    memset(a as *mut libc::c_void, 0 as libc::c_int, nb as libc::c_ulong);
    p = 1 as libc::c_int;
    while p <= nn {
        len = sub_adjacent(csa, p, ind);
        k = 1 as libc::c_int;
        while k <= len {
            q = *ind.offset(k as isize);
            (1 as libc::c_int <= q && q <= nn && q != p
                || {
                    glp_assert_(
                        b"1 <= q && q <= nn && q != p\0" as *const u8
                            as *const libc::c_char,
                        b"intopt/cfg1.c\0" as *const u8 as *const libc::c_char,
                        588 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if p > q {
                i = p;
                j = q;
            } else {
                i = q;
                j = p;
            }
            t = (i - 1 as libc::c_int) * (i - 2 as libc::c_int) / 2 as libc::c_int
                + (j - 1 as libc::c_int);
            let ref mut fresh0 = *a.offset((t / 8 as libc::c_int) as isize);
            *fresh0 = (*fresh0 as libc::c_int
                | ((1 as libc::c_int)
                    << 8 as libc::c_int - 1 as libc::c_int - t % 8 as libc::c_int)
                    as libc::c_uchar as libc::c_int) as libc::c_uchar;
            k += 1;
            k;
        }
        p += 1;
        p;
    }
    iwt = ind;
    i = 1 as libc::c_int;
    while i <= nn {
        t = (1000.0f64 * *wgt.offset(i as isize) + 0.5f64) as libc::c_int;
        if t < 0 as libc::c_int {
            t = 0 as libc::c_int;
        } else if t > 1000 as libc::c_int {
            t = 1000 as libc::c_int;
        }
        *iwt.offset(i as isize) = t;
        i += 1;
        i;
    }
    len = _glp_wclique(nn, iwt as *const libc::c_int, a as *const libc::c_uchar, c_ind);
    glp_free(ind as *mut libc::c_void);
    glp_free(a as *mut libc::c_void);
    return len;
}
unsafe extern "C" fn func(
    mut info: *mut libc::c_void,
    mut i: libc::c_int,
    mut ind: *mut libc::c_int,
) -> libc::c_int {
    let mut csa: *mut csa = info as *mut csa;
    (1 as libc::c_int <= i && i <= (*csa).nn
        || {
            glp_assert_(
                b"1 <= i && i <= csa->nn\0" as *const u8 as *const libc::c_char,
                b"intopt/cfg1.c\0" as *const u8 as *const libc::c_char,
                625 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return sub_adjacent(csa, i, ind);
}
unsafe extern "C" fn find_clique1(
    mut csa: *mut csa,
    mut c_ind: *mut libc::c_int,
) -> libc::c_int {
    let mut nn: libc::c_int = (*csa).nn;
    let mut wgt: *mut libc::c_double = (*csa).wgt;
    let mut len: libc::c_int = 0;
    (nn >= 2 as libc::c_int
        || {
            glp_assert_(
                b"nn >= 2\0" as *const u8 as *const libc::c_char,
                b"intopt/cfg1.c\0" as *const u8 as *const libc::c_char,
                635 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    len = _glp_wclique1(
        nn,
        wgt as *const libc::c_double,
        Some(
            func
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_int,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
        csa as *mut libc::c_void,
        c_ind,
    );
    return len;
}
pub unsafe extern "C" fn _glp_cfg_find_clique(
    mut P: *mut libc::c_void,
    mut G: *mut CFG,
    mut ind: *mut libc::c_int,
    mut sum_: *mut libc::c_double,
) -> libc::c_int {
    let mut nv: libc::c_int = (*G).nv;
    let mut csa: csa = csa {
        P: 0 as *mut glp_prob,
        G: 0 as *mut CFG,
        ind: 0 as *mut libc::c_int,
        nn: 0,
        vtoi: 0 as *mut libc::c_int,
        itov: 0 as *mut libc::c_int,
        wgt: 0 as *mut libc::c_double,
    };
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut sum: libc::c_double = 0.;
    csa.P = P as *mut glp_prob;
    csa.G = G;
    csa
        .ind = glp_alloc(
        1 as libc::c_int + nv,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    csa.nn = -(1 as libc::c_int);
    csa
        .vtoi = glp_alloc(
        1 as libc::c_int + nv,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    csa
        .itov = glp_alloc(
        1 as libc::c_int + nv,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    csa
        .wgt = glp_alloc(
        1 as libc::c_int + nv,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    build_subgraph(&mut csa);
    if csa.nn < 2 as libc::c_int {
        len = 0 as libc::c_int;
        sum = 0.0f64;
    } else {
        if csa.nn <= 50 as libc::c_int {
            len = find_clique(&mut csa, ind);
        } else {
            len = find_clique1(&mut csa, ind);
        }
        if len < 2 as libc::c_int {
            len = 0 as libc::c_int;
            sum = 0.0f64;
        } else {
            sum = 0.0f64;
            k = 1 as libc::c_int;
            while k <= len {
                i = *ind.offset(k as isize);
                (1 as libc::c_int <= i && i <= csa.nn
                    || {
                        glp_assert_(
                            b"1 <= i && i <= csa.nn\0" as *const u8
                                as *const libc::c_char,
                            b"intopt/cfg1.c\0" as *const u8 as *const libc::c_char,
                            687 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                sum += *(csa.wgt).offset(i as isize);
                *ind.offset(k as isize) = *(csa.itov).offset(i as isize);
                k += 1;
                k;
            }
        }
    }
    glp_free(csa.ind as *mut libc::c_void);
    glp_free(csa.vtoi as *mut libc::c_void);
    glp_free(csa.itov as *mut libc::c_void);
    glp_free(csa.wgt as *mut libc::c_void);
    *sum_ = sum;
    return len;
}
