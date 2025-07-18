use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type mm_bseq_file_s;
    pub type mm_idx_intv_s;
    pub type mm_idx_bucket_s;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn exit(_: libc::c_int) -> !;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn freopen(
        __filename: *const libc::c_char,
        __modes: *const libc::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    static mut mm_verbose: libc::c_int;
    static mut mm_dbg_flag: libc::c_int;
    static mut mm_realtime0: libc::c_double;
    fn mm_set_opt(
        preset: *const libc::c_char,
        io: *mut mm_idxopt_t,
        mo: *mut mm_mapopt_t,
    ) -> libc::c_int;
    fn mm_check_opt(io: *const mm_idxopt_t, mo: *const mm_mapopt_t) -> libc::c_int;
    fn mm_mapopt_update(opt: *mut mm_mapopt_t, mi: *const mm_idx_t);
    fn mm_mapopt_max_intron_len(opt: *mut mm_mapopt_t, max_intron_len: libc::c_int);
    fn mm_idx_reader_open(
        fn_0: *const libc::c_char,
        opt: *const mm_idxopt_t,
        fn_out: *const libc::c_char,
    ) -> *mut mm_idx_reader_t;
    fn mm_idx_reader_read(
        r: *mut mm_idx_reader_t,
        n_threads: libc::c_int,
    ) -> *mut mm_idx_t;
    fn mm_idx_reader_close(r: *mut mm_idx_reader_t);
    fn mm_idx_reader_eof(r: *const mm_idx_reader_t) -> libc::c_int;
    fn mm_idx_stat(idx: *const mm_idx_t);
    fn mm_idx_destroy(mi: *mut mm_idx_t);
    fn mm_map_file(
        idx: *const mm_idx_t,
        fn_0: *const libc::c_char,
        opt: *const mm_mapopt_t,
        n_threads: libc::c_int,
    ) -> libc::c_int;
    fn mm_map_file_frag(
        idx: *const mm_idx_t,
        n_segs: libc::c_int,
        fn_0: *mut *const libc::c_char,
        opt: *const mm_mapopt_t,
        n_threads: libc::c_int,
    ) -> libc::c_int;
    fn mm_idx_alt_read(mi: *mut mm_idx_t, fn_0: *const libc::c_char) -> libc::c_int;
    fn mm_idx_bed_read(
        mi: *mut mm_idx_t,
        fn_0: *const libc::c_char,
        read_junc: libc::c_int,
    ) -> libc::c_int;
    fn realtime() -> libc::c_double;
    fn cputime() -> libc::c_double;
    fn peakrss() -> libc::c_long;
    fn mm_write_sam_hdr(
        mi: *const mm_idx_t,
        rg: *const libc::c_char,
        ver: *const libc::c_char,
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn mm_split_merge(
        n_segs: libc::c_int,
        fn_0: *mut *const libc::c_char,
        opt: *const mm_mapopt_t,
        n_split_idx: libc::c_int,
    ) -> libc::c_int;
    fn getrlimit(__resource: __rlimit_resource_t, __rlimits: *mut rlimit) -> libc::c_int;
    fn setrlimit(
        __resource: __rlimit_resource_t,
        __rlimits: *const rlimit,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __rlim_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_idx_reader_t {
    pub is_idx: libc::c_int,
    pub n_parts: libc::c_int,
    pub idx_size: int64_t,
    pub opt: mm_idxopt_t,
    pub fp_out: *mut FILE,
    pub fp: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub seq: *mut mm_bseq_file_s,
    pub idx: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ketopt_t {
    pub ind: libc::c_int,
    pub opt: libc::c_int,
    pub arg: *mut libc::c_char,
    pub longidx: libc::c_int,
    pub i: libc::c_int,
    pub pos: libc::c_int,
    pub n_args: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ko_longopt_t {
    pub name: *mut libc::c_char,
    pub has_arg: libc::c_int,
    pub val: libc::c_int,
}
pub type __rlimit_resource = libc::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type rlim_t = __rlim_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = libc::c_int;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
static mut KETOPT_INIT: ketopt_t = {
    let mut init = ketopt_t {
        ind: 1 as libc::c_int,
        opt: 0 as libc::c_int,
        arg: 0 as *const libc::c_char as *mut libc::c_char,
        longidx: -(1 as libc::c_int),
        i: 1 as libc::c_int,
        pos: 0 as libc::c_int,
        n_args: 0 as libc::c_int,
    };
    init
};
unsafe extern "C" fn ketopt_permute(
    mut argv: *mut *mut libc::c_char,
    mut j: libc::c_int,
    mut n: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut p: *mut libc::c_char = *argv.offset(j as isize);
    k = 0 as libc::c_int;
    while k < n {
        let ref mut fresh0 = *argv.offset((j - k) as isize);
        *fresh0 = *argv.offset((j - k - 1 as libc::c_int) as isize);
        k += 1;
        k;
    }
    let ref mut fresh1 = *argv.offset((j - k) as isize);
    *fresh1 = p;
}
unsafe extern "C" fn ketopt(
    mut s: *mut ketopt_t,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut permute: libc::c_int,
    mut ostr: *const libc::c_char,
    mut longopts: *const ko_longopt_t,
) -> libc::c_int {
    let mut opt: libc::c_int = -(1 as libc::c_int);
    let mut i0: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if permute != 0 {
        while (*s).i < argc
            && (*(*argv.offset((*s).i as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int != '-' as i32
                || *(*argv.offset((*s).i as isize)).offset(1 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32)
        {
            (*s).i += 1;
            (*s).i;
            (*s).n_args += 1;
            (*s).n_args;
        }
    }
    (*s).arg = 0 as *mut libc::c_char;
    (*s).longidx = -(1 as libc::c_int);
    i0 = (*s).i;
    if (*s).i >= argc
        || *(*argv.offset((*s).i as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int != '-' as i32
        || *(*argv.offset((*s).i as isize)).offset(1 as libc::c_int as isize)
            as libc::c_int == '\0' as i32
    {
        (*s).ind = (*s).i - (*s).n_args;
        return -(1 as libc::c_int);
    }
    if *(*argv.offset((*s).i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
        == '-' as i32
        && *(*argv.offset((*s).i as isize)).offset(1 as libc::c_int as isize)
            as libc::c_int == '-' as i32
    {
        if *(*argv.offset((*s).i as isize)).offset(2 as libc::c_int as isize)
            as libc::c_int == '\0' as i32
        {
            ketopt_permute(argv, (*s).i, (*s).n_args);
            (*s).i += 1;
            (*s).i;
            (*s).ind = (*s).i - (*s).n_args;
            return -(1 as libc::c_int);
        }
        (*s).opt = 0 as libc::c_int;
        opt = '?' as i32;
        (*s).pos = -(1 as libc::c_int);
        if !longopts.is_null() {
            let mut k: libc::c_int = 0;
            let mut n_exact: libc::c_int = 0 as libc::c_int;
            let mut n_partial: libc::c_int = 0 as libc::c_int;
            let mut o: *const ko_longopt_t = 0 as *const ko_longopt_t;
            let mut o_exact: *const ko_longopt_t = 0 as *const ko_longopt_t;
            let mut o_partial: *const ko_longopt_t = 0 as *const ko_longopt_t;
            j = 2 as libc::c_int;
            while *(*argv.offset((*s).i as isize)).offset(j as isize) as libc::c_int
                != '\0' as i32
                && *(*argv.offset((*s).i as isize)).offset(j as isize) as libc::c_int
                    != '=' as i32
            {
                j += 1;
                j;
            }
            k = 0 as libc::c_int;
            while !((*longopts.offset(k as isize)).name).is_null() {
                if strncmp(
                    &mut *(*argv.offset((*s).i as isize))
                        .offset(2 as libc::c_int as isize),
                    (*longopts.offset(k as isize)).name,
                    (j - 2 as libc::c_int) as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    if *((*longopts.offset(k as isize)).name)
                        .offset((j - 2 as libc::c_int) as isize) as libc::c_int
                        == 0 as libc::c_int
                    {
                        n_exact += 1;
                        n_exact;
                        o_exact = &*longopts.offset(k as isize) as *const ko_longopt_t;
                    } else {
                        n_partial += 1;
                        n_partial;
                        o_partial = &*longopts.offset(k as isize) as *const ko_longopt_t;
                    }
                }
                k += 1;
                k;
            }
            if n_exact > 1 as libc::c_int
                || n_exact == 0 as libc::c_int && n_partial > 1 as libc::c_int
            {
                return '?' as i32;
            }
            o = if n_exact == 1 as libc::c_int {
                o_exact
            } else if n_partial == 1 as libc::c_int {
                o_partial
            } else {
                0 as *const ko_longopt_t
            };
            if !o.is_null() {
                opt = (*o).val;
                (*s).opt = opt;
                (*s).longidx = o.offset_from(longopts) as libc::c_long as libc::c_int;
                if *(*argv.offset((*s).i as isize)).offset(j as isize) as libc::c_int
                    == '=' as i32
                {
                    (*s)
                        .arg = &mut *(*argv.offset((*s).i as isize))
                        .offset((j + 1 as libc::c_int) as isize) as *mut libc::c_char;
                }
                if (*o).has_arg == 1 as libc::c_int
                    && *(*argv.offset((*s).i as isize)).offset(j as isize) as libc::c_int
                        == '\0' as i32
                {
                    if (*s).i < argc - 1 as libc::c_int {
                        (*s).i += 1;
                        (*s).arg = *argv.offset((*s).i as isize);
                    } else {
                        opt = ':' as i32;
                    }
                }
            }
        }
    } else {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        if (*s).pos == 0 as libc::c_int {
            (*s).pos = 1 as libc::c_int;
        }
        let fresh2 = (*s).pos;
        (*s).pos = (*s).pos + 1;
        (*s)
            .opt = *(*argv.offset((*s).i as isize)).offset(fresh2 as isize)
            as libc::c_int;
        opt = (*s).opt;
        p = strchr(ostr as *mut libc::c_char, opt);
        if p.is_null() {
            opt = '?' as i32;
        } else if *p.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32 {
            if *(*argv.offset((*s).i as isize)).offset((*s).pos as isize) as libc::c_int
                == 0 as libc::c_int
            {
                if (*s).i < argc - 1 as libc::c_int {
                    (*s).i += 1;
                    (*s).arg = *argv.offset((*s).i as isize);
                } else {
                    opt = ':' as i32;
                }
            } else {
                (*s)
                    .arg = &mut *(*argv.offset((*s).i as isize))
                    .offset((*s).pos as isize) as *mut libc::c_char;
            }
            (*s).pos = -(1 as libc::c_int);
        }
    }
    if (*s).pos < 0 as libc::c_int
        || *(*argv.offset((*s).i as isize)).offset((*s).pos as isize) as libc::c_int
            == 0 as libc::c_int
    {
        (*s).i += 1;
        (*s).i;
        (*s).pos = 0 as libc::c_int;
        if (*s).n_args > 0 as libc::c_int {
            j = i0;
            while j < (*s).i {
                ketopt_permute(argv, j, (*s).n_args);
                j += 1;
                j;
            }
        }
    }
    (*s).ind = (*s).i - (*s).n_args;
    return opt;
}
pub unsafe extern "C" fn liftrlimit() {
    let mut r: rlimit = rlimit { rlim_cur: 0, rlim_max: 0 };
    getrlimit(RLIMIT_AS as libc::c_int, &mut r);
    r.rlim_cur = r.rlim_max;
    setrlimit(RLIMIT_AS as libc::c_int, &mut r);
}
static mut long_options: [ko_longopt_t; 63] = [
    {
        let mut init = ko_longopt_t {
            name: b"bucket-bits\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 300 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"mb-size\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 'K' as i32,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"seed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 302 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"no-kalloc\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 303 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"print-qname\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 304 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"no-self\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 'D' as i32,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"print-seeds\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 306 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"max-chain-skip\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 307 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"min-dp-len\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 308 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"print-aln-seq\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 309 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"splice\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 310 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"cost-non-gt-ag\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 'C' as i32,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"no-long-join\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 312 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"sr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 313 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"frag\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 314 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"secondary\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 315 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"cs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 2 as libc::c_int,
            val: 316 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"end-bonus\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 317 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"no-pairing\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 318 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"splice-flank\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 319 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"idx-no-seq\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 320 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"end-seed-pen\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 321 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"for-only\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 322 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"rev-only\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 323 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"heap-sort\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 324 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"all-chain\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 'P' as i32,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"dual\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 326 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"max-clip-ratio\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 327 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"min-occ-floor\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 328 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"MD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 329 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"lj-min-ratio\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 330 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"score-N\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 331 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"eqx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 332 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"paf-no-hit\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 333 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"split-prefix\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 334 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"no-end-flt\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 335 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"hard-mask-level\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 336 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"cap-sw-mem\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 337 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"max-qlen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 338 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"max-chain-iter\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 339 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"junc-bed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 340 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"junc-bonus\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 341 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"sam-hit-only\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 342 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"chain-gap-scale\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 343 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"alt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 344 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"alt-drop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 345 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"mask-len\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 346 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"rmq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 2 as libc::c_int,
            val: 347 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"qstrand\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 348 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"cap-kalloc\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 349 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"q-occ-frac\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 350 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"chain-skip-scale\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 351 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"print-chains\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 352 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"no-hash-name\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 353 as libc::c_int,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"help\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"max-intron-len\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 'G' as i32,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"version\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 'V' as i32,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"min-count\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"min-chain-score\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 'm' as i32,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"mask-level\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 'M' as i32,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"min-dp-score\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: b"sam\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = ko_longopt_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
#[inline]
unsafe extern "C" fn mm_parse_num2(
    mut str: *const libc::c_char,
    mut q: *mut *mut libc::c_char,
) -> int64_t {
    let mut x: libc::c_double = 0.;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    x = strtod(str, &mut p);
    if *p as libc::c_int == 'G' as i32 || *p as libc::c_int == 'g' as i32 {
        x *= 1e9f64;
        p = p.offset(1);
        p;
    } else if *p as libc::c_int == 'M' as i32 || *p as libc::c_int == 'm' as i32 {
        x *= 1e6f64;
        p = p.offset(1);
        p;
    } else if *p as libc::c_int == 'K' as i32 || *p as libc::c_int == 'k' as i32 {
        x *= 1e3f64;
        p = p.offset(1);
        p;
    }
    if !q.is_null() {
        *q = p;
    }
    return (x + 0.499f64) as int64_t;
}
#[inline]
unsafe extern "C" fn mm_parse_num(mut str: *const libc::c_char) -> int64_t {
    return mm_parse_num2(str, 0 as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn yes_or_no(
    mut opt: *mut mm_mapopt_t,
    mut flag: int64_t,
    mut long_idx: libc::c_int,
    mut arg: *const libc::c_char,
    mut yes_to_set: libc::c_int,
) {
    if yes_to_set != 0 {
        if strcmp(arg, b"yes\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(arg, b"y\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            (*opt).flag |= flag;
        } else if strcmp(arg, b"no\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(arg, b"n\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            (*opt).flag &= !flag;
        } else {
            fprintf(
                stderr,
                b"[WARNING]\x1B[1;31m option '--%s' only accepts 'yes' or 'no'.\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
                long_options[long_idx as usize].name,
            );
        }
    } else if strcmp(arg, b"yes\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp(arg, b"y\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        (*opt).flag &= !flag;
    } else if strcmp(arg, b"no\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp(arg, b"n\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        (*opt).flag |= flag;
    } else {
        fprintf(
            stderr,
            b"[WARNING]\x1B[1;31m option '--%s' only accepts 'yes' or 'no'.\x1B[0m\n\0"
                as *const u8 as *const libc::c_char,
            long_options[long_idx as usize].name,
        );
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut opt_str: *const libc::c_char = b"2aSDw:k:K:t:r:f:Vv:g:G:I:d:XT:s:x:Hcp:M:n:z:A:B:O:E:m:N:Qu:R:hF:LC:yYPo:e:U:\0"
        as *const u8 as *const libc::c_char;
    let mut o: ketopt_t = KETOPT_INIT;
    let mut opt: mm_mapopt_t = mm_mapopt_t {
        flag: 0,
        seed: 0,
        sdust_thres: 0,
        max_qlen: 0,
        bw: 0,
        bw_long: 0,
        max_gap: 0,
        max_gap_ref: 0,
        max_frag_len: 0,
        max_chain_skip: 0,
        max_chain_iter: 0,
        min_cnt: 0,
        min_chain_score: 0,
        chain_gap_scale: 0.,
        chain_skip_scale: 0.,
        rmq_size_cap: 0,
        rmq_inner_dist: 0,
        rmq_rescue_size: 0,
        rmq_rescue_ratio: 0.,
        mask_level: 0.,
        mask_len: 0,
        pri_ratio: 0.,
        best_n: 0,
        alt_drop: 0.,
        a: 0,
        b: 0,
        q: 0,
        e: 0,
        q2: 0,
        e2: 0,
        sc_ambi: 0,
        noncan: 0,
        junc_bonus: 0,
        zdrop: 0,
        zdrop_inv: 0,
        end_bonus: 0,
        min_dp_max: 0,
        min_ksw_len: 0,
        anchor_ext_len: 0,
        anchor_ext_shift: 0,
        max_clip_ratio: 0.,
        rank_min_len: 0,
        rank_frac: 0.,
        pe_ori: 0,
        pe_bonus: 0,
        mid_occ_frac: 0.,
        q_occ_frac: 0.,
        min_mid_occ: 0,
        max_mid_occ: 0,
        mid_occ: 0,
        max_occ: 0,
        max_max_occ: 0,
        occ_dist: 0,
        mini_batch_size: 0,
        max_sw_mat: 0,
        cap_kalloc: 0,
        split_prefix: 0 as *const libc::c_char,
    };
    let mut ipt: mm_idxopt_t = mm_idxopt_t {
        k: 0,
        w: 0,
        flag: 0,
        bucket_bits: 0,
        mini_batch_size: 0,
        batch_size: 0,
    };
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut n_threads: libc::c_int = 3 as libc::c_int;
    let mut n_parts: libc::c_int = 0;
    let mut old_best_n: libc::c_int = -(1 as libc::c_int);
    let mut fnw: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut junc_bed: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut alt_list: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fp_help: *mut FILE = stderr;
    let mut idx_rdr: *mut mm_idx_reader_t = 0 as *mut mm_idx_reader_t;
    let mut mi: *mut mm_idx_t = 0 as *mut mm_idx_t;
    mm_verbose = 3 as libc::c_int;
    liftrlimit();
    mm_realtime0 = realtime();
    mm_set_opt(0 as *const libc::c_char, &mut ipt, &mut opt);
    loop {
        c = ketopt(
            &mut o,
            argc,
            argv,
            1 as libc::c_int,
            opt_str,
            long_options.as_mut_ptr(),
        );
        if !(c >= 0 as libc::c_int) {
            break;
        }
        if c == 'x' as i32 {
            if mm_set_opt(o.arg, &mut ipt, &mut opt) < 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"[ERROR] unknown preset '%s'\n\0" as *const u8
                        as *const libc::c_char,
                    o.arg,
                );
                return 1 as libc::c_int;
            }
        } else if c == ':' as i32 {
            fprintf(
                stderr,
                b"[ERROR] missing option argument\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 1 as libc::c_int;
        } else if c == '?' as i32 {
            fprintf(
                stderr,
                b"[ERROR] unknown option in \"%s\"\n\0" as *const u8
                    as *const libc::c_char,
                *argv.offset((o.i - 1 as libc::c_int) as isize),
            );
            return 1 as libc::c_int;
        }
    }
    o = KETOPT_INIT;
    loop {
        c = ketopt(
            &mut o,
            argc,
            argv,
            1 as libc::c_int,
            opt_str,
            long_options.as_mut_ptr(),
        );
        if !(c >= 0 as libc::c_int) {
            break;
        }
        if c == 'w' as i32 {
            ipt.w = atoi(o.arg) as libc::c_short;
        } else if c == 'k' as i32 {
            ipt.k = atoi(o.arg) as libc::c_short;
        } else if c == 'H' as i32 {
            ipt.flag = (ipt.flag as libc::c_int | 0x1 as libc::c_int) as libc::c_short;
        } else if c == 'd' as i32 {
            fnw = o.arg;
        } else if c == 't' as i32 {
            n_threads = atoi(o.arg);
        } else if c == 'v' as i32 {
            mm_verbose = atoi(o.arg);
        } else if c == 'g' as i32 {
            opt.max_gap = mm_parse_num(o.arg) as libc::c_int;
        } else if c == 'G' as i32 {
            mm_mapopt_max_intron_len(&mut opt, mm_parse_num(o.arg) as libc::c_int);
        } else if c == 'F' as i32 {
            opt.max_frag_len = mm_parse_num(o.arg) as libc::c_int;
        } else if c == 'N' as i32 {
            old_best_n = opt.best_n;
            opt.best_n = atoi(o.arg);
        } else if c == 'p' as i32 {
            opt.pri_ratio = atof(o.arg) as libc::c_float;
        } else if c == 'M' as i32 {
            opt.mask_level = atof(o.arg) as libc::c_float;
        } else if c == 'c' as i32 {
            opt.flag |= (0x20 as libc::c_int | 0x4 as libc::c_int) as libc::c_long;
        } else if c == 'D' as i32 {
            opt.flag |= 0x1 as libc::c_int as libc::c_long;
        } else if c == 'P' as i32 {
            opt.flag |= 0x800000 as libc::c_int as libc::c_long;
        } else if c == 'X' as i32 {
            opt.flag
                |= (0x800000 as libc::c_int | 0x1 as libc::c_int | 0x2 as libc::c_int
                    | 0x400 as libc::c_int) as libc::c_long;
        } else if c == 'a' as i32 {
            opt.flag |= (0x8 as libc::c_int | 0x4 as libc::c_int) as libc::c_long;
        } else if c == 'Q' as i32 {
            opt.flag |= 0x10 as libc::c_int as libc::c_long;
        } else if c == 'Y' as i32 {
            opt.flag |= 0x80000 as libc::c_int as libc::c_long;
        } else if c == 'L' as i32 {
            opt.flag |= 0x10000 as libc::c_int as libc::c_long;
        } else if c == 'y' as i32 {
            opt.flag |= 0x2000000 as libc::c_int as libc::c_long;
        } else if c == 'T' as i32 {
            opt.sdust_thres = atoi(o.arg);
        } else if c == 'n' as i32 {
            opt.min_cnt = atoi(o.arg);
        } else if c == 'm' as i32 {
            opt.min_chain_score = atoi(o.arg);
        } else if c == 'A' as i32 {
            opt.a = atoi(o.arg);
        } else if c == 'B' as i32 {
            opt.b = atoi(o.arg);
        } else if c == 's' as i32 {
            opt.min_dp_max = atoi(o.arg);
        } else if c == 'C' as i32 {
            opt.noncan = atoi(o.arg);
        } else if c == 'I' as i32 {
            ipt.batch_size = mm_parse_num(o.arg) as uint64_t;
        } else if c == 'K' as i32 {
            opt.mini_batch_size = mm_parse_num(o.arg);
        } else if c == 'e' as i32 {
            opt.occ_dist = mm_parse_num(o.arg) as int32_t;
        } else if c == 'R' as i32 {
            rg = o.arg;
        } else if c == 'h' as i32 {
            fp_help = stdout;
        } else if c == '2' as i32 {
            opt.flag |= 0x8000 as libc::c_int as libc::c_long;
        } else if c == 'o' as i32 {
            if strcmp(o.arg, b"-\0" as *const u8 as *const libc::c_char)
                != 0 as libc::c_int
            {
                if (freopen(o.arg, b"wb\0" as *const u8 as *const libc::c_char, stdout))
                    .is_null()
                {
                    fprintf(
                        stderr,
                        b"[ERROR]\x1B[1;31m failed to write the output to file '%s'\x1B[0m: %s\n\0"
                            as *const u8 as *const libc::c_char,
                        o.arg,
                        strerror(*__errno_location()),
                    );
                    exit(1 as libc::c_int);
                }
            }
        } else if c == 300 as libc::c_int {
            ipt.bucket_bits = atoi(o.arg) as libc::c_short;
        } else if c == 302 as libc::c_int {
            opt.seed = atoi(o.arg);
        } else if c == 303 as libc::c_int {
            mm_dbg_flag |= 0x1 as libc::c_int;
        } else if c == 304 as libc::c_int {
            mm_dbg_flag |= 0x2 as libc::c_int;
        } else if c == 306 as libc::c_int {
            mm_dbg_flag |= 0x2 as libc::c_int | 0x4 as libc::c_int;
            n_threads = 1 as libc::c_int;
        } else if c == 307 as libc::c_int {
            opt.max_chain_skip = atoi(o.arg);
        } else if c == 339 as libc::c_int {
            opt.max_chain_iter = atoi(o.arg);
        } else if c == 308 as libc::c_int {
            opt.min_ksw_len = atoi(o.arg);
        } else if c == 309 as libc::c_int {
            mm_dbg_flag |= 0x2 as libc::c_int | 0x8 as libc::c_int;
            n_threads = 1 as libc::c_int;
        } else if c == 310 as libc::c_int {
            opt.flag |= 0x80 as libc::c_int as libc::c_long;
        } else if c == 312 as libc::c_int {
            opt.flag |= 0x400 as libc::c_int as libc::c_long;
        } else if c == 313 as libc::c_int {
            opt.flag |= 0x1000 as libc::c_int as libc::c_long;
        } else if c == 317 as libc::c_int {
            opt.end_bonus = atoi(o.arg);
        } else if c == 318 as libc::c_int {
            opt.flag |= 0x20000 as libc::c_int as libc::c_long;
        } else if c == 320 as libc::c_int {
            ipt.flag = (ipt.flag as libc::c_int | 0x2 as libc::c_int) as libc::c_short;
        } else if c == 321 as libc::c_int {
            opt.anchor_ext_shift = atoi(o.arg);
        } else if c == 322 as libc::c_int {
            opt.flag |= 0x100000 as libc::c_int as libc::c_long;
        } else if c == 323 as libc::c_int {
            opt.flag |= 0x200000 as libc::c_int as libc::c_long;
        } else if c == 327 as libc::c_int {
            opt.max_clip_ratio = atof(o.arg) as libc::c_float;
        } else if c == 328 as libc::c_int {
            opt.min_mid_occ = atoi(o.arg);
        } else if c == 329 as libc::c_int {
            opt.flag |= 0x1000000 as libc::c_int as libc::c_long;
        } else if c == 331 as libc::c_int {
            opt.sc_ambi = atoi(o.arg);
        } else if c == 332 as libc::c_int {
            opt.flag |= 0x4000000 as libc::c_int as libc::c_long;
        } else if c == 333 as libc::c_int {
            opt.flag |= 0x8000000 as libc::c_int as libc::c_long;
        } else if c == 334 as libc::c_int {
            opt.split_prefix = o.arg;
        } else if c == 335 as libc::c_int {
            opt.flag |= 0x10000000 as libc::c_int as libc::c_long;
        } else if c == 336 as libc::c_int {
            opt.flag |= 0x20000000 as libc::c_int as libc::c_long;
        } else if c == 337 as libc::c_int {
            opt.max_sw_mat = mm_parse_num(o.arg);
        } else if c == 338 as libc::c_int {
            opt.max_qlen = mm_parse_num(o.arg) as libc::c_int;
        } else if c == 340 as libc::c_int {
            junc_bed = o.arg;
        } else if c == 341 as libc::c_int {
            opt.junc_bonus = atoi(o.arg);
        } else if c == 342 as libc::c_int {
            opt.flag |= 0x40000000 as libc::c_int as libc::c_long;
        } else if c == 343 as libc::c_int {
            opt.chain_gap_scale = atof(o.arg) as libc::c_float;
        } else if c == 351 as libc::c_int {
            opt.chain_skip_scale = atof(o.arg) as libc::c_float;
        } else if c == 344 as libc::c_int {
            alt_list = o.arg;
        } else if c == 345 as libc::c_int {
            opt.alt_drop = atof(o.arg) as libc::c_float;
        } else if c == 346 as libc::c_int {
            opt.mask_len = mm_parse_num(o.arg) as libc::c_int;
        } else if c == 348 as libc::c_int {
            opt
                .flag = (opt.flag as libc::c_longlong
                | (0x100000000 as libc::c_longlong | 0x200000000 as libc::c_longlong))
                as int64_t;
        } else if c == 349 as libc::c_int {
            opt.cap_kalloc = mm_parse_num(o.arg);
        } else if c == 350 as libc::c_int {
            opt.q_occ_frac = atof(o.arg) as libc::c_float;
        } else if c == 352 as libc::c_int {
            mm_dbg_flag |= 0x10 as libc::c_int;
        } else if c == 353 as libc::c_int {
            opt
                .flag = (opt.flag as libc::c_longlong | 0x400000000 as libc::c_longlong)
                as int64_t;
        } else if c == 330 as libc::c_int {
            fprintf(
                stderr,
                b"[WARNING] \x1B[1;31m --lj-min-ratio has been deprecated.\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
            );
        } else if c == 314 as libc::c_int {
            yes_or_no(
                &mut opt,
                0x2000 as libc::c_int as int64_t,
                o.longidx,
                o.arg,
                1 as libc::c_int,
            );
        } else if c == 315 as libc::c_int {
            yes_or_no(
                &mut opt,
                0x4000 as libc::c_int as int64_t,
                o.longidx,
                o.arg,
                0 as libc::c_int,
            );
        } else if c == 316 as libc::c_int {
            opt.flag |= (0x40 as libc::c_int | 0x4 as libc::c_int) as libc::c_long;
            if (o.arg).is_null()
                || strcmp(o.arg, b"short\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                opt.flag &= !(0x800 as libc::c_int) as libc::c_long;
            } else if strcmp(o.arg, b"long\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                opt.flag |= 0x800 as libc::c_int as libc::c_long;
            } else if strcmp(o.arg, b"none\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                opt.flag &= !(0x40 as libc::c_int) as libc::c_long;
            } else if mm_verbose >= 2 as libc::c_int {
                fprintf(
                    stderr,
                    b"[WARNING]\x1B[1;31m --cs only takes 'short' or 'long'. Invalid values are assumed to be 'short'.\x1B[0m\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        } else if c == 319 as libc::c_int {
            yes_or_no(
                &mut opt,
                0x40000 as libc::c_int as int64_t,
                o.longidx,
                o.arg,
                1 as libc::c_int,
            );
        } else if c == 324 as libc::c_int {
            yes_or_no(
                &mut opt,
                0x400000 as libc::c_int as int64_t,
                o.longidx,
                o.arg,
                1 as libc::c_int,
            );
        } else if c == 326 as libc::c_int {
            yes_or_no(
                &mut opt,
                0x2 as libc::c_int as int64_t,
                o.longidx,
                o.arg,
                0 as libc::c_int,
            );
        } else if c == 347 as libc::c_int {
            yes_or_no(
                &mut opt,
                0x80000000 as libc::c_longlong as int64_t,
                o.longidx,
                o.arg,
                1 as libc::c_int,
            );
        } else if c == 'S' as i32 {
            opt.flag
                |= (0x40 as libc::c_int | 0x4 as libc::c_int | 0x800 as libc::c_int)
                    as libc::c_long;
            if mm_verbose >= 2 as libc::c_int {
                fprintf(
                    stderr,
                    b"[WARNING]\x1B[1;31m option -S is deprecated and may be removed in future. Please use --cs=long instead.\x1B[0m\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        } else if c == 'V' as i32 {
            puts(b"2.24-r1122\0" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int;
        } else if c == 'r' as i32 {
            opt.bw = mm_parse_num2(o.arg, &mut s) as libc::c_int;
            if *s as libc::c_int == ',' as i32 {
                opt
                    .bw_long = mm_parse_num2(s.offset(1 as libc::c_int as isize), &mut s)
                    as libc::c_int;
            }
        } else if c == 'U' as i32 {
            opt.min_mid_occ = strtol(o.arg, &mut s, 10 as libc::c_int) as int32_t;
            if *s as libc::c_int == ',' as i32 {
                opt
                    .max_mid_occ = strtol(
                    s.offset(1 as libc::c_int as isize),
                    &mut s,
                    10 as libc::c_int,
                ) as int32_t;
            }
        } else if c == 'f' as i32 {
            let mut x: libc::c_double = 0.;
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            x = strtod(o.arg, &mut p);
            if x < 1.0f64 {
                opt.mid_occ_frac = x as libc::c_float;
                opt.mid_occ = 0 as libc::c_int;
            } else {
                opt.mid_occ = (x + 0.499f64) as libc::c_int;
            }
            if *p as libc::c_int == ',' as i32 {
                opt
                    .max_occ = (strtod(p.offset(1 as libc::c_int as isize), &mut p)
                    + 0.499f64) as libc::c_int;
            }
        } else if c == 'u' as i32 {
            if *o.arg as libc::c_int == 'b' as i32 {
                opt.flag
                    |= (0x100 as libc::c_int | 0x200 as libc::c_int) as libc::c_long;
            } else if *o.arg as libc::c_int == 'f' as i32 {
                opt.flag |= 0x100 as libc::c_int as libc::c_long;
                opt.flag &= !(0x200 as libc::c_int) as libc::c_long;
            } else if *o.arg as libc::c_int == 'r' as i32 {
                opt.flag |= 0x200 as libc::c_int as libc::c_long;
                opt.flag &= !(0x100 as libc::c_int) as libc::c_long;
            } else if *o.arg as libc::c_int == 'n' as i32 {
                opt.flag
                    &= !(0x100 as libc::c_int | 0x200 as libc::c_int) as libc::c_long;
            } else {
                fprintf(
                    stderr,
                    b"[ERROR]\x1B[1;31m unrecognized cDNA direction\x1B[0m\n\0"
                        as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
        } else if c == 'z' as i32 {
            opt.zdrop_inv = strtol(o.arg, &mut s, 10 as libc::c_int) as libc::c_int;
            opt.zdrop = opt.zdrop_inv;
            if *s as libc::c_int == ',' as i32 {
                opt
                    .zdrop_inv = strtol(
                    s.offset(1 as libc::c_int as isize),
                    &mut s,
                    10 as libc::c_int,
                ) as libc::c_int;
            }
        } else if c == 'O' as i32 {
            opt.q2 = strtol(o.arg, &mut s, 10 as libc::c_int) as libc::c_int;
            opt.q = opt.q2;
            if *s as libc::c_int == ',' as i32 {
                opt
                    .q2 = strtol(
                    s.offset(1 as libc::c_int as isize),
                    &mut s,
                    10 as libc::c_int,
                ) as libc::c_int;
            }
        } else if c == 'E' as i32 {
            opt.e2 = strtol(o.arg, &mut s, 10 as libc::c_int) as libc::c_int;
            opt.e = opt.e2;
            if *s as libc::c_int == ',' as i32 {
                opt
                    .e2 = strtol(
                    s.offset(1 as libc::c_int as isize),
                    &mut s,
                    10 as libc::c_int,
                ) as libc::c_int;
            }
        }
    }
    if opt.flag & 0x80 as libc::c_int as libc::c_long != 0
        && opt.flag & 0x2000 as libc::c_int as libc::c_long != 0
    {
        fprintf(
            stderr,
            b"[ERROR]\x1B[1;31m --splice and --frag should not be specified at the same time.\x1B[0m\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if fnw.is_null() && opt.flag & 0x4 as libc::c_int as libc::c_long == 0 {
        ipt.flag = (ipt.flag as libc::c_int | 0x2 as libc::c_int) as libc::c_short;
    }
    if mm_check_opt(&mut ipt, &mut opt) < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if opt.best_n == 0 as libc::c_int {
        fprintf(
            stderr,
            b"[WARNING]\x1B[1;31m changed '-N 0' to '-N %d --secondary=no'.\x1B[0m\n\0"
                as *const u8 as *const libc::c_char,
            old_best_n,
        );
        opt.best_n = old_best_n;
        opt.flag |= 0x4000 as libc::c_int as libc::c_long;
    }
    if argc == o.ind || fp_help == stdout {
        fprintf(
            fp_help,
            b"Usage: minimap2 [options] <target.fa>|<target.idx> [query.fa] [...]\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(fp_help, b"Options:\n\0" as *const u8 as *const libc::c_char);
        fprintf(fp_help, b"  Indexing:\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            fp_help,
            b"    -H           use homopolymer-compressed k-mer (preferrable for PacBio)\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    -k INT       k-mer size (no larger than 28) [%d]\n\0" as *const u8
                as *const libc::c_char,
            ipt.k as libc::c_int,
        );
        fprintf(
            fp_help,
            b"    -w INT       minimizer window size [%d]\n\0" as *const u8
                as *const libc::c_char,
            ipt.w as libc::c_int,
        );
        fprintf(
            fp_help,
            b"    -I NUM       split index for every ~NUM input bases [4G]\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    -d FILE      dump index to FILE []\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(fp_help, b"  Mapping:\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            fp_help,
            b"    -f FLOAT     filter out top FLOAT fraction of repetitive minimizers [%g]\n\0"
                as *const u8 as *const libc::c_char,
            opt.mid_occ_frac as libc::c_double,
        );
        fprintf(
            fp_help,
            b"    -g NUM       stop chain enlongation if there are no minimizers in INT-bp [%d]\n\0"
                as *const u8 as *const libc::c_char,
            opt.max_gap,
        );
        fprintf(
            fp_help,
            b"    -G NUM       max intron length (effective with -xsplice; changing -r) [200k]\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    -F NUM       max fragment length (effective with -xsr or in the fragment mode) [800]\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    -r NUM[,NUM] chaining/alignment bandwidth and long-join bandwidth [%d,%d]\n\0"
                as *const u8 as *const libc::c_char,
            opt.bw,
            opt.bw_long,
        );
        fprintf(
            fp_help,
            b"    -n INT       minimal number of minimizers on a chain [%d]\n\0"
                as *const u8 as *const libc::c_char,
            opt.min_cnt,
        );
        fprintf(
            fp_help,
            b"    -m INT       minimal chaining score (matching bases minus log gap penalty) [%d]\n\0"
                as *const u8 as *const libc::c_char,
            opt.min_chain_score,
        );
        fprintf(
            fp_help,
            b"    -X           skip self and dual mappings (for the all-vs-all mode)\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    -p FLOAT     min secondary-to-primary score ratio [%g]\n\0"
                as *const u8 as *const libc::c_char,
            opt.pri_ratio as libc::c_double,
        );
        fprintf(
            fp_help,
            b"    -N INT       retain at most INT secondary alignments [%d]\n\0"
                as *const u8 as *const libc::c_char,
            opt.best_n,
        );
        fprintf(fp_help, b"  Alignment:\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            fp_help,
            b"    -A INT       matching score [%d]\n\0" as *const u8
                as *const libc::c_char,
            opt.a,
        );
        fprintf(
            fp_help,
            b"    -B INT       mismatch penalty (larger value for lower divergence) [%d]\n\0"
                as *const u8 as *const libc::c_char,
            opt.b,
        );
        fprintf(
            fp_help,
            b"    -O INT[,INT] gap open penalty [%d,%d]\n\0" as *const u8
                as *const libc::c_char,
            opt.q,
            opt.q2,
        );
        fprintf(
            fp_help,
            b"    -E INT[,INT] gap extension penalty; a k-long gap costs min{O1+k*E1,O2+k*E2} [%d,%d]\n\0"
                as *const u8 as *const libc::c_char,
            opt.e,
            opt.e2,
        );
        fprintf(
            fp_help,
            b"    -z INT[,INT] Z-drop score and inversion Z-drop score [%d,%d]\n\0"
                as *const u8 as *const libc::c_char,
            opt.zdrop,
            opt.zdrop_inv,
        );
        fprintf(
            fp_help,
            b"    -s INT       minimal peak DP alignment score [%d]\n\0" as *const u8
                as *const libc::c_char,
            opt.min_dp_max,
        );
        fprintf(
            fp_help,
            b"    -u CHAR      how to find GT-AG. f:transcript strand, b:both strands, n:don't match GT-AG [n]\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(fp_help, b"  Input/Output:\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            fp_help,
            b"    -a           output in the SAM format (PAF by default)\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    -o FILE      output alignments to FILE [stdout]\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    -L           write CIGAR with >65535 ops at the CG tag\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    -R STR       SAM read group line in a format like '@RG\\tID:foo\\tSM:bar' []\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    -c           output CIGAR in PAF\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    --cs[=STR]   output the cs tag; STR is 'short' (if absent) or 'long' [none]\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    --MD         output the MD tag\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    --eqx        write =/X CIGAR operators\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    -Y           use soft clipping for supplementary alignments\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    -t INT       number of threads [%d]\n\0" as *const u8
                as *const libc::c_char,
            n_threads,
        );
        fprintf(
            fp_help,
            b"    -K NUM       minibatch size for mapping [500M]\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    --version    show version number\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(fp_help, b"  Preset:\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            fp_help,
            b"    -x STR       preset (always applied before other options; see minimap2.1 for details) []\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"                 - map-pb/map-ont - PacBio CLR/Nanopore vs reference mapping\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"                 - map-hifi - PacBio HiFi reads vs reference mapping\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"                 - ava-pb/ava-ont - PacBio/Nanopore read overlap\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"                 - asm5/asm10/asm20 - asm-to-ref mapping, for ~0.1/1/5%% sequence divergence\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"                 - splice/splice:hq - long-read/Pacbio-CCS spliced alignment\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"                 - sr - genomic short-read mapping\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"\nSee `man ./minimap2.1' for detailed description of these and other advanced command-line options.\n\0"
                as *const u8 as *const libc::c_char,
        );
        return if fp_help == stdout { 0 as libc::c_int } else { 1 as libc::c_int };
    }
    if opt.flag & 0x1000 as libc::c_int as libc::c_long != 0
        && argc - o.ind > 3 as libc::c_int
    {
        fprintf(
            stderr,
            b"[ERROR] incorrect input: in the sr mode, please specify no more than two query files.\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    idx_rdr = mm_idx_reader_open(*argv.offset(o.ind as isize), &mut ipt, fnw);
    if idx_rdr.is_null() {
        fprintf(
            stderr,
            b"[ERROR] failed to open file '%s': %s\n\0" as *const u8
                as *const libc::c_char,
            *argv.offset(o.ind as isize),
            strerror(*__errno_location()),
        );
        return 1 as libc::c_int;
    }
    if (*idx_rdr).is_idx == 0 && fnw.is_null() && argc - o.ind < 2 as libc::c_int {
        fprintf(
            stderr,
            b"[ERROR] missing input: please specify a query file to map or option -d to keep the index\n\0"
                as *const u8 as *const libc::c_char,
        );
        mm_idx_reader_close(idx_rdr);
        return 1 as libc::c_int;
    }
    if opt.best_n == 0 as libc::c_int
        && opt.flag & 0x4 as libc::c_int as libc::c_long != 0
        && mm_verbose >= 2 as libc::c_int
    {
        fprintf(
            stderr,
            b"[WARNING]\x1B[1;31m `-N 0' reduces alignment accuracy. Please use --secondary=no to suppress secondary alignments.\x1B[0m\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    loop {
        mi = mm_idx_reader_read(idx_rdr, n_threads);
        if mi.is_null() {
            break;
        }
        let mut ret: libc::c_int = 0;
        if opt.flag & 0x4 as libc::c_int as libc::c_long != 0
            && (*mi).flag & 0x2 as libc::c_int != 0
        {
            fprintf(
                stderr,
                b"[ERROR] the prebuilt index doesn't contain sequences.\n\0" as *const u8
                    as *const libc::c_char,
            );
            mm_idx_destroy(mi);
            mm_idx_reader_close(idx_rdr);
            return 1 as libc::c_int;
        }
        if opt.flag & 0x8 as libc::c_int as libc::c_long != 0
            && (*idx_rdr).n_parts == 1 as libc::c_int
        {
            if mm_idx_reader_eof(idx_rdr) != 0 {
                if (opt.split_prefix).is_null() {
                    ret = mm_write_sam_hdr(
                        mi,
                        rg,
                        b"2.24-r1122\0" as *const u8 as *const libc::c_char,
                        argc,
                        argv,
                    );
                } else {
                    ret = mm_write_sam_hdr(
                        0 as *const mm_idx_t,
                        rg,
                        b"2.24-r1122\0" as *const u8 as *const libc::c_char,
                        argc,
                        argv,
                    );
                }
            } else {
                ret = mm_write_sam_hdr(
                    0 as *const mm_idx_t,
                    rg,
                    b"2.24-r1122\0" as *const u8 as *const libc::c_char,
                    argc,
                    argv,
                );
                if (opt.split_prefix).is_null() && mm_verbose >= 2 as libc::c_int {
                    fprintf(
                        stderr,
                        b"[WARNING]\x1B[1;31m For a multi-part index, no @SQ lines will be outputted. Please use --split-prefix.\x1B[0m\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            }
            if ret != 0 as libc::c_int {
                mm_idx_destroy(mi);
                mm_idx_reader_close(idx_rdr);
                return 1 as libc::c_int;
            }
        }
        if mm_verbose >= 3 as libc::c_int {
            fprintf(
                stderr,
                b"[M::%s::%.3f*%.2f] loaded/built the index for %d target sequence(s)\n\0"
                    as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"main\0"))
                    .as_ptr(),
                realtime() - mm_realtime0,
                cputime() / (realtime() - mm_realtime0),
                (*mi).n_seq,
            );
        }
        if argc != o.ind + 1 as libc::c_int {
            mm_mapopt_update(&mut opt, mi);
        }
        if mm_verbose >= 3 as libc::c_int {
            mm_idx_stat(mi);
        }
        if !junc_bed.is_null() {
            mm_idx_bed_read(mi, junc_bed, 1 as libc::c_int);
        }
        if !alt_list.is_null() {
            mm_idx_alt_read(mi, alt_list);
        }
        if argc - (o.ind + 1 as libc::c_int) == 0 as libc::c_int {
            mm_idx_destroy(mi);
        } else {
            ret = 0 as libc::c_int;
            if opt.flag & 0x2000 as libc::c_int as libc::c_long == 0 {
                i = o.ind + 1 as libc::c_int;
                while i < argc {
                    ret = mm_map_file(mi, *argv.offset(i as isize), &mut opt, n_threads);
                    if ret < 0 as libc::c_int {
                        break;
                    }
                    i += 1;
                    i;
                }
            } else {
                ret = mm_map_file_frag(
                    mi,
                    argc - (o.ind + 1 as libc::c_int),
                    &mut *argv.offset((o.ind + 1 as libc::c_int) as isize)
                        as *mut *mut libc::c_char as *mut *const libc::c_char,
                    &mut opt,
                    n_threads,
                );
            }
            mm_idx_destroy(mi);
            if ret < 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"ERROR: failed to map the query file\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        }
    }
    n_parts = (*idx_rdr).n_parts;
    mm_idx_reader_close(idx_rdr);
    if !(opt.split_prefix).is_null() {
        mm_split_merge(
            argc - (o.ind + 1 as libc::c_int),
            &mut *argv.offset((o.ind + 1 as libc::c_int) as isize)
                as *mut *mut libc::c_char as *mut *const libc::c_char,
            &mut opt,
            n_parts,
        );
    }
    if fflush(stdout) == -(1 as libc::c_int) {
        perror(
            b"[ERROR] failed to write the results\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if mm_verbose >= 3 as libc::c_int {
        fprintf(
            stderr,
            b"[M::%s] Version: %s\n\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"main\0")).as_ptr(),
            b"2.24-r1122\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"[M::%s] CMD:\0" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"main\0")).as_ptr(),
        );
        i = 0 as libc::c_int;
        while i < argc {
            fprintf(
                stderr,
                b" %s\0" as *const u8 as *const libc::c_char,
                *argv.offset(i as isize),
            );
            i += 1;
            i;
        }
        fprintf(
            stderr,
            b"\n[M::%s] Real time: %.3f sec; CPU: %.3f sec; Peak RSS: %.3f GB\n\0"
                as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"main\0")).as_ptr(),
            realtime() - mm_realtime0,
            cputime(),
            peakrss() as libc::c_double / 1024.0f64 / 1024.0f64 / 1024.0f64,
        );
    }
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
