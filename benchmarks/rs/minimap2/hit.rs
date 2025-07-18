use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type mm_idx_intv_s;
    pub type mm_idx_bucket_s;
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
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn logf(_: libc::c_float) -> libc::c_float;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn radix_sort_128x(beg: *mut mm128_t, end: *mut mm128_t);
    fn radix_sort_64(beg: *mut uint64_t, end: *mut uint64_t);
    fn kcalloc(km: *mut libc::c_void, count: size_t, size: size_t) -> *mut libc::c_void;
    fn kfree(km: *mut libc::c_void, ptr: *mut libc::c_void);
    fn kmalloc(km: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm128_t {
    pub x: uint64_t,
    pub y: uint64_t,
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
pub struct mm_mapopt_t {
    pub flag: int64_t,
    pub seed: libc::c_int,
    pub sdust_thres: libc::c_int,
    pub max_qlen: libc::c_int,
    pub bw: libc::c_int,
    pub bw_long: libc::c_int,
    pub max_gap: libc::c_int,
    pub max_gap_ref: libc::c_int,
    pub max_frag_len: libc::c_int,
    pub max_chain_skip: libc::c_int,
    pub max_chain_iter: libc::c_int,
    pub min_cnt: libc::c_int,
    pub min_chain_score: libc::c_int,
    pub chain_gap_scale: libc::c_float,
    pub chain_skip_scale: libc::c_float,
    pub rmq_size_cap: libc::c_int,
    pub rmq_inner_dist: libc::c_int,
    pub rmq_rescue_size: libc::c_int,
    pub rmq_rescue_ratio: libc::c_float,
    pub mask_level: libc::c_float,
    pub mask_len: libc::c_int,
    pub pri_ratio: libc::c_float,
    pub best_n: libc::c_int,
    pub alt_drop: libc::c_float,
    pub a: libc::c_int,
    pub b: libc::c_int,
    pub q: libc::c_int,
    pub e: libc::c_int,
    pub q2: libc::c_int,
    pub e2: libc::c_int,
    pub sc_ambi: libc::c_int,
    pub noncan: libc::c_int,
    pub junc_bonus: libc::c_int,
    pub zdrop: libc::c_int,
    pub zdrop_inv: libc::c_int,
    pub end_bonus: libc::c_int,
    pub min_dp_max: libc::c_int,
    pub min_ksw_len: libc::c_int,
    pub anchor_ext_len: libc::c_int,
    pub anchor_ext_shift: libc::c_int,
    pub max_clip_ratio: libc::c_float,
    pub rank_min_len: libc::c_int,
    pub rank_frac: libc::c_float,
    pub pe_ori: libc::c_int,
    pub pe_bonus: libc::c_int,
    pub mid_occ_frac: libc::c_float,
    pub q_occ_frac: libc::c_float,
    pub min_mid_occ: int32_t,
    pub max_mid_occ: int32_t,
    pub mid_occ: int32_t,
    pub max_occ: int32_t,
    pub max_max_occ: int32_t,
    pub occ_dist: int32_t,
    pub mini_batch_size: int64_t,
    pub max_sw_mat: int64_t,
    pub cap_kalloc: int64_t,
    pub split_prefix: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_seg_t {
    pub n_u: libc::c_int,
    pub n_a: libc::c_int,
    pub u: *mut uint64_t,
    pub a: *mut mm128_t,
}
#[inline]
unsafe extern "C" fn mm_cal_fuzzy_len(mut r: *mut mm_reg1_t, mut a: *const mm128_t) {
    let mut i: libc::c_int = 0;
    (*r).blen = 0 as libc::c_int;
    (*r).mlen = (*r).blen;
    if (*r).cnt <= 0 as libc::c_int {
        return;
    }
    (*r)
        .blen = ((*a.offset((*r).as_0 as isize)).y >> 32 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as int32_t;
    (*r).mlen = (*r).blen;
    i = (*r).as_0 + 1 as libc::c_int;
    while i < (*r).as_0 + (*r).cnt {
        let mut span: libc::c_int = ((*a.offset(i as isize)).y >> 32 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as libc::c_int;
        let mut tl: libc::c_int = (*a.offset(i as isize)).x as int32_t
            - (*a.offset((i - 1 as libc::c_int) as isize)).x as int32_t;
        let mut ql: libc::c_int = (*a.offset(i as isize)).y as int32_t
            - (*a.offset((i - 1 as libc::c_int) as isize)).y as int32_t;
        (*r).blen += if tl > ql { tl } else { ql };
        (*r).mlen
            += if tl > span && ql > span { span } else if tl < ql { tl } else { ql };
        i += 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn mm_reg_set_coor(
    mut r: *mut mm_reg1_t,
    mut qlen: int32_t,
    mut a: *const mm128_t,
    mut is_qstrand: libc::c_int,
) {
    let mut k: int32_t = (*r).as_0;
    let mut q_span: int32_t = ((*a.offset(k as isize)).y >> 32 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as int32_t;
    (*r).set_rev(((*a.offset(k as isize)).x >> 63 as libc::c_int) as uint32_t);
    (*r)
        .rid = ((*a.offset(k as isize)).x << 1 as libc::c_int >> 33 as libc::c_int)
        as int32_t;
    (*r)
        .rs = if (*a.offset(k as isize)).x as int32_t + 1 as libc::c_int > q_span {
        (*a.offset(k as isize)).x as int32_t + 1 as libc::c_int - q_span
    } else {
        0 as libc::c_int
    };
    (*r)
        .re = (*a.offset((k + (*r).cnt - 1 as libc::c_int) as isize)).x as int32_t
        + 1 as libc::c_int;
    if (*r).rev() == 0 || is_qstrand != 0 {
        (*r).qs = (*a.offset(k as isize)).y as int32_t + 1 as libc::c_int - q_span;
        (*r)
            .qe = (*a.offset((k + (*r).cnt - 1 as libc::c_int) as isize)).y as int32_t
            + 1 as libc::c_int;
    } else {
        (*r)
            .qs = qlen
            - ((*a.offset((k + (*r).cnt - 1 as libc::c_int) as isize)).y as int32_t
                + 1 as libc::c_int);
        (*r)
            .qe = qlen
            - ((*a.offset(k as isize)).y as int32_t + 1 as libc::c_int - q_span);
    }
    mm_cal_fuzzy_len(r, a);
}
#[inline]
unsafe extern "C" fn hash64(mut key: uint64_t) -> uint64_t {
    key = (!key).wrapping_add(key << 21 as libc::c_int);
    key = key ^ key >> 24 as libc::c_int;
    key = key
        .wrapping_add(key << 3 as libc::c_int)
        .wrapping_add(key << 8 as libc::c_int);
    key = key ^ key >> 14 as libc::c_int;
    key = key
        .wrapping_add(key << 2 as libc::c_int)
        .wrapping_add(key << 4 as libc::c_int);
    key = key ^ key >> 28 as libc::c_int;
    key = key.wrapping_add(key << 31 as libc::c_int);
    return key;
}
pub unsafe extern "C" fn mm_gen_regs(
    mut km: *mut libc::c_void,
    mut hash: uint32_t,
    mut qlen: libc::c_int,
    mut n_u: libc::c_int,
    mut u: *mut uint64_t,
    mut a: *mut mm128_t,
    mut is_qstrand: libc::c_int,
) -> *mut mm_reg1_t {
    let mut z: *mut mm128_t = 0 as *mut mm128_t;
    let mut tmp: mm128_t = mm128_t { x: 0, y: 0 };
    let mut r: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if n_u == 0 as libc::c_int {
        return 0 as *mut mm_reg1_t;
    }
    z = kmalloc(km, (n_u * 16 as libc::c_int) as size_t) as *mut mm128_t;
    k = 0 as libc::c_int;
    i = k;
    while i < n_u {
        let mut h: uint32_t = 0;
        h = hash64(
            (hash64((*a.offset(k as isize)).x))
                .wrapping_add(hash64((*a.offset(k as isize)).y)) ^ hash as libc::c_ulong,
        ) as uint32_t;
        (*z.offset(i as isize)).x = *u.offset(i as isize) ^ h as libc::c_ulong;
        (*z.offset(i as isize))
            .y = (k as uint64_t) << 32 as libc::c_int
            | *u.offset(i as isize) as int32_t as libc::c_ulong;
        k += *u.offset(i as isize) as int32_t;
        i += 1;
        i;
    }
    radix_sort_128x(z, z.offset(n_u as isize));
    i = 0 as libc::c_int;
    while i < n_u >> 1 as libc::c_int {
        tmp = *z.offset(i as isize);
        *z.offset(i as isize) = *z.offset((n_u - 1 as libc::c_int - i) as isize);
        *z.offset((n_u - 1 as libc::c_int - i) as isize) = tmp;
        i += 1;
        i;
    }
    r = calloc(n_u as libc::c_ulong, ::std::mem::size_of::<mm_reg1_t>() as libc::c_ulong)
        as *mut mm_reg1_t;
    i = 0 as libc::c_int;
    while i < n_u {
        let mut ri: *mut mm_reg1_t = &mut *r.offset(i as isize) as *mut mm_reg1_t;
        (*ri).id = i;
        (*ri).parent = -(1 as libc::c_int);
        (*ri).score0 = ((*z.offset(i as isize)).x >> 32 as libc::c_int) as int32_t;
        (*ri).score = (*ri).score0;
        (*ri).hash = (*z.offset(i as isize)).x as uint32_t;
        (*ri).cnt = (*z.offset(i as isize)).y as int32_t;
        (*ri).as_0 = ((*z.offset(i as isize)).y >> 32 as libc::c_int) as int32_t;
        (*ri).div = -1.0f32;
        mm_reg_set_coor(ri, qlen, a, is_qstrand);
        i += 1;
        i;
    }
    kfree(km, z as *mut libc::c_void);
    return r;
}
pub unsafe extern "C" fn mm_mark_alt(
    mut mi: *const mm_idx_t,
    mut n: libc::c_int,
    mut r: *mut mm_reg1_t,
) {
    let mut i: libc::c_int = 0;
    if (*mi).n_alt == 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < n {
        if (*((*mi).seq).offset((*r.offset(i as isize)).rid as isize)).is_alt != 0 {
            let ref mut fresh0 = *r.offset(i as isize);
            (*fresh0).set_is_alt(1 as libc::c_int as uint32_t);
        }
        i += 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn mm_alt_score(
    mut score: libc::c_int,
    mut alt_diff_frac: libc::c_float,
) -> libc::c_int {
    if score < 0 as libc::c_int {
        return score;
    }
    score = (score as libc::c_double * (1.0f64 - alt_diff_frac as libc::c_double)
        + 0.499f64) as libc::c_int;
    return if score > 0 as libc::c_int { score } else { 1 as libc::c_int };
}
pub unsafe extern "C" fn mm_split_reg(
    mut r: *mut mm_reg1_t,
    mut r2: *mut mm_reg1_t,
    mut n: libc::c_int,
    mut qlen: libc::c_int,
    mut a: *mut mm128_t,
    mut is_qstrand: libc::c_int,
) {
    if n <= 0 as libc::c_int || n >= (*r).cnt {
        return;
    }
    *r2 = *r;
    (*r2).id = -(1 as libc::c_int);
    (*r2).set_sam_pri(0 as libc::c_int as uint32_t);
    (*r2).p = 0 as *mut mm_extra_t;
    (*r2).set_split_inv(0 as libc::c_int as uint32_t);
    (*r2).cnt = (*r).cnt - n;
    (*r2)
        .score = (((*r).score as libc::c_float
        * ((*r2).cnt as libc::c_float / (*r).cnt as libc::c_float)) as libc::c_double
        + 0.499f64) as int32_t;
    (*r2).as_0 = (*r).as_0 + n;
    if (*r).parent == (*r).id {
        (*r2).parent = -(2 as libc::c_int);
    }
    mm_reg_set_coor(r2, qlen, a, is_qstrand);
    (*r).cnt -= (*r2).cnt;
    (*r).score -= (*r2).score;
    mm_reg_set_coor(r, qlen, a, is_qstrand);
    (*r).set_split((*r).split() | 1 as libc::c_int as uint32_t);
    (*r2).set_split((*r2).split() | 2 as libc::c_int as uint32_t);
}
pub unsafe extern "C" fn mm_set_parent(
    mut km: *mut libc::c_void,
    mut mask_level: libc::c_float,
    mut mask_len: libc::c_int,
    mut n: libc::c_int,
    mut r: *mut mm_reg1_t,
    mut sub_diff: libc::c_int,
    mut hard_mask_level: libc::c_int,
    mut alt_diff_frac: libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut cov: *mut uint64_t = 0 as *mut uint64_t;
    if n <= 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < n {
        (*r.offset(i as isize)).id = i;
        i += 1;
        i;
    }
    cov = kmalloc(
        km,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong),
    ) as *mut uint64_t;
    w = kmalloc(
        km,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    *w.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    (*r.offset(0 as libc::c_int as isize)).parent = 0 as libc::c_int;
    i = 1 as libc::c_int;
    k = 1 as libc::c_int;
    while i < n {
        let mut current_block_49: u64;
        let mut ri: *mut mm_reg1_t = &mut *r.offset(i as isize) as *mut mm_reg1_t;
        let mut si: libc::c_int = (*ri).qs;
        let mut ei: libc::c_int = (*ri).qe;
        let mut n_cov: libc::c_int = 0 as libc::c_int;
        let mut uncov_len: libc::c_int = 0 as libc::c_int;
        if hard_mask_level != 0 {
            current_block_49 = 3039210444859570989;
        } else {
            j = 0 as libc::c_int;
            while j < k {
                let mut rp: *mut mm_reg1_t = &mut *r
                    .offset(*w.offset(j as isize) as isize) as *mut mm_reg1_t;
                let mut sj: libc::c_int = (*rp).qs;
                let mut ej: libc::c_int = (*rp).qe;
                if !(ej <= si || sj >= ei) {
                    if sj < si {
                        sj = si;
                    }
                    if ej > ei {
                        ej = ei;
                    }
                    let fresh1 = n_cov;
                    n_cov = n_cov + 1;
                    *cov
                        .offset(
                            fresh1 as isize,
                        ) = (sj as uint64_t) << 32 as libc::c_int | ej as libc::c_ulong;
                }
                j += 1;
                j;
            }
            if n_cov == 0 as libc::c_int {
                current_block_49 = 11071620150904311981;
            } else {
                if n_cov > 0 as libc::c_int {
                    let mut j_0: libc::c_int = 0;
                    let mut x: libc::c_int = si;
                    radix_sort_64(cov, cov.offset(n_cov as isize));
                    j_0 = 0 as libc::c_int;
                    while j_0 < n_cov {
                        if (*cov.offset(j_0 as isize) >> 32 as libc::c_int)
                            as libc::c_int > x
                        {
                            uncov_len = (uncov_len as libc::c_ulong)
                                .wrapping_add(
                                    (*cov.offset(j_0 as isize) >> 32 as libc::c_int)
                                        .wrapping_sub(x as libc::c_ulong),
                                ) as libc::c_int as libc::c_int;
                        }
                        x = if *cov.offset(j_0 as isize) as int32_t > x {
                            *cov.offset(j_0 as isize) as int32_t
                        } else {
                            x
                        };
                        j_0 += 1;
                        j_0;
                    }
                    if ei > x {
                        uncov_len += ei - x;
                    }
                }
                current_block_49 = 3039210444859570989;
            }
        }
        match current_block_49 {
            3039210444859570989 => {
                j = 0 as libc::c_int;
                while j < k {
                    let mut rp_0: *mut mm_reg1_t = &mut *r
                        .offset(*w.offset(j as isize) as isize) as *mut mm_reg1_t;
                    let mut sj_0: libc::c_int = (*rp_0).qs;
                    let mut ej_0: libc::c_int = (*rp_0).qe;
                    let mut min: libc::c_int = 0;
                    let mut max: libc::c_int = 0;
                    let mut ol: libc::c_int = 0;
                    if !(ej_0 <= si || sj_0 >= ei) {
                        min = if ej_0 - sj_0 < ei - si { ej_0 - sj_0 } else { ei - si };
                        max = if ej_0 - sj_0 > ei - si { ej_0 - sj_0 } else { ei - si };
                        ol = if si < sj_0 {
                            if ei < sj_0 {
                                0 as libc::c_int
                            } else if ei < ej_0 {
                                ei - sj_0
                            } else {
                                ej_0 - sj_0
                            }
                        } else if ej_0 < si {
                            0 as libc::c_int
                        } else if ej_0 < ei {
                            ej_0 - si
                        } else {
                            ei - si
                        };
                        if ol as libc::c_float / min as libc::c_float
                            - uncov_len as libc::c_float / max as libc::c_float
                            > mask_level && uncov_len <= mask_len
                        {
                            let mut cnt_sub: libc::c_int = 0 as libc::c_int;
                            let mut sci: libc::c_int = (*ri).score;
                            (*ri).parent = (*rp_0).parent;
                            if (*rp_0).is_alt() == 0
                                && (*ri).is_alt() as libc::c_int != 0
                            {
                                sci = mm_alt_score(sci, alt_diff_frac);
                            }
                            (*rp_0)
                                .subsc = if (*rp_0).subsc > sci {
                                (*rp_0).subsc
                            } else {
                                sci
                            };
                            if (*ri).cnt >= (*rp_0).cnt {
                                cnt_sub = 1 as libc::c_int;
                            }
                            if !((*rp_0).p).is_null() && !((*ri).p).is_null()
                                && ((*rp_0).rid != (*ri).rid || (*rp_0).rs != (*ri).rs
                                    || (*rp_0).re != (*ri).re || ol != min)
                            {
                                sci = (*(*ri).p).dp_max;
                                if (*rp_0).is_alt() == 0
                                    && (*ri).is_alt() as libc::c_int != 0
                                {
                                    sci = mm_alt_score(sci, alt_diff_frac);
                                }
                                (*(*rp_0).p)
                                    .dp_max2 = if (*(*rp_0).p).dp_max2 > sci {
                                    (*(*rp_0).p).dp_max2
                                } else {
                                    sci
                                };
                                if (*(*rp_0).p).dp_max - (*(*ri).p).dp_max <= sub_diff {
                                    cnt_sub = 1 as libc::c_int;
                                }
                            }
                            if cnt_sub != 0 {
                                (*rp_0).n_sub += 1;
                                (*rp_0).n_sub;
                            }
                            break;
                        }
                    }
                    j += 1;
                    j;
                }
            }
            _ => {}
        }
        if j == k {
            let fresh2 = k;
            k = k + 1;
            *w.offset(fresh2 as isize) = i;
            (*ri).parent = i;
            (*ri).n_sub = 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    kfree(km, cov as *mut libc::c_void);
    kfree(km, w as *mut libc::c_void);
}
pub unsafe extern "C" fn mm_hit_sort(
    mut km: *mut libc::c_void,
    mut n_regs: *mut libc::c_int,
    mut r: *mut mm_reg1_t,
    mut alt_diff_frac: libc::c_float,
) {
    let mut i: int32_t = 0;
    let mut n_aux: int32_t = 0;
    let mut n: int32_t = *n_regs;
    let mut has_cigar: int32_t = 0 as libc::c_int;
    let mut no_cigar: int32_t = 0 as libc::c_int;
    let mut aux: *mut mm128_t = 0 as *mut mm128_t;
    let mut t: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    if n <= 1 as libc::c_int {
        return;
    }
    aux = kmalloc(km, (n * 16 as libc::c_int) as size_t) as *mut mm128_t;
    t = kmalloc(
        km,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mm_reg1_t>() as libc::c_ulong),
    ) as *mut mm_reg1_t;
    n_aux = 0 as libc::c_int;
    i = n_aux;
    while i < n {
        if (*r.offset(i as isize)).inv() as libc::c_int != 0
            || (*r.offset(i as isize)).cnt > 0 as libc::c_int
        {
            let mut score: libc::c_int = 0;
            if !((*r.offset(i as isize)).p).is_null() {
                score = (*(*r.offset(i as isize)).p).dp_max;
                has_cigar = 1 as libc::c_int;
            } else {
                score = (*r.offset(i as isize)).score;
                no_cigar = 1 as libc::c_int;
            }
            if (*r.offset(i as isize)).is_alt() != 0 {
                score = mm_alt_score(score, alt_diff_frac);
            }
            (*aux.offset(n_aux as isize))
                .x = (score as uint64_t) << 32 as libc::c_int
                | (*r.offset(i as isize)).hash as libc::c_ulong;
            let fresh3 = n_aux;
            n_aux = n_aux + 1;
            (*aux.offset(fresh3 as isize)).y = i as uint64_t;
        } else if !((*r.offset(i as isize)).p).is_null() {
            free((*r.offset(i as isize)).p as *mut libc::c_void);
            let ref mut fresh4 = (*r.offset(i as isize)).p;
            *fresh4 = 0 as *mut mm_extra_t;
        }
        i += 1;
        i;
    }
    if has_cigar + no_cigar == 1 as libc::c_int {} else {
        __assert_fail(
            b"has_cigar + no_cigar == 1\0" as *const u8 as *const libc::c_char,
            b"hit.c\0" as *const u8 as *const libc::c_char,
            210 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"void mm_hit_sort(void *, int *, mm_reg1_t *, float)\0"))
                .as_ptr(),
        );
    }
    'c_7769: {
        if has_cigar + no_cigar == 1 as libc::c_int {} else {
            __assert_fail(
                b"has_cigar + no_cigar == 1\0" as *const u8 as *const libc::c_char,
                b"hit.c\0" as *const u8 as *const libc::c_char,
                210 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"void mm_hit_sort(void *, int *, mm_reg1_t *, float)\0"))
                    .as_ptr(),
            );
        }
    };
    radix_sort_128x(aux, aux.offset(n_aux as isize));
    i = n_aux - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        *t
            .offset(
                (n_aux - 1 as libc::c_int - i) as isize,
            ) = *r.offset((*aux.offset(i as isize)).y as isize);
        i -= 1;
        i;
    }
    memcpy(
        r as *mut libc::c_void,
        t as *const libc::c_void,
        (::std::mem::size_of::<mm_reg1_t>() as libc::c_ulong)
            .wrapping_mul(n_aux as libc::c_ulong),
    );
    *n_regs = n_aux;
    kfree(km, aux as *mut libc::c_void);
    kfree(km, t as *mut libc::c_void);
}
pub unsafe extern "C" fn mm_set_sam_pri(
    mut n: libc::c_int,
    mut r: *mut mm_reg1_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n_pri: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        if (*r.offset(i as isize)).id == (*r.offset(i as isize)).parent {
            n_pri += 1;
            n_pri;
            let ref mut fresh5 = *r.offset(i as isize);
            (*fresh5)
                .set_sam_pri((n_pri == 1 as libc::c_int) as libc::c_int as uint32_t);
        } else {
            let ref mut fresh6 = *r.offset(i as isize);
            (*fresh6).set_sam_pri(0 as libc::c_int as uint32_t);
        }
        i += 1;
        i;
    }
    return n_pri;
}
pub unsafe extern "C" fn mm_sync_regs(
    mut km: *mut libc::c_void,
    mut n_regs: libc::c_int,
    mut regs: *mut mm_reg1_t,
) {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut max_id: libc::c_int = -(1 as libc::c_int);
    let mut n_tmp: libc::c_int = 0;
    if n_regs <= 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < n_regs {
        max_id = if max_id > (*regs.offset(i as isize)).id {
            max_id
        } else {
            (*regs.offset(i as isize)).id
        };
        i += 1;
        i;
    }
    n_tmp = max_id + 1 as libc::c_int;
    tmp = kmalloc(
        km,
        (n_tmp as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < n_tmp {
        *tmp.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < n_regs {
        if (*regs.offset(i as isize)).id >= 0 as libc::c_int {
            *tmp.offset((*regs.offset(i as isize)).id as isize) = i;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < n_regs {
        let mut r: *mut mm_reg1_t = &mut *regs.offset(i as isize) as *mut mm_reg1_t;
        (*r).id = i;
        if (*r).parent == -(2 as libc::c_int) {
            (*r).parent = i;
        } else if (*r).parent >= 0 as libc::c_int
            && *tmp.offset((*r).parent as isize) >= 0 as libc::c_int
        {
            (*r).parent = *tmp.offset((*r).parent as isize);
        } else {
            (*r).parent = -(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    kfree(km, tmp as *mut libc::c_void);
    mm_set_sam_pri(n_regs, regs);
}
pub unsafe extern "C" fn mm_select_sub(
    mut km: *mut libc::c_void,
    mut pri_ratio: libc::c_float,
    mut min_diff: libc::c_int,
    mut best_n: libc::c_int,
    mut check_strand: libc::c_int,
    mut min_strand_sc: libc::c_int,
    mut n_: *mut libc::c_int,
    mut r: *mut mm_reg1_t,
) {
    if pri_ratio > 0.0f32 && *n_ > 0 as libc::c_int {
        let mut i: libc::c_int = 0;
        let mut k: libc::c_int = 0;
        let mut n: libc::c_int = *n_;
        let mut n_2nd: libc::c_int = 0 as libc::c_int;
        k = 0 as libc::c_int;
        i = k;
        while i < n {
            let mut p: libc::c_int = (*r.offset(i as isize)).parent;
            if p == i || (*r.offset(i as isize)).inv() as libc::c_int != 0 {
                let fresh7 = k;
                k = k + 1;
                *r.offset(fresh7 as isize) = *r.offset(i as isize);
            } else if ((*r.offset(i as isize)).score as libc::c_float
                >= (*r.offset(p as isize)).score as libc::c_float * pri_ratio
                || (*r.offset(i as isize)).score + min_diff
                    >= (*r.offset(p as isize)).score) && n_2nd < best_n
            {
                if !((*r.offset(i as isize)).qs == (*r.offset(p as isize)).qs
                    && (*r.offset(i as isize)).qe == (*r.offset(p as isize)).qe
                    && (*r.offset(i as isize)).rid == (*r.offset(p as isize)).rid
                    && (*r.offset(i as isize)).rs == (*r.offset(p as isize)).rs
                    && (*r.offset(i as isize)).re == (*r.offset(p as isize)).re)
                {
                    let fresh8 = k;
                    k = k + 1;
                    *r.offset(fresh8 as isize) = *r.offset(i as isize);
                    n_2nd += 1;
                    n_2nd;
                } else if !((*r.offset(i as isize)).p).is_null() {
                    free((*r.offset(i as isize)).p as *mut libc::c_void);
                }
            } else if check_strand != 0 && n_2nd < best_n
                && (*r.offset(i as isize)).score > min_strand_sc
                && (*r.offset(i as isize)).rev() as libc::c_int
                    != (*r.offset(p as isize)).rev() as libc::c_int
            {
                let ref mut fresh9 = *r.offset(i as isize);
                (*fresh9).set_strand_retained(1 as libc::c_int as uint32_t);
                let fresh10 = k;
                k = k + 1;
                *r.offset(fresh10 as isize) = *r.offset(i as isize);
                n_2nd += 1;
                n_2nd;
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
pub unsafe extern "C" fn mm_filter_strand_retained(
    mut n_regs: libc::c_int,
    mut r: *mut mm_reg1_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    k = 0 as libc::c_int;
    i = k;
    while i < n_regs {
        let mut p: libc::c_int = (*r.offset(i as isize)).parent;
        if (*r.offset(i as isize)).strand_retained() == 0
            || (*r.offset(i as isize)).div < (*r.offset(p as isize)).div * 5.0f32
            || (*r.offset(i as isize)).div < 0.01f32
        {
            if k < i {
                let fresh11 = k;
                k = k + 1;
                *r.offset(fresh11 as isize) = *r.offset(i as isize);
            } else {
                k += 1;
                k;
            }
        }
        i += 1;
        i;
    }
    return k;
}
pub unsafe extern "C" fn mm_filter_regs(
    mut opt: *const mm_mapopt_t,
    mut qlen: libc::c_int,
    mut n_regs: *mut libc::c_int,
    mut regs: *mut mm_reg1_t,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    k = 0 as libc::c_int;
    i = k;
    while i < *n_regs {
        let mut r: *mut mm_reg1_t = &mut *regs.offset(i as isize) as *mut mm_reg1_t;
        let mut flt: libc::c_int = 0 as libc::c_int;
        if (*r).inv() == 0 && (*r).seg_split() == 0 && (*r).cnt < (*opt).min_cnt {
            flt = 1 as libc::c_int;
        }
        if !((*r).p).is_null() {
            if (*r).mlen < (*opt).min_chain_score {
                flt = 1 as libc::c_int;
            } else if (*(*r).p).dp_max < (*opt).min_dp_max {
                flt = 1 as libc::c_int;
            } else if (*r).qs as libc::c_float
                > qlen as libc::c_float * (*opt).max_clip_ratio
                && (qlen - (*r).qe) as libc::c_float
                    > qlen as libc::c_float * (*opt).max_clip_ratio
            {
                flt = 1 as libc::c_int;
            }
            if flt != 0 {
                free((*r).p as *mut libc::c_void);
            }
        }
        if flt == 0 {
            if k < i {
                let fresh12 = k;
                k = k + 1;
                *regs.offset(fresh12 as isize) = *regs.offset(i as isize);
            } else {
                k += 1;
                k;
            }
        }
        i += 1;
        i;
    }
    *n_regs = k;
}
pub unsafe extern "C" fn mm_squeeze_a(
    mut km: *mut libc::c_void,
    mut n_regs: libc::c_int,
    mut regs: *mut mm_reg1_t,
    mut a: *mut mm128_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut as_0: libc::c_int = 0 as libc::c_int;
    let mut aux: *mut uint64_t = 0 as *mut uint64_t;
    aux = kmalloc(km, (n_regs * 8 as libc::c_int) as size_t) as *mut uint64_t;
    i = 0 as libc::c_int;
    while i < n_regs {
        *aux
            .offset(
                i as isize,
            ) = ((*regs.offset(i as isize)).as_0 as uint64_t) << 32 as libc::c_int
            | i as libc::c_ulong;
        i += 1;
        i;
    }
    radix_sort_64(aux, aux.offset(n_regs as isize));
    i = 0 as libc::c_int;
    while i < n_regs {
        let mut r: *mut mm_reg1_t = &mut *regs
            .offset(*aux.offset(i as isize) as int32_t as isize) as *mut mm_reg1_t;
        if (*r).as_0 != as_0 {
            memmove(
                &mut *a.offset(as_0 as isize) as *mut mm128_t as *mut libc::c_void,
                &mut *a.offset((*r).as_0 as isize) as *mut mm128_t
                    as *const libc::c_void,
                ((*r).cnt * 16 as libc::c_int) as libc::c_ulong,
            );
            (*r).as_0 = as_0;
        }
        as_0 += (*r).cnt;
        i += 1;
        i;
    }
    kfree(km, aux as *mut libc::c_void);
    return as_0;
}
pub unsafe extern "C" fn mm_seg_gen(
    mut km: *mut libc::c_void,
    mut hash: uint32_t,
    mut n_segs: libc::c_int,
    mut qlens: *const libc::c_int,
    mut n_regs0: libc::c_int,
    mut regs0: *const mm_reg1_t,
    mut n_regs: *mut libc::c_int,
    mut regs: *mut *mut mm_reg1_t,
    mut a: *const mm128_t,
) -> *mut mm_seg_t {
    let mut s: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut acc_qlen: [libc::c_int; 256] = [0; 256];
    let mut qlen_sum: libc::c_int = 0 as libc::c_int;
    let mut seg: *mut mm_seg_t = 0 as *mut mm_seg_t;
    if n_segs <= 255 as libc::c_int {} else {
        __assert_fail(
            b"n_segs <= MM_MAX_SEG\0" as *const u8 as *const libc::c_char,
            b"hit.c\0" as *const u8 as *const libc::c_char,
            336 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 119],
                &[libc::c_char; 119],
            >(
                b"mm_seg_t *mm_seg_gen(void *, uint32_t, int, const int *, int, const mm_reg1_t *, int *, mm_reg1_t **, const mm128_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_9403: {
        if n_segs <= 255 as libc::c_int {} else {
            __assert_fail(
                b"n_segs <= MM_MAX_SEG\0" as *const u8 as *const libc::c_char,
                b"hit.c\0" as *const u8 as *const libc::c_char,
                336 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 119],
                    &[libc::c_char; 119],
                >(
                    b"mm_seg_t *mm_seg_gen(void *, uint32_t, int, const int *, int, const mm_reg1_t *, int *, mm_reg1_t **, const mm128_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    s = 1 as libc::c_int;
    acc_qlen[0 as libc::c_int as usize] = 0 as libc::c_int;
    while s < n_segs {
        acc_qlen[s
            as usize] = acc_qlen[(s - 1 as libc::c_int) as usize]
            + *qlens.offset((s - 1 as libc::c_int) as isize);
        s += 1;
        s;
    }
    qlen_sum = acc_qlen[(n_segs - 1 as libc::c_int) as usize]
        + *qlens.offset((n_segs - 1 as libc::c_int) as isize);
    seg = kcalloc(
        km,
        n_segs as size_t,
        ::std::mem::size_of::<mm_seg_t>() as libc::c_ulong,
    ) as *mut mm_seg_t;
    s = 0 as libc::c_int;
    while s < n_segs {
        let ref mut fresh13 = (*seg.offset(s as isize)).u;
        *fresh13 = kmalloc(km, (n_regs0 * 8 as libc::c_int) as size_t) as *mut uint64_t;
        i = 0 as libc::c_int;
        while i < n_regs0 {
            *((*seg.offset(s as isize)).u)
                .offset(
                    i as isize,
                ) = ((*regs0.offset(i as isize)).score as uint64_t) << 32 as libc::c_int;
            i += 1;
            i;
        }
        s += 1;
        s;
    }
    i = 0 as libc::c_int;
    while i < n_regs0 {
        let mut r: *const mm_reg1_t = &*regs0.offset(i as isize) as *const mm_reg1_t;
        j = 0 as libc::c_int;
        while j < (*r).cnt {
            let mut sid: libc::c_int = (((*a.offset(((*r).as_0 + j) as isize)).y
                as libc::c_ulonglong & (0xff as libc::c_ulonglong) << 48 as libc::c_int)
                >> 48 as libc::c_int) as libc::c_int;
            let ref mut fresh14 = *((*seg.offset(sid as isize)).u).offset(i as isize);
            *fresh14 = (*fresh14).wrapping_add(1);
            *fresh14;
            let ref mut fresh15 = (*seg.offset(sid as isize)).n_a;
            *fresh15 += 1;
            *fresh15;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    s = 0 as libc::c_int;
    while s < n_segs {
        let mut sr: *mut mm_seg_t = &mut *seg.offset(s as isize) as *mut mm_seg_t;
        i = 0 as libc::c_int;
        (*sr).n_u = 0 as libc::c_int;
        while i < n_regs0 {
            if *((*sr).u).offset(i as isize) as int32_t != 0 as libc::c_int {
                let fresh16 = (*sr).n_u;
                (*sr).n_u = (*sr).n_u + 1;
                *((*sr).u).offset(fresh16 as isize) = *((*sr).u).offset(i as isize);
            }
            i += 1;
            i;
        }
        (*sr)
            .a = kmalloc(
            km,
            ((*sr).n_a as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<mm128_t>() as libc::c_ulong),
        ) as *mut mm128_t;
        (*sr).n_a = 0 as libc::c_int;
        s += 1;
        s;
    }
    i = 0 as libc::c_int;
    while i < n_regs0 {
        let mut r_0: *const mm_reg1_t = &*regs0.offset(i as isize) as *const mm_reg1_t;
        j = 0 as libc::c_int;
        while j < (*r_0).cnt {
            let mut sid_0: libc::c_int = (((*a.offset(((*r_0).as_0 + j) as isize)).y
                as libc::c_ulonglong & (0xff as libc::c_ulonglong) << 48 as libc::c_int)
                >> 48 as libc::c_int) as libc::c_int;
            let mut a1: mm128_t = *a.offset(((*r_0).as_0 + j) as isize);
            a1
                .y = (a1.y as libc::c_ulong)
                .wrapping_sub(
                    (if a1.x >> 63 as libc::c_int != 0 {
                        qlen_sum
                            - (*qlens.offset(sid_0 as isize) + acc_qlen[sid_0 as usize])
                    } else {
                        acc_qlen[sid_0 as usize]
                    }) as libc::c_ulong,
                ) as uint64_t as uint64_t;
            let ref mut fresh17 = (*seg.offset(sid_0 as isize)).n_a;
            let fresh18 = *fresh17;
            *fresh17 = *fresh17 + 1;
            *((*seg.offset(sid_0 as isize)).a).offset(fresh18 as isize) = a1;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    s = 0 as libc::c_int;
    while s < n_segs {
        let ref mut fresh19 = *regs.offset(s as isize);
        *fresh19 = mm_gen_regs(
            km,
            hash,
            *qlens.offset(s as isize),
            (*seg.offset(s as isize)).n_u,
            (*seg.offset(s as isize)).u,
            (*seg.offset(s as isize)).a,
            0 as libc::c_int,
        );
        *n_regs.offset(s as isize) = (*seg.offset(s as isize)).n_u;
        i = 0 as libc::c_int;
        while i < *n_regs.offset(s as isize) {
            let ref mut fresh20 = *(*regs.offset(s as isize)).offset(i as isize);
            (*fresh20).set_seg_split(1 as libc::c_int as uint32_t);
            let ref mut fresh21 = *(*regs.offset(s as isize)).offset(i as isize);
            (*fresh21).set_seg_id(s as uint32_t);
            i += 1;
            i;
        }
        s += 1;
        s;
    }
    return seg;
}
pub unsafe extern "C" fn mm_seg_free(
    mut km: *mut libc::c_void,
    mut n_segs: libc::c_int,
    mut segs: *mut mm_seg_t,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n_segs {
        kfree(km, (*segs.offset(i as isize)).u as *mut libc::c_void);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < n_segs {
        kfree(km, (*segs.offset(i as isize)).a as *mut libc::c_void);
        i += 1;
        i;
    }
    kfree(km, segs as *mut libc::c_void);
}
unsafe extern "C" fn mm_set_inv_mapq(
    mut km: *mut libc::c_void,
    mut n_regs: libc::c_int,
    mut regs: *mut mm_reg1_t,
) {
    let mut i: libc::c_int = 0;
    let mut n_aux: libc::c_int = 0;
    let mut aux: *mut mm128_t = 0 as *mut mm128_t;
    if n_regs < 3 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < n_regs {
        if (*regs.offset(i as isize)).inv() != 0 {
            break;
        }
        i += 1;
        i;
    }
    if i == n_regs {
        return;
    }
    aux = kmalloc(km, (n_regs * 16 as libc::c_int) as size_t) as *mut mm128_t;
    n_aux = 0 as libc::c_int;
    i = n_aux;
    while i < n_regs {
        if (*regs.offset(i as isize)).parent == i
            || (*regs.offset(i as isize)).parent < 0 as libc::c_int
        {
            (*aux.offset(n_aux as isize)).y = i as uint64_t;
            let fresh22 = n_aux;
            n_aux = n_aux + 1;
            (*aux.offset(fresh22 as isize))
                .x = ((*regs.offset(i as isize)).rid as uint64_t) << 32 as libc::c_int
                | (*regs.offset(i as isize)).rs as libc::c_ulong;
        }
        i += 1;
        i;
    }
    radix_sort_128x(aux, aux.offset(n_aux as isize));
    i = 1 as libc::c_int;
    while i < n_aux - 1 as libc::c_int {
        let mut inv: *mut mm_reg1_t = &mut *regs
            .offset((*aux.offset(i as isize)).y as isize) as *mut mm_reg1_t;
        if (*inv).inv() != 0 {
            let mut l: *mut mm_reg1_t = &mut *regs
                .offset((*aux.offset((i - 1 as libc::c_int) as isize)).y as isize)
                as *mut mm_reg1_t;
            let mut r: *mut mm_reg1_t = &mut *regs
                .offset((*aux.offset((i + 1 as libc::c_int) as isize)).y as isize)
                as *mut mm_reg1_t;
            (*inv)
                .set_mapq(
                    (if ((*l).mapq() as libc::c_int) < (*r).mapq() as libc::c_int {
                        (*l).mapq() as libc::c_int
                    } else {
                        (*r).mapq() as libc::c_int
                    }) as uint32_t,
                );
        }
        i += 1;
        i;
    }
    kfree(km, aux as *mut libc::c_void);
}
pub unsafe extern "C" fn mm_set_mapq(
    mut km: *mut libc::c_void,
    mut n_regs: libc::c_int,
    mut regs: *mut mm_reg1_t,
    mut min_chain_sc: libc::c_int,
    mut match_sc: libc::c_int,
    mut rep_len: libc::c_int,
    mut is_sr: libc::c_int,
) {
    static mut q_coef: libc::c_float = 40.0f32;
    let mut sum_sc: int64_t = 0 as libc::c_int as int64_t;
    let mut uniq_ratio: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    if n_regs == 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < n_regs {
        if (*regs.offset(i as isize)).parent == (*regs.offset(i as isize)).id {
            sum_sc += (*regs.offset(i as isize)).score as libc::c_long;
        }
        i += 1;
        i;
    }
    uniq_ratio = sum_sc as libc::c_float
        / (sum_sc + rep_len as libc::c_long) as libc::c_float;
    i = 0 as libc::c_int;
    while i < n_regs {
        let mut r: *mut mm_reg1_t = &mut *regs.offset(i as isize) as *mut mm_reg1_t;
        if (*r).inv() != 0 {
            (*r).set_mapq(0 as libc::c_int as uint32_t);
        } else if (*r).parent == (*r).id {
            let mut mapq: libc::c_int = 0;
            let mut subsc: libc::c_int = 0;
            let mut pen_s1: libc::c_float = (if (*r).score > 100 as libc::c_int {
                1.0f32
            } else {
                0.01f32 * (*r).score as libc::c_float
            }) * uniq_ratio;
            let mut pen_cm: libc::c_float = if (*r).cnt > 10 as libc::c_int {
                1.0f32
            } else {
                0.1f32 * (*r).cnt as libc::c_float
            };
            pen_cm = if pen_s1 < pen_cm { pen_s1 } else { pen_cm };
            subsc = if (*r).subsc > min_chain_sc { (*r).subsc } else { min_chain_sc };
            if !((*r).p).is_null() && (*(*r).p).dp_max2 > 0 as libc::c_int
                && (*(*r).p).dp_max > 0 as libc::c_int
            {
                let mut identity: libc::c_float = (*r).mlen as libc::c_float
                    / (*r).blen as libc::c_float;
                let mut x: libc::c_float = (*(*r).p).dp_max2 as libc::c_float
                    * subsc as libc::c_float / (*(*r).p).dp_max as libc::c_float
                    / (*r).score0 as libc::c_float;
                mapq = (identity * pen_cm * q_coef * (1.0f32 - x * x)
                    * logf(
                        (*(*r).p).dp_max as libc::c_float / match_sc as libc::c_float,
                    )) as libc::c_int;
                if is_sr == 0 {
                    let mut mapq_alt: libc::c_int = (6.02f32 * identity * identity
                        * ((*(*r).p).dp_max - (*(*r).p).dp_max2) as libc::c_float
                        / match_sc as libc::c_float + 0.499f32) as libc::c_int;
                    mapq = if mapq < mapq_alt { mapq } else { mapq_alt };
                }
            } else {
                let mut x_0: libc::c_float = subsc as libc::c_float
                    / (*r).score0 as libc::c_float;
                if !((*r).p).is_null() {
                    let mut identity_0: libc::c_float = (*r).mlen as libc::c_float
                        / (*r).blen as libc::c_float;
                    mapq = (identity_0 * pen_cm * q_coef * (1.0f32 - x_0)
                        * logf(
                            (*(*r).p).dp_max as libc::c_float / match_sc as libc::c_float,
                        )) as libc::c_int;
                } else {
                    mapq = (pen_cm * q_coef * (1.0f32 - x_0)
                        * logf((*r).score as libc::c_float)) as libc::c_int;
                }
            }
            mapq
                -= (4.343f32 * logf(((*r).n_sub + 1 as libc::c_int) as libc::c_float)
                    + 0.499f32) as libc::c_int;
            mapq = if mapq > 0 as libc::c_int { mapq } else { 0 as libc::c_int };
            (*r)
                .set_mapq(
                    (if mapq < 60 as libc::c_int { mapq } else { 60 as libc::c_int })
                        as uint32_t,
                );
            if !((*r).p).is_null() && (*(*r).p).dp_max > (*(*r).p).dp_max2
                && (*r).mapq() as libc::c_int == 0 as libc::c_int
            {
                (*r).set_mapq(1 as libc::c_int as uint32_t);
            }
        } else {
            (*r).set_mapq(0 as libc::c_int as uint32_t);
        }
        i += 1;
        i;
    }
    mm_set_inv_mapq(km, n_regs, regs);
}
