use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type mm_idx_intv_s;
    pub type mm_idx_bucket_s;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut mm_verbose: libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub type int32_t = __int32_t;
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
#[inline]
unsafe extern "C" fn get_for_qpos(mut qlen: int32_t, mut a: *const mm128_t) -> int32_t {
    let mut x: int32_t = (*a).y as int32_t;
    let mut q_span: int32_t = ((*a).y >> 32 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as int32_t;
    if (*a).x >> 63 as libc::c_int != 0 {
        x = qlen - 1 as libc::c_int - (x + 1 as libc::c_int - q_span);
    }
    return x;
}
unsafe extern "C" fn get_mini_idx(
    mut qlen: libc::c_int,
    mut a: *const mm128_t,
    mut n: int32_t,
    mut mini_pos: *const uint64_t,
) -> libc::c_int {
    let mut x: int32_t = 0;
    let mut L: int32_t = 0 as libc::c_int;
    let mut R: int32_t = n - 1 as libc::c_int;
    x = get_for_qpos(qlen, a);
    while L <= R {
        let mut m: int32_t = ((L as uint64_t).wrapping_add(R as libc::c_ulong)
            >> 1 as libc::c_int) as int32_t;
        let mut y: int32_t = *mini_pos.offset(m as isize) as int32_t;
        if y < x {
            L = m + 1 as libc::c_int;
        } else if y > x {
            R = m - 1 as libc::c_int;
        } else {
            return m
        }
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn mm_est_err(
    mut mi: *const mm_idx_t,
    mut qlen: libc::c_int,
    mut n_regs: libc::c_int,
    mut regs: *mut mm_reg1_t,
    mut a: *const mm128_t,
    mut n: int32_t,
    mut mini_pos: *const uint64_t,
) {
    let mut i: libc::c_int = 0;
    let mut sum_k: uint64_t = 0 as libc::c_int as uint64_t;
    let mut avg_k: libc::c_float = 0.;
    if n == 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < n {
        sum_k = (sum_k as libc::c_ulong)
            .wrapping_add(
                *mini_pos.offset(i as isize) >> 32 as libc::c_int
                    & 0xff as libc::c_int as libc::c_ulong,
            ) as uint64_t as uint64_t;
        i += 1;
        i;
    }
    avg_k = sum_k as libc::c_float / n as libc::c_float;
    i = 0 as libc::c_int;
    while i < n_regs {
        let mut r: *mut mm_reg1_t = &mut *regs.offset(i as isize) as *mut mm_reg1_t;
        let mut st: int32_t = 0;
        let mut en: int32_t = 0;
        let mut j: int32_t = 0;
        let mut k: int32_t = 0;
        let mut n_match: int32_t = 0;
        let mut n_tot: int32_t = 0;
        let mut l_ref: int32_t = 0;
        (*r).div = -1.0f32;
        if !((*r).cnt == 0 as libc::c_int) {
            en = get_mini_idx(
                qlen,
                if (*r).rev() as libc::c_int != 0 {
                    &*a.offset(((*r).as_0 + (*r).cnt - 1 as libc::c_int) as isize)
                } else {
                    &*a.offset((*r).as_0 as isize)
                },
                n,
                mini_pos,
            );
            st = en;
            if st < 0 as libc::c_int {
                if mm_verbose >= 2 as libc::c_int {
                    fprintf(
                        stderr,
                        b"[WARNING] logic inconsistency in mm_est_err(). Please contact the developer.\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            } else {
                l_ref = (*((*mi).seq).offset((*r).rid as isize)).len as int32_t;
                k = 1 as libc::c_int;
                j = st + 1 as libc::c_int;
                n_match = 1 as libc::c_int;
                while j < n && k < (*r).cnt {
                    let mut x: int32_t = 0;
                    x = get_for_qpos(
                        qlen,
                        if (*r).rev() as libc::c_int != 0 {
                            &*a
                                .offset(
                                    ((*r).as_0 + (*r).cnt - 1 as libc::c_int - k) as isize,
                                )
                        } else {
                            &*a.offset(((*r).as_0 + k) as isize)
                        },
                    );
                    if x == *mini_pos.offset(j as isize) as int32_t {
                        k += 1;
                        k;
                        en = j;
                        n_match += 1;
                        n_match;
                    }
                    j += 1;
                    j;
                }
                n_tot = en - st + 1 as libc::c_int;
                if (*r).qs as libc::c_float > avg_k && (*r).rs as libc::c_float > avg_k {
                    n_tot += 1;
                    n_tot;
                }
                if (qlen - (*r).qs) as libc::c_float > avg_k
                    && (l_ref - (*r).re) as libc::c_float > avg_k
                {
                    n_tot += 1;
                    n_tot;
                }
                (*r)
                    .div = if n_match >= n_tot {
                    0.0f32
                } else {
                    (1.0f64
                        - pow(
                            n_match as libc::c_double / n_tot as libc::c_double,
                            1.0f64 / avg_k as libc::c_double,
                        )) as libc::c_float
                };
            }
        }
        i += 1;
        i;
    }
}
