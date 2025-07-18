use ::libc;
extern "C" {
    pub type BFX;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn _glp_mpq_init() -> mpq_t;
    fn _glp_mpq_clear(x: mpq_t);
    fn _glp_mpq_set(z: mpq_t, x: mpq_t);
    fn _glp_mpq_set_si(x: mpq_t, p: libc::c_int, q: libc::c_uint);
    fn _glp_mpq_get_d(x: mpq_t) -> libc::c_double;
    fn _glp_mpq_add(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_sub(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_mul(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_div(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_neg(z: mpq_t, x: mpq_t);
    fn _glp_mpq_abs(z: mpq_t, x: mpq_t);
    fn _glp_mpq_cmp(x: mpq_t, y: mpq_t) -> libc::c_int;
    fn _glp_mpq_sgn(x: mpq_t) -> libc::c_int;
    fn _glp_bfx_create_binv() -> *mut BFX;
    fn _glp_bfx_factorize(
        binv: *mut BFX,
        m: libc::c_int,
        col: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                libc::c_int,
                *mut libc::c_int,
                *mut mpq_t,
            ) -> libc::c_int,
        >,
        info: *mut libc::c_void,
    ) -> libc::c_int;
    fn _glp_bfx_ftran(binv: *mut BFX, x: *mut mpq_t, save: libc::c_int);
    fn _glp_bfx_btran(binv: *mut BFX, x: *mut mpq_t);
    fn _glp_bfx_update(binv: *mut BFX, j: libc::c_int) -> libc::c_int;
    fn _glp_bfx_delete_binv(binv: *mut BFX);
}
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpq {
    pub p: mpz,
    pub q: mpz,
}
pub type mpq_t = *mut mpq;
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
pub unsafe extern "C" fn _glp_ssx_create(
    mut m: libc::c_int,
    mut n: libc::c_int,
    mut nnz: libc::c_int,
) -> *mut SSX {
    let mut ssx: *mut SSX = 0 as *mut SSX;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if m < 1 as libc::c_int {
        (glp_error_(
            b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
        ))
            .unwrap()(
            b"ssx_create: m = %d; invalid number of rows\n\0" as *const u8
                as *const libc::c_char,
            m,
        );
    }
    if n < 1 as libc::c_int {
        (glp_error_(
            b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
        ))
            .unwrap()(
            b"ssx_create: n = %d; invalid number of columns\n\0" as *const u8
                as *const libc::c_char,
            n,
        );
    }
    if nnz < 0 as libc::c_int {
        (glp_error_(
            b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int,
        ))
            .unwrap()(
            b"ssx_create: nnz = %d; invalid number of non-zero constraint coefficients\n\0"
                as *const u8 as *const libc::c_char,
            nnz,
        );
    }
    ssx = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<SSX>() as libc::c_ulong as libc::c_int,
    ) as *mut SSX;
    (*ssx).m = m;
    (*ssx).n = n;
    (*ssx)
        .type_0 = glp_alloc(
        1 as libc::c_int + m + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*ssx)
        .lb = glp_alloc(
        1 as libc::c_int + m + n,
        ::std::mem::size_of::<mpq_t>() as libc::c_ulong as libc::c_int,
    ) as *mut mpq_t;
    k = 1 as libc::c_int;
    while k <= m + n {
        let ref mut fresh0 = *((*ssx).lb).offset(k as isize);
        *fresh0 = _glp_mpq_init();
        k += 1;
        k;
    }
    (*ssx)
        .ub = glp_alloc(
        1 as libc::c_int + m + n,
        ::std::mem::size_of::<mpq_t>() as libc::c_ulong as libc::c_int,
    ) as *mut mpq_t;
    k = 1 as libc::c_int;
    while k <= m + n {
        let ref mut fresh1 = *((*ssx).ub).offset(k as isize);
        *fresh1 = _glp_mpq_init();
        k += 1;
        k;
    }
    (*ssx)
        .coef = glp_alloc(
        1 as libc::c_int + m + n,
        ::std::mem::size_of::<mpq_t>() as libc::c_ulong as libc::c_int,
    ) as *mut mpq_t;
    k = 0 as libc::c_int;
    while k <= m + n {
        let ref mut fresh2 = *((*ssx).coef).offset(k as isize);
        *fresh2 = _glp_mpq_init();
        k += 1;
        k;
    }
    (*ssx)
        .A_ptr = glp_alloc(
        1 as libc::c_int + n + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    *((*ssx).A_ptr).offset((n + 1 as libc::c_int) as isize) = nnz + 1 as libc::c_int;
    (*ssx)
        .A_ind = glp_alloc(
        1 as libc::c_int + nnz,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*ssx)
        .A_val = glp_alloc(
        1 as libc::c_int + nnz,
        ::std::mem::size_of::<mpq_t>() as libc::c_ulong as libc::c_int,
    ) as *mut mpq_t;
    k = 1 as libc::c_int;
    while k <= nnz {
        let ref mut fresh3 = *((*ssx).A_val).offset(k as isize);
        *fresh3 = _glp_mpq_init();
        k += 1;
        k;
    }
    (*ssx)
        .stat = glp_alloc(
        1 as libc::c_int + m + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*ssx)
        .Q_row = glp_alloc(
        1 as libc::c_int + m + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*ssx)
        .Q_col = glp_alloc(
        1 as libc::c_int + m + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*ssx).binv = _glp_bfx_create_binv();
    (*ssx)
        .bbar = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<mpq_t>() as libc::c_ulong as libc::c_int,
    ) as *mut mpq_t;
    i = 0 as libc::c_int;
    while i <= m {
        let ref mut fresh4 = *((*ssx).bbar).offset(i as isize);
        *fresh4 = _glp_mpq_init();
        i += 1;
        i;
    }
    (*ssx)
        .pi = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<mpq_t>() as libc::c_ulong as libc::c_int,
    ) as *mut mpq_t;
    i = 1 as libc::c_int;
    while i <= m {
        let ref mut fresh5 = *((*ssx).pi).offset(i as isize);
        *fresh5 = _glp_mpq_init();
        i += 1;
        i;
    }
    (*ssx)
        .cbar = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<mpq_t>() as libc::c_ulong as libc::c_int,
    ) as *mut mpq_t;
    j = 1 as libc::c_int;
    while j <= n {
        let ref mut fresh6 = *((*ssx).cbar).offset(j as isize);
        *fresh6 = _glp_mpq_init();
        j += 1;
        j;
    }
    (*ssx)
        .rho = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<mpq_t>() as libc::c_ulong as libc::c_int,
    ) as *mut mpq_t;
    i = 1 as libc::c_int;
    while i <= m {
        let ref mut fresh7 = *((*ssx).rho).offset(i as isize);
        *fresh7 = _glp_mpq_init();
        i += 1;
        i;
    }
    (*ssx)
        .ap = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<mpq_t>() as libc::c_ulong as libc::c_int,
    ) as *mut mpq_t;
    j = 1 as libc::c_int;
    while j <= n {
        let ref mut fresh8 = *((*ssx).ap).offset(j as isize);
        *fresh8 = _glp_mpq_init();
        j += 1;
        j;
    }
    (*ssx)
        .aq = glp_alloc(
        1 as libc::c_int + m,
        ::std::mem::size_of::<mpq_t>() as libc::c_ulong as libc::c_int,
    ) as *mut mpq_t;
    i = 1 as libc::c_int;
    while i <= m {
        let ref mut fresh9 = *((*ssx).aq).offset(i as isize);
        *fresh9 = _glp_mpq_init();
        i += 1;
        i;
    }
    (*ssx).delta = _glp_mpq_init();
    return ssx;
}
unsafe extern "C" fn basis_col(
    mut info: *mut libc::c_void,
    mut j: libc::c_int,
    mut ind: *mut libc::c_int,
    mut val: *mut mpq_t,
) -> libc::c_int {
    let mut ssx: *mut SSX = info as *mut SSX;
    let mut m: libc::c_int = (*ssx).m;
    let mut n: libc::c_int = (*ssx).n;
    let mut A_ptr: *mut libc::c_int = (*ssx).A_ptr;
    let mut A_ind: *mut libc::c_int = (*ssx).A_ind;
    let mut A_val: *mut mpq_t = (*ssx).A_val;
    let mut Q_col: *mut libc::c_int = (*ssx).Q_col;
    let mut k: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    (1 as libc::c_int <= j && j <= m
        || {
            glp_assert_(
                b"1 <= j && j <= m\0" as *const u8 as *const libc::c_char,
                b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                101 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = *Q_col.offset(j as isize);
    (1 as libc::c_int <= k && k <= m + n
        || {
            glp_assert_(
                b"1 <= k && k <= m+n\0" as *const u8 as *const libc::c_char,
                b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                103 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if k <= m {
        len = 1 as libc::c_int;
        *ind.offset(1 as libc::c_int as isize) = k;
        _glp_mpq_set_si(
            *val.offset(1 as libc::c_int as isize),
            1 as libc::c_int,
            1 as libc::c_int as libc::c_uint,
        );
    } else {
        len = 0 as libc::c_int;
        ptr = *A_ptr.offset((k - m) as isize);
        while ptr < *A_ptr.offset((k - m + 1 as libc::c_int) as isize) {
            len += 1;
            len;
            *ind.offset(len as isize) = *A_ind.offset(ptr as isize);
            _glp_mpq_neg(*val.offset(len as isize), *A_val.offset(ptr as isize));
            ptr += 1;
            ptr;
        }
    }
    return len;
}
pub unsafe extern "C" fn _glp_ssx_factorize(mut ssx: *mut SSX) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = _glp_bfx_factorize(
        (*ssx).binv,
        (*ssx).m,
        Some(
            basis_col
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_int,
                    *mut libc::c_int,
                    *mut mpq_t,
                ) -> libc::c_int,
        ),
        ssx as *mut libc::c_void,
    );
    return ret;
}
pub unsafe extern "C" fn _glp_ssx_get_xNj(
    mut ssx: *mut SSX,
    mut j: libc::c_int,
    mut x: mpq_t,
) {
    let mut m: libc::c_int = (*ssx).m;
    let mut n: libc::c_int = (*ssx).n;
    let mut lb: *mut mpq_t = (*ssx).lb;
    let mut ub: *mut mpq_t = (*ssx).ub;
    let mut stat: *mut libc::c_int = (*ssx).stat;
    let mut Q_col: *mut libc::c_int = (*ssx).Q_col;
    let mut k: libc::c_int = 0;
    (1 as libc::c_int <= j && j <= n
        || {
            glp_assert_(
                b"1 <= j && j <= n\0" as *const u8 as *const libc::c_char,
                b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                149 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = *Q_col.offset((m + j) as isize);
    (1 as libc::c_int <= k && k <= m + n
        || {
            glp_assert_(
                b"1 <= k && k <= m+n\0" as *const u8 as *const libc::c_char,
                b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                151 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    match *stat.offset(k as isize) {
        1 => {
            _glp_mpq_set(x, *lb.offset(k as isize));
        }
        2 => {
            _glp_mpq_set(x, *ub.offset(k as isize));
        }
        3 => {
            _glp_mpq_set_si(x, 0 as libc::c_int, 1 as libc::c_int as libc::c_uint);
        }
        4 => {
            _glp_mpq_set(x, *lb.offset(k as isize));
        }
        _ => {
            (stat != stat
                || {
                    glp_assert_(
                        b"stat != stat\0" as *const u8 as *const libc::c_char,
                        b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                        166 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn _glp_ssx_eval_bbar(mut ssx: *mut SSX) {
    let mut m: libc::c_int = (*ssx).m;
    let mut n: libc::c_int = (*ssx).n;
    let mut coef: *mut mpq_t = (*ssx).coef;
    let mut A_ptr: *mut libc::c_int = (*ssx).A_ptr;
    let mut A_ind: *mut libc::c_int = (*ssx).A_ind;
    let mut A_val: *mut mpq_t = (*ssx).A_val;
    let mut Q_col: *mut libc::c_int = (*ssx).Q_col;
    let mut bbar: *mut mpq_t = (*ssx).bbar;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut x: mpq_t = 0 as *mut mpq;
    let mut temp: mpq_t = 0 as *mut mpq;
    x = _glp_mpq_init();
    temp = _glp_mpq_init();
    i = 1 as libc::c_int;
    while i <= m {
        _glp_mpq_set_si(
            *bbar.offset(i as isize),
            0 as libc::c_int,
            1 as libc::c_int as libc::c_uint,
        );
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= n {
        _glp_ssx_get_xNj(ssx, j, x);
        if !(_glp_mpq_sgn(x) == 0 as libc::c_int) {
            k = *Q_col.offset((m + j) as isize);
            if k <= m {
                _glp_mpq_sub(*bbar.offset(k as isize), *bbar.offset(k as isize), x);
            } else {
                ptr = *A_ptr.offset((k - m) as isize);
                while ptr < *A_ptr.offset((k - m + 1 as libc::c_int) as isize) {
                    _glp_mpq_mul(temp, *A_val.offset(ptr as isize), x);
                    _glp_mpq_add(
                        *bbar.offset(*A_ind.offset(ptr as isize) as isize),
                        *bbar.offset(*A_ind.offset(ptr as isize) as isize),
                        temp,
                    );
                    ptr += 1;
                    ptr;
                }
            }
        }
        j += 1;
        j;
    }
    _glp_bfx_ftran((*ssx).binv, bbar, 0 as libc::c_int);
    _glp_mpq_set(
        *bbar.offset(0 as libc::c_int as isize),
        *coef.offset(0 as libc::c_int as isize),
    );
    i = 1 as libc::c_int;
    while i <= m {
        k = *Q_col.offset(i as isize);
        if !(_glp_mpq_sgn(*coef.offset(k as isize)) == 0 as libc::c_int) {
            _glp_mpq_mul(temp, *coef.offset(k as isize), *bbar.offset(i as isize));
            _glp_mpq_add(
                *bbar.offset(0 as libc::c_int as isize),
                *bbar.offset(0 as libc::c_int as isize),
                temp,
            );
        }
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= n {
        k = *Q_col.offset((m + j) as isize);
        if !(_glp_mpq_sgn(*coef.offset(k as isize)) == 0 as libc::c_int) {
            _glp_ssx_get_xNj(ssx, j, x);
            _glp_mpq_mul(temp, *coef.offset(k as isize), x);
            _glp_mpq_add(
                *bbar.offset(0 as libc::c_int as isize),
                *bbar.offset(0 as libc::c_int as isize),
                temp,
            );
        }
        j += 1;
        j;
    }
    _glp_mpq_clear(x);
    _glp_mpq_clear(temp);
}
pub unsafe extern "C" fn _glp_ssx_eval_pi(mut ssx: *mut SSX) {
    let mut m: libc::c_int = (*ssx).m;
    let mut coef: *mut mpq_t = (*ssx).coef;
    let mut Q_col: *mut libc::c_int = (*ssx).Q_col;
    let mut pi: *mut mpq_t = (*ssx).pi;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= m {
        _glp_mpq_set(
            *pi.offset(i as isize),
            *coef.offset(*Q_col.offset(i as isize) as isize),
        );
        i += 1;
        i;
    }
    _glp_bfx_btran((*ssx).binv, pi);
}
pub unsafe extern "C" fn _glp_ssx_eval_dj(
    mut ssx: *mut SSX,
    mut j: libc::c_int,
    mut dj: mpq_t,
) {
    let mut m: libc::c_int = (*ssx).m;
    let mut n: libc::c_int = (*ssx).n;
    let mut coef: *mut mpq_t = (*ssx).coef;
    let mut A_ptr: *mut libc::c_int = (*ssx).A_ptr;
    let mut A_ind: *mut libc::c_int = (*ssx).A_ind;
    let mut A_val: *mut mpq_t = (*ssx).A_val;
    let mut Q_col: *mut libc::c_int = (*ssx).Q_col;
    let mut pi: *mut mpq_t = (*ssx).pi;
    let mut k: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut temp: mpq_t = 0 as *mut mpq;
    temp = _glp_mpq_init();
    (1 as libc::c_int <= j && j <= n
        || {
            glp_assert_(
                b"1 <= j && j <= n\0" as *const u8 as *const libc::c_char,
                b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                290 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = *Q_col.offset((m + j) as isize);
    (1 as libc::c_int <= k && k <= m + n
        || {
            glp_assert_(
                b"1 <= k && k <= m+n\0" as *const u8 as *const libc::c_char,
                b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                292 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if k <= m {
        _glp_mpq_sub(dj, *coef.offset(k as isize), *pi.offset(k as isize));
    } else {
        _glp_mpq_set(dj, *coef.offset(k as isize));
        ptr = *A_ptr.offset((k - m) as isize);
        end = *A_ptr.offset((k - m + 1 as libc::c_int) as isize);
        while ptr < end {
            _glp_mpq_mul(
                temp,
                *A_val.offset(ptr as isize),
                *pi.offset(*A_ind.offset(ptr as isize) as isize),
            );
            _glp_mpq_add(dj, dj, temp);
            ptr += 1;
            ptr;
        }
    }
    _glp_mpq_clear(temp);
}
pub unsafe extern "C" fn _glp_ssx_eval_cbar(mut ssx: *mut SSX) {
    let mut n: libc::c_int = (*ssx).n;
    let mut cbar: *mut mpq_t = (*ssx).cbar;
    let mut j: libc::c_int = 0;
    j = 1 as libc::c_int;
    while j <= n {
        _glp_ssx_eval_dj(ssx, j, *cbar.offset(j as isize));
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn _glp_ssx_eval_rho(mut ssx: *mut SSX) {
    let mut m: libc::c_int = (*ssx).m;
    let mut p: libc::c_int = (*ssx).p;
    let mut rho: *mut mpq_t = (*ssx).rho;
    let mut i: libc::c_int = 0;
    (1 as libc::c_int <= p && p <= m
        || {
            glp_assert_(
                b"1 <= p && p <= m\0" as *const u8 as *const libc::c_char,
                b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                344 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    i = 1 as libc::c_int;
    while i <= m {
        _glp_mpq_set_si(
            *rho.offset(i as isize),
            0 as libc::c_int,
            1 as libc::c_int as libc::c_uint,
        );
        i += 1;
        i;
    }
    _glp_mpq_set_si(
        *rho.offset(p as isize),
        1 as libc::c_int,
        1 as libc::c_int as libc::c_uint,
    );
    _glp_bfx_btran((*ssx).binv, rho);
}
pub unsafe extern "C" fn _glp_ssx_eval_row(mut ssx: *mut SSX) {
    let mut m: libc::c_int = (*ssx).m;
    let mut n: libc::c_int = (*ssx).n;
    let mut A_ptr: *mut libc::c_int = (*ssx).A_ptr;
    let mut A_ind: *mut libc::c_int = (*ssx).A_ind;
    let mut A_val: *mut mpq_t = (*ssx).A_val;
    let mut Q_col: *mut libc::c_int = (*ssx).Q_col;
    let mut rho: *mut mpq_t = (*ssx).rho;
    let mut ap: *mut mpq_t = (*ssx).ap;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut temp: mpq_t = 0 as *mut mpq;
    temp = _glp_mpq_init();
    j = 1 as libc::c_int;
    while j <= n {
        k = *Q_col.offset((m + j) as isize);
        if k <= m {
            _glp_mpq_neg(*ap.offset(j as isize), *rho.offset(k as isize));
        } else {
            _glp_mpq_set_si(
                *ap.offset(j as isize),
                0 as libc::c_int,
                1 as libc::c_int as libc::c_uint,
            );
            ptr = *A_ptr.offset((k - m) as isize);
            while ptr < *A_ptr.offset((k - m + 1 as libc::c_int) as isize) {
                _glp_mpq_mul(
                    temp,
                    *A_val.offset(ptr as isize),
                    *rho.offset(*A_ind.offset(ptr as isize) as isize),
                );
                _glp_mpq_add(*ap.offset(j as isize), *ap.offset(j as isize), temp);
                ptr += 1;
                ptr;
            }
        }
        j += 1;
        j;
    }
    _glp_mpq_clear(temp);
}
pub unsafe extern "C" fn _glp_ssx_eval_col(mut ssx: *mut SSX) {
    let mut m: libc::c_int = (*ssx).m;
    let mut n: libc::c_int = (*ssx).n;
    let mut A_ptr: *mut libc::c_int = (*ssx).A_ptr;
    let mut A_ind: *mut libc::c_int = (*ssx).A_ind;
    let mut A_val: *mut mpq_t = (*ssx).A_val;
    let mut Q_col: *mut libc::c_int = (*ssx).Q_col;
    let mut q: libc::c_int = (*ssx).q;
    let mut aq: *mut mpq_t = (*ssx).aq;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    (1 as libc::c_int <= q && q <= n
        || {
            glp_assert_(
                b"1 <= q && q <= n\0" as *const u8 as *const libc::c_char,
                b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                415 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    i = 1 as libc::c_int;
    while i <= m {
        _glp_mpq_set_si(
            *aq.offset(i as isize),
            0 as libc::c_int,
            1 as libc::c_int as libc::c_uint,
        );
        i += 1;
        i;
    }
    k = *Q_col.offset((m + q) as isize);
    if k <= m {
        _glp_mpq_set_si(
            *aq.offset(k as isize),
            1 as libc::c_int,
            1 as libc::c_int as libc::c_uint,
        );
    } else {
        ptr = *A_ptr.offset((k - m) as isize);
        while ptr < *A_ptr.offset((k - m + 1 as libc::c_int) as isize) {
            _glp_mpq_neg(
                *aq.offset(*A_ind.offset(ptr as isize) as isize),
                *A_val.offset(ptr as isize),
            );
            ptr += 1;
            ptr;
        }
    }
    _glp_bfx_ftran((*ssx).binv, aq, 1 as libc::c_int);
    i = 1 as libc::c_int;
    while i <= m {
        _glp_mpq_neg(*aq.offset(i as isize), *aq.offset(i as isize));
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn _glp_ssx_chuzc(mut ssx: *mut SSX) {
    let mut m: libc::c_int = (*ssx).m;
    let mut n: libc::c_int = (*ssx).n;
    let mut dir: libc::c_int = if (*ssx).dir == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    let mut Q_col: *mut libc::c_int = (*ssx).Q_col;
    let mut stat: *mut libc::c_int = (*ssx).stat;
    let mut cbar: *mut mpq_t = (*ssx).cbar;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    let mut q_dir: libc::c_int = 0;
    let mut best: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    q = 0 as libc::c_int;
    q_dir = 0 as libc::c_int;
    best = 0.0f64;
    j = 1 as libc::c_int;
    while j <= n {
        k = *Q_col.offset((m + j) as isize);
        s = dir * _glp_mpq_sgn(*cbar.offset(j as isize));
        if (*stat.offset(k as isize) == 3 as libc::c_int
            || *stat.offset(k as isize) == 1 as libc::c_int) && s < 0 as libc::c_int
            || (*stat.offset(k as isize) == 3 as libc::c_int
                || *stat.offset(k as isize) == 2 as libc::c_int) && s > 0 as libc::c_int
        {
            temp = fabs(_glp_mpq_get_d(*cbar.offset(j as isize)));
            (temp != 0.0f64
                || {
                    glp_assert_(
                        b"temp != 0.0\0" as *const u8 as *const libc::c_char,
                        b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                        474 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if q == 0 as libc::c_int || best < temp {
                q = j;
                q_dir = -s;
                best = temp;
            }
        }
        j += 1;
        j;
    }
    (*ssx).q = q;
    (*ssx).q_dir = q_dir;
}
pub unsafe extern "C" fn _glp_ssx_chuzr(mut ssx: *mut SSX) {
    let mut m: libc::c_int = (*ssx).m;
    let mut n: libc::c_int = (*ssx).n;
    let mut type_0: *mut libc::c_int = (*ssx).type_0;
    let mut lb: *mut mpq_t = (*ssx).lb;
    let mut ub: *mut mpq_t = (*ssx).ub;
    let mut Q_col: *mut libc::c_int = (*ssx).Q_col;
    let mut bbar: *mut mpq_t = (*ssx).bbar;
    let mut q: libc::c_int = (*ssx).q;
    let mut aq: *mut mpq_t = (*ssx).aq;
    let mut q_dir: libc::c_int = (*ssx).q_dir;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut p_stat: libc::c_int = 0;
    let mut teta: mpq_t = 0 as *mut mpq;
    let mut temp: mpq_t = 0 as *mut mpq;
    teta = _glp_mpq_init();
    temp = _glp_mpq_init();
    (1 as libc::c_int <= q && q <= n
        || {
            glp_assert_(
                b"1 <= q && q <= n\0" as *const u8 as *const libc::c_char,
                b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                517 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (q_dir == 1 as libc::c_int || q_dir == -(1 as libc::c_int)
        || {
            glp_assert_(
                b"q_dir == +1 || q_dir == -1\0" as *const u8 as *const libc::c_char,
                b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                518 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    p = 0 as libc::c_int;
    p_stat = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= m {
        s = q_dir * _glp_mpq_sgn(*aq.offset(i as isize));
        if s < 0 as libc::c_int {
            k = *Q_col.offset(i as isize);
            t = *type_0.offset(k as isize);
            if t == 1 as libc::c_int || t == 3 as libc::c_int || t == 4 as libc::c_int {
                _glp_mpq_sub(temp, *bbar.offset(i as isize), *lb.offset(k as isize));
                _glp_mpq_div(temp, temp, *aq.offset(i as isize));
                _glp_mpq_abs(temp, temp);
                if p == 0 as libc::c_int || _glp_mpq_cmp(teta, temp) > 0 as libc::c_int {
                    p = i;
                    p_stat = if t == 4 as libc::c_int {
                        4 as libc::c_int
                    } else {
                        1 as libc::c_int
                    };
                    _glp_mpq_set(teta, temp);
                }
            }
        } else if s > 0 as libc::c_int {
            k = *Q_col.offset(i as isize);
            t = *type_0.offset(k as isize);
            if t == 2 as libc::c_int || t == 3 as libc::c_int || t == 4 as libc::c_int {
                _glp_mpq_sub(temp, *bbar.offset(i as isize), *ub.offset(k as isize));
                _glp_mpq_div(temp, temp, *aq.offset(i as isize));
                _glp_mpq_abs(temp, temp);
                if p == 0 as libc::c_int || _glp_mpq_cmp(teta, temp) > 0 as libc::c_int {
                    p = i;
                    p_stat = if t == 4 as libc::c_int {
                        4 as libc::c_int
                    } else {
                        2 as libc::c_int
                    };
                    _glp_mpq_set(teta, temp);
                }
            }
        }
        if p != 0 as libc::c_int && _glp_mpq_sgn(teta) == 0 as libc::c_int {
            break;
        }
        i += 1;
        i;
    }
    k = *Q_col.offset((m + q) as isize);
    if *type_0.offset(k as isize) == 3 as libc::c_int {
        _glp_mpq_sub(temp, *ub.offset(k as isize), *lb.offset(k as isize));
        if p == 0 as libc::c_int || _glp_mpq_cmp(teta, temp) > 0 as libc::c_int {
            p = -(1 as libc::c_int);
            p_stat = -(1 as libc::c_int);
            _glp_mpq_set(teta, temp);
        }
    }
    (*ssx).p = p;
    (*ssx).p_stat = p_stat;
    if p != 0 as libc::c_int {
        (_glp_mpq_sgn(teta) >= 0 as libc::c_int
            || {
                glp_assert_(
                    b"mpq_sgn(teta) >= 0\0" as *const u8 as *const libc::c_char,
                    b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                    576 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if q_dir > 0 as libc::c_int {
            _glp_mpq_set((*ssx).delta, teta);
        } else {
            _glp_mpq_neg((*ssx).delta, teta);
        }
    }
    _glp_mpq_clear(teta);
    _glp_mpq_clear(temp);
}
pub unsafe extern "C" fn _glp_ssx_update_bbar(mut ssx: *mut SSX) {
    let mut m: libc::c_int = (*ssx).m;
    let mut n: libc::c_int = (*ssx).n;
    let mut bbar: *mut mpq_t = (*ssx).bbar;
    let mut cbar: *mut mpq_t = (*ssx).cbar;
    let mut p: libc::c_int = (*ssx).p;
    let mut q: libc::c_int = (*ssx).q;
    let mut aq: *mut mpq_t = (*ssx).aq;
    let mut i: libc::c_int = 0;
    let mut temp: mpq_t = 0 as *mut mpq;
    temp = _glp_mpq_init();
    (1 as libc::c_int <= q && q <= n
        || {
            glp_assert_(
                b"1 <= q && q <= n\0" as *const u8 as *const libc::c_char,
                b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                624 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if !(p < 0 as libc::c_int) {
        (1 as libc::c_int <= p && p <= m
            || {
                glp_assert_(
                    b"1 <= p && p <= m\0" as *const u8 as *const libc::c_char,
                    b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                    632 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        _glp_ssx_get_xNj(ssx, q, temp);
        _glp_mpq_add(*bbar.offset(p as isize), temp, (*ssx).delta);
    }
    i = 1 as libc::c_int;
    while i <= m {
        if !(i == p) {
            if !(_glp_mpq_sgn(*aq.offset(i as isize)) == 0 as libc::c_int) {
                _glp_mpq_mul(temp, *aq.offset(i as isize), (*ssx).delta);
                _glp_mpq_add(*bbar.offset(i as isize), *bbar.offset(i as isize), temp);
            }
        }
        i += 1;
        i;
    }
    _glp_mpq_mul(temp, *cbar.offset(q as isize), (*ssx).delta);
    _glp_mpq_add(
        *bbar.offset(0 as libc::c_int as isize),
        *bbar.offset(0 as libc::c_int as isize),
        temp,
    );
    _glp_mpq_clear(temp);
}
pub unsafe extern "C" fn _glp_ssx_update_pi(mut ssx: *mut SSX) {
    let mut m: libc::c_int = (*ssx).m;
    let mut n: libc::c_int = (*ssx).n;
    let mut pi: *mut mpq_t = (*ssx).pi;
    let mut cbar: *mut mpq_t = (*ssx).cbar;
    let mut p: libc::c_int = (*ssx).p;
    let mut q: libc::c_int = (*ssx).q;
    let mut aq: *mut mpq_t = (*ssx).aq;
    let mut rho: *mut mpq_t = (*ssx).rho;
    let mut i: libc::c_int = 0;
    let mut new_dq: mpq_t = 0 as *mut mpq;
    let mut temp: mpq_t = 0 as *mut mpq;
    new_dq = _glp_mpq_init();
    temp = _glp_mpq_init();
    (1 as libc::c_int <= p && p <= m
        || {
            glp_assert_(
                b"1 <= p && p <= m\0" as *const u8 as *const libc::c_char,
                b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                673 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= q && q <= n
        || {
            glp_assert_(
                b"1 <= q && q <= n\0" as *const u8 as *const libc::c_char,
                b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                674 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_mpq_div(new_dq, *cbar.offset(q as isize), *aq.offset(p as isize));
    i = 1 as libc::c_int;
    while i <= m {
        if !(_glp_mpq_sgn(*rho.offset(i as isize)) == 0 as libc::c_int) {
            _glp_mpq_mul(temp, new_dq, *rho.offset(i as isize));
            _glp_mpq_sub(*pi.offset(i as isize), *pi.offset(i as isize), temp);
        }
        i += 1;
        i;
    }
    _glp_mpq_clear(new_dq);
    _glp_mpq_clear(temp);
}
pub unsafe extern "C" fn _glp_ssx_update_cbar(mut ssx: *mut SSX) {
    let mut m: libc::c_int = (*ssx).m;
    let mut n: libc::c_int = (*ssx).n;
    let mut cbar: *mut mpq_t = (*ssx).cbar;
    let mut p: libc::c_int = (*ssx).p;
    let mut q: libc::c_int = (*ssx).q;
    let mut ap: *mut mpq_t = (*ssx).ap;
    let mut j: libc::c_int = 0;
    let mut temp: mpq_t = 0 as *mut mpq;
    temp = _glp_mpq_init();
    (1 as libc::c_int <= p && p <= m
        || {
            glp_assert_(
                b"1 <= p && p <= m\0" as *const u8 as *const libc::c_char,
                b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                704 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= q && q <= n
        || {
            glp_assert_(
                b"1 <= q && q <= n\0" as *const u8 as *const libc::c_char,
                b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                705 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_mpq_div(
        *cbar.offset(q as isize),
        *cbar.offset(q as isize),
        *ap.offset(q as isize),
    );
    j = 1 as libc::c_int;
    while j <= n {
        if !(j == q) {
            if !(_glp_mpq_sgn(*ap.offset(j as isize)) == 0 as libc::c_int) {
                _glp_mpq_mul(temp, *ap.offset(j as isize), *cbar.offset(q as isize));
                _glp_mpq_sub(*cbar.offset(j as isize), *cbar.offset(j as isize), temp);
            }
        }
        j += 1;
        j;
    }
    _glp_mpq_clear(temp);
}
pub unsafe extern "C" fn _glp_ssx_change_basis(mut ssx: *mut SSX) {
    let mut m: libc::c_int = (*ssx).m;
    let mut n: libc::c_int = (*ssx).n;
    let mut type_0: *mut libc::c_int = (*ssx).type_0;
    let mut stat: *mut libc::c_int = (*ssx).stat;
    let mut Q_row: *mut libc::c_int = (*ssx).Q_row;
    let mut Q_col: *mut libc::c_int = (*ssx).Q_col;
    let mut p: libc::c_int = (*ssx).p;
    let mut q: libc::c_int = (*ssx).q;
    let mut p_stat: libc::c_int = (*ssx).p_stat;
    let mut k: libc::c_int = 0;
    let mut kp: libc::c_int = 0;
    let mut kq: libc::c_int = 0;
    if p < 0 as libc::c_int {
        (1 as libc::c_int <= q && q <= n
            || {
                glp_assert_(
                    b"1 <= q && q <= n\0" as *const u8 as *const libc::c_char,
                    b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                    740 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        k = *Q_col.offset((m + q) as isize);
        (*type_0.offset(k as isize) == 3 as libc::c_int
            || {
                glp_assert_(
                    b"type[k] == SSX_DB\0" as *const u8 as *const libc::c_char,
                    b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                    742 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        match *stat.offset(k as isize) {
            1 => {
                *stat.offset(k as isize) = 2 as libc::c_int;
            }
            2 => {
                *stat.offset(k as isize) = 1 as libc::c_int;
            }
            _ => {
                (stat != stat
                    || {
                        glp_assert_(
                            b"stat != stat\0" as *const u8 as *const libc::c_char,
                            b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                            751 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
    } else {
        (1 as libc::c_int <= p && p <= m
            || {
                glp_assert_(
                    b"1 <= p && p <= m\0" as *const u8 as *const libc::c_char,
                    b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                    756 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (1 as libc::c_int <= q && q <= n
            || {
                glp_assert_(
                    b"1 <= q && q <= n\0" as *const u8 as *const libc::c_char,
                    b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                    757 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        kp = *Q_col.offset(p as isize);
        kq = *Q_col.offset((m + q) as isize);
        match *type_0.offset(kp as isize) {
            0 => {
                (p_stat == 3 as libc::c_int
                    || {
                        glp_assert_(
                            b"p_stat == SSX_NF\0" as *const u8 as *const libc::c_char,
                            b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                            763 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            1 => {
                (p_stat == 1 as libc::c_int
                    || {
                        glp_assert_(
                            b"p_stat == SSX_NL\0" as *const u8 as *const libc::c_char,
                            b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                            766 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            2 => {
                (p_stat == 2 as libc::c_int
                    || {
                        glp_assert_(
                            b"p_stat == SSX_NU\0" as *const u8 as *const libc::c_char,
                            b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                            769 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            3 => {
                (p_stat == 1 as libc::c_int || p_stat == 2 as libc::c_int
                    || {
                        glp_assert_(
                            b"p_stat == SSX_NL || p_stat == SSX_NU\0" as *const u8
                                as *const libc::c_char,
                            b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                            772 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            4 => {
                (p_stat == 4 as libc::c_int
                    || {
                        glp_assert_(
                            b"p_stat == SSX_NS\0" as *const u8 as *const libc::c_char,
                            b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                            775 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            _ => {
                (type_0 != type_0
                    || {
                        glp_assert_(
                            b"type != type\0" as *const u8 as *const libc::c_char,
                            b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                            778 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        *stat.offset(kp as isize) = p_stat as libc::c_char as libc::c_int;
        *stat.offset(kq as isize) = 0 as libc::c_int;
        *Q_row.offset(kp as isize) = m + q;
        *Q_row.offset(kq as isize) = p;
        *Q_col.offset(p as isize) = kq;
        *Q_col.offset((m + q) as isize) = kp;
        if _glp_bfx_update((*ssx).binv, p) != 0 {
            if _glp_ssx_factorize(ssx) != 0 {
                (0 as libc::c_int != 0
                    || {
                        glp_assert_(
                            b"(\"Internal error: basis matrix is singular\", 0)\0"
                                as *const u8 as *const libc::c_char,
                            b"draft/glpssx01.c\0" as *const u8 as *const libc::c_char,
                            787 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
    };
}
pub unsafe extern "C" fn _glp_ssx_delete(mut ssx: *mut SSX) {
    let mut m: libc::c_int = (*ssx).m;
    let mut n: libc::c_int = (*ssx).n;
    let mut nnz: libc::c_int = *((*ssx).A_ptr).offset((n + 1 as libc::c_int) as isize)
        - 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    glp_free((*ssx).type_0 as *mut libc::c_void);
    k = 1 as libc::c_int;
    while k <= m + n {
        _glp_mpq_clear(*((*ssx).lb).offset(k as isize));
        k += 1;
        k;
    }
    glp_free((*ssx).lb as *mut libc::c_void);
    k = 1 as libc::c_int;
    while k <= m + n {
        _glp_mpq_clear(*((*ssx).ub).offset(k as isize));
        k += 1;
        k;
    }
    glp_free((*ssx).ub as *mut libc::c_void);
    k = 0 as libc::c_int;
    while k <= m + n {
        _glp_mpq_clear(*((*ssx).coef).offset(k as isize));
        k += 1;
        k;
    }
    glp_free((*ssx).coef as *mut libc::c_void);
    glp_free((*ssx).A_ptr as *mut libc::c_void);
    glp_free((*ssx).A_ind as *mut libc::c_void);
    k = 1 as libc::c_int;
    while k <= nnz {
        _glp_mpq_clear(*((*ssx).A_val).offset(k as isize));
        k += 1;
        k;
    }
    glp_free((*ssx).A_val as *mut libc::c_void);
    glp_free((*ssx).stat as *mut libc::c_void);
    glp_free((*ssx).Q_row as *mut libc::c_void);
    glp_free((*ssx).Q_col as *mut libc::c_void);
    _glp_bfx_delete_binv((*ssx).binv);
    i = 0 as libc::c_int;
    while i <= m {
        _glp_mpq_clear(*((*ssx).bbar).offset(i as isize));
        i += 1;
        i;
    }
    glp_free((*ssx).bbar as *mut libc::c_void);
    i = 1 as libc::c_int;
    while i <= m {
        _glp_mpq_clear(*((*ssx).pi).offset(i as isize));
        i += 1;
        i;
    }
    glp_free((*ssx).pi as *mut libc::c_void);
    j = 1 as libc::c_int;
    while j <= n {
        _glp_mpq_clear(*((*ssx).cbar).offset(j as isize));
        j += 1;
        j;
    }
    glp_free((*ssx).cbar as *mut libc::c_void);
    i = 1 as libc::c_int;
    while i <= m {
        _glp_mpq_clear(*((*ssx).rho).offset(i as isize));
        i += 1;
        i;
    }
    glp_free((*ssx).rho as *mut libc::c_void);
    j = 1 as libc::c_int;
    while j <= n {
        _glp_mpq_clear(*((*ssx).ap).offset(j as isize));
        j += 1;
        j;
    }
    glp_free((*ssx).ap as *mut libc::c_void);
    i = 1 as libc::c_int;
    while i <= m {
        _glp_mpq_clear(*((*ssx).aq).offset(i as isize));
        i += 1;
        i;
    }
    glp_free((*ssx).aq as *mut libc::c_void);
    _glp_mpq_clear((*ssx).delta);
    glp_free(ssx as *mut libc::c_void);
}
