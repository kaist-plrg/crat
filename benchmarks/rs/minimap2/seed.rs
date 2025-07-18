use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type mm_idx_intv_s;
    pub type mm_idx_bucket_s;
    fn mm_idx_get(
        mi: *const mm_idx_t,
        minier: uint64_t,
        n: *mut libc::c_int,
    ) -> *const uint64_t;
    fn radix_sort_128x(beg: *mut mm128_t, end: *mut mm128_t);
    fn kmalloc(km: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn kfree(km: *mut libc::c_void, ptr: *mut libc::c_void);
    fn ks_heapmake_uint64_t(n: size_t, _: *mut uint64_t);
    fn ks_heapdown_uint64_t(i: size_t, n: size_t, _: *mut uint64_t);
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm128_t {
    pub x: uint64_t,
    pub y: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm128_v {
    pub n: size_t,
    pub m: size_t,
    pub a: *mut mm128_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_idx_seq_t {
    pub name: *mut libc::c_char,
    pub offset: uint64_t,
    pub len: uint32_t,
    pub is_alt: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_idx_t {
    pub b: int32_t,
    pub w: int32_t,
    pub k: int32_t,
    pub flag: int32_t,
    pub n_seq: uint32_t,
    pub index: int32_t,
    pub n_alt: int32_t,
    pub seq: *mut mm_idx_seq_t,
    pub S: *mut uint32_t,
    pub B: *mut mm_idx_bucket_s,
    pub I: *mut mm_idx_intv_s,
    pub km: *mut libc::c_void,
    pub h: *mut libc::c_void,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mm_seed_t {
    pub n: uint32_t,
    pub q_pos: uint32_t,
    #[bitfield(name = "q_span", ty = "uint32_t", bits = "0..=30")]
    #[bitfield(name = "flt", ty = "uint32_t", bits = "31..=31")]
    #[bitfield(name = "seg_id", ty = "uint32_t", bits = "32..=62")]
    #[bitfield(name = "is_tandem", ty = "uint32_t", bits = "63..=63")]
    pub q_span_flt_seg_id_is_tandem: [u8; 8],
    pub cr: *const uint64_t,
}
pub unsafe extern "C" fn mm_seed_mz_flt(
    mut km: *mut libc::c_void,
    mut mv: *mut mm128_v,
    mut q_occ_max: int32_t,
    mut q_occ_frac: libc::c_float,
) {
    let mut a: *mut mm128_t = 0 as *mut mm128_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut st: size_t = 0;
    if (*mv).n <= q_occ_max as libc::c_ulong || q_occ_frac <= 0.0f32
        || q_occ_max <= 0 as libc::c_int
    {
        return;
    }
    a = kmalloc(
        km,
        ((*mv).n).wrapping_mul(::std::mem::size_of::<mm128_t>() as libc::c_ulong),
    ) as *mut mm128_t;
    i = 0 as libc::c_int as size_t;
    while i < (*mv).n {
        (*a.offset(i as isize)).x = (*((*mv).a).offset(i as isize)).x;
        (*a.offset(i as isize)).y = i;
        i = i.wrapping_add(1);
        i;
    }
    radix_sort_128x(a, a.offset((*mv).n as isize));
    st = 0 as libc::c_int as size_t;
    i = 1 as libc::c_int as size_t;
    while i <= (*mv).n {
        if i == (*mv).n || (*a.offset(i as isize)).x != (*a.offset(st as isize)).x {
            let mut cnt: int32_t = i.wrapping_sub(st) as int32_t;
            if cnt > q_occ_max
                && cnt as libc::c_float > (*mv).n as libc::c_float * q_occ_frac
            {
                j = st;
                while j < i {
                    (*((*mv).a).offset((*a.offset(j as isize)).y as isize))
                        .x = 0 as libc::c_int as uint64_t;
                    j = j.wrapping_add(1);
                    j;
                }
            }
            st = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    kfree(km, a as *mut libc::c_void);
    j = 0 as libc::c_int as size_t;
    i = j;
    while i < (*mv).n {
        if (*((*mv).a).offset(i as isize)).x != 0 as libc::c_int as libc::c_ulong {
            let fresh0 = j;
            j = j.wrapping_add(1);
            *((*mv).a).offset(fresh0 as isize) = *((*mv).a).offset(i as isize);
        }
        i = i.wrapping_add(1);
        i;
    }
    (*mv).n = j;
}
pub unsafe extern "C" fn mm_seed_collect_all(
    mut km: *mut libc::c_void,
    mut mi: *const mm_idx_t,
    mut mv: *const mm128_v,
    mut n_m_: *mut int32_t,
) -> *mut mm_seed_t {
    let mut m: *mut mm_seed_t = 0 as *mut mm_seed_t;
    let mut i: size_t = 0;
    let mut k: int32_t = 0;
    m = kmalloc(
        km,
        ((*mv).n).wrapping_mul(::std::mem::size_of::<mm_seed_t>() as libc::c_ulong),
    ) as *mut mm_seed_t;
    k = 0 as libc::c_int;
    i = k as size_t;
    while i < (*mv).n {
        let mut cr: *const uint64_t = 0 as *const uint64_t;
        let mut q: *mut mm_seed_t = 0 as *mut mm_seed_t;
        let mut p: *mut mm128_t = &mut *((*mv).a).offset(i as isize) as *mut mm128_t;
        let mut q_pos: uint32_t = (*p).y as uint32_t;
        let mut q_span: uint32_t = ((*p).x & 0xff as libc::c_int as libc::c_ulong)
            as uint32_t;
        let mut t: libc::c_int = 0;
        cr = mm_idx_get(mi, (*p).x >> 8 as libc::c_int, &mut t);
        if !(t == 0 as libc::c_int) {
            let fresh1 = k;
            k = k + 1;
            q = &mut *m.offset(fresh1 as isize) as *mut mm_seed_t;
            (*q).q_pos = q_pos;
            (*q).set_q_span(q_span);
            (*q).cr = cr;
            (*q).n = t as uint32_t;
            (*q).set_seg_id(((*p).y >> 32 as libc::c_int) as uint32_t);
            (*q).set_flt(0 as libc::c_int as uint32_t);
            (*q).set_is_tandem((*q).flt());
            if i > 0 as libc::c_int as libc::c_ulong
                && (*p).x >> 8 as libc::c_int
                    == (*((*mv).a)
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ))
                        .x >> 8 as libc::c_int
            {
                (*q).set_is_tandem(1 as libc::c_int as uint32_t);
            }
            if i < ((*mv).n).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                && (*p).x >> 8 as libc::c_int
                    == (*((*mv).a)
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        ))
                        .x >> 8 as libc::c_int
            {
                (*q).set_is_tandem(1 as libc::c_int as uint32_t);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    *n_m_ = k;
    return m;
}
pub unsafe extern "C" fn mm_seed_select(
    mut n: int32_t,
    mut a: *mut mm_seed_t,
    mut len: libc::c_int,
    mut max_occ: libc::c_int,
    mut max_max_occ: libc::c_int,
    mut dist: libc::c_int,
) {
    let mut i: int32_t = 0;
    let mut last0: int32_t = 0;
    let mut m: int32_t = 0;
    let mut b: [uint64_t; 128] = [0; 128];
    if n == 0 as libc::c_int || n == 1 as libc::c_int {
        return;
    }
    m = 0 as libc::c_int;
    i = m;
    while i < n {
        if (*a.offset(i as isize)).n > max_occ as libc::c_uint {
            m += 1;
            m;
        }
        i += 1;
        i;
    }
    if m == 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    last0 = -(1 as libc::c_int);
    while i <= n {
        if i == n || (*a.offset(i as isize)).n <= max_occ as libc::c_uint {
            if i - last0 > 1 as libc::c_int {
                let mut ps: int32_t = (if last0 < 0 as libc::c_int {
                    0 as libc::c_int as libc::c_uint
                } else {
                    (*a.offset(last0 as isize)).q_pos >> 1 as libc::c_int
                }) as int32_t;
                let mut pe: int32_t = (if i == n {
                    len as libc::c_uint
                } else {
                    (*a.offset(i as isize)).q_pos >> 1 as libc::c_int
                }) as int32_t;
                let mut j: int32_t = 0;
                let mut k: int32_t = 0;
                let mut st: int32_t = last0 + 1 as libc::c_int;
                let mut en: int32_t = i;
                let mut max_high_occ: int32_t = ((pe - ps) as libc::c_double
                    / dist as libc::c_double + 0.499f64) as int32_t;
                if max_high_occ > 0 as libc::c_int {
                    if max_high_occ > 128 as libc::c_int {
                        max_high_occ = 128 as libc::c_int;
                    }
                    j = st;
                    k = 0 as libc::c_int;
                    while j < en && k < max_high_occ {
                        b[k
                            as usize] = ((*a.offset(j as isize)).n as uint64_t)
                            << 32 as libc::c_int | j as libc::c_ulong;
                        j += 1;
                        j;
                        k += 1;
                        k;
                    }
                    ks_heapmake_uint64_t(k as size_t, b.as_mut_ptr());
                    while j < en {
                        if (*a.offset(j as isize)).n
                            < (b[0 as libc::c_int as usize] >> 32 as libc::c_int)
                                as int32_t as libc::c_uint
                        {
                            b[0 as libc::c_int
                                as usize] = ((*a.offset(j as isize)).n as uint64_t)
                                << 32 as libc::c_int | j as libc::c_ulong;
                            ks_heapdown_uint64_t(
                                0 as libc::c_int as size_t,
                                k as size_t,
                                b.as_mut_ptr(),
                            );
                        }
                        j += 1;
                        j;
                    }
                    j = 0 as libc::c_int;
                    while j < k {
                        let ref mut fresh2 = *a
                            .offset(b[j as usize] as uint32_t as isize);
                        (*fresh2).set_flt(1 as libc::c_int as uint32_t);
                        j += 1;
                        j;
                    }
                }
                j = st;
                while j < en {
                    let ref mut fresh3 = *a.offset(j as isize);
                    (*fresh3).set_flt((*fresh3).flt() ^ 1 as libc::c_int as uint32_t);
                    j += 1;
                    j;
                }
                j = st;
                while j < en {
                    if (*a.offset(j as isize)).n > max_max_occ as libc::c_uint {
                        let ref mut fresh4 = *a.offset(j as isize);
                        (*fresh4).set_flt(1 as libc::c_int as uint32_t);
                    }
                    j += 1;
                    j;
                }
            }
            last0 = i;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn mm_collect_matches(
    mut km: *mut libc::c_void,
    mut _n_m: *mut libc::c_int,
    mut qlen: libc::c_int,
    mut max_occ: libc::c_int,
    mut max_max_occ: libc::c_int,
    mut dist: libc::c_int,
    mut mi: *const mm_idx_t,
    mut mv: *const mm128_v,
    mut n_a: *mut int64_t,
    mut rep_len: *mut libc::c_int,
    mut n_mini_pos: *mut libc::c_int,
    mut mini_pos: *mut *mut uint64_t,
) -> *mut mm_seed_t {
    let mut rep_st: libc::c_int = 0 as libc::c_int;
    let mut rep_en: libc::c_int = 0 as libc::c_int;
    let mut n_m: libc::c_int = 0;
    let mut n_m0: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut m: *mut mm_seed_t = 0 as *mut mm_seed_t;
    *n_mini_pos = 0 as libc::c_int;
    *mini_pos = kmalloc(
        km,
        ((*mv).n).wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong),
    ) as *mut uint64_t;
    m = mm_seed_collect_all(km, mi, mv, &mut n_m0);
    if dist > 0 as libc::c_int && max_max_occ > max_occ {
        mm_seed_select(n_m0, m, qlen, max_occ, max_max_occ, dist);
    } else {
        i = 0 as libc::c_int as size_t;
        while i < n_m0 as libc::c_ulong {
            if (*m.offset(i as isize)).n > max_occ as libc::c_uint {
                let ref mut fresh5 = *m.offset(i as isize);
                (*fresh5).set_flt(1 as libc::c_int as uint32_t);
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    i = 0 as libc::c_int as size_t;
    n_m = 0 as libc::c_int;
    *rep_len = 0 as libc::c_int;
    *n_a = 0 as libc::c_int as int64_t;
    while i < n_m0 as libc::c_ulong {
        let mut q: *mut mm_seed_t = &mut *m.offset(i as isize) as *mut mm_seed_t;
        if (*q).flt() != 0 {
            let mut en: libc::c_int = ((*q).q_pos >> 1 as libc::c_int)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
            let mut st: libc::c_int = en - (*q).q_span() as libc::c_int;
            if st > rep_en {
                *rep_len += rep_en - rep_st;
                rep_st = st;
                rep_en = en;
            } else {
                rep_en = en;
            }
        } else {
            *n_a += (*q).n as libc::c_long;
            let fresh6 = *n_mini_pos;
            *n_mini_pos = *n_mini_pos + 1;
            *(*mini_pos)
                .offset(
                    fresh6 as isize,
                ) = ((*q).q_span() as uint64_t) << 32 as libc::c_int
                | ((*q).q_pos >> 1 as libc::c_int) as libc::c_ulong;
            let fresh7 = n_m;
            n_m = n_m + 1;
            *m.offset(fresh7 as isize) = *q;
        }
        i = i.wrapping_add(1);
        i;
    }
    *rep_len += rep_en - rep_st;
    *_n_m = n_m;
    return m;
}
