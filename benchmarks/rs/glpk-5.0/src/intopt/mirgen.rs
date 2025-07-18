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
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn floor(_: libc::c_double) -> libc::c_double;
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
    fn _glp_spv_create_vec(n: libc::c_int) -> *mut SPV;
    fn _glp_spv_set_vj(v: *mut SPV, j: libc::c_int, val: libc::c_double);
    fn _glp_spv_clear_vec(v: *mut SPV);
    fn _glp_spv_clean_vec(v: *mut SPV, eps: libc::c_double);
    fn _glp_spv_copy_vec(x: *mut SPV, y: *mut SPV);
    fn _glp_spv_linear_comb(x: *mut SPV, a: libc::c_double, y: *mut SPV);
    fn _glp_spv_delete_vec(v: *mut SPV);
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub struct glp_mir {
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub skip: *mut libc::c_char,
    pub isint: *mut libc::c_char,
    pub lb: *mut libc::c_double,
    pub vlb: *mut libc::c_int,
    pub ub: *mut libc::c_double,
    pub vub: *mut libc::c_int,
    pub x: *mut libc::c_double,
    pub agg_cnt: libc::c_int,
    pub agg_row: *mut libc::c_int,
    pub agg_vec: *mut SPV,
    pub agg_rhs: libc::c_double,
    pub subst: *mut libc::c_char,
    pub mod_vec: *mut SPV,
    pub mod_rhs: libc::c_double,
    pub cut_vec: *mut SPV,
    pub cut_rhs: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPV {
    pub n: libc::c_int,
    pub nnz: libc::c_int,
    pub pos: *mut libc::c_int,
    pub ind: *mut libc::c_int,
    pub val: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vset {
    pub j: libc::c_int,
    pub v: libc::c_double,
}
unsafe extern "C" fn set_row_attrib(mut mip: *mut glp_prob, mut mir: *mut glp_mir) {
    let mut m: libc::c_int = (*mir).m;
    let mut k: libc::c_int = 0;
    k = 1 as libc::c_int;
    while k <= m {
        let mut row: *mut GLPROW = *((*mip).row).offset(k as isize);
        *((*mir).skip).offset(k as isize) = 0 as libc::c_int as libc::c_char;
        *((*mir).isint).offset(k as isize) = 0 as libc::c_int as libc::c_char;
        match (*row).type_0 {
            1 => {
                *((*mir).lb).offset(k as isize) = -1.7976931348623157e+308f64;
                *((*mir).ub).offset(k as isize) = 1.7976931348623157e+308f64;
            }
            2 => {
                *((*mir).lb).offset(k as isize) = (*row).lb;
                *((*mir).ub).offset(k as isize) = 1.7976931348623157e+308f64;
            }
            3 => {
                *((*mir).lb).offset(k as isize) = -1.7976931348623157e+308f64;
                *((*mir).ub).offset(k as isize) = (*row).ub;
            }
            4 => {
                *((*mir).lb).offset(k as isize) = (*row).lb;
                *((*mir).ub).offset(k as isize) = (*row).ub;
            }
            5 => {
                let ref mut fresh0 = *((*mir).ub).offset(k as isize);
                *fresh0 = (*row).lb;
                *((*mir).lb).offset(k as isize) = *fresh0;
            }
            _ => {
                (row != row
                    || {
                        glp_assert_(
                            b"row != row\0" as *const u8 as *const libc::c_char,
                            b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                            166 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        let ref mut fresh1 = *((*mir).vub).offset(k as isize);
        *fresh1 = 0 as libc::c_int;
        *((*mir).vlb).offset(k as isize) = *fresh1;
        k += 1;
        k;
    }
}
unsafe extern "C" fn set_col_attrib(mut mip: *mut glp_prob, mut mir: *mut glp_mir) {
    let mut m: libc::c_int = (*mir).m;
    let mut n: libc::c_int = (*mir).n;
    let mut k: libc::c_int = 0;
    k = m + 1 as libc::c_int;
    while k <= m + n {
        let mut col: *mut GLPCOL = *((*mip).col).offset((k - m) as isize);
        match (*col).kind {
            1 => {
                *((*mir).isint).offset(k as isize) = 0 as libc::c_int as libc::c_char;
            }
            2 => {
                *((*mir).isint).offset(k as isize) = 1 as libc::c_int as libc::c_char;
            }
            _ => {
                (col != col
                    || {
                        glp_assert_(
                            b"col != col\0" as *const u8 as *const libc::c_char,
                            b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                            186 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        match (*col).type_0 {
            1 => {
                *((*mir).lb).offset(k as isize) = -1.7976931348623157e+308f64;
                *((*mir).ub).offset(k as isize) = 1.7976931348623157e+308f64;
            }
            2 => {
                *((*mir).lb).offset(k as isize) = (*col).lb;
                *((*mir).ub).offset(k as isize) = 1.7976931348623157e+308f64;
            }
            3 => {
                *((*mir).lb).offset(k as isize) = -1.7976931348623157e+308f64;
                *((*mir).ub).offset(k as isize) = (*col).ub;
            }
            4 => {
                *((*mir).lb).offset(k as isize) = (*col).lb;
                *((*mir).ub).offset(k as isize) = (*col).ub;
            }
            5 => {
                let ref mut fresh2 = *((*mir).ub).offset(k as isize);
                *fresh2 = (*col).lb;
                *((*mir).lb).offset(k as isize) = *fresh2;
            }
            _ => {
                (col != col
                    || {
                        glp_assert_(
                            b"col != col\0" as *const u8 as *const libc::c_char,
                            b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                            200 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        let ref mut fresh3 = *((*mir).vub).offset(k as isize);
        *fresh3 = 0 as libc::c_int;
        *((*mir).vlb).offset(k as isize) = *fresh3;
        k += 1;
        k;
    }
}
unsafe extern "C" fn set_var_bounds(mut mip: *mut glp_prob, mut mir: *mut glp_mir) {
    let mut m: libc::c_int = (*mir).m;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: libc::c_int = 0;
    let mut k1: libc::c_int = 0;
    let mut k2: libc::c_int = 0;
    let mut a1: libc::c_double = 0.;
    let mut a2: libc::c_double = 0.;
    let mut current_block_23: u64;
    i = 1 as libc::c_int;
    while i <= m {
        if *((*mir).lb).offset(i as isize) == 0.0f64
            && *((*mir).ub).offset(i as isize) == 1.7976931348623157e+308f64
            || *((*mir).lb).offset(i as isize) == -1.7976931348623157e+308f64
                && *((*mir).ub).offset(i as isize) == 0.0f64
        {
            aij = (**((*mip).row).offset(i as isize)).ptr;
            if !aij.is_null() {
                k1 = m + (*(*aij).col).j;
                a1 = (*aij).val;
                aij = (*aij).r_next;
                if !aij.is_null() {
                    k2 = m + (*(*aij).col).j;
                    a2 = (*aij).val;
                    if ((*aij).r_next).is_null() {
                        if *((*mir).isint).offset(k1 as isize) == 0
                            && *((*mir).isint).offset(k2 as isize) as libc::c_int != 0
                        {
                            current_block_23 = 8236137900636309791;
                        } else if *((*mir).isint).offset(k1 as isize) as libc::c_int != 0
                            && *((*mir).isint).offset(k2 as isize) == 0
                        {
                            k2 = k1;
                            a2 = a1;
                            k1 = m + (*(*aij).col).j;
                            a1 = (*aij).val;
                            current_block_23 = 8236137900636309791;
                        } else {
                            current_block_23 = 8258075665625361029;
                        }
                        match current_block_23 {
                            8258075665625361029 => {}
                            _ => {
                                if !(*((*mir).lb).offset(k2 as isize)
                                    == -1.7976931348623157e+308f64
                                    || *((*mir).ub).offset(k2 as isize)
                                        == 1.7976931348623157e+308f64
                                    || *((*mir).lb).offset(k2 as isize)
                                        == *((*mir).ub).offset(k2 as isize))
                                {
                                    if *((*mir).ub).offset(i as isize) == 0.0f64 {
                                        a1 = -a1;
                                        a2 = -a2;
                                    }
                                    if a1 > 0.0f64 {
                                        if *((*mir).vlb).offset(k1 as isize) == 0 as libc::c_int {
                                            *((*mir).lb).offset(k1 as isize) = -a2 / a1;
                                            *((*mir).vlb).offset(k1 as isize) = k2;
                                            *((*mir).skip)
                                                .offset(i as isize) = 1 as libc::c_int as libc::c_char;
                                        }
                                    } else if *((*mir).vub).offset(k1 as isize)
                                        == 0 as libc::c_int
                                    {
                                        *((*mir).ub).offset(k1 as isize) = -a2 / a1;
                                        *((*mir).vub).offset(k1 as isize) = k2;
                                        *((*mir).skip)
                                            .offset(i as isize) = 1 as libc::c_int as libc::c_char;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn mark_useless_rows(mut mip: *mut glp_prob, mut mir: *mut glp_mir) {
    let mut m: libc::c_int = (*mir).m;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nv: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= m {
        if *((*mir).lb).offset(i as isize) == -1.7976931348623157e+308f64
            && *((*mir).ub).offset(i as isize) == 1.7976931348623157e+308f64
        {
            *((*mir).skip).offset(i as isize) = 1 as libc::c_int as libc::c_char;
        } else {
            nv = 0 as libc::c_int;
            aij = (**((*mip).row).offset(i as isize)).ptr;
            while !aij.is_null() {
                k = m + (*(*aij).col).j;
                if *((*mir).lb).offset(k as isize) == -1.7976931348623157e+308f64
                    && *((*mir).ub).offset(k as isize) == 1.7976931348623157e+308f64
                {
                    *((*mir).skip).offset(i as isize) = 1 as libc::c_int as libc::c_char;
                    break;
                } else if *((*mir).isint).offset(k as isize) as libc::c_int != 0
                    && *((*mir).lb).offset(k as isize) == -1.7976931348623157e+308f64
                    || *((*mir).isint).offset(k as isize) as libc::c_int != 0
                        && *((*mir).ub).offset(k as isize) == 1.7976931348623157e+308f64
                {
                    *((*mir).skip).offset(i as isize) = 1 as libc::c_int as libc::c_char;
                    break;
                } else {
                    if !(*((*mir).vlb).offset(k as isize) == 0 as libc::c_int
                        && *((*mir).vub).offset(k as isize) == 0 as libc::c_int
                        && *((*mir).lb).offset(k as isize)
                            == *((*mir).ub).offset(k as isize))
                    {
                        nv += 1;
                        nv;
                    }
                    aij = (*aij).r_next;
                }
            }
            if nv == 0 as libc::c_int {
                *((*mir).skip).offset(i as isize) = 1 as libc::c_int as libc::c_char;
            }
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn glp_mir_init(mut mip: *mut glp_prob) -> *mut glp_mir {
    let mut m: libc::c_int = (*mip).m;
    let mut n: libc::c_int = (*mip).n;
    let mut mir: *mut glp_mir = 0 as *mut glp_mir;
    mir = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<glp_mir>() as libc::c_ulong as libc::c_int,
    ) as *mut glp_mir;
    (*mir).m = m;
    (*mir).n = n;
    (*mir)
        .skip = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    (*mir)
        .isint = glp_alloc(
        1 as libc::c_int + m + n,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    (*mir)
        .lb = glp_alloc(
        1 as libc::c_int + m + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*mir)
        .vlb = glp_alloc(
        1 as libc::c_int + m + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*mir)
        .ub = glp_alloc(
        1 as libc::c_int + m + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*mir)
        .vub = glp_alloc(
        1 as libc::c_int + m + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*mir)
        .x = glp_alloc(
        1 as libc::c_int + m + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*mir)
        .agg_row = glp_alloc(
        1 as libc::c_int + 5 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*mir).agg_vec = _glp_spv_create_vec(m + n);
    (*mir)
        .subst = glp_alloc(
        1 as libc::c_int + m + n,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    (*mir).mod_vec = _glp_spv_create_vec(m + n);
    (*mir).cut_vec = _glp_spv_create_vec(m + n);
    set_row_attrib(mip, mir);
    set_col_attrib(mip, mir);
    set_var_bounds(mip, mir);
    mark_useless_rows(mip, mir);
    return mir;
}
unsafe extern "C" fn get_current_point(mut mip: *mut glp_prob, mut mir: *mut glp_mir) {
    let mut m: libc::c_int = (*mir).m;
    let mut n: libc::c_int = (*mir).n;
    let mut k: libc::c_int = 0;
    k = 1 as libc::c_int;
    while k <= m {
        *((*mir).x).offset(k as isize) = (**((*mip).row).offset(k as isize)).prim;
        k += 1;
        k;
    }
    k = m + 1 as libc::c_int;
    while k <= m + n {
        *((*mir).x).offset(k as isize) = (**((*mip).col).offset((k - m) as isize)).prim;
        k += 1;
        k;
    }
}
unsafe extern "C" fn initial_agg_row(
    mut mip: *mut glp_prob,
    mut mir: *mut glp_mir,
    mut i: libc::c_int,
) {
    let mut m: libc::c_int = (*mir).m;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    (1 as libc::c_int <= i && i <= m
        || {
            glp_assert_(
                b"1 <= i && i <= m\0" as *const u8 as *const libc::c_char,
                b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                422 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*((*mir).skip).offset(i as isize) == 0
        || {
            glp_assert_(
                b"!mir->skip[i]\0" as *const u8 as *const libc::c_char,
                b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                423 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    *((*mir).skip).offset(i as isize) = 2 as libc::c_int as libc::c_char;
    (*mir).agg_cnt = 1 as libc::c_int;
    *((*mir).agg_row).offset(1 as libc::c_int as isize) = i;
    _glp_spv_clear_vec((*mir).agg_vec);
    _glp_spv_set_vj((*mir).agg_vec, i, 1.0f64);
    aij = (**((*mip).row).offset(i as isize)).ptr;
    while !aij.is_null() {
        _glp_spv_set_vj((*mir).agg_vec, m + (*(*aij).col).j, -(*aij).val);
        aij = (*aij).r_next;
    }
    (*mir).agg_rhs = 0.0f64;
}
unsafe extern "C" fn subst_fixed_vars(mut mir: *mut glp_mir) {
    let mut m: libc::c_int = (*mir).m;
    let mut n: libc::c_int = (*mir).n;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    j = 1 as libc::c_int;
    while j <= (*(*mir).agg_vec).nnz {
        k = *((*(*mir).agg_vec).ind).offset(j as isize);
        (1 as libc::c_int <= k && k <= m + n
            || {
                glp_assert_(
                    b"1 <= k && k <= m+n\0" as *const u8 as *const libc::c_char,
                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                    475 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if *((*mir).vlb).offset(k as isize) == 0 as libc::c_int
            && *((*mir).vub).offset(k as isize) == 0 as libc::c_int
            && *((*mir).lb).offset(k as isize) == *((*mir).ub).offset(k as isize)
        {
            (*mir).agg_rhs
                -= *((*(*mir).agg_vec).val).offset(j as isize)
                    * *((*mir).lb).offset(k as isize);
            *((*(*mir).agg_vec).val).offset(j as isize) = 0.0f64;
        }
        j += 1;
        j;
    }
    _glp_spv_clean_vec((*mir).agg_vec, 2.2204460492503131e-16f64);
}
unsafe extern "C" fn bound_subst_heur(mut mir: *mut glp_mir) {
    let mut m: libc::c_int = (*mir).m;
    let mut n: libc::c_int = (*mir).n;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut kk: libc::c_int = 0;
    let mut d1: libc::c_double = 0.;
    let mut d2: libc::c_double = 0.;
    j = 1 as libc::c_int;
    while j <= (*(*mir).agg_vec).nnz {
        k = *((*(*mir).agg_vec).ind).offset(j as isize);
        (1 as libc::c_int <= k && k <= m + n
            || {
                glp_assert_(
                    b"1 <= k && k <= m+n\0" as *const u8 as *const libc::c_char,
                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                    499 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if !(*((*mir).isint).offset(k as isize) != 0) {
            kk = *((*mir).vlb).offset(k as isize);
            if kk == 0 as libc::c_int {
                if *((*mir).lb).offset(k as isize) == -1.7976931348623157e+308f64 {
                    d1 = 1.7976931348623157e+308f64;
                } else {
                    d1 = *((*mir).x).offset(k as isize)
                        - *((*mir).lb).offset(k as isize);
                }
            } else {
                (1 as libc::c_int <= kk && kk <= m + n
                    || {
                        glp_assert_(
                            b"1 <= kk && kk <= m+n\0" as *const u8
                                as *const libc::c_char,
                            b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                            510 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*((*mir).isint).offset(kk as isize) as libc::c_int != 0
                    || {
                        glp_assert_(
                            b"mir->isint[kk]\0" as *const u8 as *const libc::c_char,
                            b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                            511 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*((*mir).lb).offset(k as isize) != -1.7976931348623157e+308f64
                    || {
                        glp_assert_(
                            b"mir->lb[k] != -DBL_MAX\0" as *const u8
                                as *const libc::c_char,
                            b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                            512 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                d1 = *((*mir).x).offset(k as isize)
                    - *((*mir).lb).offset(k as isize) * *((*mir).x).offset(kk as isize);
            }
            kk = *((*mir).vub).offset(k as isize);
            if kk == 0 as libc::c_int {
                if *((*mir).vub).offset(k as isize) as libc::c_double
                    == 1.7976931348623157e+308f64
                {
                    d2 = 1.7976931348623157e+308f64;
                } else {
                    d2 = *((*mir).ub).offset(k as isize)
                        - *((*mir).x).offset(k as isize);
                }
            } else {
                (1 as libc::c_int <= kk && kk <= m + n
                    || {
                        glp_assert_(
                            b"1 <= kk && kk <= m+n\0" as *const u8
                                as *const libc::c_char,
                            b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                            524 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*((*mir).isint).offset(kk as isize) as libc::c_int != 0
                    || {
                        glp_assert_(
                            b"mir->isint[kk]\0" as *const u8 as *const libc::c_char,
                            b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                            525 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*((*mir).ub).offset(k as isize) != 1.7976931348623157e+308f64
                    || {
                        glp_assert_(
                            b"mir->ub[k] != +DBL_MAX\0" as *const u8
                                as *const libc::c_char,
                            b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                            526 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                d2 = *((*mir).ub).offset(k as isize) * *((*mir).x).offset(kk as isize)
                    - *((*mir).x).offset(k as isize);
            }
            (d1 != 1.7976931348623157e+308f64 || d2 != 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"d1 != DBL_MAX || d2 != DBL_MAX\0" as *const u8
                            as *const libc::c_char,
                        b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                        530 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*((*mir).subst).offset(k as isize) as libc::c_int == '?' as i32
                || {
                    glp_assert_(
                        b"mir->subst[k] == '?'\0" as *const u8 as *const libc::c_char,
                        b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                        532 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if d1 <= d2 {
                *((*mir).subst).offset(k as isize) = 'L' as i32 as libc::c_char;
            } else {
                *((*mir).subst).offset(k as isize) = 'U' as i32 as libc::c_char;
            }
        }
        j += 1;
        j;
    }
}
unsafe extern "C" fn build_mod_row(mut mir: *mut glp_mir) {
    let mut m: libc::c_int = (*mir).m;
    let mut n: libc::c_int = (*mir).n;
    let mut j: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut kk: libc::c_int = 0;
    _glp_spv_copy_vec((*mir).mod_vec, (*mir).agg_vec);
    (*mir).mod_rhs = (*mir).agg_rhs;
    j = (*(*mir).mod_vec).nnz;
    while j >= 1 as libc::c_int {
        k = *((*(*mir).mod_vec).ind).offset(j as isize);
        (1 as libc::c_int <= k && k <= m + n
            || {
                glp_assert_(
                    b"1 <= k && k <= m+n\0" as *const u8 as *const libc::c_char,
                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                    557 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if !(*((*mir).isint).offset(k as isize) != 0) {
            if *((*mir).subst).offset(k as isize) as libc::c_int == 'L' as i32 {
                (*((*mir).lb).offset(k as isize) != -1.7976931348623157e+308f64
                    || {
                        glp_assert_(
                            b"mir->lb[k] != -DBL_MAX\0" as *const u8
                                as *const libc::c_char,
                            b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                            561 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                kk = *((*mir).vlb).offset(k as isize);
                if kk == 0 as libc::c_int {
                    (*mir).mod_rhs
                        -= *((*(*mir).mod_vec).val).offset(j as isize)
                            * *((*mir).lb).offset(k as isize);
                } else {
                    (*((*mir).isint).offset(kk as isize) as libc::c_int != 0
                        || {
                            glp_assert_(
                                b"mir->isint[kk]\0" as *const u8 as *const libc::c_char,
                                b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                                569 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    jj = *((*(*mir).mod_vec).pos).offset(kk as isize);
                    if jj == 0 as libc::c_int {
                        _glp_spv_set_vj((*mir).mod_vec, kk, 1.0f64);
                        jj = *((*(*mir).mod_vec).pos).offset(kk as isize);
                        *((*(*mir).mod_vec).val).offset(jj as isize) = 0.0f64;
                    }
                    *((*(*mir).mod_vec).val).offset(jj as isize)
                        += *((*(*mir).mod_vec).val).offset(j as isize)
                            * *((*mir).lb).offset(k as isize);
                }
            } else if *((*mir).subst).offset(k as isize) as libc::c_int == 'U' as i32 {
                (*((*mir).ub).offset(k as isize) != 1.7976931348623157e+308f64
                    || {
                        glp_assert_(
                            b"mir->ub[k] != +DBL_MAX\0" as *const u8
                                as *const libc::c_char,
                            b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                            582 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                kk = *((*mir).vub).offset(k as isize);
                if kk == 0 as libc::c_int {
                    (*mir).mod_rhs
                        -= *((*(*mir).mod_vec).val).offset(j as isize)
                            * *((*mir).ub).offset(k as isize);
                } else {
                    (*((*mir).isint).offset(kk as isize) as libc::c_int != 0
                        || {
                            glp_assert_(
                                b"mir->isint[kk]\0" as *const u8 as *const libc::c_char,
                                b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                                590 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    jj = *((*(*mir).mod_vec).pos).offset(kk as isize);
                    if jj == 0 as libc::c_int {
                        _glp_spv_set_vj((*mir).mod_vec, kk, 1.0f64);
                        jj = *((*(*mir).mod_vec).pos).offset(kk as isize);
                        *((*(*mir).mod_vec).val).offset(jj as isize) = 0.0f64;
                    }
                    *((*(*mir).mod_vec).val).offset(jj as isize)
                        += *((*(*mir).mod_vec).val).offset(j as isize)
                            * *((*mir).ub).offset(k as isize);
                }
                *((*(*mir).mod_vec).val)
                    .offset(j as isize) = -*((*(*mir).mod_vec).val).offset(j as isize);
            } else {
                (k != k
                    || {
                        glp_assert_(
                            b"k != k\0" as *const u8 as *const libc::c_char,
                            b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                            603 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        j -= 1;
        j;
    }
    j = 1 as libc::c_int;
    while j <= (*(*mir).mod_vec).nnz {
        k = *((*(*mir).mod_vec).ind).offset(j as isize);
        (1 as libc::c_int <= k && k <= m + n
            || {
                glp_assert_(
                    b"1 <= k && k <= m+n\0" as *const u8 as *const libc::c_char,
                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                    611 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if !(*((*mir).isint).offset(k as isize) == 0) {
            (*((*mir).subst).offset(k as isize) as libc::c_int == '?' as i32
                || {
                    glp_assert_(
                        b"mir->subst[k] == '?'\0" as *const u8 as *const libc::c_char,
                        b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                        613 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*((*mir).vlb).offset(k as isize) == 0 as libc::c_int
                && *((*mir).vub).offset(k as isize) == 0 as libc::c_int
                || {
                    glp_assert_(
                        b"mir->vlb[k] == 0 && mir->vub[k] == 0\0" as *const u8
                            as *const libc::c_char,
                        b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                        614 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*((*mir).lb).offset(k as isize) != -1.7976931348623157e+308f64
                && *((*mir).ub).offset(k as isize) != 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"mir->lb[k] != -DBL_MAX && mir->ub[k] != +DBL_MAX\0"
                            as *const u8 as *const libc::c_char,
                        b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                        615 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if fabs(*((*mir).lb).offset(k as isize))
                <= fabs(*((*mir).ub).offset(k as isize))
            {
                *((*mir).subst).offset(k as isize) = 'L' as i32 as libc::c_char;
                (*mir).mod_rhs
                    -= *((*(*mir).mod_vec).val).offset(j as isize)
                        * *((*mir).lb).offset(k as isize);
            } else {
                *((*mir).subst).offset(k as isize) = 'U' as i32 as libc::c_char;
                (*mir).mod_rhs
                    -= *((*(*mir).mod_vec).val).offset(j as isize)
                        * *((*mir).ub).offset(k as isize);
                *((*(*mir).mod_vec).val)
                    .offset(j as isize) = -*((*(*mir).mod_vec).val).offset(j as isize);
            }
        }
        j += 1;
        j;
    }
}
unsafe extern "C" fn mir_ineq(
    n: libc::c_int,
    mut a: *const libc::c_double,
    b: libc::c_double,
    mut alpha: *mut libc::c_double,
    mut beta: *mut libc::c_double,
    mut gamma: *mut libc::c_double,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut f: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    if fabs(b - floor(b + 0.5f64)) < 0.01f64 {
        return 1 as libc::c_int;
    }
    f = b - floor(b);
    j = 1 as libc::c_int;
    while j <= n {
        t = *a.offset(j as isize) - floor(*a.offset(j as isize)) - f;
        if t <= 0.0f64 {
            *alpha.offset(j as isize) = floor(*a.offset(j as isize));
        } else {
            *alpha.offset(j as isize) = floor(*a.offset(j as isize)) + t / (1.0f64 - f);
        }
        j += 1;
        j;
    }
    *beta = floor(b);
    *gamma = 1.0f64 / (1.0f64 - f);
    return 0 as libc::c_int;
}
unsafe extern "C" fn cmir_ineq(
    n: libc::c_int,
    mut a: *const libc::c_double,
    b: libc::c_double,
    mut u: *const libc::c_double,
    mut cset: *const libc::c_char,
    delta: libc::c_double,
    mut alpha: *mut libc::c_double,
    mut beta: *mut libc::c_double,
    mut gamma: *mut libc::c_double,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut aa: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut bb: libc::c_double = 0.;
    aa = alpha;
    bb = b;
    j = 1 as libc::c_int;
    while j <= n {
        *aa.offset(j as isize) = *a.offset(j as isize) / delta;
        if *cset.offset(j as isize) != 0 {
            *aa.offset(j as isize) = -*aa.offset(j as isize);
            bb -= *a.offset(j as isize) * *u.offset(j as isize);
        }
        j += 1;
        j;
    }
    bb /= delta;
    if mir_ineq(n, aa as *const libc::c_double, bb, alpha, beta, gamma) != 0 {
        return 1 as libc::c_int;
    }
    j = 1 as libc::c_int;
    while j <= n {
        if *cset.offset(j as isize) != 0 {
            *alpha.offset(j as isize) = -*alpha.offset(j as isize);
            *beta += *alpha.offset(j as isize) * *u.offset(j as isize);
        }
        j += 1;
        j;
    }
    *gamma /= delta;
    return 0 as libc::c_int;
}
unsafe extern "C" fn cmir_cmp(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    let mut v1: *const vset = p1 as *const vset;
    let mut v2: *const vset = p2 as *const vset;
    if (*v1).v < (*v2).v {
        return -(1 as libc::c_int);
    }
    if (*v1).v > (*v2).v {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn cmir_sep(
    n: libc::c_int,
    mut a: *const libc::c_double,
    b: libc::c_double,
    mut u: *const libc::c_double,
    mut x: *const libc::c_double,
    s: libc::c_double,
    mut alpha: *mut libc::c_double,
    mut beta: *mut libc::c_double,
    mut gamma: *mut libc::c_double,
) -> libc::c_double {
    let mut fail: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nv: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut delta: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    let mut d_try: [libc::c_double; 4] = [0.; 4];
    let mut r: libc::c_double = 0.;
    let mut r_best: libc::c_double = 0.;
    let mut cset: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vset: *mut vset = 0 as *mut vset;
    cset = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    vset = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<vset>() as libc::c_ulong as libc::c_int,
    ) as *mut vset;
    j = 1 as libc::c_int;
    while j <= n {
        *cset
            .offset(
                j as isize,
            ) = (*x.offset(j as isize) >= 0.5f64 * *u.offset(j as isize)) as libc::c_int
            as libc::c_char;
        j += 1;
        j;
    }
    delta = 0.0f64;
    r_best = delta;
    j = 1 as libc::c_int;
    while j <= n {
        (*a.offset(j as isize) != 0.0f64
            || {
                glp_assert_(
                    b"a[j] != 0.0\0" as *const u8 as *const libc::c_char,
                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                    822 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        eps = 1e-9f64 * (1.0f64 + fabs(*u.offset(j as isize)));
        if !(*x.offset(j as isize) < eps
            || *x.offset(j as isize) > *u.offset(j as isize) - eps)
        {
            fail = cmir_ineq(
                n,
                a,
                b,
                u,
                cset as *const libc::c_char,
                fabs(*a.offset(j as isize)),
                alpha,
                beta,
                gamma,
            );
            if !(fail != 0) {
                r = -*beta - *gamma * s;
                k = 1 as libc::c_int;
                while k <= n {
                    r += *alpha.offset(k as isize) * *x.offset(k as isize);
                    k += 1;
                    k;
                }
                if r_best < r {
                    r_best = r;
                    delta = fabs(*a.offset(j as isize));
                }
            }
        }
        j += 1;
        j;
    }
    if r_best < 0.001f64 {
        r_best = 0.0f64;
    }
    if !(r_best == 0.0f64) {
        (delta > 0.0f64
            || {
                glp_assert_(
                    b"delta > 0.0\0" as *const u8 as *const libc::c_char,
                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                    837 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        d_try[1 as libc::c_int as usize] = delta / 2.0f64;
        d_try[2 as libc::c_int as usize] = delta / 4.0f64;
        d_try[3 as libc::c_int as usize] = delta / 8.0f64;
        j = 1 as libc::c_int;
        while j <= 3 as libc::c_int {
            fail = cmir_ineq(
                n,
                a,
                b,
                u,
                cset as *const libc::c_char,
                d_try[j as usize],
                alpha,
                beta,
                gamma,
            );
            if !(fail != 0) {
                r = -*beta - *gamma * s;
                k = 1 as libc::c_int;
                while k <= n {
                    r += *alpha.offset(k as isize) * *x.offset(k as isize);
                    k += 1;
                    k;
                }
                if r_best < r {
                    r_best = r;
                    delta = d_try[j as usize];
                }
            }
            j += 1;
            j;
        }
        nv = 0 as libc::c_int;
        j = 1 as libc::c_int;
        while j <= n {
            eps = 1e-9f64 * (1.0f64 + fabs(*u.offset(j as isize)));
            if !(*x.offset(j as isize) < eps
                || *x.offset(j as isize) > *u.offset(j as isize) - eps)
            {
                nv += 1;
                nv;
                (*vset.offset(nv as isize)).j = j;
                (*vset.offset(nv as isize))
                    .v = fabs(*x.offset(j as isize) - 0.5f64 * *u.offset(j as isize));
            }
            j += 1;
            j;
        }
        qsort(
            &mut *vset.offset(1 as libc::c_int as isize) as *mut vset
                as *mut libc::c_void,
            nv as size_t,
            ::std::mem::size_of::<vset>() as libc::c_ulong,
            Some(
                cmir_cmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        v = 1 as libc::c_int;
        while v <= nv {
            j = (*vset.offset(v as isize)).j;
            *cset
                .offset(
                    j as isize,
                ) = (*cset.offset(j as isize) == 0) as libc::c_int as libc::c_char;
            fail = cmir_ineq(
                n,
                a,
                b,
                u,
                cset as *const libc::c_char,
                delta,
                alpha,
                beta,
                gamma,
            );
            *cset
                .offset(
                    j as isize,
                ) = (*cset.offset(j as isize) == 0) as libc::c_int as libc::c_char;
            if !(fail != 0) {
                r = -*beta - *gamma * s;
                k = 1 as libc::c_int;
                while k <= n {
                    r += *alpha.offset(k as isize) * *x.offset(k as isize);
                    k += 1;
                    k;
                }
                if r_best < r {
                    r_best = r;
                    *cset
                        .offset(
                            j as isize,
                        ) = (*cset.offset(j as isize) == 0) as libc::c_int
                        as libc::c_char;
                }
            }
            v += 1;
            v;
        }
        fail = cmir_ineq(
            n,
            a,
            b,
            u,
            cset as *const libc::c_char,
            delta,
            alpha,
            beta,
            gamma,
        );
        (fail == 0
            || {
                glp_assert_(
                    b"!fail\0" as *const u8 as *const libc::c_char,
                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                    885 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    glp_free(cset as *mut libc::c_void);
    glp_free(vset as *mut libc::c_void);
    return r_best;
}
unsafe extern "C" fn generate(mut mir: *mut glp_mir) -> libc::c_double {
    let mut current_block: u64;
    let mut m: libc::c_int = (*mir).m;
    let mut n: libc::c_int = (*mir).n;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut kk: libc::c_int = 0;
    let mut nint: libc::c_int = 0;
    let mut s: libc::c_double = 0.;
    let mut u: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut alpha: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut r_best: libc::c_double = 0.0f64;
    let mut b: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut gamma: libc::c_double = 0.;
    _glp_spv_copy_vec((*mir).cut_vec, (*mir).mod_vec);
    (*mir).cut_rhs = (*mir).mod_rhs;
    _glp_spv_clean_vec((*mir).cut_vec, 2.2204460492503131e-16f64);
    j = 1 as libc::c_int;
    while j <= (*(*mir).cut_vec).nnz {
        k = *((*(*mir).cut_vec).ind).offset(j as isize);
        (1 as libc::c_int <= k && k <= m + n
            || {
                glp_assert_(
                    b"1 <= k && k <= m+n\0" as *const u8 as *const libc::c_char,
                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                    910 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if *((*mir).isint).offset(k as isize) == 0
            && *((*(*mir).cut_vec).val).offset(j as isize) > 0.0f64
        {
            *((*(*mir).cut_vec).val).offset(j as isize) = 0.0f64;
        }
        j += 1;
        j;
    }
    _glp_spv_clean_vec((*mir).cut_vec, 0.0f64);
    nint = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while j <= (*(*mir).cut_vec).nnz {
        k = *((*(*mir).cut_vec).ind).offset(j as isize);
        (1 as libc::c_int <= k && k <= m + n
            || {
                glp_assert_(
                    b"1 <= k && k <= m+n\0" as *const u8 as *const libc::c_char,
                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                    923 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if *((*mir).isint).offset(k as isize) != 0 {
            let mut temp: libc::c_double = 0.;
            nint += 1;
            nint;
            kk = *((*(*mir).cut_vec).ind).offset(nint as isize);
            *((*(*mir).cut_vec).pos).offset(k as isize) = nint;
            *((*(*mir).cut_vec).pos).offset(kk as isize) = j;
            *((*(*mir).cut_vec).ind).offset(nint as isize) = k;
            *((*(*mir).cut_vec).ind).offset(j as isize) = kk;
            temp = *((*(*mir).cut_vec).val).offset(nint as isize);
            *((*(*mir).cut_vec).val)
                .offset(nint as isize) = *((*(*mir).cut_vec).val).offset(j as isize);
            *((*(*mir).cut_vec).val).offset(j as isize) = temp;
        }
        j += 1;
        j;
    }
    if !(nint == 0 as libc::c_int) {
        u = glp_alloc(
            1 as libc::c_int + nint,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        x = glp_alloc(
            1 as libc::c_int + nint,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        alpha = glp_alloc(
            1 as libc::c_int + nint,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        j = 1 as libc::c_int;
        loop {
            if !(j <= nint) {
                current_block = 7746103178988627676;
                break;
            }
            k = *((*(*mir).cut_vec).ind).offset(j as isize);
            (m + 1 as libc::c_int <= k && k <= m + n
                || {
                    glp_assert_(
                        b"m+1 <= k && k <= m+n\0" as *const u8 as *const libc::c_char,
                        b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                        950 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*((*mir).isint).offset(k as isize) as libc::c_int != 0
                || {
                    glp_assert_(
                        b"mir->isint[k]\0" as *const u8 as *const libc::c_char,
                        b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                        951 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            *u
                .offset(
                    j as isize,
                ) = *((*mir).ub).offset(k as isize) - *((*mir).lb).offset(k as isize);
            (*u.offset(j as isize) >= 1.0f64
                || {
                    glp_assert_(
                        b"u[j] >= 1.0\0" as *const u8 as *const libc::c_char,
                        b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                        953 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if *((*mir).subst).offset(k as isize) as libc::c_int == 'L' as i32 {
                *x
                    .offset(
                        j as isize,
                    ) = *((*mir).x).offset(k as isize) - *((*mir).lb).offset(k as isize);
            } else if *((*mir).subst).offset(k as isize) as libc::c_int == 'U' as i32 {
                *x
                    .offset(
                        j as isize,
                    ) = *((*mir).ub).offset(k as isize) - *((*mir).x).offset(k as isize);
            } else {
                (k != k
                    || {
                        glp_assert_(
                            b"k != k\0" as *const u8 as *const libc::c_char,
                            b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                            959 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            if *x.offset(j as isize) < -0.001f64 {
                glp_printf(
                    b"glp_mir_gen: warning: x[%d] = %g\n\0" as *const u8
                        as *const libc::c_char,
                    j,
                    *x.offset(j as isize),
                );
                r_best = 0.0f64;
                current_block = 6960623293584419601;
                break;
            } else {
                if *x.offset(j as isize) < 0.0f64 {
                    *x.offset(j as isize) = 0.0f64;
                }
                j += 1;
                j;
            }
        }
        match current_block {
            7746103178988627676 => {
                s = 0.0f64;
                j = nint + 1 as libc::c_int;
                loop {
                    if !(j <= (*(*mir).cut_vec).nnz) {
                        current_block = 8835654301469918283;
                        break;
                    }
                    let mut x_0: libc::c_double = 0.;
                    k = *((*(*mir).cut_vec).ind).offset(j as isize);
                    (1 as libc::c_int <= k && k <= m + n
                        || {
                            glp_assert_(
                                b"1 <= k && k <= m+n\0" as *const u8 as *const libc::c_char,
                                b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                                976 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    (*((*mir).isint).offset(k as isize) == 0
                        || {
                            glp_assert_(
                                b"!mir->isint[k]\0" as *const u8 as *const libc::c_char,
                                b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                                978 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    if *((*mir).subst).offset(k as isize) as libc::c_int == 'L' as i32 {
                        (*((*mir).lb).offset(k as isize) != -1.7976931348623157e+308f64
                            || {
                                glp_assert_(
                                    b"mir->lb[k] != -DBL_MAX\0" as *const u8
                                        as *const libc::c_char,
                                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                                    980 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        kk = *((*mir).vlb).offset(k as isize);
                        if kk == 0 as libc::c_int {
                            x_0 = *((*mir).x).offset(k as isize)
                                - *((*mir).lb).offset(k as isize);
                        } else {
                            x_0 = *((*mir).x).offset(k as isize)
                                - *((*mir).lb).offset(k as isize)
                                    * *((*mir).x).offset(kk as isize);
                        }
                    } else if *((*mir).subst).offset(k as isize) as libc::c_int
                        == 'U' as i32
                    {
                        (*((*mir).ub).offset(k as isize) != 1.7976931348623157e+308f64
                            || {
                                glp_assert_(
                                    b"mir->ub[k] != +DBL_MAX\0" as *const u8
                                        as *const libc::c_char,
                                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                                    988 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        kk = *((*mir).vub).offset(k as isize);
                        if kk == 0 as libc::c_int {
                            x_0 = *((*mir).ub).offset(k as isize)
                                - *((*mir).x).offset(k as isize);
                        } else {
                            x_0 = *((*mir).ub).offset(k as isize)
                                * *((*mir).x).offset(kk as isize)
                                - *((*mir).x).offset(k as isize);
                        }
                    } else {
                        (k != k
                            || {
                                glp_assert_(
                                    b"k != k\0" as *const u8 as *const libc::c_char,
                                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                                    996 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                    }
                    if x_0 < -0.001f64 {
                        glp_printf(
                            b"glp_mir_gen: warning: x = %g\n\0" as *const u8
                                as *const libc::c_char,
                            x_0,
                        );
                        r_best = 0.0f64;
                        current_block = 6960623293584419601;
                        break;
                    } else {
                        if x_0 < 0.0f64 {
                            x_0 = 0.0f64;
                        }
                        s -= *((*(*mir).cut_vec).val).offset(j as isize) * x_0;
                        j += 1;
                        j;
                    }
                }
                match current_block {
                    6960623293584419601 => {}
                    _ => {
                        (s >= 0.0f64
                            || {
                                glp_assert_(
                                    b"s >= 0.0\0" as *const u8 as *const libc::c_char,
                                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                                    1009 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        b = (*mir).cut_rhs;
                        r_best = cmir_sep(
                            nint,
                            (*(*mir).cut_vec).val as *const libc::c_double,
                            b,
                            u as *const libc::c_double,
                            x as *const libc::c_double,
                            s,
                            alpha,
                            &mut beta,
                            &mut gamma,
                        );
                        if !(r_best == 0.0f64) {
                            (r_best > 0.0f64
                                || {
                                    glp_assert_(
                                        b"r_best > 0.0\0" as *const u8 as *const libc::c_char,
                                        b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                                        1015 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                            j = 1 as libc::c_int;
                            while j <= nint {
                                *((*(*mir).cut_vec).val)
                                    .offset(j as isize) = *alpha.offset(j as isize);
                                j += 1;
                                j;
                            }
                            j = nint + 1 as libc::c_int;
                            while j <= (*(*mir).cut_vec).nnz {
                                k = *((*(*mir).cut_vec).ind).offset(j as isize);
                                if k <= m + n {
                                    *((*(*mir).cut_vec).val).offset(j as isize) *= gamma;
                                }
                                j += 1;
                                j;
                            }
                            (*mir).cut_rhs = beta;
                        }
                    }
                }
            }
            _ => {}
        }
        glp_free(u as *mut libc::c_void);
        glp_free(x as *mut libc::c_void);
        glp_free(alpha as *mut libc::c_void);
    }
    return r_best;
}
unsafe extern "C" fn back_subst(mut mir: *mut glp_mir) {
    let mut m: libc::c_int = (*mir).m;
    let mut n: libc::c_int = (*mir).n;
    let mut j: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut kk: libc::c_int = 0;
    j = 1 as libc::c_int;
    while j <= (*(*mir).cut_vec).nnz {
        k = *((*(*mir).cut_vec).ind).offset(j as isize);
        (1 as libc::c_int <= k && k <= m + n
            || {
                glp_assert_(
                    b"1 <= k && k <= m+n\0" as *const u8 as *const libc::c_char,
                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                    1089 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if !(*((*mir).isint).offset(k as isize) == 0) {
            if *((*mir).subst).offset(k as isize) as libc::c_int == 'L' as i32 {
                (*((*mir).lb).offset(k as isize) != -1.7976931348623157e+308f64
                    || {
                        glp_assert_(
                            b"mir->lb[k] != -DBL_MAX\0" as *const u8
                                as *const libc::c_char,
                            b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                            1093 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*((*mir).vlb).offset(k as isize) == 0 as libc::c_int
                    || {
                        glp_assert_(
                            b"mir->vlb[k] == 0\0" as *const u8 as *const libc::c_char,
                            b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                            1094 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*mir).cut_rhs
                    += *((*(*mir).cut_vec).val).offset(j as isize)
                        * *((*mir).lb).offset(k as isize);
            } else if *((*mir).subst).offset(k as isize) as libc::c_int == 'U' as i32 {
                (*((*mir).ub).offset(k as isize) != 1.7976931348623157e+308f64
                    || {
                        glp_assert_(
                            b"mir->ub[k] != +DBL_MAX\0" as *const u8
                                as *const libc::c_char,
                            b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                            1099 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*((*mir).vub).offset(k as isize) == 0 as libc::c_int
                    || {
                        glp_assert_(
                            b"mir->vub[k] == 0\0" as *const u8 as *const libc::c_char,
                            b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                            1100 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*mir).cut_rhs
                    -= *((*(*mir).cut_vec).val).offset(j as isize)
                        * *((*mir).ub).offset(k as isize);
                *((*(*mir).cut_vec).val)
                    .offset(j as isize) = -*((*(*mir).cut_vec).val).offset(j as isize);
            } else {
                (k != k
                    || {
                        glp_assert_(
                            b"k != k\0" as *const u8 as *const libc::c_char,
                            b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                            1105 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        j += 1;
        j;
    }
    j = 1 as libc::c_int;
    while j <= (*(*mir).cut_vec).nnz {
        k = *((*(*mir).cut_vec).ind).offset(j as isize);
        (1 as libc::c_int <= k && k <= m + n
            || {
                glp_assert_(
                    b"1 <= k && k <= m+n\0" as *const u8 as *const libc::c_char,
                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                    1110 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if !(*((*mir).isint).offset(k as isize) != 0) {
            if *((*mir).subst).offset(k as isize) as libc::c_int == 'L' as i32 {
                (*((*mir).lb).offset(k as isize) != -1.7976931348623157e+308f64
                    || {
                        glp_assert_(
                            b"mir->lb[k] != -DBL_MAX\0" as *const u8
                                as *const libc::c_char,
                            b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                            1114 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                kk = *((*mir).vlb).offset(k as isize);
                if kk == 0 as libc::c_int {
                    (*mir).cut_rhs
                        += *((*(*mir).cut_vec).val).offset(j as isize)
                            * *((*mir).lb).offset(k as isize);
                } else {
                    jj = *((*(*mir).cut_vec).pos).offset(kk as isize);
                    if jj == 0 as libc::c_int {
                        _glp_spv_set_vj((*mir).cut_vec, kk, 1.0f64);
                        jj = *((*(*mir).cut_vec).pos).offset(kk as isize);
                        (jj != 0 as libc::c_int
                            || {
                                glp_assert_(
                                    b"jj != 0\0" as *const u8 as *const libc::c_char,
                                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                                    1129 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        *((*(*mir).cut_vec).val).offset(jj as isize) = 0.0f64;
                    }
                    *((*(*mir).cut_vec).val).offset(jj as isize)
                        -= *((*(*mir).cut_vec).val).offset(j as isize)
                            * *((*mir).lb).offset(k as isize);
                }
            } else if *((*mir).subst).offset(k as isize) as libc::c_int == 'U' as i32 {
                (*((*mir).ub).offset(k as isize) != 1.7976931348623157e+308f64
                    || {
                        glp_assert_(
                            b"mir->ub[k] != +DBL_MAX\0" as *const u8
                                as *const libc::c_char,
                            b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                            1139 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                kk = *((*mir).vub).offset(k as isize);
                if kk == 0 as libc::c_int {
                    (*mir).cut_rhs
                        -= *((*(*mir).cut_vec).val).offset(j as isize)
                            * *((*mir).ub).offset(k as isize);
                } else {
                    jj = *((*(*mir).cut_vec).pos).offset(kk as isize);
                    if jj == 0 as libc::c_int {
                        _glp_spv_set_vj((*mir).cut_vec, kk, 1.0f64);
                        jj = *((*(*mir).cut_vec).pos).offset(kk as isize);
                        (jj != 0 as libc::c_int
                            || {
                                glp_assert_(
                                    b"jj != 0\0" as *const u8 as *const libc::c_char,
                                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                                    1151 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        *((*(*mir).cut_vec).val).offset(jj as isize) = 0.0f64;
                    }
                    *((*(*mir).cut_vec).val).offset(jj as isize)
                        += *((*(*mir).cut_vec).val).offset(j as isize)
                            * *((*mir).ub).offset(k as isize);
                }
                *((*(*mir).cut_vec).val)
                    .offset(j as isize) = -*((*(*mir).cut_vec).val).offset(j as isize);
            } else {
                (k != k
                    || {
                        glp_assert_(
                            b"k != k\0" as *const u8 as *const libc::c_char,
                            b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                            1160 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        j += 1;
        j;
    }
}
unsafe extern "C" fn subst_aux_vars(mut mip: *mut glp_prob, mut mir: *mut glp_mir) {
    let mut m: libc::c_int = (*mir).m;
    let mut n: libc::c_int = (*mir).n;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut kk: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    j = (*(*mir).cut_vec).nnz;
    while j >= 1 as libc::c_int {
        k = *((*(*mir).cut_vec).ind).offset(j as isize);
        (1 as libc::c_int <= k && k <= m + n
            || {
                glp_assert_(
                    b"1 <= k && k <= m+n\0" as *const u8 as *const libc::c_char,
                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                    1203 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if !(k > m) {
            aij = (**((*mip).row).offset(k as isize)).ptr;
            while !aij.is_null() {
                kk = m + (*(*aij).col).j;
                jj = *((*(*mir).cut_vec).pos).offset(kk as isize);
                if jj == 0 as libc::c_int {
                    _glp_spv_set_vj((*mir).cut_vec, kk, 1.0f64);
                    jj = *((*(*mir).cut_vec).pos).offset(kk as isize);
                    *((*(*mir).cut_vec).val).offset(jj as isize) = 0.0f64;
                }
                *((*(*mir).cut_vec).val).offset(jj as isize)
                    += *((*(*mir).cut_vec).val).offset(j as isize) * (*aij).val;
                aij = (*aij).r_next;
            }
            *((*(*mir).cut_vec).val).offset(j as isize) = 0.0f64;
        }
        j -= 1;
        j;
    }
    _glp_spv_clean_vec((*mir).cut_vec, 0.0f64);
}
unsafe extern "C" fn add_cut(mut mir: *mut glp_mir, mut pool: *mut glp_prob) {
    let mut m: libc::c_int = (*mir).m;
    let mut n: libc::c_int = (*mir).n;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ind: *mut libc::c_int = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    let mut val: *mut libc::c_double = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    len = 0 as libc::c_int;
    j = (*(*mir).cut_vec).nnz;
    while j >= 1 as libc::c_int {
        k = *((*(*mir).cut_vec).ind).offset(j as isize);
        (m + 1 as libc::c_int <= k && k <= m + n
            || {
                glp_assert_(
                    b"m+1 <= k && k <= m+n\0" as *const u8 as *const libc::c_char,
                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                    1231 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        len += 1;
        len;
        *ind.offset(len as isize) = k - m;
        *val.offset(len as isize) = *((*(*mir).cut_vec).val).offset(j as isize);
        j -= 1;
        j;
    }
    let mut i: libc::c_int = 0;
    i = glp_add_rows(pool, 1 as libc::c_int);
    glp_set_row_bnds(
        pool,
        i,
        3 as libc::c_int,
        0 as libc::c_int as libc::c_double,
        (*mir).cut_rhs,
    );
    glp_set_mat_row(
        pool,
        i,
        len,
        ind as *const libc::c_int,
        val as *const libc::c_double,
    );
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
}
unsafe extern "C" fn aggregate_row(
    mut mip: *mut glp_prob,
    mut mir: *mut glp_mir,
    mut v: *mut SPV,
) -> libc::c_int {
    let mut m: libc::c_int = (*mir).m;
    let mut n: libc::c_int = (*mir).n;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut ii: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut kk: libc::c_int = 0;
    let mut kappa: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut d1: libc::c_double = 0.;
    let mut d2: libc::c_double = 0.;
    let mut d: libc::c_double = 0.;
    let mut d_max: libc::c_double = 0.0f64;
    j = 1 as libc::c_int;
    while j <= (*(*mir).agg_vec).nnz {
        k = *((*(*mir).agg_vec).ind).offset(j as isize);
        (1 as libc::c_int <= k && k <= m + n
            || {
                glp_assert_(
                    b"1 <= k && k <= m+n\0" as *const u8 as *const libc::c_char,
                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                    1272 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if !(k <= m) {
            if !(*((*mir).isint).offset(k as isize) != 0) {
                if !(fabs(*((*(*mir).agg_vec).val).offset(j as isize)) < 0.001f64) {
                    kk = *((*mir).vlb).offset(k as isize);
                    if kk == 0 as libc::c_int {
                        if *((*mir).lb).offset(k as isize) == -1.7976931348623157e+308f64
                        {
                            d1 = 1.7976931348623157e+308f64;
                        } else {
                            d1 = *((*mir).x).offset(k as isize)
                                - *((*mir).lb).offset(k as isize);
                        }
                    } else {
                        (1 as libc::c_int <= kk && kk <= m + n
                            || {
                                glp_assert_(
                                    b"1 <= kk && kk <= m+n\0" as *const u8
                                        as *const libc::c_char,
                                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                                    1285 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        (*((*mir).isint).offset(kk as isize) as libc::c_int != 0
                            || {
                                glp_assert_(
                                    b"mir->isint[kk]\0" as *const u8 as *const libc::c_char,
                                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                                    1286 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        (*((*mir).lb).offset(k as isize) != -1.7976931348623157e+308f64
                            || {
                                glp_assert_(
                                    b"mir->lb[k] != -DBL_MAX\0" as *const u8
                                        as *const libc::c_char,
                                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                                    1287 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        d1 = *((*mir).x).offset(k as isize)
                            - *((*mir).lb).offset(k as isize)
                                * *((*mir).x).offset(kk as isize);
                    }
                    kk = *((*mir).vub).offset(k as isize);
                    if kk == 0 as libc::c_int {
                        if *((*mir).vub).offset(k as isize) as libc::c_double
                            == 1.7976931348623157e+308f64
                        {
                            d2 = 1.7976931348623157e+308f64;
                        } else {
                            d2 = *((*mir).ub).offset(k as isize)
                                - *((*mir).x).offset(k as isize);
                        }
                    } else {
                        (1 as libc::c_int <= kk && kk <= m + n
                            || {
                                glp_assert_(
                                    b"1 <= kk && kk <= m+n\0" as *const u8
                                        as *const libc::c_char,
                                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                                    1299 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        (*((*mir).isint).offset(kk as isize) as libc::c_int != 0
                            || {
                                glp_assert_(
                                    b"mir->isint[kk]\0" as *const u8 as *const libc::c_char,
                                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                                    1300 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        (*((*mir).ub).offset(k as isize) != 1.7976931348623157e+308f64
                            || {
                                glp_assert_(
                                    b"mir->ub[k] != +DBL_MAX\0" as *const u8
                                        as *const libc::c_char,
                                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                                    1301 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        d2 = *((*mir).ub).offset(k as isize)
                            * *((*mir).x).offset(kk as isize)
                            - *((*mir).x).offset(k as isize);
                    }
                    (d1 != 1.7976931348623157e+308f64 || d2 != 1.7976931348623157e+308f64
                        || {
                            glp_assert_(
                                b"d1 != DBL_MAX || d2 != DBL_MAX\0" as *const u8
                                    as *const libc::c_char,
                                b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                                1305 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    d = if d1 <= d2 { d1 } else { d2 };
                    (d != 1.7976931348623157e+308f64
                        || {
                            glp_assert_(
                                b"d != DBL_MAX\0" as *const u8 as *const libc::c_char,
                                b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                                1308 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    if !(d < 0.001f64) {
                        if d_max < d {
                            d_max = d;
                            kappa = k;
                        }
                    }
                }
            }
        }
        j += 1;
        j;
    }
    if kappa == 0 as libc::c_int {
        ret = 1 as libc::c_int;
    } else {
        (m + 1 as libc::c_int <= kappa && kappa <= m + n
            || {
                glp_assert_(
                    b"m+1 <= kappa && kappa <= m+n\0" as *const u8
                        as *const libc::c_char,
                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                    1319 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*((*mir).isint).offset(kappa as isize) == 0
            || {
                glp_assert_(
                    b"!mir->isint[kappa]\0" as *const u8 as *const libc::c_char,
                    b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                    1320 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        ii = 0 as libc::c_int;
        aij = (**((*mip).col).offset((kappa - m) as isize)).ptr;
        while !aij.is_null() {
            if !((*(*aij).row).i > m) {
                if !(*((*mir).skip).offset((*(*aij).row).i as isize) != 0) {
                    if fabs((*aij).val) >= 0.001f64 {
                        ii = (*(*aij).row).i;
                        break;
                    }
                }
            }
            aij = (*aij).c_next;
        }
        if ii == 0 as libc::c_int {
            ret = 2 as libc::c_int;
        } else {
            (*mir).agg_cnt += 1;
            (*mir).agg_cnt;
            ((*mir).agg_cnt <= 5 as libc::c_int
                || {
                    glp_assert_(
                        b"mir->agg_cnt <= MAXAGGR\0" as *const u8 as *const libc::c_char,
                        b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                        1352 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            *((*mir).agg_row).offset((*mir).agg_cnt as isize) = ii;
            *((*mir).skip).offset(ii as isize) = 2 as libc::c_int as libc::c_char;
            _glp_spv_clear_vec(v);
            _glp_spv_set_vj(v, ii, 1.0f64);
            aij = (**((*mip).row).offset(ii as isize)).ptr;
            while !aij.is_null() {
                _glp_spv_set_vj(v, m + (*(*aij).col).j, -(*aij).val);
                aij = (*aij).r_next;
            }
            j = *((*(*mir).agg_vec).pos).offset(kappa as isize);
            (j != 0 as libc::c_int
                || {
                    glp_assert_(
                        b"j != 0\0" as *const u8 as *const libc::c_char,
                        b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                        1369 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            jj = *((*v).pos).offset(kappa as isize);
            (jj != 0 as libc::c_int
                || {
                    glp_assert_(
                        b"jj != 0\0" as *const u8 as *const libc::c_char,
                        b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                        1371 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            _glp_spv_linear_comb(
                (*mir).agg_vec,
                -*((*(*mir).agg_vec).val).offset(j as isize)
                    / *((*v).val).offset(jj as isize),
                v,
            );
            _glp_spv_set_vj((*mir).agg_vec, kappa, 0.0f64);
        }
    }
    return ret;
}
pub unsafe extern "C" fn glp_mir_gen(
    mut mip: *mut glp_prob,
    mut mir: *mut glp_mir,
    mut pool: *mut glp_prob,
) -> libc::c_int {
    let mut m: libc::c_int = (*mir).m;
    let mut n: libc::c_int = (*mir).n;
    let mut i: libc::c_int = 0;
    let mut nnn: libc::c_int = 0 as libc::c_int;
    let mut r_best: libc::c_double = 0.;
    let mut work: *mut SPV = 0 as *mut SPV;
    ((*mip).m >= m
        || {
            glp_assert_(
                b"mip->m >= m\0" as *const u8 as *const libc::c_char,
                b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                1393 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*mip).n == n
        || {
            glp_assert_(
                b"mip->n == n\0" as *const u8 as *const libc::c_char,
                b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                1394 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    get_current_point(mip, mir);
    memset(
        &mut *((*mir).subst).offset(1 as libc::c_int as isize) as *mut libc::c_char
            as *mut libc::c_void,
        '?' as i32,
        (m + n) as libc::c_ulong,
    );
    work = _glp_spv_create_vec(m + n);
    i = 1 as libc::c_int;
    while i <= m {
        if !(*((*mir).skip).offset(i as isize) != 0) {
            initial_agg_row(mip, mir, i);
            loop {
                subst_fixed_vars(mir);
                bound_subst_heur(mir);
                build_mod_row(mir);
                r_best = generate(mir);
                if r_best > 0.0f64 {
                    back_subst(mir);
                    subst_aux_vars(mip, mir);
                    add_cut(mir, pool);
                    nnn += 1;
                    nnn;
                }
                let mut j: libc::c_int = 0;
                let mut k: libc::c_int = 0;
                j = 1 as libc::c_int;
                while j <= (*(*mir).mod_vec).nnz {
                    k = *((*(*mir).mod_vec).ind).offset(j as isize);
                    (1 as libc::c_int <= k && k <= m + n
                        || {
                            glp_assert_(
                                b"1 <= k && k <= m+n\0" as *const u8 as *const libc::c_char,
                                b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                                1464 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    (*((*mir).subst).offset(k as isize) as libc::c_int != '?' as i32
                        || {
                            glp_assert_(
                                b"mir->subst[k] != '?'\0" as *const u8
                                    as *const libc::c_char,
                                b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                                1465 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    *((*mir).subst).offset(k as isize) = '?' as i32 as libc::c_char;
                    j += 1;
                    j;
                }
                if !(r_best == 0.0f64) {
                    break;
                }
                if !((*mir).agg_cnt < 5 as libc::c_int) {
                    break;
                }
                if !(aggregate_row(mip, mir, work) == 0 as libc::c_int) {
                    break;
                }
            }
            let mut k_0: libc::c_int = 0;
            let mut ii: libc::c_int = 0;
            k_0 = 1 as libc::c_int;
            while k_0 <= (*mir).agg_cnt {
                ii = *((*mir).agg_row).offset(k_0 as isize);
                (1 as libc::c_int <= ii && ii <= m
                    || {
                        glp_assert_(
                            b"1 <= ii && ii <= m\0" as *const u8 as *const libc::c_char,
                            b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                            1484 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*((*mir).skip).offset(ii as isize) as libc::c_int == 2 as libc::c_int
                    || {
                        glp_assert_(
                            b"mir->skip[ii] == 2\0" as *const u8 as *const libc::c_char,
                            b"intopt/mirgen.c\0" as *const u8 as *const libc::c_char,
                            1485 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                *((*mir).skip).offset(ii as isize) = 0 as libc::c_int as libc::c_char;
                k_0 += 1;
                k_0;
            }
        }
        i += 1;
        i;
    }
    _glp_spv_delete_vec(work);
    return nnn;
}
pub unsafe extern "C" fn glp_mir_free(mut mir: *mut glp_mir) {
    glp_free((*mir).skip as *mut libc::c_void);
    glp_free((*mir).isint as *mut libc::c_void);
    glp_free((*mir).lb as *mut libc::c_void);
    glp_free((*mir).vlb as *mut libc::c_void);
    glp_free((*mir).ub as *mut libc::c_void);
    glp_free((*mir).vub as *mut libc::c_void);
    glp_free((*mir).x as *mut libc::c_void);
    glp_free((*mir).agg_row as *mut libc::c_void);
    _glp_spv_delete_vec((*mir).agg_vec);
    glp_free((*mir).subst as *mut libc::c_void);
    _glp_spv_delete_vec((*mir).mod_vec);
    _glp_spv_delete_vec((*mir).cut_vec);
    glp_free(mir as *mut libc::c_void);
}
