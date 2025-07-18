use ::libc;
extern "C" {
    fn _glp_sva_reserve_cap(sva: *mut SVA, k: libc::c_int, new_cap: libc::c_int);
    fn _glp_sva_more_space(sva: *mut SVA, m_size: libc::c_int);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn _glp_luf_f_solve(luf: *mut LUF, x: *mut libc::c_double);
    fn _glp_luf_ft_solve(luf: *mut LUF, x: *mut libc::c_double);
    fn _glp_luf_v_solve(luf: *mut LUF, b: *mut libc::c_double, x: *mut libc::c_double);
    fn _glp_luf_vt_solve(luf: *mut LUF, b: *mut libc::c_double, x: *mut libc::c_double);
    fn _glp_luf_vt_solve1(luf: *mut LUF, e: *mut libc::c_double, y: *mut libc::c_double);
    fn _glp_mc13d(
        n: libc::c_int,
        icn: *const libc::c_int,
        ip: *const libc::c_int,
        lenr: *const libc::c_int,
        ior: *mut libc::c_int,
        ib: *mut libc::c_int,
        lowl: *mut libc::c_int,
        numb: *mut libc::c_int,
        prev: *mut libc::c_int,
    ) -> libc::c_int;
    fn _glp_mc21a(
        n: libc::c_int,
        icn: *const libc::c_int,
        ip: *const libc::c_int,
        lenr: *const libc::c_int,
        iperm: *mut libc::c_int,
        pr: *mut libc::c_int,
        arp: *mut libc::c_int,
        cv: *mut libc::c_int,
        out: *mut libc::c_int,
    ) -> libc::c_int;
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
pub unsafe extern "C" fn _glp_btf_store_a_cols(
    mut btf: *mut BTF,
    mut col: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *mut libc::c_int,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
    mut info: *mut libc::c_void,
    mut ind: *mut libc::c_int,
    mut val: *mut libc::c_double,
) -> libc::c_int {
    let mut n: libc::c_int = (*btf).n;
    let mut sva: *mut SVA = (*btf).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut ac_ref: libc::c_int = (*btf).ac_ref;
    let mut ac_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((ac_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut ac_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((ac_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut nnz: libc::c_int = 0;
    nnz = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while j <= n {
        len = col.unwrap()(info, j, ind, val);
        (0 as libc::c_int <= len && len <= n
            || {
                glp_assert_(
                    b"0 <= len && len <= n\0" as *const u8 as *const libc::c_char,
                    b"bflib/btf.c\0" as *const u8 as *const libc::c_char,
                    49 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if len > 0 as libc::c_int {
            if (*sva).r_ptr - (*sva).m_ptr < len {
                _glp_sva_more_space(sva, len);
                sv_ind = (*sva).ind;
            }
            _glp_sva_reserve_cap(sva, ac_ref + (j - 1 as libc::c_int), len);
        }
        ptr = *ac_ptr.offset(j as isize);
        memcpy(
            &mut *sv_ind.offset(ptr as isize) as *mut libc::c_int as *mut libc::c_void,
            &mut *ind.offset(1 as libc::c_int as isize) as *mut libc::c_int
                as *const libc::c_void,
            (len as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        *ac_len.offset(j as isize) = len;
        nnz += len;
        j += 1;
        j;
    }
    return nnz;
}
pub unsafe extern "C" fn _glp_btf_make_blocks(mut btf: *mut BTF) -> libc::c_int {
    let mut n: libc::c_int = (*btf).n;
    let mut sva: *mut SVA = (*btf).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut pp_ind: *mut libc::c_int = (*btf).pp_ind;
    let mut pp_inv: *mut libc::c_int = (*btf).pp_inv;
    let mut qq_ind: *mut libc::c_int = (*btf).qq_ind;
    let mut qq_inv: *mut libc::c_int = (*btf).qq_inv;
    let mut beg: *mut libc::c_int = (*btf).beg;
    let mut ac_ref: libc::c_int = (*btf).ac_ref;
    let mut ac_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((ac_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut ac_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((ac_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut rank: libc::c_int = 0;
    let mut iperm: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut arp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut cv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut out: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ip: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut lenr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut lowl: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut numb: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut prev: *mut libc::c_int = 0 as *mut libc::c_int;
    iperm = qq_inv;
    pr = (*btf).p1_ind;
    arp = (*btf).p1_inv;
    cv = (*btf).q1_ind;
    out = (*btf).q1_inv;
    rank = _glp_mc21a(
        n,
        sv_ind as *const libc::c_int,
        ac_ptr as *const libc::c_int,
        ac_len as *const libc::c_int,
        iperm,
        pr,
        arp,
        cv,
        out,
    );
    (0 as libc::c_int <= rank && rank <= n
        || {
            glp_assert_(
                b"0 <= rank && rank <= n\0" as *const u8 as *const libc::c_char,
                b"bflib/btf.c\0" as *const u8 as *const libc::c_char,
                98 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if !(rank < n) {
        ip = pp_ind;
        lenr = qq_ind;
        j = 1 as libc::c_int;
        while j <= n {
            *ip.offset(j as isize) = *ac_ptr.offset(*iperm.offset(j as isize) as isize);
            *lenr
                .offset(j as isize) = *ac_len.offset(*iperm.offset(j as isize) as isize);
            j += 1;
            j;
        }
        lowl = (*btf).p1_ind;
        numb = (*btf).p1_inv;
        prev = (*btf).q1_ind;
        (*btf)
            .num = _glp_mc13d(
            n,
            sv_ind as *const libc::c_int,
            ip as *const libc::c_int,
            lenr as *const libc::c_int,
            pp_inv,
            beg,
            lowl,
            numb,
            prev,
        );
        (*beg.offset(1 as libc::c_int as isize) == 1 as libc::c_int
            || {
                glp_assert_(
                    b"beg[1] == 1\0" as *const u8 as *const libc::c_char,
                    b"bflib/btf.c\0" as *const u8 as *const libc::c_char,
                    117 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        *beg.offset(((*btf).num + 1 as libc::c_int) as isize) = n + 1 as libc::c_int;
        j = 1 as libc::c_int;
        while j <= n {
            *pp_ind.offset(*pp_inv.offset(j as isize) as isize) = j;
            j += 1;
            j;
        }
        i = 1 as libc::c_int;
        while i <= n {
            *qq_ind
                .offset(i as isize) = *iperm.offset(*pp_inv.offset(i as isize) as isize);
            i += 1;
            i;
        }
        i = 1 as libc::c_int;
        while i <= n {
            *qq_inv.offset(*qq_ind.offset(i as isize) as isize) = i;
            i += 1;
            i;
        }
    }
    return rank;
}
pub unsafe extern "C" fn _glp_btf_check_blocks(mut btf: *mut BTF) {
    let mut n: libc::c_int = (*btf).n;
    let mut sva: *mut SVA = (*btf).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut pp_ind: *mut libc::c_int = (*btf).pp_ind;
    let mut pp_inv: *mut libc::c_int = (*btf).pp_inv;
    let mut qq_ind: *mut libc::c_int = (*btf).qq_ind;
    let mut qq_inv: *mut libc::c_int = (*btf).qq_inv;
    let mut num: libc::c_int = (*btf).num;
    let mut beg: *mut libc::c_int = (*btf).beg;
    let mut ac_ref: libc::c_int = (*btf).ac_ref;
    let mut ac_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((ac_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut ac_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((ac_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut diag: libc::c_int = 0;
    (n > 0 as libc::c_int
        || {
            glp_assert_(
                b"n > 0\0" as *const u8 as *const libc::c_char,
                b"bflib/btf.c\0" as *const u8 as *const libc::c_char,
                153 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = 1 as libc::c_int;
    while k <= n {
        (1 as libc::c_int <= *pp_ind.offset(k as isize)
            && *pp_ind.offset(k as isize) <= n
            || {
                glp_assert_(
                    b"1 <= pp_ind[k] && pp_ind[k] <= n\0" as *const u8
                        as *const libc::c_char,
                    b"bflib/btf.c\0" as *const u8 as *const libc::c_char,
                    156 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*pp_inv.offset(*pp_ind.offset(k as isize) as isize) == k
            || {
                glp_assert_(
                    b"pp_inv[pp_ind[k]] == k\0" as *const u8 as *const libc::c_char,
                    b"bflib/btf.c\0" as *const u8 as *const libc::c_char,
                    157 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (1 as libc::c_int <= *qq_ind.offset(k as isize)
            && *qq_ind.offset(k as isize) <= n
            || {
                glp_assert_(
                    b"1 <= qq_ind[k] && qq_ind[k] <= n\0" as *const u8
                        as *const libc::c_char,
                    b"bflib/btf.c\0" as *const u8 as *const libc::c_char,
                    158 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*qq_inv.offset(*qq_ind.offset(k as isize) as isize) == k
            || {
                glp_assert_(
                    b"qq_inv[qq_ind[k]] == k\0" as *const u8 as *const libc::c_char,
                    b"bflib/btf.c\0" as *const u8 as *const libc::c_char,
                    159 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        k += 1;
        k;
    }
    (1 as libc::c_int <= num && num <= n
        || {
            glp_assert_(
                b"1 <= num && num <= n\0" as *const u8 as *const libc::c_char,
                b"bflib/btf.c\0" as *const u8 as *const libc::c_char,
                163 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*beg.offset(1 as libc::c_int as isize) == 1 as libc::c_int
        || {
            glp_assert_(
                b"beg[1] == 1\0" as *const u8 as *const libc::c_char,
                b"bflib/btf.c\0" as *const u8 as *const libc::c_char,
                164 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*beg.offset((num + 1 as libc::c_int) as isize) == n + 1 as libc::c_int
        || {
            glp_assert_(
                b"beg[num+1] == n+1\0" as *const u8 as *const libc::c_char,
                b"bflib/btf.c\0" as *const u8 as *const libc::c_char,
                165 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = 1 as libc::c_int;
    while k <= num {
        size = *beg.offset((k + 1 as libc::c_int) as isize) - *beg.offset(k as isize);
        (size >= 1 as libc::c_int
            || {
                glp_assert_(
                    b"size >= 1\0" as *const u8 as *const libc::c_char,
                    b"bflib/btf.c\0" as *const u8 as *const libc::c_char,
                    170 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        jj = *beg.offset(k as isize);
        while jj < *beg.offset((k + 1 as libc::c_int) as isize) {
            diag = 0 as libc::c_int;
            j = *qq_ind.offset(jj as isize);
            ptr = *ac_ptr.offset(j as isize);
            end = ptr + *ac_len.offset(j as isize);
            while ptr < end {
                i = *sv_ind.offset(ptr as isize);
                ii = *pp_ind.offset(i as isize);
                (ii < *beg.offset((k + 1 as libc::c_int) as isize)
                    || {
                        glp_assert_(
                            b"ii < beg[k+1]\0" as *const u8 as *const libc::c_char,
                            b"bflib/btf.c\0" as *const u8 as *const libc::c_char,
                            185 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                if ii == jj {
                    diag = 1 as libc::c_int;
                }
                ptr += 1;
                ptr;
            }
            (diag != 0
                || {
                    glp_assert_(
                        b"diag\0" as *const u8 as *const libc::c_char,
                        b"bflib/btf.c\0" as *const u8 as *const libc::c_char,
                        191 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            jj += 1;
            jj;
        }
        k += 1;
        k;
    }
}
pub unsafe extern "C" fn _glp_btf_build_a_rows(
    mut btf: *mut BTF,
    mut len: *mut libc::c_int,
) {
    let mut n: libc::c_int = (*btf).n;
    let mut sva: *mut SVA = (*btf).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut ar_ref: libc::c_int = (*btf).ar_ref;
    let mut ar_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((ar_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut ar_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((ar_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut ac_ref: libc::c_int = (*btf).ac_ref;
    let mut ac_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((ac_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut ac_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((ac_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut nnz: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut ptr1: libc::c_int = 0;
    nnz = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= n {
        *len.offset(i as isize) = 0 as libc::c_int;
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= n {
        nnz += *ac_len.offset(j as isize);
        ptr = *ac_ptr.offset(j as isize);
        end = ptr + *ac_len.offset(j as isize);
        while ptr < end {
            let ref mut fresh0 = *len.offset(*sv_ind.offset(ptr as isize) as isize);
            *fresh0 += 1;
            *fresh0;
            ptr += 1;
            ptr;
        }
        j += 1;
        j;
    }
    if (*sva).r_ptr - (*sva).m_ptr < nnz {
        _glp_sva_more_space(sva, nnz);
        sv_ind = (*sva).ind;
        sv_val = (*sva).val;
    }
    i = 1 as libc::c_int;
    while i <= n {
        if *len.offset(i as isize) > 0 as libc::c_int {
            _glp_sva_reserve_cap(
                sva,
                ar_ref - 1 as libc::c_int + i,
                *len.offset(i as isize),
            );
        }
        *ar_len.offset(i as isize) = *len.offset(i as isize);
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= n {
        ptr = *ac_ptr.offset(j as isize);
        end = ptr + *ac_len.offset(j as isize);
        while ptr < end {
            i = *sv_ind.offset(ptr as isize);
            let ref mut fresh1 = *len.offset(i as isize);
            *fresh1 -= 1;
            ptr1 = *ar_ptr.offset(i as isize) + *fresh1;
            *sv_ind.offset(ptr1 as isize) = j;
            *sv_val.offset(ptr1 as isize) = *sv_val.offset(ptr as isize);
            ptr += 1;
            ptr;
        }
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn _glp_btf_a_solve(
    mut btf: *mut BTF,
    mut b: *mut libc::c_double,
    mut x: *mut libc::c_double,
    mut w1: *mut libc::c_double,
    mut w2: *mut libc::c_double,
) {
    let mut sva: *mut SVA = (*btf).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut pp_inv: *mut libc::c_int = (*btf).pp_inv;
    let mut qq_ind: *mut libc::c_int = (*btf).qq_ind;
    let mut num: libc::c_int = (*btf).num;
    let mut beg: *mut libc::c_int = (*btf).beg;
    let mut ac_ref: libc::c_int = (*btf).ac_ref;
    let mut ac_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((ac_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut ac_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((ac_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut bb: *mut libc::c_double = w1;
    let mut xx: *mut libc::c_double = w2;
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
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut beg_k: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut t: libc::c_double = 0.;
    k = num;
    while k >= 1 as libc::c_int {
        beg_k = *beg.offset(k as isize);
        luf.n = *beg.offset((k + 1 as libc::c_int) as isize) - beg_k;
        if luf.n == 1 as libc::c_int {
            let ref mut fresh2 = *x.offset(*qq_ind.offset(beg_k as isize) as isize);
            *fresh2 = *b.offset(*pp_inv.offset(beg_k as isize) as isize)
                / *((*btf).vr_piv).offset(beg_k as isize);
            t = *fresh2;
            if t != 0.0f64 {
                let mut ptr: libc::c_int = *ac_ptr
                    .offset(*qq_ind.offset(beg_k as isize) as isize);
                let mut end: libc::c_int = ptr
                    + *ac_len.offset(*qq_ind.offset(beg_k as isize) as isize);
                while ptr < end {
                    *b.offset(*sv_ind.offset(ptr as isize) as isize)
                        -= *sv_val.offset(ptr as isize) * t;
                    ptr += 1;
                    ptr;
                }
            }
        } else {
            flag = 0 as libc::c_int;
            i = 1 as libc::c_int;
            while i <= luf.n {
                let ref mut fresh3 = *bb.offset(i as isize);
                *fresh3 = *b
                    .offset(
                        *pp_inv.offset((i + (beg_k - 1 as libc::c_int)) as isize)
                            as isize,
                    );
                if *fresh3 != 0.0f64 {
                    flag = 1 as libc::c_int;
                }
                i += 1;
                i;
            }
            if flag == 0 {
                j = 1 as libc::c_int;
                while j <= luf.n {
                    *x
                        .offset(
                            *qq_ind.offset((j + (beg_k - 1 as libc::c_int)) as isize)
                                as isize,
                        ) = 0.0f64;
                    j += 1;
                    j;
                }
            } else {
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
                _glp_luf_f_solve(&mut luf, bb);
                _glp_luf_v_solve(&mut luf, bb, xx);
                j = 1 as libc::c_int;
                while j <= luf.n {
                    jj = j + (beg_k - 1 as libc::c_int);
                    let ref mut fresh4 = *x.offset(*qq_ind.offset(jj as isize) as isize);
                    *fresh4 = *xx.offset(j as isize);
                    t = *fresh4;
                    if t != 0.0f64 {
                        let mut ptr_0: libc::c_int = *ac_ptr
                            .offset(*qq_ind.offset(jj as isize) as isize);
                        let mut end_0: libc::c_int = ptr_0
                            + *ac_len.offset(*qq_ind.offset(jj as isize) as isize);
                        while ptr_0 < end_0 {
                            *b.offset(*sv_ind.offset(ptr_0 as isize) as isize)
                                -= *sv_val.offset(ptr_0 as isize) * t;
                            ptr_0 += 1;
                            ptr_0;
                        }
                    }
                    j += 1;
                    j;
                }
            }
        }
        k -= 1;
        k;
    }
}
pub unsafe extern "C" fn _glp_btf_at_solve(
    mut btf: *mut BTF,
    mut b: *mut libc::c_double,
    mut x: *mut libc::c_double,
    mut w1: *mut libc::c_double,
    mut w2: *mut libc::c_double,
) {
    let mut sva: *mut SVA = (*btf).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut pp_inv: *mut libc::c_int = (*btf).pp_inv;
    let mut qq_ind: *mut libc::c_int = (*btf).qq_ind;
    let mut num: libc::c_int = (*btf).num;
    let mut beg: *mut libc::c_int = (*btf).beg;
    let mut ar_ref: libc::c_int = (*btf).ar_ref;
    let mut ar_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((ar_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut ar_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((ar_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut bb: *mut libc::c_double = w1;
    let mut xx: *mut libc::c_double = w2;
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
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut beg_k: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut t: libc::c_double = 0.;
    k = 1 as libc::c_int;
    while k <= num {
        beg_k = *beg.offset(k as isize);
        luf.n = *beg.offset((k + 1 as libc::c_int) as isize) - beg_k;
        if luf.n == 1 as libc::c_int {
            let ref mut fresh5 = *x.offset(*pp_inv.offset(beg_k as isize) as isize);
            *fresh5 = *b.offset(*qq_ind.offset(beg_k as isize) as isize)
                / *((*btf).vr_piv).offset(beg_k as isize);
            t = *fresh5;
            if t != 0.0f64 {
                let mut ptr: libc::c_int = *ar_ptr
                    .offset(*pp_inv.offset(beg_k as isize) as isize);
                let mut end: libc::c_int = ptr
                    + *ar_len.offset(*pp_inv.offset(beg_k as isize) as isize);
                while ptr < end {
                    *b.offset(*sv_ind.offset(ptr as isize) as isize)
                        -= *sv_val.offset(ptr as isize) * t;
                    ptr += 1;
                    ptr;
                }
            }
        } else {
            flag = 0 as libc::c_int;
            i = 1 as libc::c_int;
            while i <= luf.n {
                let ref mut fresh6 = *bb.offset(i as isize);
                *fresh6 = *b
                    .offset(
                        *qq_ind.offset((i + (beg_k - 1 as libc::c_int)) as isize)
                            as isize,
                    );
                if *fresh6 != 0.0f64 {
                    flag = 1 as libc::c_int;
                }
                i += 1;
                i;
            }
            if flag == 0 {
                j = 1 as libc::c_int;
                while j <= luf.n {
                    *x
                        .offset(
                            *pp_inv.offset((j + (beg_k - 1 as libc::c_int)) as isize)
                                as isize,
                        ) = 0.0f64;
                    j += 1;
                    j;
                }
            } else {
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
                _glp_luf_vt_solve(&mut luf, bb, xx);
                _glp_luf_ft_solve(&mut luf, xx);
                j = 1 as libc::c_int;
                while j <= luf.n {
                    jj = j + (beg_k - 1 as libc::c_int);
                    let ref mut fresh7 = *x.offset(*pp_inv.offset(jj as isize) as isize);
                    *fresh7 = *xx.offset(j as isize);
                    t = *fresh7;
                    if t != 0.0f64 {
                        let mut ptr_0: libc::c_int = *ar_ptr
                            .offset(*pp_inv.offset(jj as isize) as isize);
                        let mut end_0: libc::c_int = ptr_0
                            + *ar_len.offset(*pp_inv.offset(jj as isize) as isize);
                        while ptr_0 < end_0 {
                            *b.offset(*sv_ind.offset(ptr_0 as isize) as isize)
                                -= *sv_val.offset(ptr_0 as isize) * t;
                            ptr_0 += 1;
                            ptr_0;
                        }
                    }
                    j += 1;
                    j;
                }
            }
        }
        k += 1;
        k;
    }
}
pub unsafe extern "C" fn _glp_btf_at_solve1(
    mut btf: *mut BTF,
    mut e: *mut libc::c_double,
    mut y: *mut libc::c_double,
    mut w1: *mut libc::c_double,
    mut w2: *mut libc::c_double,
) {
    let mut sva: *mut SVA = (*btf).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut pp_inv: *mut libc::c_int = (*btf).pp_inv;
    let mut qq_ind: *mut libc::c_int = (*btf).qq_ind;
    let mut num: libc::c_int = (*btf).num;
    let mut beg: *mut libc::c_int = (*btf).beg;
    let mut ar_ref: libc::c_int = (*btf).ar_ref;
    let mut ar_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((ar_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut ar_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((ar_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut ee: *mut libc::c_double = w1;
    let mut yy: *mut libc::c_double = w2;
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
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut beg_k: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut e_k: libc::c_double = 0.;
    let mut y_k: libc::c_double = 0.;
    k = 1 as libc::c_int;
    while k <= num {
        beg_k = *beg.offset(k as isize);
        luf.n = *beg.offset((k + 1 as libc::c_int) as isize) - beg_k;
        if luf.n == 1 as libc::c_int {
            e_k = *e.offset(*qq_ind.offset(beg_k as isize) as isize);
            e_k = if e_k >= 0.0f64 { e_k + 1.0f64 } else { e_k - 1.0f64 };
            let ref mut fresh8 = *y.offset(*pp_inv.offset(beg_k as isize) as isize);
            *fresh8 = e_k / *((*btf).vr_piv).offset(beg_k as isize);
            y_k = *fresh8;
            ptr = *ar_ptr.offset(*pp_inv.offset(beg_k as isize) as isize);
            end = ptr + *ar_len.offset(*pp_inv.offset(beg_k as isize) as isize);
            while ptr < end {
                *e.offset(*sv_ind.offset(ptr as isize) as isize)
                    -= *sv_val.offset(ptr as isize) * y_k;
                ptr += 1;
                ptr;
            }
        } else {
            i = 1 as libc::c_int;
            while i <= luf.n {
                *ee
                    .offset(
                        i as isize,
                    ) = *e
                    .offset(
                        *qq_ind.offset((i + (beg_k - 1 as libc::c_int)) as isize)
                            as isize,
                    );
                i += 1;
                i;
            }
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
            _glp_luf_vt_solve1(&mut luf, ee, yy);
            _glp_luf_ft_solve(&mut luf, yy);
            j = 1 as libc::c_int;
            while j <= luf.n {
                jj = j + (beg_k - 1 as libc::c_int);
                let ref mut fresh9 = *y.offset(*pp_inv.offset(jj as isize) as isize);
                *fresh9 = *yy.offset(j as isize);
                y_k = *fresh9;
                ptr = *ar_ptr.offset(*pp_inv.offset(jj as isize) as isize);
                end = ptr + *ar_len.offset(*pp_inv.offset(jj as isize) as isize);
                while ptr < end {
                    *e.offset(*sv_ind.offset(ptr as isize) as isize)
                        -= *sv_val.offset(ptr as isize) * y_k;
                    ptr += 1;
                    ptr;
                }
                j += 1;
                j;
            }
        }
        k += 1;
        k;
    }
}
pub unsafe extern "C" fn _glp_btf_estimate_norm(
    mut btf: *mut BTF,
    mut w1: *mut libc::c_double,
    mut w2: *mut libc::c_double,
    mut w3: *mut libc::c_double,
    mut w4: *mut libc::c_double,
) -> libc::c_double {
    let mut n: libc::c_int = (*btf).n;
    let mut e: *mut libc::c_double = w1;
    let mut y: *mut libc::c_double = w2;
    let mut z: *mut libc::c_double = w1;
    let mut i: libc::c_int = 0;
    let mut y_norm: libc::c_double = 0.;
    let mut z_norm: libc::c_double = 0.;
    i = 1 as libc::c_int;
    while i <= n {
        *e.offset(i as isize) = 0.0f64;
        i += 1;
        i;
    }
    _glp_btf_at_solve1(btf, e, y, w3, w4);
    y_norm = 0.0f64;
    i = 1 as libc::c_int;
    while i <= n {
        y_norm
            += if *y.offset(i as isize) >= 0.0f64 {
                *y.offset(i as isize)
            } else {
                -*y.offset(i as isize)
            };
        i += 1;
        i;
    }
    _glp_btf_a_solve(btf, y, z, w3, w4);
    z_norm = 0.0f64;
    i = 1 as libc::c_int;
    while i <= n {
        z_norm
            += if *z.offset(i as isize) >= 0.0f64 {
                *z.offset(i as isize)
            } else {
                -*z.offset(i as isize)
            };
        i += 1;
        i;
    }
    return z_norm / y_norm;
}
