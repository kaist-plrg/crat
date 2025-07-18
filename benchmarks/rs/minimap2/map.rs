use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type sdust_buf_s;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type mm_idx_intv_s;
    pub type mm_idx_bucket_s;
    pub type mm_bseq_file_s;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
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
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn kt_for(
        n_threads: libc::c_int,
        func: Option::<
            unsafe extern "C" fn(*mut libc::c_void, libc::c_long, libc::c_int) -> (),
        >,
        data: *mut libc::c_void,
        n: libc::c_long,
    );
    fn kt_pipeline(
        n_threads: libc::c_int,
        func: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                libc::c_int,
                *mut libc::c_void,
            ) -> *mut libc::c_void,
        >,
        shared_data: *mut libc::c_void,
        n_steps: libc::c_int,
    );
    fn kmalloc(km: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn kfree(km: *mut libc::c_void, ptr: *mut libc::c_void);
    fn km_init() -> *mut libc::c_void;
    fn km_destroy(km: *mut libc::c_void);
    fn km_stat(_km: *const libc::c_void, s: *mut km_stat_t);
    fn sdust_buf_init(km: *mut libc::c_void) -> *mut sdust_buf_t;
    fn sdust_buf_destroy(buf: *mut sdust_buf_t);
    fn sdust_core(
        seq: *const uint8_t,
        l_seq: libc::c_int,
        T: libc::c_int,
        W: libc::c_int,
        n: *mut libc::c_int,
        buf: *mut sdust_buf_t,
    ) -> *const uint64_t;
    fn cputime() -> libc::c_double;
    fn realtime() -> libc::c_double;
    fn radix_sort_128x(beg: *mut mm128_t, end: *mut mm128_t);
    fn mm_sketch(
        km: *mut libc::c_void,
        str: *const libc::c_char,
        len: libc::c_int,
        w: libc::c_int,
        k: libc::c_int,
        rid: uint32_t,
        is_hpc: libc::c_int,
        p: *mut mm128_v,
    );
    fn mm_collect_matches(
        km: *mut libc::c_void,
        _n_m: *mut libc::c_int,
        qlen: libc::c_int,
        max_occ: libc::c_int,
        max_max_occ: libc::c_int,
        dist: libc::c_int,
        mi: *const mm_idx_t,
        mv: *const mm128_v,
        n_a: *mut int64_t,
        rep_len: *mut libc::c_int,
        n_mini_pos: *mut libc::c_int,
        mini_pos: *mut *mut uint64_t,
    ) -> *mut mm_seed_t;
    fn mm_seed_mz_flt(
        km: *mut libc::c_void,
        mv: *mut mm128_v,
        q_occ_max: int32_t,
        q_occ_frac: libc::c_float,
    );
    fn mm_write_paf3(
        s: *mut kstring_t,
        mi: *const mm_idx_t,
        t: *const mm_bseq1_t,
        r: *const mm_reg1_t,
        km: *mut libc::c_void,
        opt_flag: int64_t,
        rep_len: libc::c_int,
    );
    fn mm_write_sam3(
        s: *mut kstring_t,
        mi: *const mm_idx_t,
        t: *const mm_bseq1_t,
        seg_idx: libc::c_int,
        reg_idx: libc::c_int,
        n_seg: libc::c_int,
        n_regss: *const libc::c_int,
        regss: *const *const mm_reg1_t,
        km: *mut libc::c_void,
        opt_flag: int64_t,
        rep_len: libc::c_int,
    );
    fn mm_align_skeleton(
        km: *mut libc::c_void,
        opt: *const mm_mapopt_t,
        mi: *const mm_idx_t,
        qlen: libc::c_int,
        qstr: *const libc::c_char,
        n_regs_: *mut libc::c_int,
        regs: *mut mm_reg1_t,
        a: *mut mm128_t,
    ) -> *mut mm_reg1_t;
    fn mm_gen_regs(
        km: *mut libc::c_void,
        hash: uint32_t,
        qlen: libc::c_int,
        n_u: libc::c_int,
        u: *mut uint64_t,
        a: *mut mm128_t,
        is_qstrand: libc::c_int,
    ) -> *mut mm_reg1_t;
    fn mg_lchain_dp(
        max_dist_x: libc::c_int,
        max_dist_y: libc::c_int,
        bw: libc::c_int,
        max_skip: libc::c_int,
        max_iter: libc::c_int,
        min_cnt: libc::c_int,
        min_sc: libc::c_int,
        chn_pen_gap: libc::c_float,
        chn_pen_skip: libc::c_float,
        is_cdna: libc::c_int,
        n_segs: libc::c_int,
        n: int64_t,
        a: *mut mm128_t,
        n_u_: *mut libc::c_int,
        _u: *mut *mut uint64_t,
        km: *mut libc::c_void,
    ) -> *mut mm128_t;
    fn mg_lchain_rmq(
        max_dist: libc::c_int,
        max_dist_inner: libc::c_int,
        bw: libc::c_int,
        max_chn_skip: libc::c_int,
        cap_rmq_size: libc::c_int,
        min_cnt: libc::c_int,
        min_sc: libc::c_int,
        chn_pen_gap: libc::c_float,
        chn_pen_skip: libc::c_float,
        n: int64_t,
        a: *mut mm128_t,
        n_u_: *mut libc::c_int,
        _u: *mut *mut uint64_t,
        km: *mut libc::c_void,
    ) -> *mut mm128_t;
    fn mm_mark_alt(mi: *const mm_idx_t, n: libc::c_int, r: *mut mm_reg1_t);
    fn mm_set_sam_pri(n: libc::c_int, r: *mut mm_reg1_t) -> libc::c_int;
    fn mm_set_parent(
        km: *mut libc::c_void,
        mask_level: libc::c_float,
        mask_len: libc::c_int,
        n: libc::c_int,
        r: *mut mm_reg1_t,
        sub_diff: libc::c_int,
        hard_mask_level: libc::c_int,
        alt_diff_frac: libc::c_float,
    );
    fn mm_select_sub(
        km: *mut libc::c_void,
        pri_ratio: libc::c_float,
        min_diff: libc::c_int,
        best_n: libc::c_int,
        check_strand: libc::c_int,
        min_strand_sc: libc::c_int,
        n_: *mut libc::c_int,
        r: *mut mm_reg1_t,
    );
    fn mm_select_sub_multi(
        km: *mut libc::c_void,
        pri_ratio: libc::c_float,
        pri1: libc::c_float,
        pri2: libc::c_float,
        max_gap_ref: libc::c_int,
        min_diff: libc::c_int,
        best_n: libc::c_int,
        n_segs: libc::c_int,
        qlens: *const libc::c_int,
        n_: *mut libc::c_int,
        r: *mut mm_reg1_t,
    );
    fn mm_filter_strand_retained(n_regs: libc::c_int, r: *mut mm_reg1_t) -> libc::c_int;
    fn mm_hit_sort(
        km: *mut libc::c_void,
        n_regs: *mut libc::c_int,
        r: *mut mm_reg1_t,
        alt_diff_frac: libc::c_float,
    );
    fn mm_set_mapq(
        km: *mut libc::c_void,
        n_regs: libc::c_int,
        regs: *mut mm_reg1_t,
        min_chain_sc: libc::c_int,
        match_sc: libc::c_int,
        rep_len: libc::c_int,
        is_sr: libc::c_int,
    );
    fn mm_update_dp_max(
        qlen: libc::c_int,
        n_regs: libc::c_int,
        regs: *mut mm_reg1_t,
        frac: libc::c_float,
        a: libc::c_int,
        b: libc::c_int,
    );
    fn mm_est_err(
        mi: *const mm_idx_t,
        qlen: libc::c_int,
        n_regs: libc::c_int,
        regs: *mut mm_reg1_t,
        a: *const mm128_t,
        n: int32_t,
        mini_pos: *const uint64_t,
    );
    fn mm_seg_gen(
        km: *mut libc::c_void,
        hash: uint32_t,
        n_segs: libc::c_int,
        qlens: *const libc::c_int,
        n_regs0: libc::c_int,
        regs0: *const mm_reg1_t,
        n_regs: *mut libc::c_int,
        regs: *mut *mut mm_reg1_t,
        a: *const mm128_t,
    ) -> *mut mm_seg_t;
    fn mm_seg_free(km: *mut libc::c_void, n_segs: libc::c_int, segs: *mut mm_seg_t);
    fn mm_pair(
        km: *mut libc::c_void,
        max_gap_ref: libc::c_int,
        dp_bonus: libc::c_int,
        sub_diff: libc::c_int,
        match_sc: libc::c_int,
        qlens: *const libc::c_int,
        n_regs: *mut libc::c_int,
        regs: *mut *mut mm_reg1_t,
    );
    fn mm_split_rm_tmp(prefix: *const libc::c_char, n_splits: libc::c_int);
    fn mm_split_merge_prep(
        prefix: *const libc::c_char,
        n_splits: libc::c_int,
        fp: *mut *mut FILE,
        n_seq_part: *mut uint32_t,
    ) -> *mut mm_idx_t;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn mm_bseq_close(fp: *mut mm_bseq_file_t);
    fn mm_err_puts(str: *const libc::c_char);
    static mut stderr: *mut FILE;
    fn mm_err_fwrite(
        p: *const libc::c_void,
        size: size_t,
        nitems: size_t,
        fp: *mut FILE,
    );
    static mut seq_comp_table: [libc::c_uchar; 256];
    fn mm_err_fread(p: *mut libc::c_void, size: size_t, nitems: size_t, fp: *mut FILE);
    fn mm_bseq_read3(
        fp: *mut mm_bseq_file_t,
        chunk_size: int64_t,
        with_qual: libc::c_int,
        with_comment: libc::c_int,
        frag_mode: libc::c_int,
        n_: *mut libc::c_int,
    ) -> *mut mm_bseq1_t;
    fn mm_bseq_read_frag2(
        n_fp: libc::c_int,
        fp: *mut *mut mm_bseq_file_t,
        chunk_size: int64_t,
        with_qual: libc::c_int,
        with_comment: libc::c_int,
        n_: *mut libc::c_int,
    ) -> *mut mm_bseq1_t;
    fn mm_split_init(prefix: *const libc::c_char, mi: *const mm_idx_t) -> *mut FILE;
    fn mm_bseq_open(fn_0: *const libc::c_char) -> *mut mm_bseq_file_t;
    static mut mm_verbose: libc::c_int;
    static mut mm_dbg_flag: libc::c_int;
    static mut mm_realtime0: libc::c_double;
    fn mm_idx_destroy(mi: *mut mm_idx_t);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct km_stat_t {
    pub capacity: size_t,
    pub available: size_t,
    pub n_blocks: size_t,
    pub n_cores: size_t,
    pub largest: size_t,
}
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type sdust_buf_t = sdust_buf_s;
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
pub struct mm_tbuf_s {
    pub km: *mut libc::c_void,
    pub rep_len: libc::c_int,
    pub frag_gap: libc::c_int,
}
pub type mm_tbuf_t = mm_tbuf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_seg_t {
    pub n_u: libc::c_int,
    pub n_a: libc::c_int,
    pub u: *mut uint64_t,
    pub a: *mut mm128_t,
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
pub type khint_t = khint32_t;
pub type khint32_t = libc::c_uint;
pub type mm_bseq_file_t = mm_bseq_file_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pipeline_t {
    pub n_processed: libc::c_int,
    pub n_threads: libc::c_int,
    pub n_fp: libc::c_int,
    pub mini_batch_size: int64_t,
    pub opt: *const mm_mapopt_t,
    pub fp: *mut *mut mm_bseq_file_t,
    pub mi: *const mm_idx_t,
    pub str_0: kstring_t,
    pub n_parts: libc::c_int,
    pub rid_shift: *mut uint32_t,
    pub fp_split: *mut FILE,
    pub fp_parts: *mut *mut FILE,
}
pub type kstring_t = __kstring_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __kstring_t {
    pub l: size_t,
    pub m: size_t,
    pub s: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct step_t {
    pub p: *const pipeline_t,
    pub n_seq: libc::c_int,
    pub n_frag: libc::c_int,
    pub seq: *mut mm_bseq1_t,
    pub n_reg: *mut libc::c_int,
    pub seg_off: *mut libc::c_int,
    pub n_seg: *mut libc::c_int,
    pub rep_len: *mut libc::c_int,
    pub frag_gap: *mut libc::c_int,
    pub reg: *mut *mut mm_reg1_t,
    pub buf: *mut *mut mm_tbuf_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_bseq1_t {
    pub l_seq: libc::c_int,
    pub rid: libc::c_int,
    pub name: *mut libc::c_char,
    pub seq: *mut libc::c_char,
    pub qual: *mut libc::c_char,
    pub comment: *mut libc::c_char,
}
#[inline]
unsafe extern "C" fn mm_revcomp_bseq(mut s: *mut mm_bseq1_t) {
    let mut i: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut l: libc::c_int = (*s).l_seq;
    i = 0 as libc::c_int;
    while i < l >> 1 as libc::c_int {
        t = *((*s).seq).offset((l - i - 1 as libc::c_int) as isize) as libc::c_int;
        *((*s).seq)
            .offset(
                (l - i - 1 as libc::c_int) as isize,
            ) = seq_comp_table[*((*s).seq).offset(i as isize) as uint8_t as usize]
            as libc::c_char;
        *((*s).seq).offset(i as isize) = seq_comp_table[t as usize] as libc::c_char;
        i += 1;
        i;
    }
    if l & 1 as libc::c_int != 0 {
        *((*s).seq)
            .offset(
                (l >> 1 as libc::c_int) as isize,
            ) = seq_comp_table[*((*s).seq).offset((l >> 1 as libc::c_int) as isize)
            as uint8_t as usize] as libc::c_char;
    }
    if !((*s).qual).is_null() {
        i = 0 as libc::c_int;
        while i < l >> 1 as libc::c_int {
            t = *((*s).qual).offset((l - i - 1 as libc::c_int) as isize) as libc::c_int;
            *((*s).qual)
                .offset(
                    (l - i - 1 as libc::c_int) as isize,
                ) = *((*s).qual).offset(i as isize);
            *((*s).qual).offset(i as isize) = t as libc::c_char;
            i += 1;
            i;
        }
    }
}
#[inline]
unsafe extern "C" fn mm_qname_same(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
) -> libc::c_int {
    let mut l1: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    l1 = mm_qname_len(s1);
    l2 = mm_qname_len(s2);
    return (l1 == l2 && strncmp(s1, s2, l1 as libc::c_ulong) == 0 as libc::c_int)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn mm_qname_len(mut s: *const libc::c_char) -> libc::c_int {
    let mut l: libc::c_int = 0;
    l = strlen(s) as libc::c_int;
    return if l >= 3 as libc::c_int
        && *s.offset((l - 1 as libc::c_int) as isize) as libc::c_int >= '0' as i32
        && *s.offset((l - 1 as libc::c_int) as isize) as libc::c_int <= '9' as i32
        && *s.offset((l - 2 as libc::c_int) as isize) as libc::c_int == '/' as i32
    {
        l - 2 as libc::c_int
    } else {
        l
    };
}
#[inline]
unsafe extern "C" fn __ac_X31_hash_string(mut s: *const libc::c_char) -> khint_t {
    let mut h: khint_t = *s as khint_t;
    if h != 0 {
        s = s.offset(1);
        s;
        while *s != 0 {
            h = (h << 5 as libc::c_int).wrapping_sub(h).wrapping_add(*s as khint_t);
            s = s.offset(1);
            s;
        }
    }
    return h;
}
#[inline]
unsafe extern "C" fn __ac_Wang_hash(mut key: khint_t) -> khint_t {
    key = (key as libc::c_uint).wrapping_add(!(key << 15 as libc::c_int)) as khint_t
        as khint_t;
    key ^= key >> 10 as libc::c_int;
    key = (key as libc::c_uint).wrapping_add(key << 3 as libc::c_int) as khint_t
        as khint_t;
    key ^= key >> 6 as libc::c_int;
    key = (key as libc::c_uint).wrapping_add(!(key << 11 as libc::c_int)) as khint_t
        as khint_t;
    key ^= key >> 16 as libc::c_int;
    return key;
}
pub unsafe extern "C" fn mm_tbuf_init() -> *mut mm_tbuf_t {
    let mut b: *mut mm_tbuf_t = 0 as *mut mm_tbuf_t;
    b = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<mm_tbuf_t>() as libc::c_ulong,
    ) as *mut mm_tbuf_t;
    if mm_dbg_flag & 1 as libc::c_int == 0 {
        (*b).km = km_init();
    }
    return b;
}
pub unsafe extern "C" fn mm_tbuf_destroy(mut b: *mut mm_tbuf_t) {
    if b.is_null() {
        return;
    }
    km_destroy((*b).km);
    free(b as *mut libc::c_void);
}
pub unsafe extern "C" fn mm_tbuf_get_km(mut b: *mut mm_tbuf_t) -> *mut libc::c_void {
    return (*b).km;
}
unsafe extern "C" fn mm_dust_minier(
    mut km: *mut libc::c_void,
    mut n: libc::c_int,
    mut a: *mut mm128_t,
    mut l_seq: libc::c_int,
    mut seq: *const libc::c_char,
    mut sdust_thres: libc::c_int,
) -> libc::c_int {
    let mut n_dreg: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut u: libc::c_int = 0 as libc::c_int;
    let mut dreg: *const uint64_t = 0 as *const uint64_t;
    let mut sdb: *mut sdust_buf_t = 0 as *mut sdust_buf_t;
    if sdust_thres <= 0 as libc::c_int {
        return n;
    }
    sdb = sdust_buf_init(km);
    dreg = sdust_core(
        seq as *const uint8_t,
        l_seq,
        sdust_thres,
        64 as libc::c_int,
        &mut n_dreg,
        sdb,
    );
    k = 0 as libc::c_int;
    j = k;
    while j < n {
        let mut qpos: int32_t = ((*a.offset(j as isize)).y as uint32_t
            >> 1 as libc::c_int) as int32_t;
        let mut span: int32_t = ((*a.offset(j as isize)).x
            & 0xff as libc::c_int as libc::c_ulong) as int32_t;
        let mut s: int32_t = qpos - (span - 1 as libc::c_int);
        let mut e: int32_t = s + span;
        while u < n_dreg && *dreg.offset(u as isize) as int32_t <= s {
            u += 1;
            u;
        }
        if u < n_dreg && ((*dreg.offset(u as isize) >> 32 as libc::c_int) as int32_t) < e
        {
            let mut v: libc::c_int = 0;
            let mut l: libc::c_int = 0 as libc::c_int;
            v = u;
            while v < n_dreg
                && ((*dreg.offset(v as isize) >> 32 as libc::c_int) as int32_t) < e
            {
                let mut ss: libc::c_int = (if s
                    > (*dreg.offset(v as isize) >> 32 as libc::c_int) as int32_t
                {
                    s as libc::c_ulong
                } else {
                    *dreg.offset(v as isize) >> 32 as libc::c_int
                }) as libc::c_int;
                let mut ee: libc::c_int = (if e < *dreg.offset(v as isize) as int32_t {
                    e as libc::c_uint
                } else {
                    *dreg.offset(v as isize) as uint32_t
                }) as libc::c_int;
                l += ee - ss;
                v += 1;
                v;
            }
            if l <= span >> 1 as libc::c_int {
                let fresh0 = k;
                k = k + 1;
                *a.offset(fresh0 as isize) = *a.offset(j as isize);
            }
        } else {
            let fresh1 = k;
            k = k + 1;
            *a.offset(fresh1 as isize) = *a.offset(j as isize);
        }
        j += 1;
        j;
    }
    sdust_buf_destroy(sdb);
    return k;
}
unsafe extern "C" fn collect_minimizers(
    mut km: *mut libc::c_void,
    mut opt: *const mm_mapopt_t,
    mut mi: *const mm_idx_t,
    mut n_segs: libc::c_int,
    mut qlens: *const libc::c_int,
    mut seqs: *mut *const libc::c_char,
    mut mv: *mut mm128_v,
) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut sum: libc::c_int = 0 as libc::c_int;
    (*mv).n = 0 as libc::c_int as size_t;
    n = 0 as libc::c_int;
    i = n;
    while i < n_segs {
        let mut j: size_t = 0;
        mm_sketch(
            km,
            *seqs.offset(i as isize),
            *qlens.offset(i as isize),
            (*mi).w,
            (*mi).k,
            i as uint32_t,
            (*mi).flag & 0x1 as libc::c_int,
            mv,
        );
        j = n as size_t;
        while j < (*mv).n {
            let ref mut fresh2 = (*((*mv).a).offset(j as isize)).y;
            *fresh2 = (*fresh2 as libc::c_ulong)
                .wrapping_add((sum << 1 as libc::c_int) as libc::c_ulong) as uint64_t
                as uint64_t;
            j = j.wrapping_add(1);
            j;
        }
        if (*opt).sdust_thres > 0 as libc::c_int {
            (*mv)
                .n = (n
                + mm_dust_minier(
                    km,
                    ((*mv).n).wrapping_sub(n as libc::c_ulong) as libc::c_int,
                    ((*mv).a).offset(n as isize),
                    *qlens.offset(i as isize),
                    *seqs.offset(i as isize),
                    (*opt).sdust_thres,
                )) as size_t;
        }
        sum += *qlens.offset(i as isize);
        n = (*mv).n as libc::c_int;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn ks_heapdown_heap(
    mut i: size_t,
    mut n: size_t,
    mut l: *mut mm128_t,
) {
    let mut k: size_t = i;
    let mut tmp: mm128_t = *l.offset(i as isize);
    loop {
        k = (k << 1 as libc::c_int).wrapping_add(1 as libc::c_int as libc::c_ulong);
        if !(k < n) {
            break;
        }
        if k != n.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            && (*l.offset(k as isize)).x
                > (*l.offset(k.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize))
                    .x
        {
            k = k.wrapping_add(1);
            k;
        }
        if (*l.offset(k as isize)).x > tmp.x {
            break;
        }
        *l.offset(i as isize) = *l.offset(k as isize);
        i = k;
    }
    *l.offset(i as isize) = tmp;
}
pub unsafe extern "C" fn ks_heapmake_heap(mut lsize: size_t, mut l: *mut mm128_t) {
    let mut i: size_t = 0;
    i = (lsize >> 1 as libc::c_int).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while i != -(1 as libc::c_int) as size_t {
        ks_heapdown_heap(i, lsize, l);
        i = i.wrapping_sub(1);
        i;
    }
}
pub unsafe extern "C" fn ks_ksmall_heap(
    mut n: size_t,
    mut arr: *mut mm128_t,
    mut kk: size_t,
) -> mm128_t {
    let mut low: *mut mm128_t = 0 as *mut mm128_t;
    let mut high: *mut mm128_t = 0 as *mut mm128_t;
    let mut k: *mut mm128_t = 0 as *mut mm128_t;
    let mut ll: *mut mm128_t = 0 as *mut mm128_t;
    let mut hh: *mut mm128_t = 0 as *mut mm128_t;
    let mut mid: *mut mm128_t = 0 as *mut mm128_t;
    low = arr;
    high = arr.offset(n as isize).offset(-(1 as libc::c_int as isize));
    k = arr.offset(kk as isize);
    loop {
        if high <= low {
            return *k;
        }
        if high == low.offset(1 as libc::c_int as isize) {
            if (*high).x > (*low).x {
                let mut t: mm128_t = *low;
                *low = *high;
                *high = t;
            }
            return *k;
        }
        mid = low
            .offset(
                (high.offset_from(low) as libc::c_long
                    / 2 as libc::c_int as libc::c_long) as isize,
            );
        if (*high).x > (*mid).x {
            let mut t_0: mm128_t = *mid;
            *mid = *high;
            *high = t_0;
        }
        if (*high).x > (*low).x {
            let mut t_1: mm128_t = *low;
            *low = *high;
            *high = t_1;
        }
        if (*low).x > (*mid).x {
            let mut t_2: mm128_t = *mid;
            *mid = *low;
            *low = t_2;
        }
        let mut t_3: mm128_t = *mid;
        *mid = *low.offset(1 as libc::c_int as isize);
        *low.offset(1 as libc::c_int as isize) = t_3;
        ll = low.offset(1 as libc::c_int as isize);
        hh = high;
        loop {
            loop {
                ll = ll.offset(1);
                ll;
                if !((*ll).x > (*low).x) {
                    break;
                }
            }
            loop {
                hh = hh.offset(-1);
                hh;
                if !((*low).x > (*hh).x) {
                    break;
                }
            }
            if hh < ll {
                break;
            }
            let mut t_4: mm128_t = *ll;
            *ll = *hh;
            *hh = t_4;
        }
        let mut t_5: mm128_t = *low;
        *low = *hh;
        *hh = t_5;
        if hh <= k {
            low = ll;
        }
        if hh >= k {
            high = hh.offset(-(1 as libc::c_int as isize));
        }
    };
}
#[inline]
unsafe extern "C" fn skip_seed(
    mut flag: libc::c_int,
    mut r: uint64_t,
    mut q: *const mm_seed_t,
    mut qname: *const libc::c_char,
    mut qlen: libc::c_int,
    mut mi: *const mm_idx_t,
    mut is_self: *mut libc::c_int,
) -> libc::c_int {
    *is_self = 0 as libc::c_int;
    if !qname.is_null() && flag & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0 {
        let mut s: *const mm_idx_seq_t = &mut *((*mi).seq)
            .offset((r >> 32 as libc::c_int) as isize) as *mut mm_idx_seq_t;
        let mut cmp: libc::c_int = 0;
        cmp = strcmp(qname, (*s).name);
        if flag & 0x1 as libc::c_int != 0 && cmp == 0 as libc::c_int
            && (*s).len as libc::c_int == qlen
        {
            if r as uint32_t >> 1 as libc::c_int == (*q).q_pos >> 1 as libc::c_int {
                return 1 as libc::c_int;
            }
            if r & 1 as libc::c_int as libc::c_ulong
                == ((*q).q_pos & 1 as libc::c_int as libc::c_uint) as libc::c_ulong
            {
                *is_self = 1 as libc::c_int;
            }
        }
        if flag & 0x2 as libc::c_int != 0 && cmp > 0 as libc::c_int {
            return 1 as libc::c_int;
        }
    }
    if flag & (0x100000 as libc::c_int | 0x200000 as libc::c_int) != 0 {
        if r & 1 as libc::c_int as libc::c_ulong
            == ((*q).q_pos & 1 as libc::c_int as libc::c_uint) as libc::c_ulong
        {
            if flag & 0x200000 as libc::c_int != 0 {
                return 1 as libc::c_int;
            }
        } else if flag & 0x100000 as libc::c_int != 0 {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn collect_seed_hits_heap(
    mut km: *mut libc::c_void,
    mut opt: *const mm_mapopt_t,
    mut max_occ: libc::c_int,
    mut mi: *const mm_idx_t,
    mut qname: *const libc::c_char,
    mut mv: *const mm128_v,
    mut qlen: libc::c_int,
    mut n_a: *mut int64_t,
    mut rep_len: *mut libc::c_int,
    mut n_mini_pos: *mut libc::c_int,
    mut mini_pos: *mut *mut uint64_t,
) -> *mut mm128_t {
    let mut i: libc::c_int = 0;
    let mut n_m: libc::c_int = 0;
    let mut heap_size: libc::c_int = 0 as libc::c_int;
    let mut j: int64_t = 0;
    let mut n_for: int64_t = 0 as libc::c_int as int64_t;
    let mut n_rev: int64_t = 0 as libc::c_int as int64_t;
    let mut m: *mut mm_seed_t = 0 as *mut mm_seed_t;
    let mut a: *mut mm128_t = 0 as *mut mm128_t;
    let mut heap: *mut mm128_t = 0 as *mut mm128_t;
    m = mm_collect_matches(
        km,
        &mut n_m,
        qlen,
        max_occ,
        (*opt).max_max_occ,
        (*opt).occ_dist,
        mi,
        mv,
        n_a,
        rep_len,
        n_mini_pos,
        mini_pos,
    );
    heap = kmalloc(
        km,
        (n_m as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mm128_t>() as libc::c_ulong),
    ) as *mut mm128_t;
    a = kmalloc(
        km,
        (*n_a as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mm128_t>() as libc::c_ulong),
    ) as *mut mm128_t;
    i = 0 as libc::c_int;
    heap_size = 0 as libc::c_int;
    while i < n_m {
        if (*m.offset(i as isize)).n > 0 as libc::c_int as libc::c_uint {
            (*heap.offset(heap_size as isize))
                .x = *((*m.offset(i as isize)).cr).offset(0 as libc::c_int as isize);
            (*heap.offset(heap_size as isize)).y = (i as uint64_t) << 32 as libc::c_int;
            heap_size += 1;
            heap_size;
        }
        i += 1;
        i;
    }
    ks_heapmake_heap(heap_size as size_t, heap);
    while heap_size > 0 as libc::c_int {
        let mut q: *mut mm_seed_t = &mut *m
            .offset(((*heap).y >> 32 as libc::c_int) as isize) as *mut mm_seed_t;
        let mut p: *mut mm128_t = 0 as *mut mm128_t;
        let mut r: uint64_t = (*heap).x;
        let mut is_self: int32_t = 0;
        let mut rpos: int32_t = (r as uint32_t >> 1 as libc::c_int) as int32_t;
        if skip_seed((*opt).flag as libc::c_int, r, q, qname, qlen, mi, &mut is_self)
            == 0
        {
            if r & 1 as libc::c_int as libc::c_ulong
                == ((*q).q_pos & 1 as libc::c_int as libc::c_uint) as libc::c_ulong
            {
                let fresh3 = n_for;
                n_for = n_for + 1;
                p = &mut *a.offset(fresh3 as isize) as *mut mm128_t;
                (*p)
                    .x = (r as libc::c_ulonglong
                    & 0xffffffff00000000 as libc::c_ulonglong
                    | rpos as libc::c_ulonglong) as uint64_t;
                (*p)
                    .y = ((*q).q_span() as uint64_t) << 32 as libc::c_int
                    | ((*q).q_pos >> 1 as libc::c_int) as libc::c_ulong;
            } else {
                n_rev += 1;
                p = &mut *a.offset((*n_a - n_rev) as isize) as *mut mm128_t;
                (*p)
                    .x = ((1 as libc::c_ulonglong) << 63 as libc::c_int
                    | r as libc::c_ulonglong & 0xffffffff00000000 as libc::c_ulonglong
                    | rpos as libc::c_ulonglong) as uint64_t;
                (*p)
                    .y = ((*q).q_span() as uint64_t) << 32 as libc::c_int
                    | (qlen as libc::c_uint)
                        .wrapping_sub(
                            ((*q).q_pos >> 1 as libc::c_int)
                                .wrapping_add(1 as libc::c_int as libc::c_uint)
                                .wrapping_sub((*q).q_span()),
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong;
            }
            (*p).y |= ((*q).seg_id() as uint64_t) << 48 as libc::c_int;
            if (*q).is_tandem() != 0 {
                (*p)
                    .y = ((*p).y as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 42 as libc::c_int) as uint64_t;
            }
            if is_self != 0 {
                (*p)
                    .y = ((*p).y as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 43 as libc::c_int) as uint64_t;
            }
        }
        if ((*heap).y as uint32_t)
            < ((*q).n).wrapping_sub(1 as libc::c_int as libc::c_uint)
        {
            let ref mut fresh4 = (*heap.offset(0 as libc::c_int as isize)).y;
            *fresh4 = (*fresh4).wrapping_add(1);
            *fresh4;
            (*heap.offset(0 as libc::c_int as isize))
                .x = *((*m
                .offset(
                    ((*heap.offset(0 as libc::c_int as isize)).y >> 32 as libc::c_int)
                        as isize,
                ))
                .cr)
                .offset(
                    (*heap.offset(0 as libc::c_int as isize)).y as uint32_t as isize,
                );
        } else {
            *heap
                .offset(
                    0 as libc::c_int as isize,
                ) = *heap.offset((heap_size - 1 as libc::c_int) as isize);
            heap_size -= 1;
            heap_size;
        }
        ks_heapdown_heap(0 as libc::c_int as size_t, heap_size as size_t, heap);
    }
    kfree(km, m as *mut libc::c_void);
    kfree(km, heap as *mut libc::c_void);
    j = 0 as libc::c_int as int64_t;
    while j < n_rev >> 1 as libc::c_int {
        let mut t: mm128_t = *a
            .offset((*n_a - 1 as libc::c_int as libc::c_long - j) as isize);
        *a
            .offset(
                (*n_a - 1 as libc::c_int as libc::c_long - j) as isize,
            ) = *a.offset((*n_a - (n_rev - j)) as isize);
        *a.offset((*n_a - (n_rev - j)) as isize) = t;
        j += 1;
        j;
    }
    if *n_a > n_for + n_rev {
        memmove(
            a.offset(n_for as isize) as *mut libc::c_void,
            a.offset(*n_a as isize).offset(-(n_rev as isize)) as *const libc::c_void,
            (n_rev as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<mm128_t>() as libc::c_ulong),
        );
        *n_a = n_for + n_rev;
    }
    return a;
}
unsafe extern "C" fn collect_seed_hits(
    mut km: *mut libc::c_void,
    mut opt: *const mm_mapopt_t,
    mut max_occ: libc::c_int,
    mut mi: *const mm_idx_t,
    mut qname: *const libc::c_char,
    mut mv: *const mm128_v,
    mut qlen: libc::c_int,
    mut n_a: *mut int64_t,
    mut rep_len: *mut libc::c_int,
    mut n_mini_pos: *mut libc::c_int,
    mut mini_pos: *mut *mut uint64_t,
) -> *mut mm128_t {
    let mut i: libc::c_int = 0;
    let mut n_m: libc::c_int = 0;
    let mut m: *mut mm_seed_t = 0 as *mut mm_seed_t;
    let mut a: *mut mm128_t = 0 as *mut mm128_t;
    m = mm_collect_matches(
        km,
        &mut n_m,
        qlen,
        max_occ,
        (*opt).max_max_occ,
        (*opt).occ_dist,
        mi,
        mv,
        n_a,
        rep_len,
        n_mini_pos,
        mini_pos,
    );
    a = kmalloc(
        km,
        (*n_a as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mm128_t>() as libc::c_ulong),
    ) as *mut mm128_t;
    i = 0 as libc::c_int;
    *n_a = 0 as libc::c_int as int64_t;
    while i < n_m {
        let mut q: *mut mm_seed_t = &mut *m.offset(i as isize) as *mut mm_seed_t;
        let mut r: *const uint64_t = (*q).cr;
        let mut k: uint32_t = 0;
        k = 0 as libc::c_int as uint32_t;
        while k < (*q).n {
            let mut is_self: int32_t = 0;
            let mut rpos: int32_t = (*r.offset(k as isize) as uint32_t
                >> 1 as libc::c_int) as int32_t;
            let mut p: *mut mm128_t = 0 as *mut mm128_t;
            if !(skip_seed(
                (*opt).flag as libc::c_int,
                *r.offset(k as isize),
                q,
                qname,
                qlen,
                mi,
                &mut is_self,
            ) != 0)
            {
                let fresh5 = *n_a;
                *n_a = *n_a + 1;
                p = &mut *a.offset(fresh5 as isize) as *mut mm128_t;
                if *r.offset(k as isize) & 1 as libc::c_int as libc::c_ulong
                    == ((*q).q_pos & 1 as libc::c_int as libc::c_uint) as libc::c_ulong
                {
                    (*p)
                        .x = (*r.offset(k as isize) as libc::c_ulonglong
                        & 0xffffffff00000000 as libc::c_ulonglong
                        | rpos as libc::c_ulonglong) as uint64_t;
                    (*p)
                        .y = ((*q).q_span() as uint64_t) << 32 as libc::c_int
                        | ((*q).q_pos >> 1 as libc::c_int) as libc::c_ulong;
                } else if (*opt).flag as libc::c_longlong
                    & 0x100000000 as libc::c_longlong == 0
                {
                    (*p)
                        .x = ((1 as libc::c_ulonglong) << 63 as libc::c_int
                        | *r.offset(k as isize) as libc::c_ulonglong
                            & 0xffffffff00000000 as libc::c_ulonglong
                        | rpos as libc::c_ulonglong) as uint64_t;
                    (*p)
                        .y = ((*q).q_span() as uint64_t) << 32 as libc::c_int
                        | (qlen as libc::c_uint)
                            .wrapping_sub(
                                ((*q).q_pos >> 1 as libc::c_int)
                                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                                    .wrapping_sub((*q).q_span()),
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong;
                } else {
                    let mut len: int32_t = (*((*mi).seq)
                        .offset((*r.offset(k as isize) >> 32 as libc::c_int) as isize))
                        .len as int32_t;
                    (*p)
                        .x = ((1 as libc::c_ulonglong) << 63 as libc::c_int
                        | *r.offset(k as isize) as libc::c_ulonglong
                            & 0xffffffff00000000 as libc::c_ulonglong
                        | (len - (rpos + 1 as libc::c_int - (*q).q_span() as libc::c_int)
                            - 1 as libc::c_int) as libc::c_ulonglong) as uint64_t;
                    (*p)
                        .y = ((*q).q_span() as uint64_t) << 32 as libc::c_int
                        | ((*q).q_pos >> 1 as libc::c_int) as libc::c_ulong;
                }
                (*p).y |= ((*q).seg_id() as uint64_t) << 48 as libc::c_int;
                if (*q).is_tandem() != 0 {
                    (*p)
                        .y = ((*p).y as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << 42 as libc::c_int) as uint64_t;
                }
                if is_self != 0 {
                    (*p)
                        .y = ((*p).y as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << 43 as libc::c_int) as uint64_t;
                }
            }
            k = k.wrapping_add(1);
            k;
        }
        i += 1;
        i;
    }
    kfree(km, m as *mut libc::c_void);
    radix_sort_128x(a, a.offset(*n_a as isize));
    return a;
}
unsafe extern "C" fn chain_post(
    mut opt: *const mm_mapopt_t,
    mut max_chain_gap_ref: libc::c_int,
    mut mi: *const mm_idx_t,
    mut km: *mut libc::c_void,
    mut qlen: libc::c_int,
    mut n_segs: libc::c_int,
    mut qlens: *const libc::c_int,
    mut n_regs: *mut libc::c_int,
    mut regs: *mut mm_reg1_t,
    mut a: *mut mm128_t,
) {
    if (*opt).flag & 0x800000 as libc::c_int as libc::c_long == 0 {
        mm_set_parent(
            km,
            (*opt).mask_level,
            (*opt).mask_len,
            *n_regs,
            regs,
            (*opt).a * 2 as libc::c_int + (*opt).b,
            ((*opt).flag & 0x20000000 as libc::c_int as libc::c_long) as libc::c_int,
            (*opt).alt_drop,
        );
        if n_segs <= 1 as libc::c_int {
            mm_select_sub(
                km,
                (*opt).pri_ratio,
                (*mi).k * 2 as libc::c_int,
                (*opt).best_n,
                1 as libc::c_int,
                ((*opt).max_gap as libc::c_double * 0.8f64) as libc::c_int,
                n_regs,
                regs,
            );
        } else {
            mm_select_sub_multi(
                km,
                (*opt).pri_ratio,
                0.2f32,
                0.7f32,
                max_chain_gap_ref,
                (*mi).k * 2 as libc::c_int,
                (*opt).best_n,
                n_segs,
                qlens,
                n_regs,
                regs,
            );
        }
    }
}
unsafe extern "C" fn align_regs(
    mut opt: *const mm_mapopt_t,
    mut mi: *const mm_idx_t,
    mut km: *mut libc::c_void,
    mut qlen: libc::c_int,
    mut seq: *const libc::c_char,
    mut n_regs: *mut libc::c_int,
    mut regs: *mut mm_reg1_t,
    mut a: *mut mm128_t,
) -> *mut mm_reg1_t {
    if (*opt).flag & 0x4 as libc::c_int as libc::c_long == 0 {
        return regs;
    }
    regs = mm_align_skeleton(km, opt, mi, qlen, seq, n_regs, regs, a);
    if (*opt).flag & 0x800000 as libc::c_int as libc::c_long == 0 {
        mm_set_parent(
            km,
            (*opt).mask_level,
            (*opt).mask_len,
            *n_regs,
            regs,
            (*opt).a * 2 as libc::c_int + (*opt).b,
            ((*opt).flag & 0x20000000 as libc::c_int as libc::c_long) as libc::c_int,
            (*opt).alt_drop,
        );
        mm_select_sub(
            km,
            (*opt).pri_ratio,
            (*mi).k * 2 as libc::c_int,
            (*opt).best_n,
            0 as libc::c_int,
            ((*opt).max_gap as libc::c_double * 0.8f64) as libc::c_int,
            n_regs,
            regs,
        );
        mm_set_sam_pri(*n_regs, regs);
    }
    return regs;
}
pub unsafe extern "C" fn mm_map_frag(
    mut mi: *const mm_idx_t,
    mut n_segs: libc::c_int,
    mut qlens: *const libc::c_int,
    mut seqs: *mut *const libc::c_char,
    mut n_regs: *mut libc::c_int,
    mut regs: *mut *mut mm_reg1_t,
    mut b: *mut mm_tbuf_t,
    mut opt: *const mm_mapopt_t,
    mut qname: *const libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut rep_len: libc::c_int = 0;
    let mut qlen_sum: libc::c_int = 0;
    let mut n_regs0: libc::c_int = 0;
    let mut n_mini_pos: libc::c_int = 0;
    let mut max_chain_gap_qry: libc::c_int = 0;
    let mut max_chain_gap_ref: libc::c_int = 0;
    let mut is_splice: libc::c_int = ((*opt).flag & 0x80 as libc::c_int as libc::c_long
        != 0) as libc::c_int;
    let mut is_sr: libc::c_int = ((*opt).flag & 0x1000 as libc::c_int as libc::c_long
        != 0) as libc::c_int;
    let mut hash: uint32_t = 0;
    let mut n_a: int64_t = 0;
    let mut u: *mut uint64_t = 0 as *mut uint64_t;
    let mut mini_pos: *mut uint64_t = 0 as *mut uint64_t;
    let mut a: *mut mm128_t = 0 as *mut mm128_t;
    let mut mv: mm128_v = {
        let mut init = mm128_v {
            n: 0 as libc::c_int as size_t,
            m: 0 as libc::c_int as size_t,
            a: 0 as *mut mm128_t,
        };
        init
    };
    let mut regs0: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut kmst: km_stat_t = km_stat_t {
        capacity: 0,
        available: 0,
        n_blocks: 0,
        n_cores: 0,
        largest: 0,
    };
    let mut chn_pen_gap: libc::c_float = 0.;
    let mut chn_pen_skip: libc::c_float = 0.;
    i = 0 as libc::c_int;
    qlen_sum = 0 as libc::c_int;
    while i < n_segs {
        qlen_sum += *qlens.offset(i as isize);
        *n_regs.offset(i as isize) = 0 as libc::c_int;
        let ref mut fresh6 = *regs.offset(i as isize);
        *fresh6 = 0 as *mut mm_reg1_t;
        i += 1;
        i;
    }
    if qlen_sum == 0 as libc::c_int || n_segs <= 0 as libc::c_int
        || n_segs > 255 as libc::c_int
    {
        return;
    }
    if (*opt).max_qlen > 0 as libc::c_int && qlen_sum > (*opt).max_qlen {
        return;
    }
    hash = if !qname.is_null()
        && (*opt).flag as libc::c_longlong & 0x400000000 as libc::c_longlong == 0
    {
        __ac_X31_hash_string(qname)
    } else {
        0 as libc::c_int as libc::c_uint
    };
    hash
        ^= (__ac_Wang_hash(qlen_sum as khint_t))
            .wrapping_add(__ac_Wang_hash((*opt).seed as khint_t));
    hash = __ac_Wang_hash(hash);
    collect_minimizers((*b).km, opt, mi, n_segs, qlens, seqs, &mut mv);
    if (*opt).q_occ_frac > 0.0f32 {
        mm_seed_mz_flt((*b).km, &mut mv, (*opt).mid_occ, (*opt).q_occ_frac);
    }
    if (*opt).flag & 0x400000 as libc::c_int as libc::c_long != 0 {
        a = collect_seed_hits_heap(
            (*b).km,
            opt,
            (*opt).mid_occ,
            mi,
            qname,
            &mut mv,
            qlen_sum,
            &mut n_a,
            &mut rep_len,
            &mut n_mini_pos,
            &mut mini_pos,
        );
    } else {
        a = collect_seed_hits(
            (*b).km,
            opt,
            (*opt).mid_occ,
            mi,
            qname,
            &mut mv,
            qlen_sum,
            &mut n_a,
            &mut rep_len,
            &mut n_mini_pos,
            &mut mini_pos,
        );
    }
    if mm_dbg_flag & 0x4 as libc::c_int != 0 {
        fprintf(stderr, b"RS\t%d\n\0" as *const u8 as *const libc::c_char, rep_len);
        i = 0 as libc::c_int;
        while (i as libc::c_long) < n_a {
            fprintf(
                stderr,
                b"SD\t%s\t%d\t%c\t%d\t%d\t%d\n\0" as *const u8 as *const libc::c_char,
                (*((*mi).seq)
                    .offset(
                        ((*a.offset(i as isize)).x << 1 as libc::c_int
                            >> 33 as libc::c_int) as isize,
                    ))
                    .name,
                (*a.offset(i as isize)).x as int32_t,
                (*::std::mem::transmute::<
                    &[u8; 3],
                    &[libc::c_char; 3],
                >(b"+-\0"))[((*a.offset(i as isize)).x >> 63 as libc::c_int) as usize]
                    as libc::c_int,
                (*a.offset(i as isize)).y as int32_t,
                ((*a.offset(i as isize)).y >> 32 as libc::c_int
                    & 0xff as libc::c_int as libc::c_ulong) as int32_t,
                if i == 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    (*a.offset(i as isize)).y as int32_t
                        - (*a.offset((i - 1 as libc::c_int) as isize)).y as int32_t
                        - ((*a.offset(i as isize)).x as int32_t
                            - (*a.offset((i - 1 as libc::c_int) as isize)).x as int32_t)
                },
            );
            i += 1;
            i;
        }
    }
    if is_sr != 0 {
        max_chain_gap_qry = if qlen_sum > (*opt).max_gap {
            qlen_sum
        } else {
            (*opt).max_gap
        };
    } else {
        max_chain_gap_qry = (*opt).max_gap;
    }
    if (*opt).max_gap_ref > 0 as libc::c_int {
        max_chain_gap_ref = (*opt).max_gap_ref;
    } else if (*opt).max_frag_len > 0 as libc::c_int {
        max_chain_gap_ref = (*opt).max_frag_len - qlen_sum;
        if max_chain_gap_ref < (*opt).max_gap {
            max_chain_gap_ref = (*opt).max_gap;
        }
    } else {
        max_chain_gap_ref = (*opt).max_gap;
    }
    chn_pen_gap = ((*opt).chain_gap_scale as libc::c_double * 0.01f64
        * (*mi).k as libc::c_double) as libc::c_float;
    chn_pen_skip = ((*opt).chain_skip_scale as libc::c_double * 0.01f64
        * (*mi).k as libc::c_double) as libc::c_float;
    if (*opt).flag as libc::c_longlong & 0x80000000 as libc::c_longlong != 0 {
        a = mg_lchain_rmq(
            (*opt).max_gap,
            (*opt).rmq_inner_dist,
            (*opt).bw,
            (*opt).max_chain_skip,
            (*opt).rmq_size_cap,
            (*opt).min_cnt,
            (*opt).min_chain_score,
            chn_pen_gap,
            chn_pen_skip,
            n_a,
            a,
            &mut n_regs0,
            &mut u,
            (*b).km,
        );
    } else {
        a = mg_lchain_dp(
            max_chain_gap_ref,
            max_chain_gap_qry,
            (*opt).bw,
            (*opt).max_chain_skip,
            (*opt).max_chain_iter,
            (*opt).min_cnt,
            (*opt).min_chain_score,
            chn_pen_gap,
            chn_pen_skip,
            is_splice,
            n_segs,
            n_a,
            a,
            &mut n_regs0,
            &mut u,
            (*b).km,
        );
    }
    if (*opt).bw_long > (*opt).bw
        && (*opt).flag
            & (0x80 as libc::c_int | 0x1000 as libc::c_int | 0x400 as libc::c_int)
                as libc::c_long == 0 as libc::c_int as libc::c_long
        && n_segs == 1 as libc::c_int && n_regs0 > 1 as libc::c_int
    {
        let mut st: int32_t = (*a.offset(0 as libc::c_int as isize)).y as int32_t;
        let mut en: int32_t = (*a
            .offset(
                (*u.offset(0 as libc::c_int as isize) as int32_t - 1 as libc::c_int)
                    as isize,
            ))
            .y as int32_t;
        if qlen_sum - (en - st) > (*opt).rmq_rescue_size
            || (en - st) as libc::c_float
                > qlen_sum as libc::c_float * (*opt).rmq_rescue_ratio
        {
            let mut i_0: int32_t = 0;
            i_0 = 0 as libc::c_int;
            n_a = 0 as libc::c_int as int64_t;
            while i_0 < n_regs0 {
                n_a += *u.offset(i_0 as isize) as int32_t as libc::c_long;
                i_0 += 1;
                i_0;
            }
            kfree((*b).km, u as *mut libc::c_void);
            radix_sort_128x(a, a.offset(n_a as isize));
            a = mg_lchain_rmq(
                (*opt).max_gap,
                (*opt).rmq_inner_dist,
                (*opt).bw_long,
                (*opt).max_chain_skip,
                (*opt).rmq_size_cap,
                (*opt).min_cnt,
                (*opt).min_chain_score,
                chn_pen_gap,
                chn_pen_skip,
                n_a,
                a,
                &mut n_regs0,
                &mut u,
                (*b).km,
            );
        }
    } else if (*opt).max_occ > (*opt).mid_occ && rep_len > 0 as libc::c_int
        && (*opt).flag as libc::c_longlong & 0x80000000 as libc::c_longlong == 0
    {
        let mut rechain: libc::c_int = 0 as libc::c_int;
        if n_regs0 > 0 as libc::c_int {
            let mut n_chained_segs: libc::c_int = 1 as libc::c_int;
            let mut max: libc::c_int = 0 as libc::c_int;
            let mut max_i: libc::c_int = -(1 as libc::c_int);
            let mut max_off: libc::c_int = -(1 as libc::c_int);
            let mut off: libc::c_int = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < n_regs0 {
                if max < (*u.offset(i as isize) >> 32 as libc::c_int) as libc::c_int {
                    max = (*u.offset(i as isize) >> 32 as libc::c_int) as libc::c_int;
                    max_i = i;
                    max_off = off;
                }
                off = (off as libc::c_uint)
                    .wrapping_add(*u.offset(i as isize) as uint32_t) as libc::c_int
                    as libc::c_int;
                i += 1;
                i;
            }
            i = 1 as libc::c_int;
            while i < *u.offset(max_i as isize) as int32_t {
                if (*a.offset((max_off + i) as isize)).y as libc::c_ulonglong
                    & (0xff as libc::c_ulonglong) << 48 as libc::c_int
                    != (*a.offset((max_off + i - 1 as libc::c_int) as isize)).y
                        as libc::c_ulonglong
                        & (0xff as libc::c_ulonglong) << 48 as libc::c_int
                {
                    n_chained_segs += 1;
                    n_chained_segs;
                }
                i += 1;
                i;
            }
            if n_chained_segs < n_segs {
                rechain = 1 as libc::c_int;
            }
        } else {
            rechain = 1 as libc::c_int;
        }
        if rechain != 0 {
            kfree((*b).km, a as *mut libc::c_void);
            kfree((*b).km, u as *mut libc::c_void);
            kfree((*b).km, mini_pos as *mut libc::c_void);
            if (*opt).flag & 0x400000 as libc::c_int as libc::c_long != 0 {
                a = collect_seed_hits_heap(
                    (*b).km,
                    opt,
                    (*opt).max_occ,
                    mi,
                    qname,
                    &mut mv,
                    qlen_sum,
                    &mut n_a,
                    &mut rep_len,
                    &mut n_mini_pos,
                    &mut mini_pos,
                );
            } else {
                a = collect_seed_hits(
                    (*b).km,
                    opt,
                    (*opt).max_occ,
                    mi,
                    qname,
                    &mut mv,
                    qlen_sum,
                    &mut n_a,
                    &mut rep_len,
                    &mut n_mini_pos,
                    &mut mini_pos,
                );
            }
            a = mg_lchain_dp(
                max_chain_gap_ref,
                max_chain_gap_qry,
                (*opt).bw,
                (*opt).max_chain_skip,
                (*opt).max_chain_iter,
                (*opt).min_cnt,
                (*opt).min_chain_score,
                chn_pen_gap,
                chn_pen_skip,
                is_splice,
                n_segs,
                n_a,
                a,
                &mut n_regs0,
                &mut u,
                (*b).km,
            );
        }
    }
    (*b).frag_gap = max_chain_gap_ref;
    (*b).rep_len = rep_len;
    regs0 = mm_gen_regs(
        (*b).km,
        hash,
        qlen_sum,
        n_regs0,
        u,
        a,
        ((*opt).flag as libc::c_longlong & 0x100000000 as libc::c_longlong != 0)
            as libc::c_int,
    );
    if (*mi).n_alt != 0 {
        mm_mark_alt(mi, n_regs0, regs0);
        mm_hit_sort((*b).km, &mut n_regs0, regs0, (*opt).alt_drop);
    }
    if mm_dbg_flag & (0x4 as libc::c_int | 0x10 as libc::c_int) != 0 {
        j = 0 as libc::c_int;
        while j < n_regs0 {
            i = (*regs0.offset(j as isize)).as_0;
            while i < (*regs0.offset(j as isize)).as_0 + (*regs0.offset(j as isize)).cnt
            {
                fprintf(
                    stderr,
                    b"CN\t%d\t%s\t%d\t%c\t%d\t%d\t%d\n\0" as *const u8
                        as *const libc::c_char,
                    j,
                    (*((*mi).seq)
                        .offset(
                            ((*a.offset(i as isize)).x << 1 as libc::c_int
                                >> 33 as libc::c_int) as isize,
                        ))
                        .name,
                    (*a.offset(i as isize)).x as int32_t,
                    (*::std::mem::transmute::<
                        &[u8; 3],
                        &[libc::c_char; 3],
                    >(
                        b"+-\0",
                    ))[((*a.offset(i as isize)).x >> 63 as libc::c_int) as usize]
                        as libc::c_int,
                    (*a.offset(i as isize)).y as int32_t,
                    ((*a.offset(i as isize)).y >> 32 as libc::c_int
                        & 0xff as libc::c_int as libc::c_ulong) as int32_t,
                    if i == (*regs0.offset(j as isize)).as_0 {
                        0 as libc::c_int
                    } else {
                        (*a.offset(i as isize)).y as int32_t
                            - (*a.offset((i - 1 as libc::c_int) as isize)).y as int32_t
                            - ((*a.offset(i as isize)).x as int32_t
                                - (*a.offset((i - 1 as libc::c_int) as isize)).x as int32_t)
                    },
                );
                i += 1;
                i;
            }
            j += 1;
            j;
        }
    }
    chain_post(
        opt,
        max_chain_gap_ref,
        mi,
        (*b).km,
        qlen_sum,
        n_segs,
        qlens,
        &mut n_regs0,
        regs0,
        a,
    );
    if is_sr == 0
        && (*opt).flag as libc::c_longlong & 0x100000000 as libc::c_longlong == 0
    {
        mm_est_err(mi, qlen_sum, n_regs0, regs0, a, n_mini_pos, mini_pos);
        n_regs0 = mm_filter_strand_retained(n_regs0, regs0);
    }
    if n_segs == 1 as libc::c_int {
        regs0 = align_regs(
            opt,
            mi,
            (*b).km,
            *qlens.offset(0 as libc::c_int as isize),
            *seqs.offset(0 as libc::c_int as isize),
            &mut n_regs0,
            regs0,
            a,
        );
        regs0 = realloc(
            regs0 as *mut libc::c_void,
            (::std::mem::size_of::<mm_reg1_t>() as libc::c_ulong)
                .wrapping_mul(n_regs0 as libc::c_ulong),
        ) as *mut mm_reg1_t;
        mm_set_mapq(
            (*b).km,
            n_regs0,
            regs0,
            (*opt).min_chain_score,
            (*opt).a,
            rep_len,
            is_sr,
        );
        *n_regs.offset(0 as libc::c_int as isize) = n_regs0;
        let ref mut fresh7 = *regs.offset(0 as libc::c_int as isize);
        *fresh7 = regs0;
    } else {
        let mut seg: *mut mm_seg_t = 0 as *mut mm_seg_t;
        seg = mm_seg_gen((*b).km, hash, n_segs, qlens, n_regs0, regs0, n_regs, regs, a);
        free(regs0 as *mut libc::c_void);
        i = 0 as libc::c_int;
        while i < n_segs {
            mm_set_parent(
                (*b).km,
                (*opt).mask_level,
                (*opt).mask_len,
                *n_regs.offset(i as isize),
                *regs.offset(i as isize),
                (*opt).a * 2 as libc::c_int + (*opt).b,
                ((*opt).flag & 0x20000000 as libc::c_int as libc::c_long) as libc::c_int,
                (*opt).alt_drop,
            );
            let ref mut fresh8 = *regs.offset(i as isize);
            *fresh8 = align_regs(
                opt,
                mi,
                (*b).km,
                *qlens.offset(i as isize),
                *seqs.offset(i as isize),
                &mut *n_regs.offset(i as isize),
                *regs.offset(i as isize),
                (*seg.offset(i as isize)).a,
            );
            mm_set_mapq(
                (*b).km,
                *n_regs.offset(i as isize),
                *regs.offset(i as isize),
                (*opt).min_chain_score,
                (*opt).a,
                rep_len,
                is_sr,
            );
            i += 1;
            i;
        }
        mm_seg_free((*b).km, n_segs, seg);
        if n_segs == 2 as libc::c_int && (*opt).pe_ori >= 0 as libc::c_int
            && (*opt).flag & 0x4 as libc::c_int as libc::c_long != 0
        {
            mm_pair(
                (*b).km,
                max_chain_gap_ref,
                (*opt).pe_bonus,
                (*opt).a * 2 as libc::c_int + (*opt).b,
                (*opt).a,
                qlens,
                n_regs,
                regs,
            );
        }
    }
    kfree((*b).km, mv.a as *mut libc::c_void);
    kfree((*b).km, a as *mut libc::c_void);
    kfree((*b).km, u as *mut libc::c_void);
    kfree((*b).km, mini_pos as *mut libc::c_void);
    if !((*b).km).is_null() {
        km_stat((*b).km, &mut kmst);
        if mm_dbg_flag & 0x2 as libc::c_int != 0 {
            fprintf(
                stderr,
                b"QM\t%s\t%d\tcap=%ld,nCore=%ld,largest=%ld\n\0" as *const u8
                    as *const libc::c_char,
                qname,
                qlen_sum,
                kmst.capacity,
                kmst.n_cores,
                kmst.largest,
            );
        }
        if kmst.n_blocks == kmst.n_cores {} else {
            __assert_fail(
                b"kmst.n_blocks == kmst.n_cores\0" as *const u8 as *const libc::c_char,
                b"map.c\0" as *const u8 as *const libc::c_char,
                371 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 137],
                    &[libc::c_char; 137],
                >(
                    b"void mm_map_frag(const mm_idx_t *, int, const int *, const char **, int *, mm_reg1_t **, mm_tbuf_t *, const mm_mapopt_t *, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_2849: {
            if kmst.n_blocks == kmst.n_cores {} else {
                __assert_fail(
                    b"kmst.n_blocks == kmst.n_cores\0" as *const u8
                        as *const libc::c_char,
                    b"map.c\0" as *const u8 as *const libc::c_char,
                    371 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 137],
                        &[libc::c_char; 137],
                    >(
                        b"void mm_map_frag(const mm_idx_t *, int, const int *, const char **, int *, mm_reg1_t **, mm_tbuf_t *, const mm_mapopt_t *, const char *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if kmst.largest > ((1 as libc::c_uint) << 28 as libc::c_int) as libc::c_ulong
            || (*opt).cap_kalloc > 0 as libc::c_int as libc::c_long
                && kmst.capacity > (*opt).cap_kalloc as libc::c_ulong
        {
            if mm_dbg_flag & 0x2 as libc::c_int != 0 {
                fprintf(
                    stderr,
                    b"[W::%s] reset thread-local memory after read %s\n\0" as *const u8
                        as *const libc::c_char,
                    (*::std::mem::transmute::<
                        &[u8; 12],
                        &[libc::c_char; 12],
                    >(b"mm_map_frag\0"))
                        .as_ptr(),
                    qname,
                );
            }
            km_destroy((*b).km);
            (*b).km = km_init();
        }
    }
}
pub unsafe extern "C" fn mm_map(
    mut mi: *const mm_idx_t,
    mut qlen: libc::c_int,
    mut seq: *const libc::c_char,
    mut n_regs: *mut libc::c_int,
    mut b: *mut mm_tbuf_t,
    mut opt: *const mm_mapopt_t,
    mut qname: *const libc::c_char,
) -> *mut mm_reg1_t {
    let mut regs: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    mm_map_frag(
        mi,
        1 as libc::c_int,
        &mut qlen,
        &mut seq,
        n_regs,
        &mut regs,
        b,
        opt,
        qname,
    );
    return regs;
}
unsafe extern "C" fn worker_for(
    mut _data: *mut libc::c_void,
    mut i: libc::c_long,
    mut tid: libc::c_int,
) {
    let mut s: *mut step_t = _data as *mut step_t;
    let mut qlens: [libc::c_int; 255] = [0; 255];
    let mut j: libc::c_int = 0;
    let mut off: libc::c_int = *((*s).seg_off).offset(i as isize);
    let mut pe_ori: libc::c_int = (*(*(*s).p).opt).pe_ori;
    let mut qseqs: [*const libc::c_char; 255] = [0 as *const libc::c_char; 255];
    let mut t: libc::c_double = 0.0f64;
    let mut b: *mut mm_tbuf_t = *((*s).buf).offset(tid as isize);
    if *((*s).n_seg).offset(i as isize) <= 255 as libc::c_int {} else {
        __assert_fail(
            b"s->n_seg[i] <= MM_MAX_SEG\0" as *const u8 as *const libc::c_char,
            b"map.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void worker_for(void *, long, int)\0"))
                .as_ptr(),
        );
    }
    'c_9395: {
        if *((*s).n_seg).offset(i as isize) <= 255 as libc::c_int {} else {
            __assert_fail(
                b"s->n_seg[i] <= MM_MAX_SEG\0" as *const u8 as *const libc::c_char,
                b"map.c\0" as *const u8 as *const libc::c_char,
                421 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void worker_for(void *, long, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if mm_dbg_flag & 0x2 as libc::c_int != 0 {
        fprintf(
            stderr,
            b"QR\t%s\t%d\t%d\n\0" as *const u8 as *const libc::c_char,
            (*((*s).seq).offset(off as isize)).name,
            tid,
            (*((*s).seq).offset(off as isize)).l_seq,
        );
        t = realtime();
    }
    j = 0 as libc::c_int;
    while j < *((*s).n_seg).offset(i as isize) {
        if *((*s).n_seg).offset(i as isize) == 2 as libc::c_int
            && (j == 0 as libc::c_int
                && pe_ori >> 1 as libc::c_int & 1 as libc::c_int != 0
                || j == 1 as libc::c_int && pe_ori & 1 as libc::c_int != 0)
        {
            mm_revcomp_bseq(&mut *((*s).seq).offset((off + j) as isize));
        }
        qlens[j as usize] = (*((*s).seq).offset((off + j) as isize)).l_seq;
        qseqs[j as usize] = (*((*s).seq).offset((off + j) as isize)).seq;
        j += 1;
        j;
    }
    if (*(*(*s).p).opt).flag & 0x20000 as libc::c_int as libc::c_long != 0 {
        j = 0 as libc::c_int;
        while j < *((*s).n_seg).offset(i as isize) {
            mm_map_frag(
                (*(*s).p).mi,
                1 as libc::c_int,
                &mut *qlens.as_mut_ptr().offset(j as isize),
                &mut *qseqs.as_mut_ptr().offset(j as isize),
                &mut *((*s).n_reg).offset((off + j) as isize),
                &mut *((*s).reg).offset((off + j) as isize),
                b,
                (*(*s).p).opt,
                (*((*s).seq).offset((off + j) as isize)).name,
            );
            *((*s).rep_len).offset((off + j) as isize) = (*b).rep_len;
            *((*s).frag_gap).offset((off + j) as isize) = (*b).frag_gap;
            j += 1;
            j;
        }
    } else {
        mm_map_frag(
            (*(*s).p).mi,
            *((*s).n_seg).offset(i as isize),
            qlens.as_mut_ptr(),
            qseqs.as_mut_ptr(),
            &mut *((*s).n_reg).offset(off as isize),
            &mut *((*s).reg).offset(off as isize),
            b,
            (*(*s).p).opt,
            (*((*s).seq).offset(off as isize)).name,
        );
        j = 0 as libc::c_int;
        while j < *((*s).n_seg).offset(i as isize) {
            *((*s).rep_len).offset((off + j) as isize) = (*b).rep_len;
            *((*s).frag_gap).offset((off + j) as isize) = (*b).frag_gap;
            j += 1;
            j;
        }
    }
    j = 0 as libc::c_int;
    while j < *((*s).n_seg).offset(i as isize) {
        if *((*s).n_seg).offset(i as isize) == 2 as libc::c_int
            && (j == 0 as libc::c_int
                && pe_ori >> 1 as libc::c_int & 1 as libc::c_int != 0
                || j == 1 as libc::c_int && pe_ori & 1 as libc::c_int != 0)
        {
            let mut k: libc::c_int = 0;
            let mut t_0: libc::c_int = 0;
            mm_revcomp_bseq(&mut *((*s).seq).offset((off + j) as isize));
            k = 0 as libc::c_int;
            while k < *((*s).n_reg).offset((off + j) as isize) {
                let mut r: *mut mm_reg1_t = &mut *(*((*s).reg)
                    .offset((off + j) as isize))
                    .offset(k as isize) as *mut mm_reg1_t;
                t_0 = (*r).qs;
                (*r).qs = qlens[j as usize] - (*r).qe;
                (*r).qe = qlens[j as usize] - t_0;
                (*r).set_rev(((*r).rev() == 0) as libc::c_int as uint32_t);
                k += 1;
                k;
            }
        }
        j += 1;
        j;
    }
    if mm_dbg_flag & 0x2 as libc::c_int != 0 {
        fprintf(
            stderr,
            b"QT\t%s\t%d\t%.6f\n\0" as *const u8 as *const libc::c_char,
            (*((*s).seq).offset(off as isize)).name,
            tid,
            realtime() - t,
        );
    }
}
unsafe extern "C" fn merge_hits(mut s: *mut step_t) {
    let mut f: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k0: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut max_seg: libc::c_int = 0 as libc::c_int;
    let mut n_reg_part: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rep_len_part: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut frag_gap_part: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut qlens: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut km: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut fp: *mut *mut FILE = (*(*s).p).fp_parts;
    let mut opt: *const mm_mapopt_t = (*(*s).p).opt;
    km = km_init();
    f = 0 as libc::c_int;
    while f < (*s).n_frag {
        max_seg = if max_seg > *((*s).n_seg).offset(f as isize) {
            max_seg
        } else {
            *((*s).n_seg).offset(f as isize)
        };
        f += 1;
        f;
    }
    qlens = calloc(
        (max_seg + (*(*s).p).n_parts * 3 as libc::c_int) as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    n_reg_part = qlens.offset(max_seg as isize);
    rep_len_part = n_reg_part.offset((*(*s).p).n_parts as isize);
    frag_gap_part = rep_len_part.offset((*(*s).p).n_parts as isize);
    f = 0 as libc::c_int;
    k0 = 0 as libc::c_int;
    k = k0;
    while f < (*s).n_frag {
        k0 = k;
        i = 0 as libc::c_int;
        while i < *((*s).n_seg).offset(f as isize) {
            let mut j: libc::c_int = 0;
            let mut l: libc::c_int = 0;
            let mut t: libc::c_int = 0;
            let mut rep_len: libc::c_int = 0 as libc::c_int;
            *qlens.offset(i as isize) = (*((*s).seq).offset(k as isize)).l_seq;
            j = 0 as libc::c_int;
            *((*s).n_reg).offset(k as isize) = 0 as libc::c_int;
            while j < (*(*s).p).n_parts {
                mm_err_fread(
                    &mut *n_reg_part.offset(j as isize) as *mut libc::c_int
                        as *mut libc::c_void,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    1 as libc::c_int as size_t,
                    *fp.offset(j as isize),
                );
                mm_err_fread(
                    &mut *rep_len_part.offset(j as isize) as *mut libc::c_int
                        as *mut libc::c_void,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    1 as libc::c_int as size_t,
                    *fp.offset(j as isize),
                );
                mm_err_fread(
                    &mut *frag_gap_part.offset(j as isize) as *mut libc::c_int
                        as *mut libc::c_void,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    1 as libc::c_int as size_t,
                    *fp.offset(j as isize),
                );
                *((*s).n_reg).offset(k as isize) += *n_reg_part.offset(j as isize);
                if rep_len < *rep_len_part.offset(j as isize) {
                    rep_len = *rep_len_part.offset(j as isize);
                }
                j += 1;
                j;
            }
            let ref mut fresh9 = *((*s).reg).offset(k as isize);
            *fresh9 = calloc(
                *((*s).n_reg).offset(k as isize) as libc::c_ulong,
                ::std::mem::size_of::<mm_reg1_t>() as libc::c_ulong,
            ) as *mut mm_reg1_t;
            j = 0 as libc::c_int;
            l = 0 as libc::c_int;
            while j < (*(*s).p).n_parts {
                t = 0 as libc::c_int;
                while t < *n_reg_part.offset(j as isize) {
                    let mut r: *mut mm_reg1_t = &mut *(*((*s).reg).offset(k as isize))
                        .offset(l as isize) as *mut mm_reg1_t;
                    let mut capacity: uint32_t = 0;
                    mm_err_fread(
                        r as *mut libc::c_void,
                        ::std::mem::size_of::<mm_reg1_t>() as libc::c_ulong,
                        1 as libc::c_int as size_t,
                        *fp.offset(j as isize),
                    );
                    (*r)
                        .rid = ((*r).rid as libc::c_uint)
                        .wrapping_add(*((*(*s).p).rid_shift).offset(j as isize))
                        as int32_t as int32_t;
                    if (*opt).flag & 0x4 as libc::c_int as libc::c_long != 0 {
                        mm_err_fread(
                            &mut capacity as *mut uint32_t as *mut libc::c_void,
                            4 as libc::c_int as size_t,
                            1 as libc::c_int as size_t,
                            *fp.offset(j as isize),
                        );
                        (*r)
                            .p = calloc(
                            capacity as libc::c_ulong,
                            4 as libc::c_int as libc::c_ulong,
                        ) as *mut mm_extra_t;
                        (*(*r).p).capacity = capacity;
                        mm_err_fread(
                            (*r).p as *mut libc::c_void,
                            (*(*r).p).capacity as size_t,
                            4 as libc::c_int as size_t,
                            *fp.offset(j as isize),
                        );
                    }
                    t += 1;
                    t;
                    l += 1;
                    l;
                }
                j += 1;
                j;
            }
            if (*opt).flag & 0x1000 as libc::c_int as libc::c_long == 0
                && (*((*s).seq).offset(k as isize)).l_seq >= (*opt).rank_min_len
            {
                mm_update_dp_max(
                    (*((*s).seq).offset(k as isize)).l_seq,
                    *((*s).n_reg).offset(k as isize),
                    *((*s).reg).offset(k as isize),
                    (*opt).rank_frac,
                    (*opt).a,
                    (*opt).b,
                );
            }
            j = 0 as libc::c_int;
            while j < *((*s).n_reg).offset(k as isize) {
                let mut r_0: *mut mm_reg1_t = &mut *(*((*s).reg).offset(k as isize))
                    .offset(j as isize) as *mut mm_reg1_t;
                if !((*r_0).p).is_null() {
                    (*(*r_0).p).dp_max2 = 0 as libc::c_int;
                }
                (*r_0).subsc = 0 as libc::c_int;
                (*r_0).n_sub = 0 as libc::c_int;
                j += 1;
                j;
            }
            mm_hit_sort(
                km,
                &mut *((*s).n_reg).offset(k as isize),
                *((*s).reg).offset(k as isize),
                (*opt).alt_drop,
            );
            mm_set_parent(
                km,
                (*opt).mask_level,
                (*opt).mask_len,
                *((*s).n_reg).offset(k as isize),
                *((*s).reg).offset(k as isize),
                (*opt).a * 2 as libc::c_int + (*opt).b,
                ((*opt).flag & 0x20000000 as libc::c_int as libc::c_long) as libc::c_int,
                (*opt).alt_drop,
            );
            if (*opt).flag & 0x800000 as libc::c_int as libc::c_long == 0 {
                mm_select_sub(
                    km,
                    (*opt).pri_ratio,
                    (*(*(*s).p).mi).k * 2 as libc::c_int,
                    (*opt).best_n,
                    0 as libc::c_int,
                    ((*opt).max_gap as libc::c_double * 0.8f64) as libc::c_int,
                    &mut *((*s).n_reg).offset(k as isize),
                    *((*s).reg).offset(k as isize),
                );
                mm_set_sam_pri(
                    *((*s).n_reg).offset(k as isize),
                    *((*s).reg).offset(k as isize),
                );
            }
            mm_set_mapq(
                km,
                *((*s).n_reg).offset(k as isize),
                *((*s).reg).offset(k as isize),
                (*opt).min_chain_score,
                (*opt).a,
                rep_len,
                ((*opt).flag & 0x1000 as libc::c_int as libc::c_long != 0) as libc::c_int,
            );
            i += 1;
            i;
            k += 1;
            k;
        }
        if *((*s).n_seg).offset(f as isize) == 2 as libc::c_int
            && (*opt).pe_ori >= 0 as libc::c_int
            && (*opt).flag & 0x4 as libc::c_int as libc::c_long != 0
        {
            mm_pair(
                km,
                *frag_gap_part.offset(0 as libc::c_int as isize),
                (*opt).pe_bonus,
                (*opt).a * 2 as libc::c_int + (*opt).b,
                (*opt).a,
                qlens,
                &mut *((*s).n_reg).offset(k0 as isize),
                &mut *((*s).reg).offset(k0 as isize),
            );
        }
        f += 1;
        f;
    }
    free(qlens as *mut libc::c_void);
    km_destroy(km);
}
unsafe extern "C" fn worker_pipeline(
    mut shared: *mut libc::c_void,
    mut step: libc::c_int,
    mut in_0: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut p: *mut pipeline_t = shared as *mut pipeline_t;
    if step == 0 as libc::c_int {
        let mut with_qual: libc::c_int = ((*(*p).opt).flag
            & 0x8 as libc::c_int as libc::c_long != 0
            && (*(*p).opt).flag & 0x10 as libc::c_int as libc::c_long == 0)
            as libc::c_int;
        let mut with_comment: libc::c_int = ((*(*p).opt).flag
            & 0x2000000 as libc::c_int as libc::c_long != 0) as libc::c_int;
        let mut frag_mode: libc::c_int = ((*p).n_fp > 1 as libc::c_int
            || (*(*p).opt).flag & 0x2000 as libc::c_int as libc::c_long != 0)
            as libc::c_int;
        let mut s: *mut step_t = 0 as *mut step_t;
        s = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<step_t>() as libc::c_ulong,
        ) as *mut step_t;
        if (*p).n_fp > 1 as libc::c_int {
            (*s)
                .seq = mm_bseq_read_frag2(
                (*p).n_fp,
                (*p).fp,
                (*p).mini_batch_size,
                with_qual,
                with_comment,
                &mut (*s).n_seq,
            );
        } else {
            (*s)
                .seq = mm_bseq_read3(
                *((*p).fp).offset(0 as libc::c_int as isize),
                (*p).mini_batch_size,
                with_qual,
                with_comment,
                frag_mode,
                &mut (*s).n_seq,
            );
        }
        if !((*s).seq).is_null() {
            (*s).p = p;
            i = 0 as libc::c_int;
            while i < (*s).n_seq {
                let fresh10 = (*p).n_processed;
                (*p).n_processed = (*p).n_processed + 1;
                (*((*s).seq).offset(i as isize)).rid = fresh10;
                i += 1;
                i;
            }
            (*s)
                .buf = calloc(
                (*p).n_threads as libc::c_ulong,
                ::std::mem::size_of::<*mut mm_tbuf_t>() as libc::c_ulong,
            ) as *mut *mut mm_tbuf_t;
            i = 0 as libc::c_int;
            while i < (*p).n_threads {
                let ref mut fresh11 = *((*s).buf).offset(i as isize);
                *fresh11 = mm_tbuf_init();
                i += 1;
                i;
            }
            (*s)
                .n_reg = calloc(
                (5 as libc::c_int * (*s).n_seq) as libc::c_ulong,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            ) as *mut libc::c_int;
            (*s).seg_off = ((*s).n_reg).offset((*s).n_seq as isize);
            (*s).n_seg = ((*s).seg_off).offset((*s).n_seq as isize);
            (*s).rep_len = ((*s).n_seg).offset((*s).n_seq as isize);
            (*s).frag_gap = ((*s).rep_len).offset((*s).n_seq as isize);
            (*s)
                .reg = calloc(
                (*s).n_seq as libc::c_ulong,
                ::std::mem::size_of::<*mut mm_reg1_t>() as libc::c_ulong,
            ) as *mut *mut mm_reg1_t;
            i = 1 as libc::c_int;
            j = 0 as libc::c_int;
            while i <= (*s).n_seq {
                if i == (*s).n_seq || frag_mode == 0
                    || mm_qname_same(
                        (*((*s).seq).offset((i - 1 as libc::c_int) as isize)).name,
                        (*((*s).seq).offset(i as isize)).name,
                    ) == 0
                {
                    *((*s).n_seg).offset((*s).n_frag as isize) = i - j;
                    let fresh12 = (*s).n_frag;
                    (*s).n_frag = (*s).n_frag + 1;
                    *((*s).seg_off).offset(fresh12 as isize) = j;
                    j = i;
                }
                i += 1;
                i;
            }
            return s as *mut libc::c_void;
        } else {
            free(s as *mut libc::c_void);
        }
    } else if step == 1 as libc::c_int {
        if (*p).n_parts > 0 as libc::c_int {
            merge_hits(in_0 as *mut step_t);
        } else {
            kt_for(
                (*p).n_threads,
                Some(
                    worker_for
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            libc::c_long,
                            libc::c_int,
                        ) -> (),
                ),
                in_0,
                (*(in_0 as *mut step_t)).n_frag as libc::c_long,
            );
        }
        return in_0;
    } else if step == 2 as libc::c_int {
        let mut km: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut s_0: *mut step_t = in_0 as *mut step_t;
        let mut mi: *const mm_idx_t = (*p).mi;
        i = 0 as libc::c_int;
        while i < (*p).n_threads {
            mm_tbuf_destroy(*((*s_0).buf).offset(i as isize));
            i += 1;
            i;
        }
        free((*s_0).buf as *mut libc::c_void);
        if (*(*p).opt).flag & 0x40 as libc::c_int as libc::c_long != 0
            && mm_dbg_flag & 0x1 as libc::c_int == 0
        {
            km = km_init();
        }
        k = 0 as libc::c_int;
        while k < (*s_0).n_frag {
            let mut seg_st: libc::c_int = *((*s_0).seg_off).offset(k as isize);
            let mut seg_en: libc::c_int = *((*s_0).seg_off).offset(k as isize)
                + *((*s_0).n_seg).offset(k as isize);
            i = seg_st;
            while i < seg_en {
                let mut t: *mut mm_bseq1_t = &mut *((*s_0).seq).offset(i as isize)
                    as *mut mm_bseq1_t;
                if !((*(*p).opt).split_prefix).is_null()
                    && (*p).n_parts == 0 as libc::c_int
                {
                    mm_err_fwrite(
                        &mut *((*s_0).n_reg).offset(i as isize) as *mut libc::c_int
                            as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        1 as libc::c_int as size_t,
                        (*p).fp_split,
                    );
                    mm_err_fwrite(
                        &mut *((*s_0).rep_len).offset(i as isize) as *mut libc::c_int
                            as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        1 as libc::c_int as size_t,
                        (*p).fp_split,
                    );
                    mm_err_fwrite(
                        &mut *((*s_0).frag_gap).offset(i as isize) as *mut libc::c_int
                            as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        1 as libc::c_int as size_t,
                        (*p).fp_split,
                    );
                    j = 0 as libc::c_int;
                    while j < *((*s_0).n_reg).offset(i as isize) {
                        let mut r: *mut mm_reg1_t = &mut *(*((*s_0).reg)
                            .offset(i as isize))
                            .offset(j as isize) as *mut mm_reg1_t;
                        mm_err_fwrite(
                            r as *const libc::c_void,
                            ::std::mem::size_of::<mm_reg1_t>() as libc::c_ulong,
                            1 as libc::c_int as size_t,
                            (*p).fp_split,
                        );
                        if (*(*p).opt).flag & 0x4 as libc::c_int as libc::c_long != 0 {
                            mm_err_fwrite(
                                &mut (*(*r).p).capacity as *mut uint32_t
                                    as *const libc::c_void,
                                4 as libc::c_int as size_t,
                                1 as libc::c_int as size_t,
                                (*p).fp_split,
                            );
                            mm_err_fwrite(
                                (*r).p as *const libc::c_void,
                                (*(*r).p).capacity as size_t,
                                4 as libc::c_int as size_t,
                                (*p).fp_split,
                            );
                        }
                        j += 1;
                        j;
                    }
                } else if *((*s_0).n_reg).offset(i as isize) > 0 as libc::c_int {
                    j = 0 as libc::c_int;
                    while j < *((*s_0).n_reg).offset(i as isize) {
                        let mut r_0: *mut mm_reg1_t = &mut *(*((*s_0).reg)
                            .offset(i as isize))
                            .offset(j as isize) as *mut mm_reg1_t;
                        if (*r_0).sam_pri() == 0 || (*r_0).id == (*r_0).parent {} else {
                            __assert_fail(
                                b"!r->sam_pri || r->id == r->parent\0" as *const u8
                                    as *const libc::c_char,
                                b"map.c\0" as *const u8 as *const libc::c_char,
                                589 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 43],
                                    &[libc::c_char; 43],
                                >(b"void *worker_pipeline(void *, int, void *)\0"))
                                    .as_ptr(),
                            );
                        }
                        'c_8203: {
                            if (*r_0).sam_pri() == 0 || (*r_0).id == (*r_0).parent
                            {} else {
                                __assert_fail(
                                    b"!r->sam_pri || r->id == r->parent\0" as *const u8
                                        as *const libc::c_char,
                                    b"map.c\0" as *const u8 as *const libc::c_char,
                                    589 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 43],
                                        &[libc::c_char; 43],
                                    >(b"void *worker_pipeline(void *, int, void *)\0"))
                                        .as_ptr(),
                                );
                            }
                        };
                        if !((*(*p).opt).flag & 0x4000 as libc::c_int as libc::c_long
                            != 0 && (*r_0).id != (*r_0).parent)
                        {
                            if (*(*p).opt).flag & 0x8 as libc::c_int as libc::c_long != 0
                            {
                                mm_write_sam3(
                                    &mut (*p).str_0,
                                    mi,
                                    t,
                                    i - seg_st,
                                    j,
                                    *((*s_0).n_seg).offset(k as isize),
                                    &mut *((*s_0).n_reg).offset(seg_st as isize),
                                    &mut *((*s_0).reg).offset(seg_st as isize)
                                        as *mut *mut mm_reg1_t as *const *const mm_reg1_t,
                                    km,
                                    (*(*p).opt).flag,
                                    *((*s_0).rep_len).offset(i as isize),
                                );
                            } else {
                                mm_write_paf3(
                                    &mut (*p).str_0,
                                    mi,
                                    t,
                                    r_0,
                                    km,
                                    (*(*p).opt).flag,
                                    *((*s_0).rep_len).offset(i as isize),
                                );
                            }
                            mm_err_puts((*p).str_0.s);
                        }
                        j += 1;
                        j;
                    }
                } else if (*(*p).opt).flag & 0x8000000 as libc::c_int as libc::c_long
                    != 0
                    || (*(*p).opt).flag & 0x8 as libc::c_int as libc::c_long != 0
                        && (*(*p).opt).flag & 0x40000000 as libc::c_int as libc::c_long
                            == 0
                {
                    if (*(*p).opt).flag & 0x8 as libc::c_int as libc::c_long != 0 {
                        mm_write_sam3(
                            &mut (*p).str_0,
                            mi,
                            t,
                            i - seg_st,
                            -(1 as libc::c_int),
                            *((*s_0).n_seg).offset(k as isize),
                            &mut *((*s_0).n_reg).offset(seg_st as isize),
                            &mut *((*s_0).reg).offset(seg_st as isize)
                                as *mut *mut mm_reg1_t as *const *const mm_reg1_t,
                            km,
                            (*(*p).opt).flag,
                            *((*s_0).rep_len).offset(i as isize),
                        );
                    } else {
                        mm_write_paf3(
                            &mut (*p).str_0,
                            mi,
                            t,
                            0 as *const mm_reg1_t,
                            0 as *mut libc::c_void,
                            (*(*p).opt).flag,
                            *((*s_0).rep_len).offset(i as isize),
                        );
                    }
                    mm_err_puts((*p).str_0.s);
                }
                i += 1;
                i;
            }
            i = seg_st;
            while i < seg_en {
                j = 0 as libc::c_int;
                while j < *((*s_0).n_reg).offset(i as isize) {
                    free(
                        (*(*((*s_0).reg).offset(i as isize)).offset(j as isize)).p
                            as *mut libc::c_void,
                    );
                    j += 1;
                    j;
                }
                free(*((*s_0).reg).offset(i as isize) as *mut libc::c_void);
                free((*((*s_0).seq).offset(i as isize)).seq as *mut libc::c_void);
                free((*((*s_0).seq).offset(i as isize)).name as *mut libc::c_void);
                if !((*((*s_0).seq).offset(i as isize)).qual).is_null() {
                    free((*((*s_0).seq).offset(i as isize)).qual as *mut libc::c_void);
                }
                if !((*((*s_0).seq).offset(i as isize)).comment).is_null() {
                    free(
                        (*((*s_0).seq).offset(i as isize)).comment as *mut libc::c_void,
                    );
                }
                i += 1;
                i;
            }
            k += 1;
            k;
        }
        free((*s_0).reg as *mut libc::c_void);
        free((*s_0).n_reg as *mut libc::c_void);
        free((*s_0).seq as *mut libc::c_void);
        km_destroy(km);
        if mm_verbose >= 3 as libc::c_int {
            fprintf(
                stderr,
                b"[M::%s::%.3f*%.2f] mapped %d sequences\n\0" as *const u8
                    as *const libc::c_char,
                (*::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"worker_pipeline\0"))
                    .as_ptr(),
                realtime() - mm_realtime0,
                cputime() / (realtime() - mm_realtime0),
                (*s_0).n_seq,
            );
        }
        free(s_0 as *mut libc::c_void);
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn open_bseqs(
    mut n: libc::c_int,
    mut fn_0: *mut *const libc::c_char,
) -> *mut *mut mm_bseq_file_t {
    let mut fp: *mut *mut mm_bseq_file_t = 0 as *mut *mut mm_bseq_file_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    fp = calloc(
        n as libc::c_ulong,
        ::std::mem::size_of::<*mut mm_bseq_file_t>() as libc::c_ulong,
    ) as *mut *mut mm_bseq_file_t;
    i = 0 as libc::c_int;
    while i < n {
        let ref mut fresh13 = *fp.offset(i as isize);
        *fresh13 = mm_bseq_open(*fn_0.offset(i as isize));
        if (*fresh13).is_null() {
            if mm_verbose >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"ERROR: failed to open file '%s': %s\n\0" as *const u8
                        as *const libc::c_char,
                    *fn_0.offset(i as isize),
                    strerror(*__errno_location()),
                );
            }
            j = 0 as libc::c_int;
            while j < i {
                mm_bseq_close(*fp.offset(j as isize));
                j += 1;
                j;
            }
            free(fp as *mut libc::c_void);
            return 0 as *mut *mut mm_bseq_file_t;
        }
        i += 1;
        i;
    }
    return fp;
}
pub unsafe extern "C" fn mm_map_file_frag(
    mut idx: *const mm_idx_t,
    mut n_segs: libc::c_int,
    mut fn_0: *mut *const libc::c_char,
    mut opt: *const mm_mapopt_t,
    mut n_threads: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut pl_threads: libc::c_int = 0;
    let mut pl: pipeline_t = pipeline_t {
        n_processed: 0,
        n_threads: 0,
        n_fp: 0,
        mini_batch_size: 0,
        opt: 0 as *const mm_mapopt_t,
        fp: 0 as *mut *mut mm_bseq_file_t,
        mi: 0 as *const mm_idx_t,
        str_0: kstring_t {
            l: 0,
            m: 0,
            s: 0 as *mut libc::c_char,
        },
        n_parts: 0,
        rid_shift: 0 as *mut uint32_t,
        fp_split: 0 as *mut FILE,
        fp_parts: 0 as *mut *mut FILE,
    };
    if n_segs < 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    memset(
        &mut pl as *mut pipeline_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<pipeline_t>() as libc::c_ulong,
    );
    pl.n_fp = n_segs;
    pl.fp = open_bseqs(pl.n_fp, fn_0);
    if (pl.fp).is_null() {
        return -(1 as libc::c_int);
    }
    pl.opt = opt;
    pl.mi = idx;
    pl
        .n_threads = if n_threads > 1 as libc::c_int {
        n_threads
    } else {
        1 as libc::c_int
    };
    pl.mini_batch_size = (*opt).mini_batch_size;
    if !((*opt).split_prefix).is_null() {
        pl.fp_split = mm_split_init((*opt).split_prefix, idx);
    }
    pl_threads = if n_threads == 1 as libc::c_int {
        1 as libc::c_int
    } else if (*opt).flag & 0x8000 as libc::c_int as libc::c_long != 0 {
        3 as libc::c_int
    } else {
        2 as libc::c_int
    };
    kt_pipeline(
        pl_threads,
        Some(
            worker_pipeline
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
        &mut pl as *mut pipeline_t as *mut libc::c_void,
        3 as libc::c_int,
    );
    free(pl.str_0.s as *mut libc::c_void);
    if !(pl.fp_split).is_null() {
        fclose(pl.fp_split);
    }
    i = 0 as libc::c_int;
    while i < pl.n_fp {
        mm_bseq_close(*(pl.fp).offset(i as isize));
        i += 1;
        i;
    }
    free(pl.fp as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mm_map_file(
    mut idx: *const mm_idx_t,
    mut fn_0: *const libc::c_char,
    mut opt: *const mm_mapopt_t,
    mut n_threads: libc::c_int,
) -> libc::c_int {
    return mm_map_file_frag(idx, 1 as libc::c_int, &mut fn_0, opt, n_threads);
}
pub unsafe extern "C" fn mm_split_merge(
    mut n_segs: libc::c_int,
    mut fn_0: *mut *const libc::c_char,
    mut opt: *const mm_mapopt_t,
    mut n_split_idx: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut pl: pipeline_t = pipeline_t {
        n_processed: 0,
        n_threads: 0,
        n_fp: 0,
        mini_batch_size: 0,
        opt: 0 as *const mm_mapopt_t,
        fp: 0 as *mut *mut mm_bseq_file_t,
        mi: 0 as *const mm_idx_t,
        str_0: kstring_t {
            l: 0,
            m: 0,
            s: 0 as *mut libc::c_char,
        },
        n_parts: 0,
        rid_shift: 0 as *mut uint32_t,
        fp_split: 0 as *mut FILE,
        fp_parts: 0 as *mut *mut FILE,
    };
    let mut mi: *mut mm_idx_t = 0 as *mut mm_idx_t;
    if n_segs < 1 as libc::c_int || n_split_idx < 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    memset(
        &mut pl as *mut pipeline_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<pipeline_t>() as libc::c_ulong,
    );
    pl.n_fp = n_segs;
    pl.fp = open_bseqs(pl.n_fp, fn_0);
    if (pl.fp).is_null() {
        return -(1 as libc::c_int);
    }
    pl.opt = opt;
    pl.mini_batch_size = (*opt).mini_batch_size;
    pl.n_parts = n_split_idx;
    pl
        .fp_parts = calloc(
        pl.n_parts as libc::c_ulong,
        ::std::mem::size_of::<*mut FILE>() as libc::c_ulong,
    ) as *mut *mut FILE;
    pl
        .rid_shift = calloc(
        pl.n_parts as libc::c_ulong,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
    ) as *mut uint32_t;
    mi = mm_split_merge_prep(
        (*opt).split_prefix,
        n_split_idx,
        pl.fp_parts,
        pl.rid_shift,
    );
    pl.mi = mi;
    if (pl.mi).is_null() {
        free(pl.fp_parts as *mut libc::c_void);
        free(pl.rid_shift as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    i = n_split_idx - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        *(pl.rid_shift)
            .offset(
                i as isize,
            ) = *(pl.rid_shift).offset((i - 1 as libc::c_int) as isize);
        i -= 1;
        i;
    }
    *(pl.rid_shift).offset(0 as libc::c_int as isize) = 0 as libc::c_int as uint32_t;
    i = 1 as libc::c_int;
    while i < n_split_idx {
        let ref mut fresh14 = *(pl.rid_shift).offset(i as isize);
        *fresh14 = (*fresh14 as libc::c_uint)
            .wrapping_add(*(pl.rid_shift).offset((i - 1 as libc::c_int) as isize))
            as uint32_t as uint32_t;
        i += 1;
        i;
    }
    if (*opt).flag & 0x8 as libc::c_int as libc::c_long != 0 {
        i = 0 as libc::c_int;
        while i < (*pl.mi).n_seq as int32_t {
            printf(
                b"@SQ\tSN:%s\tLN:%d\n\0" as *const u8 as *const libc::c_char,
                (*((*pl.mi).seq).offset(i as isize)).name,
                (*((*pl.mi).seq).offset(i as isize)).len,
            );
            i += 1;
            i;
        }
    }
    kt_pipeline(
        2 as libc::c_int,
        Some(
            worker_pipeline
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
        &mut pl as *mut pipeline_t as *mut libc::c_void,
        3 as libc::c_int,
    );
    free(pl.str_0.s as *mut libc::c_void);
    mm_idx_destroy(mi);
    free(pl.rid_shift as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < n_split_idx {
        fclose(*(pl.fp_parts).offset(i as isize));
        i += 1;
        i;
    }
    free(pl.fp_parts as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < pl.n_fp {
        mm_bseq_close(*(pl.fp).offset(i as isize));
        i += 1;
        i;
    }
    free(pl.fp as *mut libc::c_void);
    mm_split_rm_tmp((*opt).split_prefix, n_split_idx);
    return 0 as libc::c_int;
}
