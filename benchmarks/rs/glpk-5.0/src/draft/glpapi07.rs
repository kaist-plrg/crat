use ::libc;
extern "C" {
    pub type BFD;
    pub type AVL;
    pub type AVLNODE;
    pub type glp_tree;
    pub type DMP;
    pub type BFX;
    fn glp_get_mat_col(
        P: *mut glp_prob,
        j: libc::c_int,
        ind: *mut libc::c_int,
        val: *mut libc::c_double,
    ) -> libc::c_int;
    fn glp_set_row_stat(P: *mut glp_prob, i: libc::c_int, stat: libc::c_int);
    fn glp_set_col_stat(P: *mut glp_prob, j: libc::c_int, stat: libc::c_int);
    fn glp_init_smcp(parm: *mut glp_smcp);
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_time() -> libc::c_double;
    fn frexp(_: libc::c_double, _: *mut libc::c_int) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn _glp_ssx_create(m: libc::c_int, n: libc::c_int, nnz: libc::c_int) -> *mut SSX;
    fn _glp_mpq_init() -> mpq_t;
    fn _glp_mpq_set_d(x: mpq_t, val: libc::c_double);
    fn _glp_mpq_set_si(x: mpq_t, p: libc::c_int, q: libc::c_uint);
    fn abs(_: libc::c_int) -> libc::c_int;
    fn _glp_mpq_add(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_mul(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_div(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_clear(x: mpq_t);
    fn _glp_mpq_neg(z: mpq_t, x: mpq_t);
    fn _glp_ssx_driver(ssx: *mut SSX) -> libc::c_int;
    fn _glp_mpq_get_d(x: mpq_t) -> libc::c_double;
    fn _glp_ssx_delete(ssx: *mut SSX);
    fn _glp_gmp_pool_count() -> libc::c_int;
    fn _glp_gmp_free_mem();
    fn _glp_fp2rat(
        x: libc::c_double,
        eps: libc::c_double,
        p: *mut libc::c_double,
        q: *mut libc::c_double,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_smcp {
    pub msg_lev: libc::c_int,
    pub meth: libc::c_int,
    pub pricing: libc::c_int,
    pub r_test: libc::c_int,
    pub tol_bnd: libc::c_double,
    pub tol_dj: libc::c_double,
    pub tol_piv: libc::c_double,
    pub obj_ll: libc::c_double,
    pub obj_ul: libc::c_double,
    pub it_lim: libc::c_int,
    pub tm_lim: libc::c_int,
    pub out_frq: libc::c_int,
    pub out_dly: libc::c_int,
    pub presolve: libc::c_int,
    pub excl: libc::c_int,
    pub shift: libc::c_int,
    pub aorn: libc::c_int,
    pub foo_bar: [libc::c_double; 33],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SSX {
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub type_0: *mut libc::c_int,
    pub lb: *mut mpq_t,
    pub ub: *mut mpq_t,
    pub dir: libc::c_int,
    pub coef: *mut mpq_t,
    pub A_ptr: *mut libc::c_int,
    pub A_ind: *mut libc::c_int,
    pub A_val: *mut mpq_t,
    pub stat: *mut libc::c_int,
    pub Q_row: *mut libc::c_int,
    pub Q_col: *mut libc::c_int,
    pub binv: *mut BFX,
    pub bbar: *mut mpq_t,
    pub pi: *mut mpq_t,
    pub cbar: *mut mpq_t,
    pub p: libc::c_int,
    pub rho: *mut mpq_t,
    pub ap: *mut mpq_t,
    pub q: libc::c_int,
    pub aq: *mut mpq_t,
    pub q_dir: libc::c_int,
    pub p_stat: libc::c_int,
    pub delta: mpq_t,
    pub msg_lev: libc::c_int,
    pub it_lim: libc::c_int,
    pub it_cnt: libc::c_int,
    pub tm_lim: libc::c_double,
    pub out_frq: libc::c_double,
    pub tm_beg: libc::c_double,
    pub tm_lag: libc::c_double,
}
pub type mpq_t = *mut mpq;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpq {
    pub p: mpz,
    pub q: mpz,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpz {
    pub val: libc::c_int,
    pub ptr: *mut mpz_seg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpz_seg {
    pub d: [libc::c_ushort; 6],
    pub next: *mut mpz_seg,
}
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
unsafe extern "C" fn set_d_eps(mut x: mpq_t, mut val: libc::c_double) {
    let mut current_block: u64;
    let mut s: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut f: libc::c_double = 0.;
    let mut p: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    let mut eps: libc::c_double = 1e-9f64;
    let mut temp: mpq_t = 0 as *mut mpq;
    (-1.7976931348623157e+308f64 <= val && val <= 1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"-DBL_MAX <= val && val <= +DBL_MAX\0" as *const u8
                    as *const libc::c_char,
                b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                90 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if val == floor(val) {
        _glp_mpq_set_d(x, val);
    } else {
        if val > 0.0f64 {
            s = 1 as libc::c_int;
            current_block = 10886091980245723256;
        } else if val < 0.0f64 {
            s = -(1 as libc::c_int);
            current_block = 10886091980245723256;
        } else {
            _glp_mpq_set_si(x, 0 as libc::c_int, 1 as libc::c_int as libc::c_uint);
            current_block = 9140253083748406772;
        }
        match current_block {
            9140253083748406772 => {}
            _ => {
                f = frexp(fabs(val), &mut n);
                _glp_fp2rat(f, 0.1f64 * eps, &mut p, &mut q);
                temp = _glp_mpq_init();
                _glp_mpq_set_d(x, p);
                _glp_mpq_set_d(temp, q);
                _glp_mpq_div(x, x, temp);
                _glp_mpq_set_si(
                    temp,
                    1 as libc::c_int,
                    1 as libc::c_int as libc::c_uint,
                );
                j = 1 as libc::c_int;
                while j <= abs(n) {
                    _glp_mpq_add(temp, temp, temp);
                    j += 1;
                    j;
                }
                if n > 0 as libc::c_int {
                    _glp_mpq_mul(x, x, temp);
                } else if n < 0 as libc::c_int {
                    _glp_mpq_div(x, x, temp);
                }
                _glp_mpq_clear(temp);
                if s < 0 as libc::c_int {
                    _glp_mpq_neg(x, x);
                }
                (fabs(val - _glp_mpq_get_d(x)) <= eps * (1.0f64 + fabs(val))
                    || {
                        glp_assert_(
                            b"fabs(val - mpq_get_d(x)) <= eps * (1.0 + fabs(val))\0"
                                as *const u8 as *const libc::c_char,
                            b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                            124 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
    };
}
unsafe extern "C" fn load_data(mut ssx: *mut SSX, mut lp: *mut glp_prob) {
    let mut m: libc::c_int = (*ssx).m;
    let mut n: libc::c_int = (*ssx).n;
    let mut nnz: libc::c_int = *((*ssx).A_ptr).offset((n + 1 as libc::c_int) as isize)
        - 1 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut loc: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    let mut coef: libc::c_double = 0.;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    ((*lp).m == m
        || {
            glp_assert_(
                b"lp->m == m\0" as *const u8 as *const libc::c_char,
                b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                135 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*lp).n == n
        || {
            glp_assert_(
                b"lp->n == n\0" as *const u8 as *const libc::c_char,
                b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                136 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*lp).nnz == nnz
        || {
            glp_assert_(
                b"lp->nnz == nnz\0" as *const u8 as *const libc::c_char,
                b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                137 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = 1 as libc::c_int;
    while k <= m + n {
        if k <= m {
            type_0 = (**((*lp).row).offset(k as isize)).type_0;
            lb = (**((*lp).row).offset(k as isize)).lb;
            ub = (**((*lp).row).offset(k as isize)).ub;
        } else {
            type_0 = (**((*lp).col).offset((k - m) as isize)).type_0;
            lb = (**((*lp).col).offset((k - m) as isize)).lb;
            ub = (**((*lp).col).offset((k - m) as isize)).ub;
        }
        match type_0 {
            1 => {
                type_0 = 0 as libc::c_int;
            }
            2 => {
                type_0 = 1 as libc::c_int;
            }
            3 => {
                type_0 = 2 as libc::c_int;
            }
            4 => {
                type_0 = 3 as libc::c_int;
            }
            5 => {
                type_0 = 4 as libc::c_int;
            }
            _ => {
                (type_0 != type_0
                    || {
                        glp_assert_(
                            b"type != type\0" as *const u8 as *const libc::c_char,
                            b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                            156 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        *((*ssx).type_0).offset(k as isize) = type_0;
        set_d_eps(*((*ssx).lb).offset(k as isize), lb);
        set_d_eps(*((*ssx).ub).offset(k as isize), ub);
        k += 1;
        k;
    }
    match (*lp).dir {
        1 => {
            (*ssx).dir = 0 as libc::c_int;
        }
        2 => {
            (*ssx).dir = 1 as libc::c_int;
        }
        _ => {
            (lp != lp
                || {
                    glp_assert_(
                        b"lp != lp\0" as *const u8 as *const libc::c_char,
                        b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                        166 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    k = 0 as libc::c_int;
    while k <= m + n {
        if k == 0 as libc::c_int {
            coef = (*lp).c0;
        } else if k <= m {
            coef = 0.0f64;
        } else {
            coef = (**((*lp).col).offset((k - m) as isize)).coef;
        }
        set_d_eps(*((*ssx).coef).offset(k as isize), coef);
        k += 1;
        k;
    }
    ind = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    val = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    loc = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while j <= n {
        *((*ssx).A_ptr).offset(j as isize) = loc + 1 as libc::c_int;
        len = glp_get_mat_col(lp, j, ind, val);
        k = 1 as libc::c_int;
        while k <= len {
            loc += 1;
            loc;
            *((*ssx).A_ind).offset(loc as isize) = *ind.offset(k as isize);
            set_d_eps(*((*ssx).A_val).offset(loc as isize), *val.offset(k as isize));
            k += 1;
            k;
        }
        j += 1;
        j;
    }
    (loc == nnz
        || {
            glp_assert_(
                b"loc == nnz\0" as *const u8 as *const libc::c_char,
                b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                191 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
}
unsafe extern "C" fn load_basis(
    mut ssx: *mut SSX,
    mut lp: *mut glp_prob,
) -> libc::c_int {
    let mut m: libc::c_int = (*ssx).m;
    let mut n: libc::c_int = (*ssx).n;
    let mut type_0: *mut libc::c_int = (*ssx).type_0;
    let mut stat: *mut libc::c_int = (*ssx).stat;
    let mut Q_row: *mut libc::c_int = (*ssx).Q_row;
    let mut Q_col: *mut libc::c_int = (*ssx).Q_col;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    ((*lp).m == m
        || {
            glp_assert_(
                b"lp->m == m\0" as *const u8 as *const libc::c_char,
                b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                206 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*lp).n == n
        || {
            glp_assert_(
                b"lp->n == n\0" as *const u8 as *const libc::c_char,
                b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                207 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = 1 as libc::c_int;
    while k <= m + n {
        if k <= m {
            *stat.offset(k as isize) = (**((*lp).row).offset(k as isize)).stat;
        } else {
            *stat.offset(k as isize) = (**((*lp).col).offset((k - m) as isize)).stat;
        }
        match *stat.offset(k as isize) {
            1 => {
                *stat.offset(k as isize) = 0 as libc::c_int;
            }
            2 => {
                *stat.offset(k as isize) = 1 as libc::c_int;
                (*type_0.offset(k as isize) == 1 as libc::c_int
                    || *type_0.offset(k as isize) == 3 as libc::c_int
                    || {
                        glp_assert_(
                            b"type[k] == SSX_LO || type[k] == SSX_DB\0" as *const u8
                                as *const libc::c_char,
                            b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                            220 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            3 => {
                *stat.offset(k as isize) = 2 as libc::c_int;
                (*type_0.offset(k as isize) == 2 as libc::c_int
                    || *type_0.offset(k as isize) == 3 as libc::c_int
                    || {
                        glp_assert_(
                            b"type[k] == SSX_UP || type[k] == SSX_DB\0" as *const u8
                                as *const libc::c_char,
                            b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                            224 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            4 => {
                *stat.offset(k as isize) = 3 as libc::c_int;
                (*type_0.offset(k as isize) == 0 as libc::c_int
                    || {
                        glp_assert_(
                            b"type[k] == SSX_FR\0" as *const u8 as *const libc::c_char,
                            b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                            228 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            5 => {
                *stat.offset(k as isize) = 4 as libc::c_int;
                (*type_0.offset(k as isize) == 4 as libc::c_int
                    || {
                        glp_assert_(
                            b"type[k] == SSX_FX\0" as *const u8 as *const libc::c_char,
                            b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                            232 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            _ => {
                (stat != stat
                    || {
                        glp_assert_(
                            b"stat != stat\0" as *const u8 as *const libc::c_char,
                            b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                            235 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        k += 1;
        k;
    }
    j = 0 as libc::c_int;
    i = j;
    k = 1 as libc::c_int;
    while k <= m + n {
        if *stat.offset(k as isize) == 0 as libc::c_int {
            i += 1;
            i;
            if i > m {
                return 1 as libc::c_int;
            }
            *Q_row.offset(k as isize) = i;
            *Q_col.offset(i as isize) = k;
        } else {
            j += 1;
            j;
            if j > n {
                return 1 as libc::c_int;
            }
            *Q_row.offset(k as isize) = m + j;
            *Q_col.offset((m + j) as isize) = k;
        }
        k += 1;
        k;
    }
    (i == m && j == n
        || {
            glp_assert_(
                b"i == m && j == n\0" as *const u8 as *const libc::c_char,
                b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                252 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn glp_exact(
    mut lp: *mut glp_prob,
    mut parm: *const glp_smcp,
) -> libc::c_int {
    let mut current_block: u64;
    let mut _parm: glp_smcp = glp_smcp {
        msg_lev: 0,
        meth: 0,
        pricing: 0,
        r_test: 0,
        tol_bnd: 0.,
        tol_dj: 0.,
        tol_piv: 0.,
        obj_ll: 0.,
        obj_ul: 0.,
        it_lim: 0,
        tm_lim: 0,
        out_frq: 0,
        out_dly: 0,
        presolve: 0,
        excl: 0,
        shift: 0,
        aorn: 0,
        foo_bar: [0.; 33],
    };
    let mut ssx: *mut SSX = 0 as *mut SSX;
    let mut m: libc::c_int = (*lp).m;
    let mut n: libc::c_int = (*lp).n;
    let mut nnz: libc::c_int = (*lp).nnz;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut pst: libc::c_int = 0;
    let mut dst: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut stat: libc::c_int = 0;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    let mut prim: libc::c_double = 0.;
    let mut dual: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    if parm.is_null() {
        parm = &mut _parm;
        glp_init_smcp(parm as *mut glp_smcp);
    }
    match (*parm).msg_lev {
        0 | 1 | 2 | 3 | 4 => {}
        _ => {
            (glp_error_(
                b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                276 as libc::c_int,
            ))
                .unwrap()(
                b"glp_exact: msg_lev = %d; invalid parameter\n\0" as *const u8
                    as *const libc::c_char,
                (*parm).msg_lev,
            );
        }
    }
    if (*parm).it_lim < 0 as libc::c_int {
        (glp_error_(
            b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
            281 as libc::c_int,
        ))
            .unwrap()(
            b"glp_exact: it_lim = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).it_lim,
        );
    }
    if (*parm).tm_lim < 0 as libc::c_int {
        (glp_error_(
            b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
            284 as libc::c_int,
        ))
            .unwrap()(
            b"glp_exact: tm_lim = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).tm_lim,
        );
    }
    if !(m > 0 as libc::c_int && n > 0 as libc::c_int) {
        if (*parm).msg_lev >= 1 as libc::c_int {
            glp_printf(
                b"glp_exact: problem has no rows/columns\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 0x5 as libc::c_int;
    }
    (*lp).dbs_stat = 1 as libc::c_int;
    (*lp).pbs_stat = (*lp).dbs_stat;
    (*lp).obj_val = 0.0f64;
    (*lp).some = 0 as libc::c_int;
    k = 1 as libc::c_int;
    while k <= m + n {
        if k <= m {
            type_0 = (**((*lp).row).offset(k as isize)).type_0;
            lb = (**((*lp).row).offset(k as isize)).lb;
            ub = (**((*lp).row).offset(k as isize)).ub;
        } else {
            type_0 = (**((*lp).col).offset((k - m) as isize)).type_0;
            lb = (**((*lp).col).offset((k - m) as isize)).lb;
            ub = (**((*lp).col).offset((k - m) as isize)).ub;
        }
        if type_0 == 4 as libc::c_int && lb >= ub {
            if (*parm).msg_lev >= 1 as libc::c_int {
                glp_printf(
                    b"glp_exact: %s %d has invalid bounds\n\0" as *const u8
                        as *const libc::c_char,
                    if k <= m {
                        b"row\0" as *const u8 as *const libc::c_char
                    } else {
                        b"column\0" as *const u8 as *const libc::c_char
                    },
                    if k <= m { k } else { k - m },
                );
            }
            return 0x4 as libc::c_int;
        }
        k += 1;
        k;
    }
    if (*parm).msg_lev >= 3 as libc::c_int {
        glp_printf(
            b"glp_exact: %d rows, %d columns, %d non-zeros\n\0" as *const u8
                as *const libc::c_char,
            m,
            n,
            nnz,
        );
        glp_printf(
            b"GLPK bignum module is being used\n\0" as *const u8 as *const libc::c_char,
        );
        glp_printf(
            b"(Consider installing GNU MP to attain a much better performance.)\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    ssx = _glp_ssx_create(m, n, nnz);
    load_data(ssx, lp);
    if load_basis(ssx, lp) != 0 {
        if (*parm).msg_lev >= 1 as libc::c_int {
            glp_printf(
                b"glp_exact: initial LP basis is invalid\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        ret = 0x1 as libc::c_int;
    } else {
        (*ssx).msg_lev = (*parm).msg_lev;
        (*ssx).it_lim = (*parm).it_lim;
        (*ssx).it_cnt = (*lp).it_cnt;
        (*ssx).tm_lim = (*parm).tm_lim as libc::c_double / 1000.0f64;
        (*ssx).out_frq = 5.0f64;
        (*ssx).tm_beg = glp_time();
        (*ssx).tm_lag = 0.0f64;
        ret = _glp_ssx_driver(ssx);
        (*lp).it_cnt = (*ssx).it_cnt;
        match ret {
            0 => {
                ret = 0 as libc::c_int;
                dst = 2 as libc::c_int;
                pst = dst;
                current_block = 5807581744382915773;
            }
            1 => {
                ret = 0 as libc::c_int;
                pst = 4 as libc::c_int;
                dst = 3 as libc::c_int;
                current_block = 5807581744382915773;
            }
            2 => {
                ret = 0 as libc::c_int;
                pst = 2 as libc::c_int;
                dst = 4 as libc::c_int;
                (1 as libc::c_int <= (*ssx).q && (*ssx).q <= n
                    || {
                        glp_assert_(
                            b"1 <= ssx->q && ssx->q <= n\0" as *const u8
                                as *const libc::c_char,
                            b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                            404 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*lp).some = *((*ssx).Q_col).offset((m + (*ssx).q) as isize);
                (1 as libc::c_int <= (*lp).some && (*lp).some <= m + n
                    || {
                        glp_assert_(
                            b"1 <= lp->some && lp->some <= m+n\0" as *const u8
                                as *const libc::c_char,
                            b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                            406 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                current_block = 5807581744382915773;
            }
            3 => {
                ret = 0x8 as libc::c_int;
                dst = 3 as libc::c_int;
                pst = dst;
                current_block = 5807581744382915773;
            }
            4 => {
                ret = 0x8 as libc::c_int;
                pst = 2 as libc::c_int;
                dst = 3 as libc::c_int;
                current_block = 5807581744382915773;
            }
            5 => {
                ret = 0x9 as libc::c_int;
                dst = 3 as libc::c_int;
                pst = dst;
                current_block = 5807581744382915773;
            }
            6 => {
                ret = 0x9 as libc::c_int;
                pst = 2 as libc::c_int;
                dst = 3 as libc::c_int;
                current_block = 5807581744382915773;
            }
            7 => {
                ret = 0x2 as libc::c_int;
                current_block = 14930884760110778261;
            }
            _ => {
                (ret != ret
                    || {
                        glp_assert_(
                            b"ret != ret\0" as *const u8 as *const libc::c_char,
                            b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                            434 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                current_block = 5807581744382915773;
            }
        }
        match current_block {
            14930884760110778261 => {}
            _ => {
                (*lp).pbs_stat = pst;
                (*lp).dbs_stat = dst;
                sum = (*lp).c0;
                k = 1 as libc::c_int;
                while k <= m + n {
                    if *((*ssx).stat).offset(k as isize) == 0 as libc::c_int {
                        i = *((*ssx).Q_row).offset(k as isize);
                        (1 as libc::c_int <= i && i <= m
                            || {
                                glp_assert_(
                                    b"1 <= i && i <= m\0" as *const u8 as *const libc::c_char,
                                    b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                                    443 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        stat = 1 as libc::c_int;
                        prim = _glp_mpq_get_d(*((*ssx).bbar).offset(i as isize));
                        dual = 0.0f64;
                    } else {
                        j = *((*ssx).Q_row).offset(k as isize) - m;
                        (1 as libc::c_int <= j && j <= n
                            || {
                                glp_assert_(
                                    b"1 <= j && j <= n\0" as *const u8 as *const libc::c_char,
                                    b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                                    450 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        match *((*ssx).stat).offset(k as isize) {
                            3 => {
                                stat = 4 as libc::c_int;
                                prim = 0.0f64;
                            }
                            1 => {
                                stat = 2 as libc::c_int;
                                prim = _glp_mpq_get_d(*((*ssx).lb).offset(k as isize));
                            }
                            2 => {
                                stat = 3 as libc::c_int;
                                prim = _glp_mpq_get_d(*((*ssx).ub).offset(k as isize));
                            }
                            4 => {
                                stat = 5 as libc::c_int;
                                prim = _glp_mpq_get_d(*((*ssx).lb).offset(k as isize));
                            }
                            _ => {
                                (ssx != ssx
                                    || {
                                        glp_assert_(
                                            b"ssx != ssx\0" as *const u8 as *const libc::c_char,
                                            b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                                            469 as libc::c_int,
                                        );
                                        1 as libc::c_int != 0
                                    }) as libc::c_int;
                            }
                        }
                        dual = _glp_mpq_get_d(*((*ssx).cbar).offset(j as isize));
                    }
                    if k <= m {
                        glp_set_row_stat(lp, k, stat);
                        (**((*lp).row).offset(k as isize)).prim = prim;
                        (**((*lp).row).offset(k as isize)).dual = dual;
                    } else {
                        glp_set_col_stat(lp, k - m, stat);
                        (**((*lp).col).offset((k - m) as isize)).prim = prim;
                        (**((*lp).col).offset((k - m) as isize)).dual = dual;
                        sum += (**((*lp).col).offset((k - m) as isize)).coef * prim;
                    }
                    k += 1;
                    k;
                }
                (*lp).obj_val = sum;
            }
        }
    }
    _glp_ssx_delete(ssx);
    (_glp_gmp_pool_count() == 0 as libc::c_int
        || {
            glp_assert_(
                b"gmp_pool_count() == 0\0" as *const u8 as *const libc::c_char,
                b"draft/glpapi07.c\0" as *const u8 as *const libc::c_char,
                489 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_gmp_free_mem();
    return ret;
}
