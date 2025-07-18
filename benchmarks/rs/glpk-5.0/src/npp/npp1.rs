use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn glp_set_prob_name(P: *mut glp_prob, name: *const libc::c_char);
    fn glp_set_obj_name(P: *mut glp_prob, name: *const libc::c_char);
    fn glp_set_obj_dir(P: *mut glp_prob, dir: libc::c_int);
    fn glp_add_rows(P: *mut glp_prob, nrs: libc::c_int) -> libc::c_int;
    fn glp_add_cols(P: *mut glp_prob, ncs: libc::c_int) -> libc::c_int;
    fn glp_set_row_name(P: *mut glp_prob, i: libc::c_int, name: *const libc::c_char);
    fn glp_set_col_name(P: *mut glp_prob, j: libc::c_int, name: *const libc::c_char);
    fn glp_set_row_bnds(
        P: *mut glp_prob,
        i: libc::c_int,
        type_0: libc::c_int,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_set_col_bnds(
        P: *mut glp_prob,
        j: libc::c_int,
        type_0: libc::c_int,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_set_obj_coef(P: *mut glp_prob, j: libc::c_int, coef: libc::c_double);
    fn glp_set_mat_col(
        P: *mut glp_prob,
        j: libc::c_int,
        len: libc::c_int,
        ind: *const libc::c_int,
        val: *const libc::c_double,
    );
    fn glp_erase_prob(P: *mut glp_prob);
    fn glp_set_col_kind(P: *mut glp_prob, j: libc::c_int, kind: libc::c_int);
    fn _glp_dmp_delete_pool(pool: *mut DMP);
    fn _glp_dmp_free_atom(pool: *mut DMP, atom: *mut libc::c_void, size: libc::c_int);
    fn _glp_dmp_get_atom(pool: *mut DMP, size: libc::c_int) -> *mut libc::c_void;
    fn _glp_dmp_create_pool() -> *mut DMP;
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
pub struct glp_prep {
    pub orig_dir: libc::c_int,
    pub orig_m: libc::c_int,
    pub orig_n: libc::c_int,
    pub orig_nnz: libc::c_int,
    pub pool: *mut DMP,
    pub name: *mut libc::c_char,
    pub obj: *mut libc::c_char,
    pub c0: libc::c_double,
    pub nrows: libc::c_int,
    pub ncols: libc::c_int,
    pub r_head: *mut NPPROW,
    pub r_tail: *mut NPPROW,
    pub c_head: *mut NPPCOL,
    pub c_tail: *mut NPPCOL,
    pub stack: *mut DMP,
    pub top: *mut NPPTSE,
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub nnz: libc::c_int,
    pub row_ref: *mut libc::c_int,
    pub col_ref: *mut libc::c_int,
    pub sol: libc::c_int,
    pub scaling: libc::c_int,
    pub p_stat: libc::c_int,
    pub d_stat: libc::c_int,
    pub t_stat: libc::c_int,
    pub i_stat: libc::c_int,
    pub r_stat: *mut libc::c_char,
    pub c_stat: *mut libc::c_char,
    pub r_pi: *mut libc::c_double,
    pub c_value: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPTSE {
    pub func: Option::<unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> libc::c_int>,
    pub info: *mut libc::c_void,
    pub link: *mut NPPTSE,
}
pub type NPP = glp_prep;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPCOL {
    pub j: libc::c_int,
    pub name: *mut libc::c_char,
    pub is_int: libc::c_char,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub coef: libc::c_double,
    pub ptr: *mut NPPAIJ,
    pub temp: libc::c_int,
    pub ll: C2RustUnnamed_0,
    pub uu: C2RustUnnamed,
    pub prev: *mut NPPCOL,
    pub next: *mut NPPCOL,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub uu: libc::c_double,
    pub neg: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ll: libc::c_double,
    pub pos: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPAIJ {
    pub row: *mut NPPROW,
    pub col: *mut NPPCOL,
    pub val: libc::c_double,
    pub r_prev: *mut NPPAIJ,
    pub r_next: *mut NPPAIJ,
    pub c_prev: *mut NPPAIJ,
    pub c_next: *mut NPPAIJ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPROW {
    pub i: libc::c_int,
    pub name: *mut libc::c_char,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut NPPAIJ,
    pub temp: libc::c_int,
    pub prev: *mut NPPROW,
    pub next: *mut NPPROW,
}
pub unsafe extern "C" fn _glp_npp_create_wksp() -> *mut NPP {
    let mut npp: *mut NPP = 0 as *mut NPP;
    npp = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<NPP>() as libc::c_ulong as libc::c_int,
    ) as *mut NPP;
    (*npp).orig_dir = 0 as libc::c_int;
    (*npp).orig_nnz = 0 as libc::c_int;
    (*npp).orig_n = (*npp).orig_nnz;
    (*npp).orig_m = (*npp).orig_n;
    (*npp).pool = _glp_dmp_create_pool();
    (*npp).obj = 0 as *mut libc::c_char;
    (*npp).name = (*npp).obj;
    (*npp).c0 = 0.0f64;
    (*npp).ncols = 0 as libc::c_int;
    (*npp).nrows = (*npp).ncols;
    (*npp).r_tail = 0 as *mut NPPROW;
    (*npp).r_head = (*npp).r_tail;
    (*npp).c_tail = 0 as *mut NPPCOL;
    (*npp).c_head = (*npp).c_tail;
    (*npp).stack = _glp_dmp_create_pool();
    (*npp).top = 0 as *mut NPPTSE;
    (*npp).nnz = 0 as libc::c_int;
    (*npp).n = (*npp).nnz;
    (*npp).m = (*npp).n;
    (*npp).col_ref = 0 as *mut libc::c_int;
    (*npp).row_ref = (*npp).col_ref;
    (*npp).scaling = 0 as libc::c_int;
    (*npp).sol = (*npp).scaling;
    (*npp).i_stat = 0 as libc::c_int;
    (*npp).t_stat = (*npp).i_stat;
    (*npp).d_stat = (*npp).t_stat;
    (*npp).p_stat = (*npp).d_stat;
    (*npp).r_stat = 0 as *mut libc::c_char;
    (*npp).r_pi = 0 as *mut libc::c_double;
    (*npp).c_stat = 0 as *mut libc::c_char;
    (*npp).c_value = 0 as *mut libc::c_double;
    return npp;
}
pub unsafe extern "C" fn _glp_npp_insert_row(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
    mut where_0: libc::c_int,
) {
    if where_0 == 0 as libc::c_int {
        (*row).prev = 0 as *mut NPPROW;
        (*row).next = (*npp).r_head;
        if ((*row).next).is_null() {
            (*npp).r_tail = row;
        } else {
            (*(*row).next).prev = row;
        }
        (*npp).r_head = row;
    } else {
        (*row).prev = (*npp).r_tail;
        (*row).next = 0 as *mut NPPROW;
        if ((*row).prev).is_null() {
            (*npp).r_head = row;
        } else {
            (*(*row).prev).next = row;
        }
        (*npp).r_tail = row;
    };
}
pub unsafe extern "C" fn _glp_npp_remove_row(mut npp: *mut NPP, mut row: *mut NPPROW) {
    if ((*row).prev).is_null() {
        (*npp).r_head = (*row).next;
    } else {
        (*(*row).prev).next = (*row).next;
    }
    if ((*row).next).is_null() {
        (*npp).r_tail = (*row).prev;
    } else {
        (*(*row).next).prev = (*row).prev;
    };
}
pub unsafe extern "C" fn _glp_npp_activate_row(mut npp: *mut NPP, mut row: *mut NPPROW) {
    if (*row).temp == 0 {
        (*row).temp = 1 as libc::c_int;
        _glp_npp_remove_row(npp, row);
        _glp_npp_insert_row(npp, row, 0 as libc::c_int);
    }
}
pub unsafe extern "C" fn _glp_npp_deactivate_row(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) {
    if (*row).temp != 0 {
        (*row).temp = 0 as libc::c_int;
        _glp_npp_remove_row(npp, row);
        _glp_npp_insert_row(npp, row, 1 as libc::c_int);
    }
}
pub unsafe extern "C" fn _glp_npp_insert_col(
    mut npp: *mut NPP,
    mut col: *mut NPPCOL,
    mut where_0: libc::c_int,
) {
    if where_0 == 0 as libc::c_int {
        (*col).prev = 0 as *mut NPPCOL;
        (*col).next = (*npp).c_head;
        if ((*col).next).is_null() {
            (*npp).c_tail = col;
        } else {
            (*(*col).next).prev = col;
        }
        (*npp).c_head = col;
    } else {
        (*col).prev = (*npp).c_tail;
        (*col).next = 0 as *mut NPPCOL;
        if ((*col).prev).is_null() {
            (*npp).c_head = col;
        } else {
            (*(*col).prev).next = col;
        }
        (*npp).c_tail = col;
    };
}
pub unsafe extern "C" fn _glp_npp_remove_col(mut npp: *mut NPP, mut col: *mut NPPCOL) {
    if ((*col).prev).is_null() {
        (*npp).c_head = (*col).next;
    } else {
        (*(*col).prev).next = (*col).next;
    }
    if ((*col).next).is_null() {
        (*npp).c_tail = (*col).prev;
    } else {
        (*(*col).next).prev = (*col).prev;
    };
}
pub unsafe extern "C" fn _glp_npp_activate_col(mut npp: *mut NPP, mut col: *mut NPPCOL) {
    if (*col).temp == 0 {
        (*col).temp = 1 as libc::c_int;
        _glp_npp_remove_col(npp, col);
        _glp_npp_insert_col(npp, col, 0 as libc::c_int);
    }
}
pub unsafe extern "C" fn _glp_npp_deactivate_col(
    mut npp: *mut NPP,
    mut col: *mut NPPCOL,
) {
    if (*col).temp != 0 {
        (*col).temp = 0 as libc::c_int;
        _glp_npp_remove_col(npp, col);
        _glp_npp_insert_col(npp, col, 1 as libc::c_int);
    }
}
pub unsafe extern "C" fn _glp_npp_add_row(mut npp: *mut NPP) -> *mut NPPROW {
    let mut row: *mut NPPROW = 0 as *mut NPPROW;
    row = _glp_dmp_get_atom(
        (*npp).pool,
        ::std::mem::size_of::<NPPROW>() as libc::c_ulong as libc::c_int,
    ) as *mut NPPROW;
    (*npp).nrows += 1;
    (*row).i = (*npp).nrows;
    (*row).name = 0 as *mut libc::c_char;
    (*row).lb = -1.7976931348623157e+308f64;
    (*row).ub = 1.7976931348623157e+308f64;
    (*row).ptr = 0 as *mut NPPAIJ;
    (*row).temp = 0 as libc::c_int;
    _glp_npp_insert_row(npp, row, 1 as libc::c_int);
    return row;
}
pub unsafe extern "C" fn _glp_npp_add_col(mut npp: *mut NPP) -> *mut NPPCOL {
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    col = _glp_dmp_get_atom(
        (*npp).pool,
        ::std::mem::size_of::<NPPCOL>() as libc::c_ulong as libc::c_int,
    ) as *mut NPPCOL;
    (*npp).ncols += 1;
    (*col).j = (*npp).ncols;
    (*col).name = 0 as *mut libc::c_char;
    (*col).is_int = 0 as libc::c_int as libc::c_char;
    (*col).coef = 0.0f64;
    (*col).ub = (*col).coef;
    (*col).lb = (*col).ub;
    (*col).ptr = 0 as *mut NPPAIJ;
    (*col).temp = 0 as libc::c_int;
    _glp_npp_insert_col(npp, col, 1 as libc::c_int);
    return col;
}
pub unsafe extern "C" fn _glp_npp_add_aij(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
    mut col: *mut NPPCOL,
    mut val: libc::c_double,
) -> *mut NPPAIJ {
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    aij = _glp_dmp_get_atom(
        (*npp).pool,
        ::std::mem::size_of::<NPPAIJ>() as libc::c_ulong as libc::c_int,
    ) as *mut NPPAIJ;
    (*aij).row = row;
    (*aij).col = col;
    (*aij).val = val;
    (*aij).r_prev = 0 as *mut NPPAIJ;
    (*aij).r_next = (*row).ptr;
    (*aij).c_prev = 0 as *mut NPPAIJ;
    (*aij).c_next = (*col).ptr;
    if !((*aij).r_next).is_null() {
        (*(*aij).r_next).r_prev = aij;
    }
    if !((*aij).c_next).is_null() {
        (*(*aij).c_next).c_prev = aij;
    }
    (*col).ptr = aij;
    (*row).ptr = (*col).ptr;
    return aij;
}
pub unsafe extern "C" fn _glp_npp_row_nnz(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> libc::c_int {
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut nnz: libc::c_int = 0;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const libc::c_char,
                b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                227 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    nnz = 0 as libc::c_int;
    aij = (*row).ptr;
    while !aij.is_null() {
        nnz += 1;
        nnz;
        aij = (*aij).r_next;
    }
    return nnz;
}
pub unsafe extern "C" fn _glp_npp_col_nnz(
    mut npp: *mut NPP,
    mut col: *mut NPPCOL,
) -> libc::c_int {
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut nnz: libc::c_int = 0;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const libc::c_char,
                b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                238 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    nnz = 0 as libc::c_int;
    aij = (*col).ptr;
    while !aij.is_null() {
        nnz += 1;
        nnz;
        aij = (*aij).c_next;
    }
    return nnz;
}
pub unsafe extern "C" fn _glp_npp_push_tse(
    mut npp: *mut NPP,
    mut func: Option::<unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> libc::c_int>,
    mut size: libc::c_int,
) -> *mut libc::c_void {
    let mut tse: *mut NPPTSE = 0 as *mut NPPTSE;
    tse = _glp_dmp_get_atom(
        (*npp).stack,
        ::std::mem::size_of::<NPPTSE>() as libc::c_ulong as libc::c_int,
    ) as *mut NPPTSE;
    (*tse).func = func;
    (*tse).info = _glp_dmp_get_atom((*npp).stack, size);
    (*tse).link = (*npp).top;
    (*npp).top = tse;
    return (*tse).info;
}
pub unsafe extern "C" fn _glp_npp_erase_row(mut npp: *mut NPP, mut row: *mut NPPROW) {
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    while !((*row).ptr).is_null() {
        aij = (*row).ptr;
        (*row).ptr = (*aij).r_next;
        if ((*aij).c_prev).is_null() {
            (*(*aij).col).ptr = (*aij).c_next;
        } else {
            (*(*aij).c_prev).c_next = (*aij).c_next;
        }
        if !((*aij).c_next).is_null() {
            (*(*aij).c_next).c_prev = (*aij).c_prev;
        }
        _glp_dmp_free_atom(
            (*npp).pool,
            aij as *mut libc::c_void,
            ::std::mem::size_of::<NPPAIJ>() as libc::c_ulong as libc::c_int,
        );
    }
}
pub unsafe extern "C" fn _glp_npp_del_row(mut npp: *mut NPP, mut row: *mut NPPROW) {
    if !((*row).name).is_null() {
        _glp_dmp_free_atom(
            (*npp).pool,
            (*row).name as *mut libc::c_void,
            (strlen((*row).name)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_int,
        );
    }
    _glp_npp_erase_row(npp, row);
    _glp_npp_remove_row(npp, row);
    _glp_dmp_free_atom(
        (*npp).pool,
        row as *mut libc::c_void,
        ::std::mem::size_of::<NPPROW>() as libc::c_ulong as libc::c_int,
    );
}
pub unsafe extern "C" fn _glp_npp_del_col(mut npp: *mut NPP, mut col: *mut NPPCOL) {
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    if !((*col).name).is_null() {
        _glp_dmp_free_atom(
            (*npp).pool,
            (*col).name as *mut libc::c_void,
            (strlen((*col).name)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_int,
        );
    }
    while !((*col).ptr).is_null() {
        aij = (*col).ptr;
        (*col).ptr = (*aij).c_next;
        if ((*aij).r_prev).is_null() {
            (*(*aij).row).ptr = (*aij).r_next;
        } else {
            (*(*aij).r_prev).r_next = (*aij).r_next;
        }
        if !((*aij).r_next).is_null() {
            (*(*aij).r_next).r_prev = (*aij).r_prev;
        }
        _glp_dmp_free_atom(
            (*npp).pool,
            aij as *mut libc::c_void,
            ::std::mem::size_of::<NPPAIJ>() as libc::c_ulong as libc::c_int,
        );
    }
    _glp_npp_remove_col(npp, col);
    _glp_dmp_free_atom(
        (*npp).pool,
        col as *mut libc::c_void,
        ::std::mem::size_of::<NPPCOL>() as libc::c_ulong as libc::c_int,
    );
}
pub unsafe extern "C" fn _glp_npp_del_aij(mut npp: *mut NPP, mut aij: *mut NPPAIJ) {
    if ((*aij).r_prev).is_null() {
        (*(*aij).row).ptr = (*aij).r_next;
    } else {
        (*(*aij).r_prev).r_next = (*aij).r_next;
    }
    if !((*aij).r_next).is_null() {
        (*(*aij).r_next).r_prev = (*aij).r_prev;
    }
    if ((*aij).c_prev).is_null() {
        (*(*aij).col).ptr = (*aij).c_next;
    } else {
        (*(*aij).c_prev).c_next = (*aij).c_next;
    }
    if !((*aij).c_next).is_null() {
        (*(*aij).c_next).c_prev = (*aij).c_prev;
    }
    _glp_dmp_free_atom(
        (*npp).pool,
        aij as *mut libc::c_void,
        ::std::mem::size_of::<NPPAIJ>() as libc::c_ulong as libc::c_int,
    );
}
pub unsafe extern "C" fn _glp_npp_load_prob(
    mut npp: *mut NPP,
    mut orig: *mut glp_prob,
    mut names: libc::c_int,
    mut sol: libc::c_int,
    mut scaling: libc::c_int,
) {
    let mut m: libc::c_int = (*orig).m;
    let mut n: libc::c_int = (*orig).n;
    let mut link: *mut *mut NPPROW = 0 as *mut *mut NPPROW;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut dir: libc::c_double = 0.;
    (names == 0 as libc::c_int || names == 1 as libc::c_int
        || {
            glp_assert_(
                b"names == GLP_OFF || names == GLP_ON\0" as *const u8
                    as *const libc::c_char,
                b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                360 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (sol == 1 as libc::c_int || sol == 2 as libc::c_int || sol == 3 as libc::c_int
        || {
            glp_assert_(
                b"sol == GLP_SOL || sol == GLP_IPT || sol == GLP_MIP\0" as *const u8
                    as *const libc::c_char,
                b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                361 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (scaling == 0 as libc::c_int || scaling == 1 as libc::c_int
        || {
            glp_assert_(
                b"scaling == GLP_OFF || scaling == GLP_ON\0" as *const u8
                    as *const libc::c_char,
                b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                362 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if sol == 3 as libc::c_int {
        (scaling == 0
            || {
                glp_assert_(
                    b"!scaling\0" as *const u8 as *const libc::c_char,
                    b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                    363 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    (*npp).orig_dir = (*orig).dir;
    if (*npp).orig_dir == 1 as libc::c_int {
        dir = 1.0f64;
    } else if (*npp).orig_dir == 2 as libc::c_int {
        dir = -1.0f64;
    } else {
        (npp != npp
            || {
                glp_assert_(
                    b"npp != npp\0" as *const u8 as *const libc::c_char,
                    b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                    370 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    (*npp).orig_m = m;
    (*npp).orig_n = n;
    (*npp).orig_nnz = (*orig).nnz;
    if names != 0 && !((*orig).name).is_null() {
        (*npp)
            .name = _glp_dmp_get_atom(
            (*npp).pool,
            (strlen((*orig).name)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_int,
        ) as *mut libc::c_char;
        strcpy((*npp).name, (*orig).name);
    }
    if names != 0 && !((*orig).obj).is_null() {
        (*npp)
            .obj = _glp_dmp_get_atom(
            (*npp).pool,
            (strlen((*orig).obj)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_int,
        ) as *mut libc::c_char;
        strcpy((*npp).obj, (*orig).obj);
    }
    (*npp).c0 = dir * (*orig).c0;
    link = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<*mut NPPROW>() as libc::c_ulong as libc::c_int,
    ) as *mut *mut NPPROW;
    i = 1 as libc::c_int;
    while i <= m {
        let mut rrr: *mut GLPROW = *((*orig).row).offset(i as isize);
        let mut row: *mut NPPROW = 0 as *mut NPPROW;
        row = _glp_npp_add_row(npp);
        let ref mut fresh0 = *link.offset(i as isize);
        *fresh0 = row;
        ((*row).i == i
            || {
                glp_assert_(
                    b"row->i == i\0" as *const u8 as *const libc::c_char,
                    b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                    389 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if names != 0 && !((*rrr).name).is_null() {
            (*row)
                .name = _glp_dmp_get_atom(
                (*npp).pool,
                (strlen((*rrr).name)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                    as libc::c_int,
            ) as *mut libc::c_char;
            strcpy((*row).name, (*rrr).name);
        }
        if scaling == 0 {
            if (*rrr).type_0 == 1 as libc::c_int {
                (*row).lb = -1.7976931348623157e+308f64;
                (*row).ub = 1.7976931348623157e+308f64;
            } else if (*rrr).type_0 == 2 as libc::c_int {
                (*row).lb = (*rrr).lb;
                (*row).ub = 1.7976931348623157e+308f64;
            } else if (*rrr).type_0 == 3 as libc::c_int {
                (*row).lb = -1.7976931348623157e+308f64;
                (*row).ub = (*rrr).ub;
            } else if (*rrr).type_0 == 4 as libc::c_int {
                (*row).lb = (*rrr).lb;
                (*row).ub = (*rrr).ub;
            } else if (*rrr).type_0 == 5 as libc::c_int {
                (*row).ub = (*rrr).lb;
                (*row).lb = (*row).ub;
            } else {
                (rrr != rrr
                    || {
                        glp_assert_(
                            b"rrr != rrr\0" as *const u8 as *const libc::c_char,
                            b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                            406 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        } else {
            let mut rii: libc::c_double = (*rrr).rii;
            if (*rrr).type_0 == 1 as libc::c_int {
                (*row).lb = -1.7976931348623157e+308f64;
                (*row).ub = 1.7976931348623157e+308f64;
            } else if (*rrr).type_0 == 2 as libc::c_int {
                (*row).lb = (*rrr).lb * rii;
                (*row).ub = 1.7976931348623157e+308f64;
            } else if (*rrr).type_0 == 3 as libc::c_int {
                (*row).lb = -1.7976931348623157e+308f64;
                (*row).ub = (*rrr).ub * rii;
            } else if (*rrr).type_0 == 4 as libc::c_int {
                (*row).lb = (*rrr).lb * rii;
                (*row).ub = (*rrr).ub * rii;
            } else if (*rrr).type_0 == 5 as libc::c_int {
                (*row).ub = (*rrr).lb * rii;
                (*row).lb = (*row).ub;
            } else {
                (rrr != rrr
                    || {
                        glp_assert_(
                            b"rrr != rrr\0" as *const u8 as *const libc::c_char,
                            b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                            421 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= n {
        let mut ccc: *mut GLPCOL = *((*orig).col).offset(j as isize);
        let mut aaa: *mut GLPAIJ = 0 as *mut GLPAIJ;
        let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
        col = _glp_npp_add_col(npp);
        ((*col).j == j
            || {
                glp_assert_(
                    b"col->j == j\0" as *const u8 as *const libc::c_char,
                    b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                    430 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if names != 0 && !((*ccc).name).is_null() {
            (*col)
                .name = _glp_dmp_get_atom(
                (*npp).pool,
                (strlen((*ccc).name)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                    as libc::c_int,
            ) as *mut libc::c_char;
            strcpy((*col).name, (*ccc).name);
        }
        if sol == 3 as libc::c_int {
            (*col)
                .is_int = ((*ccc).kind == 2 as libc::c_int) as libc::c_int
                as libc::c_char;
        }
        if scaling == 0 {
            if (*ccc).type_0 == 1 as libc::c_int {
                (*col).lb = -1.7976931348623157e+308f64;
                (*col).ub = 1.7976931348623157e+308f64;
            } else if (*ccc).type_0 == 2 as libc::c_int {
                (*col).lb = (*ccc).lb;
                (*col).ub = 1.7976931348623157e+308f64;
            } else if (*ccc).type_0 == 3 as libc::c_int {
                (*col).lb = -1.7976931348623157e+308f64;
                (*col).ub = (*ccc).ub;
            } else if (*ccc).type_0 == 4 as libc::c_int {
                (*col).lb = (*ccc).lb;
                (*col).ub = (*ccc).ub;
            } else if (*ccc).type_0 == 5 as libc::c_int {
                (*col).ub = (*ccc).lb;
                (*col).lb = (*col).ub;
            } else {
                (ccc != ccc
                    || {
                        glp_assert_(
                            b"ccc != ccc\0" as *const u8 as *const libc::c_char,
                            b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                            453 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            (*col).coef = dir * (*ccc).coef;
            aaa = (*ccc).ptr;
            while !aaa.is_null() {
                _glp_npp_add_aij(
                    npp,
                    *link.offset((*(*aaa).row).i as isize),
                    col,
                    (*aaa).val,
                );
                aaa = (*aaa).c_next;
            }
        } else {
            let mut sjj: libc::c_double = (*ccc).sjj;
            if (*ccc).type_0 == 1 as libc::c_int {
                (*col).lb = -1.7976931348623157e+308f64;
                (*col).ub = 1.7976931348623157e+308f64;
            } else if (*ccc).type_0 == 2 as libc::c_int {
                (*col).lb = (*ccc).lb / sjj;
                (*col).ub = 1.7976931348623157e+308f64;
            } else if (*ccc).type_0 == 3 as libc::c_int {
                (*col).lb = -1.7976931348623157e+308f64;
                (*col).ub = (*ccc).ub / sjj;
            } else if (*ccc).type_0 == 4 as libc::c_int {
                (*col).lb = (*ccc).lb / sjj;
                (*col).ub = (*ccc).ub / sjj;
            } else if (*ccc).type_0 == 5 as libc::c_int {
                (*col).ub = (*ccc).lb / sjj;
                (*col).lb = (*col).ub;
            } else {
                (ccc != ccc
                    || {
                        glp_assert_(
                            b"ccc != ccc\0" as *const u8 as *const libc::c_char,
                            b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                            471 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            (*col).coef = dir * (*ccc).coef * sjj;
            aaa = (*ccc).ptr;
            while !aaa.is_null() {
                _glp_npp_add_aij(
                    npp,
                    *link.offset((*(*aaa).row).i as isize),
                    col,
                    (*(*aaa).row).rii * (*aaa).val * sjj,
                );
                aaa = (*aaa).c_next;
            }
        }
        j += 1;
        j;
    }
    glp_free(link as *mut libc::c_void);
    (*npp).sol = sol;
    (*npp).scaling = scaling;
}
pub unsafe extern "C" fn _glp_npp_build_prob(
    mut npp: *mut NPP,
    mut prob: *mut glp_prob,
) {
    let mut row: *mut NPPROW = 0 as *mut NPPROW;
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut dir: libc::c_double = 0.;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    glp_erase_prob(prob);
    glp_set_prob_name(prob, (*npp).name);
    glp_set_obj_name(prob, (*npp).obj);
    glp_set_obj_dir(prob, (*npp).orig_dir);
    if (*npp).orig_dir == 1 as libc::c_int {
        dir = 1.0f64;
    } else if (*npp).orig_dir == 2 as libc::c_int {
        dir = -1.0f64;
    } else {
        (npp != npp
            || {
                glp_assert_(
                    b"npp != npp\0" as *const u8 as *const libc::c_char,
                    b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                    501 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    glp_set_obj_coef(prob, 0 as libc::c_int, dir * (*npp).c0);
    row = (*npp).r_head;
    while !row.is_null() {
        i = glp_add_rows(prob, 1 as libc::c_int);
        (*row).temp = i;
        glp_set_row_name(prob, i, (*row).name);
        if (*row).lb == -1.7976931348623157e+308f64
            && (*row).ub == 1.7976931348623157e+308f64
        {
            type_0 = 1 as libc::c_int;
        } else if (*row).ub == 1.7976931348623157e+308f64 {
            type_0 = 2 as libc::c_int;
        } else if (*row).lb == -1.7976931348623157e+308f64 {
            type_0 = 3 as libc::c_int;
        } else if (*row).lb != (*row).ub {
            type_0 = 4 as libc::c_int;
        } else {
            type_0 = 5 as libc::c_int;
        }
        glp_set_row_bnds(prob, i, type_0, (*row).lb, (*row).ub);
        row = (*row).next;
    }
    ind = glp_alloc(
        1 as libc::c_int + (*prob).m,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    val = glp_alloc(
        1 as libc::c_int + (*prob).m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    col = (*npp).c_head;
    while !col.is_null() {
        j = glp_add_cols(prob, 1 as libc::c_int);
        glp_set_col_name(prob, j, (*col).name);
        glp_set_col_kind(
            prob,
            j,
            if (*col).is_int as libc::c_int != 0 {
                2 as libc::c_int
            } else {
                1 as libc::c_int
            },
        );
        if (*col).lb == -1.7976931348623157e+308f64
            && (*col).ub == 1.7976931348623157e+308f64
        {
            type_0 = 1 as libc::c_int;
        } else if (*col).ub == 1.7976931348623157e+308f64 {
            type_0 = 2 as libc::c_int;
        } else if (*col).lb == -1.7976931348623157e+308f64 {
            type_0 = 3 as libc::c_int;
        } else if (*col).lb != (*col).ub {
            type_0 = 4 as libc::c_int;
        } else {
            type_0 = 5 as libc::c_int;
        }
        glp_set_col_bnds(prob, j, type_0, (*col).lb, (*col).ub);
        glp_set_obj_coef(prob, j, dir * (*col).coef);
        len = 0 as libc::c_int;
        aij = (*col).ptr;
        while !aij.is_null() {
            len += 1;
            len;
            *ind.offset(len as isize) = (*(*aij).row).temp;
            *val.offset(len as isize) = (*aij).val;
            aij = (*aij).c_next;
        }
        glp_set_mat_col(
            prob,
            j,
            len,
            ind as *const libc::c_int,
            val as *const libc::c_double,
        );
        col = (*col).next;
    }
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
    (*npp).m = (*prob).m;
    (*npp).n = (*prob).n;
    (*npp).nnz = (*prob).nnz;
    (*npp)
        .row_ref = glp_alloc(
        1 as libc::c_int + (*npp).m,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*npp)
        .col_ref = glp_alloc(
        1 as libc::c_int + (*npp).n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    row = (*npp).r_head;
    i = 0 as libc::c_int;
    while !row.is_null() {
        i += 1;
        *((*npp).row_ref).offset(i as isize) = (*row).i;
        row = (*row).next;
    }
    col = (*npp).c_head;
    j = 0 as libc::c_int;
    while !col.is_null() {
        j += 1;
        *((*npp).col_ref).offset(j as isize) = (*col).j;
        col = (*col).next;
    }
    _glp_dmp_delete_pool((*npp).pool);
    (*npp).pool = 0 as *mut DMP;
    (*npp).obj = 0 as *mut libc::c_char;
    (*npp).name = (*npp).obj;
    (*npp).c0 = 0.0f64;
    (*npp).r_tail = 0 as *mut NPPROW;
    (*npp).r_head = (*npp).r_tail;
    (*npp).c_tail = 0 as *mut NPPCOL;
    (*npp).c_head = (*npp).c_tail;
}
pub unsafe extern "C" fn _glp_npp_postprocess(
    mut npp: *mut NPP,
    mut prob: *mut glp_prob,
) {
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut tse: *mut NPPTSE = 0 as *mut NPPTSE;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut dir: libc::c_double = 0.;
    ((*npp).orig_dir == (*prob).dir
        || {
            glp_assert_(
                b"npp->orig_dir == prob->dir\0" as *const u8 as *const libc::c_char,
                b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                578 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*npp).orig_dir == 1 as libc::c_int {
        dir = 1.0f64;
    } else if (*npp).orig_dir == 2 as libc::c_int {
        dir = -1.0f64;
    } else {
        (npp != npp
            || {
                glp_assert_(
                    b"npp != npp\0" as *const u8 as *const libc::c_char,
                    b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                    584 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    if (*npp).sol != 3 as libc::c_int {
        ((*npp).m == (*prob).m
            || {
                glp_assert_(
                    b"npp->m == prob->m\0" as *const u8 as *const libc::c_char,
                    b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                    589 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    ((*npp).n == (*prob).n
        || {
            glp_assert_(
                b"npp->n == prob->n\0" as *const u8 as *const libc::c_char,
                b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                591 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*npp).sol != 3 as libc::c_int {
        ((*npp).nnz == (*prob).nnz
            || {
                glp_assert_(
                    b"npp->nnz == prob->nnz\0" as *const u8 as *const libc::c_char,
                    b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                    596 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    if (*npp).sol == 1 as libc::c_int {
        (*npp).p_stat = (*prob).pbs_stat;
        (*npp).d_stat = (*prob).dbs_stat;
    } else if (*npp).sol == 2 as libc::c_int {
        (*npp).t_stat = (*prob).ipt_stat;
    } else if (*npp).sol == 3 as libc::c_int {
        (*npp).i_stat = (*prob).mip_stat;
    } else {
        (npp != npp
            || {
                glp_assert_(
                    b"npp != npp\0" as *const u8 as *const libc::c_char,
                    b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                    608 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    if (*npp).sol == 1 as libc::c_int {
        if ((*npp).r_stat).is_null() {
            (*npp)
                .r_stat = glp_alloc(
                1 as libc::c_int + (*npp).nrows,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
            ) as *mut libc::c_char;
        }
        i = 1 as libc::c_int;
        while i <= (*npp).nrows {
            *((*npp).r_stat).offset(i as isize) = 0 as libc::c_int as libc::c_char;
            i += 1;
            i;
        }
        if ((*npp).c_stat).is_null() {
            (*npp)
                .c_stat = glp_alloc(
                1 as libc::c_int + (*npp).ncols,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
            ) as *mut libc::c_char;
        }
        j = 1 as libc::c_int;
        while j <= (*npp).ncols {
            *((*npp).c_stat).offset(j as isize) = 0 as libc::c_int as libc::c_char;
            j += 1;
            j;
        }
    }
    if ((*npp).c_value).is_null() {
        (*npp)
            .c_value = glp_alloc(
            1 as libc::c_int + (*npp).ncols,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
    }
    j = 1 as libc::c_int;
    while j <= (*npp).ncols {
        *((*npp).c_value).offset(j as isize) = 1.7976931348623157e+308f64;
        j += 1;
        j;
    }
    if (*npp).sol != 3 as libc::c_int {
        if ((*npp).r_pi).is_null() {
            (*npp)
                .r_pi = glp_alloc(
                1 as libc::c_int + (*npp).nrows,
                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
            ) as *mut libc::c_double;
        }
        i = 1 as libc::c_int;
        while i <= (*npp).nrows {
            *((*npp).r_pi).offset(i as isize) = 1.7976931348623157e+308f64;
            i += 1;
            i;
        }
    }
    if (*npp).sol == 1 as libc::c_int {
        i = 1 as libc::c_int;
        while i <= (*npp).m {
            row = *((*prob).row).offset(i as isize);
            k = *((*npp).row_ref).offset(i as isize);
            *((*npp).r_stat).offset(k as isize) = (*row).stat as libc::c_char;
            *((*npp).r_pi).offset(k as isize) = dir * (*row).dual;
            i += 1;
            i;
        }
        j = 1 as libc::c_int;
        while j <= (*npp).n {
            col = *((*prob).col).offset(j as isize);
            k = *((*npp).col_ref).offset(j as isize);
            *((*npp).c_stat).offset(k as isize) = (*col).stat as libc::c_char;
            *((*npp).c_value).offset(k as isize) = (*col).prim;
            j += 1;
            j;
        }
    } else if (*npp).sol == 2 as libc::c_int {
        i = 1 as libc::c_int;
        while i <= (*npp).m {
            row = *((*prob).row).offset(i as isize);
            k = *((*npp).row_ref).offset(i as isize);
            *((*npp).r_pi).offset(k as isize) = dir * (*row).dval;
            i += 1;
            i;
        }
        j = 1 as libc::c_int;
        while j <= (*npp).n {
            col = *((*prob).col).offset(j as isize);
            k = *((*npp).col_ref).offset(j as isize);
            *((*npp).c_value).offset(k as isize) = (*col).pval;
            j += 1;
            j;
        }
    } else if (*npp).sol == 3 as libc::c_int {
        j = 1 as libc::c_int;
        while j <= (*npp).n {
            col = *((*prob).col).offset(j as isize);
            k = *((*npp).col_ref).offset(j as isize);
            *((*npp).c_value).offset(k as isize) = (*col).mipx;
            j += 1;
            j;
        }
    } else {
        (npp != npp
            || {
                glp_assert_(
                    b"npp != npp\0" as *const u8 as *const libc::c_char,
                    b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                    689 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    tse = (*npp).top;
    while !tse.is_null() {
        (((*tse).func).is_some()
            || {
                glp_assert_(
                    b"tse->func != NULL\0" as *const u8 as *const libc::c_char,
                    b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                    693 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (((*tse).func).unwrap()(npp, (*tse).info) == 0 as libc::c_int
            || {
                glp_assert_(
                    b"tse->func(npp, tse->info) == 0\0" as *const u8
                        as *const libc::c_char,
                    b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                    694 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        tse = (*tse).link;
    }
}
pub unsafe extern "C" fn _glp_npp_unload_sol(
    mut npp: *mut NPP,
    mut orig: *mut glp_prob,
) {
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut dir: libc::c_double = 0.;
    ((*npp).orig_dir == (*orig).dir
        || {
            glp_assert_(
                b"npp->orig_dir == orig->dir\0" as *const u8 as *const libc::c_char,
                b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                705 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*npp).orig_dir == 1 as libc::c_int {
        dir = 1.0f64;
    } else if (*npp).orig_dir == 2 as libc::c_int {
        dir = -1.0f64;
    } else {
        (npp != npp
            || {
                glp_assert_(
                    b"npp != npp\0" as *const u8 as *const libc::c_char,
                    b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                    711 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    ((*npp).orig_m == (*orig).m
        || {
            glp_assert_(
                b"npp->orig_m == orig->m\0" as *const u8 as *const libc::c_char,
                b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                712 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*npp).orig_n == (*orig).n
        || {
            glp_assert_(
                b"npp->orig_n == orig->n\0" as *const u8 as *const libc::c_char,
                b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                713 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*npp).orig_nnz == (*orig).nnz
        || {
            glp_assert_(
                b"npp->orig_nnz == orig->nnz\0" as *const u8 as *const libc::c_char,
                b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                714 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*npp).sol == 1 as libc::c_int {
        (*orig).valid = 0 as libc::c_int;
        (*orig).pbs_stat = (*npp).p_stat;
        (*orig).dbs_stat = (*npp).d_stat;
        (*orig).obj_val = (*orig).c0;
        (*orig).some = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i <= (*orig).m {
            row = *((*orig).row).offset(i as isize);
            (*row).stat = *((*npp).r_stat).offset(i as isize) as libc::c_int;
            if (*npp).scaling == 0 {
                (*row).dual = dir * *((*npp).r_pi).offset(i as isize);
            } else {
                (*row).dual = dir * *((*npp).r_pi).offset(i as isize) * (*row).rii;
            }
            if (*row).stat == 1 as libc::c_int {
                (*row).dual = 0.0f64;
            } else if (*row).stat == 2 as libc::c_int {
                ((*row).type_0 == 2 as libc::c_int || (*row).type_0 == 4 as libc::c_int
                    || {
                        glp_assert_(
                            b"row->type == GLP_LO || row->type == GLP_DB\0" as *const u8
                                as *const libc::c_char,
                            b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                            736 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*row).prim = (*row).lb;
            } else if (*row).stat == 3 as libc::c_int {
                ((*row).type_0 == 3 as libc::c_int || (*row).type_0 == 4 as libc::c_int
                    || {
                        glp_assert_(
                            b"row->type == GLP_UP || row->type == GLP_DB\0" as *const u8
                                as *const libc::c_char,
                            b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                            740 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*row).prim = (*row).ub;
            } else if (*row).stat == 4 as libc::c_int {
                ((*row).type_0 == 1 as libc::c_int
                    || {
                        glp_assert_(
                            b"row->type == GLP_FR\0" as *const u8 as *const libc::c_char,
                            b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                            744 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*row).prim = 0.0f64;
            } else if (*row).stat == 5 as libc::c_int {
                ((*row).type_0 == 5 as libc::c_int
                    || {
                        glp_assert_(
                            b"row->type == GLP_FX\0" as *const u8 as *const libc::c_char,
                            b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                            748 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*row).prim = (*row).lb;
            } else {
                (row != row
                    || {
                        glp_assert_(
                            b"row != row\0" as *const u8 as *const libc::c_char,
                            b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                            752 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            i += 1;
            i;
        }
        j = 1 as libc::c_int;
        while j <= (*orig).n {
            col = *((*orig).col).offset(j as isize);
            (*col).stat = *((*npp).c_stat).offset(j as isize) as libc::c_int;
            if (*npp).scaling == 0 {
                (*col).prim = *((*npp).c_value).offset(j as isize);
            } else {
                (*col).prim = *((*npp).c_value).offset(j as isize) * (*col).sjj;
            }
            if (*col).stat == 1 as libc::c_int {
                (*col).dual = 0.0f64;
            } else if (*col).stat == 2 as libc::c_int {
                ((*col).type_0 == 2 as libc::c_int || (*col).type_0 == 4 as libc::c_int
                    || {
                        glp_assert_(
                            b"col->type == GLP_LO || col->type == GLP_DB\0" as *const u8
                                as *const libc::c_char,
                            b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                            769 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*col).prim = (*col).lb;
            } else if (*col).stat == 3 as libc::c_int {
                ((*col).type_0 == 3 as libc::c_int || (*col).type_0 == 4 as libc::c_int
                    || {
                        glp_assert_(
                            b"col->type == GLP_UP || col->type == GLP_DB\0" as *const u8
                                as *const libc::c_char,
                            b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                            773 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*col).prim = (*col).ub;
            } else if (*col).stat == 4 as libc::c_int {
                ((*col).type_0 == 1 as libc::c_int
                    || {
                        glp_assert_(
                            b"col->type == GLP_FR\0" as *const u8 as *const libc::c_char,
                            b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                            777 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*col).prim = 0.0f64;
            } else if (*col).stat == 5 as libc::c_int {
                ((*col).type_0 == 5 as libc::c_int
                    || {
                        glp_assert_(
                            b"col->type == GLP_FX\0" as *const u8 as *const libc::c_char,
                            b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                            781 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*col).prim = (*col).lb;
            } else {
                (col != col
                    || {
                        glp_assert_(
                            b"col != col\0" as *const u8 as *const libc::c_char,
                            b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                            785 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            (*orig).obj_val += (*col).coef * (*col).prim;
            j += 1;
            j;
        }
        i = 1 as libc::c_int;
        while i <= (*orig).m {
            row = *((*orig).row).offset(i as isize);
            if (*row).stat == 1 as libc::c_int {
                let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
                let mut temp: libc::c_double = 0.;
                temp = 0.0f64;
                aij = (*row).ptr;
                while !aij.is_null() {
                    temp += (*aij).val * (*(*aij).col).prim;
                    aij = (*aij).r_next;
                }
                (*row).prim = temp;
            }
            i += 1;
            i;
        }
        j = 1 as libc::c_int;
        while j <= (*orig).n {
            col = *((*orig).col).offset(j as isize);
            if (*col).stat != 1 as libc::c_int {
                let mut aij_0: *mut GLPAIJ = 0 as *mut GLPAIJ;
                let mut temp_0: libc::c_double = 0.;
                temp_0 = (*col).coef;
                aij_0 = (*col).ptr;
                while !aij_0.is_null() {
                    temp_0 -= (*aij_0).val * (*(*aij_0).row).dual;
                    aij_0 = (*aij_0).c_next;
                }
                (*col).dual = temp_0;
            }
            j += 1;
            j;
        }
    } else if (*npp).sol == 2 as libc::c_int {
        (*orig).ipt_stat = (*npp).t_stat;
        (*orig).ipt_obj = (*orig).c0;
        i = 1 as libc::c_int;
        while i <= (*orig).m {
            row = *((*orig).row).offset(i as isize);
            if (*npp).scaling == 0 {
                (*row).dval = dir * *((*npp).r_pi).offset(i as isize);
            } else {
                (*row).dval = dir * *((*npp).r_pi).offset(i as isize) * (*row).rii;
            }
            i += 1;
            i;
        }
        j = 1 as libc::c_int;
        while j <= (*orig).n {
            col = *((*orig).col).offset(j as isize);
            if (*npp).scaling == 0 {
                (*col).pval = *((*npp).c_value).offset(j as isize);
            } else {
                (*col).pval = *((*npp).c_value).offset(j as isize) * (*col).sjj;
            }
            (*orig).ipt_obj += (*col).coef * (*col).pval;
            j += 1;
            j;
        }
        i = 1 as libc::c_int;
        while i <= (*orig).m {
            row = *((*orig).row).offset(i as isize);
            let mut aij_1: *mut GLPAIJ = 0 as *mut GLPAIJ;
            let mut temp_1: libc::c_double = 0.;
            temp_1 = 0.0f64;
            aij_1 = (*row).ptr;
            while !aij_1.is_null() {
                temp_1 += (*aij_1).val * (*(*aij_1).col).pval;
                aij_1 = (*aij_1).r_next;
            }
            (*row).pval = temp_1;
            i += 1;
            i;
        }
        j = 1 as libc::c_int;
        while j <= (*orig).n {
            col = *((*orig).col).offset(j as isize);
            let mut aij_2: *mut GLPAIJ = 0 as *mut GLPAIJ;
            let mut temp_2: libc::c_double = 0.;
            temp_2 = (*col).coef;
            aij_2 = (*col).ptr;
            while !aij_2.is_null() {
                temp_2 -= (*aij_2).val * (*(*aij_2).row).dval;
                aij_2 = (*aij_2).c_next;
            }
            (*col).dval = temp_2;
            j += 1;
            j;
        }
    } else if (*npp).sol == 3 as libc::c_int {
        ((*npp).scaling == 0
            || {
                glp_assert_(
                    b"!npp->scaling\0" as *const u8 as *const libc::c_char,
                    b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                    870 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*orig).mip_stat = (*npp).i_stat;
        (*orig).mip_obj = (*orig).c0;
        j = 1 as libc::c_int;
        while j <= (*orig).n {
            col = *((*orig).col).offset(j as isize);
            (*col).mipx = *((*npp).c_value).offset(j as isize);
            if (*col).kind == 2 as libc::c_int {
                ((*col).mipx == floor((*col).mipx)
                    || {
                        glp_assert_(
                            b"col->mipx == floor(col->mipx)\0" as *const u8
                                as *const libc::c_char,
                            b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                            883 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            (*orig).mip_obj += (*col).coef * (*col).mipx;
            j += 1;
            j;
        }
        i = 1 as libc::c_int;
        while i <= (*orig).m {
            row = *((*orig).row).offset(i as isize);
            let mut aij_3: *mut GLPAIJ = 0 as *mut GLPAIJ;
            let mut temp_3: libc::c_double = 0.;
            temp_3 = 0.0f64;
            aij_3 = (*row).ptr;
            while !aij_3.is_null() {
                temp_3 += (*aij_3).val * (*(*aij_3).col).mipx;
                aij_3 = (*aij_3).r_next;
            }
            (*row).mipx = temp_3;
            i += 1;
            i;
        }
    } else {
        (npp != npp
            || {
                glp_assert_(
                    b"npp != npp\0" as *const u8 as *const libc::c_char,
                    b"npp/npp1.c\0" as *const u8 as *const libc::c_char,
                    901 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    };
}
pub unsafe extern "C" fn _glp_npp_delete_wksp(mut npp: *mut NPP) {
    if !((*npp).pool).is_null() {
        _glp_dmp_delete_pool((*npp).pool);
    }
    if !((*npp).stack).is_null() {
        _glp_dmp_delete_pool((*npp).stack);
    }
    if !((*npp).row_ref).is_null() {
        glp_free((*npp).row_ref as *mut libc::c_void);
    }
    if !((*npp).col_ref).is_null() {
        glp_free((*npp).col_ref as *mut libc::c_void);
    }
    if !((*npp).r_stat).is_null() {
        glp_free((*npp).r_stat as *mut libc::c_void);
    }
    if !((*npp).r_pi).is_null() {
        glp_free((*npp).r_pi as *mut libc::c_void);
    }
    if !((*npp).c_stat).is_null() {
        glp_free((*npp).c_stat as *mut libc::c_void);
    }
    if !((*npp).c_value).is_null() {
        glp_free((*npp).c_value as *mut libc::c_void);
    }
    glp_free(npp as *mut libc::c_void);
}
