use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type mm_idx_intv_s;
    pub type mm_idx_bucket_s;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut mm_verbose: libc::c_int;
    static mut mm_realtime0: libc::c_double;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn realtime() -> libc::c_double;
    fn cputime() -> libc::c_double;
    fn mm_idx_cal_max_occ(mi: *const mm_idx_t, f: libc::c_float) -> int32_t;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_idxopt_t {
    pub k: libc::c_short,
    pub w: libc::c_short,
    pub flag: libc::c_short,
    pub bucket_bits: libc::c_short,
    pub mini_batch_size: int64_t,
    pub batch_size: uint64_t,
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
pub unsafe extern "C" fn mm_idxopt_init(mut opt: *mut mm_idxopt_t) {
    memset(
        opt as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mm_idxopt_t>() as libc::c_ulong,
    );
    (*opt).k = 15 as libc::c_int as libc::c_short;
    (*opt).w = 10 as libc::c_int as libc::c_short;
    (*opt).flag = 0 as libc::c_int as libc::c_short;
    (*opt).bucket_bits = 14 as libc::c_int as libc::c_short;
    (*opt).mini_batch_size = 50000000 as libc::c_int as int64_t;
    (*opt).batch_size = 4000000000 as libc::c_ulonglong as uint64_t;
}
pub unsafe extern "C" fn mm_mapopt_init(mut opt: *mut mm_mapopt_t) {
    memset(
        opt as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mm_mapopt_t>() as libc::c_ulong,
    );
    (*opt).seed = 11 as libc::c_int;
    (*opt).mid_occ_frac = 2e-4f32;
    (*opt).min_mid_occ = 10 as libc::c_int;
    (*opt).max_mid_occ = 1000000 as libc::c_int;
    (*opt).sdust_thres = 0 as libc::c_int;
    (*opt).q_occ_frac = 0.01f32;
    (*opt).min_cnt = 3 as libc::c_int;
    (*opt).min_chain_score = 40 as libc::c_int;
    (*opt).bw = 500 as libc::c_int;
    (*opt).bw_long = 20000 as libc::c_int;
    (*opt).max_gap = 5000 as libc::c_int;
    (*opt).max_gap_ref = -(1 as libc::c_int);
    (*opt).max_chain_skip = 25 as libc::c_int;
    (*opt).max_chain_iter = 5000 as libc::c_int;
    (*opt).rmq_inner_dist = 1000 as libc::c_int;
    (*opt).rmq_size_cap = 100000 as libc::c_int;
    (*opt).rmq_rescue_size = 1000 as libc::c_int;
    (*opt).rmq_rescue_ratio = 0.1f32;
    (*opt).chain_gap_scale = 0.8f32;
    (*opt).chain_skip_scale = 0.0f32;
    (*opt).max_max_occ = 4095 as libc::c_int;
    (*opt).occ_dist = 500 as libc::c_int;
    (*opt).mask_level = 0.5f32;
    (*opt).mask_len = 2147483647 as libc::c_int;
    (*opt).pri_ratio = 0.8f32;
    (*opt).best_n = 5 as libc::c_int;
    (*opt).alt_drop = 0.15f32;
    (*opt).a = 2 as libc::c_int;
    (*opt).b = 4 as libc::c_int;
    (*opt).q = 4 as libc::c_int;
    (*opt).e = 2 as libc::c_int;
    (*opt).q2 = 24 as libc::c_int;
    (*opt).e2 = 1 as libc::c_int;
    (*opt).sc_ambi = 1 as libc::c_int;
    (*opt).zdrop = 400 as libc::c_int;
    (*opt).zdrop_inv = 200 as libc::c_int;
    (*opt).end_bonus = -(1 as libc::c_int);
    (*opt).min_dp_max = (*opt).min_chain_score * (*opt).a;
    (*opt).min_ksw_len = 200 as libc::c_int;
    (*opt).anchor_ext_len = 20 as libc::c_int;
    (*opt).anchor_ext_shift = 6 as libc::c_int;
    (*opt).max_clip_ratio = 1.0f32;
    (*opt).mini_batch_size = 500000000 as libc::c_int as int64_t;
    (*opt).max_sw_mat = 100000000 as libc::c_int as int64_t;
    (*opt).cap_kalloc = 1000000000 as libc::c_int as int64_t;
    (*opt).rank_min_len = 500 as libc::c_int;
    (*opt).rank_frac = 0.9f32;
    (*opt).pe_ori = 0 as libc::c_int;
    (*opt).pe_bonus = 33 as libc::c_int;
}
pub unsafe extern "C" fn mm_mapopt_update(
    mut opt: *mut mm_mapopt_t,
    mut mi: *const mm_idx_t,
) {
    if (*opt).flag & 0x100 as libc::c_int as libc::c_long != 0
        || (*opt).flag & 0x200 as libc::c_int as libc::c_long != 0
    {
        (*opt).flag |= 0x80 as libc::c_int as libc::c_long;
    }
    if (*opt).mid_occ <= 0 as libc::c_int {
        (*opt).mid_occ = mm_idx_cal_max_occ(mi, (*opt).mid_occ_frac);
        if (*opt).mid_occ < (*opt).min_mid_occ {
            (*opt).mid_occ = (*opt).min_mid_occ;
        }
        if (*opt).max_mid_occ > (*opt).min_mid_occ && (*opt).mid_occ > (*opt).max_mid_occ
        {
            (*opt).mid_occ = (*opt).max_mid_occ;
        }
    }
    if (*opt).bw_long < (*opt).bw {
        (*opt).bw_long = (*opt).bw;
    }
    if mm_verbose >= 3 as libc::c_int {
        fprintf(
            stderr,
            b"[M::%s::%.3f*%.2f] mid_occ = %d\n\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"mm_mapopt_update\0"))
                .as_ptr(),
            realtime() - mm_realtime0,
            cputime() / (realtime() - mm_realtime0),
            (*opt).mid_occ,
        );
    }
}
pub unsafe extern "C" fn mm_mapopt_max_intron_len(
    mut opt: *mut mm_mapopt_t,
    mut max_intron_len: libc::c_int,
) {
    if (*opt).flag & 0x80 as libc::c_int as libc::c_long != 0
        && max_intron_len > 0 as libc::c_int
    {
        (*opt).bw_long = max_intron_len;
        (*opt).bw = (*opt).bw_long;
        (*opt).max_gap_ref = (*opt).bw;
    }
}
pub unsafe extern "C" fn mm_set_opt(
    mut preset: *const libc::c_char,
    mut io: *mut mm_idxopt_t,
    mut mo: *mut mm_mapopt_t,
) -> libc::c_int {
    if preset.is_null() {
        mm_idxopt_init(io);
        mm_mapopt_init(mo);
    } else if !(strcmp(preset, b"map-ont\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int)
    {
        if strcmp(preset, b"ava-ont\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*io).flag = 0 as libc::c_int as libc::c_short;
            (*io).k = 15 as libc::c_int as libc::c_short;
            (*io).w = 5 as libc::c_int as libc::c_short;
            (*mo).flag
                |= (0x800000 as libc::c_int | 0x1 as libc::c_int | 0x2 as libc::c_int
                    | 0x400 as libc::c_int) as libc::c_long;
            (*mo).min_chain_score = 100 as libc::c_int;
            (*mo).pri_ratio = 0.0f32;
            (*mo).max_chain_skip = 25 as libc::c_int;
            (*mo).bw_long = 2000 as libc::c_int;
            (*mo).bw = (*mo).bw_long;
            (*mo).occ_dist = 0 as libc::c_int;
        } else if strcmp(preset, b"map10k\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(preset, b"map-pb\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            (*io)
                .flag = ((*io).flag as libc::c_int | 0x1 as libc::c_int)
                as libc::c_short;
            (*io).k = 19 as libc::c_int as libc::c_short;
        } else if strcmp(preset, b"ava-pb\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*io)
                .flag = ((*io).flag as libc::c_int | 0x1 as libc::c_int)
                as libc::c_short;
            (*io).k = 19 as libc::c_int as libc::c_short;
            (*io).w = 5 as libc::c_int as libc::c_short;
            (*mo).flag
                |= (0x800000 as libc::c_int | 0x1 as libc::c_int | 0x2 as libc::c_int
                    | 0x400 as libc::c_int) as libc::c_long;
            (*mo).min_chain_score = 100 as libc::c_int;
            (*mo).pri_ratio = 0.0f32;
            (*mo).max_chain_skip = 25 as libc::c_int;
            (*mo).bw_long = (*mo).bw;
            (*mo).occ_dist = 0 as libc::c_int;
        } else if strcmp(preset, b"map-hifi\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(preset, b"map-ccs\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            (*io).flag = 0 as libc::c_int as libc::c_short;
            (*io).k = 19 as libc::c_int as libc::c_short;
            (*io).w = 19 as libc::c_int as libc::c_short;
            (*mo).max_gap = 10000 as libc::c_int;
            (*mo).a = 1 as libc::c_int;
            (*mo).b = 4 as libc::c_int;
            (*mo).q = 6 as libc::c_int;
            (*mo).q2 = 26 as libc::c_int;
            (*mo).e = 2 as libc::c_int;
            (*mo).e2 = 1 as libc::c_int;
            (*mo).occ_dist = 500 as libc::c_int;
            (*mo).min_mid_occ = 50 as libc::c_int;
            (*mo).max_mid_occ = 500 as libc::c_int;
            (*mo).min_dp_max = 200 as libc::c_int;
        } else if strncmp(
            preset,
            b"asm\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            (*io).flag = 0 as libc::c_int as libc::c_short;
            (*io).k = 19 as libc::c_int as libc::c_short;
            (*io).w = 19 as libc::c_int as libc::c_short;
            (*mo).bw = 1000 as libc::c_int;
            (*mo).bw_long = 100000 as libc::c_int;
            (*mo).max_gap = 10000 as libc::c_int;
            (*mo)
                .flag = ((*mo).flag as libc::c_longlong | 0x80000000 as libc::c_longlong)
                as int64_t;
            (*mo).min_mid_occ = 50 as libc::c_int;
            (*mo).max_mid_occ = 500 as libc::c_int;
            (*mo).min_dp_max = 200 as libc::c_int;
            (*mo).best_n = 50 as libc::c_int;
            if strcmp(preset, b"asm5\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*mo).a = 1 as libc::c_int;
                (*mo).b = 19 as libc::c_int;
                (*mo).q = 39 as libc::c_int;
                (*mo).q2 = 81 as libc::c_int;
                (*mo).e = 3 as libc::c_int;
                (*mo).e2 = 1 as libc::c_int;
                (*mo).zdrop_inv = 200 as libc::c_int;
                (*mo).zdrop = (*mo).zdrop_inv;
            } else if strcmp(preset, b"asm10\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*mo).a = 1 as libc::c_int;
                (*mo).b = 9 as libc::c_int;
                (*mo).q = 16 as libc::c_int;
                (*mo).q2 = 41 as libc::c_int;
                (*mo).e = 2 as libc::c_int;
                (*mo).e2 = 1 as libc::c_int;
                (*mo).zdrop_inv = 200 as libc::c_int;
                (*mo).zdrop = (*mo).zdrop_inv;
            } else if strcmp(preset, b"asm20\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*mo).a = 1 as libc::c_int;
                (*mo).b = 4 as libc::c_int;
                (*mo).q = 6 as libc::c_int;
                (*mo).q2 = 26 as libc::c_int;
                (*mo).e = 2 as libc::c_int;
                (*mo).e2 = 1 as libc::c_int;
                (*mo).zdrop_inv = 200 as libc::c_int;
                (*mo).zdrop = (*mo).zdrop_inv;
                (*io).w = 10 as libc::c_int as libc::c_short;
            } else {
                return -(1 as libc::c_int)
            }
        } else if strcmp(preset, b"short\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(preset, b"sr\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            (*io).flag = 0 as libc::c_int as libc::c_short;
            (*io).k = 21 as libc::c_int as libc::c_short;
            (*io).w = 11 as libc::c_int as libc::c_short;
            (*mo).flag
                |= (0x1000 as libc::c_int | 0x2000 as libc::c_int | 0x4000 as libc::c_int
                    | 0x8000 as libc::c_int | 0x400000 as libc::c_int) as libc::c_long;
            (*mo).pe_ori = (0 as libc::c_int) << 1 as libc::c_int | 1 as libc::c_int;
            (*mo).a = 2 as libc::c_int;
            (*mo).b = 8 as libc::c_int;
            (*mo).q = 12 as libc::c_int;
            (*mo).e = 2 as libc::c_int;
            (*mo).q2 = 24 as libc::c_int;
            (*mo).e2 = 1 as libc::c_int;
            (*mo).zdrop_inv = 100 as libc::c_int;
            (*mo).zdrop = (*mo).zdrop_inv;
            (*mo).end_bonus = 10 as libc::c_int;
            (*mo).max_frag_len = 800 as libc::c_int;
            (*mo).max_gap = 100 as libc::c_int;
            (*mo).bw_long = 100 as libc::c_int;
            (*mo).bw = (*mo).bw_long;
            (*mo).pri_ratio = 0.5f32;
            (*mo).min_cnt = 2 as libc::c_int;
            (*mo).min_chain_score = 25 as libc::c_int;
            (*mo).min_dp_max = 40 as libc::c_int;
            (*mo).best_n = 20 as libc::c_int;
            (*mo).mid_occ = 1000 as libc::c_int;
            (*mo).max_occ = 5000 as libc::c_int;
            (*mo).mini_batch_size = 50000000 as libc::c_int as int64_t;
        } else if strncmp(
            preset,
            b"splice\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
            || strcmp(preset, b"cdna\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            (*io).flag = 0 as libc::c_int as libc::c_short;
            (*io).k = 15 as libc::c_int as libc::c_short;
            (*io).w = 5 as libc::c_int as libc::c_short;
            (*mo).flag
                |= (0x80 as libc::c_int | 0x100 as libc::c_int | 0x200 as libc::c_int
                    | 0x40000 as libc::c_int) as libc::c_long;
            (*mo).max_sw_mat = 0 as libc::c_int as int64_t;
            (*mo).max_gap = 2000 as libc::c_int;
            (*mo).bw_long = 200000 as libc::c_int;
            (*mo).bw = (*mo).bw_long;
            (*mo).max_gap_ref = (*mo).bw;
            (*mo).a = 1 as libc::c_int;
            (*mo).b = 2 as libc::c_int;
            (*mo).q = 2 as libc::c_int;
            (*mo).e = 1 as libc::c_int;
            (*mo).q2 = 32 as libc::c_int;
            (*mo).e2 = 0 as libc::c_int;
            (*mo).noncan = 9 as libc::c_int;
            (*mo).junc_bonus = 9 as libc::c_int;
            (*mo).zdrop = 200 as libc::c_int;
            (*mo).zdrop_inv = 100 as libc::c_int;
            if strcmp(preset, b"splice:hq\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*mo).junc_bonus = 5 as libc::c_int;
                (*mo).b = 4 as libc::c_int;
                (*mo).q = 6 as libc::c_int;
                (*mo).q2 = 24 as libc::c_int;
            }
        } else {
            return -(1 as libc::c_int)
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mm_check_opt(
    mut io: *const mm_idxopt_t,
    mut mo: *const mm_mapopt_t,
) -> libc::c_int {
    if (*mo).bw > (*mo).bw_long {
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR]\x1B[1;31m with '-rNUM1,NUM2', NUM1 (%d) can't be larger than NUM2 (%d)\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
                (*mo).bw,
                (*mo).bw_long,
            );
        }
        return -(8 as libc::c_int);
    }
    if (*mo).flag as libc::c_longlong & 0x80000000 as libc::c_longlong != 0
        && (*mo).flag & (0x1000 as libc::c_int | 0x80 as libc::c_int) as libc::c_long
            != 0
    {
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR]\x1B[1;31m --rmq doesn't work with --sr or --splice\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        return -(7 as libc::c_int);
    }
    if !((*mo).split_prefix).is_null()
        && (*mo).flag & (0x40 as libc::c_int | 0x1000000 as libc::c_int) as libc::c_long
            != 0
    {
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR]\x1B[1;31m --cs or --MD doesn't work with --split-prefix\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        return -(6 as libc::c_int);
    }
    if (*io).k as libc::c_int <= 0 as libc::c_int
        || (*io).w as libc::c_int <= 0 as libc::c_int
    {
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR]\x1B[1;31m -k and -w must be positive\x1B[0m\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return -(5 as libc::c_int);
    }
    if (*mo).best_n < 0 as libc::c_int {
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR]\x1B[1;31m -N must be no less than 0\x1B[0m\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return -(4 as libc::c_int);
    }
    if (*mo).best_n == 0 as libc::c_int && mm_verbose >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"[WARNING]\x1B[1;31m '-N 0' reduces mapping accuracy. Please use '--secondary=no' instead.\x1B[0m\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if (*mo).pri_ratio < 0.0f32 || (*mo).pri_ratio > 1.0f32 {
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR]\x1B[1;31m -p must be within 0 and 1 (including 0 and 1)\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        return -(4 as libc::c_int);
    }
    if (*mo).flag & 0x100000 as libc::c_int as libc::c_long != 0
        && (*mo).flag & 0x200000 as libc::c_int as libc::c_long != 0
    {
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR]\x1B[1;31m --for-only and --rev-only can't be applied at the same time\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        return -(3 as libc::c_int);
    }
    if (*mo).e <= 0 as libc::c_int || (*mo).q <= 0 as libc::c_int {
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR]\x1B[1;31m -O and -E must be positive\x1B[0m\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return -(1 as libc::c_int);
    }
    if ((*mo).q != (*mo).q2 || (*mo).e != (*mo).e2)
        && !((*mo).e > (*mo).e2 && (*mo).q + (*mo).e < (*mo).q2 + (*mo).e2)
    {
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR]\x1B[1;31m dual gap penalties violating E1>E2 and O1+E1<O2+E2\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        return -(2 as libc::c_int);
    }
    if (*mo).q + (*mo).e + ((*mo).q2 + (*mo).e2) > 127 as libc::c_int {
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR]\x1B[1;31m scoring system violating ({-O}+{-E})+({-O2}+{-E2}) <= 127\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        return -(1 as libc::c_int);
    }
    if (*mo).zdrop < (*mo).zdrop_inv {
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR]\x1B[1;31m Z-drop should not be less than inversion-Z-drop\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        return -(5 as libc::c_int);
    }
    if (*mo).flag & 0x4000 as libc::c_int as libc::c_long != 0
        && (*mo).flag & 0x800000 as libc::c_int as libc::c_long != 0
    {
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR]\x1B[1;31m -X/-P and --secondary=no can't be applied at the same time\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        return -(5 as libc::c_int);
    }
    if (*mo).flag as libc::c_longlong & 0x100000000 as libc::c_longlong != 0
        && ((*mo).flag
            & (0x8 as libc::c_int | 0x80 as libc::c_int | 0x2000 as libc::c_int)
                as libc::c_long != 0
            || (*io).flag as libc::c_int & 0x1 as libc::c_int != 0)
    {
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR]\x1B[1;31m --qstrand doesn't work with -a, -H, --frag or --splice\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        return -(5 as libc::c_int);
    }
    return 0 as libc::c_int;
}
