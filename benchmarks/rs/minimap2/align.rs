use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type mm_idx_intv_s;
    pub type mm_idx_bucket_s;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abs(_: libc::c_int) -> libc::c_int;
    fn log(_: libc::c_double) -> libc::c_double;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn mm_idx_bed_junc(
        mi: *const mm_idx_t,
        ctg: int32_t,
        st: int32_t,
        en: int32_t,
        s: *mut uint8_t,
    ) -> libc::c_int;
    fn mm_idx_getseq(
        mi: *const mm_idx_t,
        rid: uint32_t,
        st: uint32_t,
        en: uint32_t,
        seq: *mut uint8_t,
    ) -> libc::c_int;
    static mut mm_dbg_flag: libc::c_int;
    static mut seq_nt4_table: [libc::c_uchar; 256];
    fn mm_split_reg(
        r: *mut mm_reg1_t,
        r2: *mut mm_reg1_t,
        n: libc::c_int,
        qlen: libc::c_int,
        a: *mut mm128_t,
        is_qstrand: libc::c_int,
    );
    fn mm_filter_regs(
        opt: *const mm_mapopt_t,
        qlen: libc::c_int,
        n_regs: *mut libc::c_int,
        regs: *mut mm_reg1_t,
    );
    fn mm_hit_sort(
        km: *mut libc::c_void,
        n_regs: *mut libc::c_int,
        r: *mut mm_reg1_t,
        alt_diff_frac: libc::c_float,
    );
    fn mm_idx_getseq2(
        mi: *const mm_idx_t,
        is_rev: libc::c_int,
        rid: uint32_t,
        st: uint32_t,
        en: uint32_t,
        seq: *mut uint8_t,
    ) -> libc::c_int;
    fn mm_squeeze_a(
        km: *mut libc::c_void,
        n_regs: libc::c_int,
        regs: *mut mm_reg1_t,
        a: *mut mm128_t,
    ) -> libc::c_int;
    fn krealloc(
        km: *mut libc::c_void,
        ptr: *mut libc::c_void,
        size: size_t,
    ) -> *mut libc::c_void;
    fn kmalloc(km: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn ksw_ll_qinit(
        km: *mut libc::c_void,
        size: libc::c_int,
        qlen: libc::c_int,
        query: *const uint8_t,
        m: libc::c_int,
        mat: *const int8_t,
    ) -> *mut libc::c_void;
    fn ksw_ll_i16(
        q: *mut libc::c_void,
        tlen: libc::c_int,
        target: *const uint8_t,
        gapo: libc::c_int,
        gape: libc::c_int,
        qe: *mut libc::c_int,
        te: *mut libc::c_int,
    ) -> libc::c_int;
    fn ksw_exts2_sse(
        km: *mut libc::c_void,
        qlen: libc::c_int,
        query: *const uint8_t,
        tlen: libc::c_int,
        target: *const uint8_t,
        m: int8_t,
        mat: *const int8_t,
        gapo: int8_t,
        gape: int8_t,
        gapo2: int8_t,
        noncan: int8_t,
        zdrop: libc::c_int,
        junc_bonus: int8_t,
        flag: libc::c_int,
        junc: *const uint8_t,
        ez: *mut ksw_extz_t,
    );
    fn ksw_extz2_sse(
        km: *mut libc::c_void,
        qlen: libc::c_int,
        query: *const uint8_t,
        tlen: libc::c_int,
        target: *const uint8_t,
        m: int8_t,
        mat: *const int8_t,
        q: int8_t,
        e: int8_t,
        w: libc::c_int,
        zdrop: libc::c_int,
        end_bonus: libc::c_int,
        flag: libc::c_int,
        ez: *mut ksw_extz_t,
    );
    fn ksw_extd2_sse(
        km: *mut libc::c_void,
        qlen: libc::c_int,
        query: *const uint8_t,
        tlen: libc::c_int,
        target: *const uint8_t,
        m: int8_t,
        mat: *const int8_t,
        gapo: int8_t,
        gape: int8_t,
        gapo2: int8_t,
        gape2: int8_t,
        w: libc::c_int,
        zdrop: libc::c_int,
        end_bonus: libc::c_int,
        flag: libc::c_int,
        ez: *mut ksw_extz_t,
    );
    fn kfree(km: *mut libc::c_void, ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub union C2RustUnnamed {
    pub f: libc::c_float,
    pub i: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct ksw_extz_t {
    #[bitfield(name = "max", ty = "uint32_t", bits = "0..=30")]
    #[bitfield(name = "zdropped", ty = "uint32_t", bits = "31..=31")]
    pub max_zdropped: [u8; 4],
    pub max_q: libc::c_int,
    pub max_t: libc::c_int,
    pub mqe: libc::c_int,
    pub mqe_t: libc::c_int,
    pub mte: libc::c_int,
    pub mte_q: libc::c_int,
    pub score: libc::c_int,
    pub m_cigar: libc::c_int,
    pub n_cigar: libc::c_int,
    pub reach_end: libc::c_int,
    pub cigar: *mut uint32_t,
}
#[inline]
unsafe extern "C" fn mg_log2(mut x: libc::c_float) -> libc::c_float {
    let mut z: C2RustUnnamed = C2RustUnnamed { f: x };
    let mut log_2: libc::c_float = (z.i >> 23 as libc::c_int
        & 255 as libc::c_int as libc::c_uint)
        .wrapping_sub(128 as libc::c_int as libc::c_uint) as libc::c_float;
    z.i &= !((255 as libc::c_int) << 23 as libc::c_int) as libc::c_uint;
    z
        .i = (z.i as libc::c_uint)
        .wrapping_add(((127 as libc::c_int) << 23 as libc::c_int) as libc::c_uint)
        as uint32_t as uint32_t;
    log_2 += (-0.34484843f32 * z.f + 2.02466578f32) * z.f - 0.67487759f32;
    return log_2;
}
#[inline]
unsafe extern "C" fn ksw_push_cigar(
    mut km: *mut libc::c_void,
    mut n_cigar: *mut libc::c_int,
    mut m_cigar: *mut libc::c_int,
    mut cigar: *mut uint32_t,
    mut op: uint32_t,
    mut len: libc::c_int,
) -> *mut uint32_t {
    if *n_cigar == 0 as libc::c_int
        || op
            != *cigar.offset((*n_cigar - 1 as libc::c_int) as isize)
                & 0xf as libc::c_int as libc::c_uint
    {
        if *n_cigar == *m_cigar {
            *m_cigar = if *m_cigar != 0 {
                *m_cigar << 1 as libc::c_int
            } else {
                4 as libc::c_int
            };
            cigar = krealloc(
                km,
                cigar as *mut libc::c_void,
                (*m_cigar << 2 as libc::c_int) as size_t,
            ) as *mut uint32_t;
        }
        let fresh0 = *n_cigar;
        *n_cigar = *n_cigar + 1;
        *cigar.offset(fresh0 as isize) = (len << 4 as libc::c_int) as libc::c_uint | op;
    } else {
        let ref mut fresh1 = *cigar.offset((*n_cigar - 1 as libc::c_int) as isize);
        *fresh1 = (*fresh1 as libc::c_uint)
            .wrapping_add((len << 4 as libc::c_int) as libc::c_uint) as uint32_t
            as uint32_t;
    }
    return cigar;
}
#[inline]
unsafe extern "C" fn ksw_reset_extz(mut ez: *mut ksw_extz_t) {
    (*ez).mte_q = -(1 as libc::c_int);
    (*ez).mqe_t = (*ez).mte_q;
    (*ez).max_t = (*ez).mqe_t;
    (*ez).max_q = (*ez).max_t;
    (*ez).set_max(0 as libc::c_int as uint32_t);
    (*ez).mte = -(0x40000000 as libc::c_int);
    (*ez).mqe = (*ez).mte;
    (*ez).score = (*ez).mqe;
    (*ez).n_cigar = 0 as libc::c_int;
    (*ez).set_zdropped(0 as libc::c_int as uint32_t);
    (*ez).reach_end = 0 as libc::c_int;
}
unsafe extern "C" fn ksw_gen_simple_mat(
    mut m: libc::c_int,
    mut mat: *mut int8_t,
    mut a: int8_t,
    mut b: int8_t,
    mut sc_ambi: int8_t,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    a = (if (a as libc::c_int) < 0 as libc::c_int {
        -(a as libc::c_int)
    } else {
        a as libc::c_int
    }) as int8_t;
    b = (if b as libc::c_int > 0 as libc::c_int {
        -(b as libc::c_int)
    } else {
        b as libc::c_int
    }) as int8_t;
    sc_ambi = (if sc_ambi as libc::c_int > 0 as libc::c_int {
        -(sc_ambi as libc::c_int)
    } else {
        sc_ambi as libc::c_int
    }) as int8_t;
    i = 0 as libc::c_int;
    while i < m - 1 as libc::c_int {
        j = 0 as libc::c_int;
        while j < m - 1 as libc::c_int {
            *mat
                .offset(
                    (i * m + j) as isize,
                ) = (if i == j { a as libc::c_int } else { b as libc::c_int }) as int8_t;
            j += 1;
            j;
        }
        *mat.offset((i * m + m - 1 as libc::c_int) as isize) = sc_ambi;
        i += 1;
        i;
    }
    j = 0 as libc::c_int;
    while j < m {
        *mat.offset(((m - 1 as libc::c_int) * m + j) as isize) = sc_ambi;
        j += 1;
        j;
    }
}
#[inline]
unsafe extern "C" fn mm_seq_rev(mut len: uint32_t, mut seq: *mut uint8_t) {
    let mut i: uint32_t = 0;
    let mut t: uint8_t = 0;
    i = 0 as libc::c_int as uint32_t;
    while i < len >> 1 as libc::c_int {
        t = *seq.offset(i as isize);
        *seq
            .offset(
                i as isize,
            ) = *seq
            .offset(
                len.wrapping_sub(1 as libc::c_int as libc::c_uint).wrapping_sub(i)
                    as isize,
            );
        *seq
            .offset(
                len.wrapping_sub(1 as libc::c_int as libc::c_uint).wrapping_sub(i)
                    as isize,
            ) = t;
        i = i.wrapping_add(1);
        i;
    }
}
#[inline]
unsafe extern "C" fn update_max_zdrop(
    mut score: int32_t,
    mut i: libc::c_int,
    mut j: libc::c_int,
    mut max: *mut int32_t,
    mut max_i: *mut libc::c_int,
    mut max_j: *mut libc::c_int,
    mut e: libc::c_int,
    mut max_zdrop: *mut libc::c_int,
    mut pos: *mut [libc::c_int; 2],
) {
    if score < *max {
        let mut li: libc::c_int = i - *max_i;
        let mut lj: libc::c_int = j - *max_j;
        let mut diff: libc::c_int = if li > lj { li - lj } else { lj - li };
        let mut z: libc::c_int = *max - score - diff * e;
        if z > *max_zdrop {
            *max_zdrop = z;
            (*pos.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] = *max_i;
            (*pos.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] = i;
            (*pos.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] = *max_j;
            (*pos.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] = j;
        }
    } else {
        *max = score;
        *max_i = i;
        *max_j = j;
    };
}
unsafe extern "C" fn mm_test_zdrop(
    mut km: *mut libc::c_void,
    mut opt: *const mm_mapopt_t,
    mut qseq: *const uint8_t,
    mut tseq: *const uint8_t,
    mut n_cigar: uint32_t,
    mut cigar: *mut uint32_t,
    mut mat: *const int8_t,
) -> libc::c_int {
    let mut k: uint32_t = 0;
    let mut score: int32_t = 0 as libc::c_int;
    let mut max: int32_t = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    let mut max_i: int32_t = -(1 as libc::c_int);
    let mut max_j: int32_t = -(1 as libc::c_int);
    let mut i: int32_t = 0 as libc::c_int;
    let mut j: int32_t = 0 as libc::c_int;
    let mut max_zdrop: int32_t = 0 as libc::c_int;
    let mut pos: [[libc::c_int; 2]; 2] = [
        [-(1 as libc::c_int), -(1 as libc::c_int)],
        [-(1 as libc::c_int), -(1 as libc::c_int)],
    ];
    let mut q_len: libc::c_int = 0;
    let mut t_len: libc::c_int = 0;
    k = 0 as libc::c_int as uint32_t;
    score = 0 as libc::c_int;
    while k < n_cigar {
        let mut l: uint32_t = 0;
        let mut op: uint32_t = *cigar.offset(k as isize)
            & 0xf as libc::c_int as libc::c_uint;
        let mut len: uint32_t = *cigar.offset(k as isize) >> 4 as libc::c_int;
        if op == 0 as libc::c_int as libc::c_uint {
            l = 0 as libc::c_int as uint32_t;
            while l < len {
                score
                    += *mat
                        .offset(
                            (*tseq.offset((i as libc::c_uint).wrapping_add(l) as isize)
                                as libc::c_int * 5 as libc::c_int
                                + *qseq.offset((j as libc::c_uint).wrapping_add(l) as isize)
                                    as libc::c_int) as isize,
                        ) as libc::c_int;
                update_max_zdrop(
                    score,
                    (i as libc::c_uint).wrapping_add(l) as libc::c_int,
                    (j as libc::c_uint).wrapping_add(l) as libc::c_int,
                    &mut max,
                    &mut max_i,
                    &mut max_j,
                    (*opt).e,
                    &mut max_zdrop,
                    pos.as_mut_ptr(),
                );
                l = l.wrapping_add(1);
                l;
            }
            i = (i as libc::c_uint).wrapping_add(len) as int32_t as int32_t;
            j = (j as libc::c_uint).wrapping_add(len) as int32_t as int32_t;
        } else if op == 1 as libc::c_int as libc::c_uint
            || op == 2 as libc::c_int as libc::c_uint
            || op == 3 as libc::c_int as libc::c_uint
        {
            score = (score as libc::c_uint)
                .wrapping_sub(
                    ((*opt).q as libc::c_uint)
                        .wrapping_add(((*opt).e as libc::c_uint).wrapping_mul(len)),
                ) as int32_t as int32_t;
            if op == 1 as libc::c_int as libc::c_uint {
                j = (j as libc::c_uint).wrapping_add(len) as int32_t as int32_t;
            } else {
                i = (i as libc::c_uint).wrapping_add(len) as int32_t as int32_t;
            }
            update_max_zdrop(
                score,
                i,
                j,
                &mut max,
                &mut max_i,
                &mut max_j,
                (*opt).e,
                &mut max_zdrop,
                pos.as_mut_ptr(),
            );
        }
        k = k.wrapping_add(1);
        k;
    }
    q_len = pos[1 as libc::c_int as usize][1 as libc::c_int as usize]
        - pos[1 as libc::c_int as usize][0 as libc::c_int as usize];
    t_len = pos[0 as libc::c_int as usize][1 as libc::c_int as usize]
        - pos[0 as libc::c_int as usize][0 as libc::c_int as usize];
    if (*opt).flag
        & (0x80 as libc::c_int | 0x1000 as libc::c_int | 0x100000 as libc::c_int
            | 0x200000 as libc::c_int) as libc::c_long == 0
        && max_zdrop > (*opt).zdrop_inv && q_len < (*opt).max_gap
        && t_len < (*opt).max_gap
    {
        let mut qseq2: *mut uint8_t = 0 as *mut uint8_t;
        let mut qp: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut q_off: libc::c_int = 0;
        let mut t_off: libc::c_int = 0;
        qseq2 = kmalloc(km, q_len as size_t) as *mut uint8_t;
        i = 0 as libc::c_int;
        while i < q_len {
            let mut c: libc::c_int = *qseq
                .offset(
                    (pos[1 as libc::c_int as usize][1 as libc::c_int as usize] - i
                        - 1 as libc::c_int) as isize,
                ) as libc::c_int;
            *qseq2
                .offset(
                    i as isize,
                ) = (if c >= 4 as libc::c_int {
                4 as libc::c_int
            } else {
                3 as libc::c_int - c
            }) as uint8_t;
            i += 1;
            i;
        }
        qp = ksw_ll_qinit(km, 2 as libc::c_int, q_len, qseq2, 5 as libc::c_int, mat);
        score = ksw_ll_i16(
            qp,
            t_len,
            tseq
                .offset(
                    pos[0 as libc::c_int as usize][0 as libc::c_int as usize] as isize,
                ),
            (*opt).q,
            (*opt).e,
            &mut q_off,
            &mut t_off,
        );
        kfree(km, qseq2 as *mut libc::c_void);
        kfree(km, qp);
        if score >= (*opt).min_chain_score * (*opt).a && score >= (*opt).min_dp_max {
            return 2 as libc::c_int;
        }
    }
    return if max_zdrop > (*opt).zdrop { 1 as libc::c_int } else { 0 as libc::c_int };
}
unsafe extern "C" fn mm_fix_cigar(
    mut r: *mut mm_reg1_t,
    mut qseq: *const uint8_t,
    mut tseq: *const uint8_t,
    mut qshift: *mut libc::c_int,
    mut tshift: *mut libc::c_int,
) {
    let mut p: *mut mm_extra_t = (*r).p;
    let mut toff: int32_t = 0 as libc::c_int;
    let mut qoff: int32_t = 0 as libc::c_int;
    let mut to_shrink: int32_t = 0 as libc::c_int;
    let mut k: uint32_t = 0;
    *tshift = 0 as libc::c_int;
    *qshift = *tshift;
    if (*p).n_cigar <= 1 as libc::c_int as libc::c_uint {
        return;
    }
    k = 0 as libc::c_int as uint32_t;
    while k < (*p).n_cigar {
        let mut op: uint32_t = *((*p).cigar).as_mut_ptr().offset(k as isize)
            & 0xf as libc::c_int as libc::c_uint;
        let mut len: uint32_t = *((*p).cigar).as_mut_ptr().offset(k as isize)
            >> 4 as libc::c_int;
        if len == 0 as libc::c_int as libc::c_uint {
            to_shrink = 1 as libc::c_int;
        }
        if op == 0 as libc::c_int as libc::c_uint {
            toff = (toff as libc::c_uint).wrapping_add(len) as int32_t as int32_t;
            qoff = (qoff as libc::c_uint).wrapping_add(len) as int32_t as int32_t;
        } else if op == 1 as libc::c_int as libc::c_uint
            || op == 2 as libc::c_int as libc::c_uint
        {
            if k > 0 as libc::c_int as libc::c_uint
                && k < ((*p).n_cigar).wrapping_sub(1 as libc::c_int as libc::c_uint)
                && *((*p).cigar)
                    .as_mut_ptr()
                    .offset(k.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
                    & 0xf as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                && *((*p).cigar)
                    .as_mut_ptr()
                    .offset(k.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
                    & 0xf as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
            {
                let mut l: libc::c_int = 0;
                let mut prev_len: libc::c_int = (*((*p).cigar)
                    .as_mut_ptr()
                    .offset(k.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
                    >> 4 as libc::c_int) as libc::c_int;
                if op == 1 as libc::c_int as libc::c_uint {
                    l = 0 as libc::c_int;
                    while l < prev_len {
                        if *qseq.offset((qoff - 1 as libc::c_int - l) as isize)
                            as libc::c_int
                            != *qseq
                                .offset(
                                    (qoff as libc::c_uint)
                                        .wrapping_add(len)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                        .wrapping_sub(l as libc::c_uint) as isize,
                                ) as libc::c_int
                        {
                            break;
                        }
                        l += 1;
                        l;
                    }
                } else {
                    l = 0 as libc::c_int;
                    while l < prev_len {
                        if *tseq.offset((toff - 1 as libc::c_int - l) as isize)
                            as libc::c_int
                            != *tseq
                                .offset(
                                    (toff as libc::c_uint)
                                        .wrapping_add(len)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                        .wrapping_sub(l as libc::c_uint) as isize,
                                ) as libc::c_int
                        {
                            break;
                        }
                        l += 1;
                        l;
                    }
                }
                if l > 0 as libc::c_int {
                    let ref mut fresh2 = *((*p).cigar)
                        .as_mut_ptr()
                        .offset(
                            k.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        );
                    *fresh2 = (*fresh2 as libc::c_uint)
                        .wrapping_sub((l << 4 as libc::c_int) as libc::c_uint)
                        as uint32_t as uint32_t;
                    let ref mut fresh3 = *((*p).cigar)
                        .as_mut_ptr()
                        .offset(
                            k.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        );
                    *fresh3 = (*fresh3 as libc::c_uint)
                        .wrapping_add((l << 4 as libc::c_int) as libc::c_uint)
                        as uint32_t as uint32_t;
                    qoff -= l;
                    toff -= l;
                }
                if l == prev_len {
                    to_shrink = 1 as libc::c_int;
                }
            }
            if op == 1 as libc::c_int as libc::c_uint {
                qoff = (qoff as libc::c_uint).wrapping_add(len) as int32_t as int32_t;
            } else {
                toff = (toff as libc::c_uint).wrapping_add(len) as int32_t as int32_t;
            }
        } else if op == 3 as libc::c_int as libc::c_uint {
            toff = (toff as libc::c_uint).wrapping_add(len) as int32_t as int32_t;
        }
        k = k.wrapping_add(1);
        k;
    }
    if qoff == (*r).qe - (*r).qs && toff == (*r).re - (*r).rs {} else {
        __assert_fail(
            b"qoff == r->qe - r->qs && toff == r->re - r->rs\0" as *const u8
                as *const libc::c_char,
            b"align.c\0" as *const u8 as *const libc::c_char,
            125 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 79],
                &[libc::c_char; 79],
            >(
                b"void mm_fix_cigar(mm_reg1_t *, const uint8_t *, const uint8_t *, int *, int *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_7659: {
        if qoff == (*r).qe - (*r).qs && toff == (*r).re - (*r).rs {} else {
            __assert_fail(
                b"qoff == r->qe - r->qs && toff == r->re - r->rs\0" as *const u8
                    as *const libc::c_char,
                b"align.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 79],
                    &[libc::c_char; 79],
                >(
                    b"void mm_fix_cigar(mm_reg1_t *, const uint8_t *, const uint8_t *, int *, int *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    k = 0 as libc::c_int as uint32_t;
    while k < ((*p).n_cigar).wrapping_sub(2 as libc::c_int as libc::c_uint) {
        if *((*p).cigar).as_mut_ptr().offset(k as isize)
            & 0xf as libc::c_int as libc::c_uint > 0 as libc::c_int as libc::c_uint
            && (*((*p).cigar).as_mut_ptr().offset(k as isize)
                & 0xf as libc::c_int as libc::c_uint)
                .wrapping_add(
                    *((*p).cigar)
                        .as_mut_ptr()
                        .offset(
                            k.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) & 0xf as libc::c_int as libc::c_uint,
                ) == 3 as libc::c_int as libc::c_uint
        {
            let mut l_0: uint32_t = 0;
            let mut s: [uint32_t; 3] = [
                0 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
            ];
            l_0 = k;
            while l_0 < (*p).n_cigar {
                let mut op_0: uint32_t = *((*p).cigar).as_mut_ptr().offset(l_0 as isize)
                    & 0xf as libc::c_int as libc::c_uint;
                if !(op_0 == 1 as libc::c_int as libc::c_uint
                    || op_0 == 2 as libc::c_int as libc::c_uint
                    || *((*p).cigar).as_mut_ptr().offset(l_0 as isize)
                        >> 4 as libc::c_int == 0 as libc::c_int as libc::c_uint)
                {
                    break;
                }
                s[op_0
                    as usize] = (s[op_0 as usize] as libc::c_uint)
                    .wrapping_add(
                        *((*p).cigar).as_mut_ptr().offset(l_0 as isize)
                            >> 4 as libc::c_int,
                    ) as uint32_t as uint32_t;
                l_0 = l_0.wrapping_add(1);
                l_0;
            }
            if s[1 as libc::c_int as usize] > 0 as libc::c_int as libc::c_uint
                && s[2 as libc::c_int as usize] > 0 as libc::c_int as libc::c_uint
                && l_0.wrapping_sub(k) > 2 as libc::c_int as libc::c_uint
            {
                *((*p).cigar)
                    .as_mut_ptr()
                    .offset(
                        k as isize,
                    ) = s[1 as libc::c_int as usize] << 4 as libc::c_int
                    | 1 as libc::c_int as libc::c_uint;
                *((*p).cigar)
                    .as_mut_ptr()
                    .offset(
                        k.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                    ) = s[2 as libc::c_int as usize] << 4 as libc::c_int
                    | 2 as libc::c_int as libc::c_uint;
                k = (k as libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
                    as uint32_t as uint32_t;
                while k < l_0 {
                    let ref mut fresh4 = *((*p).cigar).as_mut_ptr().offset(k as isize);
                    *fresh4 &= 0xf as libc::c_int as libc::c_uint;
                    k = k.wrapping_add(1);
                    k;
                }
                to_shrink = 1 as libc::c_int;
            }
            k = l_0;
        }
        k = k.wrapping_add(1);
        k;
    }
    if to_shrink != 0 {
        let mut l_1: int32_t = 0 as libc::c_int;
        k = 0 as libc::c_int as uint32_t;
        while k < (*p).n_cigar {
            if *((*p).cigar).as_mut_ptr().offset(k as isize) >> 4 as libc::c_int
                != 0 as libc::c_int as libc::c_uint
            {
                let fresh5 = l_1;
                l_1 = l_1 + 1;
                *((*p).cigar)
                    .as_mut_ptr()
                    .offset(
                        fresh5 as isize,
                    ) = *((*p).cigar).as_mut_ptr().offset(k as isize);
            }
            k = k.wrapping_add(1);
            k;
        }
        (*p).n_cigar = l_1 as uint32_t;
        l_1 = 0 as libc::c_int;
        k = l_1 as uint32_t;
        while k < (*p).n_cigar {
            if k == ((*p).n_cigar).wrapping_sub(1 as libc::c_int as libc::c_uint)
                || *((*p).cigar).as_mut_ptr().offset(k as isize)
                    & 0xf as libc::c_int as libc::c_uint
                    != *((*p).cigar)
                        .as_mut_ptr()
                        .offset(
                            k.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) & 0xf as libc::c_int as libc::c_uint
            {
                let fresh6 = l_1;
                l_1 = l_1 + 1;
                *((*p).cigar)
                    .as_mut_ptr()
                    .offset(
                        fresh6 as isize,
                    ) = *((*p).cigar).as_mut_ptr().offset(k as isize);
            } else {
                let ref mut fresh7 = *((*p).cigar)
                    .as_mut_ptr()
                    .offset(k.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
                *fresh7 = (*fresh7 as libc::c_uint)
                    .wrapping_add(
                        (*((*p).cigar).as_mut_ptr().offset(k as isize)
                            >> 4 as libc::c_int) << 4 as libc::c_int,
                    ) as uint32_t as uint32_t;
            }
            k = k.wrapping_add(1);
            k;
        }
        (*p).n_cigar = l_1 as uint32_t;
    }
    if *((*p).cigar).as_mut_ptr().offset(0 as libc::c_int as isize)
        & 0xf as libc::c_int as libc::c_uint == 1 as libc::c_int as libc::c_uint
        || *((*p).cigar).as_mut_ptr().offset(0 as libc::c_int as isize)
            & 0xf as libc::c_int as libc::c_uint == 2 as libc::c_int as libc::c_uint
    {
        let mut l_2: int32_t = (*((*p).cigar)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) >> 4 as libc::c_int) as int32_t;
        if *((*p).cigar).as_mut_ptr().offset(0 as libc::c_int as isize)
            & 0xf as libc::c_int as libc::c_uint == 1 as libc::c_int as libc::c_uint
        {
            if (*r).rev() != 0 {
                (*r).qe -= l_2;
            } else {
                (*r).qs += l_2;
            }
            *qshift = l_2;
        } else {
            (*r).rs += l_2;
            *tshift = l_2;
        }
        (*p).n_cigar = ((*p).n_cigar).wrapping_sub(1);
        (*p).n_cigar;
        memmove(
            ((*p).cigar).as_mut_ptr() as *mut libc::c_void,
            ((*p).cigar).as_mut_ptr().offset(1 as libc::c_int as isize)
                as *const libc::c_void,
            ((*p).n_cigar).wrapping_mul(4 as libc::c_int as libc::c_uint)
                as libc::c_ulong,
        );
    }
}
unsafe extern "C" fn mm_update_cigar_eqx(
    mut r: *mut mm_reg1_t,
    mut qseq: *const uint8_t,
    mut tseq: *const uint8_t,
) {
    let mut n_EQX: uint32_t = 0 as libc::c_int as uint32_t;
    let mut k: uint32_t = 0;
    let mut l: uint32_t = 0;
    let mut m: uint32_t = 0;
    let mut cap: uint32_t = 0;
    let mut toff: uint32_t = 0 as libc::c_int as uint32_t;
    let mut qoff: uint32_t = 0 as libc::c_int as uint32_t;
    let mut n_M: uint32_t = 0 as libc::c_int as uint32_t;
    let mut p: *mut mm_extra_t = 0 as *mut mm_extra_t;
    if ((*r).p).is_null() {
        return;
    }
    k = 0 as libc::c_int as uint32_t;
    while k < (*(*r).p).n_cigar {
        let mut op: uint32_t = *((*(*r).p).cigar).as_mut_ptr().offset(k as isize)
            & 0xf as libc::c_int as libc::c_uint;
        let mut len: uint32_t = *((*(*r).p).cigar).as_mut_ptr().offset(k as isize)
            >> 4 as libc::c_int;
        if op == 0 as libc::c_int as libc::c_uint {
            while len > 0 as libc::c_int as libc::c_uint {
                l = 0 as libc::c_int as uint32_t;
                while l < len
                    && *qseq.offset(qoff.wrapping_add(l) as isize) as libc::c_int
                        == *tseq.offset(toff.wrapping_add(l) as isize) as libc::c_int
                {
                    l = l.wrapping_add(1);
                    l;
                }
                if l > 0 as libc::c_int as libc::c_uint {
                    n_EQX = n_EQX.wrapping_add(1);
                    n_EQX;
                    len = (len as libc::c_uint).wrapping_sub(l) as uint32_t as uint32_t;
                    toff = (toff as libc::c_uint).wrapping_add(l) as uint32_t
                        as uint32_t;
                    qoff = (qoff as libc::c_uint).wrapping_add(l) as uint32_t
                        as uint32_t;
                }
                l = 0 as libc::c_int as uint32_t;
                while l < len
                    && *qseq.offset(qoff.wrapping_add(l) as isize) as libc::c_int
                        != *tseq.offset(toff.wrapping_add(l) as isize) as libc::c_int
                {
                    l = l.wrapping_add(1);
                    l;
                }
                if l > 0 as libc::c_int as libc::c_uint {
                    n_EQX = n_EQX.wrapping_add(1);
                    n_EQX;
                    len = (len as libc::c_uint).wrapping_sub(l) as uint32_t as uint32_t;
                    toff = (toff as libc::c_uint).wrapping_add(l) as uint32_t
                        as uint32_t;
                    qoff = (qoff as libc::c_uint).wrapping_add(l) as uint32_t
                        as uint32_t;
                }
            }
            n_M = n_M.wrapping_add(1);
            n_M;
        } else if op == 1 as libc::c_int as libc::c_uint {
            qoff = (qoff as libc::c_uint).wrapping_add(len) as uint32_t as uint32_t;
        } else if op == 2 as libc::c_int as libc::c_uint {
            toff = (toff as libc::c_uint).wrapping_add(len) as uint32_t as uint32_t;
        } else if op == 3 as libc::c_int as libc::c_uint {
            toff = (toff as libc::c_uint).wrapping_add(len) as uint32_t as uint32_t;
        }
        k = k.wrapping_add(1);
        k;
    }
    if n_EQX == n_M {
        k = 0 as libc::c_int as uint32_t;
        while k < (*(*r).p).n_cigar {
            let mut op_0: uint32_t = *((*(*r).p).cigar).as_mut_ptr().offset(k as isize)
                & 0xf as libc::c_int as libc::c_uint;
            let mut len_0: uint32_t = *((*(*r).p).cigar).as_mut_ptr().offset(k as isize)
                >> 4 as libc::c_int;
            if op_0 == 0 as libc::c_int as libc::c_uint {
                *((*(*r).p).cigar)
                    .as_mut_ptr()
                    .offset(
                        k as isize,
                    ) = len_0 << 4 as libc::c_int | 7 as libc::c_int as libc::c_uint;
            }
            k = k.wrapping_add(1);
            k;
        }
        return;
    }
    cap = (((*(*r).p).n_cigar).wrapping_add(n_EQX.wrapping_sub(n_M)) as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<mm_extra_t>() as libc::c_ulong) as uint32_t;
    cap = cap.wrapping_sub(1);
    cap;
    cap |= cap >> 1 as libc::c_int;
    cap |= cap >> 2 as libc::c_int;
    cap |= cap >> 4 as libc::c_int;
    cap |= cap >> 8 as libc::c_int;
    cap |= cap >> 16 as libc::c_int;
    cap = cap.wrapping_add(1);
    cap;
    p = calloc(cap as libc::c_ulong, 4 as libc::c_int as libc::c_ulong)
        as *mut mm_extra_t;
    memcpy(
        p as *mut libc::c_void,
        (*r).p as *const libc::c_void,
        ::std::mem::size_of::<mm_extra_t>() as libc::c_ulong,
    );
    (*p).capacity = cap;
    m = 0 as libc::c_int as uint32_t;
    qoff = m;
    toff = qoff;
    k = 0 as libc::c_int as uint32_t;
    while k < (*(*r).p).n_cigar {
        let mut op_1: uint32_t = *((*(*r).p).cigar).as_mut_ptr().offset(k as isize)
            & 0xf as libc::c_int as libc::c_uint;
        let mut len_1: uint32_t = *((*(*r).p).cigar).as_mut_ptr().offset(k as isize)
            >> 4 as libc::c_int;
        if op_1 == 0 as libc::c_int as libc::c_uint {
            while len_1 > 0 as libc::c_int as libc::c_uint {
                l = 0 as libc::c_int as uint32_t;
                while l < len_1
                    && *qseq.offset(qoff.wrapping_add(l) as isize) as libc::c_int
                        == *tseq.offset(toff.wrapping_add(l) as isize) as libc::c_int
                {
                    l = l.wrapping_add(1);
                    l;
                }
                if l > 0 as libc::c_int as libc::c_uint {
                    let fresh8 = m;
                    m = m.wrapping_add(1);
                    *((*p).cigar)
                        .as_mut_ptr()
                        .offset(
                            fresh8 as isize,
                        ) = l << 4 as libc::c_int | 7 as libc::c_int as libc::c_uint;
                }
                len_1 = (len_1 as libc::c_uint).wrapping_sub(l) as uint32_t as uint32_t;
                toff = (toff as libc::c_uint).wrapping_add(l) as uint32_t as uint32_t;
                qoff = (qoff as libc::c_uint).wrapping_add(l) as uint32_t as uint32_t;
                l = 0 as libc::c_int as uint32_t;
                while l < len_1
                    && *qseq.offset(qoff.wrapping_add(l) as isize) as libc::c_int
                        != *tseq.offset(toff.wrapping_add(l) as isize) as libc::c_int
                {
                    l = l.wrapping_add(1);
                    l;
                }
                if l > 0 as libc::c_int as libc::c_uint {
                    let fresh9 = m;
                    m = m.wrapping_add(1);
                    *((*p).cigar)
                        .as_mut_ptr()
                        .offset(
                            fresh9 as isize,
                        ) = l << 4 as libc::c_int | 8 as libc::c_int as libc::c_uint;
                }
                len_1 = (len_1 as libc::c_uint).wrapping_sub(l) as uint32_t as uint32_t;
                toff = (toff as libc::c_uint).wrapping_add(l) as uint32_t as uint32_t;
                qoff = (qoff as libc::c_uint).wrapping_add(l) as uint32_t as uint32_t;
            }
        } else {
            if op_1 == 1 as libc::c_int as libc::c_uint {
                qoff = (qoff as libc::c_uint).wrapping_add(len_1) as uint32_t
                    as uint32_t;
            } else if op_1 == 2 as libc::c_int as libc::c_uint {
                toff = (toff as libc::c_uint).wrapping_add(len_1) as uint32_t
                    as uint32_t;
            } else if op_1 == 3 as libc::c_int as libc::c_uint {
                toff = (toff as libc::c_uint).wrapping_add(len_1) as uint32_t
                    as uint32_t;
            }
            let fresh10 = m;
            m = m.wrapping_add(1);
            *((*p).cigar)
                .as_mut_ptr()
                .offset(
                    fresh10 as isize,
                ) = *((*(*r).p).cigar).as_mut_ptr().offset(k as isize);
        }
        k = k.wrapping_add(1);
        k;
    }
    (*p).n_cigar = m;
    free((*r).p as *mut libc::c_void);
    (*r).p = p;
}
unsafe extern "C" fn mm_update_extra(
    mut r: *mut mm_reg1_t,
    mut qseq: *const uint8_t,
    mut tseq: *const uint8_t,
    mut mat: *const int8_t,
    mut q: int8_t,
    mut e: int8_t,
    mut is_eqx: libc::c_int,
    mut log_gap: libc::c_int,
) {
    let mut k: uint32_t = 0;
    let mut l: uint32_t = 0;
    let mut qshift: int32_t = 0;
    let mut tshift: int32_t = 0;
    let mut toff: int32_t = 0 as libc::c_int;
    let mut qoff: int32_t = 0 as libc::c_int;
    let mut s: libc::c_double = 0.0f64;
    let mut max: libc::c_double = 0.0f64;
    let mut p: *mut mm_extra_t = (*r).p;
    if p.is_null() {
        return;
    }
    mm_fix_cigar(r, qseq, tseq, &mut qshift, &mut tshift);
    qseq = qseq.offset(qshift as isize);
    tseq = tseq.offset(tshift as isize);
    (*r).mlen = 0 as libc::c_int;
    (*r).blen = (*r).mlen;
    k = 0 as libc::c_int as uint32_t;
    while k < (*p).n_cigar {
        let mut op: uint32_t = *((*p).cigar).as_mut_ptr().offset(k as isize)
            & 0xf as libc::c_int as libc::c_uint;
        let mut len: uint32_t = *((*p).cigar).as_mut_ptr().offset(k as isize)
            >> 4 as libc::c_int;
        if op == 0 as libc::c_int as libc::c_uint {
            let mut n_ambi: libc::c_int = 0 as libc::c_int;
            let mut n_diff: libc::c_int = 0 as libc::c_int;
            l = 0 as libc::c_int as uint32_t;
            while l < len {
                let mut cq: libc::c_int = *qseq
                    .offset((qoff as libc::c_uint).wrapping_add(l) as isize)
                    as libc::c_int;
                let mut ct: libc::c_int = *tseq
                    .offset((toff as libc::c_uint).wrapping_add(l) as isize)
                    as libc::c_int;
                if ct > 3 as libc::c_int || cq > 3 as libc::c_int {
                    n_ambi += 1;
                    n_ambi;
                } else if ct != cq {
                    n_diff += 1;
                    n_diff;
                }
                s
                    += *mat.offset((ct * 5 as libc::c_int + cq) as isize) as libc::c_int
                        as libc::c_double;
                if s < 0 as libc::c_int as libc::c_double {
                    s = 0 as libc::c_int as libc::c_double;
                } else {
                    max = if max > s { max } else { s };
                }
                l = l.wrapping_add(1);
                l;
            }
            (*r)
                .blen = ((*r).blen as libc::c_uint)
                .wrapping_add(len.wrapping_sub(n_ambi as libc::c_uint)) as int32_t
                as int32_t;
            (*r)
                .mlen = ((*r).mlen as libc::c_uint)
                .wrapping_add(len.wrapping_sub((n_ambi + n_diff) as libc::c_uint))
                as int32_t as int32_t;
            (*p).set_n_ambi((*p).n_ambi() + n_ambi as uint32_t);
            toff = (toff as libc::c_uint).wrapping_add(len) as int32_t as int32_t;
            qoff = (qoff as libc::c_uint).wrapping_add(len) as int32_t as int32_t;
        } else if op == 1 as libc::c_int as libc::c_uint {
            let mut n_ambi_0: libc::c_int = 0 as libc::c_int;
            l = 0 as libc::c_int as uint32_t;
            while l < len {
                if *qseq.offset((qoff as libc::c_uint).wrapping_add(l) as isize)
                    as libc::c_int > 3 as libc::c_int
                {
                    n_ambi_0 += 1;
                    n_ambi_0;
                }
                l = l.wrapping_add(1);
                l;
            }
            (*r)
                .blen = ((*r).blen as libc::c_uint)
                .wrapping_add(len.wrapping_sub(n_ambi_0 as libc::c_uint)) as int32_t
                as int32_t;
            (*p).set_n_ambi((*p).n_ambi() + n_ambi_0 as uint32_t);
            if log_gap != 0 {
                s
                    -= q as libc::c_int as libc::c_double
                        + e as libc::c_double
                            * mg_log2((1.0f64 + len as libc::c_double) as libc::c_float)
                                as libc::c_double;
            } else {
                s -= (q as libc::c_int + e as libc::c_int) as libc::c_double;
            }
            if s < 0 as libc::c_int as libc::c_double {
                s = 0 as libc::c_int as libc::c_double;
            }
            qoff = (qoff as libc::c_uint).wrapping_add(len) as int32_t as int32_t;
        } else if op == 2 as libc::c_int as libc::c_uint {
            let mut n_ambi_1: libc::c_int = 0 as libc::c_int;
            l = 0 as libc::c_int as uint32_t;
            while l < len {
                if *tseq.offset((toff as libc::c_uint).wrapping_add(l) as isize)
                    as libc::c_int > 3 as libc::c_int
                {
                    n_ambi_1 += 1;
                    n_ambi_1;
                }
                l = l.wrapping_add(1);
                l;
            }
            (*r)
                .blen = ((*r).blen as libc::c_uint)
                .wrapping_add(len.wrapping_sub(n_ambi_1 as libc::c_uint)) as int32_t
                as int32_t;
            (*p).set_n_ambi((*p).n_ambi() + n_ambi_1 as uint32_t);
            if log_gap != 0 {
                s
                    -= q as libc::c_int as libc::c_double
                        + e as libc::c_double
                            * mg_log2((1.0f64 + len as libc::c_double) as libc::c_float)
                                as libc::c_double;
            } else {
                s -= (q as libc::c_int + e as libc::c_int) as libc::c_double;
            }
            if s < 0 as libc::c_int as libc::c_double {
                s = 0 as libc::c_int as libc::c_double;
            }
            toff = (toff as libc::c_uint).wrapping_add(len) as int32_t as int32_t;
        } else if op == 3 as libc::c_int as libc::c_uint {
            toff = (toff as libc::c_uint).wrapping_add(len) as int32_t as int32_t;
        }
        k = k.wrapping_add(1);
        k;
    }
    (*p).dp_max = (max + 0.499f64) as int32_t;
    if qoff == (*r).qe - (*r).qs && toff == (*r).re - (*r).rs {} else {
        __assert_fail(
            b"qoff == r->qe - r->qs && toff == r->re - r->rs\0" as *const u8
                as *const libc::c_char,
            b"align.c\0" as *const u8 as *const libc::c_char,
            287 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 110],
                &[libc::c_char; 110],
            >(
                b"void mm_update_extra(mm_reg1_t *, const uint8_t *, const uint8_t *, const int8_t *, int8_t, int8_t, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_6517: {
        if qoff == (*r).qe - (*r).qs && toff == (*r).re - (*r).rs {} else {
            __assert_fail(
                b"qoff == r->qe - r->qs && toff == r->re - r->rs\0" as *const u8
                    as *const libc::c_char,
                b"align.c\0" as *const u8 as *const libc::c_char,
                287 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 110],
                    &[libc::c_char; 110],
                >(
                    b"void mm_update_extra(mm_reg1_t *, const uint8_t *, const uint8_t *, const int8_t *, int8_t, int8_t, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if is_eqx != 0 {
        mm_update_cigar_eqx(r, qseq, tseq);
    }
}
unsafe extern "C" fn mm_append_cigar(
    mut r: *mut mm_reg1_t,
    mut n_cigar: uint32_t,
    mut cigar: *mut uint32_t,
) {
    let mut p: *mut mm_extra_t = 0 as *mut mm_extra_t;
    if n_cigar == 0 as libc::c_int as libc::c_uint {
        return;
    }
    if ((*r).p).is_null() {
        let mut capacity: uint32_t = (n_cigar as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<mm_extra_t>() as libc::c_ulong)
                    .wrapping_div(4 as libc::c_int as libc::c_ulong),
            ) as uint32_t;
        capacity = capacity.wrapping_sub(1);
        capacity;
        capacity |= capacity >> 1 as libc::c_int;
        capacity |= capacity >> 2 as libc::c_int;
        capacity |= capacity >> 4 as libc::c_int;
        capacity |= capacity >> 8 as libc::c_int;
        capacity |= capacity >> 16 as libc::c_int;
        capacity = capacity.wrapping_add(1);
        capacity;
        (*r)
            .p = calloc(capacity as libc::c_ulong, 4 as libc::c_int as libc::c_ulong)
            as *mut mm_extra_t;
        (*(*r).p).capacity = capacity;
    } else if (((*(*r).p).n_cigar).wrapping_add(n_cigar) as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<mm_extra_t>() as libc::c_ulong)
                .wrapping_div(4 as libc::c_int as libc::c_ulong),
        ) > (*(*r).p).capacity as libc::c_ulong
    {
        (*(*r).p)
            .capacity = (((*(*r).p).n_cigar).wrapping_add(n_cigar) as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<mm_extra_t>() as libc::c_ulong)
                    .wrapping_div(4 as libc::c_int as libc::c_ulong),
            ) as uint32_t;
        (*(*r).p).capacity = ((*(*r).p).capacity).wrapping_sub(1);
        (*(*r).p).capacity;
        (*(*r).p).capacity |= (*(*r).p).capacity >> 1 as libc::c_int;
        (*(*r).p).capacity |= (*(*r).p).capacity >> 2 as libc::c_int;
        (*(*r).p).capacity |= (*(*r).p).capacity >> 4 as libc::c_int;
        (*(*r).p).capacity |= (*(*r).p).capacity >> 8 as libc::c_int;
        (*(*r).p).capacity |= (*(*r).p).capacity >> 16 as libc::c_int;
        (*(*r).p).capacity = ((*(*r).p).capacity).wrapping_add(1);
        (*(*r).p).capacity;
        (*r)
            .p = realloc(
            (*r).p as *mut libc::c_void,
            ((*(*r).p).capacity).wrapping_mul(4 as libc::c_int as libc::c_uint)
                as libc::c_ulong,
        ) as *mut mm_extra_t;
    }
    p = (*r).p;
    if (*p).n_cigar > 0 as libc::c_int as libc::c_uint
        && *((*p).cigar)
            .as_mut_ptr()
            .offset(
                ((*p).n_cigar).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
            ) & 0xf as libc::c_int as libc::c_uint
            == *cigar.offset(0 as libc::c_int as isize)
                & 0xf as libc::c_int as libc::c_uint
    {
        let ref mut fresh11 = *((*p).cigar)
            .as_mut_ptr()
            .offset(
                ((*p).n_cigar).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
            );
        *fresh11 = (*fresh11 as libc::c_uint)
            .wrapping_add(
                (*cigar.offset(0 as libc::c_int as isize) >> 4 as libc::c_int)
                    << 4 as libc::c_int,
            ) as uint32_t as uint32_t;
        if n_cigar > 1 as libc::c_int as libc::c_uint {
            memcpy(
                ((*p).cigar).as_mut_ptr().offset((*p).n_cigar as isize)
                    as *mut libc::c_void,
                cigar.offset(1 as libc::c_int as isize) as *const libc::c_void,
                n_cigar
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_mul(4 as libc::c_int as libc::c_uint) as libc::c_ulong,
            );
        }
        (*p)
            .n_cigar = ((*p).n_cigar as libc::c_uint)
            .wrapping_add(n_cigar.wrapping_sub(1 as libc::c_int as libc::c_uint))
            as uint32_t as uint32_t;
    } else {
        memcpy(
            ((*p).cigar).as_mut_ptr().offset((*p).n_cigar as isize) as *mut libc::c_void,
            cigar as *const libc::c_void,
            n_cigar.wrapping_mul(4 as libc::c_int as libc::c_uint) as libc::c_ulong,
        );
        (*p)
            .n_cigar = ((*p).n_cigar as libc::c_uint).wrapping_add(n_cigar) as uint32_t
            as uint32_t;
    };
}
unsafe extern "C" fn mm_align_pair(
    mut km: *mut libc::c_void,
    mut opt: *const mm_mapopt_t,
    mut qlen: libc::c_int,
    mut qseq: *const uint8_t,
    mut tlen: libc::c_int,
    mut tseq: *const uint8_t,
    mut junc: *const uint8_t,
    mut mat: *const int8_t,
    mut w: libc::c_int,
    mut end_bonus: libc::c_int,
    mut zdrop: libc::c_int,
    mut flag: libc::c_int,
    mut ez: *mut ksw_extz_t,
) {
    if mm_dbg_flag & 0x8 as libc::c_int != 0 {
        let mut i: libc::c_int = 0;
        fprintf(
            stderr,
            b"===> q=(%d,%d), e=(%d,%d), bw=%d, flag=%d, zdrop=%d <===\n\0" as *const u8
                as *const libc::c_char,
            (*opt).q,
            (*opt).q2,
            (*opt).e,
            (*opt).e2,
            w,
            flag,
            (*opt).zdrop,
        );
        i = 0 as libc::c_int;
        while i < tlen {
            fputc(
                (*::std::mem::transmute::<
                    &[u8; 6],
                    &[libc::c_char; 6],
                >(b"ACGTN\0"))[*tseq.offset(i as isize) as usize] as libc::c_int,
                stderr,
            );
            i += 1;
            i;
        }
        fputc('\n' as i32, stderr);
        i = 0 as libc::c_int;
        while i < qlen {
            fputc(
                (*::std::mem::transmute::<
                    &[u8; 6],
                    &[libc::c_char; 6],
                >(b"ACGTN\0"))[*qseq.offset(i as isize) as usize] as libc::c_int,
                stderr,
            );
            i += 1;
            i;
        }
        fputc('\n' as i32, stderr);
    }
    if (*opt).max_sw_mat > 0 as libc::c_int as libc::c_long
        && tlen as int64_t * qlen as libc::c_long > (*opt).max_sw_mat
    {
        ksw_reset_extz(ez);
        (*ez).set_zdropped(1 as libc::c_int as uint32_t);
    } else if (*opt).flag & 0x80 as libc::c_int as libc::c_long != 0 {
        ksw_exts2_sse(
            km,
            qlen,
            qseq,
            tlen,
            tseq,
            5 as libc::c_int as int8_t,
            mat,
            (*opt).q as int8_t,
            (*opt).e as int8_t,
            (*opt).q2 as int8_t,
            (*opt).noncan as int8_t,
            zdrop,
            (*opt).junc_bonus as int8_t,
            flag,
            junc,
            ez,
        );
    } else if (*opt).q == (*opt).q2 && (*opt).e == (*opt).e2 {
        ksw_extz2_sse(
            km,
            qlen,
            qseq,
            tlen,
            tseq,
            5 as libc::c_int as int8_t,
            mat,
            (*opt).q as int8_t,
            (*opt).e as int8_t,
            w,
            zdrop,
            end_bonus,
            flag,
            ez,
        );
    } else {
        ksw_extd2_sse(
            km,
            qlen,
            qseq,
            tlen,
            tseq,
            5 as libc::c_int as int8_t,
            mat,
            (*opt).q as int8_t,
            (*opt).e as int8_t,
            (*opt).q2 as int8_t,
            (*opt).e2 as int8_t,
            w,
            zdrop,
            end_bonus,
            flag,
            ez,
        );
    }
    if mm_dbg_flag & 0x8 as libc::c_int != 0 {
        let mut i_0: libc::c_int = 0;
        fprintf(
            stderr,
            b"score=%d, cigar=\0" as *const u8 as *const libc::c_char,
            (*ez).score,
        );
        i_0 = 0 as libc::c_int;
        while i_0 < (*ez).n_cigar {
            fprintf(
                stderr,
                b"%d%c\0" as *const u8 as *const libc::c_char,
                *((*ez).cigar).offset(i_0 as isize) >> 4 as libc::c_int,
                (*::std::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(
                    b"MIDNSHP=XB\0",
                ))[(*((*ez).cigar).offset(i_0 as isize)
                    & 0xf as libc::c_int as libc::c_uint) as usize] as libc::c_int,
            );
            i_0 += 1;
            i_0;
        }
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
}
#[inline]
unsafe extern "C" fn mm_get_hplen_back(
    mut mi: *const mm_idx_t,
    mut rid: uint32_t,
    mut x: uint32_t,
) -> libc::c_int {
    let mut i: int64_t = 0;
    let mut off0: int64_t = (*((*mi).seq).offset(rid as isize)).offset as int64_t;
    let mut off: int64_t = off0 + x as libc::c_long;
    let mut c: libc::c_int = (*((*mi).S).offset((off >> 3 as libc::c_int) as isize)
        >> ((off & 7 as libc::c_int as libc::c_long) << 2 as libc::c_int)
        & 0xf as libc::c_int as libc::c_uint) as libc::c_int;
    i = off - 1 as libc::c_int as libc::c_long;
    while i >= off0 {
        if *((*mi).S).offset((i >> 3 as libc::c_int) as isize)
            >> ((i & 7 as libc::c_int as libc::c_long) << 2 as libc::c_int)
            & 0xf as libc::c_int as libc::c_uint != c as libc::c_uint
        {
            break;
        }
        i -= 1;
        i;
    }
    return (off - i) as libc::c_int;
}
#[inline]
unsafe extern "C" fn mm_adjust_minier(
    mut mi: *const mm_idx_t,
    mut qseq0: *const *mut uint8_t,
    mut a: *mut mm128_t,
    mut r: *mut int32_t,
    mut q: *mut int32_t,
) {
    if (*mi).flag & 0x1 as libc::c_int != 0 {
        let mut qseq: *const uint8_t = *qseq0
            .offset(((*a).x >> 63 as libc::c_int) as isize);
        let mut i: libc::c_int = 0;
        let mut c: libc::c_int = 0;
        *q = (*a).y as int32_t;
        i = *q - 1 as libc::c_int;
        c = *qseq.offset(*q as isize) as libc::c_int;
        while i > 0 as libc::c_int {
            if *qseq.offset(i as isize) as libc::c_int != c {
                break;
            }
            i -= 1;
            i;
        }
        *q = i + 1 as libc::c_int;
        c = mm_get_hplen_back(
            mi,
            ((*a).x << 1 as libc::c_int >> 33 as libc::c_int) as uint32_t,
            (*a).x as int32_t as uint32_t,
        );
        *r = (*a).x as int32_t + 1 as libc::c_int - c;
    } else {
        *r = (*a).x as int32_t - ((*mi).k >> 1 as libc::c_int);
        *q = (*a).y as int32_t - ((*mi).k >> 1 as libc::c_int);
    };
}
unsafe extern "C" fn collect_long_gaps(
    mut km: *mut libc::c_void,
    mut as1: libc::c_int,
    mut cnt1: libc::c_int,
    mut a: *mut mm128_t,
    mut min_gap: libc::c_int,
    mut n_: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut K: *mut libc::c_int = 0 as *mut libc::c_int;
    *n_ = 0 as libc::c_int;
    i = 1 as libc::c_int;
    n = 0 as libc::c_int;
    while i < cnt1 {
        let mut gap: libc::c_int = ((*a.offset((as1 + i) as isize)).y as int32_t
            as libc::c_ulong)
            .wrapping_sub((*a.offset((as1 + i - 1 as libc::c_int) as isize)).y)
            .wrapping_sub(
                ((*a.offset((as1 + i) as isize)).x as int32_t as libc::c_ulong)
                    .wrapping_sub((*a.offset((as1 + i - 1 as libc::c_int) as isize)).x),
            ) as libc::c_int;
        if gap < -min_gap || gap > min_gap {
            n += 1;
            n;
        }
        i += 1;
        i;
    }
    if n <= 1 as libc::c_int {
        return 0 as *mut libc::c_int;
    }
    K = kmalloc(
        km,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 1 as libc::c_int;
    n = 0 as libc::c_int;
    while i < cnt1 {
        let mut gap_0: libc::c_int = ((*a.offset((as1 + i) as isize)).y as int32_t
            as libc::c_ulong)
            .wrapping_sub((*a.offset((as1 + i - 1 as libc::c_int) as isize)).y)
            .wrapping_sub(
                ((*a.offset((as1 + i) as isize)).x as int32_t as libc::c_ulong)
                    .wrapping_sub((*a.offset((as1 + i - 1 as libc::c_int) as isize)).x),
            ) as libc::c_int;
        if gap_0 < -min_gap || gap_0 > min_gap {
            let fresh12 = n;
            n = n + 1;
            *K.offset(fresh12 as isize) = i;
        }
        i += 1;
        i;
    }
    *n_ = n;
    return K;
}
unsafe extern "C" fn mm_filter_bad_seeds(
    mut km: *mut libc::c_void,
    mut as1: libc::c_int,
    mut cnt1: libc::c_int,
    mut a: *mut mm128_t,
    mut min_gap: libc::c_int,
    mut diff_thres: libc::c_int,
    mut max_ext_len: libc::c_int,
    mut max_ext_cnt: libc::c_int,
) {
    let mut max_st: libc::c_int = 0;
    let mut max_en: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut K: *mut libc::c_int = 0 as *mut libc::c_int;
    K = collect_long_gaps(km, as1, cnt1, a, min_gap, &mut n);
    if K.is_null() {
        return;
    }
    max = 0 as libc::c_int;
    max_en = -(1 as libc::c_int);
    max_st = max_en;
    k = 0 as libc::c_int;
    loop {
        let mut gap: libc::c_int = 0;
        let mut l: libc::c_int = 0;
        let mut n_ins: libc::c_int = 0 as libc::c_int;
        let mut n_del: libc::c_int = 0 as libc::c_int;
        let mut qs: libc::c_int = 0;
        let mut rs: libc::c_int = 0;
        let mut max_diff: libc::c_int = 0 as libc::c_int;
        let mut max_diff_l: libc::c_int = -(1 as libc::c_int);
        if k == n || k >= max_en {
            if max_en > 0 as libc::c_int {
                i = *K.offset(max_st as isize);
                while i < *K.offset(max_en as isize) {
                    let ref mut fresh13 = (*a.offset((as1 + i) as isize)).y;
                    *fresh13 = (*fresh13 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << 41 as libc::c_int) as uint64_t;
                    i += 1;
                    i;
                }
            }
            max = 0 as libc::c_int;
            max_en = -(1 as libc::c_int);
            max_st = max_en;
            if k == n {
                break;
            }
        }
        i = *K.offset(k as isize);
        gap = (*a.offset((as1 + i) as isize)).y as int32_t
            - (*a.offset((as1 + i - 1 as libc::c_int) as isize)).y as int32_t
            - ((*a.offset((as1 + i) as isize)).x)
                .wrapping_sub((*a.offset((as1 + i - 1 as libc::c_int) as isize)).x)
                as int32_t;
        if gap > 0 as libc::c_int {
            n_ins += gap;
        } else {
            n_del += -gap;
        }
        qs = (*a.offset((as1 + i - 1 as libc::c_int) as isize)).y as int32_t;
        rs = (*a.offset((as1 + i - 1 as libc::c_int) as isize)).x as int32_t;
        l = k + 1 as libc::c_int;
        while l < n && l <= k + max_ext_cnt {
            let mut j: libc::c_int = *K.offset(l as isize);
            let mut diff: libc::c_int = 0;
            if (*a.offset((as1 + j) as isize)).y as int32_t - qs > max_ext_len
                || (*a.offset((as1 + j) as isize)).x as int32_t - rs > max_ext_len
            {
                break;
            }
            gap = (*a.offset((as1 + j) as isize)).y as int32_t
                - (*a.offset((as1 + j - 1 as libc::c_int) as isize)).y as int32_t
                - ((*a.offset((as1 + j) as isize)).x)
                    .wrapping_sub((*a.offset((as1 + j - 1 as libc::c_int) as isize)).x)
                    as int32_t;
            if gap > 0 as libc::c_int {
                n_ins += gap;
            } else {
                n_del += -gap;
            }
            diff = n_ins + n_del - abs(n_ins - n_del);
            if max_diff < diff {
                max_diff = diff;
                max_diff_l = l;
            }
            l += 1;
            l;
        }
        if max_diff > diff_thres && max_diff > max {
            max = max_diff;
            max_st = k;
            max_en = max_diff_l;
        }
        k += 1;
        k;
    }
    kfree(km, K as *mut libc::c_void);
}
unsafe extern "C" fn mm_filter_bad_seeds_alt(
    mut km: *mut libc::c_void,
    mut as1: libc::c_int,
    mut cnt1: libc::c_int,
    mut a: *mut mm128_t,
    mut min_gap: libc::c_int,
    mut max_ext: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut K: *mut libc::c_int = 0 as *mut libc::c_int;
    K = collect_long_gaps(km, as1, cnt1, a, min_gap, &mut n);
    if K.is_null() {
        return;
    }
    k = 0 as libc::c_int;
    while k < n {
        let mut i: libc::c_int = *K.offset(k as isize);
        let mut l: libc::c_int = 0;
        let mut gap1: libc::c_int = (*a.offset((as1 + i) as isize)).y as int32_t
            - (*a.offset((as1 + i - 1 as libc::c_int) as isize)).y as int32_t
            - ((*a.offset((as1 + i) as isize)).x as int32_t
                - (*a.offset((as1 + i - 1 as libc::c_int) as isize)).x as int32_t);
        let mut re1: libc::c_int = (*a.offset((as1 + i) as isize)).x as int32_t;
        let mut qe1: libc::c_int = (*a.offset((as1 + i) as isize)).y as int32_t;
        gap1 = if gap1 > 0 as libc::c_int { gap1 } else { -gap1 };
        l = k + 1 as libc::c_int;
        while l < n {
            let mut j: libc::c_int = *K.offset(l as isize);
            let mut gap2: libc::c_int = 0;
            let mut q_span_pre: libc::c_int = 0;
            let mut rs2: libc::c_int = 0;
            let mut qs2: libc::c_int = 0;
            let mut m: libc::c_int = 0;
            if (*a.offset((as1 + j) as isize)).y as int32_t - qe1 > max_ext
                || (*a.offset((as1 + j) as isize)).x as int32_t - re1 > max_ext
            {
                break;
            }
            gap2 = (*a.offset((as1 + j) as isize)).y as int32_t
                - (*a.offset((as1 + j - 1 as libc::c_int) as isize)).y as int32_t
                - ((*a.offset((as1 + j) as isize)).x)
                    .wrapping_sub((*a.offset((as1 + j - 1 as libc::c_int) as isize)).x)
                    as int32_t;
            q_span_pre = ((*a.offset((as1 + j - 1 as libc::c_int) as isize)).y
                >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as libc::c_int;
            rs2 = (*a.offset((as1 + j - 1 as libc::c_int) as isize)).x as int32_t
                + q_span_pre;
            qs2 = (*a.offset((as1 + j - 1 as libc::c_int) as isize)).y as int32_t
                + q_span_pre;
            m = if rs2 - re1 < qs2 - qe1 { rs2 - re1 } else { qs2 - qe1 };
            gap2 = if gap2 > 0 as libc::c_int { gap2 } else { -gap2 };
            if m > gap1 + gap2 {
                break;
            }
            re1 = (*a.offset((as1 + j) as isize)).x as int32_t;
            qe1 = (*a.offset((as1 + j) as isize)).y as int32_t;
            gap1 = gap2;
            l += 1;
            l;
        }
        if l > k + 1 as libc::c_int {
            let mut j_0: libc::c_int = 0;
            let mut end: libc::c_int = *K.offset((l - 1 as libc::c_int) as isize);
            j_0 = *K.offset(k as isize);
            while j_0 < end {
                let ref mut fresh14 = (*a.offset((as1 + j_0) as isize)).y;
                *fresh14 = (*fresh14 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 41 as libc::c_int) as uint64_t;
                j_0 += 1;
                j_0;
            }
            let ref mut fresh15 = (*a.offset((as1 + end) as isize)).y;
            *fresh15 = (*fresh15 as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 40 as libc::c_int) as uint64_t;
        }
        k = l;
    }
    kfree(km, K as *mut libc::c_void);
}
unsafe extern "C" fn mm_fix_bad_ends(
    mut r: *const mm_reg1_t,
    mut a: *const mm128_t,
    mut bw: libc::c_int,
    mut min_match: libc::c_int,
    mut as_0: *mut int32_t,
    mut cnt: *mut int32_t,
) {
    let mut i: int32_t = 0;
    let mut l: int32_t = 0;
    let mut m: int32_t = 0;
    *as_0 = (*r).as_0;
    *cnt = (*r).cnt;
    if (*r).cnt < 3 as libc::c_int {
        return;
    }
    l = ((*a.offset((*r).as_0 as isize)).y >> 32 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as int32_t;
    m = l;
    i = (*r).as_0 + 1 as libc::c_int;
    while i < (*r).as_0 + (*r).cnt - 1 as libc::c_int {
        let mut lq: int32_t = 0;
        let mut lr: int32_t = 0;
        let mut min: int32_t = 0;
        let mut max: int32_t = 0;
        let mut q_span: int32_t = ((*a.offset(i as isize)).y >> 32 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as int32_t;
        if (*a.offset(i as isize)).y as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 40 as libc::c_int != 0
        {
            break;
        }
        lr = (*a.offset(i as isize)).x as int32_t
            - (*a.offset((i - 1 as libc::c_int) as isize)).x as int32_t;
        lq = (*a.offset(i as isize)).y as int32_t
            - (*a.offset((i - 1 as libc::c_int) as isize)).y as int32_t;
        min = if lr < lq { lr } else { lq };
        max = if lr > lq { lr } else { lq };
        if max - min > l >> 1 as libc::c_int {
            *as_0 = i;
        }
        l += min;
        m += if min < q_span { min } else { q_span };
        if l >= bw << 1 as libc::c_int || m >= min_match && m >= bw
            || m >= (*r).mlen >> 1 as libc::c_int
        {
            break;
        }
        i += 1;
        i;
    }
    *cnt = (*r).as_0 + (*r).cnt - *as_0;
    l = ((*a.offset(((*r).as_0 + (*r).cnt - 1 as libc::c_int) as isize)).y
        >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as int32_t;
    m = l;
    i = (*r).as_0 + (*r).cnt - 2 as libc::c_int;
    while i > *as_0 {
        let mut lq_0: int32_t = 0;
        let mut lr_0: int32_t = 0;
        let mut min_0: int32_t = 0;
        let mut max_0: int32_t = 0;
        let mut q_span_0: int32_t = ((*a.offset((i + 1 as libc::c_int) as isize)).y
            >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as int32_t;
        if (*a.offset((i + 1 as libc::c_int) as isize)).y as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 40 as libc::c_int != 0
        {
            break;
        }
        lr_0 = (*a.offset((i + 1 as libc::c_int) as isize)).x as int32_t
            - (*a.offset(i as isize)).x as int32_t;
        lq_0 = (*a.offset((i + 1 as libc::c_int) as isize)).y as int32_t
            - (*a.offset(i as isize)).y as int32_t;
        min_0 = if lr_0 < lq_0 { lr_0 } else { lq_0 };
        max_0 = if lr_0 > lq_0 { lr_0 } else { lq_0 };
        if max_0 - min_0 > l >> 1 as libc::c_int {
            *cnt = i + 1 as libc::c_int - *as_0;
        }
        l += min_0;
        m += if min_0 < q_span_0 { min_0 } else { q_span_0 };
        if l >= bw << 1 as libc::c_int || m >= min_match && m >= bw
            || m >= (*r).mlen >> 1 as libc::c_int
        {
            break;
        }
        i -= 1;
        i;
    }
}
unsafe extern "C" fn mm_max_stretch(
    mut r: *const mm_reg1_t,
    mut a: *const mm128_t,
    mut as_0: *mut int32_t,
    mut cnt: *mut int32_t,
) {
    let mut i: int32_t = 0;
    let mut score: int32_t = 0;
    let mut max_score: int32_t = 0;
    let mut len: int32_t = 0;
    let mut max_i: int32_t = 0;
    let mut max_len: int32_t = 0;
    *as_0 = (*r).as_0;
    *cnt = (*r).cnt;
    if (*r).cnt < 2 as libc::c_int {
        return;
    }
    max_score = -(1 as libc::c_int);
    max_i = -(1 as libc::c_int);
    max_len = 0 as libc::c_int;
    score = ((*a.offset((*r).as_0 as isize)).y >> 32 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as int32_t;
    len = 1 as libc::c_int;
    i = (*r).as_0 + 1 as libc::c_int;
    while i < (*r).as_0 + (*r).cnt {
        let mut lq: int32_t = 0;
        let mut lr: int32_t = 0;
        let mut q_span: int32_t = 0;
        q_span = ((*a.offset(i as isize)).y >> 32 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as int32_t;
        lr = (*a.offset(i as isize)).x as int32_t
            - (*a.offset((i - 1 as libc::c_int) as isize)).x as int32_t;
        lq = (*a.offset(i as isize)).y as int32_t
            - (*a.offset((i - 1 as libc::c_int) as isize)).y as int32_t;
        if lq == lr {
            score += if lq < q_span { lq } else { q_span };
            len += 1;
            len;
        } else {
            if score > max_score {
                max_score = score;
                max_len = len;
                max_i = i - len;
            }
            score = q_span;
            len = 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    if score > max_score {
        max_score = score;
        max_len = len;
        max_i = i - len;
    }
    *as_0 = max_i;
    *cnt = max_len;
}
unsafe extern "C" fn mm_seed_ext_score(
    mut km: *mut libc::c_void,
    mut opt: *const mm_mapopt_t,
    mut mi: *const mm_idx_t,
    mut mat: *const int8_t,
    mut qlen: libc::c_int,
    mut qseq0: *mut *mut uint8_t,
    mut a: *const mm128_t,
) -> libc::c_int {
    let mut qseq: *mut uint8_t = 0 as *mut uint8_t;
    let mut tseq: *mut uint8_t = 0 as *mut uint8_t;
    let mut q_span: libc::c_int = ((*a).y >> 32 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut qs: libc::c_int = 0;
    let mut qe: libc::c_int = 0;
    let mut rs: libc::c_int = 0;
    let mut re: libc::c_int = 0;
    let mut rid: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    let mut q_off: libc::c_int = 0;
    let mut t_off: libc::c_int = 0;
    let mut ext_len: libc::c_int = (*opt).anchor_ext_len;
    let mut qp: *mut libc::c_void = 0 as *mut libc::c_void;
    rid = ((*a).x << 1 as libc::c_int >> 33 as libc::c_int) as libc::c_int;
    re = ((*a).x as uint32_t).wrapping_add(1 as libc::c_int as libc::c_uint)
        as libc::c_int;
    rs = re - q_span;
    qe = ((*a).y as uint32_t).wrapping_add(1 as libc::c_int as libc::c_uint)
        as libc::c_int;
    qs = qe - q_span;
    rs = if rs - ext_len > 0 as libc::c_int { rs - ext_len } else { 0 as libc::c_int };
    qs = if qs - ext_len > 0 as libc::c_int { qs - ext_len } else { 0 as libc::c_int };
    re = (if re + ext_len < (*((*mi).seq).offset(rid as isize)).len as int32_t {
        (re + ext_len) as libc::c_uint
    } else {
        (*((*mi).seq).offset(rid as isize)).len
    }) as libc::c_int;
    qe = if qe + ext_len < qlen { qe + ext_len } else { qlen };
    tseq = kmalloc(km, (re - rs) as size_t) as *mut uint8_t;
    if (*opt).flag as libc::c_longlong & 0x100000000 as libc::c_longlong != 0 {
        qseq = (*qseq0.offset(0 as libc::c_int as isize)).offset(qs as isize);
        mm_idx_getseq2(
            mi,
            ((*a).x >> 63 as libc::c_int) as libc::c_int,
            rid as uint32_t,
            rs as uint32_t,
            re as uint32_t,
            tseq,
        );
    } else {
        qseq = (*qseq0.offset(((*a).x >> 63 as libc::c_int) as isize))
            .offset(qs as isize);
        mm_idx_getseq(mi, rid as uint32_t, rs as uint32_t, re as uint32_t, tseq);
    }
    qp = ksw_ll_qinit(km, 2 as libc::c_int, qe - qs, qseq, 5 as libc::c_int, mat);
    score = ksw_ll_i16(qp, re - rs, tseq, (*opt).q, (*opt).e, &mut q_off, &mut t_off);
    kfree(km, tseq as *mut libc::c_void);
    kfree(km, qp);
    return score;
}
unsafe extern "C" fn mm_fix_bad_ends_splice(
    mut km: *mut libc::c_void,
    mut opt: *const mm_mapopt_t,
    mut mi: *const mm_idx_t,
    mut r: *const mm_reg1_t,
    mut mat: *const int8_t,
    mut qlen: libc::c_int,
    mut qseq0: *mut *mut uint8_t,
    mut a: *const mm128_t,
    mut as1: *mut libc::c_int,
    mut cnt1: *mut libc::c_int,
) {
    let mut score: libc::c_int = 0;
    let mut log_gap: libc::c_double = 0.;
    *as1 = (*r).as_0;
    *cnt1 = (*r).cnt;
    if (*r).cnt < 3 as libc::c_int {
        return;
    }
    log_gap = log(
        ((*a.offset(((*r).as_0 + 1 as libc::c_int) as isize)).x as int32_t
            - (*a.offset((*r).as_0 as isize)).x as int32_t) as libc::c_double,
    );
    if (((*a.offset((*r).as_0 as isize)).y >> 32 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as libc::c_double)
        < log_gap + (*opt).anchor_ext_shift as libc::c_double
    {
        score = mm_seed_ext_score(
            km,
            opt,
            mi,
            mat,
            qlen,
            qseq0,
            &*a.offset((*r).as_0 as isize),
        );
        if (score as libc::c_double
            / *mat.offset(0 as libc::c_int as isize) as libc::c_int as libc::c_double)
            < log_gap + (*opt).anchor_ext_shift as libc::c_double
        {
            *as1 += 1;
            *as1;
            *cnt1 -= 1;
            *cnt1;
        }
    }
    log_gap = log(
        ((*a.offset(((*r).as_0 + (*r).cnt - 1 as libc::c_int) as isize)).x as int32_t
            - (*a.offset(((*r).as_0 + (*r).cnt - 2 as libc::c_int) as isize)).x
                as int32_t) as libc::c_double,
    );
    if (((*a.offset(((*r).as_0 + (*r).cnt - 1 as libc::c_int) as isize)).y
        >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as libc::c_double)
        < log_gap + (*opt).anchor_ext_shift as libc::c_double
    {
        score = mm_seed_ext_score(
            km,
            opt,
            mi,
            mat,
            qlen,
            qseq0,
            &*a.offset(((*r).as_0 + (*r).cnt - 1 as libc::c_int) as isize),
        );
        if (score as libc::c_double
            / *mat.offset(0 as libc::c_int as isize) as libc::c_int as libc::c_double)
            < log_gap + (*opt).anchor_ext_shift as libc::c_double
        {
            *cnt1 -= 1;
            *cnt1;
        }
    }
}
unsafe extern "C" fn mm_align1(
    mut km: *mut libc::c_void,
    mut opt: *const mm_mapopt_t,
    mut mi: *const mm_idx_t,
    mut qlen: libc::c_int,
    mut qseq0: *mut *mut uint8_t,
    mut r: *mut mm_reg1_t,
    mut r2: *mut mm_reg1_t,
    mut n_a: libc::c_int,
    mut a: *mut mm128_t,
    mut ez: *mut ksw_extz_t,
    mut splice_flag: libc::c_int,
) {
    let mut is_sr: libc::c_int = ((*opt).flag & 0x1000 as libc::c_int as libc::c_long
        != 0) as libc::c_int;
    let mut is_splice: libc::c_int = ((*opt).flag & 0x80 as libc::c_int as libc::c_long
        != 0) as libc::c_int;
    let mut rid: int32_t = ((*a.offset((*r).as_0 as isize)).x << 1 as libc::c_int
        >> 33 as libc::c_int) as int32_t;
    let mut rev: int32_t = ((*a.offset((*r).as_0 as isize)).x >> 63 as libc::c_int)
        as int32_t;
    let mut as1: int32_t = 0;
    let mut cnt1: int32_t = 0;
    let mut tseq: *mut uint8_t = 0 as *mut uint8_t;
    let mut qseq: *mut uint8_t = 0 as *mut uint8_t;
    let mut junc: *mut uint8_t = 0 as *mut uint8_t;
    let mut i: int32_t = 0;
    let mut l: int32_t = 0;
    let mut bw: int32_t = 0;
    let mut bw_long: int32_t = 0;
    let mut dropped: int32_t = 0 as libc::c_int;
    let mut extra_flag: int32_t = 0 as libc::c_int;
    let mut rs0: int32_t = 0;
    let mut re0: int32_t = 0;
    let mut qs0: int32_t = 0;
    let mut qe0: int32_t = 0;
    let mut rs: int32_t = 0;
    let mut re: int32_t = 0;
    let mut qs: int32_t = 0;
    let mut qe: int32_t = 0;
    let mut rs1: int32_t = 0;
    let mut qs1: int32_t = 0;
    let mut re1: int32_t = 0;
    let mut qe1: int32_t = 0;
    let mut mat: [int8_t; 25] = [0; 25];
    if is_sr != 0 {
        if (*mi).flag & 0x1 as libc::c_int == 0 {} else {
            __assert_fail(
                b"!(mi->flag & MM_I_HPC)\0" as *const u8 as *const libc::c_char,
                b"align.c\0" as *const u8 as *const libc::c_char,
                583 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 140],
                    &[libc::c_char; 140],
                >(
                    b"void mm_align1(void *, const mm_mapopt_t *, const mm_idx_t *, int, uint8_t **, mm_reg1_t *, mm_reg1_t *, int, mm128_t *, ksw_extz_t *, int)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_17426: {
            if (*mi).flag & 0x1 as libc::c_int == 0 {} else {
                __assert_fail(
                    b"!(mi->flag & MM_I_HPC)\0" as *const u8 as *const libc::c_char,
                    b"align.c\0" as *const u8 as *const libc::c_char,
                    583 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 140],
                        &[libc::c_char; 140],
                    >(
                        b"void mm_align1(void *, const mm_mapopt_t *, const mm_idx_t *, int, uint8_t **, mm_reg1_t *, mm_reg1_t *, int, mm128_t *, ksw_extz_t *, int)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
    }
    (*r2).cnt = 0 as libc::c_int;
    if (*r).cnt == 0 as libc::c_int {
        return;
    }
    ksw_gen_simple_mat(
        5 as libc::c_int,
        mat.as_mut_ptr(),
        (*opt).a as int8_t,
        (*opt).b as int8_t,
        (*opt).sc_ambi as int8_t,
    );
    bw = ((*opt).bw as libc::c_double * 1.5f64 + 1.0f64) as libc::c_int;
    bw_long = ((*opt).bw_long as libc::c_double * 1.5f64 + 1.0f64) as libc::c_int;
    if bw_long < bw {
        bw_long = bw;
    }
    if is_sr != 0 && (*mi).flag & 0x1 as libc::c_int == 0 {
        mm_max_stretch(r, a, &mut as1, &mut cnt1);
        rs = (*a.offset(as1 as isize)).x as int32_t + 1 as libc::c_int
            - ((*a.offset(as1 as isize)).y >> 32 as libc::c_int
                & 0xff as libc::c_int as libc::c_ulong) as int32_t;
        qs = (*a.offset(as1 as isize)).y as int32_t + 1 as libc::c_int
            - ((*a.offset(as1 as isize)).y >> 32 as libc::c_int
                & 0xff as libc::c_int as libc::c_ulong) as int32_t;
        re = (*a.offset((as1 + cnt1 - 1 as libc::c_int) as isize)).x as int32_t
            + 1 as libc::c_int;
        qe = (*a.offset((as1 + cnt1 - 1 as libc::c_int) as isize)).y as int32_t
            + 1 as libc::c_int;
    } else {
        if (*opt).flag & 0x10000000 as libc::c_int as libc::c_long == 0 {
            if is_splice != 0 {
                mm_fix_bad_ends_splice(
                    km,
                    opt,
                    mi,
                    r,
                    mat.as_mut_ptr() as *const int8_t,
                    qlen,
                    qseq0,
                    a,
                    &mut as1,
                    &mut cnt1,
                );
            } else {
                mm_fix_bad_ends(
                    r,
                    a,
                    (*opt).bw,
                    (*opt).min_chain_score * 2 as libc::c_int,
                    &mut as1,
                    &mut cnt1,
                );
            }
        } else {
            as1 = (*r).as_0;
            cnt1 = (*r).cnt;
        }
        mm_filter_bad_seeds(
            km,
            as1,
            cnt1,
            a,
            10 as libc::c_int,
            40 as libc::c_int,
            (*opt).max_gap >> 1 as libc::c_int,
            10 as libc::c_int,
        );
        mm_filter_bad_seeds_alt(
            km,
            as1,
            cnt1,
            a,
            30 as libc::c_int,
            (*opt).max_gap >> 1 as libc::c_int,
        );
        mm_adjust_minier(
            mi,
            qseq0 as *const *mut uint8_t,
            &mut *a.offset(as1 as isize),
            &mut rs,
            &mut qs,
        );
        mm_adjust_minier(
            mi,
            qseq0 as *const *mut uint8_t,
            &mut *a.offset((as1 + cnt1 - 1 as libc::c_int) as isize),
            &mut re,
            &mut qe,
        );
    }
    if cnt1 > 0 as libc::c_int {} else {
        __assert_fail(
            b"cnt1 > 0\0" as *const u8 as *const libc::c_char,
            b"align.c\0" as *const u8 as *const libc::c_char,
            610 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 140],
                &[libc::c_char; 140],
            >(
                b"void mm_align1(void *, const mm_mapopt_t *, const mm_idx_t *, int, uint8_t **, mm_reg1_t *, mm_reg1_t *, int, mm128_t *, ksw_extz_t *, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_14413: {
        if cnt1 > 0 as libc::c_int {} else {
            __assert_fail(
                b"cnt1 > 0\0" as *const u8 as *const libc::c_char,
                b"align.c\0" as *const u8 as *const libc::c_char,
                610 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 140],
                    &[libc::c_char; 140],
                >(
                    b"void mm_align1(void *, const mm_mapopt_t *, const mm_idx_t *, int, uint8_t **, mm_reg1_t *, mm_reg1_t *, int, mm128_t *, ksw_extz_t *, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if is_splice != 0 {
        if splice_flag & 0x100 as libc::c_int != 0 {
            extra_flag
                |= if rev != 0 { 0x200 as libc::c_int } else { 0x100 as libc::c_int };
        }
        if splice_flag & 0x200 as libc::c_int != 0 {
            extra_flag
                |= if rev != 0 { 0x100 as libc::c_int } else { 0x200 as libc::c_int };
        }
        if (*opt).flag & 0x40000 as libc::c_int as libc::c_long != 0 {
            extra_flag |= 0x400 as libc::c_int;
        }
    }
    if is_sr != 0 {
        qs0 = 0 as libc::c_int;
        qe0 = qlen;
        l = qs;
        l
            += if l * (*opt).a + (*opt).end_bonus > (*opt).q {
                (l * (*opt).a + (*opt).end_bonus - (*opt).q) / (*opt).e
            } else {
                0 as libc::c_int
            };
        rs0 = if rs - l > 0 as libc::c_int { rs - l } else { 0 as libc::c_int };
        l = qlen - qe;
        l
            += if l * (*opt).a + (*opt).end_bonus > (*opt).q {
                (l * (*opt).a + (*opt).end_bonus - (*opt).q) / (*opt).e
            } else {
                0 as libc::c_int
            };
        re0 = (if re + l < (*((*mi).seq).offset(rid as isize)).len as int32_t {
            (re + l) as libc::c_uint
        } else {
            (*((*mi).seq).offset(rid as isize)).len
        }) as int32_t;
    } else {
        rs0 = (*a.offset((*r).as_0 as isize)).x as int32_t + 1 as libc::c_int
            - ((*a.offset((*r).as_0 as isize)).y >> 32 as libc::c_int
                & 0xff as libc::c_int as libc::c_ulong) as int32_t;
        qs0 = (*a.offset((*r).as_0 as isize)).y as int32_t + 1 as libc::c_int
            - ((*a.offset((*r).as_0 as isize)).y >> 32 as libc::c_int
                & 0xff as libc::c_int as libc::c_ulong) as int32_t;
        if rs0 < 0 as libc::c_int {
            rs0 = 0 as libc::c_int;
        }
        if qs0 >= 0 as libc::c_int {} else {
            __assert_fail(
                b"qs0 >= 0\0" as *const u8 as *const libc::c_char,
                b"align.c\0" as *const u8 as *const libc::c_char,
                636 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 140],
                    &[libc::c_char; 140],
                >(
                    b"void mm_align1(void *, const mm_mapopt_t *, const mm_idx_t *, int, uint8_t **, mm_reg1_t *, mm_reg1_t *, int, mm128_t *, ksw_extz_t *, int)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_14083: {
            if qs0 >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"qs0 >= 0\0" as *const u8 as *const libc::c_char,
                    b"align.c\0" as *const u8 as *const libc::c_char,
                    636 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 140],
                        &[libc::c_char; 140],
                    >(
                        b"void mm_align1(void *, const mm_mapopt_t *, const mm_idx_t *, int, uint8_t **, mm_reg1_t *, mm_reg1_t *, int, mm128_t *, ksw_extz_t *, int)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        qs1 = 0 as libc::c_int;
        rs1 = qs1;
        i = (*r).as_0 - 1 as libc::c_int;
        l = 0 as libc::c_int;
        while i >= 0 as libc::c_int
            && (*a.offset(i as isize)).x >> 32 as libc::c_int
                == (*a.offset((*r).as_0 as isize)).x >> 32 as libc::c_int
        {
            let mut x: int32_t = (*a.offset(i as isize)).x as int32_t + 1 as libc::c_int
                - ((*a.offset(i as isize)).y >> 32 as libc::c_int
                    & 0xff as libc::c_int as libc::c_ulong) as int32_t;
            let mut y: int32_t = (*a.offset(i as isize)).y as int32_t + 1 as libc::c_int
                - ((*a.offset(i as isize)).y >> 32 as libc::c_int
                    & 0xff as libc::c_int as libc::c_ulong) as int32_t;
            if x < rs0 && y < qs0 {
                l += 1;
                if l > (*opt).min_cnt {
                    l = if rs0 - x > qs0 - y { rs0 - x } else { qs0 - y };
                    rs1 = rs0 - l;
                    qs1 = qs0 - l;
                    if rs1 < 0 as libc::c_int {
                        rs1 = 0 as libc::c_int;
                    }
                    break;
                }
            }
            i -= 1;
            i;
        }
        if qs > 0 as libc::c_int && rs > 0 as libc::c_int {
            l = if qs < (*opt).max_gap { qs } else { (*opt).max_gap };
            qs1 = if qs1 > qs - l { qs1 } else { qs - l };
            qs0 = if qs0 < qs1 { qs0 } else { qs1 };
            l
                += if l * (*opt).a > (*opt).q {
                    (l * (*opt).a - (*opt).q) / (*opt).e
                } else {
                    0 as libc::c_int
                };
            l = if l < (*opt).max_gap { l } else { (*opt).max_gap };
            l = if l < rs { l } else { rs };
            rs1 = if rs1 > rs - l { rs1 } else { rs - l };
            rs0 = if rs0 < rs1 { rs0 } else { rs1 };
            rs0 = if rs0 < rs { rs0 } else { rs };
        } else {
            rs0 = rs;
            qs0 = qs;
        }
        re0 = (*a.offset(((*r).as_0 + (*r).cnt - 1 as libc::c_int) as isize)).x
            as int32_t + 1 as libc::c_int;
        qe0 = (*a.offset(((*r).as_0 + (*r).cnt - 1 as libc::c_int) as isize)).y
            as int32_t + 1 as libc::c_int;
        re1 = (*((*mi).seq).offset(rid as isize)).len as int32_t;
        qe1 = qlen;
        i = (*r).as_0 + (*r).cnt;
        l = 0 as libc::c_int;
        while i < n_a
            && (*a.offset(i as isize)).x >> 32 as libc::c_int
                == (*a.offset((*r).as_0 as isize)).x >> 32 as libc::c_int
        {
            let mut x_0: int32_t = (*a.offset(i as isize)).x as int32_t
                + 1 as libc::c_int;
            let mut y_0: int32_t = (*a.offset(i as isize)).y as int32_t
                + 1 as libc::c_int;
            if x_0 > re0 && y_0 > qe0 {
                l += 1;
                if l > (*opt).min_cnt {
                    l = if x_0 - re0 > y_0 - qe0 { x_0 - re0 } else { y_0 - qe0 };
                    re1 = re0 + l;
                    qe1 = qe0 + l;
                    break;
                }
            }
            i += 1;
            i;
        }
        if qe < qlen && re < (*((*mi).seq).offset(rid as isize)).len as int32_t {
            l = if qlen - qe < (*opt).max_gap { qlen - qe } else { (*opt).max_gap };
            qe1 = if qe1 < qe + l { qe1 } else { qe + l };
            qe0 = if qe0 > qe1 { qe0 } else { qe1 };
            l
                += if l * (*opt).a > (*opt).q {
                    (l * (*opt).a - (*opt).q) / (*opt).e
                } else {
                    0 as libc::c_int
                };
            l = if l < (*opt).max_gap { l } else { (*opt).max_gap };
            l = (if l < (*((*mi).seq).offset(rid as isize)).len as int32_t - re {
                l as libc::c_uint
            } else {
                ((*((*mi).seq).offset(rid as isize)).len)
                    .wrapping_sub(re as libc::c_uint)
            }) as int32_t;
            re1 = if re1 < re + l { re1 } else { re + l };
            re0 = if re0 > re1 { re0 } else { re1 };
        } else {
            re0 = re;
            qe0 = qe;
        }
    }
    if (*a.offset((*r).as_0 as isize)).y as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 43 as libc::c_int != 0
    {
        let mut max_ext: libc::c_int = if (*r).qs > (*r).rs {
            (*r).qs - (*r).rs
        } else {
            (*r).rs - (*r).qs
        };
        if (*r).rs - rs0 > max_ext {
            rs0 = (*r).rs - max_ext;
        }
        if (*r).qs - qs0 > max_ext {
            qs0 = (*r).qs - max_ext;
        }
        max_ext = if (*r).qe > (*r).re { (*r).qe - (*r).re } else { (*r).re - (*r).qe };
        if re0 - (*r).re > max_ext {
            re0 = (*r).re + max_ext;
        }
        if qe0 - (*r).qe > max_ext {
            qe0 = (*r).qe + max_ext;
        }
    }
    if re0 > rs0 {} else {
        __assert_fail(
            b"re0 > rs0\0" as *const u8 as *const libc::c_char,
            b"align.c\0" as *const u8 as *const libc::c_char,
            696 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 140],
                &[libc::c_char; 140],
            >(
                b"void mm_align1(void *, const mm_mapopt_t *, const mm_idx_t *, int, uint8_t **, mm_reg1_t *, mm_reg1_t *, int, mm128_t *, ksw_extz_t *, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_13114: {
        if re0 > rs0 {} else {
            __assert_fail(
                b"re0 > rs0\0" as *const u8 as *const libc::c_char,
                b"align.c\0" as *const u8 as *const libc::c_char,
                696 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 140],
                    &[libc::c_char; 140],
                >(
                    b"void mm_align1(void *, const mm_mapopt_t *, const mm_idx_t *, int, uint8_t **, mm_reg1_t *, mm_reg1_t *, int, mm128_t *, ksw_extz_t *, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    tseq = kmalloc(km, (re0 - rs0) as size_t) as *mut uint8_t;
    junc = kmalloc(km, (re0 - rs0) as size_t) as *mut uint8_t;
    if qs > 0 as libc::c_int && rs > 0 as libc::c_int {
        if (*opt).flag as libc::c_longlong & 0x100000000 as libc::c_longlong != 0 {
            qseq = &mut *(*qseq0.offset(0 as libc::c_int as isize)).offset(qs0 as isize)
                as *mut uint8_t;
            mm_idx_getseq2(
                mi,
                rev,
                rid as uint32_t,
                rs0 as uint32_t,
                rs as uint32_t,
                tseq,
            );
        } else {
            qseq = &mut *(*qseq0.offset(rev as isize)).offset(qs0 as isize)
                as *mut uint8_t;
            mm_idx_getseq(mi, rid as uint32_t, rs0 as uint32_t, rs as uint32_t, tseq);
        }
        mm_idx_bed_junc(mi, rid, rs0, rs, junc);
        mm_seq_rev((qs - qs0) as uint32_t, qseq);
        mm_seq_rev((rs - rs0) as uint32_t, tseq);
        mm_seq_rev((rs - rs0) as uint32_t, junc);
        mm_align_pair(
            km,
            opt,
            qs - qs0,
            qseq,
            rs - rs0,
            tseq,
            junc,
            mat.as_mut_ptr(),
            bw,
            (*opt).end_bonus,
            if (*r).split_inv() as libc::c_int != 0 {
                (*opt).zdrop_inv
            } else {
                (*opt).zdrop
            },
            extra_flag | 0x40 as libc::c_int | 0x2 as libc::c_int | 0x80 as libc::c_int,
            ez,
        );
        if (*ez).n_cigar > 0 as libc::c_int {
            mm_append_cigar(r, (*ez).n_cigar as uint32_t, (*ez).cigar);
            (*(*r).p).dp_score += (*ez).max() as libc::c_int;
        }
        rs1 = rs
            - (if (*ez).reach_end != 0 {
                (*ez).mqe_t + 1 as libc::c_int
            } else {
                (*ez).max_t + 1 as libc::c_int
            });
        qs1 = qs
            - (if (*ez).reach_end != 0 {
                qs - qs0
            } else {
                (*ez).max_q + 1 as libc::c_int
            });
        mm_seq_rev((qs - qs0) as uint32_t, qseq);
    } else {
        rs1 = rs;
        qs1 = qs;
    }
    re1 = rs;
    qe1 = qs;
    if qs1 >= 0 as libc::c_int && rs1 >= 0 as libc::c_int {} else {
        __assert_fail(
            b"qs1 >= 0 && rs1 >= 0\0" as *const u8 as *const libc::c_char,
            b"align.c\0" as *const u8 as *const libc::c_char,
            722 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 140],
                &[libc::c_char; 140],
            >(
                b"void mm_align1(void *, const mm_mapopt_t *, const mm_idx_t *, int, uint8_t **, mm_reg1_t *, mm_reg1_t *, int, mm128_t *, ksw_extz_t *, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_12731: {
        if qs1 >= 0 as libc::c_int && rs1 >= 0 as libc::c_int {} else {
            __assert_fail(
                b"qs1 >= 0 && rs1 >= 0\0" as *const u8 as *const libc::c_char,
                b"align.c\0" as *const u8 as *const libc::c_char,
                722 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 140],
                    &[libc::c_char; 140],
                >(
                    b"void mm_align1(void *, const mm_mapopt_t *, const mm_idx_t *, int, uint8_t **, mm_reg1_t *, mm_reg1_t *, int, mm128_t *, ksw_extz_t *, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    i = if is_sr != 0 { cnt1 - 1 as libc::c_int } else { 1 as libc::c_int };
    while i < cnt1 {
        if !((*a.offset((as1 + i) as isize)).y as libc::c_ulonglong
            & ((1 as libc::c_ulonglong) << 41 as libc::c_int
                | (1 as libc::c_ulonglong) << 42 as libc::c_int) != 0
            && i != cnt1 - 1 as libc::c_int)
        {
            if is_sr != 0 && (*mi).flag & 0x1 as libc::c_int == 0 {
                re = (*a.offset((as1 + i) as isize)).x as int32_t + 1 as libc::c_int;
                qe = (*a.offset((as1 + i) as isize)).y as int32_t + 1 as libc::c_int;
            } else {
                mm_adjust_minier(
                    mi,
                    qseq0 as *const *mut uint8_t,
                    &mut *a.offset((as1 + i) as isize),
                    &mut re,
                    &mut qe,
                );
            }
            re1 = re;
            qe1 = qe;
            if i == cnt1 - 1 as libc::c_int
                || (*a.offset((as1 + i) as isize)).y as libc::c_ulonglong
                    & (1 as libc::c_ulonglong) << 40 as libc::c_int != 0
                || qe - qs >= (*opt).min_ksw_len && re - rs >= (*opt).min_ksw_len
            {
                let mut j: libc::c_int = 0;
                let mut bw1: libc::c_int = bw_long;
                let mut zdrop_code: libc::c_int = 0;
                if (*a.offset((as1 + i) as isize)).y as libc::c_ulonglong
                    & (1 as libc::c_ulonglong) << 40 as libc::c_int != 0
                {
                    bw1 = if qe - qs > re - rs { qe - qs } else { re - rs };
                }
                if (*opt).flag as libc::c_longlong & 0x100000000 as libc::c_longlong != 0
                {
                    qseq = &mut *(*qseq0.offset(0 as libc::c_int as isize))
                        .offset(qs as isize) as *mut uint8_t;
                    mm_idx_getseq2(
                        mi,
                        rev,
                        rid as uint32_t,
                        rs as uint32_t,
                        re as uint32_t,
                        tseq,
                    );
                } else {
                    qseq = &mut *(*qseq0.offset(rev as isize)).offset(qs as isize)
                        as *mut uint8_t;
                    mm_idx_getseq(
                        mi,
                        rid as uint32_t,
                        rs as uint32_t,
                        re as uint32_t,
                        tseq,
                    );
                }
                mm_idx_bed_junc(mi, rid, rs, re, junc);
                if is_sr != 0 {
                    if qe - qs == re - rs {} else {
                        __assert_fail(
                            b"qe - qs == re - rs\0" as *const u8 as *const libc::c_char,
                            b"align.c\0" as *const u8 as *const libc::c_char,
                            745 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 140],
                                &[libc::c_char; 140],
                            >(
                                b"void mm_align1(void *, const mm_mapopt_t *, const mm_idx_t *, int, uint8_t **, mm_reg1_t *, mm_reg1_t *, int, mm128_t *, ksw_extz_t *, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_12086: {
                        if qe - qs == re - rs {} else {
                            __assert_fail(
                                b"qe - qs == re - rs\0" as *const u8 as *const libc::c_char,
                                b"align.c\0" as *const u8 as *const libc::c_char,
                                745 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 140],
                                    &[libc::c_char; 140],
                                >(
                                    b"void mm_align1(void *, const mm_mapopt_t *, const mm_idx_t *, int, uint8_t **, mm_reg1_t *, mm_reg1_t *, int, mm128_t *, ksw_extz_t *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    ksw_reset_extz(ez);
                    j = 0 as libc::c_int;
                    (*ez).score = 0 as libc::c_int;
                    while j < qe - qs {
                        if *qseq.offset(j as isize) as libc::c_int >= 4 as libc::c_int
                            || *tseq.offset(j as isize) as libc::c_int
                                >= 4 as libc::c_int
                        {
                            (*ez).score += (*opt).e2;
                        } else {
                            (*ez).score
                                += if *qseq.offset(j as isize) as libc::c_int
                                    == *tseq.offset(j as isize) as libc::c_int
                                {
                                    (*opt).a
                                } else {
                                    -(*opt).b
                                };
                        }
                        j += 1;
                        j;
                    }
                    (*ez)
                        .cigar = ksw_push_cigar(
                        km,
                        &mut (*ez).n_cigar,
                        &mut (*ez).m_cigar,
                        (*ez).cigar,
                        0 as libc::c_int as uint32_t,
                        qe - qs,
                    );
                } else {
                    mm_align_pair(
                        km,
                        opt,
                        qe - qs,
                        qseq,
                        re - rs,
                        tseq,
                        junc,
                        mat.as_mut_ptr(),
                        bw1,
                        -(1 as libc::c_int),
                        (*opt).zdrop,
                        extra_flag | 0x8 as libc::c_int,
                        ez,
                    );
                }
                zdrop_code = mm_test_zdrop(
                    km,
                    opt,
                    qseq,
                    tseq,
                    (*ez).n_cigar as uint32_t,
                    (*ez).cigar,
                    mat.as_mut_ptr(),
                );
                if zdrop_code != 0 as libc::c_int {
                    mm_align_pair(
                        km,
                        opt,
                        qe - qs,
                        qseq,
                        re - rs,
                        tseq,
                        junc,
                        mat.as_mut_ptr(),
                        bw1,
                        -(1 as libc::c_int),
                        if zdrop_code == 2 as libc::c_int {
                            (*opt).zdrop_inv
                        } else {
                            (*opt).zdrop
                        },
                        extra_flag,
                        ez,
                    );
                }
                if (*ez).n_cigar > 0 as libc::c_int {
                    mm_append_cigar(r, (*ez).n_cigar as uint32_t, (*ez).cigar);
                }
                if (*ez).zdropped() != 0 {
                    if ((*r).p).is_null() {
                        if (*ez).n_cigar == 0 as libc::c_int {} else {
                            __assert_fail(
                                b"ez->n_cigar == 0\0" as *const u8 as *const libc::c_char,
                                b"align.c\0" as *const u8 as *const libc::c_char,
                                763 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 140],
                                    &[libc::c_char; 140],
                                >(
                                    b"void mm_align1(void *, const mm_mapopt_t *, const mm_idx_t *, int, uint8_t **, mm_reg1_t *, mm_reg1_t *, int, mm128_t *, ksw_extz_t *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_10956: {
                            if (*ez).n_cigar == 0 as libc::c_int {} else {
                                __assert_fail(
                                    b"ez->n_cigar == 0\0" as *const u8 as *const libc::c_char,
                                    b"align.c\0" as *const u8 as *const libc::c_char,
                                    763 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 140],
                                        &[libc::c_char; 140],
                                    >(
                                        b"void mm_align1(void *, const mm_mapopt_t *, const mm_idx_t *, int, uint8_t **, mm_reg1_t *, mm_reg1_t *, int, mm128_t *, ksw_extz_t *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        let mut capacity: uint32_t = (::std::mem::size_of::<mm_extra_t>()
                            as libc::c_ulong)
                            .wrapping_div(4 as libc::c_int as libc::c_ulong) as uint32_t;
                        capacity = capacity.wrapping_sub(1);
                        capacity;
                        capacity |= capacity >> 1 as libc::c_int;
                        capacity |= capacity >> 2 as libc::c_int;
                        capacity |= capacity >> 4 as libc::c_int;
                        capacity |= capacity >> 8 as libc::c_int;
                        capacity |= capacity >> 16 as libc::c_int;
                        capacity = capacity.wrapping_add(1);
                        capacity;
                        (*r)
                            .p = calloc(
                            capacity as libc::c_ulong,
                            4 as libc::c_int as libc::c_ulong,
                        ) as *mut mm_extra_t;
                        (*(*r).p).capacity = capacity;
                    }
                    j = i - 1 as libc::c_int;
                    while j >= 0 as libc::c_int {
                        if (*a.offset((as1 + j) as isize)).x as int32_t
                            <= rs + (*ez).max_t
                        {
                            break;
                        }
                        j -= 1;
                        j;
                    }
                    dropped = 1 as libc::c_int;
                    if j < 0 as libc::c_int {
                        j = 0 as libc::c_int;
                    }
                    (*(*r).p).dp_score += (*ez).max() as libc::c_int;
                    re1 = rs + ((*ez).max_t + 1 as libc::c_int);
                    qe1 = qs + ((*ez).max_q + 1 as libc::c_int);
                    if cnt1 - (j + 1 as libc::c_int) >= (*opt).min_cnt {
                        mm_split_reg(
                            r,
                            r2,
                            as1 + j + 1 as libc::c_int - (*r).as_0,
                            qlen,
                            a,
                            ((*opt).flag as libc::c_longlong
                                & 0x100000000 as libc::c_longlong != 0) as libc::c_int,
                        );
                        if zdrop_code == 2 as libc::c_int {
                            (*r2).set_split_inv(1 as libc::c_int as uint32_t);
                        }
                    }
                    break;
                } else {
                    (*(*r).p).dp_score += (*ez).score;
                    rs = re;
                    qs = qe;
                }
            }
        }
        i += 1;
        i;
    }
    if dropped == 0 && qe < qe0 && re < re0 {
        if (*opt).flag as libc::c_longlong & 0x100000000 as libc::c_longlong != 0 {
            qseq = &mut *(*qseq0.offset(0 as libc::c_int as isize)).offset(qe as isize)
                as *mut uint8_t;
            mm_idx_getseq2(
                mi,
                rev,
                rid as uint32_t,
                re as uint32_t,
                re0 as uint32_t,
                tseq,
            );
        } else {
            qseq = &mut *(*qseq0.offset(rev as isize)).offset(qe as isize)
                as *mut uint8_t;
            mm_idx_getseq(mi, rid as uint32_t, re as uint32_t, re0 as uint32_t, tseq);
        }
        mm_idx_bed_junc(mi, rid, re, re0, junc);
        mm_align_pair(
            km,
            opt,
            qe0 - qe,
            qseq,
            re0 - re,
            tseq,
            junc,
            mat.as_mut_ptr(),
            bw,
            (*opt).end_bonus,
            (*opt).zdrop,
            extra_flag | 0x40 as libc::c_int,
            ez,
        );
        if (*ez).n_cigar > 0 as libc::c_int {
            mm_append_cigar(r, (*ez).n_cigar as uint32_t, (*ez).cigar);
            (*(*r).p).dp_score += (*ez).max() as libc::c_int;
        }
        re1 = re
            + (if (*ez).reach_end != 0 {
                (*ez).mqe_t + 1 as libc::c_int
            } else {
                (*ez).max_t + 1 as libc::c_int
            });
        qe1 = qe
            + (if (*ez).reach_end != 0 {
                qe0 - qe
            } else {
                (*ez).max_q + 1 as libc::c_int
            });
    }
    if qe1 <= qlen {} else {
        __assert_fail(
            b"qe1 <= qlen\0" as *const u8 as *const libc::c_char,
            b"align.c\0" as *const u8 as *const libc::c_char,
            804 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 140],
                &[libc::c_char; 140],
            >(
                b"void mm_align1(void *, const mm_mapopt_t *, const mm_idx_t *, int, uint8_t **, mm_reg1_t *, mm_reg1_t *, int, mm128_t *, ksw_extz_t *, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_10384: {
        if qe1 <= qlen {} else {
            __assert_fail(
                b"qe1 <= qlen\0" as *const u8 as *const libc::c_char,
                b"align.c\0" as *const u8 as *const libc::c_char,
                804 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 140],
                    &[libc::c_char; 140],
                >(
                    b"void mm_align1(void *, const mm_mapopt_t *, const mm_idx_t *, int, uint8_t **, mm_reg1_t *, mm_reg1_t *, int, mm128_t *, ksw_extz_t *, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    (*r).rs = rs1;
    (*r).re = re1;
    if rev == 0 || (*opt).flag as libc::c_longlong & 0x100000000 as libc::c_longlong != 0
    {
        (*r).qs = qs1;
        (*r).qe = qe1;
    } else {
        (*r).qs = qlen - qe1;
        (*r).qe = qlen - qs1;
    }
    if re1 - rs1 <= re0 - rs0 {} else {
        __assert_fail(
            b"re1 - rs1 <= re0 - rs0\0" as *const u8 as *const libc::c_char,
            b"align.c\0" as *const u8 as *const libc::c_char,
            810 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 140],
                &[libc::c_char; 140],
            >(
                b"void mm_align1(void *, const mm_mapopt_t *, const mm_idx_t *, int, uint8_t **, mm_reg1_t *, mm_reg1_t *, int, mm128_t *, ksw_extz_t *, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_10269: {
        if re1 - rs1 <= re0 - rs0 {} else {
            __assert_fail(
                b"re1 - rs1 <= re0 - rs0\0" as *const u8 as *const libc::c_char,
                b"align.c\0" as *const u8 as *const libc::c_char,
                810 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 140],
                    &[libc::c_char; 140],
                >(
                    b"void mm_align1(void *, const mm_mapopt_t *, const mm_idx_t *, int, uint8_t **, mm_reg1_t *, mm_reg1_t *, int, mm128_t *, ksw_extz_t *, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !((*r).p).is_null() {
        if (*opt).flag as libc::c_longlong & 0x100000000 as libc::c_longlong != 0 {
            mm_idx_getseq2(
                mi,
                (*r).rev() as libc::c_int,
                rid as uint32_t,
                rs1 as uint32_t,
                re1 as uint32_t,
                tseq,
            );
            qseq = &mut *(*qseq0.offset(0 as libc::c_int as isize)).offset(qs1 as isize)
                as *mut uint8_t;
        } else {
            mm_idx_getseq(mi, rid as uint32_t, rs1 as uint32_t, re1 as uint32_t, tseq);
            qseq = &mut *(*qseq0.offset((*r).rev() as isize)).offset(qs1 as isize)
                as *mut uint8_t;
        }
        mm_update_extra(
            r,
            qseq,
            tseq,
            mat.as_mut_ptr(),
            (*opt).q as int8_t,
            (*opt).e as int8_t,
            ((*opt).flag & 0x4000000 as libc::c_int as libc::c_long) as libc::c_int,
            ((*opt).flag & 0x1000 as libc::c_int as libc::c_long == 0) as libc::c_int,
        );
        if rev != 0 && (*(*r).p).trans_strand() as libc::c_int != 0 {
            (*(*r).p)
                .set_trans_strand(
                    (*(*r).p).trans_strand() ^ 3 as libc::c_int as uint32_t,
                );
        }
    }
    kfree(km, tseq as *mut libc::c_void);
    kfree(km, junc as *mut libc::c_void);
}
unsafe extern "C" fn mm_align1_inv(
    mut km: *mut libc::c_void,
    mut opt: *const mm_mapopt_t,
    mut mi: *const mm_idx_t,
    mut qlen: libc::c_int,
    mut qseq0: *mut *mut uint8_t,
    mut r1: *const mm_reg1_t,
    mut r2: *const mm_reg1_t,
    mut r_inv: *mut mm_reg1_t,
    mut ez: *mut ksw_extz_t,
) -> libc::c_int {
    let mut tl: libc::c_int = 0;
    let mut ql: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut q_off: libc::c_int = 0;
    let mut t_off: libc::c_int = 0;
    let mut tseq: *mut uint8_t = 0 as *mut uint8_t;
    let mut qseq: *mut uint8_t = 0 as *mut uint8_t;
    let mut mat: [int8_t; 25] = [0; 25];
    let mut qp: *mut libc::c_void = 0 as *mut libc::c_void;
    memset(
        r_inv as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mm_reg1_t>() as libc::c_ulong,
    );
    if (*r1).split() as libc::c_int & 1 as libc::c_int == 0
        || (*r2).split() as libc::c_int & 2 as libc::c_int == 0
    {
        return 0 as libc::c_int;
    }
    if (*r1).id != (*r1).parent && (*r1).parent != -(2 as libc::c_int) {
        return 0 as libc::c_int;
    }
    if (*r2).id != (*r2).parent && (*r2).parent != -(2 as libc::c_int) {
        return 0 as libc::c_int;
    }
    if (*r1).rid != (*r2).rid || (*r1).rev() as libc::c_int != (*r2).rev() as libc::c_int
    {
        return 0 as libc::c_int;
    }
    ql = if (*r1).rev() as libc::c_int != 0 {
        (*r1).qs - (*r2).qe
    } else {
        (*r2).qs - (*r1).qe
    };
    tl = (*r2).rs - (*r1).re;
    if ql < (*opt).min_chain_score || ql > (*opt).max_gap {
        return 0 as libc::c_int;
    }
    if tl < (*opt).min_chain_score || tl > (*opt).max_gap {
        return 0 as libc::c_int;
    }
    ksw_gen_simple_mat(
        5 as libc::c_int,
        mat.as_mut_ptr(),
        (*opt).a as int8_t,
        (*opt).b as int8_t,
        (*opt).sc_ambi as int8_t,
    );
    tseq = kmalloc(km, tl as size_t) as *mut uint8_t;
    mm_idx_getseq(
        mi,
        (*r1).rid as uint32_t,
        (*r1).re as uint32_t,
        (*r2).rs as uint32_t,
        tseq,
    );
    qseq = if (*r1).rev() as libc::c_int != 0 {
        &mut *(*qseq0.offset(0 as libc::c_int as isize)).offset((*r2).qe as isize)
            as *mut uint8_t
    } else {
        &mut *(*qseq0.offset(1 as libc::c_int as isize))
            .offset((qlen - (*r2).qs) as isize) as *mut uint8_t
    };
    mm_seq_rev(ql as uint32_t, qseq);
    mm_seq_rev(tl as uint32_t, tseq);
    qp = ksw_ll_qinit(
        km,
        2 as libc::c_int,
        ql,
        qseq,
        5 as libc::c_int,
        mat.as_mut_ptr(),
    );
    score = ksw_ll_i16(qp, tl, tseq, (*opt).q, (*opt).e, &mut q_off, &mut t_off);
    kfree(km, qp);
    mm_seq_rev(ql as uint32_t, qseq);
    mm_seq_rev(tl as uint32_t, tseq);
    if !(score < (*opt).min_dp_max) {
        q_off = ql - (q_off + 1 as libc::c_int);
        t_off = tl - (t_off + 1 as libc::c_int);
        mm_align_pair(
            km,
            opt,
            ql - q_off,
            qseq.offset(q_off as isize),
            tl - t_off,
            tseq.offset(t_off as isize),
            0 as *const uint8_t,
            mat.as_mut_ptr(),
            ((*opt).bw as libc::c_double * 1.5f64) as libc::c_int,
            -(1 as libc::c_int),
            (*opt).zdrop,
            0x40 as libc::c_int,
            ez,
        );
        if !((*ez).n_cigar == 0 as libc::c_int) {
            mm_append_cigar(r_inv, (*ez).n_cigar as uint32_t, (*ez).cigar);
            (*(*r_inv).p).dp_score = (*ez).max() as int32_t;
            (*r_inv).id = -(1 as libc::c_int);
            (*r_inv).parent = -(1 as libc::c_int);
            (*r_inv).set_inv(1 as libc::c_int as uint32_t);
            (*r_inv).set_rev(((*r1).rev() == 0) as libc::c_int as uint32_t);
            (*r_inv).rid = (*r1).rid;
            (*r_inv).div = -1.0f32;
            if (*r_inv).rev() as libc::c_int == 0 as libc::c_int {
                (*r_inv).qs = (*r2).qe + q_off;
                (*r_inv).qe = (*r_inv).qs + (*ez).max_q + 1 as libc::c_int;
            } else {
                (*r_inv).qe = (*r2).qs - q_off;
                (*r_inv).qs = (*r_inv).qe - ((*ez).max_q + 1 as libc::c_int);
            }
            (*r_inv).rs = (*r1).re + t_off;
            (*r_inv).re = (*r_inv).rs + (*ez).max_t + 1 as libc::c_int;
            mm_update_extra(
                r_inv,
                &mut *qseq.offset(q_off as isize),
                &mut *tseq.offset(t_off as isize),
                mat.as_mut_ptr(),
                (*opt).q as int8_t,
                (*opt).e as int8_t,
                ((*opt).flag & 0x4000000 as libc::c_int as libc::c_long) as libc::c_int,
                ((*opt).flag & 0x1000 as libc::c_int as libc::c_long == 0) as libc::c_int,
            );
            ret = 1 as libc::c_int;
        }
    }
    kfree(km, tseq as *mut libc::c_void);
    return ret;
}
#[inline]
unsafe extern "C" fn mm_insert_reg(
    mut r: *const mm_reg1_t,
    mut i: libc::c_int,
    mut n_regs: *mut libc::c_int,
    mut regs: *mut mm_reg1_t,
) -> *mut mm_reg1_t {
    regs = realloc(
        regs as *mut libc::c_void,
        ((*n_regs + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mm_reg1_t>() as libc::c_ulong),
    ) as *mut mm_reg1_t;
    if i + 1 as libc::c_int != *n_regs {
        memmove(
            &mut *regs.offset((i + 2 as libc::c_int) as isize) as *mut mm_reg1_t
                as *mut libc::c_void,
            &mut *regs.offset((i + 1 as libc::c_int) as isize) as *mut mm_reg1_t
                as *const libc::c_void,
            (::std::mem::size_of::<mm_reg1_t>() as libc::c_ulong)
                .wrapping_mul((*n_regs - i - 1 as libc::c_int) as libc::c_ulong),
        );
    }
    *regs.offset((i + 1 as libc::c_int) as isize) = *r;
    *n_regs += 1;
    *n_regs;
    return regs;
}
#[inline]
unsafe extern "C" fn mm_count_gaps(
    mut r: *const mm_reg1_t,
    mut n_gap_: *mut int32_t,
    mut n_gapo_: *mut int32_t,
) {
    let mut i: uint32_t = 0;
    let mut n_gapo: int32_t = 0 as libc::c_int;
    let mut n_gap: int32_t = 0 as libc::c_int;
    *n_gapo_ = -(1 as libc::c_int);
    *n_gap_ = *n_gapo_;
    if ((*r).p).is_null() {
        return;
    }
    i = 0 as libc::c_int as uint32_t;
    while i < (*(*r).p).n_cigar {
        let mut op: int32_t = (*((*(*r).p).cigar).as_mut_ptr().offset(i as isize)
            & 0xf as libc::c_int as libc::c_uint) as int32_t;
        let mut len: int32_t = (*((*(*r).p).cigar).as_mut_ptr().offset(i as isize)
            >> 4 as libc::c_int) as int32_t;
        if op == 1 as libc::c_int || op == 2 as libc::c_int {
            n_gapo += 1;
            n_gapo;
            n_gap += len;
        }
        i = i.wrapping_add(1);
        i;
    }
    *n_gap_ = n_gap;
    *n_gapo_ = n_gapo;
}
pub unsafe extern "C" fn mm_event_identity(mut r: *const mm_reg1_t) -> libc::c_double {
    let mut n_gap: int32_t = 0;
    let mut n_gapo: int32_t = 0;
    if ((*r).p).is_null() {
        return -1.0f32 as libc::c_double;
    }
    mm_count_gaps(r, &mut n_gap, &mut n_gapo);
    return (*r).mlen as libc::c_double
        / ((*r).blen + (*(*r).p).n_ambi() as libc::c_int - n_gap + n_gapo)
            as libc::c_double;
}
unsafe extern "C" fn mm_recal_max_dp(
    mut r: *const mm_reg1_t,
    mut b2: libc::c_double,
    mut match_sc: int32_t,
) -> int32_t {
    let mut i: uint32_t = 0;
    let mut n_gap: int32_t = 0 as libc::c_int;
    let mut n_gapo: int32_t = 0 as libc::c_int;
    let mut n_mis: int32_t = 0;
    let mut gap_cost: libc::c_double = 0.0f64;
    if ((*r).p).is_null() {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int as uint32_t;
    while i < (*(*r).p).n_cigar {
        let mut op: int32_t = (*((*(*r).p).cigar).as_mut_ptr().offset(i as isize)
            & 0xf as libc::c_int as libc::c_uint) as int32_t;
        let mut len: int32_t = (*((*(*r).p).cigar).as_mut_ptr().offset(i as isize)
            >> 4 as libc::c_int) as int32_t;
        if op == 1 as libc::c_int || op == 2 as libc::c_int {
            gap_cost
                += b2
                    + mg_log2((1.0f64 + len as libc::c_double) as libc::c_float)
                        as libc::c_double;
            n_gapo += 1;
            n_gapo;
            n_gap += len;
        }
        i = i.wrapping_add(1);
        i;
    }
    n_mis = (*r).blen + (*(*r).p).n_ambi() as libc::c_int - (*r).mlen - n_gap;
    return (match_sc as libc::c_double
        * ((*r).mlen as libc::c_double - b2 * n_mis as libc::c_double - gap_cost)
        + 0.499f64) as int32_t;
}
pub unsafe extern "C" fn mm_update_dp_max(
    mut qlen: libc::c_int,
    mut n_regs: libc::c_int,
    mut regs: *mut mm_reg1_t,
    mut frac: libc::c_float,
    mut a: libc::c_int,
    mut b: libc::c_int,
) {
    let mut max: int32_t = -(1 as libc::c_int);
    let mut max2: int32_t = -(1 as libc::c_int);
    let mut i: int32_t = 0;
    let mut max_i: int32_t = -(1 as libc::c_int);
    let mut div: libc::c_double = 0.;
    let mut b2: libc::c_double = 0.;
    if n_regs < 2 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < n_regs {
        let mut r: *mut mm_reg1_t = &mut *regs.offset(i as isize) as *mut mm_reg1_t;
        if !((*r).p).is_null() {
            if (*(*r).p).dp_max > max {
                max2 = max;
                max = (*(*r).p).dp_max;
                max_i = i;
            } else if (*(*r).p).dp_max > max2 {
                max2 = (*(*r).p).dp_max;
            }
        }
        i += 1;
        i;
    }
    if max_i < 0 as libc::c_int || max < 0 as libc::c_int || max2 < 0 as libc::c_int {
        return;
    }
    if (((*regs.offset(max_i as isize)).qe - (*regs.offset(max_i as isize)).qs)
        as libc::c_double) < qlen as libc::c_double * frac as libc::c_double
    {
        return;
    }
    if (max2 as libc::c_double) < max as libc::c_double * frac as libc::c_double {
        return;
    }
    div = 1.0f64 - mm_event_identity(&mut *regs.offset(max_i as isize));
    if div < 0.02f64 {
        div = 0.02f64;
    }
    b2 = 0.5f64 / div;
    if (b2 * a as libc::c_double) < b as libc::c_double {
        b2 = a as libc::c_double / b as libc::c_double;
    }
    i = 0 as libc::c_int;
    while i < n_regs {
        let mut r_0: *mut mm_reg1_t = &mut *regs.offset(i as isize) as *mut mm_reg1_t;
        if !((*r_0).p).is_null() {
            (*(*r_0).p).dp_max = mm_recal_max_dp(r_0, b2, a);
            if (*(*r_0).p).dp_max < 0 as libc::c_int {
                (*(*r_0).p).dp_max = 0 as libc::c_int;
            }
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn mm_align_skeleton(
    mut km: *mut libc::c_void,
    mut opt: *const mm_mapopt_t,
    mut mi: *const mm_idx_t,
    mut qlen: libc::c_int,
    mut qstr: *const libc::c_char,
    mut n_regs_: *mut libc::c_int,
    mut regs: *mut mm_reg1_t,
    mut a: *mut mm128_t,
) -> *mut mm_reg1_t {
    extern "C" {
        #[link_name = "seq_nt4_table"]
        static mut seq_nt4_table_0: [libc::c_uchar; 256];
    }
    let mut i: int32_t = 0;
    let mut n_regs: int32_t = *n_regs_;
    let mut n_a: int32_t = 0;
    let mut qseq0: [*mut uint8_t; 2] = [0 as *mut uint8_t; 2];
    let mut ez: ksw_extz_t = ksw_extz_t {
        max_zdropped: [0; 4],
        max_q: 0,
        max_t: 0,
        mqe: 0,
        mqe_t: 0,
        mte: 0,
        mte_q: 0,
        score: 0,
        m_cigar: 0,
        n_cigar: 0,
        reach_end: 0,
        cigar: 0 as *mut uint32_t,
    };
    qseq0[0 as libc::c_int
        as usize] = kmalloc(km, (qlen * 2 as libc::c_int) as size_t) as *mut uint8_t;
    qseq0[1 as libc::c_int
        as usize] = (qseq0[0 as libc::c_int as usize]).offset(qlen as isize);
    i = 0 as libc::c_int;
    while i < qlen {
        *(qseq0[0 as libc::c_int as usize])
            .offset(
                i as isize,
            ) = seq_nt4_table[*qstr.offset(i as isize) as uint8_t as usize];
        *(qseq0[1 as libc::c_int as usize])
            .offset(
                (qlen - 1 as libc::c_int - i) as isize,
            ) = (if (*(qseq0[0 as libc::c_int as usize]).offset(i as isize)
            as libc::c_int) < 4 as libc::c_int
        {
            3 as libc::c_int
                - *(qseq0[0 as libc::c_int as usize]).offset(i as isize) as libc::c_int
        } else {
            4 as libc::c_int
        }) as uint8_t;
        i += 1;
        i;
    }
    n_a = mm_squeeze_a(km, n_regs, regs, a);
    memset(
        &mut ez as *mut ksw_extz_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ksw_extz_t>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < n_regs {
        let mut r2: mm_reg1_t = mm_reg1_t {
            id: 0,
            cnt: 0,
            rid: 0,
            score: 0,
            qs: 0,
            qe: 0,
            rs: 0,
            re: 0,
            parent: 0,
            subsc: 0,
            as_0: 0,
            mlen: 0,
            blen: 0,
            n_sub: 0,
            score0: 0,
            mapq_split_rev_inv_sam_pri_proper_frag_pe_thru_seg_split_seg_id_split_inv_is_alt_strand_retained_dummy: [0; 4],
            hash: 0,
            div: 0.,
            p: 0 as *mut mm_extra_t,
        };
        if (*opt).flag & 0x80 as libc::c_int as libc::c_long != 0
            && (*opt).flag & 0x100 as libc::c_int as libc::c_long != 0
            && (*opt).flag & 0x200 as libc::c_int as libc::c_long != 0
        {
            let mut s: [mm_reg1_t; 2] = [mm_reg1_t {
                id: 0,
                cnt: 0,
                rid: 0,
                score: 0,
                qs: 0,
                qe: 0,
                rs: 0,
                re: 0,
                parent: 0,
                subsc: 0,
                as_0: 0,
                mlen: 0,
                blen: 0,
                n_sub: 0,
                score0: 0,
                mapq_split_rev_inv_sam_pri_proper_frag_pe_thru_seg_split_seg_id_split_inv_is_alt_strand_retained_dummy: [0; 4],
                hash: 0,
                div: 0.,
                p: 0 as *mut mm_extra_t,
            }; 2];
            let mut s2: [mm_reg1_t; 2] = [mm_reg1_t {
                id: 0,
                cnt: 0,
                rid: 0,
                score: 0,
                qs: 0,
                qe: 0,
                rs: 0,
                re: 0,
                parent: 0,
                subsc: 0,
                as_0: 0,
                mlen: 0,
                blen: 0,
                n_sub: 0,
                score0: 0,
                mapq_split_rev_inv_sam_pri_proper_frag_pe_thru_seg_split_seg_id_split_inv_is_alt_strand_retained_dummy: [0; 4],
                hash: 0,
                div: 0.,
                p: 0 as *mut mm_extra_t,
            }; 2];
            let mut which: libc::c_int = 0;
            let mut trans_strand: libc::c_int = 0;
            s[1 as libc::c_int as usize] = *regs.offset(i as isize);
            s[0 as libc::c_int as usize] = s[1 as libc::c_int as usize];
            mm_align1(
                km,
                opt,
                mi,
                qlen,
                qseq0.as_mut_ptr(),
                &mut *s.as_mut_ptr().offset(0 as libc::c_int as isize),
                &mut *s2.as_mut_ptr().offset(0 as libc::c_int as isize),
                n_a,
                a,
                &mut ez,
                0x100 as libc::c_int,
            );
            mm_align1(
                km,
                opt,
                mi,
                qlen,
                qseq0.as_mut_ptr(),
                &mut *s.as_mut_ptr().offset(1 as libc::c_int as isize),
                &mut *s2.as_mut_ptr().offset(1 as libc::c_int as isize),
                n_a,
                a,
                &mut ez,
                0x200 as libc::c_int,
            );
            if (*s[0 as libc::c_int as usize].p).dp_score
                > (*s[1 as libc::c_int as usize].p).dp_score
            {
                which = 0 as libc::c_int;
                trans_strand = 1 as libc::c_int;
            } else if (*s[0 as libc::c_int as usize].p).dp_score
                < (*s[1 as libc::c_int as usize].p).dp_score
            {
                which = 1 as libc::c_int;
                trans_strand = 2 as libc::c_int;
            } else {
                trans_strand = 3 as libc::c_int;
                which = qlen + (*s[0 as libc::c_int as usize].p).dp_score
                    & 1 as libc::c_int;
            }
            if which == 0 as libc::c_int {
                *regs.offset(i as isize) = s[0 as libc::c_int as usize];
                r2 = s2[0 as libc::c_int as usize];
                free(s[1 as libc::c_int as usize].p as *mut libc::c_void);
            } else {
                *regs.offset(i as isize) = s[1 as libc::c_int as usize];
                r2 = s2[1 as libc::c_int as usize];
                free(s[0 as libc::c_int as usize].p as *mut libc::c_void);
            }
            let ref mut fresh16 = *(*regs.offset(i as isize)).p;
            (*fresh16).set_trans_strand(trans_strand as uint32_t);
        } else {
            mm_align1(
                km,
                opt,
                mi,
                qlen,
                qseq0.as_mut_ptr(),
                &mut *regs.offset(i as isize),
                &mut r2,
                n_a,
                a,
                &mut ez,
                (*opt).flag as libc::c_int,
            );
            if (*opt).flag & 0x80 as libc::c_int as libc::c_long != 0 {
                let ref mut fresh17 = *(*regs.offset(i as isize)).p;
                (*fresh17)
                    .set_trans_strand(
                        (if (*opt).flag & 0x100 as libc::c_int as libc::c_long != 0 {
                            1 as libc::c_int
                        } else {
                            2 as libc::c_int
                        }) as uint32_t,
                    );
            }
        }
        if r2.cnt > 0 as libc::c_int {
            regs = mm_insert_reg(&mut r2, i, &mut n_regs, regs);
        }
        if i > 0 as libc::c_int
            && (*regs.offset(i as isize)).split_inv() as libc::c_int != 0
            && (*opt).flag as libc::c_longlong & 0x200000000 as libc::c_longlong == 0
        {
            if mm_align1_inv(
                km,
                opt,
                mi,
                qlen,
                qseq0.as_mut_ptr(),
                &mut *regs.offset((i - 1 as libc::c_int) as isize),
                &mut *regs.offset(i as isize),
                &mut r2,
                &mut ez,
            ) != 0
            {
                regs = mm_insert_reg(&mut r2, i, &mut n_regs, regs);
                i += 1;
                i;
            }
        }
        i += 1;
        i;
    }
    *n_regs_ = n_regs;
    kfree(km, qseq0[0 as libc::c_int as usize] as *mut libc::c_void);
    kfree(km, ez.cigar as *mut libc::c_void);
    mm_filter_regs(opt, qlen, n_regs_, regs);
    if (*opt).flag & 0x1000 as libc::c_int as libc::c_long == 0
        && ((*opt).split_prefix).is_null() && qlen >= (*opt).rank_min_len
    {
        mm_update_dp_max(qlen, *n_regs_, regs, (*opt).rank_frac, (*opt).a, (*opt).b);
        mm_filter_regs(opt, qlen, n_regs_, regs);
    }
    mm_hit_sort(km, n_regs_, regs, (*opt).alt_drop);
    return regs;
}
