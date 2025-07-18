use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
pub unsafe extern "C" fn _glp_spx_init_lp(
    mut lp: *mut SPXLP,
    mut P: *mut glp_prob,
    mut excl: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut nnz: libc::c_int = 0;
    m = (*P).m;
    (m > 0 as libc::c_int
        || {
            glp_assert_(
                b"m > 0\0" as *const u8 as *const libc::c_char,
                b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                40 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    n = 0 as libc::c_int;
    nnz = (*P).nnz;
    ((*P).valid != 0
        || {
            glp_assert_(
                b"P->valid\0" as *const u8 as *const libc::c_char,
                b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                43 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    i = 1 as libc::c_int;
    while i <= m {
        let mut row: *mut GLPROW = *((*P).row).offset(i as isize);
        if !(excl != 0 && (*row).stat == 5 as libc::c_int) {
            n += 1;
            n;
            nnz += 1;
            nnz;
        }
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= (*P).n {
        let mut col: *mut GLPCOL = *((*P).col).offset(j as isize);
        if excl != 0 && (*col).stat == 5 as libc::c_int {
            let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
            aij = (*col).ptr;
            while !aij.is_null() {
                nnz -= 1;
                nnz;
                aij = (*aij).c_next;
            }
        } else {
            n += 1;
            n;
        }
        j += 1;
        j;
    }
    memset(
        lp as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<SPXLP>() as libc::c_ulong,
    );
    (*lp).m = m;
    (n > 0 as libc::c_int
        || {
            glp_assert_(
                b"n > 0\0" as *const u8 as *const libc::c_char,
                b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                74 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*lp).n = n;
    (*lp).nnz = nnz;
}
pub unsafe extern "C" fn _glp_spx_alloc_lp(mut lp: *mut SPXLP) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut nnz: libc::c_int = (*lp).nnz;
    (*lp)
        .A_ptr = glp_alloc(
        1 as libc::c_int + n + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*lp)
        .A_ind = glp_alloc(
        1 as libc::c_int + nnz,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*lp)
        .A_val = glp_alloc(
        1 as libc::c_int + nnz,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*lp)
        .b = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*lp)
        .c = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*lp)
        .l = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*lp)
        .u = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*lp)
        .head = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*lp)
        .flag = glp_alloc(
        1 as libc::c_int + n - m,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
}
pub unsafe extern "C" fn _glp_spx_build_lp(
    mut lp: *mut SPXLP,
    mut P: *mut glp_prob,
    mut excl: libc::c_int,
    mut shift: libc::c_int,
    mut map: *mut libc::c_int,
) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut nnz: libc::c_int = (*lp).nnz;
    let mut A_ptr: *mut libc::c_int = (*lp).A_ptr;
    let mut A_ind: *mut libc::c_int = (*lp).A_ind;
    let mut A_val: *mut libc::c_double = (*lp).A_val;
    let mut b: *mut libc::c_double = (*lp).b;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut kk: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut dir: libc::c_double = 0.;
    let mut delta: libc::c_double = 0.;
    match (*P).dir {
        1 => {
            dir = 1.0f64;
        }
        2 => {
            dir = -1.0f64;
        }
        _ => {
            (P != P
                || {
                    glp_assert_(
                        b"P != P\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                        164 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    *c.offset(0 as libc::c_int as isize) = dir * (*P).c0;
    k = 0 as libc::c_int;
    ptr = 1 as libc::c_int;
    ((*P).m == m
        || {
            glp_assert_(
                b"P->m == m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                171 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    i = 1 as libc::c_int;
    while i <= m {
        let mut row: *mut GLPROW = *((*P).row).offset(i as isize);
        if excl != 0 && (*row).stat == 5 as libc::c_int {
            ((*row).type_0 == 5 as libc::c_int
                || {
                    glp_assert_(
                        b"row->type == GLP_FX\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                        177 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            *map.offset(i as isize) = 0 as libc::c_int;
            *b.offset(i as isize) = -(*row).lb * (*row).rii;
        } else {
            k += 1;
            *map.offset(i as isize) = k;
            *A_ptr.offset(k as isize) = ptr;
            *A_ind.offset(ptr as isize) = i;
            *A_val.offset(ptr as isize) = 1.0f64;
            ptr += 1;
            ptr;
            let ref mut fresh0 = *c.offset(k as isize);
            *fresh0 = 0.0f64;
            *b.offset(i as isize) = *fresh0;
            match (*row).type_0 {
                1 => {
                    *l.offset(k as isize) = -1.7976931348623157e+308f64;
                    *u.offset(k as isize) = 1.7976931348623157e+308f64;
                }
                2 => {
                    *l.offset(k as isize) = (*row).lb * (*row).rii;
                    *u.offset(k as isize) = 1.7976931348623157e+308f64;
                }
                3 => {
                    *l.offset(k as isize) = -1.7976931348623157e+308f64;
                    *u.offset(k as isize) = (*row).ub * (*row).rii;
                }
                4 => {
                    *l.offset(k as isize) = (*row).lb * (*row).rii;
                    *u.offset(k as isize) = (*row).ub * (*row).rii;
                    (*l.offset(k as isize) != *u.offset(k as isize)
                        || {
                            glp_assert_(
                                b"l[k] != u[k]\0" as *const u8 as *const libc::c_char,
                                b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                                206 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
                5 => {
                    let ref mut fresh1 = *u.offset(k as isize);
                    *fresh1 = (*row).lb * (*row).rii;
                    *l.offset(k as isize) = *fresh1;
                }
                _ => {
                    (row != row
                        || {
                            glp_assert_(
                                b"row != row\0" as *const u8 as *const libc::c_char,
                                b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                                212 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
            }
        }
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= (*P).n {
        let mut col: *mut GLPCOL = *((*P).col).offset(j as isize);
        let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
        if excl != 0 && (*col).stat == 5 as libc::c_int {
            ((*col).type_0 == 5 as libc::c_int
                || {
                    glp_assert_(
                        b"col->type == GLP_FX\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                        223 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            *map.offset((m + j) as isize) = 0 as libc::c_int;
            if (*col).lb != 0.0f64 {
                aij = (*col).ptr;
                while !aij.is_null() {
                    *b.offset((*(*aij).row).i as isize)
                        += (*(*aij).row).rii * (*aij).val * (*col).lb;
                    aij = (*aij).c_next;
                }
                *c.offset(0 as libc::c_int as isize) += dir * (*col).coef * (*col).lb;
            }
        } else {
            k += 1;
            *map.offset((m + j) as isize) = k;
            *A_ptr.offset(k as isize) = ptr;
            aij = (*col).ptr;
            while !aij.is_null() {
                *A_ind.offset(ptr as isize) = (*(*aij).row).i;
                *A_val
                    .offset(ptr as isize) = -(*(*aij).row).rii * (*aij).val * (*col).sjj;
                ptr += 1;
                ptr;
                aij = (*aij).c_next;
            }
            *c.offset(k as isize) = dir * (*col).coef * (*col).sjj;
            match (*col).type_0 {
                1 => {
                    *l.offset(k as isize) = -1.7976931348623157e+308f64;
                    *u.offset(k as isize) = 1.7976931348623157e+308f64;
                }
                2 => {
                    *l.offset(k as isize) = (*col).lb / (*col).sjj;
                    *u.offset(k as isize) = 1.7976931348623157e+308f64;
                }
                3 => {
                    *l.offset(k as isize) = -1.7976931348623157e+308f64;
                    *u.offset(k as isize) = (*col).ub / (*col).sjj;
                }
                4 => {
                    *l.offset(k as isize) = (*col).lb / (*col).sjj;
                    *u.offset(k as isize) = (*col).ub / (*col).sjj;
                    (*l.offset(k as isize) != *u.offset(k as isize)
                        || {
                            glp_assert_(
                                b"l[k] != u[k]\0" as *const u8 as *const libc::c_char,
                                b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                                259 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
                5 => {
                    let ref mut fresh2 = *u.offset(k as isize);
                    *fresh2 = (*col).lb / (*col).sjj;
                    *l.offset(k as isize) = *fresh2;
                }
                _ => {
                    (col != col
                        || {
                            glp_assert_(
                                b"col != col\0" as *const u8 as *const libc::c_char,
                                b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                                265 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
            }
        }
        j += 1;
        j;
    }
    (k == n
        || {
            glp_assert_(
                b"k == n\0" as *const u8 as *const libc::c_char,
                b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                269 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (ptr == nnz + 1 as libc::c_int
        || {
            glp_assert_(
                b"ptr == nnz+1\0" as *const u8 as *const libc::c_char,
                b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                270 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    *A_ptr.offset((n + 1 as libc::c_int) as isize) = ptr;
    if shift != 0 {
        kk = 1 as libc::c_int;
        while kk <= m + (*P).n {
            k = *map.offset(kk as isize);
            if !(k == 0 as libc::c_int) {
                if *l.offset(k as isize) == -1.7976931348623157e+308f64
                    && *u.offset(k as isize) == 1.7976931348623157e+308f64
                {
                    delta = 0.0f64;
                } else if *l.offset(k as isize) != -1.7976931348623157e+308f64
                    && *u.offset(k as isize) == 1.7976931348623157e+308f64
                {
                    delta = *l.offset(k as isize);
                    *l.offset(k as isize) = 0.0f64;
                } else if *l.offset(k as isize) == -1.7976931348623157e+308f64
                    && *u.offset(k as isize) != 1.7976931348623157e+308f64
                {
                    *map.offset(kk as isize) = -k;
                    delta = *u.offset(k as isize);
                    *u.offset(k as isize) = 0.0f64;
                } else if *l.offset(k as isize) != *u.offset(k as isize) {
                    if fabs(*l.offset(k as isize)) <= fabs(*u.offset(k as isize)) {
                        delta = *l.offset(k as isize);
                        *l.offset(k as isize) = 0.0f64;
                        *u.offset(k as isize) -= delta;
                    } else {
                        *map.offset(kk as isize) = -k;
                        delta = *u.offset(k as isize);
                        *l.offset(k as isize) -= delta;
                        *u.offset(k as isize) = 0.0f64;
                    }
                    (*l.offset(k as isize) != *u.offset(k as isize)
                        || {
                            glp_assert_(
                                b"l[k] != u[k]\0" as *const u8 as *const libc::c_char,
                                b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                                309 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                } else {
                    delta = *l.offset(k as isize);
                    let ref mut fresh3 = *u.offset(k as isize);
                    *fresh3 = 0.0f64;
                    *l.offset(k as isize) = *fresh3;
                }
                if delta != 0.0f64 {
                    ptr = *A_ptr.offset(k as isize);
                    end = *A_ptr.offset((k + 1 as libc::c_int) as isize);
                    while ptr < end {
                        *b.offset(*A_ind.offset(ptr as isize) as isize)
                            -= *A_val.offset(ptr as isize) * delta;
                        ptr += 1;
                        ptr;
                    }
                    *c.offset(0 as libc::c_int as isize)
                        += *c.offset(k as isize) * delta;
                }
            }
            kk += 1;
            kk;
        }
    }
}
pub unsafe extern "C" fn _glp_spx_build_basis(
    mut lp: *mut SPXLP,
    mut P: *mut glp_prob,
    mut map: *const libc::c_int,
) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut flag: *mut libc::c_char = (*lp).flag;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    ((*P).m == m
        || {
            glp_assert_(
                b"P->m == m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                349 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*P).valid != 0
        || {
            glp_assert_(
                b"P->valid\0" as *const u8 as *const libc::c_char,
                b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                350 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    memset(
        &mut *head.offset(1 as libc::c_int as isize) as *mut libc::c_int
            as *mut libc::c_void,
        0 as libc::c_int,
        (m as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    jj = 0 as libc::c_int;
    ((*P).m == m
        || {
            glp_assert_(
                b"P->m == m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                355 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    i = 1 as libc::c_int;
    while i <= m {
        let mut row: *mut GLPROW = *((*P).row).offset(i as isize);
        k = *map.offset(i as isize);
        if k < 0 as libc::c_int {
            k = -k;
        }
        if !(k == 0 as libc::c_int) {
            (1 as libc::c_int <= k && k <= n
                || {
                    glp_assert_(
                        b"1 <= k && k <= n\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                        365 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if (*row).stat == 1 as libc::c_int {
                ii = (*row).bind;
                (1 as libc::c_int <= ii && ii <= m
                    || {
                        glp_assert_(
                            b"1 <= ii && ii <= m\0" as *const u8 as *const libc::c_char,
                            b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                            369 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*head.offset(ii as isize) == 0 as libc::c_int
                    || {
                        glp_assert_(
                            b"head[ii] == 0\0" as *const u8 as *const libc::c_char,
                            b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                            370 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                *head.offset(ii as isize) = k;
            } else {
                jj += 1;
                jj;
                *head.offset((m + jj) as isize) = k;
                *flag
                    .offset(
                        jj as isize,
                    ) = ((*row).stat == 3 as libc::c_int) as libc::c_int as libc::c_char;
            }
        }
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= (*P).n {
        let mut col: *mut GLPCOL = *((*P).col).offset(j as isize);
        k = *map.offset((m + j) as isize);
        if k < 0 as libc::c_int {
            k = -k;
        }
        if !(k == 0 as libc::c_int) {
            (1 as libc::c_int <= k && k <= n
                || {
                    glp_assert_(
                        b"1 <= k && k <= n\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                        390 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if (*col).stat == 1 as libc::c_int {
                ii = (*col).bind;
                (1 as libc::c_int <= ii && ii <= m
                    || {
                        glp_assert_(
                            b"1 <= ii && ii <= m\0" as *const u8 as *const libc::c_char,
                            b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                            394 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*head.offset(ii as isize) == 0 as libc::c_int
                    || {
                        glp_assert_(
                            b"head[ii] == 0\0" as *const u8 as *const libc::c_char,
                            b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                            395 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                *head.offset(ii as isize) = k;
            } else {
                jj += 1;
                jj;
                *head.offset((m + jj) as isize) = k;
                *flag
                    .offset(
                        jj as isize,
                    ) = ((*col).stat == 3 as libc::c_int) as libc::c_int as libc::c_char;
            }
        }
        j += 1;
        j;
    }
    (m + jj == n
        || {
            glp_assert_(
                b"m+jj == n\0" as *const u8 as *const libc::c_char,
                b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                405 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*lp).valid = 1 as libc::c_int;
    (*lp).bfd = (*P).bfd;
    (*P).valid = 0 as libc::c_int;
    (*P).bfd = 0 as *mut BFD;
}
pub unsafe extern "C" fn _glp_spx_store_basis(
    mut lp: *mut SPXLP,
    mut P: *mut glp_prob,
    mut map: *const libc::c_int,
    mut daeh: *mut libc::c_int,
) {
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut head: *mut libc::c_int = (*lp).head;
    let mut flag: *mut libc::c_char = (*lp).flag;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut kk: libc::c_int = 0;
    kk = 1 as libc::c_int;
    while kk <= n {
        *daeh.offset(*head.offset(kk as isize) as isize) = kk;
        kk += 1;
        kk;
    }
    ((*P).m == m
        || {
            glp_assert_(
                b"P->m == m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                441 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    i = 1 as libc::c_int;
    while i <= m {
        let mut row: *mut GLPROW = *((*P).row).offset(i as isize);
        k = *map.offset(i as isize);
        if k < 0 as libc::c_int {
            k = -k;
        }
        if k == 0 as libc::c_int {
            ((*row).type_0 == 5 as libc::c_int
                || {
                    glp_assert_(
                        b"row->type == GLP_FX\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                        448 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*row).stat = 5 as libc::c_int;
            (*row).bind = 0 as libc::c_int;
        } else {
            kk = *daeh.offset(k as isize);
            if kk <= m {
                *((*P).head).offset(kk as isize) = i;
                (*row).stat = 1 as libc::c_int;
                (*row).bind = kk;
            } else {
                match (*row).type_0 {
                    1 => {
                        (*row).stat = 4 as libc::c_int;
                    }
                    2 => {
                        (*row).stat = 2 as libc::c_int;
                    }
                    3 => {
                        (*row).stat = 3 as libc::c_int;
                    }
                    4 => {
                        (*row)
                            .stat = if *flag.offset((kk - m) as isize) as libc::c_int
                            != 0
                        {
                            3 as libc::c_int
                        } else {
                            2 as libc::c_int
                        };
                    }
                    5 => {
                        (*row).stat = 5 as libc::c_int;
                    }
                    _ => {
                        (row != row
                            || {
                                glp_assert_(
                                    b"row != row\0" as *const u8 as *const libc::c_char,
                                    b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                                    480 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                    }
                }
                (*row).bind = 0 as libc::c_int;
            }
        }
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= (*P).n {
        let mut col: *mut GLPCOL = *((*P).col).offset(j as isize);
        k = *map.offset((m + j) as isize);
        if k < 0 as libc::c_int {
            k = -k;
        }
        if k == 0 as libc::c_int {
            ((*col).type_0 == 5 as libc::c_int
                || {
                    glp_assert_(
                        b"col->type == GLP_FX\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                        493 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*col).stat = 5 as libc::c_int;
            (*col).bind = 0 as libc::c_int;
        } else {
            kk = *daeh.offset(k as isize);
            if kk <= m {
                *((*P).head).offset(kk as isize) = m + j;
                (*col).stat = 1 as libc::c_int;
                (*col).bind = kk;
            } else {
                match (*col).type_0 {
                    1 => {
                        (*col).stat = 4 as libc::c_int;
                    }
                    2 => {
                        (*col).stat = 2 as libc::c_int;
                    }
                    3 => {
                        (*col).stat = 3 as libc::c_int;
                    }
                    4 => {
                        (*col)
                            .stat = if *flag.offset((kk - m) as isize) as libc::c_int
                            != 0
                        {
                            3 as libc::c_int
                        } else {
                            2 as libc::c_int
                        };
                    }
                    5 => {
                        (*col).stat = 5 as libc::c_int;
                    }
                    _ => {
                        (col != col
                            || {
                                glp_assert_(
                                    b"col != col\0" as *const u8 as *const libc::c_char,
                                    b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                                    525 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                    }
                }
                (*col).bind = 0 as libc::c_int;
            }
        }
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn _glp_spx_store_sol(
    mut lp: *mut SPXLP,
    mut P: *mut glp_prob,
    mut shift: libc::c_int,
    mut map: *const libc::c_int,
    mut daeh: *const libc::c_int,
    mut beta: *const libc::c_double,
    mut pi: *const libc::c_double,
    mut d: *const libc::c_double,
) {
    let mut m: libc::c_int = (*lp).m;
    let mut flag: *mut libc::c_char = (*lp).flag;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut kk: libc::c_int = 0;
    let mut dir: libc::c_double = 0.;
    match (*P).dir {
        1 => {
            dir = 1.0f64;
        }
        2 => {
            dir = -1.0f64;
        }
        _ => {
            (P != P
                || {
                    glp_assert_(
                        b"P != P\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                        584 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    ((*P).m == m
        || {
            glp_assert_(
                b"P->m == m\0" as *const u8 as *const libc::c_char,
                b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                587 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    i = 1 as libc::c_int;
    while i <= m {
        let mut row: *mut GLPROW = *((*P).row).offset(i as isize);
        k = *map.offset(i as isize);
        if k < 0 as libc::c_int {
            k = -k;
        }
        if k == 0 as libc::c_int {
            ((*row).type_0 == 5 as libc::c_int
                || {
                    glp_assert_(
                        b"row->type == GLP_FX\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                        594 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*row).prim = (*row).lb;
            (*row).dual = -dir * *pi.offset(i as isize) * (*row).rii;
        } else {
            kk = *daeh.offset(k as isize);
            if kk <= m {
                (*row).prim = *beta.offset(kk as isize) / (*row).rii;
                if shift != 0 {
                    (*row).prim
                        += if *map.offset(i as isize) < 0 as libc::c_int {
                            (*row).ub
                        } else {
                            (*row).lb
                        };
                }
                (*row).dual = 0.0f64;
            } else {
                (*row)
                    .prim = if *flag.offset((kk - m) as isize) as libc::c_int != 0 {
                    (*row).ub
                } else {
                    (*row).lb
                };
                (*row).dual = dir * *d.offset((kk - m) as isize) * (*row).rii;
            }
        }
        i += 1;
        i;
    }
    (*P).obj_val = (*P).c0;
    j = 1 as libc::c_int;
    while j <= (*P).n {
        let mut col: *mut GLPCOL = *((*P).col).offset(j as isize);
        k = *map.offset((m + j) as isize);
        if k < 0 as libc::c_int {
            k = -k;
        }
        if k == 0 as libc::c_int {
            let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
            let mut dk: libc::c_double = 0.;
            ((*col).type_0 == 5 as libc::c_int
                || {
                    glp_assert_(
                        b"col->type == GLP_FX\0" as *const u8 as *const libc::c_char,
                        b"simplex/spxprob.c\0" as *const u8 as *const libc::c_char,
                        627 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*col).prim = (*col).lb;
            dk = dir * (*col).coef;
            aij = (*col).ptr;
            while !aij.is_null() {
                dk
                    += (*(*aij).row).rii * (*aij).val
                        * *pi.offset((*(*aij).row).i as isize);
                aij = (*aij).c_next;
            }
            (*col).dual = dir * dk;
        } else {
            kk = *daeh.offset(k as isize);
            if kk <= m {
                (*col).prim = *beta.offset(kk as isize) * (*col).sjj;
                if shift != 0 {
                    (*col).prim
                        += if *map.offset((m + j) as isize) < 0 as libc::c_int {
                            (*col).ub
                        } else {
                            (*col).lb
                        };
                }
                (*col).dual = 0.0f64;
            } else {
                (*col)
                    .prim = if *flag.offset((kk - m) as isize) as libc::c_int != 0 {
                    (*col).ub
                } else {
                    (*col).lb
                };
                (*col).dual = dir * *d.offset((kk - m) as isize) / (*col).sjj;
            }
        }
        (*P).obj_val += (*col).coef * (*col).prim;
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn _glp_spx_free_lp(mut lp: *mut SPXLP) {
    glp_free((*lp).A_ptr as *mut libc::c_void);
    glp_free((*lp).A_ind as *mut libc::c_void);
    glp_free((*lp).A_val as *mut libc::c_void);
    glp_free((*lp).b as *mut libc::c_void);
    glp_free((*lp).c as *mut libc::c_void);
    glp_free((*lp).l as *mut libc::c_void);
    glp_free((*lp).u as *mut libc::c_void);
    glp_free((*lp).head as *mut libc::c_void);
    glp_free((*lp).flag as *mut libc::c_void);
}
