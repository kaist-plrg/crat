use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn _glp_mat_adat_symbolic(
        m: libc::c_int,
        n: libc::c_int,
        P_per: *mut libc::c_int,
        A_ptr: *mut libc::c_int,
        A_ind: *mut libc::c_int,
        S_ptr: *mut libc::c_int,
    ) -> *mut libc::c_int;
    fn _glp_mat_adat_numeric(
        m: libc::c_int,
        n: libc::c_int,
        P_per: *mut libc::c_int,
        A_ptr: *mut libc::c_int,
        A_ind: *mut libc::c_int,
        A_val: *mut libc::c_double,
        D_diag: *mut libc::c_double,
        S_ptr: *mut libc::c_int,
        S_ind: *mut libc::c_int,
        S_val: *mut libc::c_double,
        S_diag: *mut libc::c_double,
    );
    fn _glp_mat_min_degree(
        n: libc::c_int,
        A_ptr: *mut libc::c_int,
        A_ind: *mut libc::c_int,
        P_per: *mut libc::c_int,
    );
    fn _glp_mat_amd_order1(
        n: libc::c_int,
        A_ptr: *mut libc::c_int,
        A_ind: *mut libc::c_int,
        P_per: *mut libc::c_int,
    );
    fn _glp_mat_symamd_ord(
        n: libc::c_int,
        A_ptr: *mut libc::c_int,
        A_ind: *mut libc::c_int,
        P_per: *mut libc::c_int,
    );
    fn _glp_mat_chol_symbolic(
        n: libc::c_int,
        A_ptr: *mut libc::c_int,
        A_ind: *mut libc::c_int,
        U_ptr: *mut libc::c_int,
    ) -> *mut libc::c_int;
    fn _glp_mat_chol_numeric(
        n: libc::c_int,
        A_ptr: *mut libc::c_int,
        A_ind: *mut libc::c_int,
        A_val: *mut libc::c_double,
        A_diag: *mut libc::c_double,
        U_ptr: *mut libc::c_int,
        U_ind: *mut libc::c_int,
        U_val: *mut libc::c_double,
        U_diag: *mut libc::c_double,
    ) -> libc::c_int;
    fn _glp_mat_u_solve(
        n: libc::c_int,
        U_ptr: *mut libc::c_int,
        U_ind: *mut libc::c_int,
        U_val: *mut libc::c_double,
        U_diag: *mut libc::c_double,
        x: *mut libc::c_double,
    );
    fn _glp_mat_ut_solve(
        n: libc::c_int,
        U_ptr: *mut libc::c_int,
        U_ind: *mut libc::c_int,
        U_val: *mut libc::c_double,
        U_diag: *mut libc::c_double,
        x: *mut libc::c_double,
    );
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
pub struct glp_iptcp {
    pub msg_lev: libc::c_int,
    pub ord_alg: libc::c_int,
    pub foo_bar: [libc::c_double; 48],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csa {
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub A_ptr: *mut libc::c_int,
    pub A_ind: *mut libc::c_int,
    pub A_val: *mut libc::c_double,
    pub b: *mut libc::c_double,
    pub c: *mut libc::c_double,
    pub x: *mut libc::c_double,
    pub y: *mut libc::c_double,
    pub z: *mut libc::c_double,
    pub parm: *const glp_iptcp,
    pub D: *mut libc::c_double,
    pub P: *mut libc::c_int,
    pub S_ptr: *mut libc::c_int,
    pub S_ind: *mut libc::c_int,
    pub S_val: *mut libc::c_double,
    pub S_diag: *mut libc::c_double,
    pub U_ptr: *mut libc::c_int,
    pub U_ind: *mut libc::c_int,
    pub U_val: *mut libc::c_double,
    pub U_diag: *mut libc::c_double,
    pub iter: libc::c_int,
    pub obj: libc::c_double,
    pub rpi: libc::c_double,
    pub rdi: libc::c_double,
    pub gap: libc::c_double,
    pub phi: libc::c_double,
    pub mu: libc::c_double,
    pub rmu: libc::c_double,
    pub rmu0: libc::c_double,
    pub phi_min: *mut libc::c_double,
    pub best_iter: libc::c_int,
    pub best_x: *mut libc::c_double,
    pub best_y: *mut libc::c_double,
    pub best_z: *mut libc::c_double,
    pub best_obj: libc::c_double,
    pub dx_aff: *mut libc::c_double,
    pub dy_aff: *mut libc::c_double,
    pub dz_aff: *mut libc::c_double,
    pub alfa_aff_p: libc::c_double,
    pub alfa_aff_d: libc::c_double,
    pub mu_aff: libc::c_double,
    pub sigma: libc::c_double,
    pub dx_cc: *mut libc::c_double,
    pub dy_cc: *mut libc::c_double,
    pub dz_cc: *mut libc::c_double,
    pub dx: *mut libc::c_double,
    pub dy: *mut libc::c_double,
    pub dz: *mut libc::c_double,
    pub alfa_max_p: libc::c_double,
    pub alfa_max_d: libc::c_double,
}
unsafe extern "C" fn initialize(mut csa: *mut csa) {
    let mut m: libc::c_int = (*csa).m;
    let mut n: libc::c_int = (*csa).n;
    let mut i: libc::c_int = 0;
    if (*(*csa).parm).msg_lev >= 3 as libc::c_int {
        glp_printf(
            b"Matrix A has %d non-zeros\n\0" as *const u8 as *const libc::c_char,
            *((*csa).A_ptr).offset((m + 1 as libc::c_int) as isize) - 1 as libc::c_int,
        );
    }
    (*csa)
        .D = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*csa)
        .P = glp_alloc(
        1 as libc::c_int + m + m,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    i = 1 as libc::c_int;
    while i <= m {
        let ref mut fresh0 = *((*csa).P).offset((m + i) as isize);
        *fresh0 = i;
        *((*csa).P).offset(i as isize) = *fresh0;
        i += 1;
        i;
    }
    (*csa)
        .S_ptr = glp_alloc(
        1 as libc::c_int + m + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*csa)
        .S_ind = _glp_mat_adat_symbolic(
        m,
        n,
        (*csa).P,
        (*csa).A_ptr,
        (*csa).A_ind,
        (*csa).S_ptr,
    );
    if (*(*csa).parm).msg_lev >= 3 as libc::c_int {
        glp_printf(
            b"Matrix S = A*A' has %d non-zeros (upper triangle)\n\0" as *const u8
                as *const libc::c_char,
            *((*csa).S_ptr).offset((m + 1 as libc::c_int) as isize) - 1 as libc::c_int
                + m,
        );
    }
    if (*(*csa).parm).ord_alg == 0 as libc::c_int {
        if (*(*csa).parm).msg_lev >= 3 as libc::c_int {
            glp_printf(
                b"Original ordering is being used\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        i = 1 as libc::c_int;
        while i <= m {
            let ref mut fresh1 = *((*csa).P).offset((m + i) as isize);
            *fresh1 = i;
            *((*csa).P).offset(i as isize) = *fresh1;
            i += 1;
            i;
        }
    } else if (*(*csa).parm).ord_alg == 1 as libc::c_int {
        if (*(*csa).parm).msg_lev >= 3 as libc::c_int {
            glp_printf(
                b"Minimum degree ordering (QMD)...\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        _glp_mat_min_degree(m, (*csa).S_ptr, (*csa).S_ind, (*csa).P);
    } else if (*(*csa).parm).ord_alg == 2 as libc::c_int {
        if (*(*csa).parm).msg_lev >= 3 as libc::c_int {
            glp_printf(
                b"Approximate minimum degree ordering (AMD)...\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        _glp_mat_amd_order1(m, (*csa).S_ptr, (*csa).S_ind, (*csa).P);
    } else if (*(*csa).parm).ord_alg == 3 as libc::c_int {
        if (*(*csa).parm).msg_lev >= 3 as libc::c_int {
            glp_printf(
                b"Approximate minimum degree ordering (SYMAMD)...\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        _glp_mat_symamd_ord(m, (*csa).S_ptr, (*csa).S_ind, (*csa).P);
    } else {
        (csa != csa
            || {
                glp_assert_(
                    b"csa != csa\0" as *const u8 as *const libc::c_char,
                    b"draft/glpipm.c\0" as *const u8 as *const libc::c_char,
                    188 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    glp_free((*csa).S_ind as *mut libc::c_void);
    (*csa)
        .S_ind = _glp_mat_adat_symbolic(
        m,
        n,
        (*csa).P,
        (*csa).A_ptr,
        (*csa).A_ind,
        (*csa).S_ptr,
    );
    (*csa)
        .S_val = glp_alloc(
        *((*csa).S_ptr).offset((m + 1 as libc::c_int) as isize),
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*csa)
        .S_diag = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    if (*(*csa).parm).msg_lev >= 3 as libc::c_int {
        glp_printf(
            b"Computing Cholesky factorization S = L*L'...\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*csa)
        .U_ptr = glp_alloc(
        1 as libc::c_int + m + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*csa).U_ind = _glp_mat_chol_symbolic(m, (*csa).S_ptr, (*csa).S_ind, (*csa).U_ptr);
    if (*(*csa).parm).msg_lev >= 3 as libc::c_int {
        glp_printf(
            b"Matrix L has %d non-zeros\n\0" as *const u8 as *const libc::c_char,
            *((*csa).U_ptr).offset((m + 1 as libc::c_int) as isize) - 1 as libc::c_int
                + m,
        );
    }
    (*csa)
        .U_val = glp_alloc(
        *((*csa).U_ptr).offset((m + 1 as libc::c_int) as isize),
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*csa)
        .U_diag = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*csa).iter = 0 as libc::c_int;
    (*csa).obj = 0.0f64;
    (*csa).rpi = 0.0f64;
    (*csa).rdi = 0.0f64;
    (*csa).gap = 0.0f64;
    (*csa).phi = 0.0f64;
    (*csa).mu = 0.0f64;
    (*csa).rmu = 0.0f64;
    (*csa).rmu0 = 0.0f64;
    (*csa)
        .phi_min = glp_alloc(
        1 as libc::c_int + 100 as libc::c_int,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*csa).best_iter = 0 as libc::c_int;
    (*csa)
        .best_x = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*csa)
        .best_y = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*csa)
        .best_z = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*csa).best_obj = 0.0f64;
    (*csa)
        .dx_aff = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*csa)
        .dy_aff = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*csa)
        .dz_aff = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*csa).alfa_aff_p = 0.0f64;
    (*csa).alfa_aff_d = 0.0f64;
    (*csa).mu_aff = 0.0f64;
    (*csa).sigma = 0.0f64;
    (*csa)
        .dx_cc = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*csa)
        .dy_cc = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*csa)
        .dz_cc = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*csa).dx = (*csa).dx_aff;
    (*csa).dy = (*csa).dy_aff;
    (*csa).dz = (*csa).dz_aff;
    (*csa).alfa_max_p = 0.0f64;
    (*csa).alfa_max_d = 0.0f64;
}
unsafe extern "C" fn A_by_vec(
    mut csa: *mut csa,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
) {
    let mut m: libc::c_int = (*csa).m;
    let mut A_ptr: *mut libc::c_int = (*csa).A_ptr;
    let mut A_ind: *mut libc::c_int = (*csa).A_ind;
    let mut A_val: *mut libc::c_double = (*csa).A_val;
    let mut i: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut beg: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut temp: libc::c_double = 0.;
    i = 1 as libc::c_int;
    while i <= m {
        temp = 0.0f64;
        beg = *A_ptr.offset(i as isize);
        end = *A_ptr.offset((i + 1 as libc::c_int) as isize);
        t = beg;
        while t < end {
            temp
                += *A_val.offset(t as isize)
                    * *x.offset(*A_ind.offset(t as isize) as isize);
            t += 1;
            t;
        }
        *y.offset(i as isize) = temp;
        i += 1;
        i;
    }
}
unsafe extern "C" fn AT_by_vec(
    mut csa: *mut csa,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
) {
    let mut m: libc::c_int = (*csa).m;
    let mut n: libc::c_int = (*csa).n;
    let mut A_ptr: *mut libc::c_int = (*csa).A_ptr;
    let mut A_ind: *mut libc::c_int = (*csa).A_ind;
    let mut A_val: *mut libc::c_double = (*csa).A_val;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut beg: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut temp: libc::c_double = 0.;
    j = 1 as libc::c_int;
    while j <= n {
        *y.offset(j as isize) = 0.0f64;
        j += 1;
        j;
    }
    i = 1 as libc::c_int;
    while i <= m {
        temp = *x.offset(i as isize);
        if !(temp == 0.0f64) {
            beg = *A_ptr.offset(i as isize);
            end = *A_ptr.offset((i + 1 as libc::c_int) as isize);
            t = beg;
            while t < end {
                *y.offset(*A_ind.offset(t as isize) as isize)
                    += *A_val.offset(t as isize) * temp;
                t += 1;
                t;
            }
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn decomp_NE(mut csa: *mut csa) {
    _glp_mat_adat_numeric(
        (*csa).m,
        (*csa).n,
        (*csa).P,
        (*csa).A_ptr,
        (*csa).A_ind,
        (*csa).A_val,
        (*csa).D,
        (*csa).S_ptr,
        (*csa).S_ind,
        (*csa).S_val,
        (*csa).S_diag,
    );
    _glp_mat_chol_numeric(
        (*csa).m,
        (*csa).S_ptr,
        (*csa).S_ind,
        (*csa).S_val,
        (*csa).S_diag,
        (*csa).U_ptr,
        (*csa).U_ind,
        (*csa).U_val,
        (*csa).U_diag,
    );
}
unsafe extern "C" fn solve_NE(
    mut csa: *mut csa,
    mut y: *mut libc::c_double,
) -> libc::c_int {
    let mut m: libc::c_int = (*csa).m;
    let mut n: libc::c_int = (*csa).n;
    let mut P: *mut libc::c_int = (*csa).P;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut h: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut r: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut w: *mut libc::c_double = 0 as *mut libc::c_double;
    h = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    i = 1 as libc::c_int;
    while i <= m {
        *h.offset(i as isize) = *y.offset(i as isize);
        i += 1;
        i;
    }
    w = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    i = 1 as libc::c_int;
    while i <= m {
        *w.offset(i as isize) = *y.offset(*P.offset(i as isize) as isize);
        i += 1;
        i;
    }
    _glp_mat_ut_solve(m, (*csa).U_ptr, (*csa).U_ind, (*csa).U_val, (*csa).U_diag, w);
    _glp_mat_u_solve(m, (*csa).U_ptr, (*csa).U_ind, (*csa).U_val, (*csa).U_diag, w);
    i = 1 as libc::c_int;
    while i <= m {
        *y.offset(i as isize) = *w.offset(*P.offset((m + i) as isize) as isize);
        i += 1;
        i;
    }
    glp_free(w as *mut libc::c_void);
    r = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    w = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    AT_by_vec(csa, y, w);
    j = 1 as libc::c_int;
    while j <= n {
        *w.offset(j as isize) *= *((*csa).D).offset(j as isize);
        j += 1;
        j;
    }
    A_by_vec(csa, w, r);
    glp_free(w as *mut libc::c_void);
    i = 1 as libc::c_int;
    while i <= m {
        *r.offset(i as isize) -= *h.offset(i as isize);
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i <= m {
        if fabs(*r.offset(i as isize)) / (1.0f64 + fabs(*h.offset(i as isize))) > 1e-4f64
        {
            ret = 1 as libc::c_int;
            break;
        } else {
            i += 1;
            i;
        }
    }
    glp_free(h as *mut libc::c_void);
    glp_free(r as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn solve_NS(
    mut csa: *mut csa,
    mut p: *mut libc::c_double,
    mut q: *mut libc::c_double,
    mut r: *mut libc::c_double,
    mut dx: *mut libc::c_double,
    mut dy: *mut libc::c_double,
    mut dz: *mut libc::c_double,
) -> libc::c_int {
    let mut m: libc::c_int = (*csa).m;
    let mut n: libc::c_int = (*csa).n;
    let mut x: *mut libc::c_double = (*csa).x;
    let mut z: *mut libc::c_double = (*csa).z;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut w: *mut libc::c_double = dx;
    j = 1 as libc::c_int;
    while j <= n {
        *w
            .offset(
                j as isize,
            ) = (*x.offset(j as isize) * *q.offset(j as isize) - *r.offset(j as isize))
            / *z.offset(j as isize);
        j += 1;
        j;
    }
    A_by_vec(csa, w, dy);
    i = 1 as libc::c_int;
    while i <= m {
        *dy.offset(i as isize) += *p.offset(i as isize);
        i += 1;
        i;
    }
    ret = solve_NE(csa, dy);
    AT_by_vec(csa, dy, dx);
    j = 1 as libc::c_int;
    while j <= n {
        *dx
            .offset(
                j as isize,
            ) = (*x.offset(j as isize) * (*dx.offset(j as isize) - *q.offset(j as isize))
            + *r.offset(j as isize)) / *z.offset(j as isize);
        *dz
            .offset(
                j as isize,
            ) = (*r.offset(j as isize) - *z.offset(j as isize) * *dx.offset(j as isize))
            / *x.offset(j as isize);
        j += 1;
        j;
    }
    return ret;
}
unsafe extern "C" fn initial_point(mut csa: *mut csa) {
    let mut m: libc::c_int = (*csa).m;
    let mut n: libc::c_int = (*csa).n;
    let mut b: *mut libc::c_double = (*csa).b;
    let mut c: *mut libc::c_double = (*csa).c;
    let mut x: *mut libc::c_double = (*csa).x;
    let mut y: *mut libc::c_double = (*csa).y;
    let mut z: *mut libc::c_double = (*csa).z;
    let mut D: *mut libc::c_double = (*csa).D;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut dp: libc::c_double = 0.;
    let mut dd: libc::c_double = 0.;
    let mut ex: libc::c_double = 0.;
    let mut ez: libc::c_double = 0.;
    let mut xz: libc::c_double = 0.;
    j = 1 as libc::c_int;
    while j <= n {
        *D.offset(j as isize) = 1.0f64;
        j += 1;
        j;
    }
    decomp_NE(csa);
    i = 1 as libc::c_int;
    while i <= m {
        *y.offset(i as isize) = *b.offset(i as isize);
        i += 1;
        i;
    }
    solve_NE(csa, y);
    AT_by_vec(csa, y, x);
    A_by_vec(csa, c, y);
    solve_NE(csa, y);
    AT_by_vec(csa, y, z);
    j = 1 as libc::c_int;
    while j <= n {
        *z.offset(j as isize) = *c.offset(j as isize) - *z.offset(j as isize);
        j += 1;
        j;
    }
    dd = 0.0f64;
    dp = dd;
    j = 1 as libc::c_int;
    while j <= n {
        if dp < -1.5f64 * *x.offset(j as isize) {
            dp = -1.5f64 * *x.offset(j as isize);
        }
        if dd < -1.5f64 * *z.offset(j as isize) {
            dd = -1.5f64 * *z.offset(j as isize);
        }
        j += 1;
        j;
    }
    if dp == 0.0f64 {
        dp = 1.5f64;
    }
    if dd == 0.0f64 {
        dd = 1.5f64;
    }
    xz = 0.0f64;
    ez = xz;
    ex = ez;
    j = 1 as libc::c_int;
    while j <= n {
        ex += *x.offset(j as isize) + dp;
        ez += *z.offset(j as isize) + dd;
        xz += (*x.offset(j as isize) + dp) * (*z.offset(j as isize) + dd);
        j += 1;
        j;
    }
    dp += 0.5f64 * (xz / ez);
    dd += 0.5f64 * (xz / ex);
    j = 1 as libc::c_int;
    while j <= n {
        *x.offset(j as isize) += dp;
        *z.offset(j as isize) += dd;
        (*x.offset(j as isize) > 0.0f64 && *z.offset(j as isize) > 0.0f64
            || {
                glp_assert_(
                    b"x[j] > 0.0 && z[j] > 0.0\0" as *const u8 as *const libc::c_char,
                    b"draft/glpipm.c\0" as *const u8 as *const libc::c_char,
                    505 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        j += 1;
        j;
    }
}
unsafe extern "C" fn basic_info(mut csa: *mut csa) {
    let mut m: libc::c_int = (*csa).m;
    let mut n: libc::c_int = (*csa).n;
    let mut b: *mut libc::c_double = (*csa).b;
    let mut c: *mut libc::c_double = (*csa).c;
    let mut x: *mut libc::c_double = (*csa).x;
    let mut y: *mut libc::c_double = (*csa).y;
    let mut z: *mut libc::c_double = (*csa).z;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut norm1: libc::c_double = 0.;
    let mut bnorm: libc::c_double = 0.;
    let mut norm2: libc::c_double = 0.;
    let mut cnorm: libc::c_double = 0.;
    let mut cx: libc::c_double = 0.;
    let mut by: libc::c_double = 0.;
    let mut work: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut temp: libc::c_double = 0.;
    temp = *c.offset(0 as libc::c_int as isize);
    j = 1 as libc::c_int;
    while j <= n {
        temp += *c.offset(j as isize) * *x.offset(j as isize);
        j += 1;
        j;
    }
    (*csa).obj = temp;
    work = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    A_by_vec(csa, x, work);
    norm1 = 0.0f64;
    i = 1 as libc::c_int;
    while i <= m {
        norm1
            += (*work.offset(i as isize) - *b.offset(i as isize))
                * (*work.offset(i as isize) - *b.offset(i as isize));
        i += 1;
        i;
    }
    norm1 = sqrt(norm1);
    glp_free(work as *mut libc::c_void);
    bnorm = 0.0f64;
    i = 1 as libc::c_int;
    while i <= m {
        bnorm += *b.offset(i as isize) * *b.offset(i as isize);
        i += 1;
        i;
    }
    bnorm = sqrt(bnorm);
    (*csa).rpi = norm1 / (1.0f64 + bnorm);
    work = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    AT_by_vec(csa, y, work);
    norm2 = 0.0f64;
    j = 1 as libc::c_int;
    while j <= n {
        norm2
            += (*work.offset(j as isize) + *z.offset(j as isize) - *c.offset(j as isize))
                * (*work.offset(j as isize) + *z.offset(j as isize)
                    - *c.offset(j as isize));
        j += 1;
        j;
    }
    norm2 = sqrt(norm2);
    glp_free(work as *mut libc::c_void);
    cnorm = 0.0f64;
    j = 1 as libc::c_int;
    while j <= n {
        cnorm += *c.offset(j as isize) * *c.offset(j as isize);
        j += 1;
        j;
    }
    cnorm = sqrt(cnorm);
    (*csa).rdi = norm2 / (1.0f64 + cnorm);
    by = 0.0f64;
    i = 1 as libc::c_int;
    while i <= m {
        by += *b.offset(i as isize) * *y.offset(i as isize);
        i += 1;
        i;
    }
    cx = 0.0f64;
    j = 1 as libc::c_int;
    while j <= n {
        cx += *c.offset(j as isize) * *x.offset(j as isize);
        j += 1;
        j;
    }
    (*csa).gap = fabs(cx - by) / (1.0f64 + fabs(cx));
    (*csa).phi = 0.0f64;
    (*csa).phi += norm1 / (if bnorm > 1.0f64 { bnorm } else { 1.0f64 });
    (*csa).phi += norm2 / (if cnorm > 1.0f64 { cnorm } else { 1.0f64 });
    temp = 1.0f64;
    if temp < bnorm {
        temp = bnorm;
    }
    if temp < cnorm {
        temp = cnorm;
    }
    (*csa).phi += fabs(cx - by) / temp;
    temp = 0.0f64;
    j = 1 as libc::c_int;
    while j <= n {
        temp += *x.offset(j as isize) * *z.offset(j as isize);
        j += 1;
        j;
    }
    (*csa).mu = temp / n as libc::c_double;
    (*csa).rmu = (if norm1 > norm2 { norm1 } else { norm2 }) / (*csa).mu;
}
unsafe extern "C" fn make_step(mut csa: *mut csa) -> libc::c_int {
    let mut m: libc::c_int = (*csa).m;
    let mut n: libc::c_int = (*csa).n;
    let mut b: *mut libc::c_double = (*csa).b;
    let mut c: *mut libc::c_double = (*csa).c;
    let mut x: *mut libc::c_double = (*csa).x;
    let mut y: *mut libc::c_double = (*csa).y;
    let mut z: *mut libc::c_double = (*csa).z;
    let mut dx_aff: *mut libc::c_double = (*csa).dx_aff;
    let mut dy_aff: *mut libc::c_double = (*csa).dy_aff;
    let mut dz_aff: *mut libc::c_double = (*csa).dz_aff;
    let mut dx_cc: *mut libc::c_double = (*csa).dx_cc;
    let mut dy_cc: *mut libc::c_double = (*csa).dy_cc;
    let mut dz_cc: *mut libc::c_double = (*csa).dz_cc;
    let mut dx: *mut libc::c_double = (*csa).dx;
    let mut dy: *mut libc::c_double = (*csa).dy;
    let mut dz: *mut libc::c_double = (*csa).dz;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut temp: libc::c_double = 0.;
    let mut gamma_p: libc::c_double = 0.;
    let mut gamma_d: libc::c_double = 0.;
    let mut p: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut q: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut r: *mut libc::c_double = 0 as *mut libc::c_double;
    p = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    q = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    r = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    A_by_vec(csa, x, p);
    i = 1 as libc::c_int;
    while i <= m {
        *p.offset(i as isize) = *b.offset(i as isize) - *p.offset(i as isize);
        i += 1;
        i;
    }
    AT_by_vec(csa, y, q);
    j = 1 as libc::c_int;
    while j <= n {
        *q
            .offset(
                j as isize,
            ) = *c.offset(j as isize) - *q.offset(j as isize) - *z.offset(j as isize);
        j += 1;
        j;
    }
    j = 1 as libc::c_int;
    while j <= n {
        *r.offset(j as isize) = -*x.offset(j as isize) * *z.offset(j as isize);
        j += 1;
        j;
    }
    if solve_NS(csa, p, q, r, dx_aff, dy_aff, dz_aff) != 0 {
        ret = 1 as libc::c_int;
    } else {
        (*csa).alfa_aff_d = 1.0f64;
        (*csa).alfa_aff_p = (*csa).alfa_aff_d;
        j = 1 as libc::c_int;
        while j <= n {
            if *dx_aff.offset(j as isize) < 0.0f64 {
                temp = -*x.offset(j as isize) / *dx_aff.offset(j as isize);
                if (*csa).alfa_aff_p > temp {
                    (*csa).alfa_aff_p = temp;
                }
            }
            if *dz_aff.offset(j as isize) < 0.0f64 {
                temp = -*z.offset(j as isize) / *dz_aff.offset(j as isize);
                if (*csa).alfa_aff_d > temp {
                    (*csa).alfa_aff_d = temp;
                }
            }
            j += 1;
            j;
        }
        temp = 0.0f64;
        j = 1 as libc::c_int;
        while j <= n {
            temp
                += (*x.offset(j as isize)
                    + (*csa).alfa_aff_p * *dx_aff.offset(j as isize))
                    * (*z.offset(j as isize)
                        + (*csa).alfa_aff_d * *dz_aff.offset(j as isize));
            j += 1;
            j;
        }
        (*csa).mu_aff = temp / n as libc::c_double;
        temp = (*csa).mu_aff / (*csa).mu;
        (*csa).sigma = temp * temp * temp;
        i = 1 as libc::c_int;
        while i <= m {
            *p.offset(i as isize) = 0.0f64;
            i += 1;
            i;
        }
        j = 1 as libc::c_int;
        while j <= n {
            *q.offset(j as isize) = 0.0f64;
            j += 1;
            j;
        }
        j = 1 as libc::c_int;
        while j <= n {
            *r
                .offset(
                    j as isize,
                ) = (*csa).sigma * (*csa).mu
                - *dx_aff.offset(j as isize) * *dz_aff.offset(j as isize);
            j += 1;
            j;
        }
        if solve_NS(csa, p, q, r, dx_cc, dy_cc, dz_cc) != 0 {
            ret = 1 as libc::c_int;
        } else {
            j = 1 as libc::c_int;
            while j <= n {
                *dx
                    .offset(
                        j as isize,
                    ) = *dx_aff.offset(j as isize) + *dx_cc.offset(j as isize);
                j += 1;
                j;
            }
            i = 1 as libc::c_int;
            while i <= m {
                *dy
                    .offset(
                        i as isize,
                    ) = *dy_aff.offset(i as isize) + *dy_cc.offset(i as isize);
                i += 1;
                i;
            }
            j = 1 as libc::c_int;
            while j <= n {
                *dz
                    .offset(
                        j as isize,
                    ) = *dz_aff.offset(j as isize) + *dz_cc.offset(j as isize);
                j += 1;
                j;
            }
            (*csa).alfa_max_d = 1.0f64;
            (*csa).alfa_max_p = (*csa).alfa_max_d;
            j = 1 as libc::c_int;
            while j <= n {
                if *dx.offset(j as isize) < 0.0f64 {
                    temp = -*x.offset(j as isize) / *dx.offset(j as isize);
                    if (*csa).alfa_max_p > temp {
                        (*csa).alfa_max_p = temp;
                    }
                }
                if *dz.offset(j as isize) < 0.0f64 {
                    temp = -*z.offset(j as isize) / *dz.offset(j as isize);
                    if (*csa).alfa_max_d > temp {
                        (*csa).alfa_max_d = temp;
                    }
                }
                j += 1;
                j;
            }
            gamma_p = 0.90f64;
            gamma_d = 0.90f64;
            j = 1 as libc::c_int;
            while j <= n {
                *x.offset(j as isize)
                    += gamma_p * (*csa).alfa_max_p * *dx.offset(j as isize);
                (*x.offset(j as isize) > 0.0f64
                    || {
                        glp_assert_(
                            b"x[j] > 0.0\0" as *const u8 as *const libc::c_char,
                            b"draft/glpipm.c\0" as *const u8 as *const libc::c_char,
                            784 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                j += 1;
                j;
            }
            i = 1 as libc::c_int;
            while i <= m {
                *y.offset(i as isize)
                    += gamma_d * (*csa).alfa_max_d * *dy.offset(i as isize);
                i += 1;
                i;
            }
            j = 1 as libc::c_int;
            while j <= n {
                *z.offset(j as isize)
                    += gamma_d * (*csa).alfa_max_d * *dz.offset(j as isize);
                (*z.offset(j as isize) > 0.0f64
                    || {
                        glp_assert_(
                            b"z[j] > 0.0\0" as *const u8 as *const libc::c_char,
                            b"draft/glpipm.c\0" as *const u8 as *const libc::c_char,
                            790 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                j += 1;
                j;
            }
        }
    }
    glp_free(p as *mut libc::c_void);
    glp_free(q as *mut libc::c_void);
    glp_free(r as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn terminate(mut csa: *mut csa) {
    glp_free((*csa).D as *mut libc::c_void);
    glp_free((*csa).P as *mut libc::c_void);
    glp_free((*csa).S_ptr as *mut libc::c_void);
    glp_free((*csa).S_ind as *mut libc::c_void);
    glp_free((*csa).S_val as *mut libc::c_void);
    glp_free((*csa).S_diag as *mut libc::c_void);
    glp_free((*csa).U_ptr as *mut libc::c_void);
    glp_free((*csa).U_ind as *mut libc::c_void);
    glp_free((*csa).U_val as *mut libc::c_void);
    glp_free((*csa).U_diag as *mut libc::c_void);
    glp_free((*csa).phi_min as *mut libc::c_void);
    glp_free((*csa).best_x as *mut libc::c_void);
    glp_free((*csa).best_y as *mut libc::c_void);
    glp_free((*csa).best_z as *mut libc::c_void);
    glp_free((*csa).dx_aff as *mut libc::c_void);
    glp_free((*csa).dy_aff as *mut libc::c_void);
    glp_free((*csa).dz_aff as *mut libc::c_void);
    glp_free((*csa).dx_cc as *mut libc::c_void);
    glp_free((*csa).dy_cc as *mut libc::c_void);
    glp_free((*csa).dz_cc as *mut libc::c_void);
}
unsafe extern "C" fn ipm_main(mut csa: *mut csa) -> libc::c_int {
    let mut m: libc::c_int = (*csa).m;
    let mut n: libc::c_int = (*csa).n;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut temp: libc::c_double = 0.;
    if (*(*csa).parm).msg_lev >= 3 as libc::c_int {
        glp_printf(b"Guessing initial point...\n\0" as *const u8 as *const libc::c_char);
    }
    initial_point(csa);
    if (*(*csa).parm).msg_lev >= 3 as libc::c_int {
        glp_printf(b"Optimization begins...\n\0" as *const u8 as *const libc::c_char);
    }
    loop {
        basic_info(csa);
        if (*csa).iter == 0 as libc::c_int {
            (*csa).rmu0 = (*csa).rmu;
        }
        ((*csa).iter <= 100 as libc::c_int
            || {
                glp_assert_(
                    b"csa->iter <= ITER_MAX\0" as *const u8 as *const libc::c_char,
                    b"draft/glpipm.c\0" as *const u8 as *const libc::c_char,
                    863 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if (*csa).iter == 0 as libc::c_int
            || *((*csa).phi_min).offset(((*csa).iter - 1 as libc::c_int) as isize)
                > (*csa).phi
        {
            *((*csa).phi_min).offset((*csa).iter as isize) = (*csa).phi;
            (*csa).best_iter = (*csa).iter;
            j = 1 as libc::c_int;
            while j <= n {
                *((*csa).best_x).offset(j as isize) = *((*csa).x).offset(j as isize);
                j += 1;
                j;
            }
            i = 1 as libc::c_int;
            while i <= m {
                *((*csa).best_y).offset(i as isize) = *((*csa).y).offset(i as isize);
                i += 1;
                i;
            }
            j = 1 as libc::c_int;
            while j <= n {
                *((*csa).best_z).offset(j as isize) = *((*csa).z).offset(j as isize);
                j += 1;
                j;
            }
            (*csa).best_obj = (*csa).obj;
        } else {
            *((*csa).phi_min)
                .offset(
                    (*csa).iter as isize,
                ) = *((*csa).phi_min).offset(((*csa).iter - 1 as libc::c_int) as isize);
        }
        if (*(*csa).parm).msg_lev >= 2 as libc::c_int {
            glp_printf(
                b"%3d: obj = %17.9e; rpi = %8.1e; rdi = %8.1e; gap = %8.1e\n\0"
                    as *const u8 as *const libc::c_char,
                (*csa).iter,
                (*csa).obj,
                (*csa).rpi,
                (*csa).rdi,
                (*csa).gap,
            );
        }
        if (*csa).rpi < 1e-8f64 && (*csa).rdi < 1e-8f64 && (*csa).gap < 1e-8f64 {
            if (*(*csa).parm).msg_lev >= 3 as libc::c_int {
                glp_printf(
                    b"OPTIMAL SOLUTION FOUND\n\0" as *const u8 as *const libc::c_char,
                );
            }
            status = 0 as libc::c_int;
            break;
        } else {
            temp = 1e5f64 * *((*csa).phi_min).offset((*csa).iter as isize);
            if temp < 1e-8f64 {
                temp = 1e-8f64;
            }
            if (*csa).phi >= temp {
                if (*(*csa).parm).msg_lev >= 3 as libc::c_int {
                    glp_printf(
                        b"PROBLEM HAS NO FEASIBLE PRIMAL/DUAL SOLUTION\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                status = 1 as libc::c_int;
                break;
            } else if ((*csa).rpi >= 1e-8f64 || (*csa).rdi >= 1e-8f64)
                && (*csa).rmu / (*csa).rmu0 >= 1e6f64
                || (*csa).iter >= 30 as libc::c_int
                    && *((*csa).phi_min).offset((*csa).iter as isize)
                        >= 0.5f64
                            * *((*csa).phi_min)
                                .offset(((*csa).iter - 30 as libc::c_int) as isize)
            {
                if (*(*csa).parm).msg_lev >= 3 as libc::c_int {
                    glp_printf(
                        b"NO CONVERGENCE; SEARCH TERMINATED\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                status = 2 as libc::c_int;
                break;
            } else if (*csa).iter == 100 as libc::c_int {
                if (*(*csa).parm).msg_lev >= 3 as libc::c_int {
                    glp_printf(
                        b"ITERATION LIMIT EXCEEDED; SEARCH TERMINATED\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                status = 3 as libc::c_int;
                break;
            } else {
                (*csa).iter += 1;
                (*csa).iter;
                j = 1 as libc::c_int;
                while j <= n {
                    *((*csa).D)
                        .offset(
                            j as isize,
                        ) = *((*csa).x).offset(j as isize)
                        / *((*csa).z).offset(j as isize);
                    j += 1;
                    j;
                }
                decomp_NE(csa);
                if !(make_step(csa) != 0) {
                    continue;
                }
                if (*(*csa).parm).msg_lev >= 3 as libc::c_int {
                    glp_printf(
                        b"NUMERIC INSTABILITY; SEARCH TERMINATED\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                status = 4 as libc::c_int;
                break;
            }
        }
    }
    if status != 0 as libc::c_int {
        j = 1 as libc::c_int;
        while j <= n {
            *((*csa).x).offset(j as isize) = *((*csa).best_x).offset(j as isize);
            j += 1;
            j;
        }
        i = 1 as libc::c_int;
        while i <= m {
            *((*csa).y).offset(i as isize) = *((*csa).best_y).offset(i as isize);
            i += 1;
            i;
        }
        j = 1 as libc::c_int;
        while j <= n {
            *((*csa).z).offset(j as isize) = *((*csa).best_z).offset(j as isize);
            j += 1;
            j;
        }
        if (*(*csa).parm).msg_lev >= 3 as libc::c_int {
            glp_printf(
                b"Best point %17.9e was reached on iteration %d\n\0" as *const u8
                    as *const libc::c_char,
                (*csa).best_obj,
                (*csa).best_iter,
            );
        }
    }
    return status;
}
pub unsafe extern "C" fn _glp_ipm_solve(
    mut P: *mut glp_prob,
    mut parm: *const glp_iptcp,
) -> libc::c_int {
    let mut _dsa: csa = csa {
        m: 0,
        n: 0,
        A_ptr: 0 as *mut libc::c_int,
        A_ind: 0 as *mut libc::c_int,
        A_val: 0 as *mut libc::c_double,
        b: 0 as *mut libc::c_double,
        c: 0 as *mut libc::c_double,
        x: 0 as *mut libc::c_double,
        y: 0 as *mut libc::c_double,
        z: 0 as *mut libc::c_double,
        parm: 0 as *const glp_iptcp,
        D: 0 as *mut libc::c_double,
        P: 0 as *mut libc::c_int,
        S_ptr: 0 as *mut libc::c_int,
        S_ind: 0 as *mut libc::c_int,
        S_val: 0 as *mut libc::c_double,
        S_diag: 0 as *mut libc::c_double,
        U_ptr: 0 as *mut libc::c_int,
        U_ind: 0 as *mut libc::c_int,
        U_val: 0 as *mut libc::c_double,
        U_diag: 0 as *mut libc::c_double,
        iter: 0,
        obj: 0.,
        rpi: 0.,
        rdi: 0.,
        gap: 0.,
        phi: 0.,
        mu: 0.,
        rmu: 0.,
        rmu0: 0.,
        phi_min: 0 as *mut libc::c_double,
        best_iter: 0,
        best_x: 0 as *mut libc::c_double,
        best_y: 0 as *mut libc::c_double,
        best_z: 0 as *mut libc::c_double,
        best_obj: 0.,
        dx_aff: 0 as *mut libc::c_double,
        dy_aff: 0 as *mut libc::c_double,
        dz_aff: 0 as *mut libc::c_double,
        alfa_aff_p: 0.,
        alfa_aff_d: 0.,
        mu_aff: 0.,
        sigma: 0.,
        dx_cc: 0 as *mut libc::c_double,
        dy_cc: 0 as *mut libc::c_double,
        dz_cc: 0 as *mut libc::c_double,
        dx: 0 as *mut libc::c_double,
        dy: 0 as *mut libc::c_double,
        dz: 0 as *mut libc::c_double,
        alfa_max_p: 0.,
        alfa_max_d: 0.,
    };
    let mut csa: *mut csa = &mut _dsa;
    let mut m: libc::c_int = (*P).m;
    let mut n: libc::c_int = (*P).n;
    let mut nnz: libc::c_int = (*P).nnz;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut loc: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut A_ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut A_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut dir: libc::c_double = 0.;
    let mut A_val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut b: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut c: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut z: *mut libc::c_double = 0 as *mut libc::c_double;
    (m > 0 as libc::c_int
        || {
            glp_assert_(
                b"m > 0\0" as *const u8 as *const libc::c_char,
                b"draft/glpipm.c\0" as *const u8 as *const libc::c_char,
                1031 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (n > 0 as libc::c_int
        || {
            glp_assert_(
                b"n > 0\0" as *const u8 as *const libc::c_char,
                b"draft/glpipm.c\0" as *const u8 as *const libc::c_char,
                1032 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    A_ptr = glp_alloc(
        1 as libc::c_int + m + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    A_ind = glp_alloc(
        1 as libc::c_int + nnz,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    A_val = glp_alloc(
        1 as libc::c_int + nnz,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    b = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    c = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    x = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    y = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    z = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    loc = 1 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= m {
        row = *((*P).row).offset(i as isize);
        ((*row).type_0 == 5 as libc::c_int
            || {
                glp_assert_(
                    b"row->type == GLP_FX\0" as *const u8 as *const libc::c_char,
                    b"draft/glpipm.c\0" as *const u8 as *const libc::c_char,
                    1046 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        *b.offset(i as isize) = (*row).lb * (*row).rii;
        *A_ptr.offset(i as isize) = loc;
        aij = (*row).ptr;
        while !aij.is_null() {
            *A_ind.offset(loc as isize) = (*(*aij).col).j;
            *A_val.offset(loc as isize) = (*row).rii * (*aij).val * (*(*aij).col).sjj;
            loc += 1;
            loc;
            aij = (*aij).r_next;
        }
        i += 1;
        i;
    }
    *A_ptr.offset((m + 1 as libc::c_int) as isize) = loc;
    (loc - 1 as libc::c_int == nnz
        || {
            glp_assert_(
                b"loc-1 == nnz\0" as *const u8 as *const libc::c_char,
                b"draft/glpipm.c\0" as *const u8 as *const libc::c_char,
                1056 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*P).dir == 1 as libc::c_int {
        dir = 1.0f64;
    } else if (*P).dir == 2 as libc::c_int {
        dir = -1.0f64;
    } else {
        (P != P
            || {
                glp_assert_(
                    b"P != P\0" as *const u8 as *const libc::c_char,
                    b"draft/glpipm.c\0" as *const u8 as *const libc::c_char,
                    1063 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    *c.offset(0 as libc::c_int as isize) = dir * (*P).c0;
    j = 1 as libc::c_int;
    while j <= n {
        col = *((*P).col).offset(j as isize);
        ((*col).type_0 == 2 as libc::c_int && (*col).lb == 0.0f64
            || {
                glp_assert_(
                    b"col->type == GLP_LO && col->lb == 0.0\0" as *const u8
                        as *const libc::c_char,
                    b"draft/glpipm.c\0" as *const u8 as *const libc::c_char,
                    1067 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        *c.offset(j as isize) = dir * (*col).coef * (*col).sjj;
        j += 1;
        j;
    }
    (*csa).m = m;
    (*csa).n = n;
    (*csa).A_ptr = A_ptr;
    (*csa).A_ind = A_ind;
    (*csa).A_val = A_val;
    (*csa).b = b;
    (*csa).c = c;
    (*csa).x = x;
    (*csa).y = y;
    (*csa).z = z;
    (*csa).parm = parm;
    initialize(csa);
    ret = ipm_main(csa);
    terminate(csa);
    if ret == 0 as libc::c_int {
        (*P).ipt_stat = 5 as libc::c_int;
        ret = 0 as libc::c_int;
    } else if ret == 1 as libc::c_int {
        (*P).ipt_stat = 4 as libc::c_int;
        ret = 0 as libc::c_int;
    } else if ret == 2 as libc::c_int {
        (*P).ipt_stat = 3 as libc::c_int;
        ret = 0x10 as libc::c_int;
    } else if ret == 3 as libc::c_int {
        (*P).ipt_stat = 3 as libc::c_int;
        ret = 0x8 as libc::c_int;
    } else if ret == 4 as libc::c_int {
        (*P).ipt_stat = 3 as libc::c_int;
        ret = 0x11 as libc::c_int;
    } else {
        (ret != ret
            || {
                glp_assert_(
                    b"ret != ret\0" as *const u8 as *const libc::c_char,
                    b"draft/glpipm.c\0" as *const u8 as *const libc::c_char,
                    1114 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    i = 1 as libc::c_int;
    while i <= m {
        row = *((*P).row).offset(i as isize);
        (*row).pval = (*row).lb;
        (*row).dval = dir * *y.offset(i as isize) * (*row).rii;
        i += 1;
        i;
    }
    (*P).ipt_obj = (*P).c0;
    j = 1 as libc::c_int;
    while j <= n {
        col = *((*P).col).offset(j as isize);
        (*col).pval = *x.offset(j as isize) * (*col).sjj;
        (*col).dval = dir * *z.offset(j as isize) / (*col).sjj;
        (*P).ipt_obj += (*col).coef * (*col).pval;
        j += 1;
        j;
    }
    glp_free(A_ptr as *mut libc::c_void);
    glp_free(A_ind as *mut libc::c_void);
    glp_free(A_val as *mut libc::c_void);
    glp_free(b as *mut libc::c_void);
    glp_free(c as *mut libc::c_void);
    glp_free(x as *mut libc::c_void);
    glp_free(y as *mut libc::c_void);
    glp_free(z as *mut libc::c_void);
    return ret;
}
