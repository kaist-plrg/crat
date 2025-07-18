use ::libc;
extern "C" {
    fn glp_free(ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn _glp_luf_store_v_cols(
        luf: *mut LUF,
        col: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                libc::c_int,
                *mut libc::c_int,
                *mut libc::c_double,
            ) -> libc::c_int,
        >,
        info: *mut libc::c_void,
        ind: *mut libc::c_int,
        val: *mut libc::c_double,
    ) -> libc::c_int;
    fn _glp_sva_delete_area(sva: *mut SVA);
    fn _glp_sva_alloc_vecs(sva: *mut SVA, nnn: libc::c_int) -> libc::c_int;
    fn _glp_sva_create_area(n_max: libc::c_int, size: libc::c_int) -> *mut SVA;
    fn _glp_sgf_factorize(sgf: *mut SGF, singl: libc::c_int) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SVA {
    pub n_max: libc::c_int,
    pub n: libc::c_int,
    pub ptr: *mut libc::c_int,
    pub len: *mut libc::c_int,
    pub cap: *mut libc::c_int,
    pub size: libc::c_int,
    pub m_ptr: libc::c_int,
    pub r_ptr: libc::c_int,
    pub head: libc::c_int,
    pub tail: libc::c_int,
    pub prev: *mut libc::c_int,
    pub next: *mut libc::c_int,
    pub ind: *mut libc::c_int,
    pub val: *mut libc::c_double,
    pub talky: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LUF {
    pub n: libc::c_int,
    pub sva: *mut SVA,
    pub fr_ref: libc::c_int,
    pub fc_ref: libc::c_int,
    pub vr_ref: libc::c_int,
    pub vr_piv: *mut libc::c_double,
    pub vc_ref: libc::c_int,
    pub pp_ind: *mut libc::c_int,
    pub pp_inv: *mut libc::c_int,
    pub qq_ind: *mut libc::c_int,
    pub qq_inv: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SGF {
    pub luf: *mut LUF,
    pub rs_head: *mut libc::c_int,
    pub rs_prev: *mut libc::c_int,
    pub rs_next: *mut libc::c_int,
    pub cs_head: *mut libc::c_int,
    pub cs_prev: *mut libc::c_int,
    pub cs_next: *mut libc::c_int,
    pub vr_max: *mut libc::c_double,
    pub flag: *mut libc::c_char,
    pub work: *mut libc::c_double,
    pub updat: libc::c_int,
    pub piv_tol: libc::c_double,
    pub piv_lim: libc::c_int,
    pub suhl: libc::c_int,
    pub eps_tol: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LUFINT {
    pub n_max: libc::c_int,
    pub valid: libc::c_int,
    pub sva: *mut SVA,
    pub luf: *mut LUF,
    pub sgf: *mut SGF,
    pub sva_n_max: libc::c_int,
    pub sva_size: libc::c_int,
    pub delta_n0: libc::c_int,
    pub delta_n: libc::c_int,
    pub sgf_updat: libc::c_int,
    pub sgf_piv_tol: libc::c_double,
    pub sgf_piv_lim: libc::c_int,
    pub sgf_suhl: libc::c_int,
    pub sgf_eps_tol: libc::c_double,
}
pub unsafe extern "C" fn _glp_lufint_create() -> *mut LUFINT {
    let mut fi: *mut LUFINT = 0 as *mut LUFINT;
    fi = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<LUFINT>() as libc::c_ulong as libc::c_int,
    ) as *mut LUFINT;
    (*fi).n_max = 0 as libc::c_int;
    (*fi).valid = 0 as libc::c_int;
    (*fi).sva = 0 as *mut SVA;
    (*fi).luf = 0 as *mut LUF;
    (*fi).sgf = 0 as *mut SGF;
    (*fi).sva_size = 0 as libc::c_int;
    (*fi).sva_n_max = (*fi).sva_size;
    (*fi).delta_n = 0 as libc::c_int;
    (*fi).delta_n0 = (*fi).delta_n;
    (*fi).sgf_updat = 0 as libc::c_int;
    (*fi).sgf_piv_tol = 0.10f64;
    (*fi).sgf_piv_lim = 4 as libc::c_int;
    (*fi).sgf_suhl = 1 as libc::c_int;
    (*fi).sgf_eps_tol = 2.2204460492503131e-16f64;
    return fi;
}
pub unsafe extern "C" fn _glp_lufint_factorize(
    mut fi: *mut LUFINT,
    mut n: libc::c_int,
    mut col: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *mut libc::c_int,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    mut info: *mut libc::c_void,
) -> libc::c_int {
    let mut sva: *mut SVA = 0 as *mut SVA;
    let mut luf: *mut LUF = 0 as *mut LUF;
    let mut sgf: *mut SGF = 0 as *mut SGF;
    let mut k: libc::c_int = 0;
    (n > 0 as libc::c_int
        || {
            glp_assert_(
                b"n > 0\0" as *const u8 as *const libc::c_char,
                b"bflib/lufint.c\0" as *const u8 as *const libc::c_char,
                51 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*fi).valid = 0 as libc::c_int;
    sva = (*fi).sva;
    if sva.is_null() {
        let mut sva_n_max: libc::c_int = (*fi).sva_n_max;
        let mut sva_size: libc::c_int = (*fi).sva_size;
        if sva_n_max == 0 as libc::c_int {
            sva_n_max = 4 as libc::c_int * n;
        }
        if sva_size == 0 as libc::c_int {
            sva_size = 10 as libc::c_int * n;
        }
        (*fi).sva = _glp_sva_create_area(sva_n_max, sva_size);
        sva = (*fi).sva;
    }
    if (*fi).n_max < n {
        let mut n_max: libc::c_int = (*fi).n_max;
        if n_max == 0 as libc::c_int {
            (*fi).n_max = n + (*fi).delta_n0;
            n_max = (*fi).n_max;
        } else {
            (*fi).n_max = n + (*fi).delta_n;
            n_max = (*fi).n_max;
        }
        (n_max >= n
            || {
                glp_assert_(
                    b"n_max >= n\0" as *const u8 as *const libc::c_char,
                    b"bflib/lufint.c\0" as *const u8 as *const libc::c_char,
                    71 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        luf = (*fi).luf;
        if luf.is_null() {
            (*fi)
                .luf = glp_alloc(
                1 as libc::c_int,
                ::std::mem::size_of::<LUF>() as libc::c_ulong as libc::c_int,
            ) as *mut LUF;
            luf = (*fi).luf;
            memset(
                luf as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<LUF>() as libc::c_ulong,
            );
            (*luf).sva = sva;
        } else {
            glp_free((*luf).vr_piv as *mut libc::c_void);
            glp_free((*luf).pp_ind as *mut libc::c_void);
            glp_free((*luf).pp_inv as *mut libc::c_void);
            glp_free((*luf).qq_ind as *mut libc::c_void);
            glp_free((*luf).qq_inv as *mut libc::c_void);
        }
        (*luf)
            .vr_piv = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        (*luf)
            .pp_ind = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*luf)
            .pp_inv = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*luf)
            .qq_ind = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*luf)
            .qq_inv = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        sgf = (*fi).sgf;
        if sgf.is_null() {
            (*fi)
                .sgf = glp_alloc(
                1 as libc::c_int,
                ::std::mem::size_of::<SGF>() as libc::c_ulong as libc::c_int,
            ) as *mut SGF;
            sgf = (*fi).sgf;
            memset(
                sgf as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<SGF>() as libc::c_ulong,
            );
            (*sgf).luf = luf;
        } else {
            glp_free((*sgf).rs_head as *mut libc::c_void);
            glp_free((*sgf).rs_prev as *mut libc::c_void);
            glp_free((*sgf).rs_next as *mut libc::c_void);
            glp_free((*sgf).cs_head as *mut libc::c_void);
            glp_free((*sgf).cs_prev as *mut libc::c_void);
            glp_free((*sgf).cs_next as *mut libc::c_void);
            glp_free((*sgf).vr_max as *mut libc::c_void);
            glp_free((*sgf).flag as *mut libc::c_void);
            glp_free((*sgf).work as *mut libc::c_void);
        }
        (*sgf)
            .rs_head = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*sgf)
            .rs_prev = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*sgf)
            .rs_next = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*sgf)
            .cs_head = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*sgf)
            .cs_prev = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*sgf)
            .cs_next = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*sgf)
            .vr_max = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        (*sgf)
            .flag = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_char;
        (*sgf)
            .work = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
    }
    luf = (*fi).luf;
    sgf = (*fi).sgf;
    (*sva).n = 0 as libc::c_int;
    (*sva).m_ptr = 1 as libc::c_int;
    (*sva).r_ptr = (*sva).size + 1 as libc::c_int;
    (*sva).tail = 0 as libc::c_int;
    (*sva).head = (*sva).tail;
    (*luf).n = n;
    (*luf).fr_ref = _glp_sva_alloc_vecs(sva, n);
    (*luf).fc_ref = _glp_sva_alloc_vecs(sva, n);
    (*luf).vr_ref = _glp_sva_alloc_vecs(sva, n);
    (*luf).vc_ref = _glp_sva_alloc_vecs(sva, n);
    _glp_luf_store_v_cols(luf, col, info, (*sgf).rs_prev, (*sgf).work);
    (*sgf).updat = (*fi).sgf_updat;
    (*sgf).piv_tol = (*fi).sgf_piv_tol;
    (*sgf).piv_lim = (*fi).sgf_piv_lim;
    (*sgf).suhl = (*fi).sgf_suhl;
    (*sgf).eps_tol = (*fi).sgf_eps_tol;
    k = _glp_sgf_factorize(sgf, 1 as libc::c_int);
    if k == 0 as libc::c_int {
        (*fi).valid = 1 as libc::c_int;
    }
    return k;
}
pub unsafe extern "C" fn _glp_lufint_delete(mut fi: *mut LUFINT) {
    let mut sva: *mut SVA = (*fi).sva;
    let mut luf: *mut LUF = (*fi).luf;
    let mut sgf: *mut SGF = (*fi).sgf;
    if !sva.is_null() {
        _glp_sva_delete_area(sva);
    }
    if !luf.is_null() {
        glp_free((*luf).vr_piv as *mut libc::c_void);
        glp_free((*luf).pp_ind as *mut libc::c_void);
        glp_free((*luf).pp_inv as *mut libc::c_void);
        glp_free((*luf).qq_ind as *mut libc::c_void);
        glp_free((*luf).qq_inv as *mut libc::c_void);
        glp_free(luf as *mut libc::c_void);
    }
    if !sgf.is_null() {
        glp_free((*sgf).rs_head as *mut libc::c_void);
        glp_free((*sgf).rs_prev as *mut libc::c_void);
        glp_free((*sgf).rs_next as *mut libc::c_void);
        glp_free((*sgf).cs_head as *mut libc::c_void);
        glp_free((*sgf).cs_prev as *mut libc::c_void);
        glp_free((*sgf).cs_next as *mut libc::c_void);
        glp_free((*sgf).vr_max as *mut libc::c_void);
        glp_free((*sgf).flag as *mut libc::c_void);
        glp_free((*sgf).work as *mut libc::c_void);
        glp_free(sgf as *mut libc::c_void);
    }
    glp_free(fi as *mut libc::c_void);
}
