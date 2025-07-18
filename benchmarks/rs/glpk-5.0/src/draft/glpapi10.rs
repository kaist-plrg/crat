use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
}
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
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
pub unsafe extern "C" fn glp_check_kkt(
    mut P: *mut glp_prob,
    mut sol: libc::c_int,
    mut cond: libc::c_int,
    mut _ae_max: *mut libc::c_double,
    mut _ae_ind: *mut libc::c_int,
    mut _re_max: *mut libc::c_double,
    mut _re_ind: *mut libc::c_int,
) {
    let mut m: libc::c_int = (*P).m;
    let mut n: libc::c_int = (*P).n;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ae_ind: libc::c_int = 0;
    let mut re_ind: libc::c_int = 0;
    let mut e: libc::c_double = 0.;
    let mut sp: libc::c_double = 0.;
    let mut sn: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut ae_max: libc::c_double = 0.;
    let mut re_max: libc::c_double = 0.;
    if !(sol == 1 as libc::c_int || sol == 2 as libc::c_int || sol == 3 as libc::c_int) {
        (glp_error_(
            b"draft/glpapi10.c\0" as *const u8 as *const libc::c_char,
            36 as libc::c_int,
        ))
            .unwrap()(
            b"glp_check_kkt: sol = %d; invalid solution indicator\n\0" as *const u8
                as *const libc::c_char,
            sol,
        );
    }
    if !(cond == 1 as libc::c_int || cond == 2 as libc::c_int || cond == 3 as libc::c_int
        || cond == 4 as libc::c_int || cond == 5 as libc::c_int)
    {
        (glp_error_(
            b"draft/glpapi10.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
        ))
            .unwrap()(
            b"glp_check_kkt: cond = %d; invalid condition indicator \n\0" as *const u8
                as *const libc::c_char,
            cond,
        );
    }
    re_max = 0.0f64;
    ae_max = re_max;
    re_ind = 0 as libc::c_int;
    ae_ind = re_ind;
    if cond == 1 as libc::c_int {
        i = 1 as libc::c_int;
        while i <= m {
            row = *((*P).row).offset(i as isize);
            sn = 0.0f64;
            sp = sn;
            if sol == 1 as libc::c_int {
                t = (*row).prim;
            } else if sol == 2 as libc::c_int {
                t = (*row).pval;
            } else if sol == 3 as libc::c_int {
                t = (*row).mipx;
            } else {
                (sol != sol
                    || {
                        glp_assert_(
                            b"sol != sol\0" as *const u8 as *const libc::c_char,
                            b"draft/glpapi10.c\0" as *const u8 as *const libc::c_char,
                            58 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            if t >= 0.0f64 {
                sp += t;
            } else {
                sn -= t;
            }
            aij = (*row).ptr;
            while !aij.is_null() {
                col = (*aij).col;
                if sol == 1 as libc::c_int {
                    t = -(*aij).val * (*col).prim;
                } else if sol == 2 as libc::c_int {
                    t = -(*aij).val * (*col).pval;
                } else if sol == 3 as libc::c_int {
                    t = -(*aij).val * (*col).mipx;
                } else {
                    (sol != sol
                        || {
                            glp_assert_(
                                b"sol != sol\0" as *const u8 as *const libc::c_char,
                                b"draft/glpapi10.c\0" as *const u8 as *const libc::c_char,
                                70 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
                if t >= 0.0f64 {
                    sp += t;
                } else {
                    sn -= t;
                }
                aij = (*aij).r_next;
            }
            e = fabs(sp - sn);
            if ae_max < e {
                ae_max = e;
                ae_ind = i;
            }
            e /= 1.0f64 + sp + sn;
            if re_max < e {
                re_max = e;
                re_ind = i;
            }
            i += 1;
            i;
        }
    } else if cond == 2 as libc::c_int {
        i = 1 as libc::c_int;
        while i <= m {
            row = *((*P).row).offset(i as isize);
            if sol == 1 as libc::c_int {
                t = (*row).prim;
            } else if sol == 2 as libc::c_int {
                t = (*row).pval;
            } else if sol == 3 as libc::c_int {
                t = (*row).mipx;
            } else {
                (sol != sol
                    || {
                        glp_assert_(
                            b"sol != sol\0" as *const u8 as *const libc::c_char,
                            b"draft/glpapi10.c\0" as *const u8 as *const libc::c_char,
                            95 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            if (*row).type_0 == 2 as libc::c_int || (*row).type_0 == 4 as libc::c_int
                || (*row).type_0 == 5 as libc::c_int
            {
                if t < (*row).lb {
                    e = (*row).lb - t;
                    if ae_max < e {
                        ae_max = e;
                        ae_ind = i;
                    }
                    e /= 1.0f64 + fabs((*row).lb);
                    if re_max < e {
                        re_max = e;
                        re_ind = i;
                    }
                }
            }
            if (*row).type_0 == 3 as libc::c_int || (*row).type_0 == 4 as libc::c_int
                || (*row).type_0 == 5 as libc::c_int
            {
                if t > (*row).ub {
                    e = t - (*row).ub;
                    if ae_max < e {
                        ae_max = e;
                        ae_ind = i;
                    }
                    e /= 1.0f64 + fabs((*row).ub);
                    if re_max < e {
                        re_max = e;
                        re_ind = i;
                    }
                }
            }
            i += 1;
            i;
        }
        j = 1 as libc::c_int;
        while j <= n {
            col = *((*P).col).offset(j as isize);
            if sol == 1 as libc::c_int {
                t = (*col).prim;
            } else if sol == 2 as libc::c_int {
                t = (*col).pval;
            } else if sol == 3 as libc::c_int {
                t = (*col).mipx;
            } else {
                (sol != sol
                    || {
                        glp_assert_(
                            b"sol != sol\0" as *const u8 as *const libc::c_char,
                            b"draft/glpapi10.c\0" as *const u8 as *const libc::c_char,
                            136 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            if (*col).type_0 == 2 as libc::c_int || (*col).type_0 == 4 as libc::c_int
                || (*col).type_0 == 5 as libc::c_int
            {
                if t < (*col).lb {
                    e = (*col).lb - t;
                    if ae_max < e {
                        ae_max = e;
                        ae_ind = m + j;
                    }
                    e /= 1.0f64 + fabs((*col).lb);
                    if re_max < e {
                        re_max = e;
                        re_ind = m + j;
                    }
                }
            }
            if (*col).type_0 == 3 as libc::c_int || (*col).type_0 == 4 as libc::c_int
                || (*col).type_0 == 5 as libc::c_int
            {
                if t > (*col).ub {
                    e = t - (*col).ub;
                    if ae_max < e {
                        ae_max = e;
                        ae_ind = m + j;
                    }
                    e /= 1.0f64 + fabs((*col).ub);
                    if re_max < e {
                        re_max = e;
                        re_ind = m + j;
                    }
                }
            }
            j += 1;
            j;
        }
    } else if cond == 3 as libc::c_int {
        j = 1 as libc::c_int;
        while j <= n {
            col = *((*P).col).offset(j as isize);
            sn = 0.0f64;
            sp = sn;
            if sol == 1 as libc::c_int {
                t = (*col).dual - (*col).coef;
            } else if sol == 2 as libc::c_int {
                t = (*col).dval - (*col).coef;
            } else {
                (sol != sol
                    || {
                        glp_assert_(
                            b"sol != sol\0" as *const u8 as *const libc::c_char,
                            b"draft/glpapi10.c\0" as *const u8 as *const libc::c_char,
                            178 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            if t >= 0.0f64 {
                sp += t;
            } else {
                sn -= t;
            }
            aij = (*col).ptr;
            while !aij.is_null() {
                row = (*aij).row;
                if sol == 1 as libc::c_int {
                    t = (*aij).val * (*row).dual;
                } else if sol == 2 as libc::c_int {
                    t = (*aij).val * (*row).dval;
                } else {
                    (sol != sol
                        || {
                            glp_assert_(
                                b"sol != sol\0" as *const u8 as *const libc::c_char,
                                b"draft/glpapi10.c\0" as *const u8 as *const libc::c_char,
                                188 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
                if t >= 0.0f64 {
                    sp += t;
                } else {
                    sn -= t;
                }
                aij = (*aij).c_next;
            }
            e = fabs(sp - sn);
            if ae_max < e {
                ae_max = e;
                ae_ind = m + j;
            }
            e /= 1.0f64 + sp + sn;
            if re_max < e {
                re_max = e;
                re_ind = m + j;
            }
            j += 1;
            j;
        }
    } else if cond == 4 as libc::c_int {
        i = 1 as libc::c_int;
        while i <= m {
            row = *((*P).row).offset(i as isize);
            if sol == 1 as libc::c_int {
                t = (*row).dual;
            } else if sol == 2 as libc::c_int {
                t = (*row).dval;
            } else {
                (sol != sol
                    || {
                        glp_assert_(
                            b"sol != sol\0" as *const u8 as *const libc::c_char,
                            b"draft/glpapi10.c\0" as *const u8 as *const libc::c_char,
                            211 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            if (*P).dir == 1 as libc::c_int {
                t = t;
            } else if (*P).dir == 2 as libc::c_int {
                t = -t;
            } else {
                (P != P
                    || {
                        glp_assert_(
                            b"P != P\0" as *const u8 as *const libc::c_char,
                            b"draft/glpapi10.c\0" as *const u8 as *const libc::c_char,
                            218 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            if (*row).type_0 == 1 as libc::c_int || (*row).type_0 == 2 as libc::c_int {
                if t < 0.0f64 {
                    e = -t;
                    if ae_max < e {
                        re_max = e;
                        ae_max = re_max;
                        re_ind = i;
                        ae_ind = re_ind;
                    }
                }
            }
            if (*row).type_0 == 1 as libc::c_int || (*row).type_0 == 3 as libc::c_int {
                if t > 0.0f64 {
                    e = t;
                    if ae_max < e {
                        re_max = e;
                        ae_max = re_max;
                        re_ind = i;
                        ae_ind = re_ind;
                    }
                }
            }
            i += 1;
            i;
        }
        j = 1 as libc::c_int;
        while j <= n {
            col = *((*P).col).offset(j as isize);
            if sol == 1 as libc::c_int {
                t = (*col).dual;
            } else if sol == 2 as libc::c_int {
                t = (*col).dval;
            } else {
                (sol != sol
                    || {
                        glp_assert_(
                            b"sol != sol\0" as *const u8 as *const libc::c_char,
                            b"draft/glpapi10.c\0" as *const u8 as *const libc::c_char,
                            257 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            if (*P).dir == 1 as libc::c_int {
                t = t;
            } else if (*P).dir == 2 as libc::c_int {
                t = -t;
            } else {
                (P != P
                    || {
                        glp_assert_(
                            b"P != P\0" as *const u8 as *const libc::c_char,
                            b"draft/glpapi10.c\0" as *const u8 as *const libc::c_char,
                            264 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            if (*col).type_0 == 1 as libc::c_int || (*col).type_0 == 2 as libc::c_int {
                if t < 0.0f64 {
                    e = -t;
                    if ae_max < e {
                        re_max = e;
                        ae_max = re_max;
                        re_ind = m + j;
                        ae_ind = re_ind;
                    }
                }
            }
            if (*col).type_0 == 1 as libc::c_int || (*col).type_0 == 3 as libc::c_int {
                if t > 0.0f64 {
                    e = t;
                    if ae_max < e {
                        re_max = e;
                        ae_max = re_max;
                        re_ind = m + j;
                        ae_ind = re_ind;
                    }
                }
            }
            j += 1;
            j;
        }
    } else {
        (cond != cond
            || {
                glp_assert_(
                    b"cond != cond\0" as *const u8 as *const libc::c_char,
                    b"draft/glpapi10.c\0" as *const u8 as *const libc::c_char,
                    294 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    if !_ae_max.is_null() {
        *_ae_max = ae_max;
    }
    if !_ae_ind.is_null() {
        *_ae_ind = ae_ind;
    }
    if !_re_max.is_null() {
        *_re_max = re_max;
    }
    if !_re_ind.is_null() {
        *_re_ind = re_ind;
    }
}
