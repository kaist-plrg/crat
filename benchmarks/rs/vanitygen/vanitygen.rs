use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type bignum_ctx;
    pub type ec_group_st;
    pub type ec_point_st;
    pub type ec_key_st;
    pub type _timing_info_s;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn SHA256(
        d: *const libc::c_uchar,
        n: size_t,
        md: *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
    fn RIPEMD160(
        d: *const libc::c_uchar,
        n: size_t,
        md: *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn BN_sub(r: *mut BIGNUM, a: *const BIGNUM, b: *const BIGNUM) -> libc::c_int;
    fn BN_set_word(a: *mut BIGNUM, w: libc::c_ulong) -> libc::c_int;
    fn BN_get_word(a: *const BIGNUM) -> libc::c_ulong;
    fn EC_KEY_generate_key(key: *mut EC_KEY) -> libc::c_int;
    fn EC_KEY_get0_public_key(key: *const EC_KEY) -> *const EC_POINT;
    fn EC_KEY_get0_private_key(key: *const EC_KEY) -> *const BIGNUM;
    fn EC_KEY_get0_group(key: *const EC_KEY) -> *const EC_GROUP;
    fn EC_KEY_free(key: *mut EC_KEY);
    fn EC_POINT_mul(
        group: *const EC_GROUP,
        r: *mut EC_POINT,
        n: *const BIGNUM,
        q: *const EC_POINT,
        m: *const BIGNUM,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn EC_POINTs_make_affine(
        group: *const EC_GROUP,
        num: size_t,
        points: *mut *mut EC_POINT,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn EC_POINT_make_affine(
        group: *const EC_GROUP,
        point: *mut EC_POINT,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn EC_POINT_add(
        group: *const EC_GROUP,
        r: *mut EC_POINT,
        a: *const EC_POINT,
        b: *const EC_POINT,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn EC_POINT_hex2point(
        _: *const EC_GROUP,
        _: *const libc::c_char,
        _: *mut EC_POINT,
        _: *mut BN_CTX,
    ) -> *mut EC_POINT;
    fn EC_POINT_point2oct(
        group: *const EC_GROUP,
        p: *const EC_POINT,
        form: point_conversion_form_t,
        buf: *mut libc::c_uchar,
        len: size_t,
        ctx: *mut BN_CTX,
    ) -> size_t;
    fn EC_POINT_copy(dst: *mut EC_POINT, src: *const EC_POINT) -> libc::c_int;
    fn EC_POINT_free(point: *mut EC_POINT);
    fn EC_POINT_new(group: *const EC_GROUP) -> *mut EC_POINT;
    fn EC_GROUP_get_order(
        group: *const EC_GROUP,
        order: *mut BIGNUM,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn EC_GROUP_get0_generator(group: *const EC_GROUP) -> *const EC_POINT;
    fn RAND_load_file(file: *const libc::c_char, max_bytes: libc::c_long) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn vg_exec_context_upgrade_lock(vxcp: *mut vg_exec_context_t) -> libc::c_int;
    fn vg_context_add_patterns(
        vcp: *mut vg_context_t,
        patterns: *mut *const libc::c_char,
        npatterns: libc::c_int,
    ) -> libc::c_int;
    fn vg_prefix_context_new(
        addrtype: libc::c_int,
        privtype: libc::c_int,
        caseinsensitive: libc::c_int,
    ) -> *mut vg_context_t;
    fn vg_prefix_context_set_case_insensitive(
        vcp: *mut vg_context_t,
        caseinsensitive: libc::c_int,
    );
    fn vg_regex_context_new(
        addrtype: libc::c_int,
        privtype: libc::c_int,
    ) -> *mut vg_context_t;
    fn vg_output_timing(
        vcp: *mut vg_context_t,
        cycle: libc::c_int,
        last: *mut timeval,
    ) -> libc::c_int;
    fn vg_output_match_console(
        vcp: *mut vg_context_t,
        pkey: *mut EC_KEY,
        pattern: *const libc::c_char,
    );
    fn vg_output_timing_console(
        vcp: *mut vg_context_t,
        count: libc::c_double,
        rate: libc::c_ulonglong,
        total: libc::c_ulonglong,
    );
    fn vg_context_thread_exit(vcp: *mut vg_context_t);
    fn vg_exec_context_init(
        vcp: *mut vg_context_t,
        vxcp: *mut vg_exec_context_t,
    ) -> libc::c_int;
    fn vg_exec_context_del(vxcp: *mut vg_exec_context_t);
    fn vg_exec_context_new_key() -> *mut EC_KEY;
    fn vg_exec_context_downgrade_lock(vxcp: *mut vg_exec_context_t);
    fn vg_exec_context_yield(vxcp: *mut vg_exec_context_t);
    fn vg_check_password_complexity(
        pass: *const libc::c_char,
        verbose: libc::c_int,
    ) -> libc::c_int;
    fn vg_read_password(buf: *mut libc::c_char, size: size_t) -> libc::c_int;
    fn vg_read_file(
        fp: *mut FILE,
        result: *mut *mut *mut libc::c_char,
        rescount: *mut libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bignum_st {
    pub d: *mut libc::c_ulong,
    pub top: libc::c_int,
    pub dmax: libc::c_int,
    pub neg: libc::c_int,
    pub flags: libc::c_int,
}
pub type BIGNUM = bignum_st;
pub type BN_CTX = bignum_ctx;
pub type point_conversion_form_t = libc::c_uint;
pub const POINT_CONVERSION_HYBRID: point_conversion_form_t = 6;
pub const POINT_CONVERSION_UNCOMPRESSED: point_conversion_form_t = 4;
pub const POINT_CONVERSION_COMPRESSED: point_conversion_form_t = 2;
pub type EC_GROUP = ec_group_st;
pub type EC_POINT = ec_point_st;
pub type EC_KEY = ec_key_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _vg_context_s {
    pub vc_addrtype: libc::c_int,
    pub vc_privtype: libc::c_int,
    pub vc_npatterns: libc::c_ulong,
    pub vc_npatterns_start: libc::c_ulong,
    pub vc_found: libc::c_ulonglong,
    pub vc_pattern_generation: libc::c_int,
    pub vc_chance: libc::c_double,
    pub vc_result_file: *const libc::c_char,
    pub vc_key_protect_pass: *const libc::c_char,
    pub vc_remove_on_match: libc::c_int,
    pub vc_only_one: libc::c_int,
    pub vc_verbose: libc::c_int,
    pub vc_format: vg_format,
    pub vc_pubkeytype: libc::c_int,
    pub vc_pubkey_base: *mut EC_POINT,
    pub vc_halt: libc::c_int,
    pub vc_threads: *mut vg_exec_context_t,
    pub vc_thread_excl: libc::c_int,
    pub vc_free: vg_free_func_t,
    pub vc_add_patterns: vg_add_pattern_func_t,
    pub vc_clear_all_patterns: vg_clear_all_patterns_func_t,
    pub vc_test: vg_test_func_t,
    pub vc_hash160_sort: vg_hash160_sort_func_t,
    pub vc_timing_total: libc::c_ulonglong,
    pub vc_timing_prevfound: libc::c_ulonglong,
    pub vc_timing_sincelast: libc::c_ulonglong,
    pub vc_timing_head: *mut _timing_info_s,
    pub vc_output_error: vg_output_error_func_t,
    pub vc_output_match: vg_output_match_func_t,
    pub vc_output_timing: vg_output_timing_func_t,
}
pub type vg_output_timing_func_t = Option::<
    unsafe extern "C" fn(
        *mut vg_context_t,
        libc::c_double,
        libc::c_ulonglong,
        libc::c_ulonglong,
    ) -> (),
>;
pub type vg_context_t = _vg_context_s;
pub type vg_output_match_func_t = Option::<
    unsafe extern "C" fn(*mut vg_context_t, *mut EC_KEY, *const libc::c_char) -> (),
>;
pub type vg_output_error_func_t = Option::<
    unsafe extern "C" fn(*mut vg_context_t, *const libc::c_char) -> (),
>;
pub type vg_hash160_sort_func_t = Option::<
    unsafe extern "C" fn(*mut vg_context_t, *mut libc::c_void) -> libc::c_int,
>;
pub type vg_test_func_t = Option::<
    unsafe extern "C" fn(*mut vg_exec_context_t) -> libc::c_int,
>;
pub type vg_exec_context_t = _vg_exec_context_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _vg_exec_context_s {
    pub vxc_vc: *mut vg_context_t,
    pub vxc_bnctx: *mut BN_CTX,
    pub vxc_key: *mut EC_KEY,
    pub vxc_delta: libc::c_int,
    pub vxc_binres: [libc::c_uchar; 28],
    pub vxc_bntarg: BIGNUM,
    pub vxc_bnbase: BIGNUM,
    pub vxc_bntmp: BIGNUM,
    pub vxc_bntmp2: BIGNUM,
    pub vxc_threadfunc: vg_exec_context_threadfunc_t,
    pub vxc_pthread: pthread_t,
    pub vxc_thread_active: libc::c_int,
    pub vxc_next: *mut _vg_exec_context_s,
    pub vxc_lockmode: libc::c_int,
    pub vxc_stop: libc::c_int,
}
pub type vg_exec_context_threadfunc_t = Option::<
    unsafe extern "C" fn(*mut vg_exec_context_t) -> *mut libc::c_void,
>;
pub type vg_clear_all_patterns_func_t = Option::<
    unsafe extern "C" fn(*mut vg_context_t) -> (),
>;
pub type vg_add_pattern_func_t = Option::<
    unsafe extern "C" fn(
        *mut vg_context_t,
        *mut *const libc::c_char,
        libc::c_int,
    ) -> libc::c_int,
>;
pub type vg_free_func_t = Option::<unsafe extern "C" fn(*mut vg_context_t) -> ()>;
pub type vg_format = libc::c_uint;
pub const VCF_SCRIPT: vg_format = 1;
pub const VCF_PUBKEY: vg_format = 0;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
pub static mut version: *const libc::c_char = b"0.22\0" as *const u8
    as *const libc::c_char;
pub unsafe extern "C" fn vg_thread_loop(
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut hash_buf: [libc::c_uchar; 128] = [0; 128];
    let mut eckey_buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut hash1: [libc::c_uchar; 32] = [0; 32];
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut output_interval: libc::c_int = 0;
    let mut hash_len: libc::c_int = 0;
    let rekey_max: libc::c_ulong = 10000000 as libc::c_int as libc::c_ulong;
    let mut npoints: libc::c_ulong = 0;
    let mut rekey_at: libc::c_ulong = 0;
    let mut nbatch: libc::c_ulong = 0;
    let mut vcp: *mut vg_context_t = arg as *mut vg_context_t;
    let mut pkey: *mut EC_KEY = 0 as *mut EC_KEY;
    let mut pgroup: *const EC_GROUP = 0 as *const EC_GROUP;
    let mut pgen: *const EC_POINT = 0 as *const EC_POINT;
    let ptarraysize: libc::c_int = 256 as libc::c_int;
    let mut ppnt: [*mut EC_POINT; 256] = [0 as *mut EC_POINT; 256];
    let mut pbatchinc: *mut EC_POINT = 0 as *mut EC_POINT;
    let mut test_func: vg_test_func_t = (*vcp).vc_test;
    let mut ctx: vg_exec_context_t = vg_exec_context_t {
        vxc_vc: 0 as *mut vg_context_t,
        vxc_bnctx: 0 as *mut BN_CTX,
        vxc_key: 0 as *mut EC_KEY,
        vxc_delta: 0,
        vxc_binres: [0; 28],
        vxc_bntarg: BIGNUM {
            d: 0 as *mut libc::c_ulong,
            top: 0,
            dmax: 0,
            neg: 0,
            flags: 0,
        },
        vxc_bnbase: BIGNUM {
            d: 0 as *mut libc::c_ulong,
            top: 0,
            dmax: 0,
            neg: 0,
            flags: 0,
        },
        vxc_bntmp: BIGNUM {
            d: 0 as *mut libc::c_ulong,
            top: 0,
            dmax: 0,
            neg: 0,
            flags: 0,
        },
        vxc_bntmp2: BIGNUM {
            d: 0 as *mut libc::c_ulong,
            top: 0,
            dmax: 0,
            neg: 0,
            flags: 0,
        },
        vxc_threadfunc: None,
        vxc_pthread: 0,
        vxc_thread_active: 0,
        vxc_next: 0 as *mut _vg_exec_context_s,
        vxc_lockmode: 0,
        vxc_stop: 0,
    };
    let mut vxcp: *mut vg_exec_context_t = 0 as *mut vg_exec_context_t;
    let mut tvstart: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    memset(
        &mut ctx as *mut vg_exec_context_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<vg_exec_context_t>() as libc::c_ulong,
    );
    vxcp = &mut ctx;
    vg_exec_context_init(vcp, &mut ctx);
    pkey = (*vxcp).vxc_key;
    pgroup = EC_KEY_get0_group(pkey);
    pgen = EC_GROUP_get0_generator(pgroup);
    i = 0 as libc::c_int;
    while i < ptarraysize {
        ppnt[i as usize] = EC_POINT_new(pgroup);
        if (ppnt[i as usize]).is_null() {
            fprintf(
                stderr,
                b"ERROR: out of memory?\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    pbatchinc = EC_POINT_new(pgroup);
    if pbatchinc.is_null() {
        fprintf(
            stderr,
            b"ERROR: out of memory?\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    BN_set_word(&mut (*vxcp).vxc_bntmp, ptarraysize as libc::c_ulong);
    EC_POINT_mul(
        pgroup,
        pbatchinc,
        &mut (*vxcp).vxc_bntmp,
        0 as *const EC_POINT,
        0 as *const BIGNUM,
        (*vxcp).vxc_bnctx,
    );
    EC_POINT_make_affine(pgroup, pbatchinc, (*vxcp).vxc_bnctx);
    npoints = 0 as libc::c_int as libc::c_ulong;
    rekey_at = 0 as libc::c_int as libc::c_ulong;
    nbatch = 0 as libc::c_int as libc::c_ulong;
    (*vxcp).vxc_key = pkey;
    (*vxcp).vxc_binres[0 as libc::c_int as usize] = (*vcp).vc_addrtype as libc::c_uchar;
    c = 0 as libc::c_int;
    output_interval = 1000 as libc::c_int;
    gettimeofday(&mut tvstart, 0 as *mut libc::c_void);
    if (*vcp).vc_format as libc::c_uint == VCF_SCRIPT as libc::c_int as libc::c_uint {
        hash_buf[0 as libc::c_int as usize] = 0x51 as libc::c_int as libc::c_uchar;
        hash_buf[1 as libc::c_int as usize] = 0x41 as libc::c_int as libc::c_uchar;
        hash_buf[67 as libc::c_int as usize] = 0x51 as libc::c_int as libc::c_uchar;
        hash_buf[68 as libc::c_int as usize] = 0xae as libc::c_int as libc::c_uchar;
        eckey_buf = hash_buf.as_mut_ptr().offset(2 as libc::c_int as isize);
        hash_len = 69 as libc::c_int;
    } else {
        eckey_buf = hash_buf.as_mut_ptr();
        hash_len = 65 as libc::c_int;
    }
    's_173: while (*vcp).vc_halt == 0 {
        npoints = npoints.wrapping_add(1);
        if npoints >= rekey_at {
            vg_exec_context_upgrade_lock(vxcp);
            EC_KEY_generate_key(pkey);
            npoints = 0 as libc::c_int as libc::c_ulong;
            EC_GROUP_get_order(pgroup, &mut (*vxcp).vxc_bntmp, (*vxcp).vxc_bnctx);
            BN_sub(
                &mut (*vxcp).vxc_bntmp2,
                &mut (*vxcp).vxc_bntmp,
                EC_KEY_get0_private_key(pkey),
            );
            rekey_at = BN_get_word(&mut (*vxcp).vxc_bntmp2);
            if rekey_at == 0xffffffffffffffff as libc::c_ulong || rekey_at > rekey_max {
                rekey_at = rekey_max;
            }
            if rekey_at > 0 as libc::c_int as libc::c_ulong {} else {
                __assert_fail(
                    b"rekey_at > 0\0" as *const u8 as *const libc::c_char,
                    b"vanitygen.c\0" as *const u8 as *const libc::c_char,
                    136 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 29],
                        &[libc::c_char; 29],
                    >(b"void *vg_thread_loop(void *)\0"))
                        .as_ptr(),
                );
            }
            'c_9993: {
                if rekey_at > 0 as libc::c_int as libc::c_ulong {} else {
                    __assert_fail(
                        b"rekey_at > 0\0" as *const u8 as *const libc::c_char,
                        b"vanitygen.c\0" as *const u8 as *const libc::c_char,
                        136 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 29],
                            &[libc::c_char; 29],
                        >(b"void *vg_thread_loop(void *)\0"))
                            .as_ptr(),
                    );
                }
            };
            EC_POINT_copy(ppnt[0 as libc::c_int as usize], EC_KEY_get0_public_key(pkey));
            vg_exec_context_downgrade_lock(vxcp);
            npoints = npoints.wrapping_add(1);
            npoints;
            (*vxcp).vxc_delta = 0 as libc::c_int;
            if !((*vcp).vc_pubkey_base).is_null() {
                EC_POINT_add(
                    pgroup,
                    ppnt[0 as libc::c_int as usize],
                    ppnt[0 as libc::c_int as usize],
                    (*vcp).vc_pubkey_base,
                    (*vxcp).vxc_bnctx,
                );
            }
            nbatch = 1 as libc::c_int as libc::c_ulong;
            while nbatch < ptarraysize as libc::c_ulong && npoints < rekey_at {
                EC_POINT_add(
                    pgroup,
                    ppnt[nbatch as usize],
                    ppnt[nbatch.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        as usize],
                    pgen,
                    (*vxcp).vxc_bnctx,
                );
                nbatch = nbatch.wrapping_add(1);
                nbatch;
                npoints = npoints.wrapping_add(1);
                npoints;
            }
        } else {
            if nbatch == ptarraysize as libc::c_ulong {} else {
                __assert_fail(
                    b"nbatch == ptarraysize\0" as *const u8 as *const libc::c_char,
                    b"vanitygen.c\0" as *const u8 as *const libc::c_char,
                    169 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 29],
                        &[libc::c_char; 29],
                    >(b"void *vg_thread_loop(void *)\0"))
                        .as_ptr(),
                );
            }
            'c_9821: {
                if nbatch == ptarraysize as libc::c_ulong {} else {
                    __assert_fail(
                        b"nbatch == ptarraysize\0" as *const u8 as *const libc::c_char,
                        b"vanitygen.c\0" as *const u8 as *const libc::c_char,
                        169 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 29],
                            &[libc::c_char; 29],
                        >(b"void *vg_thread_loop(void *)\0"))
                            .as_ptr(),
                    );
                }
            };
            nbatch = 0 as libc::c_int as libc::c_ulong;
            while nbatch < ptarraysize as libc::c_ulong && npoints < rekey_at {
                EC_POINT_add(
                    pgroup,
                    ppnt[nbatch as usize],
                    ppnt[nbatch as usize],
                    pbatchinc,
                    (*vxcp).vxc_bnctx,
                );
                nbatch = nbatch.wrapping_add(1);
                nbatch;
                npoints = npoints.wrapping_add(1);
                npoints;
            }
        }
        EC_POINTs_make_affine(pgroup, nbatch, ppnt.as_mut_ptr(), (*vxcp).vxc_bnctx);
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) < nbatch {
            len = EC_POINT_point2oct(
                pgroup,
                ppnt[i as usize],
                POINT_CONVERSION_UNCOMPRESSED,
                eckey_buf,
                65 as libc::c_int as size_t,
                (*vxcp).vxc_bnctx,
            ) as libc::c_int;
            if len == 65 as libc::c_int {} else {
                __assert_fail(
                    b"len == 65\0" as *const u8 as *const libc::c_char,
                    b"vanitygen.c\0" as *const u8 as *const libc::c_char,
                    201 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 29],
                        &[libc::c_char; 29],
                    >(b"void *vg_thread_loop(void *)\0"))
                        .as_ptr(),
                );
            }
            'c_9666: {
                if len == 65 as libc::c_int {} else {
                    __assert_fail(
                        b"len == 65\0" as *const u8 as *const libc::c_char,
                        b"vanitygen.c\0" as *const u8 as *const libc::c_char,
                        201 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 29],
                            &[libc::c_char; 29],
                        >(b"void *vg_thread_loop(void *)\0"))
                            .as_ptr(),
                    );
                }
            };
            SHA256(hash_buf.as_mut_ptr(), hash_len as size_t, hash1.as_mut_ptr());
            RIPEMD160(
                hash1.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong,
                &mut *((*vxcp).vxc_binres).as_mut_ptr().offset(1 as libc::c_int as isize),
            );
            match test_func.unwrap()(vxcp) {
                1 => {
                    npoints = 0 as libc::c_int as libc::c_ulong;
                    rekey_at = 0 as libc::c_int as libc::c_ulong;
                    i = nbatch as libc::c_int;
                }
                2 => {
                    break 's_173;
                }
                _ => {}
            }
            i += 1;
            i;
            (*vxcp).vxc_delta += 1;
            (*vxcp).vxc_delta;
        }
        c += i;
        if c >= output_interval {
            output_interval = vg_output_timing(vcp, c, &mut tvstart);
            if output_interval > 250000 as libc::c_int {
                output_interval = 250000 as libc::c_int;
            }
            c = 0 as libc::c_int;
        }
        vg_exec_context_yield(vxcp);
    }
    vg_exec_context_del(&mut ctx);
    vg_context_thread_exit(vcp);
    i = 0 as libc::c_int;
    while i < ptarraysize {
        if !(ppnt[i as usize]).is_null() {
            EC_POINT_free(ppnt[i as usize]);
        }
        i += 1;
        i;
    }
    if !pbatchinc.is_null() {
        EC_POINT_free(pbatchinc);
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn count_processors() -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut count: libc::c_int = 0 as libc::c_int;
    fp = fopen(
        b"/proc/cpuinfo\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if fp.is_null() {
        return -(1 as libc::c_int);
    }
    while !(fgets(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as libc::c_int,
        fp,
    ))
        .is_null()
    {
        if strncmp(
            buf.as_mut_ptr(),
            b"processor\t\0" as *const u8 as *const libc::c_char,
            10 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            count += 1 as libc::c_int;
        }
    }
    fclose(fp);
    return count;
}
pub unsafe extern "C" fn start_threads(
    mut vcp: *mut vg_context_t,
    mut nthreads: libc::c_int,
) -> libc::c_int {
    let mut thread: pthread_t = 0;
    if nthreads <= 0 as libc::c_int {
        nthreads = count_processors();
        if nthreads <= 0 as libc::c_int {
            fprintf(
                stderr,
                b"ERROR: could not determine processor count\n\0" as *const u8
                    as *const libc::c_char,
            );
            nthreads = 1 as libc::c_int;
        }
    }
    if (*vcp).vc_verbose > 1 as libc::c_int {
        fprintf(
            stderr,
            b"Using %d worker thread(s)\n\0" as *const u8 as *const libc::c_char,
            nthreads,
        );
    }
    loop {
        nthreads -= 1;
        if !(nthreads != 0) {
            break;
        }
        if pthread_create(
            &mut thread,
            0 as *const pthread_attr_t,
            Some(
                vg_thread_loop
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            vcp as *mut libc::c_void,
        ) != 0
        {
            return 0 as libc::c_int;
        }
    }
    vg_thread_loop(vcp as *mut libc::c_void);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn usage(mut name: *const libc::c_char) {
    fprintf(
        stderr,
        b"Vanitygen %s (OpenSSL 1.0.1u  22 Sep 2016)\nUsage: %s [-vqnrik1NT] [-t <threads>] [-f <filename>|-] [<pattern>...]\nGenerates a bitcoin receiving address matching <pattern>, and outputs the\naddress and associated private key.  The private key may be stored in a safe\nlocation or imported into a bitcoin client to spend any balance received on\nthe address.\nBy default, <pattern> is interpreted as an exact prefix.\n\nOptions:\n-v            Verbose output\n-q            Quiet output\n-n            Simulate\n-r            Use regular expression match instead of prefix\n              (Feasibility of expression is not checked)\n-i            Case-insensitive prefix search\n-k            Keep pattern and continue search after finding a match\n-1            Stop after first match\n-N            Generate namecoin address\n-T            Generate bitcoin testnet address\n-X <version>  Generate address with the given version\n-F <format>   Generate address with the given format (pubkey or script)\n-P <pubkey>   Specify base public key for piecewise key generation\n-e            Encrypt private keys, prompt for password\n-E <password> Encrypt private keys with <password> (UNSAFE)\n-t <threads>  Set number of worker threads (Default: number of CPUs)\n-f <file>     File containing list of patterns, one per line\n              (Use \"-\" as the file name for stdin)\n-o <file>     Write pattern matches to <file>\n-s <file>     Seed random number generator from <file>\n\0"
            as *const u8 as *const libc::c_char,
        version,
        name,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut addrtype: libc::c_int = 0 as libc::c_int;
    let mut scriptaddrtype: libc::c_int = 5 as libc::c_int;
    let mut privtype: libc::c_int = 128 as libc::c_int;
    let mut pubkeytype: libc::c_int = 0;
    let mut format: vg_format = VCF_PUBKEY;
    let mut regex: libc::c_int = 0 as libc::c_int;
    let mut caseinsensitive: libc::c_int = 0 as libc::c_int;
    let mut verbose: libc::c_int = 1 as libc::c_int;
    let mut simulate: libc::c_int = 0 as libc::c_int;
    let mut remove_on_match: libc::c_int = 1 as libc::c_int;
    let mut only_one: libc::c_int = 0 as libc::c_int;
    let mut prompt_password: libc::c_int = 0 as libc::c_int;
    let mut opt: libc::c_int = 0;
    let mut seedfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pwbuf: [libc::c_char; 128] = [0; 128];
    let mut result_file: *const libc::c_char = 0 as *const libc::c_char;
    let mut key_password: *const libc::c_char = 0 as *const libc::c_char;
    let mut patterns: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut npatterns: libc::c_int = 0 as libc::c_int;
    let mut nthreads: libc::c_int = 0 as libc::c_int;
    let mut vcp: *mut vg_context_t = 0 as *mut vg_context_t;
    let mut pubkey_base: *mut EC_POINT = 0 as *mut EC_POINT;
    let mut pattfp: [*mut FILE; 4] = [0 as *mut FILE; 4];
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut pattfpi: [libc::c_int; 4] = [0; 4];
    let mut npattfp: libc::c_int = 0 as libc::c_int;
    let mut pattstdin: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    loop {
        opt = getopt(
            argc,
            argv,
            b"vqnrik1eE:P:NTX:F:t:h?f:o:s:\0" as *const u8 as *const libc::c_char,
        );
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        match opt {
            118 => {
                verbose = 2 as libc::c_int;
            }
            113 => {
                verbose = 0 as libc::c_int;
            }
            110 => {
                simulate = 1 as libc::c_int;
            }
            114 => {
                regex = 1 as libc::c_int;
            }
            105 => {
                caseinsensitive = 1 as libc::c_int;
            }
            107 => {
                remove_on_match = 0 as libc::c_int;
            }
            49 => {
                only_one = 1 as libc::c_int;
            }
            78 => {
                addrtype = 52 as libc::c_int;
                privtype = 180 as libc::c_int;
                scriptaddrtype = -(1 as libc::c_int);
            }
            84 => {
                addrtype = 111 as libc::c_int;
                privtype = 239 as libc::c_int;
                scriptaddrtype = 196 as libc::c_int;
            }
            88 => {
                addrtype = atoi(optarg);
                privtype = 128 as libc::c_int + addrtype;
                scriptaddrtype = addrtype;
            }
            70 => {
                if strcmp(optarg, b"script\0" as *const u8 as *const libc::c_char) == 0 {
                    format = VCF_SCRIPT;
                } else if strcmp(optarg, b"pubkey\0" as *const u8 as *const libc::c_char)
                    != 0
                {
                    fprintf(
                        stderr,
                        b"Invalid format '%s'\n\0" as *const u8 as *const libc::c_char,
                        optarg,
                    );
                    return 1 as libc::c_int;
                }
            }
            80 => {
                if !pubkey_base.is_null() {
                    fprintf(
                        stderr,
                        b"Multiple base pubkeys specified\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    return 1 as libc::c_int;
                }
                let mut pkey: *mut EC_KEY = vg_exec_context_new_key();
                pubkey_base = EC_POINT_hex2point(
                    EC_KEY_get0_group(pkey),
                    optarg,
                    0 as *mut EC_POINT,
                    0 as *mut BN_CTX,
                );
                EC_KEY_free(pkey);
                if pubkey_base.is_null() {
                    fprintf(
                        stderr,
                        b"Invalid base pubkey\n\0" as *const u8 as *const libc::c_char,
                    );
                    return 1 as libc::c_int;
                }
            }
            101 => {
                prompt_password = 1 as libc::c_int;
            }
            69 => {
                key_password = optarg;
            }
            116 => {
                nthreads = atoi(optarg);
                if nthreads == 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Invalid thread count '%s'\n\0" as *const u8
                            as *const libc::c_char,
                        optarg,
                    );
                    return 1 as libc::c_int;
                }
            }
            102 => {
                if npattfp >= 4 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Too many input files specified\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    return 1 as libc::c_int;
                }
                if strcmp(optarg, b"-\0" as *const u8 as *const libc::c_char) == 0 {
                    if pattstdin != 0 {
                        fprintf(
                            stderr,
                            b"ERROR: stdin specified multiple times\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        return 1 as libc::c_int;
                    }
                    fp = stdin;
                } else {
                    fp = fopen(optarg, b"r\0" as *const u8 as *const libc::c_char);
                    if fp.is_null() {
                        fprintf(
                            stderr,
                            b"Could not open %s: %s\n\0" as *const u8
                                as *const libc::c_char,
                            optarg,
                            strerror(*__errno_location()),
                        );
                        return 1 as libc::c_int;
                    }
                }
                pattfp[npattfp as usize] = fp;
                pattfpi[npattfp as usize] = caseinsensitive;
                npattfp += 1;
                npattfp;
            }
            111 => {
                if !result_file.is_null() {
                    fprintf(
                        stderr,
                        b"Multiple output files specified\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    return 1 as libc::c_int;
                }
                result_file = optarg;
            }
            115 => {
                if !seedfile.is_null() {
                    fprintf(
                        stderr,
                        b"Multiple RNG seeds specified\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    return 1 as libc::c_int;
                }
                seedfile = optarg;
            }
            _ => {
                usage(*argv.offset(0 as libc::c_int as isize));
                return 1 as libc::c_int;
            }
        }
    }
    if caseinsensitive != 0 && regex != 0 {
        fprintf(
            stderr,
            b"WARNING: case insensitive mode incompatible with regular expressions\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    pubkeytype = addrtype;
    if format as libc::c_uint == VCF_SCRIPT as libc::c_int as libc::c_uint {
        if scriptaddrtype == -(1 as libc::c_int) {
            fprintf(
                stderr,
                b"Address type incompatible with script format\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
        addrtype = scriptaddrtype;
    }
    if !seedfile.is_null() {
        opt = -(1 as libc::c_int);
        let mut st: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        if stat(seedfile, &mut st) == 0
            && st.st_mode
                & (0o60000 as libc::c_int | 0o20000 as libc::c_int) as libc::c_uint != 0
        {
            opt = 32 as libc::c_int;
        }
        opt = RAND_load_file(seedfile, opt as libc::c_long);
        if opt == 0 {
            fprintf(
                stderr,
                b"Could not load RNG seed %s\n\0" as *const u8 as *const libc::c_char,
                optarg,
            );
            return 1 as libc::c_int;
        }
        if verbose > 0 as libc::c_int {
            fprintf(
                stderr,
                b"Read %d bytes from RNG seed file\n\0" as *const u8
                    as *const libc::c_char,
                opt,
            );
        }
    }
    if regex != 0 {
        vcp = vg_regex_context_new(addrtype, privtype);
    } else {
        vcp = vg_prefix_context_new(addrtype, privtype, caseinsensitive);
    }
    (*vcp).vc_verbose = verbose;
    (*vcp).vc_result_file = result_file;
    (*vcp).vc_remove_on_match = remove_on_match;
    (*vcp).vc_only_one = only_one;
    (*vcp).vc_format = format;
    (*vcp).vc_pubkeytype = pubkeytype;
    (*vcp).vc_pubkey_base = pubkey_base;
    (*vcp)
        .vc_output_match = Some(
        vg_output_match_console
            as unsafe extern "C" fn(
                *mut vg_context_t,
                *mut EC_KEY,
                *const libc::c_char,
            ) -> (),
    );
    (*vcp)
        .vc_output_timing = Some(
        vg_output_timing_console
            as unsafe extern "C" fn(
                *mut vg_context_t,
                libc::c_double,
                libc::c_ulonglong,
                libc::c_ulonglong,
            ) -> (),
    );
    if npattfp == 0 {
        if optind >= argc {
            usage(*argv.offset(0 as libc::c_int as isize));
            return 1 as libc::c_int;
        }
        patterns = &mut *argv.offset(optind as isize) as *mut *mut libc::c_char;
        npatterns = argc - optind;
        if vg_context_add_patterns(vcp, patterns as *mut *const libc::c_char, npatterns)
            == 0
        {
            return 1 as libc::c_int;
        }
    }
    i = 0 as libc::c_int;
    while i < npattfp {
        fp = pattfp[i as usize];
        if vg_read_file(fp, &mut patterns, &mut npatterns) == 0 {
            fprintf(
                stderr,
                b"Failed to load pattern file\n\0" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
        if fp != stdin {
            fclose(fp);
        }
        if regex == 0 {
            vg_prefix_context_set_case_insensitive(vcp, pattfpi[i as usize]);
        }
        if vg_context_add_patterns(vcp, patterns as *mut *const libc::c_char, npatterns)
            == 0
        {
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    if (*vcp).vc_npatterns == 0 {
        fprintf(
            stderr,
            b"No patterns to search\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if prompt_password != 0 {
        if vg_read_password(
            pwbuf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        ) == 0
        {
            return 1 as libc::c_int;
        }
        key_password = pwbuf.as_mut_ptr();
    }
    (*vcp).vc_key_protect_pass = key_password;
    if !key_password.is_null() {
        if vg_check_password_complexity(key_password, verbose) == 0 {
            fprintf(
                stderr,
                b"WARNING: Protecting private keys with weak password\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if verbose > 0 as libc::c_int && regex != 0
        && (*vcp).vc_npatterns > 1 as libc::c_int as libc::c_ulong
    {
        fprintf(
            stderr,
            b"Regular expressions: %ld\n\0" as *const u8 as *const libc::c_char,
            (*vcp).vc_npatterns,
        );
    }
    if simulate != 0 {
        return 0 as libc::c_int;
    }
    if start_threads(vcp, nthreads) == 0 {
        return 1 as libc::c_int;
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
