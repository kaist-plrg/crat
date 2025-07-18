use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_eval_tab_row(
        P: *mut glp_prob,
        k: libc::c_int,
        ind: *mut libc::c_int,
        val: *mut libc::c_double,
    ) -> libc::c_int;
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
pub unsafe extern "C" fn glp_gmi_cut(
    mut P: *mut glp_prob,
    mut j: libc::c_int,
    mut ind: *mut libc::c_int,
    mut val: *mut libc::c_double,
    mut phi: *mut libc::c_double,
) -> libc::c_int {
    let mut current_block: u64;
    let mut m: libc::c_int = (*P).m;
    let mut n: libc::c_int = (*P).n;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut kind: libc::c_int = 0;
    let mut stat: libc::c_int = 0;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    let mut alfa: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut ksi: libc::c_double = 0.;
    let mut phi1: libc::c_double = 0.;
    let mut rhs: libc::c_double = 0.;
    if !((*P).m == 0 as libc::c_int || (*P).valid != 0) {
        return -(1 as libc::c_int);
    }
    if !((*P).pbs_stat == 2 as libc::c_int && (*P).dbs_stat == 2 as libc::c_int) {
        return -(2 as libc::c_int);
    }
    if !(1 as libc::c_int <= j && j <= n) {
        return -(3 as libc::c_int);
    }
    col = *((*P).col).offset(j as isize);
    if (*col).kind != 2 as libc::c_int {
        return -(4 as libc::c_int);
    }
    if (*col).type_0 == 5 as libc::c_int || (*col).stat != 1 as libc::c_int {
        return -(5 as libc::c_int);
    }
    if fabs((*col).prim - floor((*col).prim + 0.5f64)) < 0.001f64 {
        return -(6 as libc::c_int);
    }
    len = glp_eval_tab_row(P, m + j, ind, val);
    beta = (**((*P).col).offset(j as isize)).prim;
    k = 1 as libc::c_int;
    while k <= m + n {
        *phi.offset(k as isize) = 0.0f64;
        k += 1;
        k;
    }
    rhs = beta - floor(beta);
    j = 1 as libc::c_int;
    while j <= len {
        k = *ind.offset(j as isize);
        (1 as libc::c_int <= k && k <= m + n
            || {
                glp_assert_(
                    b"1 <= k && k <= m+n\0" as *const u8 as *const libc::c_char,
                    b"intopt/gmicut.c\0" as *const u8 as *const libc::c_char,
                    150 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if k <= m {
            row = *((*P).row).offset(k as isize);
            kind = 1 as libc::c_int;
            lb = (*row).lb;
            ub = (*row).ub;
            stat = (*row).stat;
        } else {
            col = *((*P).col).offset((k - m) as isize);
            kind = (*col).kind;
            lb = (*col).lb;
            ub = (*col).ub;
            stat = (*col).stat;
        }
        (stat != 1 as libc::c_int
            || {
                glp_assert_(
                    b"stat != GLP_BS\0" as *const u8 as *const libc::c_char,
                    b"intopt/gmicut.c\0" as *const u8 as *const libc::c_char,
                    170 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        ksi = *val.offset(j as isize);
        if fabs(ksi) > 1e+05f64 {
            return -(7 as libc::c_int);
        }
        if !(fabs(ksi) < 1e-10f64) {
            match stat {
                4 => return -(8 as libc::c_int),
                2 => {
                    alfa = -ksi;
                    current_block = 12199444798915819164;
                }
                3 => {
                    alfa = ksi;
                    current_block = 12199444798915819164;
                }
                5 => {
                    current_block = 11584701595673473500;
                }
                _ => {
                    (stat != stat
                        || {
                            glp_assert_(
                                b"stat != stat\0" as *const u8 as *const libc::c_char,
                                b"intopt/gmicut.c\0" as *const u8 as *const libc::c_char,
                                197 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    current_block = 12199444798915819164;
                }
            }
            match current_block {
                11584701595673473500 => {}
                _ => {
                    match kind {
                        2 => {
                            if fabs(alfa - floor(alfa + 0.5f64)) < 1e-10f64 {
                                current_block = 11584701595673473500;
                            } else {
                                if alfa - floor(alfa) <= beta - floor(beta) {
                                    phi1 = alfa - floor(alfa);
                                } else {
                                    phi1 = (beta - floor(beta))
                                        / (1.0f64 - (beta - floor(beta)))
                                        * (1.0f64 - (alfa - floor(alfa)));
                                }
                                current_block = 7420279277351916581;
                            }
                        }
                        1 => {
                            if alfa >= 0.0f64 {
                                phi1 = alfa;
                            } else {
                                phi1 = (beta - floor(beta))
                                    / (1.0f64 - (beta - floor(beta))) * -alfa;
                            }
                            current_block = 7420279277351916581;
                        }
                        _ => {
                            (kind != kind
                                || {
                                    glp_assert_(
                                        b"kind != kind\0" as *const u8 as *const libc::c_char,
                                        b"intopt/gmicut.c\0" as *const u8 as *const libc::c_char,
                                        220 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                            current_block = 7420279277351916581;
                        }
                    }
                    match current_block {
                        11584701595673473500 => {}
                        _ => {
                            match stat {
                                2 => {
                                    *phi.offset(k as isize) = phi1;
                                    rhs += phi1 * lb;
                                }
                                3 => {
                                    *phi.offset(k as isize) = -phi1;
                                    rhs -= phi1 * ub;
                                }
                                _ => {
                                    (stat != stat
                                        || {
                                            glp_assert_(
                                                b"stat != stat\0" as *const u8 as *const libc::c_char,
                                                b"intopt/gmicut.c\0" as *const u8 as *const libc::c_char,
                                                236 as libc::c_int,
                                            );
                                            1 as libc::c_int != 0
                                        }) as libc::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
        j += 1;
        j;
    }
    i = 1 as libc::c_int;
    while i <= m {
        if !(fabs(*phi.offset(i as isize)) < 1e-10f64) {
            row = *((*P).row).offset(i as isize);
            ((*row).type_0 != 5 as libc::c_int
                || {
                    glp_assert_(
                        b"row->type != GLP_FX\0" as *const u8 as *const libc::c_char,
                        b"intopt/gmicut.c\0" as *const u8 as *const libc::c_char,
                        252 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            aij = (*row).ptr;
            while !aij.is_null() {
                *phi.offset((m + (*(*aij).col).j) as isize)
                    += *phi.offset(i as isize) * (*aij).val;
                aij = (*aij).r_next;
            }
        }
        i += 1;
        i;
    }
    len = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while j <= n {
        if !(fabs(*phi.offset((m + j) as isize)) < 1e-10f64) {
            col = *((*P).col).offset(j as isize);
            if (*col).type_0 == 5 as libc::c_int {
                rhs -= *phi.offset((m + j) as isize) * (*col).lb;
            } else {
                len += 1;
                len;
                *ind.offset(len as isize) = j;
                *val.offset(len as isize) = *phi.offset((m + j) as isize);
            }
        }
        j += 1;
        j;
    }
    if fabs(rhs) < 1e-12f64 {
        rhs = 0.0f64;
    }
    *ind.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    *val.offset(0 as libc::c_int as isize) = rhs;
    return len;
}
