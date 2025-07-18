use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn glp_add_rows(P: *mut glp_prob, nrs: libc::c_int) -> libc::c_int;
    fn glp_set_row_bnds(
        P: *mut glp_prob,
        i: libc::c_int,
        type_0: libc::c_int,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_set_mat_row(
        P: *mut glp_prob,
        i: libc::c_int,
        len: libc::c_int,
        ind: *const libc::c_int,
        val: *const libc::c_double,
    );
    fn glp_gmi_cut(
        P: *mut glp_prob,
        j: libc::c_int,
        ind: *mut libc::c_int,
        val: *mut libc::c_double,
        phi: *mut libc::c_double,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct var {
    pub j: libc::c_int,
    pub f: libc::c_double,
}
unsafe extern "C" fn fcmp(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    let mut v1: *const var = p1 as *const var;
    let mut v2: *const var = p2 as *const var;
    if (*v1).f > (*v2).f {
        return -(1 as libc::c_int);
    }
    if (*v1).f < (*v2).f {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn glp_gmi_gen(
    mut P: *mut glp_prob,
    mut pool: *mut glp_prob,
    mut max_cuts: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut m: libc::c_int = (*P).m;
    let mut n: libc::c_int = (*P).n;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut var: *mut var = 0 as *mut var;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut nv: libc::c_int = 0;
    let mut nnn: libc::c_int = 0;
    let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut frac: libc::c_double = 0.;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut phi: *mut libc::c_double = 0 as *mut libc::c_double;
    if !((*P).m == 0 as libc::c_int || (*P).valid != 0) {
        (glp_error_(
            b"intopt/gmigen.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
        ))
            .unwrap()(
            b"glp_gmi_gen: basis factorization does not exist\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !((*P).pbs_stat == 2 as libc::c_int && (*P).dbs_stat == 2 as libc::c_int) {
        (glp_error_(
            b"intopt/gmigen.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
        ))
            .unwrap()(
            b"glp_gmi_gen: optimal basic solution required\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if (*pool).n != n {
        (glp_error_(
            b"intopt/gmigen.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int,
        ))
            .unwrap()(
            b"glp_gmi_gen: cut pool has wrong number of columns\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    var = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<var>() as libc::c_ulong as libc::c_int,
    ) as *mut var;
    ind = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    val = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    phi = glp_alloc(
        1 as libc::c_int + m + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    nv = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while j <= n {
        col = *((*P).col).offset(j as isize);
        if !((*col).kind != 2 as libc::c_int) {
            if !((*col).type_0 == 5 as libc::c_int) {
                if !((*col).stat != 1 as libc::c_int) {
                    frac = (*col).prim - floor((*col).prim);
                    if 0.05f64 <= frac && frac <= 0.95f64 {
                        nv += 1;
                        nv;
                        (*var.offset(nv as isize)).j = j;
                        (*var.offset(nv as isize)).f = frac;
                    }
                }
            }
        }
        j += 1;
        j;
    }
    qsort(
        &mut *var.offset(1 as libc::c_int as isize) as *mut var as *mut libc::c_void,
        nv as size_t,
        ::std::mem::size_of::<var>() as libc::c_ulong,
        Some(
            fcmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    nnn = 0 as libc::c_int;
    t = 1 as libc::c_int;
    while t <= nv {
        len = glp_gmi_cut(P, (*var.offset(t as isize)).j, ind, val, phi);
        if !(len < 1 as libc::c_int) {
            k = 1 as libc::c_int;
            loop {
                if !(k <= len) {
                    current_block = 13550086250199790493;
                    break;
                }
                if fabs(*val.offset(k as isize)) < 1e-03f64 {
                    current_block = 11584701595673473500;
                    break;
                }
                if fabs(*val.offset(k as isize)) > 1e+03f64 {
                    current_block = 11584701595673473500;
                    break;
                }
                k += 1;
                k;
            }
            match current_block {
                11584701595673473500 => {}
                _ => {
                    i = glp_add_rows(pool, 1 as libc::c_int);
                    glp_set_row_bnds(
                        pool,
                        i,
                        2 as libc::c_int,
                        *val.offset(0 as libc::c_int as isize),
                        0 as libc::c_int as libc::c_double,
                    );
                    glp_set_mat_row(
                        pool,
                        i,
                        len,
                        ind as *const libc::c_int,
                        val as *const libc::c_double,
                    );
                    nnn += 1;
                    nnn;
                    if nnn == max_cuts {
                        break;
                    }
                }
            }
        }
        t += 1;
        t;
    }
    glp_free(var as *mut libc::c_void);
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
    glp_free(phi as *mut libc::c_void);
    return nnn;
}
