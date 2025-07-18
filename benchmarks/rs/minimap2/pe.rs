use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn abs(_: libc::c_int) -> libc::c_int;
    fn logf(_: libc::c_float) -> libc::c_float;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn mm_sync_regs(km: *mut libc::c_void, n_regs: libc::c_int, regs: *mut mm_reg1_t);
    fn radix_sort_64(beg: *mut uint64_t, end: *mut uint64_t);
    fn kfree(km: *mut libc::c_void, ptr: *mut libc::c_void);
    fn kmalloc(km: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn krealloc(
        km: *mut libc::c_void,
        ptr: *mut libc::c_void,
        size: size_t,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mm_extra_t {
    pub capacity: uint32_t,
    pub dp_score: int32_t,
    pub dp_max: int32_t,
    pub dp_max2: int32_t,
    #[bitfield(name = "n_ambi", ty = "uint32_t", bits = "0..=29")]
    #[bitfield(name = "trans_strand", ty = "uint32_t", bits = "30..=31")]
    pub n_ambi_trans_strand: [u8; 4],
    pub n_cigar: uint32_t,
    pub cigar: [uint32_t; 0],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mm_reg1_t {
    pub id: int32_t,
    pub cnt: int32_t,
    pub rid: int32_t,
    pub score: int32_t,
    pub qs: int32_t,
    pub qe: int32_t,
    pub rs: int32_t,
    pub re: int32_t,
    pub parent: int32_t,
    pub subsc: int32_t,
    pub as_0: int32_t,
    pub mlen: int32_t,
    pub blen: int32_t,
    pub n_sub: int32_t,
    pub score0: int32_t,
    #[bitfield(name = "mapq", ty = "uint32_t", bits = "0..=7")]
    #[bitfield(name = "split", ty = "uint32_t", bits = "8..=9")]
    #[bitfield(name = "rev", ty = "uint32_t", bits = "10..=10")]
    #[bitfield(name = "inv", ty = "uint32_t", bits = "11..=11")]
    #[bitfield(name = "sam_pri", ty = "uint32_t", bits = "12..=12")]
    #[bitfield(name = "proper_frag", ty = "uint32_t", bits = "13..=13")]
    #[bitfield(name = "pe_thru", ty = "uint32_t", bits = "14..=14")]
    #[bitfield(name = "seg_split", ty = "uint32_t", bits = "15..=15")]
    #[bitfield(name = "seg_id", ty = "uint32_t", bits = "16..=23")]
    #[bitfield(name = "split_inv", ty = "uint32_t", bits = "24..=24")]
    #[bitfield(name = "is_alt", ty = "uint32_t", bits = "25..=25")]
    #[bitfield(name = "strand_retained", ty = "uint32_t", bits = "26..=26")]
    #[bitfield(name = "dummy", ty = "uint32_t", bits = "27..=31")]
    pub mapq_split_rev_inv_sam_pri_proper_frag_pe_thru_seg_split_seg_id_split_inv_is_alt_strand_retained_dummy: [u8; 4],
    pub hash: uint32_t,
    pub div: libc::c_float,
    pub p: *mut mm_extra_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub n: size_t,
    pub m: size_t,
    pub a: *mut uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pair_arr_t {
    pub s: libc::c_int,
    pub rev: libc::c_int,
    pub key: uint64_t,
    pub r: *mut mm_reg1_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsbucket_pair_t {
    pub b: *mut pair_arr_t,
    pub e: *mut pair_arr_t,
}
pub unsafe extern "C" fn mm_select_sub_multi(
    mut km: *mut libc::c_void,
    mut pri_ratio: libc::c_float,
    mut pri1: libc::c_float,
    mut pri2: libc::c_float,
    mut max_gap_ref: libc::c_int,
    mut min_diff: libc::c_int,
    mut best_n: libc::c_int,
    mut n_segs: libc::c_int,
    mut qlens: *const libc::c_int,
    mut n_: *mut libc::c_int,
    mut r: *mut mm_reg1_t,
) {
    if pri_ratio > 0.0f32 && *n_ > 0 as libc::c_int {
        let mut i: libc::c_int = 0;
        let mut k: libc::c_int = 0;
        let mut n: libc::c_int = *n_;
        let mut n_2nd: libc::c_int = 0 as libc::c_int;
        let mut max_dist: libc::c_int = if n_segs == 2 as libc::c_int {
            *qlens.offset(0 as libc::c_int as isize)
                + *qlens.offset(1 as libc::c_int as isize) + max_gap_ref
        } else {
            0 as libc::c_int
        };
        k = 0 as libc::c_int;
        i = k;
        while i < n {
            let mut to_keep: libc::c_int = 0 as libc::c_int;
            if (*r.offset(i as isize)).parent == i {
                to_keep = 1 as libc::c_int;
            } else if (*r.offset(i as isize)).score + min_diff
                >= (*r.offset((*r.offset(i as isize)).parent as isize)).score
            {
                to_keep = 1 as libc::c_int;
            } else {
                let mut p: *mut mm_reg1_t = &mut *r
                    .offset((*r.offset(i as isize)).parent as isize) as *mut mm_reg1_t;
                let mut q: *mut mm_reg1_t = &mut *r.offset(i as isize) as *mut mm_reg1_t;
                if (*p).rev() as libc::c_int == (*q).rev() as libc::c_int
                    && (*p).rid == (*q).rid && (*q).re - (*p).rs < max_dist
                    && (*p).re - (*q).rs < max_dist
                {
                    if (*q).score as libc::c_float >= (*p).score as libc::c_float * pri1
                    {
                        to_keep = 1 as libc::c_int;
                    }
                } else {
                    let mut is_par_both: libc::c_int = (n_segs == 2 as libc::c_int
                        && (*p).qs < *qlens.offset(0 as libc::c_int as isize)
                        && (*p).qe > *qlens.offset(0 as libc::c_int as isize))
                        as libc::c_int;
                    let mut is_chi_both: libc::c_int = (n_segs == 2 as libc::c_int
                        && (*q).qs < *qlens.offset(0 as libc::c_int as isize)
                        && (*q).qe > *qlens.offset(0 as libc::c_int as isize))
                        as libc::c_int;
                    if is_chi_both != 0 || is_chi_both == is_par_both {
                        if (*q).score as libc::c_float
                            >= (*p).score as libc::c_float * pri_ratio
                        {
                            to_keep = 1 as libc::c_int;
                        }
                    } else if (*q).score as libc::c_float
                        >= (*p).score as libc::c_float * pri2
                    {
                        to_keep = 1 as libc::c_int;
                    }
                }
            }
            if to_keep != 0 && (*r.offset(i as isize)).parent != i {
                let fresh0 = n_2nd;
                n_2nd = n_2nd + 1;
                if fresh0 >= best_n {
                    to_keep = 0 as libc::c_int;
                }
            }
            if to_keep != 0 {
                let fresh1 = k;
                k = k + 1;
                *r.offset(fresh1 as isize) = *r.offset(i as isize);
            } else if !((*r.offset(i as isize)).p).is_null() {
                free((*r.offset(i as isize)).p as *mut libc::c_void);
            }
            i += 1;
            i;
        }
        if k != n {
            mm_sync_regs(km, k, r);
        }
        *n_ = k;
    }
}
pub unsafe extern "C" fn mm_set_pe_thru(
    mut qlens: *const libc::c_int,
    mut n_regs: *mut libc::c_int,
    mut regs: *mut *mut mm_reg1_t,
) {
    let mut s: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut n_pri: [libc::c_int; 2] = [0; 2];
    let mut pri: [libc::c_int; 2] = [0; 2];
    n_pri[1 as libc::c_int as usize] = 0 as libc::c_int;
    n_pri[0 as libc::c_int as usize] = n_pri[1 as libc::c_int as usize];
    pri[1 as libc::c_int as usize] = -(1 as libc::c_int);
    pri[0 as libc::c_int as usize] = pri[1 as libc::c_int as usize];
    s = 0 as libc::c_int;
    while s < 2 as libc::c_int {
        i = 0 as libc::c_int;
        while i < *n_regs.offset(s as isize) {
            if (*(*regs.offset(s as isize)).offset(i as isize)).id
                == (*(*regs.offset(s as isize)).offset(i as isize)).parent
            {
                n_pri[s as usize] += 1;
                n_pri[s as usize];
                pri[s as usize] = i;
            }
            i += 1;
            i;
        }
        s += 1;
        s;
    }
    if n_pri[0 as libc::c_int as usize] == 1 as libc::c_int
        && n_pri[1 as libc::c_int as usize] == 1 as libc::c_int
    {
        let mut p: *mut mm_reg1_t = &mut *(*regs.offset(0 as libc::c_int as isize))
            .offset(*pri.as_mut_ptr().offset(0 as libc::c_int as isize) as isize)
            as *mut mm_reg1_t;
        let mut q: *mut mm_reg1_t = &mut *(*regs.offset(1 as libc::c_int as isize))
            .offset(*pri.as_mut_ptr().offset(1 as libc::c_int as isize) as isize)
            as *mut mm_reg1_t;
        if (*p).rid == (*q).rid && (*p).rev() as libc::c_int == (*q).rev() as libc::c_int
            && abs((*p).rs - (*q).rs) < 3 as libc::c_int
            && abs((*p).re - (*q).re) < 3 as libc::c_int
            && ((*p).qs == 0 as libc::c_int
                && *qlens.offset(1 as libc::c_int as isize) - (*q).qe == 0 as libc::c_int
                || (*q).qs == 0 as libc::c_int
                    && *qlens.offset(0 as libc::c_int as isize) - (*p).qe
                        == 0 as libc::c_int)
        {
            (*q).set_pe_thru(1 as libc::c_int as uint32_t);
            (*p).set_pe_thru((*q).pe_thru());
        }
    }
}
pub unsafe extern "C" fn radix_sort_pair(
    mut beg: *mut pair_arr_t,
    mut end: *mut pair_arr_t,
) {
    if end.offset_from(beg) as libc::c_long <= 64 as libc::c_int as libc::c_long {
        rs_insertsort_pair(beg, end);
    } else {
        rs_sort_pair(
            beg,
            end,
            8 as libc::c_int,
            (8 as libc::c_int - 1 as libc::c_int) * 8 as libc::c_int,
        );
    };
}
pub unsafe extern "C" fn rs_insertsort_pair(
    mut beg: *mut pair_arr_t,
    mut end: *mut pair_arr_t,
) {
    let mut i: *mut pair_arr_t = 0 as *mut pair_arr_t;
    i = beg.offset(1 as libc::c_int as isize);
    while i < end {
        if (*i).key < (*i.offset(-(1 as libc::c_int as isize))).key {
            let mut j: *mut pair_arr_t = 0 as *mut pair_arr_t;
            let mut tmp: pair_arr_t = *i;
            j = i;
            while j > beg && tmp.key < (*j.offset(-(1 as libc::c_int as isize))).key {
                *j = *j.offset(-(1 as libc::c_int as isize));
                j = j.offset(-1);
                j;
            }
            *j = tmp;
        }
        i = i.offset(1);
        i;
    }
}
pub unsafe extern "C" fn rs_sort_pair(
    mut beg: *mut pair_arr_t,
    mut end: *mut pair_arr_t,
    mut n_bits: libc::c_int,
    mut s: libc::c_int,
) {
    let mut i: *mut pair_arr_t = 0 as *mut pair_arr_t;
    let mut size: libc::c_int = (1 as libc::c_int) << n_bits;
    let mut m: libc::c_int = size - 1 as libc::c_int;
    let mut k: *mut rsbucket_pair_t = 0 as *mut rsbucket_pair_t;
    let mut b: [rsbucket_pair_t; 256] = [rsbucket_pair_t {
        b: 0 as *mut pair_arr_t,
        e: 0 as *mut pair_arr_t,
    }; 256];
    let mut be: *mut rsbucket_pair_t = b.as_mut_ptr().offset(size as isize);
    if n_bits <= 8 as libc::c_int {} else {
        __assert_fail(
            b"n_bits <= RS_MAX_BITS\0" as *const u8 as *const libc::c_char,
            b"pe.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"void rs_sort_pair(pair_arr_t *, pair_arr_t *, int, int)\0"))
                .as_ptr(),
        );
    }
    'c_7144: {
        if n_bits <= 8 as libc::c_int {} else {
            __assert_fail(
                b"n_bits <= RS_MAX_BITS\0" as *const u8 as *const libc::c_char,
                b"pe.c\0" as *const u8 as *const libc::c_char,
                74 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 56],
                    &[libc::c_char; 56],
                >(b"void rs_sort_pair(pair_arr_t *, pair_arr_t *, int, int)\0"))
                    .as_ptr(),
            );
        }
    };
    k = b.as_mut_ptr();
    while k != be {
        (*k).e = beg;
        (*k).b = (*k).e;
        k = k.offset(1);
        k;
    }
    i = beg;
    while i != end {
        b[((*i).key >> s & m as libc::c_ulong) as usize]
            .e = (b[((*i).key >> s & m as libc::c_ulong) as usize].e).offset(1);
        b[((*i).key >> s & m as libc::c_ulong) as usize].e;
        i = i.offset(1);
        i;
    }
    k = b.as_mut_ptr().offset(1 as libc::c_int as isize);
    while k != be {
        (*k)
            .e = ((*k).e)
            .offset(
                ((*k.offset(-(1 as libc::c_int as isize))).e).offset_from(beg)
                    as libc::c_long as isize,
            );
        (*k).b = (*k.offset(-(1 as libc::c_int as isize))).e;
        k = k.offset(1);
        k;
    }
    k = b.as_mut_ptr();
    while k != be {
        if (*k).b != (*k).e {
            let mut l: *mut rsbucket_pair_t = 0 as *mut rsbucket_pair_t;
            l = b
                .as_mut_ptr()
                .offset(((*(*k).b).key >> s & m as libc::c_ulong) as isize);
            if l != k {
                let mut tmp: pair_arr_t = *(*k).b;
                let mut swap: pair_arr_t = pair_arr_t {
                    s: 0,
                    rev: 0,
                    key: 0,
                    r: 0 as *mut mm_reg1_t,
                };
                loop {
                    swap = tmp;
                    tmp = *(*l).b;
                    let fresh2 = (*l).b;
                    (*l).b = ((*l).b).offset(1);
                    *fresh2 = swap;
                    l = b
                        .as_mut_ptr()
                        .offset((tmp.key >> s & m as libc::c_ulong) as isize);
                    if !(l != k) {
                        break;
                    }
                }
                let fresh3 = (*k).b;
                (*k).b = ((*k).b).offset(1);
                *fresh3 = tmp;
            } else {
                (*k).b = ((*k).b).offset(1);
                (*k).b;
            }
        } else {
            k = k.offset(1);
            k;
        }
    }
    let ref mut fresh4 = (*b.as_mut_ptr()).b;
    *fresh4 = beg;
    k = b.as_mut_ptr().offset(1 as libc::c_int as isize);
    while k != be {
        (*k).b = (*k.offset(-(1 as libc::c_int as isize))).e;
        k = k.offset(1);
        k;
    }
    if s != 0 {
        s = if s > n_bits { s - n_bits } else { 0 as libc::c_int };
        k = b.as_mut_ptr();
        while k != be {
            if ((*k).e).offset_from((*k).b) as libc::c_long
                > 64 as libc::c_int as libc::c_long
            {
                rs_sort_pair((*k).b, (*k).e, n_bits, s);
            } else if ((*k).e).offset_from((*k).b) as libc::c_long
                > 1 as libc::c_int as libc::c_long
            {
                rs_insertsort_pair((*k).b, (*k).e);
            }
            k = k.offset(1);
            k;
        }
    }
}
pub unsafe extern "C" fn mm_pair(
    mut km: *mut libc::c_void,
    mut max_gap_ref: libc::c_int,
    mut pe_bonus: libc::c_int,
    mut sub_diff: libc::c_int,
    mut match_sc: libc::c_int,
    mut qlens: *const libc::c_int,
    mut n_regs: *mut libc::c_int,
    mut regs: *mut *mut mm_reg1_t,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut last: [libc::c_int; 2] = [0; 2];
    let mut dp_thres: libc::c_int = 0;
    let mut segs: libc::c_int = 0 as libc::c_int;
    let mut max_idx: [libc::c_int; 2] = [0; 2];
    let mut max: int64_t = 0;
    let mut a: *mut pair_arr_t = 0 as *mut pair_arr_t;
    let mut sc: C2RustUnnamed = {
        let mut init = C2RustUnnamed {
            n: 0 as libc::c_int as size_t,
            m: 0 as libc::c_int as size_t,
            a: 0 as *mut uint64_t,
        };
        init
    };
    a = kmalloc(
        km,
        ((*n_regs.offset(0 as libc::c_int as isize)
            + *n_regs.offset(1 as libc::c_int as isize)) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<pair_arr_t>() as libc::c_ulong),
    ) as *mut pair_arr_t;
    n = 0 as libc::c_int;
    s = n;
    dp_thres = 0 as libc::c_int;
    while s < 2 as libc::c_int {
        let mut max_0: libc::c_int = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < *n_regs.offset(s as isize) {
            (*a.offset(n as isize)).s = s;
            let ref mut fresh5 = (*a.offset(n as isize)).r;
            *fresh5 = &mut *(*regs.offset(s as isize)).offset(i as isize)
                as *mut mm_reg1_t;
            (*a.offset(n as isize))
                .rev = (*(*a.offset(n as isize)).r).rev() as libc::c_int;
            (*a.offset(n as isize))
                .key = ((*(*a.offset(n as isize)).r).rid as uint64_t)
                << 32 as libc::c_int
                | ((*(*a.offset(n as isize)).r).rs << 1 as libc::c_int) as libc::c_ulong
                | (s ^ (*a.offset(n as isize)).rev) as libc::c_ulong;
            max_0 = if max_0 > (*(*(*a.offset(n as isize)).r).p).dp_max {
                max_0
            } else {
                (*(*(*a.offset(n as isize)).r).p).dp_max
            };
            n += 1;
            n;
            segs |= (1 as libc::c_int) << s;
            i += 1;
            i;
        }
        dp_thres += max_0;
        s += 1;
        s;
    }
    if segs != 3 as libc::c_int {
        kfree(km, a as *mut libc::c_void);
        return;
    }
    dp_thres -= pe_bonus;
    if dp_thres < 0 as libc::c_int {
        dp_thres = 0 as libc::c_int;
    }
    radix_sort_pair(a, a.offset(n as isize));
    max = -(1 as libc::c_int) as int64_t;
    max_idx[1 as libc::c_int as usize] = -(1 as libc::c_int);
    max_idx[0 as libc::c_int as usize] = max_idx[1 as libc::c_int as usize];
    last[1 as libc::c_int as usize] = -(1 as libc::c_int);
    last[0 as libc::c_int as usize] = last[1 as libc::c_int as usize];
    if sc.m < n as size_t {
        sc.m = n as size_t;
        sc.m = (sc.m).wrapping_sub(1);
        sc.m;
        sc.m |= sc.m >> 1 as libc::c_int;
        sc.m |= sc.m >> 2 as libc::c_int;
        sc.m |= sc.m >> 4 as libc::c_int;
        sc.m |= sc.m >> 8 as libc::c_int;
        sc.m |= sc.m >> 16 as libc::c_int;
        sc.m = (sc.m).wrapping_add(1);
        sc.m;
        sc
            .a = krealloc(
            km,
            sc.a as *mut libc::c_void,
            (::std::mem::size_of::<uint64_t>() as libc::c_ulong).wrapping_mul(sc.m),
        ) as *mut uint64_t;
    }
    i = 0 as libc::c_int;
    while i < n {
        if (*a.offset(i as isize)).key & 1 as libc::c_int as libc::c_ulong != 0 {
            let mut q: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
            let mut r: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
            if !(last[(*a.offset(i as isize)).rev as usize] < 0 as libc::c_int) {
                r = (*a.offset(i as isize)).r;
                q = (*a.offset(last[(*a.offset(i as isize)).rev as usize] as isize)).r;
                if !((*r).rid != (*q).rid || (*r).rs - (*q).re > max_gap_ref) {
                    j = last[(*a.offset(i as isize)).rev as usize];
                    while j >= 0 as libc::c_int {
                        let mut score: int64_t = 0;
                        if !((*a.offset(j as isize)).rev != (*a.offset(i as isize)).rev
                            || (*a.offset(j as isize)).s == (*a.offset(i as isize)).s)
                        {
                            q = (*a.offset(j as isize)).r;
                            if (*r).rid != (*q).rid || (*r).rs - (*q).re > max_gap_ref {
                                break;
                            }
                            if !((*(*r).p).dp_max + (*(*q).p).dp_max < dp_thres) {
                                score = (((*(*r).p).dp_max + (*(*q).p).dp_max) as int64_t)
                                    << 32 as libc::c_int
                                    | ((*r).hash).wrapping_add((*q).hash) as libc::c_long;
                                if score > max {
                                    max = score;
                                    max_idx[(*a.offset(j as isize)).s as usize] = j;
                                    max_idx[(*a.offset(i as isize)).s as usize] = i;
                                }
                                if sc.n == sc.m {
                                    sc
                                        .m = if sc.m != 0 {
                                        sc.m << 1 as libc::c_int
                                    } else {
                                        2 as libc::c_int as libc::c_ulong
                                    };
                                    sc
                                        .a = krealloc(
                                        km,
                                        sc.a as *mut libc::c_void,
                                        (::std::mem::size_of::<uint64_t>() as libc::c_ulong)
                                            .wrapping_mul(sc.m),
                                    ) as *mut uint64_t;
                                }
                                let fresh6 = sc.n;
                                sc.n = (sc.n).wrapping_add(1);
                                *(sc.a).offset(fresh6 as isize) = score as uint64_t;
                            }
                        }
                        j -= 1;
                        j;
                    }
                }
            }
        } else {
            last[(*a.offset(i as isize)).rev as usize] = i;
        }
        i += 1;
        i;
    }
    if sc.n > 1 as libc::c_int as libc::c_ulong {
        radix_sort_64(sc.a, (sc.a).offset(sc.n as isize));
    }
    if sc.n > 0 as libc::c_int as libc::c_ulong && max > 0 as libc::c_int as libc::c_long
    {
        let mut n_sub: libc::c_int = 0 as libc::c_int;
        let mut mapq_pe: libc::c_int = 0;
        let mut r_0: [*mut mm_reg1_t; 2] = [0 as *mut mm_reg1_t; 2];
        r_0[0 as libc::c_int
            as usize] = (*a.offset(max_idx[0 as libc::c_int as usize] as isize)).r;
        r_0[1 as libc::c_int
            as usize] = (*a.offset(max_idx[1 as libc::c_int as usize] as isize)).r;
        (*r_0[1 as libc::c_int as usize]).set_proper_frag(1 as libc::c_int as uint32_t);
        (*r_0[0 as libc::c_int as usize])
            .set_proper_frag((*r_0[1 as libc::c_int as usize]).proper_frag());
        s = 0 as libc::c_int;
        while s < 2 as libc::c_int {
            if (*r_0[s as usize]).id != (*r_0[s as usize]).parent {
                let mut p: *mut mm_reg1_t = &mut *(*regs.offset(s as isize))
                    .offset((**r_0.as_mut_ptr().offset(s as isize)).parent as isize)
                    as *mut mm_reg1_t;
                i = 0 as libc::c_int;
                while i < *n_regs.offset(s as isize) {
                    if (*(*regs.offset(s as isize)).offset(i as isize)).parent == (*p).id
                    {
                        (*(*regs.offset(s as isize)).offset(i as isize))
                            .parent = (*r_0[s as usize]).id;
                    }
                    i += 1;
                    i;
                }
                (*p).set_mapq(0 as libc::c_int as uint32_t);
            }
            if (*r_0[s as usize]).sam_pri() == 0 {
                i = 0 as libc::c_int;
                while i < *n_regs.offset(s as isize) {
                    let ref mut fresh7 = *(*regs.offset(s as isize)).offset(i as isize);
                    (*fresh7).set_sam_pri(0 as libc::c_int as uint32_t);
                    i += 1;
                    i;
                }
                (*r_0[s as usize]).set_sam_pri(1 as libc::c_int as uint32_t);
            }
            s += 1;
            s;
        }
        mapq_pe = if (*r_0[0 as libc::c_int as usize]).mapq() as libc::c_int
            > (*r_0[1 as libc::c_int as usize]).mapq() as libc::c_int
        {
            (*r_0[0 as libc::c_int as usize]).mapq() as libc::c_int
        } else {
            (*r_0[1 as libc::c_int as usize]).mapq() as libc::c_int
        };
        i = 0 as libc::c_int;
        while i < sc.n as libc::c_int {
            if (*(sc.a).offset(i as isize) >> 32 as libc::c_int)
                .wrapping_add(sub_diff as libc::c_ulong)
                >= max as uint64_t >> 32 as libc::c_int
            {
                n_sub += 1;
                n_sub;
            }
            i += 1;
            i;
        }
        if sc.n > 1 as libc::c_int as libc::c_ulong {
            let mut mapq_pe_alt: libc::c_int = 0;
            mapq_pe_alt = (6.02f32
                * ((max >> 32 as libc::c_int) as libc::c_ulong)
                    .wrapping_sub(
                        *(sc.a)
                            .offset(
                                (sc.n).wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) >> 32 as libc::c_int,
                    ) as libc::c_float / match_sc as libc::c_float
                - 4.343f32 * logf(n_sub as libc::c_float)) as libc::c_int;
            mapq_pe = if mapq_pe < mapq_pe_alt { mapq_pe } else { mapq_pe_alt };
        }
        if ((*r_0[0 as libc::c_int as usize]).mapq() as libc::c_int) < mapq_pe {
            (*r_0[0 as libc::c_int as usize])
                .set_mapq(
                    (0.2f32
                        * (*r_0[0 as libc::c_int as usize]).mapq() as libc::c_int
                            as libc::c_float + 0.8f32 * mapq_pe as libc::c_float
                        + 0.499f32) as libc::c_int as uint32_t,
                );
        }
        if ((*r_0[1 as libc::c_int as usize]).mapq() as libc::c_int) < mapq_pe {
            (*r_0[1 as libc::c_int as usize])
                .set_mapq(
                    (0.2f32
                        * (*r_0[1 as libc::c_int as usize]).mapq() as libc::c_int
                            as libc::c_float + 0.8f32 * mapq_pe as libc::c_float
                        + 0.499f32) as libc::c_int as uint32_t,
                );
        }
        if sc.n == 1 as libc::c_int as libc::c_ulong {
            if ((*r_0[0 as libc::c_int as usize]).mapq() as libc::c_int)
                < 2 as libc::c_int
            {
                (*r_0[0 as libc::c_int as usize]).set_mapq(2 as libc::c_int as uint32_t);
            }
            if ((*r_0[1 as libc::c_int as usize]).mapq() as libc::c_int)
                < 2 as libc::c_int
            {
                (*r_0[1 as libc::c_int as usize]).set_mapq(2 as libc::c_int as uint32_t);
            }
        } else if max as uint64_t >> 32 as libc::c_int
            > *(sc.a)
                .offset((sc.n).wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
                >> 32 as libc::c_int
        {
            if ((*r_0[0 as libc::c_int as usize]).mapq() as libc::c_int)
                < 1 as libc::c_int
            {
                (*r_0[0 as libc::c_int as usize]).set_mapq(1 as libc::c_int as uint32_t);
            }
            if ((*r_0[1 as libc::c_int as usize]).mapq() as libc::c_int)
                < 1 as libc::c_int
            {
                (*r_0[1 as libc::c_int as usize]).set_mapq(1 as libc::c_int as uint32_t);
            }
        }
    }
    kfree(km, a as *mut libc::c_void);
    kfree(km, sc.a as *mut libc::c_void);
    mm_set_pe_thru(qlens, n_regs, regs);
}
