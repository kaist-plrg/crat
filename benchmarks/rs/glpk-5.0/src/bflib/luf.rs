use ::libc;
extern "C" {
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
    fn _glp_sva_enlarge_cap(
        sva: *mut SVA,
        k: libc::c_int,
        new_cap: libc::c_int,
        skip: libc::c_int,
    );
    fn _glp_sva_reserve_cap(sva: *mut SVA, k: libc::c_int, new_cap: libc::c_int);
    fn _glp_sva_more_space(sva: *mut SVA, m_size: libc::c_int);
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
pub unsafe extern "C" fn _glp_luf_store_v_cols(
    mut luf: *mut LUF,
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
    let mut n: libc::c_int = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut vc_ref: libc::c_int = (*luf).vc_ref;
    let mut vc_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((vc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut vc_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((vc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut vc_cap: *mut libc::c_int = &mut *((*sva).cap)
        .offset((vc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
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
                    b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                    48 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if *vc_cap.offset(j as isize) < len {
            if (*sva).r_ptr - (*sva).m_ptr < len {
                _glp_sva_more_space(sva, len);
                sv_ind = (*sva).ind;
                sv_val = (*sva).val;
            }
            _glp_sva_enlarge_cap(
                sva,
                vc_ref - 1 as libc::c_int + j,
                len,
                0 as libc::c_int,
            );
        }
        ptr = *vc_ptr.offset(j as isize);
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
        *vc_len.offset(j as isize) = len;
        nnz += len;
        j += 1;
        j;
    }
    return nnz;
}
pub unsafe extern "C" fn _glp_luf_check_all(mut luf: *mut LUF, mut k: libc::c_int) {
    let mut n: libc::c_int = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut fr_ref: libc::c_int = (*luf).fr_ref;
    let mut fr_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((fr_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut fc_ref: libc::c_int = (*luf).fc_ref;
    let mut fc_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((fc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut fc_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((fc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut vr_ref: libc::c_int = (*luf).vr_ref;
    let mut vr_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((vr_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut vr_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((vr_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut vc_ref: libc::c_int = (*luf).vc_ref;
    let mut vc_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((vc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut vc_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((vc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut pp_ind: *mut libc::c_int = (*luf).pp_ind;
    let mut pp_inv: *mut libc::c_int = (*luf).pp_inv;
    let mut qq_ind: *mut libc::c_int = (*luf).qq_ind;
    let mut qq_inv: *mut libc::c_int = (*luf).qq_inv;
    let mut i: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut i_ptr: libc::c_int = 0;
    let mut i_end: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut j_ptr: libc::c_int = 0;
    let mut j_end: libc::c_int = 0;
    (n > 0 as libc::c_int
        || {
            glp_assert_(
                b"n > 0\0" as *const u8 as *const libc::c_char,
                b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                100 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= k && k <= n + 1 as libc::c_int
        || {
            glp_assert_(
                b"1 <= k && k <= n+1\0" as *const u8 as *const libc::c_char,
                b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                101 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    i = 1 as libc::c_int;
    while i <= n {
        ii = *pp_ind.offset(i as isize);
        (1 as libc::c_int <= ii && ii <= n
            || {
                glp_assert_(
                    b"1 <= ii && ii <= n\0" as *const u8 as *const libc::c_char,
                    b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                    105 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*pp_inv.offset(ii as isize) == i
            || {
                glp_assert_(
                    b"pp_inv[ii] == i\0" as *const u8 as *const libc::c_char,
                    b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                    106 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= n {
        jj = *qq_inv.offset(j as isize);
        (1 as libc::c_int <= jj && jj <= n
            || {
                glp_assert_(
                    b"1 <= jj && jj <= n\0" as *const u8 as *const libc::c_char,
                    b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                    111 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*qq_ind.offset(jj as isize) == j
            || {
                glp_assert_(
                    b"qq_ind[jj] == j\0" as *const u8 as *const libc::c_char,
                    b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                    112 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        j += 1;
        j;
    }
    i = 1 as libc::c_int;
    while i <= n {
        (*fr_len.offset(i as isize) == 0 as libc::c_int
            || {
                glp_assert_(
                    b"fr_len[i] == 0\0" as *const u8 as *const libc::c_char,
                    b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                    116 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= n {
        jj = *pp_ind.offset(j as isize);
        if jj < k {
            j_ptr = *fc_ptr.offset(j as isize);
            j_end = j_ptr + *fc_len.offset(j as isize);
            while j_ptr < j_end {
                i = *sv_ind.offset(j_ptr as isize);
                (1 as libc::c_int <= i && i <= n
                    || {
                        glp_assert_(
                            b"1 <= i && i <= n\0" as *const u8 as *const libc::c_char,
                            b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                            126 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                ii = *pp_ind.offset(i as isize);
                (ii > jj
                    || {
                        glp_assert_(
                            b"ii > jj\0" as *const u8 as *const libc::c_char,
                            b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                            128 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*sv_val.offset(j_ptr as isize) != 0.0f64
                    || {
                        glp_assert_(
                            b"sv_val[j_ptr] != 0.0\0" as *const u8
                                as *const libc::c_char,
                            b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                            129 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                j_ptr += 1;
                j_ptr;
            }
        } else {
            (*fc_len.offset(j as isize) == 0 as libc::c_int
                || {
                    glp_assert_(
                        b"fc_len[j] == 0\0" as *const u8 as *const libc::c_char,
                        b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                        133 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        j += 1;
        j;
    }
    i = 1 as libc::c_int;
    while i <= n {
        ii = *pp_ind.offset(i as isize);
        i_ptr = *vr_ptr.offset(i as isize);
        i_end = i_ptr + *vr_len.offset(i as isize);
        while i_ptr < i_end {
            j = *sv_ind.offset(i_ptr as isize);
            (1 as libc::c_int <= j && j <= n
                || {
                    glp_assert_(
                        b"1 <= j && j <= n\0" as *const u8 as *const libc::c_char,
                        b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                        143 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            jj = *qq_inv.offset(j as isize);
            if ii < k {
                (jj > ii
                    || {
                        glp_assert_(
                            b"jj > ii\0" as *const u8 as *const libc::c_char,
                            b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                            146 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            } else {
                (jj >= k
                    || {
                        glp_assert_(
                            b"jj >= k\0" as *const u8 as *const libc::c_char,
                            b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                            148 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                j_ptr = *vc_ptr.offset(j as isize);
                j_end = j_ptr + *vc_len.offset(j as isize);
                while *sv_ind.offset(j_ptr as isize) != i {
                    j_ptr += 1;
                    j_ptr;
                }
                (j_ptr < j_end
                    || {
                        glp_assert_(
                            b"j_ptr < j_end\0" as *const u8 as *const libc::c_char,
                            b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                            154 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            (*sv_val.offset(i_ptr as isize) != 0.0f64
                || {
                    glp_assert_(
                        b"sv_val[i_ptr] != 0.0\0" as *const u8 as *const libc::c_char,
                        b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                        156 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            i_ptr += 1;
            i_ptr;
        }
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= n {
        jj = *qq_inv.offset(j as isize);
        if jj < k {
            (*vc_len.offset(j as isize) == 0 as libc::c_int
                || {
                    glp_assert_(
                        b"vc_len[j] == 0\0" as *const u8 as *const libc::c_char,
                        b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                        164 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        } else {
            j_ptr = *vc_ptr.offset(j as isize);
            j_end = j_ptr + *vc_len.offset(j as isize);
            while j_ptr < j_end {
                i = *sv_ind.offset(j_ptr as isize);
                ii = *pp_ind.offset(i as isize);
                (ii >= k
                    || {
                        glp_assert_(
                            b"ii >= k\0" as *const u8 as *const libc::c_char,
                            b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                            171 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                i_ptr = *vr_ptr.offset(i as isize);
                i_end = i_ptr + *vr_len.offset(i as isize);
                while *sv_ind.offset(i_ptr as isize) != j {
                    i_ptr += 1;
                    i_ptr;
                }
                (i_ptr < i_end
                    || {
                        glp_assert_(
                            b"i_ptr < i_end\0" as *const u8 as *const libc::c_char,
                            b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                            177 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                j_ptr += 1;
                j_ptr;
            }
        }
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn _glp_luf_build_v_rows(
    mut luf: *mut LUF,
    mut len: *mut libc::c_int,
) {
    let mut n: libc::c_int = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut vr_ref: libc::c_int = (*luf).vr_ref;
    let mut vr_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((vr_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut vr_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((vr_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut vc_ref: libc::c_int = (*luf).vc_ref;
    let mut vc_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((vc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut vc_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((vc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
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
        nnz += *vc_len.offset(j as isize);
        ptr = *vc_ptr.offset(j as isize);
        end = ptr + *vc_len.offset(j as isize);
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
            _glp_sva_enlarge_cap(
                sva,
                vr_ref - 1 as libc::c_int + i,
                *len.offset(i as isize),
                0 as libc::c_int,
            );
        }
        *vr_len.offset(i as isize) = *len.offset(i as isize);
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= n {
        ptr = *vc_ptr.offset(j as isize);
        end = ptr + *vc_len.offset(j as isize);
        while ptr < end {
            i = *sv_ind.offset(ptr as isize);
            let ref mut fresh1 = *len.offset(i as isize);
            *fresh1 -= 1;
            ptr1 = *vr_ptr.offset(i as isize) + *fresh1;
            *sv_ind.offset(ptr1 as isize) = j;
            *sv_val.offset(ptr1 as isize) = *sv_val.offset(ptr as isize);
            ptr += 1;
            ptr;
        }
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn _glp_luf_build_f_rows(
    mut luf: *mut LUF,
    mut len: *mut libc::c_int,
) {
    let mut n: libc::c_int = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut fr_ref: libc::c_int = (*luf).fr_ref;
    let mut fr_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((fr_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut fr_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((fr_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut fc_ref: libc::c_int = (*luf).fc_ref;
    let mut fc_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((fc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut fc_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((fc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
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
        nnz += *fc_len.offset(j as isize);
        ptr = *fc_ptr.offset(j as isize);
        end = ptr + *fc_len.offset(j as isize);
        while ptr < end {
            let ref mut fresh2 = *len.offset(*sv_ind.offset(ptr as isize) as isize);
            *fresh2 += 1;
            *fresh2;
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
                fr_ref - 1 as libc::c_int + i,
                *len.offset(i as isize),
            );
        }
        *fr_len.offset(i as isize) = *len.offset(i as isize);
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= n {
        ptr = *fc_ptr.offset(j as isize);
        end = ptr + *fc_len.offset(j as isize);
        while ptr < end {
            i = *sv_ind.offset(ptr as isize);
            let ref mut fresh3 = *len.offset(i as isize);
            *fresh3 -= 1;
            ptr1 = *fr_ptr.offset(i as isize) + *fresh3;
            *sv_ind.offset(ptr1 as isize) = j;
            *sv_val.offset(ptr1 as isize) = *sv_val.offset(ptr as isize);
            ptr += 1;
            ptr;
        }
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn _glp_luf_build_v_cols(
    mut luf: *mut LUF,
    mut updat: libc::c_int,
    mut len: *mut libc::c_int,
) {
    let mut n: libc::c_int = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut vr_ref: libc::c_int = (*luf).vr_ref;
    let mut vr_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((vr_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut vr_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((vr_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut vc_ref: libc::c_int = (*luf).vc_ref;
    let mut vc_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((vc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut vc_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((vc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut nnz: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut ptr1: libc::c_int = 0;
    nnz = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while j <= n {
        *len.offset(j as isize) = 0 as libc::c_int;
        j += 1;
        j;
    }
    i = 1 as libc::c_int;
    while i <= n {
        nnz += *vr_len.offset(i as isize);
        ptr = *vr_ptr.offset(i as isize);
        end = ptr + *vr_len.offset(i as isize);
        while ptr < end {
            let ref mut fresh4 = *len.offset(*sv_ind.offset(ptr as isize) as isize);
            *fresh4 += 1;
            *fresh4;
            ptr += 1;
            ptr;
        }
        i += 1;
        i;
    }
    if (*sva).r_ptr - (*sva).m_ptr < nnz {
        _glp_sva_more_space(sva, nnz);
        sv_ind = (*sva).ind;
        sv_val = (*sva).val;
    }
    j = 1 as libc::c_int;
    while j <= n {
        if *len.offset(j as isize) > 0 as libc::c_int {
            if updat != 0 {
                _glp_sva_enlarge_cap(
                    sva,
                    vc_ref - 1 as libc::c_int + j,
                    *len.offset(j as isize),
                    0 as libc::c_int,
                );
            } else {
                _glp_sva_reserve_cap(
                    sva,
                    vc_ref - 1 as libc::c_int + j,
                    *len.offset(j as isize),
                );
            }
        }
        *vc_len.offset(j as isize) = *len.offset(j as isize);
        j += 1;
        j;
    }
    i = 1 as libc::c_int;
    while i <= n {
        ptr = *vr_ptr.offset(i as isize);
        end = ptr + *vr_len.offset(i as isize);
        while ptr < end {
            j = *sv_ind.offset(ptr as isize);
            let ref mut fresh5 = *len.offset(j as isize);
            *fresh5 -= 1;
            ptr1 = *vc_ptr.offset(j as isize) + *fresh5;
            *sv_ind.offset(ptr1 as isize) = i;
            *sv_val.offset(ptr1 as isize) = *sv_val.offset(ptr as isize);
            ptr += 1;
            ptr;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn _glp_luf_check_f_rc(mut luf: *mut LUF) {
    let mut n: libc::c_int = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut fr_ref: libc::c_int = (*luf).fr_ref;
    let mut fr_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((fr_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut fr_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((fr_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut fc_ref: libc::c_int = (*luf).fc_ref;
    let mut fc_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((fc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut fc_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((fc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut i_end: libc::c_int = 0;
    let mut i_ptr: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut j_end: libc::c_int = 0;
    let mut j_ptr: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= n {
        i_ptr = *fr_ptr.offset(i as isize);
        i_end = i_ptr + *fr_len.offset(i as isize);
        while i_ptr < i_end {
            j = *sv_ind.offset(i_ptr as isize);
            j_ptr = *fc_ptr.offset(j as isize);
            j_end = j_ptr + *fc_len.offset(j as isize);
            while *sv_ind.offset(j_ptr as isize) != i {
                j_ptr += 1;
                j_ptr;
            }
            (j_ptr < j_end
                || {
                    glp_assert_(
                        b"j_ptr < j_end\0" as *const u8 as *const libc::c_char,
                        b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                        389 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*sv_val.offset(i_ptr as isize) == *sv_val.offset(j_ptr as isize)
                || {
                    glp_assert_(
                        b"sv_val[i_ptr] == sv_val[j_ptr]\0" as *const u8
                            as *const libc::c_char,
                        b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                        390 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            *sv_ind.offset(j_ptr as isize) = -i;
            i_ptr += 1;
            i_ptr;
        }
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= n {
        j_ptr = *fc_ptr.offset(j as isize);
        j_end = j_ptr + *fc_len.offset(j as isize);
        while j_ptr < j_end {
            i = *sv_ind.offset(j_ptr as isize);
            (i < 0 as libc::c_int
                || {
                    glp_assert_(
                        b"(i = sv_ind[j_ptr]) < 0\0" as *const u8 as *const libc::c_char,
                        b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                        400 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            *sv_ind.offset(j_ptr as isize) = -i;
            j_ptr += 1;
            j_ptr;
        }
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn _glp_luf_check_v_rc(mut luf: *mut LUF) {
    let mut n: libc::c_int = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut vr_ref: libc::c_int = (*luf).vr_ref;
    let mut vr_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((vr_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut vr_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((vr_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut vc_ref: libc::c_int = (*luf).vc_ref;
    let mut vc_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((vc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut vc_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((vc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut i_end: libc::c_int = 0;
    let mut i_ptr: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut j_end: libc::c_int = 0;
    let mut j_ptr: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= n {
        i_ptr = *vr_ptr.offset(i as isize);
        i_end = i_ptr + *vr_len.offset(i as isize);
        while i_ptr < i_end {
            j = *sv_ind.offset(i_ptr as isize);
            j_ptr = *vc_ptr.offset(j as isize);
            j_end = j_ptr + *vc_len.offset(j as isize);
            while *sv_ind.offset(j_ptr as isize) != i {
                j_ptr += 1;
                j_ptr;
            }
            (j_ptr < j_end
                || {
                    glp_assert_(
                        b"j_ptr < j_end\0" as *const u8 as *const libc::c_char,
                        b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                        437 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*sv_val.offset(i_ptr as isize) == *sv_val.offset(j_ptr as isize)
                || {
                    glp_assert_(
                        b"sv_val[i_ptr] == sv_val[j_ptr]\0" as *const u8
                            as *const libc::c_char,
                        b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                        438 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            *sv_ind.offset(j_ptr as isize) = -i;
            i_ptr += 1;
            i_ptr;
        }
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= n {
        j_ptr = *vc_ptr.offset(j as isize);
        j_end = j_ptr + *vc_len.offset(j as isize);
        while j_ptr < j_end {
            i = *sv_ind.offset(j_ptr as isize);
            (i < 0 as libc::c_int
                || {
                    glp_assert_(
                        b"(i = sv_ind[j_ptr]) < 0\0" as *const u8 as *const libc::c_char,
                        b"bflib/luf.c\0" as *const u8 as *const libc::c_char,
                        448 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            *sv_ind.offset(j_ptr as isize) = -i;
            j_ptr += 1;
            j_ptr;
        }
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn _glp_luf_f_solve(
    mut luf: *mut LUF,
    mut x: *mut libc::c_double,
) {
    let mut n: libc::c_int = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut fc_ref: libc::c_int = (*luf).fc_ref;
    let mut fc_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((fc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut fc_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((fc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut pp_inv: *mut libc::c_int = (*luf).pp_inv;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut x_j: libc::c_double = 0.;
    k = 1 as libc::c_int;
    while k <= n {
        j = *pp_inv.offset(k as isize);
        x_j = *x.offset(j as isize);
        if x_j != 0.0f64 {
            ptr = *fc_ptr.offset(j as isize);
            end = ptr + *fc_len.offset(j as isize);
            while ptr < end {
                *x.offset(*sv_ind.offset(ptr as isize) as isize)
                    -= *sv_val.offset(ptr as isize) * x_j;
                ptr += 1;
                ptr;
            }
        }
        k += 1;
        k;
    }
}
pub unsafe extern "C" fn _glp_luf_ft_solve(
    mut luf: *mut LUF,
    mut x: *mut libc::c_double,
) {
    let mut n: libc::c_int = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut fr_ref: libc::c_int = (*luf).fr_ref;
    let mut fr_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((fr_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut fr_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((fr_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut pp_inv: *mut libc::c_int = (*luf).pp_inv;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut x_i: libc::c_double = 0.;
    k = n;
    while k >= 1 as libc::c_int {
        i = *pp_inv.offset(k as isize);
        x_i = *x.offset(i as isize);
        if x_i != 0.0f64 {
            ptr = *fr_ptr.offset(i as isize);
            end = ptr + *fr_len.offset(i as isize);
            while ptr < end {
                *x.offset(*sv_ind.offset(ptr as isize) as isize)
                    -= *sv_val.offset(ptr as isize) * x_i;
                ptr += 1;
                ptr;
            }
        }
        k -= 1;
        k;
    }
}
pub unsafe extern "C" fn _glp_luf_v_solve(
    mut luf: *mut LUF,
    mut b: *mut libc::c_double,
    mut x: *mut libc::c_double,
) {
    let mut n: libc::c_int = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut vr_piv: *mut libc::c_double = (*luf).vr_piv;
    let mut vc_ref: libc::c_int = (*luf).vc_ref;
    let mut vc_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((vc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut vc_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((vc_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut pp_inv: *mut libc::c_int = (*luf).pp_inv;
    let mut qq_ind: *mut libc::c_int = (*luf).qq_ind;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut x_j: libc::c_double = 0.;
    k = n;
    while k >= 1 as libc::c_int {
        i = *pp_inv.offset(k as isize);
        j = *qq_ind.offset(k as isize);
        let ref mut fresh6 = *x.offset(j as isize);
        *fresh6 = *b.offset(i as isize) / *vr_piv.offset(i as isize);
        x_j = *fresh6;
        if x_j != 0.0f64 {
            ptr = *vc_ptr.offset(j as isize);
            end = ptr + *vc_len.offset(j as isize);
            while ptr < end {
                *b.offset(*sv_ind.offset(ptr as isize) as isize)
                    -= *sv_val.offset(ptr as isize) * x_j;
                ptr += 1;
                ptr;
            }
        }
        k -= 1;
        k;
    }
}
pub unsafe extern "C" fn _glp_luf_vt_solve(
    mut luf: *mut LUF,
    mut b: *mut libc::c_double,
    mut x: *mut libc::c_double,
) {
    let mut n: libc::c_int = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut vr_piv: *mut libc::c_double = (*luf).vr_piv;
    let mut vr_ref: libc::c_int = (*luf).vr_ref;
    let mut vr_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((vr_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut vr_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((vr_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut pp_inv: *mut libc::c_int = (*luf).pp_inv;
    let mut qq_ind: *mut libc::c_int = (*luf).qq_ind;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut x_i: libc::c_double = 0.;
    k = 1 as libc::c_int;
    while k <= n {
        i = *pp_inv.offset(k as isize);
        j = *qq_ind.offset(k as isize);
        let ref mut fresh7 = *x.offset(i as isize);
        *fresh7 = *b.offset(j as isize) / *vr_piv.offset(i as isize);
        x_i = *fresh7;
        if x_i != 0.0f64 {
            ptr = *vr_ptr.offset(i as isize);
            end = ptr + *vr_len.offset(i as isize);
            while ptr < end {
                *b.offset(*sv_ind.offset(ptr as isize) as isize)
                    -= *sv_val.offset(ptr as isize) * x_i;
                ptr += 1;
                ptr;
            }
        }
        k += 1;
        k;
    }
}
pub unsafe extern "C" fn _glp_luf_vt_solve1(
    mut luf: *mut LUF,
    mut e: *mut libc::c_double,
    mut y: *mut libc::c_double,
) {
    let mut n: libc::c_int = (*luf).n;
    let mut sva: *mut SVA = (*luf).sva;
    let mut sv_ind: *mut libc::c_int = (*sva).ind;
    let mut sv_val: *mut libc::c_double = (*sva).val;
    let mut vr_piv: *mut libc::c_double = (*luf).vr_piv;
    let mut vr_ref: libc::c_int = (*luf).vr_ref;
    let mut vr_ptr: *mut libc::c_int = &mut *((*sva).ptr)
        .offset((vr_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut vr_len: *mut libc::c_int = &mut *((*sva).len)
        .offset((vr_ref - 1 as libc::c_int) as isize) as *mut libc::c_int;
    let mut pp_inv: *mut libc::c_int = (*luf).pp_inv;
    let mut qq_ind: *mut libc::c_int = (*luf).qq_ind;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut e_j: libc::c_double = 0.;
    let mut y_i: libc::c_double = 0.;
    k = 1 as libc::c_int;
    while k <= n {
        i = *pp_inv.offset(k as isize);
        j = *qq_ind.offset(k as isize);
        e_j = if *e.offset(j as isize) >= 0.0f64 {
            *e.offset(j as isize) + 1.0f64
        } else {
            *e.offset(j as isize) - 1.0f64
        };
        let ref mut fresh8 = *y.offset(i as isize);
        *fresh8 = e_j / *vr_piv.offset(i as isize);
        y_i = *fresh8;
        ptr = *vr_ptr.offset(i as isize);
        end = ptr + *vr_len.offset(i as isize);
        while ptr < end {
            *e.offset(*sv_ind.offset(ptr as isize) as isize)
                -= *sv_val.offset(ptr as isize) * y_i;
            ptr += 1;
            ptr;
        }
        k += 1;
        k;
    }
}
pub unsafe extern "C" fn _glp_luf_estimate_norm(
    mut luf: *mut LUF,
    mut w1: *mut libc::c_double,
    mut w2: *mut libc::c_double,
) -> libc::c_double {
    let mut n: libc::c_int = (*luf).n;
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
    _glp_luf_vt_solve1(luf, e, y);
    _glp_luf_ft_solve(luf, y);
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
    _glp_luf_f_solve(luf, y);
    _glp_luf_v_solve(luf, y, z);
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
