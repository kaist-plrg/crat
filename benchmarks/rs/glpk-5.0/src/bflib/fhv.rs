use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn _glp_sva_more_space(sva: *mut SVA, m_size: libc::c_int);
    fn _glp_sva_enlarge_cap(
        sva: *mut SVA,
        k: libc::c_int,
        new_cap: libc::c_int,
        skip: libc::c_int,
    );
    fn _glp_sva_reserve_cap(sva: *mut SVA, k: libc::c_int, new_cap: libc::c_int);
    fn _glp_luf_f_solve(luf: *mut LUF, x: *mut libc::c_double);
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
pub struct FHV {
    pub luf: *mut LUF,
    pub nfs_max: libc::c_int,
    pub nfs: libc::c_int,
    pub hh_ind: *mut libc::c_int,
    pub hh_ref: libc::c_int,
    pub p0_ind: *mut libc::c_int,
    pub p0_inv: *mut libc::c_int,
}
pub unsafe extern "C" fn _glp_fhv_ft_update(
    mut fhv: *mut FHV,
    mut q: libc::c_int,
    mut aq_len: libc::c_int,
    mut aq_ind: *const libc::c_int,
    mut aq_val: *const libc::c_double,
    mut ind: *mut libc::c_int,
    mut val: *mut libc::c_double,
    mut work: *mut libc::c_double,
) -> libc::c_int {
    let mut luf: *mut LUF = (*fhv).luf;
    let mut n: libc::c_int = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut vr_ref: libc::c_int = (*luf).vr_ref;
    let mut vr_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((vr_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut vr_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((vr_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut vr_cap: *mut libc::c_int = &mut *((*sva).cap)
        .offset((vr_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut vr_piv: *mut libc::c_double = (*luf).vr_piv;
    let mut vc_ref: libc::c_int = (*luf).vc_ref;
    let mut vc_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((vc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut vc_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((vc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut vc_cap: *mut libc::c_int = &mut *((*sva).cap)
        .offset((vc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut pp_ind: *mut libc::c_int = (*luf).pp_ind;
    let mut pp_inv: *mut libc::c_int = (*luf).pp_inv;
    let mut qq_ind: *mut libc::c_int = (*luf).qq_ind;
    let mut qq_inv: *mut libc::c_int = (*luf).qq_inv;
    let mut hh_ind: *mut libc::c_int = (*fhv).hh_ind;
    let mut hh_ref: libc::c_int = (*fhv).hh_ref;
    let mut hh_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((hh_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut hh_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((hh_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let eps_tol: libc::c_double = 2.2204460492503131e-16f64;
    let vpq_tol: libc::c_double = 1e-5f64;
    let err_tol: libc::c_double = 1e-10f64;
    let mut end: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut i_end: libc::c_int = 0;
    let mut i_ptr: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut j_end: libc::c_int = 0;
    let mut j_ptr: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut nnz: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut p_end: libc::c_int = 0;
    let mut p_ptr: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut q_end: libc::c_int = 0;
    let mut q_ptr: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut f: libc::c_double = 0.;
    let mut vpq: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    (1 as libc::c_int <= q && q <= n
        || {
            glp_assert_(
                b"1 <= q && q <= n\0" as *const u8 as *const libc::c_char,
                b"bflib/fhv.c\0" as *const u8 as *const libc::c_char,
                179 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    i = 1 as libc::c_int;
    while i <= n {
        *val.offset(i as isize) = 0.0f64;
        i += 1;
        i;
    }
    (0 as libc::c_int <= aq_len && aq_len <= n
        || {
            glp_assert_(
                b"0 <= aq_len && aq_len <= n\0" as *const u8 as *const libc::c_char,
                b"bflib/fhv.c\0" as *const u8 as *const libc::c_char,
                183 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = 1 as libc::c_int;
    while k <= aq_len {
        i = *aq_ind.offset(k as isize);
        (1 as libc::c_int <= i && i <= n
            || {
                glp_assert_(
                    b"1 <= i && i <= n\0" as *const u8 as *const libc::c_char,
                    b"bflib/fhv.c\0" as *const u8 as *const libc::c_char,
                    186 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*val.offset(i as isize) == 0.0f64
            || {
                glp_assert_(
                    b"val[i] == 0.0\0" as *const u8 as *const libc::c_char,
                    b"bflib/fhv.c\0" as *const u8 as *const libc::c_char,
                    187 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*aq_val.offset(k as isize) != 0.0f64
            || {
                glp_assert_(
                    b"aq_val[k] != 0.0\0" as *const u8 as *const libc::c_char,
                    b"bflib/fhv.c\0" as *const u8 as *const libc::c_char,
                    188 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        *val.offset(i as isize) = *aq_val.offset(k as isize);
        k += 1;
        k;
    }
    (*luf).pp_ind = (*fhv).p0_ind;
    (*luf).pp_inv = (*fhv).p0_inv;
    _glp_luf_f_solve(luf, val);
    (*luf).pp_ind = pp_ind;
    (*luf).pp_inv = pp_inv;
    _glp_fhv_h_solve(fhv, val);
    s = *qq_inv.offset(q as isize);
    p = *pp_inv.offset(s as isize);
    vpq = 0.0f64;
    len = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= n {
        temp = *val.offset(i as isize);
        if !(-eps_tol < temp && temp < eps_tol) {
            if i == p {
                vpq = temp;
            } else {
                len += 1;
                *ind.offset(len as isize) = i;
                *val.offset(len as isize) = temp;
            }
        }
        i += 1;
        i;
    }
    q_ptr = *vc_ptr.offset(q as isize);
    q_end = q_ptr + *vc_len.offset(q as isize);
    while q_ptr < q_end {
        i = *sv_ind.offset(q_ptr as isize);
        i_ptr = *vr_ptr.offset(i as isize);
        i_end = i_ptr + *vr_len.offset(i as isize);
        while *sv_ind.offset(i_ptr as isize) != q {
            i_ptr += 1;
            i_ptr;
        }
        (i_ptr < i_end
            || {
                glp_assert_(
                    b"i_ptr < i_end\0" as *const u8 as *const libc::c_char,
                    b"bflib/fhv.c\0" as *const u8 as *const libc::c_char,
                    231 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        *sv_ind
            .offset(
                i_ptr as isize,
            ) = *sv_ind.offset((i_end - 1 as libc::c_int) as isize);
        *sv_val
            .offset(
                i_ptr as isize,
            ) = *sv_val.offset((i_end - 1 as libc::c_int) as isize);
        let ref mut fresh0 = *vr_len.offset(i as isize);
        *fresh0 -= 1;
        *fresh0;
        q_ptr += 1;
        q_ptr;
    }
    *vc_len.offset(q as isize) = 0 as libc::c_int;
    if len > 0 as libc::c_int {
        if *vc_cap.offset(q as isize) < len {
            if (*sva).r_ptr - (*sva).m_ptr < len {
                _glp_sva_more_space(sva, len);
                sv_ind = (*sva).ind;
                sv_val = (*sva).val;
            }
            _glp_sva_enlarge_cap(
                sva,
                vc_ref - 1 as libc::c_int + q,
                len,
                0 as libc::c_int,
            );
        }
        ptr = *vc_ptr.offset(q as isize);
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
        *vc_len.offset(q as isize) = len;
    }
    t = if vpq == 0.0f64 { 0 as libc::c_int } else { s };
    k = 1 as libc::c_int;
    while k <= len {
        i = *ind.offset(k as isize);
        if *vr_cap.offset(i as isize) == *vr_len.offset(i as isize) {
            let mut need: libc::c_int = *vr_len.offset(i as isize) + 5 as libc::c_int;
            if (*sva).r_ptr - (*sva).m_ptr < need {
                _glp_sva_more_space(sva, need);
                sv_ind = (*sva).ind;
                sv_val = (*sva).val;
            }
            _glp_sva_enlarge_cap(
                sva,
                vr_ref - 1 as libc::c_int + i,
                need,
                0 as libc::c_int,
            );
        }
        let ref mut fresh1 = *vr_len.offset(i as isize);
        let fresh2 = *fresh1;
        *fresh1 = *fresh1 + 1;
        ptr = *vr_ptr.offset(i as isize) + fresh2;
        *sv_ind.offset(ptr as isize) = q;
        *sv_val.offset(ptr as isize) = *val.offset(k as isize);
        if t < *pp_ind.offset(i as isize) {
            t = *pp_ind.offset(i as isize);
        }
        k += 1;
        k;
    }
    if s >= t {
        *vr_piv.offset(p as isize) = vpq;
        if s > t {
            (vpq == 0.0f64
                || {
                    glp_assert_(
                        b"vpq == 0.0\0" as *const u8 as *const libc::c_char,
                        b"bflib/fhv.c\0" as *const u8 as *const libc::c_char,
                        293 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            return 1 as libc::c_int;
        } else if -vpq_tol < vpq && vpq < vpq_tol {
            return 2 as libc::c_int
        } else {
            return 0 as libc::c_int
        }
    }
    (p == *pp_inv.offset(s as isize) && q == *qq_ind.offset(s as isize)
        || {
            glp_assert_(
                b"p == pp_inv[s] && q == qq_ind[s]\0" as *const u8
                    as *const libc::c_char,
                b"bflib/fhv.c\0" as *const u8 as *const libc::c_char,
                313 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = s;
    while k < t {
        let ref mut fresh3 = *pp_inv.offset(k as isize);
        *fresh3 = *pp_inv.offset((k + 1 as libc::c_int) as isize);
        *pp_ind.offset(*fresh3 as isize) = k;
        let ref mut fresh4 = *qq_ind.offset(k as isize);
        *fresh4 = *qq_ind.offset((k + 1 as libc::c_int) as isize);
        *qq_inv.offset(*fresh4 as isize) = k;
        k += 1;
        k;
    }
    let ref mut fresh5 = *qq_ind.offset(t as isize);
    *fresh5 = q;
    let ref mut fresh6 = *qq_inv.offset(*fresh5 as isize);
    *fresh6 = t;
    let ref mut fresh7 = *pp_inv.offset(t as isize);
    *fresh7 = p;
    *pp_ind.offset(*fresh7 as isize) = *fresh6;
    p_ptr = *vr_ptr.offset(p as isize);
    p_end = p_ptr + *vr_len.offset(p as isize);
    while p_ptr < p_end {
        if *qq_inv.offset(*sv_ind.offset(p_ptr as isize) as isize) < t {
            break;
        }
        p_ptr += 1;
        p_ptr;
    }
    if p_ptr == p_end {
        *vr_piv.offset(p as isize) = vpq;
        if -vpq_tol < vpq && vpq < vpq_tol {
            return 2 as libc::c_int
        } else {
            return 0 as libc::c_int
        }
    }
    j = 1 as libc::c_int;
    while j <= n {
        *work.offset(j as isize) = 0.0f64;
        j += 1;
        j;
    }
    *work.offset(q as isize) = vpq;
    p_ptr = *vr_ptr.offset(p as isize);
    p_end = p_ptr + *vr_len.offset(p as isize);
    while p_ptr < p_end {
        j = *sv_ind.offset(p_ptr as isize);
        *work.offset(j as isize) = *sv_val.offset(p_ptr as isize);
        j_ptr = *vc_ptr.offset(j as isize);
        j_end = j_ptr + *vc_len.offset(j as isize);
        while *sv_ind.offset(j_ptr as isize) != p {
            j_ptr += 1;
            j_ptr;
        }
        (j_ptr < j_end
            || {
                glp_assert_(
                    b"j_ptr < j_end\0" as *const u8 as *const libc::c_char,
                    b"bflib/fhv.c\0" as *const u8 as *const libc::c_char,
                    366 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        *sv_ind
            .offset(
                j_ptr as isize,
            ) = *sv_ind.offset((j_end - 1 as libc::c_int) as isize);
        *sv_val
            .offset(
                j_ptr as isize,
            ) = *sv_val.offset((j_end - 1 as libc::c_int) as isize);
        let ref mut fresh8 = *vc_len.offset(j as isize);
        *fresh8 -= 1;
        *fresh8;
        p_ptr += 1;
        p_ptr;
    }
    *vr_len.offset(p as isize) = 0 as libc::c_int;
    nnz = 0 as libc::c_int;
    k = s;
    while k < t {
        i = *pp_inv.offset(k as isize);
        j = *qq_ind.offset(k as isize);
        temp = *work.offset(j as isize);
        if !(-eps_tol < temp && temp < eps_tol) {
            nnz += 1;
            *ind.offset(nnz as isize) = i;
            f = *work.offset(j as isize) / *vr_piv.offset(i as isize);
            *val.offset(nnz as isize) = f;
            i_ptr = *vr_ptr.offset(i as isize);
            i_end = i_ptr + *vr_len.offset(i as isize);
            while i_ptr < i_end {
                *work.offset(*sv_ind.offset(i_ptr as isize) as isize)
                    -= f * *sv_val.offset(i_ptr as isize);
                i_ptr += 1;
                i_ptr;
            }
        }
        k += 1;
        k;
    }
    if -vpq_tol < *work.offset(q as isize) && *work.offset(q as isize) < vpq_tol {
        return 3 as libc::c_int;
    }
    if nnz > 0 as libc::c_int {
        if (*fhv).nfs == (*fhv).nfs_max {
            return 4 as libc::c_int;
        }
        (*fhv).nfs += 1;
        k = (*fhv).nfs;
        *hh_ind.offset(k as isize) = p;
        if (*sva).r_ptr - (*sva).m_ptr < nnz {
            _glp_sva_more_space(sva, nnz);
            sv_ind = (*sva).ind;
            sv_val = (*sva).val;
        }
        _glp_sva_reserve_cap(sva, (*fhv).hh_ref - 1 as libc::c_int + k, nnz);
        ptr = *hh_ptr.offset(k as isize);
        memcpy(
            &mut *sv_ind.offset(ptr as isize) as *mut libc::c_int as *mut libc::c_void,
            &mut *ind.offset(1 as libc::c_int as isize) as *mut libc::c_int
                as *const libc::c_void,
            (nnz as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        memcpy(
            &mut *sv_val.offset(ptr as isize) as *mut libc::c_double
                as *mut libc::c_void,
            &mut *val.offset(1 as libc::c_int as isize) as *mut libc::c_double
                as *const libc::c_void,
            (nnz as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        *hh_len.offset(k as isize) = nnz;
    }
    len = 0 as libc::c_int;
    k = t + 1 as libc::c_int;
    while k <= n {
        j = *qq_ind.offset(k as isize);
        temp = *work.offset(j as isize);
        if !(-eps_tol < temp && temp < eps_tol) {
            if *vc_cap.offset(j as isize) == *vc_len.offset(j as isize) {
                let mut need_0: libc::c_int = *vc_len.offset(j as isize)
                    + 5 as libc::c_int;
                if (*sva).r_ptr - (*sva).m_ptr < need_0 {
                    _glp_sva_more_space(sva, need_0);
                    sv_ind = (*sva).ind;
                    sv_val = (*sva).val;
                }
                _glp_sva_enlarge_cap(
                    sva,
                    vc_ref - 1 as libc::c_int + j,
                    need_0,
                    0 as libc::c_int,
                );
            }
            let ref mut fresh9 = *vc_len.offset(j as isize);
            let fresh10 = *fresh9;
            *fresh9 = *fresh9 + 1;
            ptr = *vc_ptr.offset(j as isize) + fresh10;
            *sv_ind.offset(ptr as isize) = p;
            *sv_val.offset(ptr as isize) = temp;
            len += 1;
            *ind.offset(len as isize) = j;
            *val.offset(len as isize) = temp;
        }
        k += 1;
        k;
    }
    if *vr_cap.offset(p as isize) < len {
        if (*sva).r_ptr - (*sva).m_ptr < len {
            _glp_sva_more_space(sva, len);
            sv_ind = (*sva).ind;
            sv_val = (*sva).val;
        }
        _glp_sva_enlarge_cap(sva, vr_ref - 1 as libc::c_int + p, len, 0 as libc::c_int);
    }
    ptr = *vr_ptr.offset(p as isize);
    memcpy(
        &mut *sv_ind.offset(ptr as isize) as *mut libc::c_int as *mut libc::c_void,
        &mut *ind.offset(1 as libc::c_int as isize) as *mut libc::c_int
            as *const libc::c_void,
        (len as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    memcpy(
        &mut *sv_val.offset(ptr as isize) as *mut libc::c_double as *mut libc::c_void,
        &mut *val.offset(1 as libc::c_int as isize) as *mut libc::c_double
            as *const libc::c_void,
        (len as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    *vr_len.offset(p as isize) = len;
    *vr_piv.offset(p as isize) = *work.offset(q as isize);
    if nnz > 0 as libc::c_int {
        j = 1 as libc::c_int;
        while j <= n {
            *work.offset(j as isize) = 0.0f64;
            j += 1;
            j;
        }
        k = (*fhv).nfs;
        ptr = *hh_ptr.offset(k as isize);
        end = ptr + *hh_len.offset(k as isize);
        while ptr < end {
            *work
                .offset(
                    *sv_ind.offset(ptr as isize) as isize,
                ) = *sv_val.offset(ptr as isize);
            ptr += 1;
            ptr;
        }
        temp = *vr_piv.offset(p as isize);
        ptr = *vc_ptr.offset(q as isize);
        end = ptr + *vc_len.offset(q as isize);
        while ptr < end {
            temp
                += *work.offset(*sv_ind.offset(ptr as isize) as isize)
                    * *sv_val.offset(ptr as isize);
            ptr += 1;
            ptr;
        }
        temp = fabs(vpq - temp) / (1.0f64 + fabs(vpq));
        if temp > err_tol {
            return 5 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_fhv_h_solve(
    mut fhv: *mut FHV,
    mut x: *mut libc::c_double,
) {
    let mut sva: *mut SVA = (*(*fhv).luf).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut nfs: libc::c_int = (*fhv).nfs;
    let mut hh_ind: *mut libc::c_int = (*fhv).hh_ind;
    let mut hh_ref: libc::c_int = (*fhv).hh_ref;
    let mut hh_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((hh_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut hh_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((hh_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut x_i: libc::c_double = 0.;
    k = 1 as libc::c_int;
    while k <= nfs {
        i = *hh_ind.offset(k as isize);
        x_i = *x.offset(i as isize);
        ptr = *hh_ptr.offset(k as isize);
        end = ptr + *hh_len.offset(k as isize);
        while ptr < end {
            x_i
                -= *sv_val.offset(ptr as isize)
                    * *x.offset(*sv_ind.offset(ptr as isize) as isize);
            ptr += 1;
            ptr;
        }
        *x.offset(i as isize) = x_i;
        k += 1;
        k;
    }
}
pub unsafe extern "C" fn _glp_fhv_ht_solve(
    mut fhv: *mut FHV,
    mut x: *mut libc::c_double,
) {
    let mut sva: *mut SVA = (*(*fhv).luf).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut nfs: libc::c_int = (*fhv).nfs;
    let mut hh_ind: *mut libc::c_int = (*fhv).hh_ind;
    let mut hh_ref: libc::c_int = (*fhv).hh_ref;
    let mut hh_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((hh_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut hh_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((hh_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut k: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut x_j: libc::c_double = 0.;
    k = nfs;
    while k >= 1 as libc::c_int {
        x_j = *x.offset(*hh_ind.offset(k as isize) as isize);
        if !(x_j == 0.0f64) {
            ptr = *hh_ptr.offset(k as isize);
            end = ptr + *hh_len.offset(k as isize);
            while ptr < end {
                *x.offset(*sv_ind.offset(ptr as isize) as isize)
                    -= *sv_val.offset(ptr as isize) * x_j;
                ptr += 1;
                ptr;
            }
        }
        k -= 1;
        k;
    }
}
