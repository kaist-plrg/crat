use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
    fn glp_free(ptr: *mut libc::c_void);
    fn _glp_sva_delete_area(sva: *mut SVA);
    fn _glp_btf_store_a_cols(
        btf: *mut BTF,
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
    fn _glp_btf_make_blocks(btf: *mut BTF) -> libc::c_int;
    fn _glp_btf_build_a_rows(btf: *mut BTF, len: *mut libc::c_int);
    fn _glp_sva_enlarge_cap(
        sva: *mut SVA,
        k: libc::c_int,
        new_cap: libc::c_int,
        skip: libc::c_int,
    );
    fn _glp_sva_more_space(sva: *mut SVA, m_size: libc::c_int);
    fn _glp_sva_alloc_vecs(sva: *mut SVA, nnn: libc::c_int) -> libc::c_int;
    fn _glp_sva_reserve_cap(sva: *mut SVA, k: libc::c_int, new_cap: libc::c_int);
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
pub struct BTF {
    pub n: libc::c_int,
    pub sva: *mut SVA,
    pub pp_ind: *mut libc::c_int,
    pub pp_inv: *mut libc::c_int,
    pub qq_ind: *mut libc::c_int,
    pub qq_inv: *mut libc::c_int,
    pub num: libc::c_int,
    pub beg: *mut libc::c_int,
    pub ar_ref: libc::c_int,
    pub ac_ref: libc::c_int,
    pub fr_ref: libc::c_int,
    pub fc_ref: libc::c_int,
    pub vr_ref: libc::c_int,
    pub vr_piv: *mut libc::c_double,
    pub vc_ref: libc::c_int,
    pub p1_ind: *mut libc::c_int,
    pub p1_inv: *mut libc::c_int,
    pub q1_ind: *mut libc::c_int,
    pub q1_inv: *mut libc::c_int,
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
pub struct BTFINT {
    pub n_max: libc::c_int,
    pub valid: libc::c_int,
    pub sva: *mut SVA,
    pub btf: *mut BTF,
    pub sgf: *mut SGF,
    pub sva_n_max: libc::c_int,
    pub sva_size: libc::c_int,
    pub delta_n0: libc::c_int,
    pub delta_n: libc::c_int,
    pub sgf_piv_tol: libc::c_double,
    pub sgf_piv_lim: libc::c_int,
    pub sgf_suhl: libc::c_int,
    pub sgf_eps_tol: libc::c_double,
}
pub unsafe extern "C" fn _glp_btfint_create() -> *mut BTFINT {
    let mut fi: *mut BTFINT = 0 as *mut BTFINT;
    fi = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<BTFINT>() as libc::c_ulong as libc::c_int,
    ) as *mut BTFINT;
    (*fi).n_max = 0 as libc::c_int;
    (*fi).valid = 0 as libc::c_int;
    (*fi).sva = 0 as *mut SVA;
    (*fi).btf = 0 as *mut BTF;
    (*fi).sgf = 0 as *mut SGF;
    (*fi).sva_size = 0 as libc::c_int;
    (*fi).sva_n_max = (*fi).sva_size;
    (*fi).delta_n = 0 as libc::c_int;
    (*fi).delta_n0 = (*fi).delta_n;
    (*fi).sgf_piv_tol = 0.10f64;
    (*fi).sgf_piv_lim = 4 as libc::c_int;
    (*fi).sgf_suhl = 1 as libc::c_int;
    (*fi).sgf_eps_tol = 2.2204460492503131e-16f64;
    return fi;
}
unsafe extern "C" fn factorize_triv(
    mut fi: *mut BTFINT,
    mut k: libc::c_int,
    mut col: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *mut libc::c_int,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    mut info: *mut libc::c_void,
) {
    let mut sva: *mut SVA = (*fi).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut btf: *mut BTF = (*fi).btf;
    let mut pp_inv: *mut libc::c_int = (*btf).pp_inv;
    let mut qq_ind: *mut libc::c_int = (*btf).qq_ind;
    let mut beg: *mut libc::c_int = (*btf).beg;
    let mut ac_ref: libc::c_int = (*btf).ac_ref;
    let mut ac_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((ac_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut ac_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((ac_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut sgf: *mut SGF = (*fi).sgf;
    let mut ind: *mut libc::c_int = (*sgf).vr_max as *mut libc::c_int;
    let mut val: *mut libc::c_double = (*sgf).work;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut beg_k: libc::c_int = 0;
    beg_k = *beg.offset(k as isize);
    i = *pp_inv.offset(beg_k as isize);
    j = *qq_ind.offset(beg_k as isize);
    len = col.unwrap()(info, j, ind, val);
    t = 1 as libc::c_int;
    while t <= len {
        if *ind.offset(t as isize) == i {
            break;
        }
        t += 1;
        t;
    }
    (t <= len
        || {
            glp_assert_(
                b"t <= len\0" as *const u8 as *const libc::c_char,
                b"bflib/btfint.c\0" as *const u8 as *const libc::c_char,
                74 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*val.offset(t as isize) != 0.0f64
        || {
            glp_assert_(
                b"val[t] != 0.0\0" as *const u8 as *const libc::c_char,
                b"bflib/btfint.c\0" as *const u8 as *const libc::c_char,
                78 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    *((*btf).vr_piv).offset(beg_k as isize) = *val.offset(t as isize);
    let ref mut fresh0 = *((*btf).p1_inv).offset(beg_k as isize);
    *fresh0 = 1 as libc::c_int;
    *((*btf).p1_ind).offset(beg_k as isize) = *fresh0;
    let ref mut fresh1 = *((*btf).q1_inv).offset(beg_k as isize);
    *fresh1 = 1 as libc::c_int;
    *((*btf).q1_ind).offset(beg_k as isize) = *fresh1;
    memmove(
        &mut *ind.offset(t as isize) as *mut libc::c_int as *mut libc::c_void,
        &mut *ind.offset((t + 1 as libc::c_int) as isize) as *mut libc::c_int
            as *const libc::c_void,
        ((len - t) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    memmove(
        &mut *val.offset(t as isize) as *mut libc::c_double as *mut libc::c_void,
        &mut *val.offset((t + 1 as libc::c_int) as isize) as *mut libc::c_double
            as *const libc::c_void,
        ((len - t) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    len -= 1;
    len;
    if len > 0 as libc::c_int {
        if (*sva).r_ptr - (*sva).m_ptr < len {
            _glp_sva_more_space(sva, len);
            sv_ind = (*sva).ind;
            sv_val = (*sva).val;
        }
        _glp_sva_reserve_cap(sva, ac_ref + (j - 1 as libc::c_int), len);
        ptr = *ac_ptr.offset(j as isize);
        memcpy(
            &mut *sv_ind.offset(ptr as isize) as *mut libc::c_int as *mut libc::c_void,
            &mut *ind.offset(1 as libc::c_int as isize) as *mut libc::c_int
                as *const libc::c_void,
            (len as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        memcpy(
            &mut *sv_val.offset(ptr as isize) as *mut libc::c_double
                as *mut libc::c_void,
            &mut *val.offset(1 as libc::c_int as isize) as *mut libc::c_double
                as *const libc::c_void,
            (len as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        *ac_len.offset(j as isize) = len;
    }
}
unsafe extern "C" fn factorize_block(
    mut fi: *mut BTFINT,
    mut k: libc::c_int,
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
    let mut sva: *mut SVA = (*fi).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut btf: *mut BTF = (*fi).btf;
    let mut pp_ind: *mut libc::c_int = (*btf).pp_ind;
    let mut qq_ind: *mut libc::c_int = (*btf).qq_ind;
    let mut beg: *mut libc::c_int = (*btf).beg;
    let mut ac_ref: libc::c_int = (*btf).ac_ref;
    let mut ac_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((ac_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut ac_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((ac_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut sgf: *mut SGF = (*fi).sgf;
    let mut ind: *mut libc::c_int = (*sgf).vr_max as *mut libc::c_int;
    let mut val: *mut libc::c_double = (*sgf).work;
    let mut luf: LUF = LUF {
        n: 0,
        sva: 0 as *mut SVA,
        fr_ref: 0,
        fc_ref: 0,
        vr_ref: 0,
        vr_piv: 0 as *mut libc::c_double,
        vc_ref: 0,
        pp_ind: 0 as *mut libc::c_int,
        pp_inv: 0 as *mut libc::c_int,
        qq_ind: 0 as *mut libc::c_int,
        qq_inv: 0 as *mut libc::c_int,
    };
    let mut vc_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut vc_len: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut vc_cap: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut beg_k: libc::c_int = 0;
    (*sgf).luf = &mut luf;
    beg_k = *beg.offset(k as isize);
    luf.n = *beg.offset((k + 1 as libc::c_int) as isize) - beg_k;
    luf.sva = sva;
    luf.fr_ref = (*btf).fr_ref + (beg_k - 1 as libc::c_int);
    luf.fc_ref = (*btf).fc_ref + (beg_k - 1 as libc::c_int);
    luf.vr_ref = (*btf).vr_ref + (beg_k - 1 as libc::c_int);
    luf.vr_piv = ((*btf).vr_piv).offset((beg_k - 1 as libc::c_int) as isize);
    luf.vc_ref = (*btf).vc_ref + (beg_k - 1 as libc::c_int);
    luf.pp_ind = ((*btf).p1_ind).offset((beg_k - 1 as libc::c_int) as isize);
    luf.pp_inv = ((*btf).p1_inv).offset((beg_k - 1 as libc::c_int) as isize);
    luf.qq_ind = ((*btf).q1_ind).offset((beg_k - 1 as libc::c_int) as isize);
    luf.qq_inv = ((*btf).q1_inv).offset((beg_k - 1 as libc::c_int) as isize);
    vc_ptr = &mut *((*sva).ptr).offset((luf.vc_ref - 1 as libc::c_int) as isize)
        as *mut libc::c_int;
    vc_len = &mut *((*sva).len).offset((luf.vc_ref - 1 as libc::c_int) as isize)
        as *mut libc::c_int;
    vc_cap = &mut *((*sva).cap).offset((luf.vc_ref - 1 as libc::c_int) as isize)
        as *mut libc::c_int;
    jj = 1 as libc::c_int;
    while jj <= luf.n {
        j = *qq_ind.offset((jj + (beg_k - 1 as libc::c_int)) as isize);
        len = col.unwrap()(info, j, ind, val);
        cnt = 0 as libc::c_int;
        t = 1 as libc::c_int;
        while t <= len {
            i = *ind.offset(t as isize);
            ii = *pp_ind.offset(i as isize);
            if ii >= beg_k {
                let mut temp: libc::c_double = 0.;
                cnt += 1;
                cnt;
                *ind.offset(t as isize) = *ind.offset(cnt as isize);
                *ind.offset(cnt as isize) = ii - (beg_k - 1 as libc::c_int);
                temp = *val.offset(t as isize);
                *val.offset(t as isize) = *val.offset(cnt as isize);
                *val.offset(cnt as isize) = temp;
            }
            t += 1;
            t;
        }
        if *vc_cap.offset(jj as isize) < cnt {
            if (*sva).r_ptr - (*sva).m_ptr < cnt {
                _glp_sva_more_space(sva, cnt);
                sv_ind = (*sva).ind;
                sv_val = (*sva).val;
            }
            _glp_sva_enlarge_cap(
                sva,
                luf.vc_ref + (jj - 1 as libc::c_int),
                cnt,
                0 as libc::c_int,
            );
        }
        ptr = *vc_ptr.offset(jj as isize);
        memcpy(
            &mut *sv_ind.offset(ptr as isize) as *mut libc::c_int as *mut libc::c_void,
            &mut *ind.offset(1 as libc::c_int as isize) as *mut libc::c_int
                as *const libc::c_void,
            (cnt as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        memcpy(
            &mut *sv_val.offset(ptr as isize) as *mut libc::c_double
                as *mut libc::c_void,
            &mut *val.offset(1 as libc::c_int as isize) as *mut libc::c_double
                as *const libc::c_void,
            (cnt as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        *vc_len.offset(jj as isize) = cnt;
        len -= cnt;
        if len > 0 as libc::c_int {
            if (*sva).r_ptr - (*sva).m_ptr < len {
                _glp_sva_more_space(sva, len);
                sv_ind = (*sva).ind;
                sv_val = (*sva).val;
            }
            _glp_sva_reserve_cap(sva, ac_ref - 1 as libc::c_int + j, len);
            ptr = *ac_ptr.offset(j as isize);
            memcpy(
                &mut *sv_ind.offset(ptr as isize) as *mut libc::c_int
                    as *mut libc::c_void,
                &mut *ind.offset((cnt + 1 as libc::c_int) as isize) as *mut libc::c_int
                    as *const libc::c_void,
                (len as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
            memcpy(
                &mut *sv_val.offset(ptr as isize) as *mut libc::c_double
                    as *mut libc::c_void,
                &mut *val.offset((cnt + 1 as libc::c_int) as isize)
                    as *mut libc::c_double as *const libc::c_void,
                (len as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            );
            *ac_len.offset(j as isize) = len;
        }
        jj += 1;
        jj;
    }
    k = _glp_sgf_factorize(sgf, 0 as libc::c_int);
    ((*sva).m_ptr == 1 as libc::c_int
        || {
            glp_assert_(
                b"sva->m_ptr == 1\0" as *const u8 as *const libc::c_char,
                b"bflib/btfint.c\0" as *const u8 as *const libc::c_char,
                204 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return k;
}
pub unsafe extern "C" fn _glp_btfint_factorize(
    mut fi: *mut BTFINT,
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
    let mut btf: *mut BTF = 0 as *mut BTF;
    let mut sgf: *mut SGF = 0 as *mut SGF;
    let mut k: libc::c_int = 0;
    let mut rank: libc::c_int = 0;
    (n > 0 as libc::c_int
        || {
            glp_assert_(
                b"n > 0\0" as *const u8 as *const libc::c_char,
                b"bflib/btfint.c\0" as *const u8 as *const libc::c_char,
                215 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*fi).valid = 0 as libc::c_int;
    sva = (*fi).sva;
    if sva.is_null() {
        let mut sva_n_max: libc::c_int = (*fi).sva_n_max;
        let mut sva_size: libc::c_int = (*fi).sva_size;
        if sva_n_max == 0 as libc::c_int {
            sva_n_max = 6 as libc::c_int * n;
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
                    b"bflib/btfint.c\0" as *const u8 as *const libc::c_char,
                    235 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        btf = (*fi).btf;
        if btf.is_null() {
            (*fi)
                .btf = glp_alloc(
                1 as libc::c_int,
                ::std::mem::size_of::<BTF>() as libc::c_ulong as libc::c_int,
            ) as *mut BTF;
            btf = (*fi).btf;
            memset(
                btf as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<BTF>() as libc::c_ulong,
            );
            (*btf).sva = sva;
        } else {
            glp_free((*btf).pp_ind as *mut libc::c_void);
            glp_free((*btf).pp_inv as *mut libc::c_void);
            glp_free((*btf).qq_ind as *mut libc::c_void);
            glp_free((*btf).qq_inv as *mut libc::c_void);
            glp_free((*btf).beg as *mut libc::c_void);
            glp_free((*btf).vr_piv as *mut libc::c_void);
            glp_free((*btf).p1_ind as *mut libc::c_void);
            glp_free((*btf).p1_inv as *mut libc::c_void);
            glp_free((*btf).q1_ind as *mut libc::c_void);
            glp_free((*btf).q1_inv as *mut libc::c_void);
        }
        (*btf)
            .pp_ind = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*btf)
            .pp_inv = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*btf)
            .qq_ind = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*btf)
            .qq_inv = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*btf)
            .beg = glp_alloc(
            1 as libc::c_int + n_max + 1 as libc::c_int,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*btf)
            .vr_piv = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        (*btf)
            .p1_ind = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*btf)
            .p1_inv = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*btf)
            .q1_ind = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        (*btf)
            .q1_inv = glp_alloc(
            1 as libc::c_int + n_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        sgf = (*fi).sgf;
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
    btf = (*fi).btf;
    (*btf).n = n;
    sgf = (*fi).sgf;
    (*sva).n = 0 as libc::c_int;
    (*sva).m_ptr = 1 as libc::c_int;
    (*sva).r_ptr = (*sva).size + 1 as libc::c_int;
    (*sva).tail = 0 as libc::c_int;
    (*sva).head = (*sva).tail;
    (*btf).ac_ref = _glp_sva_alloc_vecs((*btf).sva, (*btf).n);
    _glp_btf_store_a_cols(btf, col, info, (*btf).pp_ind, (*btf).vr_piv);
    rank = _glp_btf_make_blocks(btf);
    if rank != n {
        return 1 as libc::c_int;
    }
    (*sva).n = 0 as libc::c_int;
    (*sva).m_ptr = 1 as libc::c_int;
    (*sva).r_ptr = (*sva).size + 1 as libc::c_int;
    (*sva).tail = 0 as libc::c_int;
    (*sva).head = (*sva).tail;
    (*btf).ar_ref = _glp_sva_alloc_vecs((*btf).sva, (*btf).n);
    (*btf).ac_ref = _glp_sva_alloc_vecs((*btf).sva, (*btf).n);
    (*btf).fr_ref = _glp_sva_alloc_vecs((*btf).sva, (*btf).n);
    (*btf).fc_ref = _glp_sva_alloc_vecs((*btf).sva, (*btf).n);
    (*btf).vr_ref = _glp_sva_alloc_vecs((*btf).sva, (*btf).n);
    (*btf).vc_ref = _glp_sva_alloc_vecs((*btf).sva, (*btf).n);
    (*sgf).updat = 0 as libc::c_int;
    (*sgf).piv_tol = (*fi).sgf_piv_tol;
    (*sgf).piv_lim = (*fi).sgf_piv_lim;
    (*sgf).suhl = (*fi).sgf_suhl;
    (*sgf).eps_tol = (*fi).sgf_eps_tol;
    k = 1 as libc::c_int;
    while k <= (*btf).num {
        if *((*btf).beg).offset((k + 1 as libc::c_int) as isize)
            - *((*btf).beg).offset(k as isize) == 1 as libc::c_int
        {
            factorize_triv(fi, k, col, info);
        } else if factorize_block(fi, k, col, info) != 0 as libc::c_int {
            return 2 as libc::c_int
        }
        k += 1;
        k;
    }
    _glp_btf_build_a_rows((*fi).btf, (*(*fi).sgf).rs_head);
    (*fi).valid = 1 as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_btfint_delete(mut fi: *mut BTFINT) {
    let mut sva: *mut SVA = (*fi).sva;
    let mut btf: *mut BTF = (*fi).btf;
    let mut sgf: *mut SGF = (*fi).sgf;
    if !sva.is_null() {
        _glp_sva_delete_area(sva);
    }
    if !btf.is_null() {
        glp_free((*btf).pp_ind as *mut libc::c_void);
        glp_free((*btf).pp_inv as *mut libc::c_void);
        glp_free((*btf).qq_ind as *mut libc::c_void);
        glp_free((*btf).qq_inv as *mut libc::c_void);
        glp_free((*btf).beg as *mut libc::c_void);
        glp_free((*btf).vr_piv as *mut libc::c_void);
        glp_free((*btf).p1_ind as *mut libc::c_void);
        glp_free((*btf).p1_inv as *mut libc::c_void);
        glp_free((*btf).q1_ind as *mut libc::c_void);
        glp_free((*btf).q1_inv as *mut libc::c_void);
        glp_free(btf as *mut libc::c_void);
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
