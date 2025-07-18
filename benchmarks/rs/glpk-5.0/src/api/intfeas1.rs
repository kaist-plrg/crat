use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_create_prob() -> *mut glp_prob;
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
    fn glp_del_rows(P: *mut glp_prob, nrs: libc::c_int, num: *const libc::c_int);
    fn glp_delete_prob(P: *mut glp_prob);
    fn glp_minisat1(P: *mut glp_prob) -> libc::c_int;
    fn _glp_npp_delete_wksp(npp: *mut NPP);
    fn _glp_npp_unload_sol(npp: *mut NPP, orig: *mut glp_prob);
    fn _glp_npp_postprocess(npp: *mut NPP, prob: *mut glp_prob);
    fn _glp_npp_build_prob(npp: *mut NPP, prob: *mut glp_prob);
    fn _glp_npp_sat_encode_prob(npp: *mut NPP) -> libc::c_int;
    fn _glp_npp_load_prob(
        npp: *mut NPP,
        orig: *mut glp_prob,
        names: libc::c_int,
        sol: libc::c_int,
        scaling: libc::c_int,
    );
    fn _glp_npp_create_wksp() -> *mut NPP;
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
pub unsafe extern "C" fn glp_intfeas1(
    mut P: *mut glp_prob,
    mut use_bound: libc::c_int,
    mut obj_bound: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut npp: *mut NPP = 0 as *mut NPP;
    let mut mip: *mut glp_prob = 0 as *mut glp_prob;
    let mut obj_ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut obj_val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut obj_row: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut obj_len: libc::c_int = 0;
    let mut temp: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if !((*P).tree).is_null() {
        (glp_error_(
            b"api/intfeas1.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
        ))
            .unwrap()(
            b"glp_intfeas1: operation not allowed\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*P).mip_stat = 1 as libc::c_int;
    (*P).mip_obj = 0.0f64;
    j = 1 as libc::c_int;
    loop {
        if !(j <= (*P).n) {
            current_block = 15089075282327824602;
            break;
        }
        let mut col: *mut GLPCOL = *((*P).col).offset(j as isize);
        if !((*col).kind == 2 as libc::c_int && (*col).lb == 0.0f64
            && (*col).ub == 1.0f64 || (*col).type_0 == 5 as libc::c_int)
        {
            glp_printf(
                b"glp_intfeas1: column %d: non-binary non-fixed variable not allowed\n\0"
                    as *const u8 as *const libc::c_char,
                j,
            );
            ret = 0x12 as libc::c_int;
            current_block = 9034745588010668110;
            break;
        } else {
            temp = (*col).lb as libc::c_int;
            if temp as libc::c_double != (*col).lb {
                if (*col).type_0 == 5 as libc::c_int {
                    glp_printf(
                        b"glp_intfeas1: column %d: fixed value %g is non-integer or out of range\n\0"
                            as *const u8 as *const libc::c_char,
                        j,
                        (*col).lb,
                    );
                } else {
                    glp_printf(
                        b"glp_intfeas1: column %d: lower bound %g is non-integer or out of range\n\0"
                            as *const u8 as *const libc::c_char,
                        j,
                        (*col).lb,
                    );
                }
                ret = 0x12 as libc::c_int;
                current_block = 9034745588010668110;
                break;
            } else {
                temp = (*col).ub as libc::c_int;
                if temp as libc::c_double != (*col).ub {
                    glp_printf(
                        b"glp_intfeas1: column %d: upper bound %g is non-integer or out of range\n\0"
                            as *const u8 as *const libc::c_char,
                        j,
                        (*col).ub,
                    );
                    ret = 0x12 as libc::c_int;
                    current_block = 9034745588010668110;
                    break;
                } else if (*col).type_0 == 4 as libc::c_int && (*col).lb > (*col).ub {
                    glp_printf(
                        b"glp_intfeas1: column %d: lower bound %g is greater than upper bound %g\n\0"
                            as *const u8 as *const libc::c_char,
                        j,
                        (*col).lb,
                        (*col).ub,
                    );
                    ret = 0x4 as libc::c_int;
                    current_block = 9034745588010668110;
                    break;
                } else {
                    j += 1;
                    j;
                }
            }
        }
    }
    match current_block {
        15089075282327824602 => {
            i = 1 as libc::c_int;
            's_110: loop {
                if !(i <= (*P).m) {
                    current_block = 17784502470059252271;
                    break;
                }
                let mut row: *mut GLPROW = *((*P).row).offset(i as isize);
                let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
                aij = (*row).ptr;
                while !aij.is_null() {
                    temp = (*aij).val as libc::c_int;
                    if temp as libc::c_double != (*aij).val {
                        glp_printf(
                            b"glp_intfeas1: row = %d, column %d: constraint coefficient %g is non-integer or out of range\n\0"
                                as *const u8 as *const libc::c_char,
                            i,
                            (*(*aij).col).j,
                            (*aij).val,
                        );
                        ret = 0x12 as libc::c_int;
                        current_block = 9034745588010668110;
                        break 's_110;
                    } else {
                        aij = (*aij).r_next;
                    }
                }
                temp = (*row).lb as libc::c_int;
                if temp as libc::c_double != (*row).lb {
                    if (*row).type_0 == 5 as libc::c_int {
                        glp_printf(
                            b"glp_intfeas1: row = %d: fixed value %g is non-integer or out of range\n\0"
                                as *const u8 as *const libc::c_char,
                            i,
                            (*row).lb,
                        );
                    } else {
                        glp_printf(
                            b"glp_intfeas1: row = %d: lower bound %g is non-integer or out of range\n\0"
                                as *const u8 as *const libc::c_char,
                            i,
                            (*row).lb,
                        );
                    }
                    ret = 0x12 as libc::c_int;
                    current_block = 9034745588010668110;
                    break;
                } else {
                    temp = (*row).ub as libc::c_int;
                    if temp as libc::c_double != (*row).ub {
                        glp_printf(
                            b"glp_intfeas1: row = %d: upper bound %g is non-integer or out of range\n\0"
                                as *const u8 as *const libc::c_char,
                            i,
                            (*row).ub,
                        );
                        ret = 0x12 as libc::c_int;
                        current_block = 9034745588010668110;
                        break;
                    } else if (*row).type_0 == 4 as libc::c_int && (*row).lb > (*row).ub
                    {
                        glp_printf(
                            b"glp_intfeas1: row %d: lower bound %g is greater than upper bound %g\n\0"
                                as *const u8 as *const libc::c_char,
                            i,
                            (*row).lb,
                            (*row).ub,
                        );
                        ret = 0x4 as libc::c_int;
                        current_block = 9034745588010668110;
                        break;
                    } else {
                        i += 1;
                        i;
                    }
                }
            }
            match current_block {
                9034745588010668110 => {}
                _ => {
                    if use_bound == 0 {
                        current_block = 13321564401369230990;
                    } else {
                        temp = (*P).c0 as libc::c_int;
                        if temp as libc::c_double != (*P).c0 {
                            glp_printf(
                                b"glp_intfeas1: objective constant term %g is non-integer or out of range\n\0"
                                    as *const u8 as *const libc::c_char,
                                (*P).c0,
                            );
                            ret = 0x12 as libc::c_int;
                            current_block = 9034745588010668110;
                        } else {
                            j = 1 as libc::c_int;
                            loop {
                                if !(j <= (*P).n) {
                                    current_block = 13321564401369230990;
                                    break;
                                }
                                temp = (**((*P).col).offset(j as isize)).coef
                                    as libc::c_int;
                                if temp as libc::c_double
                                    != (**((*P).col).offset(j as isize)).coef
                                {
                                    glp_printf(
                                        b"glp_intfeas1: column %d: objective coefficient is non-integer or out of range\n\0"
                                            as *const u8 as *const libc::c_char,
                                        j,
                                        (**((*P).col).offset(j as isize)).coef,
                                    );
                                    ret = 0x12 as libc::c_int;
                                    current_block = 9034745588010668110;
                                    break;
                                } else {
                                    j += 1;
                                    j;
                                }
                            }
                        }
                    }
                    match current_block {
                        9034745588010668110 => {}
                        _ => {
                            obj_ind = glp_alloc(
                                1 as libc::c_int + (*P).n,
                                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    as libc::c_int,
                            ) as *mut libc::c_int;
                            obj_val = glp_alloc(
                                1 as libc::c_int + (*P).n,
                                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong
                                    as libc::c_int,
                            ) as *mut libc::c_double;
                            obj_len = 0 as libc::c_int;
                            *obj_ind
                                .offset(0 as libc::c_int as isize) = 0 as libc::c_int;
                            *obj_val.offset(0 as libc::c_int as isize) = (*P).c0;
                            (*P).c0 = 0.0f64;
                            j = 1 as libc::c_int;
                            while j <= (*P).n {
                                if (**((*P).col).offset(j as isize)).coef != 0.0f64 {
                                    obj_len += 1;
                                    obj_len;
                                    *obj_ind.offset(obj_len as isize) = j;
                                    *obj_val
                                        .offset(
                                            obj_len as isize,
                                        ) = (**((*P).col).offset(j as isize)).coef;
                                    (**((*P).col).offset(j as isize)).coef = 0.0f64;
                                }
                                j += 1;
                                j;
                            }
                            if use_bound == 0 {
                                glp_printf(
                                    b"Will search for ANY feasible solution\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                            } else {
                                glp_printf(
                                    b"Will search only for solution not worse than %d\n\0"
                                        as *const u8 as *const libc::c_char,
                                    obj_bound,
                                );
                                obj_row = glp_add_rows(P, 1 as libc::c_int);
                                glp_set_mat_row(
                                    P,
                                    obj_row,
                                    obj_len,
                                    obj_ind as *const libc::c_int,
                                    obj_val as *const libc::c_double,
                                );
                                if (*P).dir == 1 as libc::c_int {
                                    glp_set_row_bnds(
                                        P,
                                        obj_row,
                                        3 as libc::c_int,
                                        0.0f64,
                                        obj_bound as libc::c_double
                                            - *obj_val.offset(0 as libc::c_int as isize),
                                    );
                                } else if (*P).dir == 2 as libc::c_int {
                                    glp_set_row_bnds(
                                        P,
                                        obj_row,
                                        2 as libc::c_int,
                                        obj_bound as libc::c_double
                                            - *obj_val.offset(0 as libc::c_int as isize),
                                        0.0f64,
                                    );
                                } else {
                                    (P != P
                                        || {
                                            glp_assert_(
                                                b"P != P\0" as *const u8 as *const libc::c_char,
                                                b"api/intfeas1.c\0" as *const u8 as *const libc::c_char,
                                                180 as libc::c_int,
                                            );
                                            1 as libc::c_int != 0
                                        }) as libc::c_int;
                                }
                            }
                            glp_printf(
                                b"Translating to CNF-SAT...\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            glp_printf(
                                b"Original problem has %d row%s, %d column%s, and %d non-zero%s\n\0"
                                    as *const u8 as *const libc::c_char,
                                (*P).m,
                                if (*P).m == 1 as libc::c_int {
                                    b"\0" as *const u8 as *const libc::c_char
                                } else {
                                    b"s\0" as *const u8 as *const libc::c_char
                                },
                                (*P).n,
                                if (*P).n == 1 as libc::c_int {
                                    b"\0" as *const u8 as *const libc::c_char
                                } else {
                                    b"s\0" as *const u8 as *const libc::c_char
                                },
                                (*P).nnz,
                                if (*P).nnz == 1 as libc::c_int {
                                    b"\0" as *const u8 as *const libc::c_char
                                } else {
                                    b"s\0" as *const u8 as *const libc::c_char
                                },
                            );
                            npp = _glp_npp_create_wksp();
                            _glp_npp_load_prob(
                                npp,
                                P,
                                0 as libc::c_int,
                                3 as libc::c_int,
                                0 as libc::c_int,
                            );
                            ret = _glp_npp_sat_encode_prob(npp);
                            if !(ret == 0 as libc::c_int) {
                                if ret == 0xa as libc::c_int {
                                    glp_printf(
                                        b"PROBLEM HAS NO INTEGER FEASIBLE SOLUTION\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                } else if ret == 0x13 as libc::c_int {
                                    glp_printf(
                                        b"glp_intfeas1: translation to SAT-CNF failed because of integer overflow\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                } else {
                                    (ret != ret
                                        || {
                                            glp_assert_(
                                                b"ret != ret\0" as *const u8 as *const libc::c_char,
                                                b"api/intfeas1.c\0" as *const u8 as *const libc::c_char,
                                                200 as libc::c_int,
                                            );
                                            1 as libc::c_int != 0
                                        }) as libc::c_int;
                                }
                            }
                            if !(ret != 0 as libc::c_int) {
                                mip = glp_create_prob();
                                _glp_npp_build_prob(npp, mip);
                                ret = glp_minisat1(mip);
                                if !((*mip).mip_stat == 5 as libc::c_int
                                    || (*mip).mip_stat == 2 as libc::c_int)
                                {
                                    (*P).mip_stat = (*mip).mip_stat;
                                } else {
                                    _glp_npp_postprocess(npp, mip);
                                    glp_delete_prob(mip);
                                    mip = 0 as *mut glp_prob;
                                    _glp_npp_unload_sol(npp, P);
                                    (*P).mip_stat = 2 as libc::c_int;
                                    i = 1 as libc::c_int;
                                    while i <= (*P).m {
                                        let mut row_0: *mut GLPROW = 0 as *mut GLPROW;
                                        let mut aij_0: *mut GLPAIJ = 0 as *mut GLPAIJ;
                                        let mut sum: libc::c_double = 0.;
                                        row_0 = *((*P).row).offset(i as isize);
                                        sum = 0.0f64;
                                        aij_0 = (*row_0).ptr;
                                        while !aij_0.is_null() {
                                            sum += (*aij_0).val * (*(*aij_0).col).mipx;
                                            aij_0 = (*aij_0).r_next;
                                        }
                                        (sum == (*row_0).mipx
                                            || {
                                                glp_assert_(
                                                    b"sum == row->mipx\0" as *const u8 as *const libc::c_char,
                                                    b"api/intfeas1.c\0" as *const u8 as *const libc::c_char,
                                                    229 as libc::c_int,
                                                );
                                                1 as libc::c_int != 0
                                            }) as libc::c_int;
                                        if (*row_0).type_0 == 2 as libc::c_int
                                            || (*row_0).type_0 == 4 as libc::c_int
                                            || (*row_0).type_0 == 5 as libc::c_int
                                        {
                                            (sum >= (*row_0).lb
                                                || {
                                                    glp_assert_(
                                                        b"sum >= row->lb\0" as *const u8 as *const libc::c_char,
                                                        b"api/intfeas1.c\0" as *const u8 as *const libc::c_char,
                                                        232 as libc::c_int,
                                                    );
                                                    1 as libc::c_int != 0
                                                }) as libc::c_int;
                                        }
                                        if (*row_0).type_0 == 3 as libc::c_int
                                            || (*row_0).type_0 == 4 as libc::c_int
                                            || (*row_0).type_0 == 5 as libc::c_int
                                        {
                                            (sum <= (*row_0).ub
                                                || {
                                                    glp_assert_(
                                                        b"sum <= row->ub\0" as *const u8 as *const libc::c_char,
                                                        b"api/intfeas1.c\0" as *const u8 as *const libc::c_char,
                                                        235 as libc::c_int,
                                                    );
                                                    1 as libc::c_int != 0
                                                }) as libc::c_int;
                                        }
                                        i += 1;
                                        i;
                                    }
                                    (*P).mip_obj = *obj_val.offset(0 as libc::c_int as isize);
                                    k = 1 as libc::c_int;
                                    while k <= obj_len {
                                        (*P).mip_obj
                                            += *obj_val.offset(k as isize)
                                                * (**((*P).col)
                                                    .offset(*obj_ind.offset(k as isize) as isize))
                                                    .mipx;
                                        k += 1;
                                        k;
                                    }
                                    glp_printf(
                                        b"Objective value = %17.9e\n\0" as *const u8
                                            as *const libc::c_char,
                                        (*P).mip_obj,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if !mip.is_null() {
        glp_delete_prob(mip);
    }
    if !npp.is_null() {
        _glp_npp_delete_wksp(npp);
    }
    if obj_row > 0 as libc::c_int {
        let mut ind: [libc::c_int; 2] = [0; 2];
        ind[1 as libc::c_int as usize] = obj_row;
        glp_del_rows(P, 1 as libc::c_int, ind.as_mut_ptr() as *const libc::c_int);
    }
    if !obj_ind.is_null() {
        (*P).c0 = *obj_val.offset(0 as libc::c_int as isize);
        k = 1 as libc::c_int;
        while k <= obj_len {
            (**((*P).col).offset(*obj_ind.offset(k as isize) as isize))
                .coef = *obj_val.offset(k as isize);
            k += 1;
            k;
        }
        glp_free(obj_ind as *mut libc::c_void);
        glp_free(obj_val as *mut libc::c_void);
    }
    return ret;
}
