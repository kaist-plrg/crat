use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_get_status(P: *mut glp_prob) -> libc::c_int;
    fn glp_ipt_status(P: *mut glp_prob) -> libc::c_int;
    fn glp_mip_status(P: *mut glp_prob) -> libc::c_int;
    fn _glp_npp_delete_wksp(npp: *mut NPP);
    fn _glp_npp_unload_sol(npp: *mut NPP, orig: *mut glp_prob);
    fn _glp_npp_create_wksp() -> *mut NPP;
    fn _glp_npp_postprocess(npp: *mut NPP, prob: *mut glp_prob);
    fn _glp_npp_load_prob(
        npp: *mut NPP,
        orig: *mut glp_prob,
        names: libc::c_int,
        sol: libc::c_int,
        scaling: libc::c_int,
    );
    fn _glp_npp_build_prob(npp: *mut NPP, prob: *mut glp_prob);
    fn _glp_npp_process_prob(npp: *mut NPP, hard: libc::c_int) -> libc::c_int;
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
pub unsafe extern "C" fn glp_npp_alloc_wksp() -> *mut glp_prep {
    let mut prep: *mut glp_prep = 0 as *mut glp_prep;
    prep = _glp_npp_create_wksp();
    return prep;
}
pub unsafe extern "C" fn glp_npp_load_prob(
    mut prep: *mut glp_prep,
    mut P: *mut glp_prob,
    mut sol: libc::c_int,
    mut names: libc::c_int,
) {
    if (*prep).sol != 0 as libc::c_int {
        (glp_error_(
            b"api/npp.c\0" as *const u8 as *const libc::c_char,
            35 as libc::c_int,
        ))
            .unwrap()(
            b"glp_npp_load_prob: invalid call sequence (original instance already loaded)\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if !(sol == 1 as libc::c_int || sol == 2 as libc::c_int || sol == 3 as libc::c_int) {
        (glp_error_(
            b"api/npp.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int,
        ))
            .unwrap()(
            b"glp_npp_load_prob: sol = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            sol,
        );
    }
    if !(names == 1 as libc::c_int || names == 0 as libc::c_int) {
        (glp_error_(
            b"api/npp.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
        ))
            .unwrap()(
            b"glp_npp_load_prob: names = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            names,
        );
    }
    _glp_npp_load_prob(prep, P, names, sol, 0 as libc::c_int);
}
pub unsafe extern "C" fn glp_npp_preprocess1(
    mut prep: *mut glp_prep,
    mut hard: libc::c_int,
) -> libc::c_int {
    if (*prep).sol == 0 as libc::c_int {
        (glp_error_(
            b"api/npp.c\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int,
        ))
            .unwrap()(
            b"glp_npp_preprocess1: invalid call sequence (original instance not loaded yet)\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if ((*prep).pool).is_null() {
        (glp_error_(
            b"api/npp.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
        ))
            .unwrap()(
            b"glp_npp_preprocess1: invalid call sequence (preprocessing already finished)\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if !(hard == 1 as libc::c_int || hard == 0 as libc::c_int) {
        (glp_error_(
            b"api/npp.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int,
        ))
            .unwrap()(
            b"glp_npp_preprocess1: hard = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            hard,
        );
    }
    return _glp_npp_process_prob(prep, hard);
}
pub unsafe extern "C" fn glp_npp_build_prob(
    mut prep: *mut glp_prep,
    mut Q: *mut glp_prob,
) {
    if (*prep).sol == 0 as libc::c_int {
        (glp_error_(
            b"api/npp.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
        ))
            .unwrap()(
            b"glp_npp_build_prob: invalid call sequence (original instance not loaded yet)\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if ((*prep).pool).is_null() {
        (glp_error_(
            b"api/npp.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int,
        ))
            .unwrap()(
            b"glp_npp_build_prob: invalid call sequence (resultant instance already built)\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    _glp_npp_build_prob(prep, Q);
}
pub unsafe extern "C" fn glp_npp_postprocess(
    mut prep: *mut glp_prep,
    mut Q: *mut glp_prob,
) {
    if !((*prep).pool).is_null() {
        (glp_error_(
            b"api/npp.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int,
        ))
            .unwrap()(
            b"glp_npp_postprocess: invalid call sequence (resultant instance not built yet)\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if !((*prep).m == (*Q).m && (*prep).n == (*Q).n && (*prep).nnz == (*Q).nnz) {
        (glp_error_(
            b"api/npp.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
        ))
            .unwrap()(
            b"glp_npp_postprocess: resultant instance mismatch\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    match (*prep).sol {
        1 => {
            if glp_get_status(Q) != 5 as libc::c_int {
                (glp_error_(
                    b"api/npp.c\0" as *const u8 as *const libc::c_char,
                    83 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_npp_postprocess: unable to recover non-optimal basic solution\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        }
        2 => {
            if glp_ipt_status(Q) != 5 as libc::c_int {
                (glp_error_(
                    b"api/npp.c\0" as *const u8 as *const libc::c_char,
                    88 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_npp_postprocess: unable to recover non-optimal interior-point solution\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        }
        3 => {
            if !(glp_mip_status(Q) == 5 as libc::c_int
                || glp_mip_status(Q) == 2 as libc::c_int)
            {
                (glp_error_(
                    b"api/npp.c\0" as *const u8 as *const libc::c_char,
                    94 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_npp_postprocess: unable to recover integer non-feasible solution\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        }
        _ => {
            (prep != prep
                || {
                    glp_assert_(
                        b"prep != prep\0" as *const u8 as *const libc::c_char,
                        b"api/npp.c\0" as *const u8 as *const libc::c_char,
                        98 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    _glp_npp_postprocess(prep, Q);
}
pub unsafe extern "C" fn glp_npp_obtain_sol(
    mut prep: *mut glp_prep,
    mut P: *mut glp_prob,
) {
    if !((*prep).pool).is_null() {
        (glp_error_(
            b"api/npp.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int,
        ))
            .unwrap()(
            b"glp_npp_obtain_sol: invalid call sequence (resultant instance not built yet)\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    match (*prep).sol {
        1 => {
            if (*prep).p_stat == 0 as libc::c_int || (*prep).d_stat == 0 as libc::c_int {
                (glp_error_(
                    b"api/npp.c\0" as *const u8 as *const libc::c_char,
                    112 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_npp_obtain_sol: invalid call sequence (basic solution not provided yet)\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        }
        2 => {
            if (*prep).t_stat == 0 as libc::c_int {
                (glp_error_(
                    b"api/npp.c\0" as *const u8 as *const libc::c_char,
                    117 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_npp_obtain_sol: invalid call sequence (interior-point solution not provided yet)\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        }
        3 => {
            if (*prep).i_stat == 0 as libc::c_int {
                (glp_error_(
                    b"api/npp.c\0" as *const u8 as *const libc::c_char,
                    122 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_npp_obtain_sol: invalid call sequence (MIP solution not provided yet)\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        }
        _ => {
            (prep != prep
                || {
                    glp_assert_(
                        b"prep != prep\0" as *const u8 as *const libc::c_char,
                        b"api/npp.c\0" as *const u8 as *const libc::c_char,
                        126 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    if !((*prep).orig_dir == (*P).dir && (*prep).orig_m == (*P).m
        && (*prep).orig_n == (*P).n && (*prep).orig_nnz == (*P).nnz)
    {
        (glp_error_(
            b"api/npp.c\0" as *const u8 as *const libc::c_char,
            130 as libc::c_int,
        ))
            .unwrap()(
            b"glp_npp_obtain_sol: original instance mismatch\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    _glp_npp_unload_sol(prep, P);
}
pub unsafe extern "C" fn glp_npp_free_wksp(mut prep: *mut glp_prep) {
    _glp_npp_delete_wksp(prep);
}
