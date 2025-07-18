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
    fn _glp_minisat_new() -> *mut solver;
    fn _glp_minisat_delete(s: *mut solver);
    fn _glp_minisat_addclause(s: *mut solver, begin: *mut lit, end: *mut lit) -> bool_0;
    fn _glp_minisat_solve(s: *mut solver, begin: *mut lit, end: *mut lit) -> bool_0;
    fn _glp_minisat_setnvars(s: *mut solver, n: libc::c_int);
    fn glp_check_cnfsat(P: *mut glp_prob) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
pub type bool_0 = libc::c_int;
pub type lit = libc::c_int;
pub type lbool = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct veci {
    pub size: libc::c_int,
    pub cap: libc::c_int,
    pub ptr: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vecp {
    pub size: libc::c_int,
    pub cap: libc::c_int,
    pub ptr: *mut *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clause {
    pub size_learnt: libc::c_int,
    pub lits: [lit; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats {
    pub starts: libc::c_double,
    pub decisions: libc::c_double,
    pub propagations: libc::c_double,
    pub inspects: libc::c_double,
    pub conflicts: libc::c_double,
    pub clauses: libc::c_double,
    pub clauses_literals: libc::c_double,
    pub learnts: libc::c_double,
    pub learnts_literals: libc::c_double,
    pub max_literals: libc::c_double,
    pub tot_literals: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct solver {
    pub size: libc::c_int,
    pub cap: libc::c_int,
    pub qhead: libc::c_int,
    pub qtail: libc::c_int,
    pub clauses: vecp,
    pub learnts: vecp,
    pub var_inc: libc::c_double,
    pub var_decay: libc::c_double,
    pub cla_inc: libc::c_float,
    pub cla_decay: libc::c_float,
    pub wlists: *mut vecp,
    pub activity: *mut libc::c_double,
    pub assigns: *mut lbool,
    pub orderpos: *mut libc::c_int,
    pub reasons: *mut *mut clause,
    pub levels: *mut libc::c_int,
    pub trail: *mut lit,
    pub binary: *mut clause,
    pub tags: *mut lbool,
    pub tagged: veci,
    pub stack: veci,
    pub order: veci,
    pub trail_lim: veci,
    pub model: veci,
    pub root_level: libc::c_int,
    pub simpdb_assigns: libc::c_int,
    pub simpdb_props: libc::c_int,
    pub random_seed: libc::c_double,
    pub progress_estimate: libc::c_double,
    pub verbosity: libc::c_int,
    pub stats: stats,
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
pub unsafe extern "C" fn glp_minisat1(mut P: *mut glp_prob) -> libc::c_int {
    let mut current_block: u64;
    let mut s: *mut solver = 0 as *mut solver;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut sum: libc::c_double = 0.;
    if !((*P).tree).is_null() {
        (glp_error_(
            b"api/minisat1.c\0" as *const u8 as *const libc::c_char,
            39 as libc::c_int,
        ))
            .unwrap()(
            b"glp_minisat1: operation not allowed\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*P).mip_stat = 1 as libc::c_int;
    (*P).mip_obj = 0.0f64;
    if glp_check_cnfsat(P) != 0 as libc::c_int {
        glp_printf(
            b"glp_minisat1: problem object does not encode CNF-SAT instance\n\0"
                as *const u8 as *const libc::c_char,
        );
        ret = 0x12 as libc::c_int;
    } else if ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        != ::std::mem::size_of::<size_t>() as libc::c_ulong
    {
        glp_printf(
            b"glp_minisat1: sorry, MiniSat solver is not supported on this platform\n\0"
                as *const u8 as *const libc::c_char,
        );
        ret = 0x5 as libc::c_int;
    } else {
        glp_printf(
            b"Solving CNF-SAT problem...\n\0" as *const u8 as *const libc::c_char,
        );
        glp_printf(
            b"Instance has %d variable%s, %d clause%s, and %d literal%s\n\0" as *const u8
                as *const libc::c_char,
            (*P).n,
            if (*P).n == 1 as libc::c_int {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"s\0" as *const u8 as *const libc::c_char
            },
            (*P).m,
            if (*P).m == 1 as libc::c_int {
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
        if (*P).m == 0 as libc::c_int {
            (*P).mip_stat = 5 as libc::c_int;
            j = 1 as libc::c_int;
            while j <= (*P).n {
                (**((*P).col).offset(j as isize)).mipx = 0.0f64;
                j += 1;
                j;
            }
        } else {
            i = 1 as libc::c_int;
            loop {
                if !(i <= (*P).m) {
                    current_block = 13797916685926291137;
                    break;
                }
                if ((**((*P).row).offset(i as isize)).ptr).is_null() {
                    (*P).mip_stat = 4 as libc::c_int;
                    current_block = 7465400772148191039;
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
            match current_block {
                7465400772148191039 => {}
                _ => {
                    s = _glp_minisat_new();
                    _glp_minisat_setnvars(s, (*P).n);
                    ind = glp_alloc(
                        1 as libc::c_int + (*P).n,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                            as libc::c_int,
                    ) as *mut libc::c_int;
                    i = 1 as libc::c_int;
                    loop {
                        if !(i <= (*P).m) {
                            current_block = 14434620278749266018;
                            break;
                        }
                        len = 0 as libc::c_int;
                        aij = (**((*P).row).offset(i as isize)).ptr;
                        while !aij.is_null() {
                            len += 1;
                            *ind
                                .offset(
                                    len as isize,
                                ) = (*(*aij).col).j - 1 as libc::c_int
                                + ((*(*aij).col).j - 1 as libc::c_int);
                            if (*aij).val < 0.0f64 {
                                *ind
                                    .offset(
                                        len as isize,
                                    ) = *ind.offset(len as isize) ^ 1 as libc::c_int;
                            }
                            aij = (*aij).r_next;
                        }
                        (len > 0 as libc::c_int
                            || {
                                glp_assert_(
                                    b"len > 0\0" as *const u8 as *const libc::c_char,
                                    b"api/minisat1.c\0" as *const u8 as *const libc::c_char,
                                    97 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        if _glp_minisat_addclause(
                            s,
                            &mut *ind.offset(1 as libc::c_int as isize),
                            &mut *ind.offset((1 as libc::c_int + len) as isize),
                        ) == 0
                        {
                            glp_free(ind as *mut libc::c_void);
                            _glp_minisat_delete(s);
                            (*P).mip_stat = 4 as libc::c_int;
                            current_block = 7465400772148191039;
                            break;
                        } else {
                            i += 1;
                            i;
                        }
                    }
                    match current_block {
                        7465400772148191039 => {}
                        _ => {
                            glp_free(ind as *mut libc::c_void);
                            (*s).verbosity = 1 as libc::c_int;
                            if _glp_minisat_solve(s, 0 as *mut lit, 0 as *mut lit) != 0 {
                                (*P).mip_stat = 5 as libc::c_int;
                                ((*s).model.size == (*P).n
                                    || {
                                        glp_assert_(
                                            b"s->model.size == P->n\0" as *const u8
                                                as *const libc::c_char,
                                            b"api/minisat1.c\0" as *const u8 as *const libc::c_char,
                                            117 as libc::c_int,
                                        );
                                        1 as libc::c_int != 0
                                    }) as libc::c_int;
                                j = 1 as libc::c_int;
                                while j <= (*P).n {
                                    (**((*P).col).offset(j as isize))
                                        .mipx = if *((*s).model.ptr)
                                        .offset((j - 1 as libc::c_int) as isize) == 1 as libc::c_int
                                    {
                                        1.0f64
                                    } else {
                                        0.0f64
                                    };
                                    j += 1;
                                    j;
                                }
                                i = 1 as libc::c_int;
                                while i <= (*P).m {
                                    sum = 0 as libc::c_int as libc::c_double;
                                    aij = (**((*P).row).offset(i as isize)).ptr;
                                    while !aij.is_null() {
                                        sum += (*aij).val * (*(*aij).col).mipx;
                                        aij = (*aij).r_next;
                                    }
                                    (**((*P).row).offset(i as isize)).mipx = sum;
                                    i += 1;
                                    i;
                                }
                                i = 1 as libc::c_int;
                                while i <= (*P).m {
                                    if (**((*P).row).offset(i as isize)).mipx
                                        < (**((*P).row).offset(i as isize)).lb
                                    {
                                        (*P).mip_stat = 1 as libc::c_int;
                                        break;
                                    } else {
                                        i += 1;
                                        i;
                                    }
                                }
                            } else {
                                (*P).mip_stat = 4 as libc::c_int;
                            }
                            _glp_minisat_delete(s);
                        }
                    }
                }
            }
        }
        if (*P).mip_stat == 5 as libc::c_int {
            glp_printf(b"SATISFIABLE\n\0" as *const u8 as *const libc::c_char);
            ret = 0 as libc::c_int;
        } else if (*P).mip_stat == 4 as libc::c_int {
            glp_printf(b"UNSATISFIABLE\n\0" as *const u8 as *const libc::c_char);
            ret = 0 as libc::c_int;
        } else {
            glp_printf(
                b"glp_minisat1: solver failed\n\0" as *const u8 as *const libc::c_char,
            );
            ret = 0x5 as libc::c_int;
        }
    }
    return ret;
}
