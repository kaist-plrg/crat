use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type bignum_ctx;
    pub type ec_group_st;
    pub type ec_point_st;
    pub type ec_key_st;
    pub type real_pcre;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
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
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
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
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn BN_value_one() -> *const BIGNUM;
    fn BN_CTX_new() -> *mut BN_CTX;
    fn BN_CTX_free(c: *mut BN_CTX);
    fn BN_new() -> *mut BIGNUM;
    fn BN_init(_: *mut BIGNUM);
    fn BN_clear_free(a: *mut BIGNUM);
    fn BN_copy(a: *mut BIGNUM, b: *const BIGNUM) -> *mut BIGNUM;
    fn BN_bin2bn(
        s: *const libc::c_uchar,
        len: libc::c_int,
        ret: *mut BIGNUM,
    ) -> *mut BIGNUM;
    fn BN_bn2bin(a: *const BIGNUM, to: *mut libc::c_uchar) -> libc::c_int;
    fn BN_sub(r: *mut BIGNUM, a: *const BIGNUM, b: *const BIGNUM) -> libc::c_int;
    fn BN_add(r: *mut BIGNUM, a: *const BIGNUM, b: *const BIGNUM) -> libc::c_int;
    fn BN_mul(
        r: *mut BIGNUM,
        a: *const BIGNUM,
        b: *const BIGNUM,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn BN_div(
        dv: *mut BIGNUM,
        rem: *mut BIGNUM,
        m: *const BIGNUM,
        d: *const BIGNUM,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn BN_set_word(a: *mut BIGNUM, w: libc::c_ulong) -> libc::c_int;
    fn BN_get_word(a: *const BIGNUM) -> libc::c_ulong;
    fn BN_cmp(a: *const BIGNUM, b: *const BIGNUM) -> libc::c_int;
    fn BN_free(a: *mut BIGNUM);
    fn BN_lshift(r: *mut BIGNUM, a: *const BIGNUM, n: libc::c_int) -> libc::c_int;
    fn BN_exp(
        r: *mut BIGNUM,
        a: *const BIGNUM,
        p: *const BIGNUM,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn BN_clear(a: *mut BIGNUM);
    fn BN_set_bit(a: *mut BIGNUM, n: libc::c_int) -> libc::c_int;
    fn BN_bn2dec(a: *const BIGNUM) -> *mut libc::c_char;
    fn CRYPTO_free(ptr: *mut libc::c_void);
    fn EC_POINT_new(group: *const EC_GROUP) -> *mut EC_POINT;
    fn EC_POINT_free(point: *mut EC_POINT);
    fn EC_POINT_copy(dst: *mut EC_POINT, src: *const EC_POINT) -> libc::c_int;
    fn EC_POINT_point2oct(
        group: *const EC_GROUP,
        p: *const EC_POINT,
        form: point_conversion_form_t,
        buf: *mut libc::c_uchar,
        len: size_t,
        ctx: *mut BN_CTX,
    ) -> size_t;
    fn EC_POINT_add(
        group: *const EC_GROUP,
        r: *mut EC_POINT,
        a: *const EC_POINT,
        b: *const EC_POINT,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn EC_KEY_new_by_curve_name(nid: libc::c_int) -> *mut EC_KEY;
    fn EC_KEY_get0_group(key: *const EC_KEY) -> *const EC_GROUP;
    fn EC_KEY_get0_private_key(key: *const EC_KEY) -> *const BIGNUM;
    fn EC_KEY_get0_public_key(key: *const EC_KEY) -> *const EC_POINT;
    fn EC_KEY_precompute_mult(key: *mut EC_KEY, ctx: *mut BN_CTX) -> libc::c_int;
    fn EC_KEY_check_key(key: *const EC_KEY) -> libc::c_int;
    fn i2d_ECPrivateKey(key: *mut EC_KEY, out: *mut *mut libc::c_uchar) -> libc::c_int;
    fn i2o_ECPublicKey(key: *mut EC_KEY, out: *mut *mut libc::c_uchar) -> libc::c_int;
    static mut pcre_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
    fn pcre_compile(
        _: *const libc::c_char,
        _: libc::c_int,
        _: *mut *const libc::c_char,
        _: *mut libc::c_int,
        _: *const libc::c_uchar,
    ) -> *mut pcre;
    fn pcre_exec(
        _: *const pcre,
        _: *const pcre_extra,
        _: *const libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn pcre_study(
        _: *const pcre,
        _: libc::c_int,
        _: *mut *const libc::c_char,
    ) -> *mut pcre_extra;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    static mut vg_b58_alphabet: *const libc::c_char;
    static vg_b58_reverse_map: [libc::c_schar; 256];
    fn dumphex(src: *const libc::c_uchar, len: size_t);
    fn dumpbn(bn: *const BIGNUM);
    fn vg_encode_address(
        ppoint: *const EC_POINT,
        pgroup: *const EC_GROUP,
        addrtype: libc::c_int,
        result: *mut libc::c_char,
    );
    fn vg_encode_script_address(
        ppoint: *const EC_POINT,
        pgroup: *const EC_GROUP,
        addrtype: libc::c_int,
        result: *mut libc::c_char,
    );
    fn vg_encode_privkey(
        pkey: *const EC_KEY,
        addrtype: libc::c_int,
        result: *mut libc::c_char,
    );
    fn vg_set_privkey(bnpriv: *const BIGNUM, pkey: *mut EC_KEY) -> libc::c_int;
    fn vg_protect_encode_privkey(
        out: *mut libc::c_char,
        pkey: *const EC_KEY,
        keytype: libc::c_int,
        parameter_group: libc::c_int,
        pass: *const libc::c_char,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
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
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
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
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_3 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_3 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_3 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_3 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_3 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_3 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_3 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_3 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type point_conversion_form_t = libc::c_uint;
pub const POINT_CONVERSION_HYBRID: point_conversion_form_t = 6;
pub const POINT_CONVERSION_UNCOMPRESSED: point_conversion_form_t = 4;
pub const POINT_CONVERSION_COMPRESSED: point_conversion_form_t = 2;
pub type EC_GROUP = ec_group_st;
pub type EC_POINT = ec_point_st;
pub type EC_KEY = ec_key_st;
pub type pcre = real_pcre;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pcre_extra {
    pub flags: libc::c_ulong,
    pub study_data: *mut libc::c_void,
    pub match_limit: libc::c_ulong,
    pub callout_data: *mut libc::c_void,
    pub tables: *const libc::c_uchar,
    pub match_limit_recursion: libc::c_ulong,
    pub mark: *mut *mut libc::c_uchar,
    pub executable_jit: *mut libc::c_void,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _timing_info_s {
    pub ti_next: *mut _timing_info_s,
    pub ti_thread: pthread_t,
    pub ti_last_rate: libc::c_ulong,
    pub ti_hist_time: [libc::c_ulonglong; 5],
    pub ti_hist_work: [libc::c_ulong; 5],
    pub ti_hist_last: libc::c_int,
}
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
pub type timing_info_t = _timing_info_s;
pub type vg_prefix_context_t = _vg_prefix_context_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _vg_prefix_context_s {
    pub base: vg_context_t,
    pub vcp_avlroot: avl_root_t,
    pub vcp_difficulty: BIGNUM,
    pub vcp_caseinsensitive: libc::c_int,
}
pub type avl_root_t = _avl_root_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _avl_root_s {
    pub ar_root: *mut avl_item_t,
}
pub type avl_item_t = _avl_item_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _avl_item_s {
    pub ai_left: *mut _avl_item_s,
    pub ai_right: *mut _avl_item_s,
    pub ai_up: *mut _avl_item_s,
    pub ai_balance: avl_balance_t,
    pub ai_indexed: libc::c_int,
}
pub type avl_balance_t = libc::c_uint;
pub const RIGHT: avl_balance_t = 2;
pub const LEFT: avl_balance_t = 0;
pub const CENT: avl_balance_t = 1;
pub type vg_prefix_t = _vg_prefix_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _vg_prefix_s {
    pub vp_item: avl_item_t,
    pub vp_sibling: *mut _vg_prefix_s,
    pub vp_pattern: *const libc::c_char,
    pub vp_low: *mut BIGNUM,
    pub vp_high: *mut BIGNUM,
}
pub type prefix_case_iter_t = _prefix_case_iter_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _prefix_case_iter_s {
    pub ci_prefix: [libc::c_char; 32],
    pub ci_case_map: [libc::c_char; 32],
    pub ci_nbits: libc::c_char,
    pub ci_value: libc::c_int,
}
pub type vg_regex_context_t = _vg_regex_context_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _vg_regex_context_s {
    pub base: vg_context_t,
    pub vcr_regex: *mut *mut pcre,
    pub vcr_regex_extra: *mut *mut pcre_extra,
    pub vcr_regex_pat: *mut *const libc::c_char,
    pub vcr_nalloc: libc::c_ulong,
}
pub const timing_hist_size: C2RustUnnamed_5 = 5;
pub const VG_PROTKEY_DEFAULT: C2RustUnnamed_4 = -1;
pub type C2RustUnnamed_4 = libc::c_int;
pub const VG_PROTKEY_PKCS_PBKDF2_4096_HMAC_SHA256_AES_256_CBC: C2RustUnnamed_4 = 16;
pub const VG_PROTKEY_BRIEF_PBKDF2_4096_HMAC_SHA256_AES_256_CBC: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_5 = libc::c_uint;
#[inline]
unsafe extern "C" fn pthread_equal(
    mut __thread1: pthread_t,
    mut __thread2: pthread_t,
) -> libc::c_int {
    return (__thread1 == __thread2) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn avl_root_init(mut rootp: *mut avl_root_t) {
    (*rootp).ar_root = 0 as *mut avl_item_t;
}
#[inline]
unsafe extern "C" fn avl_root_empty(mut rootp: *mut avl_root_t) -> libc::c_int {
    return if ((*rootp).ar_root).is_null() {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn avl_item_init(mut itemp: *mut avl_item_t) {
    (*itemp).ai_left = 0 as *mut _avl_item_s;
    (*itemp).ai_right = 0 as *mut _avl_item_s;
    (*itemp).ai_up = 0 as *mut _avl_item_s;
    (*itemp).ai_balance = CENT;
    (*itemp).ai_indexed = 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _avl_rotate_ll(
    mut rootp: *mut avl_root_t,
    mut itemp: *mut avl_item_t,
) {
    let mut tmp: *mut avl_item_t = 0 as *mut avl_item_t;
    tmp = (*itemp).ai_left;
    (*itemp).ai_left = (*tmp).ai_right;
    if !((*itemp).ai_left).is_null() {
        (*(*itemp).ai_left).ai_up = itemp;
    }
    (*tmp).ai_right = itemp;
    if !((*itemp).ai_up).is_null() {
        if (*(*itemp).ai_up).ai_left == itemp {
            (*(*itemp).ai_up).ai_left = tmp;
        } else {
            if (*(*itemp).ai_up).ai_right == itemp {} else {
                __assert_fail(
                    b"itemp->ai_up->ai_right == itemp\0" as *const u8
                        as *const libc::c_char,
                    b"./avl.h\0" as *const u8 as *const libc::c_char,
                    89 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 48],
                        &[libc::c_char; 48],
                    >(b"void _avl_rotate_ll(avl_root_t *, avl_item_t *)\0"))
                        .as_ptr(),
                );
            }
            'c_11942: {
                if (*(*itemp).ai_up).ai_right == itemp {} else {
                    __assert_fail(
                        b"itemp->ai_up->ai_right == itemp\0" as *const u8
                            as *const libc::c_char,
                        b"./avl.h\0" as *const u8 as *const libc::c_char,
                        89 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 48],
                            &[libc::c_char; 48],
                        >(b"void _avl_rotate_ll(avl_root_t *, avl_item_t *)\0"))
                            .as_ptr(),
                    );
                }
            };
            (*(*itemp).ai_up).ai_right = tmp;
        }
    } else {
        (*rootp).ar_root = tmp;
    }
    (*tmp).ai_up = (*itemp).ai_up;
    (*itemp).ai_up = tmp;
}
#[inline]
unsafe extern "C" fn _avl_rotate_lr(
    mut rootp: *mut avl_root_t,
    mut itemp: *mut avl_item_t,
) {
    let mut rcp: *mut avl_item_t = 0 as *mut avl_item_t;
    let mut rlcp: *mut avl_item_t = 0 as *mut avl_item_t;
    rcp = (*itemp).ai_left;
    rlcp = (*rcp).ai_right;
    if !((*itemp).ai_up).is_null() {
        if itemp == (*(*itemp).ai_up).ai_left {
            (*(*itemp).ai_up).ai_left = rlcp;
        } else {
            if itemp == (*(*itemp).ai_up).ai_right {} else {
                __assert_fail(
                    b"itemp == itemp->ai_up->ai_right\0" as *const u8
                        as *const libc::c_char,
                    b"./avl.h\0" as *const u8 as *const libc::c_char,
                    109 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 48],
                        &[libc::c_char; 48],
                    >(b"void _avl_rotate_lr(avl_root_t *, avl_item_t *)\0"))
                        .as_ptr(),
                );
            }
            'c_11783: {
                if itemp == (*(*itemp).ai_up).ai_right {} else {
                    __assert_fail(
                        b"itemp == itemp->ai_up->ai_right\0" as *const u8
                            as *const libc::c_char,
                        b"./avl.h\0" as *const u8 as *const libc::c_char,
                        109 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 48],
                            &[libc::c_char; 48],
                        >(b"void _avl_rotate_lr(avl_root_t *, avl_item_t *)\0"))
                            .as_ptr(),
                    );
                }
            };
            (*(*itemp).ai_up).ai_right = rlcp;
        }
    } else {
        (*rootp).ar_root = rlcp;
    }
    (*rlcp).ai_up = (*itemp).ai_up;
    (*rcp).ai_right = (*rlcp).ai_left;
    if !((*rcp).ai_right).is_null() {
        (*(*rcp).ai_right).ai_up = rcp;
    }
    (*itemp).ai_left = (*rlcp).ai_right;
    if !((*itemp).ai_left).is_null() {
        (*(*itemp).ai_left).ai_up = itemp;
    }
    (*rlcp).ai_left = rcp;
    (*rlcp).ai_right = itemp;
    (*rcp).ai_up = rlcp;
    (*itemp).ai_up = rlcp;
}
#[inline]
unsafe extern "C" fn _avl_rotate_rr(
    mut rootp: *mut avl_root_t,
    mut itemp: *mut avl_item_t,
) {
    let mut tmp: *mut avl_item_t = 0 as *mut avl_item_t;
    tmp = (*itemp).ai_right;
    (*itemp).ai_right = (*tmp).ai_left;
    if !((*itemp).ai_right).is_null() {
        (*(*itemp).ai_right).ai_up = itemp;
    }
    (*tmp).ai_left = itemp;
    if !((*itemp).ai_up).is_null() {
        if (*(*itemp).ai_up).ai_right == itemp {
            (*(*itemp).ai_up).ai_right = tmp;
        } else {
            if (*(*itemp).ai_up).ai_left == itemp {} else {
                __assert_fail(
                    b"itemp->ai_up->ai_left == itemp\0" as *const u8
                        as *const libc::c_char,
                    b"./avl.h\0" as *const u8 as *const libc::c_char,
                    142 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 48],
                        &[libc::c_char; 48],
                    >(b"void _avl_rotate_rr(avl_root_t *, avl_item_t *)\0"))
                        .as_ptr(),
                );
            }
            'c_11409: {
                if (*(*itemp).ai_up).ai_left == itemp {} else {
                    __assert_fail(
                        b"itemp->ai_up->ai_left == itemp\0" as *const u8
                            as *const libc::c_char,
                        b"./avl.h\0" as *const u8 as *const libc::c_char,
                        142 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 48],
                            &[libc::c_char; 48],
                        >(b"void _avl_rotate_rr(avl_root_t *, avl_item_t *)\0"))
                            .as_ptr(),
                    );
                }
            };
            (*(*itemp).ai_up).ai_left = tmp;
        }
    } else {
        (*rootp).ar_root = tmp;
    }
    (*tmp).ai_up = (*itemp).ai_up;
    (*itemp).ai_up = tmp;
}
#[inline]
unsafe extern "C" fn _avl_rotate_rl(
    mut rootp: *mut avl_root_t,
    mut itemp: *mut avl_item_t,
) {
    let mut rcp: *mut avl_item_t = 0 as *mut avl_item_t;
    let mut rlcp: *mut avl_item_t = 0 as *mut avl_item_t;
    rcp = (*itemp).ai_right;
    rlcp = (*rcp).ai_left;
    if !((*itemp).ai_up).is_null() {
        if itemp == (*(*itemp).ai_up).ai_right {
            (*(*itemp).ai_up).ai_right = rlcp;
        } else {
            if itemp == (*(*itemp).ai_up).ai_left {} else {
                __assert_fail(
                    b"itemp == itemp->ai_up->ai_left\0" as *const u8
                        as *const libc::c_char,
                    b"./avl.h\0" as *const u8 as *const libc::c_char,
                    162 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 48],
                        &[libc::c_char; 48],
                    >(b"void _avl_rotate_rl(avl_root_t *, avl_item_t *)\0"))
                        .as_ptr(),
                );
            }
            'c_11246: {
                if itemp == (*(*itemp).ai_up).ai_left {} else {
                    __assert_fail(
                        b"itemp == itemp->ai_up->ai_left\0" as *const u8
                            as *const libc::c_char,
                        b"./avl.h\0" as *const u8 as *const libc::c_char,
                        162 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 48],
                            &[libc::c_char; 48],
                        >(b"void _avl_rotate_rl(avl_root_t *, avl_item_t *)\0"))
                            .as_ptr(),
                    );
                }
            };
            (*(*itemp).ai_up).ai_left = rlcp;
        }
    } else {
        (*rootp).ar_root = rlcp;
    }
    (*rlcp).ai_up = (*itemp).ai_up;
    (*rcp).ai_left = (*rlcp).ai_right;
    if !((*rcp).ai_left).is_null() {
        (*(*rcp).ai_left).ai_up = rcp;
    }
    (*itemp).ai_right = (*rlcp).ai_left;
    if !((*itemp).ai_right).is_null() {
        (*(*itemp).ai_right).ai_up = itemp;
    }
    (*rlcp).ai_right = rcp;
    (*rlcp).ai_left = itemp;
    (*rcp).ai_up = rlcp;
    (*itemp).ai_up = rlcp;
}
unsafe extern "C" fn avl_delete_fix(
    mut rootp: *mut avl_root_t,
    mut itemp: *mut avl_item_t,
    mut parentp: *mut avl_item_t,
) {
    let mut childp: *mut avl_item_t = 0 as *mut avl_item_t;
    if ((*parentp).ai_left).is_null() && ((*parentp).ai_right).is_null() {
        if itemp.is_null() {} else {
            __assert_fail(
                b"itemp == NULL\0" as *const u8 as *const libc::c_char,
                b"./avl.h\0" as *const u8 as *const libc::c_char,
                188 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"void avl_delete_fix(avl_root_t *, avl_item_t *, avl_item_t *)\0"))
                    .as_ptr(),
            );
        }
        'c_12124: {
            if itemp.is_null() {} else {
                __assert_fail(
                    b"itemp == NULL\0" as *const u8 as *const libc::c_char,
                    b"./avl.h\0" as *const u8 as *const libc::c_char,
                    188 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 62],
                        &[libc::c_char; 62],
                    >(
                        b"void avl_delete_fix(avl_root_t *, avl_item_t *, avl_item_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        (*parentp).ai_balance = CENT;
        itemp = parentp;
        parentp = (*itemp).ai_up;
    }
    while !parentp.is_null() {
        if itemp == (*parentp).ai_right {
            itemp = (*parentp).ai_left;
            if (*parentp).ai_balance as libc::c_uint
                == LEFT as libc::c_int as libc::c_uint
            {
                if (*itemp).ai_balance as libc::c_uint
                    == LEFT as libc::c_int as libc::c_uint
                {
                    _avl_rotate_ll(rootp, parentp);
                    (*itemp).ai_balance = CENT;
                    (*parentp).ai_balance = CENT;
                    parentp = itemp;
                } else if (*itemp).ai_balance as libc::c_uint
                    == CENT as libc::c_int as libc::c_uint
                {
                    _avl_rotate_ll(rootp, parentp);
                    (*itemp).ai_balance = RIGHT;
                    (*parentp).ai_balance = LEFT;
                    break;
                } else {
                    childp = (*itemp).ai_right;
                    _avl_rotate_lr(rootp, parentp);
                    (*itemp).ai_balance = CENT;
                    (*parentp).ai_balance = CENT;
                    if (*childp).ai_balance as libc::c_uint
                        == RIGHT as libc::c_int as libc::c_uint
                    {
                        (*itemp).ai_balance = LEFT;
                    }
                    if (*childp).ai_balance as libc::c_uint
                        == LEFT as libc::c_int as libc::c_uint
                    {
                        (*parentp).ai_balance = RIGHT;
                    }
                    (*childp).ai_balance = CENT;
                    parentp = childp;
                }
            } else if (*parentp).ai_balance as libc::c_uint
                == CENT as libc::c_int as libc::c_uint
            {
                (*parentp).ai_balance = LEFT;
                break;
            } else {
                (*parentp).ai_balance = CENT;
            }
        } else {
            itemp = (*parentp).ai_right;
            if (*parentp).ai_balance as libc::c_uint
                == RIGHT as libc::c_int as libc::c_uint
            {
                if (*itemp).ai_balance as libc::c_uint
                    == RIGHT as libc::c_int as libc::c_uint
                {
                    _avl_rotate_rr(rootp, parentp);
                    (*itemp).ai_balance = CENT;
                    (*parentp).ai_balance = CENT;
                    parentp = itemp;
                } else if (*itemp).ai_balance as libc::c_uint
                    == CENT as libc::c_int as libc::c_uint
                {
                    _avl_rotate_rr(rootp, parentp);
                    (*itemp).ai_balance = LEFT;
                    (*parentp).ai_balance = RIGHT;
                    break;
                } else {
                    childp = (*itemp).ai_left;
                    _avl_rotate_rl(rootp, parentp);
                    (*itemp).ai_balance = CENT;
                    (*parentp).ai_balance = CENT;
                    if (*childp).ai_balance as libc::c_uint
                        == RIGHT as libc::c_int as libc::c_uint
                    {
                        (*parentp).ai_balance = LEFT;
                    }
                    if (*childp).ai_balance as libc::c_uint
                        == LEFT as libc::c_int as libc::c_uint
                    {
                        (*itemp).ai_balance = RIGHT;
                    }
                    (*childp).ai_balance = CENT;
                    parentp = childp;
                }
            } else if (*parentp).ai_balance as libc::c_uint
                == CENT as libc::c_int as libc::c_uint
            {
                (*parentp).ai_balance = RIGHT;
                break;
            } else {
                (*parentp).ai_balance = CENT;
            }
        }
        itemp = parentp;
        parentp = (*itemp).ai_up;
    }
}
unsafe extern "C" fn avl_insert_fix(
    mut rootp: *mut avl_root_t,
    mut itemp: *mut avl_item_t,
) {
    let mut childp: *mut avl_item_t = 0 as *mut avl_item_t;
    let mut parentp: *mut avl_item_t = (*itemp).ai_up;
    (*itemp).ai_right = 0 as *mut _avl_item_s;
    (*itemp).ai_left = (*itemp).ai_right;
    if (*itemp).ai_indexed == 0 {} else {
        __assert_fail(
            b"!itemp->ai_indexed\0" as *const u8 as *const libc::c_char,
            b"./avl.h\0" as *const u8 as *const libc::c_char,
            275 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"void avl_insert_fix(avl_root_t *, avl_item_t *)\0"))
                .as_ptr(),
        );
    }
    'c_15010: {
        if (*itemp).ai_indexed == 0 {} else {
            __assert_fail(
                b"!itemp->ai_indexed\0" as *const u8 as *const libc::c_char,
                b"./avl.h\0" as *const u8 as *const libc::c_char,
                275 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"void avl_insert_fix(avl_root_t *, avl_item_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*itemp).ai_indexed = 1 as libc::c_int;
    while !parentp.is_null() {
        if itemp == (*parentp).ai_left {
            if (*parentp).ai_balance as libc::c_uint
                == LEFT as libc::c_int as libc::c_uint
            {
                if (*itemp).ai_balance as libc::c_uint
                    == LEFT as libc::c_int as libc::c_uint
                {
                    _avl_rotate_ll(rootp, parentp);
                    (*itemp).ai_balance = CENT;
                    (*parentp).ai_balance = CENT;
                    break;
                } else {
                    if (*itemp).ai_balance as libc::c_uint
                        != CENT as libc::c_int as libc::c_uint
                    {} else {
                        __assert_fail(
                            b"itemp->ai_balance != CENT\0" as *const u8
                                as *const libc::c_char,
                            b"./avl.h\0" as *const u8 as *const libc::c_char,
                            290 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 48],
                                &[libc::c_char; 48],
                            >(b"void avl_insert_fix(avl_root_t *, avl_item_t *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_14917: {
                        if (*itemp).ai_balance as libc::c_uint
                            != CENT as libc::c_int as libc::c_uint
                        {} else {
                            __assert_fail(
                                b"itemp->ai_balance != CENT\0" as *const u8
                                    as *const libc::c_char,
                                b"./avl.h\0" as *const u8 as *const libc::c_char,
                                290 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 48],
                                    &[libc::c_char; 48],
                                >(b"void avl_insert_fix(avl_root_t *, avl_item_t *)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    childp = (*itemp).ai_right;
                    _avl_rotate_lr(rootp, parentp);
                    (*itemp).ai_balance = CENT;
                    (*parentp).ai_balance = CENT;
                    if (*childp).ai_balance as libc::c_uint
                        == RIGHT as libc::c_int as libc::c_uint
                    {
                        (*itemp).ai_balance = LEFT;
                    }
                    if (*childp).ai_balance as libc::c_uint
                        == LEFT as libc::c_int as libc::c_uint
                    {
                        (*parentp).ai_balance = RIGHT;
                    }
                    (*childp).ai_balance = CENT;
                    break;
                }
            } else if (*parentp).ai_balance as libc::c_uint
                == CENT as libc::c_int as libc::c_uint
            {
                (*parentp).ai_balance = LEFT;
            } else {
                (*parentp).ai_balance = CENT;
                return;
            }
        } else if (*parentp).ai_balance as libc::c_uint
            == RIGHT as libc::c_int as libc::c_uint
        {
            if (*itemp).ai_balance as libc::c_uint
                == RIGHT as libc::c_int as libc::c_uint
            {
                _avl_rotate_rr(rootp, parentp);
                (*itemp).ai_balance = CENT;
                (*parentp).ai_balance = CENT;
                break;
            } else {
                if (*itemp).ai_balance as libc::c_uint
                    != CENT as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"itemp->ai_balance != CENT\0" as *const u8
                            as *const libc::c_char,
                        b"./avl.h\0" as *const u8 as *const libc::c_char,
                        316 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 48],
                            &[libc::c_char; 48],
                        >(b"void avl_insert_fix(avl_root_t *, avl_item_t *)\0"))
                            .as_ptr(),
                    );
                }
                'c_14733: {
                    if (*itemp).ai_balance as libc::c_uint
                        != CENT as libc::c_int as libc::c_uint
                    {} else {
                        __assert_fail(
                            b"itemp->ai_balance != CENT\0" as *const u8
                                as *const libc::c_char,
                            b"./avl.h\0" as *const u8 as *const libc::c_char,
                            316 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 48],
                                &[libc::c_char; 48],
                            >(b"void avl_insert_fix(avl_root_t *, avl_item_t *)\0"))
                                .as_ptr(),
                        );
                    }
                };
                childp = (*itemp).ai_left;
                _avl_rotate_rl(rootp, parentp);
                (*itemp).ai_balance = CENT;
                (*parentp).ai_balance = CENT;
                if (*childp).ai_balance as libc::c_uint
                    == RIGHT as libc::c_int as libc::c_uint
                {
                    (*parentp).ai_balance = LEFT;
                }
                if (*childp).ai_balance as libc::c_uint
                    == LEFT as libc::c_int as libc::c_uint
                {
                    (*itemp).ai_balance = RIGHT;
                }
                (*childp).ai_balance = CENT;
                break;
            }
        } else if (*parentp).ai_balance as libc::c_uint
            == CENT as libc::c_int as libc::c_uint
        {
            (*parentp).ai_balance = RIGHT;
        } else {
            (*parentp).ai_balance = CENT;
            break;
        }
        itemp = parentp;
        parentp = (*itemp).ai_up;
    }
}
#[inline]
unsafe extern "C" fn avl_first(mut rootp: *mut avl_root_t) -> *mut avl_item_t {
    let mut itemp: *mut avl_item_t = (*rootp).ar_root;
    if !itemp.is_null() {
        while !((*itemp).ai_left).is_null() {
            itemp = (*itemp).ai_left;
        }
    }
    return itemp;
}
#[inline]
unsafe extern "C" fn avl_next(mut itemp: *mut avl_item_t) -> *mut avl_item_t {
    if !((*itemp).ai_right).is_null() {
        itemp = (*itemp).ai_right;
        while !((*itemp).ai_left).is_null() {
            itemp = (*itemp).ai_left;
        }
        return itemp;
    }
    while !((*itemp).ai_up).is_null() && itemp == (*(*itemp).ai_up).ai_right {
        itemp = (*itemp).ai_up;
    }
    if ((*itemp).ai_up).is_null() {
        return 0 as *mut avl_item_t;
    }
    return (*itemp).ai_up;
}
unsafe extern "C" fn avl_remove(mut rootp: *mut avl_root_t, mut itemp: *mut avl_item_t) {
    let mut relocp: *mut avl_item_t = 0 as *mut avl_item_t;
    let mut replacep: *mut avl_item_t = 0 as *mut avl_item_t;
    let mut parentp: *mut avl_item_t = 0 as *mut avl_item_t;
    if (*itemp).ai_indexed != 0 {} else {
        __assert_fail(
            b"itemp->ai_indexed\0" as *const u8 as *const libc::c_char,
            b"./avl.h\0" as *const u8 as *const libc::c_char,
            376 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void avl_remove(avl_root_t *, avl_item_t *)\0"))
                .as_ptr(),
        );
    }
    'c_12708: {
        if (*itemp).ai_indexed != 0 {} else {
            __assert_fail(
                b"itemp->ai_indexed\0" as *const u8 as *const libc::c_char,
                b"./avl.h\0" as *const u8 as *const libc::c_char,
                376 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"void avl_remove(avl_root_t *, avl_item_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*itemp).ai_indexed = 0 as libc::c_int;
    if ((*itemp).ai_left).is_null() || ((*itemp).ai_right).is_null() {
        parentp = (*itemp).ai_up;
        replacep = (*itemp).ai_left;
        if replacep.is_null() {
            replacep = (*itemp).ai_right;
        }
        if !replacep.is_null() {
            (*replacep).ai_up = parentp;
        }
        if parentp.is_null() {
            (*rootp).ar_root = replacep;
        } else {
            if itemp == (*parentp).ai_left {
                (*parentp).ai_left = replacep;
            } else {
                (*parentp).ai_right = replacep;
            }
            avl_delete_fix(rootp, replacep, parentp);
        }
        return;
    }
    relocp = avl_next(itemp);
    if !relocp.is_null() {} else {
        __assert_fail(
            b"relocp\0" as *const u8 as *const libc::c_char,
            b"./avl.h\0" as *const u8 as *const libc::c_char,
            405 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void avl_remove(avl_root_t *, avl_item_t *)\0"))
                .as_ptr(),
        );
    }
    'c_12547: {
        if !relocp.is_null() {} else {
            __assert_fail(
                b"relocp\0" as *const u8 as *const libc::c_char,
                b"./avl.h\0" as *const u8 as *const libc::c_char,
                405 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"void avl_remove(avl_root_t *, avl_item_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*relocp).ai_up).is_null() {} else {
        __assert_fail(
            b"relocp->ai_up != NULL\0" as *const u8 as *const libc::c_char,
            b"./avl.h\0" as *const u8 as *const libc::c_char,
            406 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void avl_remove(avl_root_t *, avl_item_t *)\0"))
                .as_ptr(),
        );
    }
    'c_12500: {
        if !((*relocp).ai_up).is_null() {} else {
            __assert_fail(
                b"relocp->ai_up != NULL\0" as *const u8 as *const libc::c_char,
                b"./avl.h\0" as *const u8 as *const libc::c_char,
                406 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"void avl_remove(avl_root_t *, avl_item_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if ((*relocp).ai_left).is_null() {} else {
        __assert_fail(
            b"relocp->ai_left == NULL\0" as *const u8 as *const libc::c_char,
            b"./avl.h\0" as *const u8 as *const libc::c_char,
            407 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void avl_remove(avl_root_t *, avl_item_t *)\0"))
                .as_ptr(),
        );
    }
    'c_12453: {
        if ((*relocp).ai_left).is_null() {} else {
            __assert_fail(
                b"relocp->ai_left == NULL\0" as *const u8 as *const libc::c_char,
                b"./avl.h\0" as *const u8 as *const libc::c_char,
                407 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"void avl_remove(avl_root_t *, avl_item_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    replacep = (*relocp).ai_right;
    (*relocp).ai_left = (*itemp).ai_left;
    if !((*relocp).ai_left).is_null() {
        (*(*relocp).ai_left).ai_up = relocp;
    }
    if ((*itemp).ai_up).is_null() {
        (*rootp).ar_root = relocp;
    } else if itemp == (*(*itemp).ai_up).ai_left {
        (*(*itemp).ai_up).ai_left = relocp;
    } else {
        (*(*itemp).ai_up).ai_right = relocp;
    }
    if relocp == (*(*relocp).ai_up).ai_left {
        if (*relocp).ai_up != itemp {} else {
            __assert_fail(
                b"relocp->ai_up != itemp\0" as *const u8 as *const libc::c_char,
                b"./avl.h\0" as *const u8 as *const libc::c_char,
                421 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"void avl_remove(avl_root_t *, avl_item_t *)\0"))
                    .as_ptr(),
            );
        }
        'c_12326: {
            if (*relocp).ai_up != itemp {} else {
                __assert_fail(
                    b"relocp->ai_up != itemp\0" as *const u8 as *const libc::c_char,
                    b"./avl.h\0" as *const u8 as *const libc::c_char,
                    421 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 44],
                        &[libc::c_char; 44],
                    >(b"void avl_remove(avl_root_t *, avl_item_t *)\0"))
                        .as_ptr(),
                );
            }
        };
        (*(*relocp).ai_up).ai_left = replacep;
        parentp = (*relocp).ai_up;
        if !replacep.is_null() {
            (*replacep).ai_up = (*relocp).ai_up;
        }
        (*relocp).ai_right = (*itemp).ai_right;
    } else {
        if (*relocp).ai_up == itemp {} else {
            __assert_fail(
                b"relocp->ai_up == itemp\0" as *const u8 as *const libc::c_char,
                b"./avl.h\0" as *const u8 as *const libc::c_char,
                428 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"void avl_remove(avl_root_t *, avl_item_t *)\0"))
                    .as_ptr(),
            );
        }
        'c_12239: {
            if (*relocp).ai_up == itemp {} else {
                __assert_fail(
                    b"relocp->ai_up == itemp\0" as *const u8 as *const libc::c_char,
                    b"./avl.h\0" as *const u8 as *const libc::c_char,
                    428 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 44],
                        &[libc::c_char; 44],
                    >(b"void avl_remove(avl_root_t *, avl_item_t *)\0"))
                        .as_ptr(),
                );
            }
        };
        (*relocp).ai_right = replacep;
        parentp = relocp;
    }
    if !((*relocp).ai_right).is_null() {
        (*(*relocp).ai_right).ai_up = relocp;
    }
    (*relocp).ai_up = (*itemp).ai_up;
    (*relocp).ai_balance = (*itemp).ai_balance;
    avl_delete_fix(rootp, replacep, parentp);
}
pub unsafe extern "C" fn vg_exec_context_new_key() -> *mut EC_KEY {
    return EC_KEY_new_by_curve_name(714 as libc::c_int);
}
static mut vg_thread_lock: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut vg_thread_rdcond: pthread_cond_t = pthread_cond_t {
    __data: {
        let mut init = __pthread_cond_s {
            c2rust_unnamed: C2RustUnnamed_1 {
                __wseq: 0 as libc::c_int as libc::c_ulonglong,
            },
            c2rust_unnamed_0: C2RustUnnamed {
                __g1_start: 0 as libc::c_int as libc::c_ulonglong,
            },
            __g_refs: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
            __g_size: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
            __g1_orig_size: 0 as libc::c_int as libc::c_uint,
            __wrefs: 0 as libc::c_int as libc::c_uint,
            __g_signals: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
        };
        init
    },
};
static mut vg_thread_wrcond: pthread_cond_t = pthread_cond_t {
    __data: {
        let mut init = __pthread_cond_s {
            c2rust_unnamed: C2RustUnnamed_1 {
                __wseq: 0 as libc::c_int as libc::c_ulonglong,
            },
            c2rust_unnamed_0: C2RustUnnamed {
                __g1_start: 0 as libc::c_int as libc::c_ulonglong,
            },
            __g_refs: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
            __g_size: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
            __g1_orig_size: 0 as libc::c_int as libc::c_uint,
            __wrefs: 0 as libc::c_int as libc::c_uint,
            __g_signals: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
        };
        init
    },
};
static mut vg_thread_upcond: pthread_cond_t = pthread_cond_t {
    __data: {
        let mut init = __pthread_cond_s {
            c2rust_unnamed: C2RustUnnamed_1 {
                __wseq: 0 as libc::c_int as libc::c_ulonglong,
            },
            c2rust_unnamed_0: C2RustUnnamed {
                __g1_start: 0 as libc::c_int as libc::c_ulonglong,
            },
            __g_refs: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
            __g_size: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
            __g1_orig_size: 0 as libc::c_int as libc::c_uint,
            __wrefs: 0 as libc::c_int as libc::c_uint,
            __g_signals: [
                0 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            ],
        };
        init
    },
};
unsafe extern "C" fn __vg_exec_context_yield(mut vxcp: *mut vg_exec_context_t) {
    (*vxcp).vxc_lockmode = 0 as libc::c_int;
    while (*(*vxcp).vxc_vc).vc_thread_excl != 0 {
        if (*vxcp).vxc_stop != 0 {
            if (*(*vxcp).vxc_vc).vc_thread_excl != 0 {} else {
                __assert_fail(
                    b"vxcp->vxc_vc->vc_thread_excl\0" as *const u8
                        as *const libc::c_char,
                    b"pattern.c\0" as *const u8 as *const libc::c_char,
                    64 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 50],
                        &[libc::c_char; 50],
                    >(b"void __vg_exec_context_yield(vg_exec_context_t *)\0"))
                        .as_ptr(),
                );
            }
            'c_21256: {
                if (*(*vxcp).vxc_vc).vc_thread_excl != 0 {} else {
                    __assert_fail(
                        b"vxcp->vxc_vc->vc_thread_excl\0" as *const u8
                            as *const libc::c_char,
                        b"pattern.c\0" as *const u8 as *const libc::c_char,
                        64 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 50],
                            &[libc::c_char; 50],
                        >(b"void __vg_exec_context_yield(vg_exec_context_t *)\0"))
                            .as_ptr(),
                    );
                }
            };
            (*vxcp).vxc_stop = 0 as libc::c_int;
            pthread_cond_signal(&mut vg_thread_upcond);
        }
        pthread_cond_wait(&mut vg_thread_rdcond, &mut vg_thread_lock);
    }
    if (*vxcp).vxc_stop == 0 {} else {
        __assert_fail(
            b"!vxcp->vxc_stop\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void __vg_exec_context_yield(vg_exec_context_t *)\0"))
                .as_ptr(),
        );
    }
    'c_21157: {
        if (*vxcp).vxc_stop == 0 {} else {
            __assert_fail(
                b"!vxcp->vxc_stop\0" as *const u8 as *const libc::c_char,
                b"pattern.c\0" as *const u8 as *const libc::c_char,
                70 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"void __vg_exec_context_yield(vg_exec_context_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*vxcp).vxc_lockmode == 0 {} else {
        __assert_fail(
            b"!vxcp->vxc_lockmode\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void __vg_exec_context_yield(vg_exec_context_t *)\0"))
                .as_ptr(),
        );
    }
    'c_21118: {
        if (*vxcp).vxc_lockmode == 0 {} else {
            __assert_fail(
                b"!vxcp->vxc_lockmode\0" as *const u8 as *const libc::c_char,
                b"pattern.c\0" as *const u8 as *const libc::c_char,
                71 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"void __vg_exec_context_yield(vg_exec_context_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*vxcp).vxc_lockmode = 1 as libc::c_int;
}
pub unsafe extern "C" fn vg_exec_context_upgrade_lock(
    mut vxcp: *mut vg_exec_context_t,
) -> libc::c_int {
    let mut tp: *mut vg_exec_context_t = 0 as *mut vg_exec_context_t;
    let mut vcp: *mut vg_context_t = 0 as *mut vg_context_t;
    if (*vxcp).vxc_lockmode == 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    pthread_mutex_lock(&mut vg_thread_lock);
    if (*vxcp).vxc_lockmode == 1 as libc::c_int {} else {
        __assert_fail(
            b"vxcp->vxc_lockmode == 1\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"int vg_exec_context_upgrade_lock(vg_exec_context_t *)\0"))
                .as_ptr(),
        );
    }
    'c_13567: {
        if (*vxcp).vxc_lockmode == 1 as libc::c_int {} else {
            __assert_fail(
                b"vxcp->vxc_lockmode == 1\0" as *const u8 as *const libc::c_char,
                b"pattern.c\0" as *const u8 as *const libc::c_char,
                86 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"int vg_exec_context_upgrade_lock(vg_exec_context_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*vxcp).vxc_lockmode = 0 as libc::c_int;
    vcp = (*vxcp).vxc_vc;
    let fresh0 = (*vcp).vc_thread_excl;
    (*vcp).vc_thread_excl = (*vcp).vc_thread_excl + 1;
    if fresh0 != 0 {
        if (*vxcp).vxc_stop != 0 {} else {
            __assert_fail(
                b"vxcp->vxc_stop\0" as *const u8 as *const libc::c_char,
                b"pattern.c\0" as *const u8 as *const libc::c_char,
                91 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"int vg_exec_context_upgrade_lock(vg_exec_context_t *)\0"))
                    .as_ptr(),
            );
        }
        'c_13516: {
            if (*vxcp).vxc_stop != 0 {} else {
                __assert_fail(
                    b"vxcp->vxc_stop\0" as *const u8 as *const libc::c_char,
                    b"pattern.c\0" as *const u8 as *const libc::c_char,
                    91 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 54],
                        &[libc::c_char; 54],
                    >(b"int vg_exec_context_upgrade_lock(vg_exec_context_t *)\0"))
                        .as_ptr(),
                );
            }
        };
        (*vxcp).vxc_stop = 0 as libc::c_int;
        pthread_cond_signal(&mut vg_thread_upcond);
        pthread_cond_wait(&mut vg_thread_wrcond, &mut vg_thread_lock);
        tp = (*vcp).vc_threads;
        while !tp.is_null() {
            if (*tp).vxc_lockmode == 0 {} else {
                __assert_fail(
                    b"!tp->vxc_lockmode\0" as *const u8 as *const libc::c_char,
                    b"pattern.c\0" as *const u8 as *const libc::c_char,
                    97 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 54],
                        &[libc::c_char; 54],
                    >(b"int vg_exec_context_upgrade_lock(vg_exec_context_t *)\0"))
                        .as_ptr(),
                );
            }
            'c_13408: {
                if (*tp).vxc_lockmode == 0 {} else {
                    __assert_fail(
                        b"!tp->vxc_lockmode\0" as *const u8 as *const libc::c_char,
                        b"pattern.c\0" as *const u8 as *const libc::c_char,
                        97 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 54],
                            &[libc::c_char; 54],
                        >(b"int vg_exec_context_upgrade_lock(vg_exec_context_t *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if (*tp).vxc_stop == 0 {} else {
                __assert_fail(
                    b"!tp->vxc_stop\0" as *const u8 as *const libc::c_char,
                    b"pattern.c\0" as *const u8 as *const libc::c_char,
                    98 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 54],
                        &[libc::c_char; 54],
                    >(b"int vg_exec_context_upgrade_lock(vg_exec_context_t *)\0"))
                        .as_ptr(),
                );
            }
            'c_13370: {
                if (*tp).vxc_stop == 0 {} else {
                    __assert_fail(
                        b"!tp->vxc_stop\0" as *const u8 as *const libc::c_char,
                        b"pattern.c\0" as *const u8 as *const libc::c_char,
                        98 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 54],
                            &[libc::c_char; 54],
                        >(b"int vg_exec_context_upgrade_lock(vg_exec_context_t *)\0"))
                            .as_ptr(),
                    );
                }
            };
            tp = (*tp).vxc_next;
        }
    } else {
        tp = (*vcp).vc_threads;
        while !tp.is_null() {
            if (*tp).vxc_lockmode != 0 {
                if (*tp).vxc_lockmode != 2 as libc::c_int {} else {
                    __assert_fail(
                        b"tp->vxc_lockmode != 2\0" as *const u8 as *const libc::c_char,
                        b"pattern.c\0" as *const u8 as *const libc::c_char,
                        104 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 54],
                            &[libc::c_char; 54],
                        >(b"int vg_exec_context_upgrade_lock(vg_exec_context_t *)\0"))
                            .as_ptr(),
                    );
                }
                'c_13299: {
                    if (*tp).vxc_lockmode != 2 as libc::c_int {} else {
                        __assert_fail(
                            b"tp->vxc_lockmode != 2\0" as *const u8
                                as *const libc::c_char,
                            b"pattern.c\0" as *const u8 as *const libc::c_char,
                            104 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 54],
                                &[libc::c_char; 54],
                            >(
                                b"int vg_exec_context_upgrade_lock(vg_exec_context_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                (*tp).vxc_stop = 1 as libc::c_int;
            }
            tp = (*tp).vxc_next;
        }
        loop {
            tp = (*vcp).vc_threads;
            while !tp.is_null() {
                if (*tp).vxc_lockmode != 0 {
                    if (*tp).vxc_lockmode != 2 as libc::c_int {} else {
                        __assert_fail(
                            b"tp->vxc_lockmode != 2\0" as *const u8
                                as *const libc::c_char,
                            b"pattern.c\0" as *const u8 as *const libc::c_char,
                            114 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 54],
                                &[libc::c_char; 54],
                            >(
                                b"int vg_exec_context_upgrade_lock(vg_exec_context_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_13222: {
                        if (*tp).vxc_lockmode != 2 as libc::c_int {} else {
                            __assert_fail(
                                b"tp->vxc_lockmode != 2\0" as *const u8
                                    as *const libc::c_char,
                                b"pattern.c\0" as *const u8 as *const libc::c_char,
                                114 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 54],
                                    &[libc::c_char; 54],
                                >(
                                    b"int vg_exec_context_upgrade_lock(vg_exec_context_t *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    pthread_cond_wait(&mut vg_thread_upcond, &mut vg_thread_lock);
                    break;
                } else {
                    tp = (*tp).vxc_next;
                }
            }
            if tp.is_null() {
                break;
            }
        }
    }
    (*vxcp).vxc_lockmode = 2 as libc::c_int;
    pthread_mutex_unlock(&mut vg_thread_lock);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn vg_exec_context_downgrade_lock(
    mut vxcp: *mut vg_exec_context_t,
) {
    pthread_mutex_lock(&mut vg_thread_lock);
    if (*vxcp).vxc_lockmode == 2 as libc::c_int {} else {
        __assert_fail(
            b"vxcp->vxc_lockmode == 2\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"void vg_exec_context_downgrade_lock(vg_exec_context_t *)\0"))
                .as_ptr(),
        );
    }
    'c_21823: {
        if (*vxcp).vxc_lockmode == 2 as libc::c_int {} else {
            __assert_fail(
                b"vxcp->vxc_lockmode == 2\0" as *const u8 as *const libc::c_char,
                b"pattern.c\0" as *const u8 as *const libc::c_char,
                132 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"void vg_exec_context_downgrade_lock(vg_exec_context_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*vxcp).vxc_stop == 0 {} else {
        __assert_fail(
            b"!vxcp->vxc_stop\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            133 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"void vg_exec_context_downgrade_lock(vg_exec_context_t *)\0"))
                .as_ptr(),
        );
    }
    'c_21784: {
        if (*vxcp).vxc_stop == 0 {} else {
            __assert_fail(
                b"!vxcp->vxc_stop\0" as *const u8 as *const libc::c_char,
                b"pattern.c\0" as *const u8 as *const libc::c_char,
                133 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"void vg_exec_context_downgrade_lock(vg_exec_context_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*(*vxcp).vxc_vc).vc_thread_excl -= 1;
    if (*(*vxcp).vxc_vc).vc_thread_excl == 0 {
        (*vxcp).vxc_lockmode = 1 as libc::c_int;
        pthread_cond_broadcast(&mut vg_thread_rdcond);
        pthread_mutex_unlock(&mut vg_thread_lock);
        return;
    }
    pthread_cond_signal(&mut vg_thread_wrcond);
    __vg_exec_context_yield(vxcp);
    pthread_mutex_unlock(&mut vg_thread_lock);
}
pub unsafe extern "C" fn vg_exec_context_init(
    mut vcp: *mut vg_context_t,
    mut vxcp: *mut vg_exec_context_t,
) -> libc::c_int {
    pthread_mutex_lock(&mut vg_thread_lock);
    memset(
        vxcp as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<vg_exec_context_t>() as libc::c_ulong,
    );
    (*vxcp).vxc_vc = vcp;
    BN_init(&mut (*vxcp).vxc_bntarg);
    BN_init(&mut (*vxcp).vxc_bnbase);
    BN_init(&mut (*vxcp).vxc_bntmp);
    BN_init(&mut (*vxcp).vxc_bntmp2);
    BN_set_word(&mut (*vxcp).vxc_bnbase, 58 as libc::c_int as libc::c_ulong);
    (*vxcp).vxc_bnctx = BN_CTX_new();
    if !((*vxcp).vxc_bnctx).is_null() {} else {
        __assert_fail(
            b"vxcp->vxc_bnctx\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"int vg_exec_context_init(vg_context_t *, vg_exec_context_t *)\0"))
                .as_ptr(),
        );
    }
    'c_21399: {
        if !((*vxcp).vxc_bnctx).is_null() {} else {
            __assert_fail(
                b"vxcp->vxc_bnctx\0" as *const u8 as *const libc::c_char,
                b"pattern.c\0" as *const u8 as *const libc::c_char,
                162 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"int vg_exec_context_init(vg_context_t *, vg_exec_context_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*vxcp).vxc_key = vg_exec_context_new_key();
    if !((*vxcp).vxc_key).is_null() {} else {
        __assert_fail(
            b"vxcp->vxc_key\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            164 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"int vg_exec_context_init(vg_context_t *, vg_exec_context_t *)\0"))
                .as_ptr(),
        );
    }
    'c_21346: {
        if !((*vxcp).vxc_key).is_null() {} else {
            __assert_fail(
                b"vxcp->vxc_key\0" as *const u8 as *const libc::c_char,
                b"pattern.c\0" as *const u8 as *const libc::c_char,
                164 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"int vg_exec_context_init(vg_context_t *, vg_exec_context_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    EC_KEY_precompute_mult((*vxcp).vxc_key, (*vxcp).vxc_bnctx);
    (*vxcp).vxc_lockmode = 0 as libc::c_int;
    (*vxcp).vxc_stop = 0 as libc::c_int;
    (*vxcp).vxc_next = (*vcp).vc_threads;
    (*vcp).vc_threads = vxcp;
    __vg_exec_context_yield(vxcp);
    pthread_mutex_unlock(&mut vg_thread_lock);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn vg_exec_context_del(mut vxcp: *mut vg_exec_context_t) {
    let mut tp: *mut vg_exec_context_t = 0 as *mut vg_exec_context_t;
    let mut pprev: *mut *mut vg_exec_context_t = 0 as *mut *mut vg_exec_context_t;
    if (*vxcp).vxc_lockmode == 2 as libc::c_int {
        vg_exec_context_downgrade_lock(vxcp);
    }
    pthread_mutex_lock(&mut vg_thread_lock);
    if (*vxcp).vxc_lockmode == 1 as libc::c_int {} else {
        __assert_fail(
            b"vxcp->vxc_lockmode == 1\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            186 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"void vg_exec_context_del(vg_exec_context_t *)\0"))
                .as_ptr(),
        );
    }
    'c_21680: {
        if (*vxcp).vxc_lockmode == 1 as libc::c_int {} else {
            __assert_fail(
                b"vxcp->vxc_lockmode == 1\0" as *const u8 as *const libc::c_char,
                b"pattern.c\0" as *const u8 as *const libc::c_char,
                186 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 46],
                    &[libc::c_char; 46],
                >(b"void vg_exec_context_del(vg_exec_context_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*vxcp).vxc_lockmode = 0 as libc::c_int;
    pprev = &mut (*(*vxcp).vxc_vc).vc_threads;
    tp = *pprev;
    while tp != vxcp && !tp.is_null() {
        pprev = &mut (*tp).vxc_next;
        tp = *pprev;
    }
    if tp == vxcp {} else {
        __assert_fail(
            b"tp == vxcp\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            193 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"void vg_exec_context_del(vg_exec_context_t *)\0"))
                .as_ptr(),
        );
    }
    'c_21590: {
        if tp == vxcp {} else {
            __assert_fail(
                b"tp == vxcp\0" as *const u8 as *const libc::c_char,
                b"pattern.c\0" as *const u8 as *const libc::c_char,
                193 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 46],
                    &[libc::c_char; 46],
                >(b"void vg_exec_context_del(vg_exec_context_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    *pprev = (*tp).vxc_next;
    if (*tp).vxc_stop != 0 {
        pthread_cond_signal(&mut vg_thread_upcond);
    }
    BN_clear_free(&mut (*vxcp).vxc_bntarg);
    BN_clear_free(&mut (*vxcp).vxc_bnbase);
    BN_clear_free(&mut (*vxcp).vxc_bntmp);
    BN_clear_free(&mut (*vxcp).vxc_bntmp2);
    BN_CTX_free((*vxcp).vxc_bnctx);
    (*vxcp).vxc_bnctx = 0 as *mut BN_CTX;
    pthread_mutex_unlock(&mut vg_thread_lock);
}
pub unsafe extern "C" fn vg_exec_context_yield(mut vxcp: *mut vg_exec_context_t) {
    if (*vxcp).vxc_lockmode == 2 as libc::c_int {
        vg_exec_context_downgrade_lock(vxcp);
    } else if (*vxcp).vxc_stop != 0 {
        if (*vxcp).vxc_lockmode == 1 as libc::c_int {} else {
            __assert_fail(
                b"vxcp->vxc_lockmode == 1\0" as *const u8 as *const libc::c_char,
                b"pattern.c\0" as *const u8 as *const libc::c_char,
                215 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"void vg_exec_context_yield(vg_exec_context_t *)\0"))
                    .as_ptr(),
            );
        }
        'c_22097: {
            if (*vxcp).vxc_lockmode == 1 as libc::c_int {} else {
                __assert_fail(
                    b"vxcp->vxc_lockmode == 1\0" as *const u8 as *const libc::c_char,
                    b"pattern.c\0" as *const u8 as *const libc::c_char,
                    215 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 48],
                        &[libc::c_char; 48],
                    >(b"void vg_exec_context_yield(vg_exec_context_t *)\0"))
                        .as_ptr(),
                );
            }
        };
        pthread_mutex_lock(&mut vg_thread_lock);
        __vg_exec_context_yield(vxcp);
        pthread_mutex_unlock(&mut vg_thread_lock);
    }
    if (*vxcp).vxc_lockmode == 1 as libc::c_int {} else {
        __assert_fail(
            b"vxcp->vxc_lockmode == 1\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"void vg_exec_context_yield(vg_exec_context_t *)\0"))
                .as_ptr(),
        );
    }
    'c_22033: {
        if (*vxcp).vxc_lockmode == 1 as libc::c_int {} else {
            __assert_fail(
                b"vxcp->vxc_lockmode == 1\0" as *const u8 as *const libc::c_char,
                b"pattern.c\0" as *const u8 as *const libc::c_char,
                221 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"void vg_exec_context_yield(vg_exec_context_t *)\0"))
                    .as_ptr(),
            );
        }
    };
}
pub unsafe extern "C" fn vg_exec_context_consolidate_key(
    mut vxcp: *mut vg_exec_context_t,
) {
    if (*vxcp).vxc_delta != 0 {
        BN_clear(&mut (*vxcp).vxc_bntmp);
        BN_set_word(&mut (*vxcp).vxc_bntmp, (*vxcp).vxc_delta as libc::c_ulong);
        BN_add(
            &mut (*vxcp).vxc_bntmp2,
            EC_KEY_get0_private_key((*vxcp).vxc_key),
            &mut (*vxcp).vxc_bntmp,
        );
        vg_set_privkey(&mut (*vxcp).vxc_bntmp2, (*vxcp).vxc_key);
        (*vxcp).vxc_delta = 0 as libc::c_int;
    }
}
pub unsafe extern "C" fn vg_exec_context_calc_address(mut vxcp: *mut vg_exec_context_t) {
    let mut pubkey: *mut EC_POINT = 0 as *mut EC_POINT;
    let mut pgroup: *const EC_GROUP = 0 as *const EC_GROUP;
    let mut eckey_buf: [libc::c_uchar; 96] = [0; 96];
    let mut hash1: [libc::c_uchar; 32] = [0; 32];
    let mut hash2: [libc::c_uchar; 20] = [0; 20];
    let mut len: libc::c_int = 0;
    vg_exec_context_consolidate_key(vxcp);
    pgroup = EC_KEY_get0_group((*vxcp).vxc_key);
    pubkey = EC_POINT_new(pgroup);
    EC_POINT_copy(pubkey, EC_KEY_get0_public_key((*vxcp).vxc_key));
    if !((*(*vxcp).vxc_vc).vc_pubkey_base).is_null() {
        EC_POINT_add(
            pgroup,
            pubkey,
            pubkey,
            (*(*vxcp).vxc_vc).vc_pubkey_base,
            (*vxcp).vxc_bnctx,
        );
    }
    len = EC_POINT_point2oct(
        pgroup,
        pubkey,
        POINT_CONVERSION_UNCOMPRESSED,
        eckey_buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_uchar; 96]>() as libc::c_ulong,
        (*vxcp).vxc_bnctx,
    ) as libc::c_int;
    SHA256(eckey_buf.as_mut_ptr(), len as size_t, hash1.as_mut_ptr());
    RIPEMD160(
        hash1.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong,
        hash2.as_mut_ptr(),
    );
    memcpy(
        &mut *((*vxcp).vxc_binres).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut libc::c_uchar as *mut libc::c_void,
        hash2.as_mut_ptr() as *const libc::c_void,
        20 as libc::c_int as libc::c_ulong,
    );
    EC_POINT_free(pubkey);
}
static mut timing_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
pub unsafe extern "C" fn vg_output_timing(
    mut vcp: *mut vg_context_t,
    mut cycle: libc::c_int,
    mut last: *mut timeval,
) -> libc::c_int {
    let mut me: pthread_t = 0;
    let mut tvnow: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tip: *mut timing_info_t = 0 as *mut timing_info_t;
    let mut mytip: *mut timing_info_t = 0 as *mut timing_info_t;
    let mut rate: libc::c_ulonglong = 0;
    let mut myrate: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    let mut mytime: libc::c_ulonglong = 0;
    let mut total: libc::c_ulonglong = 0;
    let mut sincelast: libc::c_ulonglong = 0;
    let mut p: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    gettimeofday(&mut tvnow, 0 as *mut libc::c_void);
    tv.tv_sec = tvnow.tv_sec - (*last).tv_sec;
    tv.tv_usec = tvnow.tv_usec - (*last).tv_usec;
    if tv.tv_usec < 0 as libc::c_int as libc::c_long {
        tv.tv_sec -= 1;
        tv.tv_sec;
        tv.tv_usec += 1000000 as libc::c_int as libc::c_long;
    }
    memcpy(
        last as *mut libc::c_void,
        &mut tvnow as *mut timeval as *const libc::c_void,
        ::std::mem::size_of::<timeval>() as libc::c_ulong,
    );
    mytime = (tv.tv_usec as libc::c_ulonglong)
        .wrapping_add(
            (1000000 as libc::c_ulonglong).wrapping_mul(tv.tv_sec as libc::c_ulonglong),
        );
    if mytime == 0 {
        mytime = 1 as libc::c_int as libc::c_ulonglong;
    }
    rate = 0 as libc::c_int as libc::c_ulonglong;
    pthread_mutex_lock(&mut timing_mutex);
    me = pthread_self();
    tip = (*vcp).vc_timing_head;
    mytip = 0 as *mut timing_info_t;
    while !tip.is_null() {
        if pthread_equal((*tip).ti_thread, me) != 0 {
            mytip = tip;
            p = ((*tip).ti_hist_last + 1 as libc::c_int)
                % timing_hist_size as libc::c_int;
            (*tip).ti_hist_time[p as usize] = mytime;
            (*tip).ti_hist_work[p as usize] = cycle as libc::c_ulong;
            (*tip).ti_hist_last = p;
            mytime = 0 as libc::c_int as libc::c_ulonglong;
            myrate = 0 as libc::c_int as libc::c_ulonglong;
            i = 0 as libc::c_int;
            while i < timing_hist_size as libc::c_int {
                mytime = mytime.wrapping_add((*tip).ti_hist_time[i as usize]);
                myrate = myrate
                    .wrapping_add((*tip).ti_hist_work[i as usize] as libc::c_ulonglong);
                i += 1;
                i;
            }
            myrate = myrate
                .wrapping_mul(1000000 as libc::c_int as libc::c_ulonglong)
                .wrapping_div(mytime);
            (*tip).ti_last_rate = myrate as libc::c_ulong;
            rate = rate.wrapping_add(myrate);
        } else {
            rate = rate.wrapping_add((*tip).ti_last_rate as libc::c_ulonglong);
        }
        tip = (*tip).ti_next;
    }
    if mytip.is_null() {
        mytip = malloc(::std::mem::size_of::<timing_info_t>() as libc::c_ulong)
            as *mut timing_info_t;
        (*mytip).ti_next = (*vcp).vc_timing_head;
        (*mytip).ti_thread = me;
        (*vcp).vc_timing_head = mytip;
        (*mytip).ti_hist_last = 0 as libc::c_int;
        (*mytip).ti_hist_time[0 as libc::c_int as usize] = mytime;
        (*mytip).ti_hist_work[0 as libc::c_int as usize] = cycle as libc::c_ulong;
        i = 1 as libc::c_int;
        while i < timing_hist_size as libc::c_int {
            (*mytip).ti_hist_time[i as usize] = 1 as libc::c_int as libc::c_ulonglong;
            (*mytip).ti_hist_work[i as usize] = 0 as libc::c_int as libc::c_ulong;
            i += 1;
            i;
        }
        myrate = (cycle as libc::c_ulonglong)
            .wrapping_mul(1000000 as libc::c_int as libc::c_ulonglong)
            .wrapping_div(mytime);
        (*mytip).ti_last_rate = myrate as libc::c_ulong;
        rate = rate.wrapping_add(myrate);
    }
    (*vcp)
        .vc_timing_total = ((*vcp).vc_timing_total)
        .wrapping_add(cycle as libc::c_ulonglong);
    if (*vcp).vc_timing_prevfound != (*vcp).vc_found {
        (*vcp).vc_timing_prevfound = (*vcp).vc_found;
        (*vcp).vc_timing_sincelast = 0 as libc::c_int as libc::c_ulonglong;
    }
    (*vcp)
        .vc_timing_sincelast = ((*vcp).vc_timing_sincelast)
        .wrapping_add(cycle as libc::c_ulonglong);
    if mytip != (*vcp).vc_timing_head {
        pthread_mutex_unlock(&mut timing_mutex);
        return myrate as libc::c_int;
    }
    total = (*vcp).vc_timing_total;
    sincelast = (*vcp).vc_timing_sincelast;
    pthread_mutex_unlock(&mut timing_mutex);
    ((*vcp).vc_output_timing).unwrap()(vcp, sincelast as libc::c_double, rate, total);
    return myrate as libc::c_int;
}
pub unsafe extern "C" fn vg_context_thread_exit(mut vcp: *mut vg_context_t) {
    let mut tip: *mut timing_info_t = 0 as *mut timing_info_t;
    let mut ptip: *mut *mut timing_info_t = 0 as *mut *mut timing_info_t;
    let mut me: pthread_t = 0;
    pthread_mutex_lock(&mut timing_mutex);
    me = pthread_self();
    ptip = &mut (*vcp).vc_timing_head;
    tip = *ptip;
    while !tip.is_null() {
        if pthread_equal((*tip).ti_thread, me) == 0 {
            ptip = &mut (*tip).ti_next;
            tip = *ptip;
        } else {
            *ptip = (*tip).ti_next;
            free(tip as *mut libc::c_void);
            break;
        }
    }
    pthread_mutex_unlock(&mut timing_mutex);
}
unsafe extern "C" fn vg_timing_info_free(mut vcp: *mut vg_context_t) {
    let mut tp: *mut timing_info_t = 0 as *mut timing_info_t;
    while !((*vcp).vc_timing_head).is_null() {
        tp = (*vcp).vc_timing_head;
        (*vcp).vc_timing_head = (*tp).ti_next;
        free(tp as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn vg_output_timing_console(
    mut vcp: *mut vg_context_t,
    mut count: libc::c_double,
    mut rate: libc::c_ulonglong,
    mut total: libc::c_ulonglong,
) {
    let mut prob: libc::c_double = 0.;
    let mut time: libc::c_double = 0.;
    let mut targ: libc::c_double = 0.;
    let mut unit: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut linebuf: [libc::c_char; 80] = [0; 80];
    let mut rem: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let targs: [libc::c_double; 6] = [0.5f64, 0.75f64, 0.8f64, 0.9f64, 0.95f64, 1.0f64];
    targ = rate as libc::c_double;
    unit = b"key/s\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if targ > 1000 as libc::c_int as libc::c_double {
        unit = b"Kkey/s\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        targ /= 1000.0f64;
        if targ > 1000 as libc::c_int as libc::c_double {
            unit = b"Mkey/s\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            targ /= 1000.0f64;
        }
    }
    rem = ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong as libc::c_int;
    p = snprintf(
        linebuf.as_mut_ptr(),
        rem as libc::c_ulong,
        b"[%.2f %s][total %lld]\0" as *const u8 as *const libc::c_char,
        targ,
        unit,
        total,
    );
    if p > 0 as libc::c_int {} else {
        __assert_fail(
            b"p > 0\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 94],
                &[libc::c_char; 94],
            >(
                b"void vg_output_timing_console(vg_context_t *, double, unsigned long long, unsigned long long)\0",
            ))
                .as_ptr(),
        );
    }
    'c_20850: {
        if p > 0 as libc::c_int {} else {
            __assert_fail(
                b"p > 0\0" as *const u8 as *const libc::c_char,
                b"pattern.c\0" as *const u8 as *const libc::c_char,
                421 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 94],
                    &[libc::c_char; 94],
                >(
                    b"void vg_output_timing_console(vg_context_t *, double, unsigned long long, unsigned long long)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    rem -= p;
    if rem < 0 as libc::c_int {
        rem = 0 as libc::c_int;
    }
    if (*vcp).vc_chance >= 1.0f64 {
        prob = 1.0f32 as libc::c_double - exp(-count / (*vcp).vc_chance);
        if prob <= 0.999f64 {
            p = snprintf(
                &mut *linebuf.as_mut_ptr().offset(p as isize) as *mut libc::c_char,
                rem as libc::c_ulong,
                b"[Prob %.1f%%]\0" as *const u8 as *const libc::c_char,
                prob * 100 as libc::c_int as libc::c_double,
            );
            if p > 0 as libc::c_int {} else {
                __assert_fail(
                    b"p > 0\0" as *const u8 as *const libc::c_char,
                    b"pattern.c\0" as *const u8 as *const libc::c_char,
                    432 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 94],
                        &[libc::c_char; 94],
                    >(
                        b"void vg_output_timing_console(vg_context_t *, double, unsigned long long, unsigned long long)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_20757: {
                if p > 0 as libc::c_int {} else {
                    __assert_fail(
                        b"p > 0\0" as *const u8 as *const libc::c_char,
                        b"pattern.c\0" as *const u8 as *const libc::c_char,
                        432 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 94],
                            &[libc::c_char; 94],
                        >(
                            b"void vg_output_timing_console(vg_context_t *, double, unsigned long long, unsigned long long)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            rem -= p;
            if rem < 0 as libc::c_int {
                rem = 0 as libc::c_int;
            }
            p = (::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong)
                .wrapping_sub(rem as libc::c_ulong) as libc::c_int;
        }
        i = 0 as libc::c_int;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<[libc::c_double; 6]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        {
            targ = targs[i as usize];
            if targ < 1.0f64 && prob <= targ {
                break;
            }
            i += 1;
            i;
        }
        if targ < 1.0f64 {
            time = (-(*vcp).vc_chance * log(1.0f64 - targ) - count)
                / rate as libc::c_double;
            unit = b"s\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            if time > 60 as libc::c_int as libc::c_double {
                time /= 60 as libc::c_int as libc::c_double;
                unit = b"min\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                if time > 60 as libc::c_int as libc::c_double {
                    time /= 60 as libc::c_int as libc::c_double;
                    unit = b"h\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    if time > 24 as libc::c_int as libc::c_double {
                        time /= 24 as libc::c_int as libc::c_double;
                        unit = b"d\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                        if time > 365 as libc::c_int as libc::c_double {
                            time /= 365 as libc::c_int as libc::c_double;
                            unit = b"y\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                        }
                    }
                }
            }
            if time > 1000000 as libc::c_int as libc::c_double {
                p = snprintf(
                    &mut *linebuf.as_mut_ptr().offset(p as isize) as *mut libc::c_char,
                    rem as libc::c_ulong,
                    b"[%d%% in %e%s]\0" as *const u8 as *const libc::c_char,
                    (100 as libc::c_int as libc::c_double * targ) as libc::c_int,
                    time,
                    unit,
                );
            } else {
                p = snprintf(
                    &mut *linebuf.as_mut_ptr().offset(p as isize) as *mut libc::c_char,
                    rem as libc::c_ulong,
                    b"[%d%% in %.1f%s]\0" as *const u8 as *const libc::c_char,
                    (100 as libc::c_int as libc::c_double * targ) as libc::c_int,
                    time,
                    unit,
                );
            }
            if p > 0 as libc::c_int {} else {
                __assert_fail(
                    b"p > 0\0" as *const u8 as *const libc::c_char,
                    b"pattern.c\0" as *const u8 as *const libc::c_char,
                    475 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 94],
                        &[libc::c_char; 94],
                    >(
                        b"void vg_output_timing_console(vg_context_t *, double, unsigned long long, unsigned long long)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_20465: {
                if p > 0 as libc::c_int {} else {
                    __assert_fail(
                        b"p > 0\0" as *const u8 as *const libc::c_char,
                        b"pattern.c\0" as *const u8 as *const libc::c_char,
                        475 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 94],
                            &[libc::c_char; 94],
                        >(
                            b"void vg_output_timing_console(vg_context_t *, double, unsigned long long, unsigned long long)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            rem -= p;
            if rem < 0 as libc::c_int {
                rem = 0 as libc::c_int;
            }
            p = (::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong)
                .wrapping_sub(rem as libc::c_ulong) as libc::c_int;
        }
    }
    if (*vcp).vc_found != 0 {
        if (*vcp).vc_remove_on_match != 0 {
            p = snprintf(
                &mut *linebuf.as_mut_ptr().offset(p as isize) as *mut libc::c_char,
                rem as libc::c_ulong,
                b"[Found %lld/%ld]\0" as *const u8 as *const libc::c_char,
                (*vcp).vc_found,
                (*vcp).vc_npatterns_start,
            );
        } else {
            p = snprintf(
                &mut *linebuf.as_mut_ptr().offset(p as isize) as *mut libc::c_char,
                rem as libc::c_ulong,
                b"[Found %lld]\0" as *const u8 as *const libc::c_char,
                (*vcp).vc_found,
            );
        }
        if p > 0 as libc::c_int {} else {
            __assert_fail(
                b"p > 0\0" as *const u8 as *const libc::c_char,
                b"pattern.c\0" as *const u8 as *const libc::c_char,
                490 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 94],
                    &[libc::c_char; 94],
                >(
                    b"void vg_output_timing_console(vg_context_t *, double, unsigned long long, unsigned long long)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_20335: {
            if p > 0 as libc::c_int {} else {
                __assert_fail(
                    b"p > 0\0" as *const u8 as *const libc::c_char,
                    b"pattern.c\0" as *const u8 as *const libc::c_char,
                    490 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 94],
                        &[libc::c_char; 94],
                    >(
                        b"void vg_output_timing_console(vg_context_t *, double, unsigned long long, unsigned long long)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        rem -= p;
        if rem < 0 as libc::c_int {
            rem = 0 as libc::c_int;
        }
    }
    if rem != 0 {
        memset(
            &mut *linebuf
                .as_mut_ptr()
                .offset(
                    (::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong)
                        .wrapping_sub(rem as libc::c_ulong) as isize,
                ) as *mut libc::c_char as *mut libc::c_void,
            0x20 as libc::c_int,
            rem as libc::c_ulong,
        );
        linebuf[(::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] = '\0' as i32 as libc::c_char;
    }
    printf(b"\r%s\0" as *const u8 as *const libc::c_char, linebuf.as_mut_ptr());
    fflush(stdout);
}
pub unsafe extern "C" fn vg_output_match_console(
    mut vcp: *mut vg_context_t,
    mut pkey: *mut EC_KEY,
    mut pattern: *const libc::c_char,
) {
    let mut key_buf: [libc::c_uchar; 512] = [0; 512];
    let mut pend: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut addr_buf: [libc::c_char; 64] = [0; 64];
    let mut addr2_buf: [libc::c_char; 64] = [0; 64];
    let mut privkey_buf: [libc::c_char; 128] = [0; 128];
    let mut keytype: *const libc::c_char = b"Privkey\0" as *const u8
        as *const libc::c_char;
    let mut len: libc::c_int = 0;
    let mut isscript: libc::c_int = ((*vcp).vc_format as libc::c_uint
        == VCF_SCRIPT as libc::c_int as libc::c_uint) as libc::c_int;
    let mut ppnt: *mut EC_POINT = 0 as *mut EC_POINT;
    let mut free_ppnt: libc::c_int = 0 as libc::c_int;
    if !((*vcp).vc_pubkey_base).is_null() {
        ppnt = EC_POINT_new(EC_KEY_get0_group(pkey));
        EC_POINT_copy(ppnt, EC_KEY_get0_public_key(pkey));
        EC_POINT_add(
            EC_KEY_get0_group(pkey),
            ppnt,
            ppnt,
            (*vcp).vc_pubkey_base,
            0 as *mut BN_CTX,
        );
        free_ppnt = 1 as libc::c_int;
        keytype = b"PrivkeyPart\0" as *const u8 as *const libc::c_char;
    } else {
        ppnt = EC_KEY_get0_public_key(pkey) as *mut EC_POINT;
    }
    if EC_KEY_check_key(pkey) != 0 {} else {
        __assert_fail(
            b"EC_KEY_check_key(pkey)\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            530 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 69],
                &[libc::c_char; 69],
            >(b"void vg_output_match_console(vg_context_t *, EC_KEY *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_20122: {
        if EC_KEY_check_key(pkey) != 0 {} else {
            __assert_fail(
                b"EC_KEY_check_key(pkey)\0" as *const u8 as *const libc::c_char,
                b"pattern.c\0" as *const u8 as *const libc::c_char,
                530 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 69],
                    &[libc::c_char; 69],
                >(
                    b"void vg_output_match_console(vg_context_t *, EC_KEY *, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    vg_encode_address(
        ppnt,
        EC_KEY_get0_group(pkey),
        (*vcp).vc_pubkeytype,
        addr_buf.as_mut_ptr(),
    );
    if isscript != 0 {
        vg_encode_script_address(
            ppnt,
            EC_KEY_get0_group(pkey),
            (*vcp).vc_addrtype,
            addr2_buf.as_mut_ptr(),
        );
    }
    if !((*vcp).vc_key_protect_pass).is_null() {
        len = vg_protect_encode_privkey(
            privkey_buf.as_mut_ptr(),
            pkey,
            (*vcp).vc_privtype,
            VG_PROTKEY_DEFAULT as libc::c_int,
            (*vcp).vc_key_protect_pass,
        );
        if len != 0 {
            keytype = b"Protkey\0" as *const u8 as *const libc::c_char;
        } else {
            fprintf(
                stderr,
                b"ERROR: could not password-protect key\n\0" as *const u8
                    as *const libc::c_char,
            );
            (*vcp).vc_key_protect_pass = 0 as *const libc::c_char;
        }
    }
    if ((*vcp).vc_key_protect_pass).is_null() {
        vg_encode_privkey(pkey, (*vcp).vc_privtype, privkey_buf.as_mut_ptr());
    }
    if ((*vcp).vc_result_file).is_null() || (*vcp).vc_verbose > 0 as libc::c_int {
        printf(
            b"\r%79s\rPattern: %s\n\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            pattern,
        );
    }
    if (*vcp).vc_verbose > 0 as libc::c_int {
        if (*vcp).vc_verbose > 1 as libc::c_int {
            pend = key_buf.as_mut_ptr();
            len = i2o_ECPublicKey(pkey, &mut pend);
            printf(b"Pubkey (hex): \0" as *const u8 as *const libc::c_char);
            dumphex(key_buf.as_mut_ptr(), len as size_t);
            printf(b"Privkey (hex): \0" as *const u8 as *const libc::c_char);
            dumpbn(EC_KEY_get0_private_key(pkey));
            pend = key_buf.as_mut_ptr();
            len = i2d_ECPrivateKey(pkey, &mut pend);
            printf(b"Privkey (ASN1): \0" as *const u8 as *const libc::c_char);
            dumphex(key_buf.as_mut_ptr(), len as size_t);
        }
    }
    if ((*vcp).vc_result_file).is_null() || (*vcp).vc_verbose > 0 as libc::c_int {
        if isscript != 0 {
            printf(
                b"P2SHAddress: %s\n\0" as *const u8 as *const libc::c_char,
                addr2_buf.as_mut_ptr(),
            );
        }
        printf(
            b"Address: %s\n%s: %s\n\0" as *const u8 as *const libc::c_char,
            addr_buf.as_mut_ptr(),
            keytype,
            privkey_buf.as_mut_ptr(),
        );
    }
    if !((*vcp).vc_result_file).is_null() {
        let mut fp: *mut FILE = fopen(
            (*vcp).vc_result_file,
            b"a\0" as *const u8 as *const libc::c_char,
        );
        if fp.is_null() {
            fprintf(
                stderr,
                b"ERROR: could not open result file: %s\n\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
        } else {
            fprintf(fp, b"Pattern: %s\n\0" as *const u8 as *const libc::c_char, pattern);
            if isscript != 0 {
                fprintf(
                    fp,
                    b"P2SHAddress: %s\n\0" as *const u8 as *const libc::c_char,
                    addr2_buf.as_mut_ptr(),
                );
            }
            fprintf(
                fp,
                b"Address: %s\n%s: %s\n\0" as *const u8 as *const libc::c_char,
                addr_buf.as_mut_ptr(),
                keytype,
                privkey_buf.as_mut_ptr(),
            );
            fclose(fp);
        }
    }
    if free_ppnt != 0 {
        EC_POINT_free(ppnt);
    }
}
pub unsafe extern "C" fn vg_context_free(mut vcp: *mut vg_context_t) {
    vg_timing_info_free(vcp);
    ((*vcp).vc_free).unwrap()(vcp);
}
pub unsafe extern "C" fn vg_context_add_patterns(
    mut vcp: *mut vg_context_t,
    patterns: *mut *const libc::c_char,
    mut npatterns: libc::c_int,
) -> libc::c_int {
    (*vcp).vc_pattern_generation += 1;
    (*vcp).vc_pattern_generation;
    return ((*vcp).vc_add_patterns).unwrap()(vcp, patterns, npatterns);
}
pub unsafe extern "C" fn vg_context_clear_all_patterns(mut vcp: *mut vg_context_t) {
    ((*vcp).vc_clear_all_patterns).unwrap()(vcp);
    (*vcp).vc_pattern_generation += 1;
    (*vcp).vc_pattern_generation;
}
pub unsafe extern "C" fn vg_context_hash160_sort(
    mut vcp: *mut vg_context_t,
    mut buf: *mut libc::c_void,
) -> libc::c_int {
    if ((*vcp).vc_hash160_sort).is_none() {
        return 0 as libc::c_int;
    }
    return ((*vcp).vc_hash160_sort).unwrap()(vcp, buf);
}
pub unsafe extern "C" fn vg_context_start_threads(
    mut vcp: *mut vg_context_t,
) -> libc::c_int {
    let mut vxcp: *mut vg_exec_context_t = 0 as *mut vg_exec_context_t;
    let mut res: libc::c_int = 0;
    vxcp = (*vcp).vc_threads;
    while !vxcp.is_null() {
        res = pthread_create(
            &mut (*vxcp).vxc_pthread as *mut pthread_t,
            0 as *const pthread_attr_t,
            ::std::mem::transmute::<
                vg_exec_context_threadfunc_t,
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
            >((*vxcp).vxc_threadfunc),
            vxcp as *mut libc::c_void,
        );
        if res != 0 {
            fprintf(
                stderr,
                b"ERROR: could not create thread: %d\n\0" as *const u8
                    as *const libc::c_char,
                res,
            );
            vg_context_stop_threads(vcp);
            return -(1 as libc::c_int);
        }
        (*vxcp).vxc_thread_active = 1 as libc::c_int;
        vxcp = (*vxcp).vxc_next;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn vg_context_stop_threads(mut vcp: *mut vg_context_t) {
    (*vcp).vc_halt = 1 as libc::c_int;
    vg_context_wait_for_completion(vcp);
    (*vcp).vc_halt = 0 as libc::c_int;
}
pub unsafe extern "C" fn vg_context_wait_for_completion(mut vcp: *mut vg_context_t) {
    let mut vxcp: *mut vg_exec_context_t = 0 as *mut vg_exec_context_t;
    vxcp = (*vcp).vc_threads;
    while !vxcp.is_null() {
        if !((*vxcp).vxc_thread_active == 0) {
            pthread_join((*vxcp).vxc_pthread, 0 as *mut *mut libc::c_void);
            (*vxcp).vxc_thread_active = 0 as libc::c_int;
        }
        vxcp = (*vxcp).vxc_next;
    }
}
unsafe extern "C" fn get_prefix_ranges(
    mut addrtype: libc::c_int,
    mut pfx: *const libc::c_char,
    mut result: *mut *mut BIGNUM,
    mut bnctx: *mut BN_CTX,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut zero_prefix: libc::c_int = 0 as libc::c_int;
    let mut check_upper: libc::c_int = 0 as libc::c_int;
    let mut b58pow: libc::c_int = 0;
    let mut b58ceil: libc::c_int = 0;
    let mut b58top: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut bntarg: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bnceil: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bnfloor: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bnbase: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bnap: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bnbp: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bntp: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bnhigh: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bnlow: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bnhigh2: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bnlow2: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bntmp: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bntmp2: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    BN_init(&mut bntarg);
    BN_init(&mut bnceil);
    BN_init(&mut bnfloor);
    BN_init(&mut bnbase);
    BN_init(&mut bntmp);
    BN_init(&mut bntmp2);
    BN_set_word(&mut bnbase, 58 as libc::c_int as libc::c_ulong);
    p = strlen(pfx) as libc::c_int;
    i = 0 as libc::c_int;
    loop {
        if !(i < p) {
            current_block = 5634871135123216486;
            break;
        }
        c = vg_b58_reverse_map[*pfx.offset(i as isize) as libc::c_int as usize]
            as libc::c_int;
        if c == -(1 as libc::c_int) {
            fprintf(
                stderr,
                b"Invalid character '%c' in prefix '%s'\n\0" as *const u8
                    as *const libc::c_char,
                *pfx.offset(i as isize) as libc::c_int,
                pfx,
            );
            current_block = 16307290864065614123;
            break;
        } else {
            if i == zero_prefix {
                if c == 0 as libc::c_int {
                    zero_prefix += 1;
                    zero_prefix;
                    if zero_prefix > 19 as libc::c_int {
                        fprintf(
                            stderr,
                            b"Prefix '%s' is too long\n\0" as *const u8
                                as *const libc::c_char,
                            pfx,
                        );
                        current_block = 16307290864065614123;
                        break;
                    }
                } else {
                    b58top = c;
                    BN_set_word(&mut bntarg, c as libc::c_ulong);
                }
            } else {
                BN_set_word(&mut bntmp2, c as libc::c_ulong);
                BN_mul(&mut bntmp, &mut bntarg, &mut bnbase, bnctx);
                BN_add(&mut bntarg, &mut bntmp, &mut bntmp2);
            }
            i += 1;
            i;
        }
    }
    match current_block {
        5634871135123216486 => {
            BN_clear(&mut bntmp);
            BN_set_bit(&mut bntmp, 200 as libc::c_int - zero_prefix * 8 as libc::c_int);
            BN_sub(&mut bnceil, &mut bntmp, BN_value_one());
            BN_set_bit(
                &mut bnfloor,
                192 as libc::c_int - zero_prefix * 8 as libc::c_int,
            );
            bnlow = BN_new();
            bnhigh = BN_new();
            if b58top != 0 {
                BN_copy(&mut bntmp, &mut bnceil);
                bnap = &mut bntmp;
                bnbp = &mut bntmp2;
                b58pow = 0 as libc::c_int;
                while BN_cmp(bnap, &mut bnbase) > 0 as libc::c_int {
                    b58pow += 1;
                    b58pow;
                    BN_div(bnbp, 0 as *mut BIGNUM, bnap, &mut bnbase, bnctx);
                    bntp = bnap;
                    bnap = bnbp;
                    bnbp = bntp;
                }
                b58ceil = BN_get_word(bnap) as libc::c_int;
                if b58pow - (p - zero_prefix) < 6 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Prefix '%s' is too long\n\0" as *const u8
                            as *const libc::c_char,
                        pfx,
                    );
                    current_block = 16307290864065614123;
                } else {
                    BN_set_word(
                        &mut bntmp2,
                        (b58pow - (p - zero_prefix)) as libc::c_ulong,
                    );
                    BN_exp(&mut bntmp, &mut bnbase, &mut bntmp2, bnctx);
                    BN_mul(bnlow, &mut bntmp, &mut bntarg, bnctx);
                    BN_sub(&mut bntmp2, &mut bntmp, BN_value_one());
                    BN_add(bnhigh, bnlow, &mut bntmp2);
                    if b58top <= b58ceil {
                        check_upper = 1 as libc::c_int;
                        bnlow2 = BN_new();
                        bnhigh2 = BN_new();
                        BN_mul(bnlow2, bnlow, &mut bnbase, bnctx);
                        BN_mul(&mut bntmp2, bnhigh, &mut bnbase, bnctx);
                        BN_set_word(&mut bntmp, 57 as libc::c_int as libc::c_ulong);
                        BN_add(bnhigh2, &mut bntmp2, &mut bntmp);
                        if BN_cmp(&mut bnceil, bnlow2) < 0 as libc::c_int {
                            check_upper = 0 as libc::c_int;
                            BN_free(bnhigh2);
                            bnhigh2 = 0 as *mut BIGNUM;
                            BN_free(bnlow2);
                            bnlow2 = 0 as *mut BIGNUM;
                        } else if BN_cmp(&mut bnceil, bnhigh2) < 0 as libc::c_int {
                            BN_copy(bnhigh2, &mut bnceil);
                        }
                        if BN_cmp(&mut bnfloor, bnhigh) >= 0 as libc::c_int {
                            if check_upper != 0 {} else {
                                __assert_fail(
                                    b"check_upper\0" as *const u8 as *const libc::c_char,
                                    b"pattern.c\0" as *const u8 as *const libc::c_char,
                                    820 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 62],
                                        &[libc::c_char; 62],
                                    >(
                                        b"int get_prefix_ranges(int, const char *, BIGNUM **, BN_CTX *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_16156: {
                                if check_upper != 0 {} else {
                                    __assert_fail(
                                        b"check_upper\0" as *const u8 as *const libc::c_char,
                                        b"pattern.c\0" as *const u8 as *const libc::c_char,
                                        820 as libc::c_int as libc::c_uint,
                                        (*::std::mem::transmute::<
                                            &[u8; 62],
                                            &[libc::c_char; 62],
                                        >(
                                            b"int get_prefix_ranges(int, const char *, BIGNUM **, BN_CTX *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            check_upper = 0 as libc::c_int;
                            BN_free(bnhigh);
                            bnhigh = bnhigh2;
                            bnhigh2 = 0 as *mut BIGNUM;
                            BN_free(bnlow);
                            bnlow = bnlow2;
                            bnlow2 = 0 as *mut BIGNUM;
                        } else if BN_cmp(&mut bnfloor, bnlow) > 0 as libc::c_int {
                            BN_copy(bnlow, &mut bnfloor);
                        }
                    }
                    current_block = 307447392441238883;
                }
            } else {
                BN_copy(bnhigh, &mut bnceil);
                BN_clear(bnlow);
                current_block = 307447392441238883;
            }
            match current_block {
                16307290864065614123 => {}
                _ => {
                    BN_clear(&mut bntmp);
                    BN_set_word(&mut bntmp, addrtype as libc::c_ulong);
                    BN_lshift(&mut bntmp2, &mut bntmp, 192 as libc::c_int);
                    if check_upper != 0 {
                        if BN_cmp(&mut bntmp2, bnhigh2) > 0 as libc::c_int {
                            check_upper = 0 as libc::c_int;
                            BN_free(bnhigh2);
                            bnhigh2 = 0 as *mut BIGNUM;
                            BN_free(bnlow2);
                            bnlow2 = 0 as *mut BIGNUM;
                        } else if BN_cmp(&mut bntmp2, bnlow2) > 0 as libc::c_int {
                            BN_copy(bnlow2, &mut bntmp2);
                        }
                    }
                    if BN_cmp(&mut bntmp2, bnhigh) > 0 as libc::c_int {
                        if check_upper == 0 {
                            current_block = 4640112314375303094;
                        } else {
                            check_upper = 0 as libc::c_int;
                            BN_free(bnhigh);
                            bnhigh = bnhigh2;
                            bnhigh2 = 0 as *mut BIGNUM;
                            BN_free(bnlow);
                            bnlow = bnlow2;
                            bnlow2 = 0 as *mut BIGNUM;
                            current_block = 12027283704867122503;
                        }
                    } else {
                        if BN_cmp(&mut bntmp2, bnlow) > 0 as libc::c_int {
                            BN_copy(bnlow, &mut bntmp2);
                        }
                        current_block = 12027283704867122503;
                    }
                    match current_block {
                        12027283704867122503 => {
                            BN_set_word(
                                &mut bntmp,
                                (addrtype + 1 as libc::c_int) as libc::c_ulong,
                            );
                            BN_lshift(&mut bntmp2, &mut bntmp, 192 as libc::c_int);
                            if check_upper != 0 {
                                if BN_cmp(&mut bntmp2, bnlow2) < 0 as libc::c_int {
                                    check_upper = 0 as libc::c_int;
                                    BN_free(bnhigh2);
                                    bnhigh2 = 0 as *mut BIGNUM;
                                    BN_free(bnlow2);
                                    bnlow2 = 0 as *mut BIGNUM;
                                } else if BN_cmp(&mut bntmp2, bnhigh2) < 0 as libc::c_int {
                                    BN_copy(bnlow2, &mut bntmp2);
                                }
                            }
                            if BN_cmp(&mut bntmp2, bnlow) < 0 as libc::c_int {
                                if check_upper == 0 {
                                    current_block = 4640112314375303094;
                                } else {
                                    check_upper = 0 as libc::c_int;
                                    BN_free(bnhigh);
                                    bnhigh = bnhigh2;
                                    bnhigh2 = 0 as *mut BIGNUM;
                                    BN_free(bnlow);
                                    bnlow = bnlow2;
                                    bnlow2 = 0 as *mut BIGNUM;
                                    current_block = 14116432890150942211;
                                }
                            } else {
                                if BN_cmp(&mut bntmp2, bnhigh) < 0 as libc::c_int {
                                    BN_copy(bnhigh, &mut bntmp2);
                                }
                                current_block = 14116432890150942211;
                            }
                            match current_block {
                                4640112314375303094 => {}
                                _ => {
                                    if check_upper != 0 || bnlow2.is_null() && bnhigh2.is_null()
                                    {} else {
                                        __assert_fail(
                                            b"check_upper || ((bnlow2 == NULL) && (bnhigh2 == NULL))\0"
                                                as *const u8 as *const libc::c_char,
                                            b"pattern.c\0" as *const u8 as *const libc::c_char,
                                            903 as libc::c_int as libc::c_uint,
                                            (*::std::mem::transmute::<
                                                &[u8; 62],
                                                &[libc::c_char; 62],
                                            >(
                                                b"int get_prefix_ranges(int, const char *, BIGNUM **, BN_CTX *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    'c_15642: {
                                        if check_upper != 0 || bnlow2.is_null() && bnhigh2.is_null()
                                        {} else {
                                            __assert_fail(
                                                b"check_upper || ((bnlow2 == NULL) && (bnhigh2 == NULL))\0"
                                                    as *const u8 as *const libc::c_char,
                                                b"pattern.c\0" as *const u8 as *const libc::c_char,
                                                903 as libc::c_int as libc::c_uint,
                                                (*::std::mem::transmute::<
                                                    &[u8; 62],
                                                    &[libc::c_char; 62],
                                                >(
                                                    b"int get_prefix_ranges(int, const char *, BIGNUM **, BN_CTX *)\0",
                                                ))
                                                    .as_ptr(),
                                            );
                                        }
                                    };
                                    let ref mut fresh1 = *result
                                        .offset(0 as libc::c_int as isize);
                                    *fresh1 = bnlow;
                                    let ref mut fresh2 = *result
                                        .offset(1 as libc::c_int as isize);
                                    *fresh2 = bnhigh;
                                    let ref mut fresh3 = *result
                                        .offset(2 as libc::c_int as isize);
                                    *fresh3 = bnlow2;
                                    let ref mut fresh4 = *result
                                        .offset(3 as libc::c_int as isize);
                                    *fresh4 = bnhigh2;
                                    bnlow = 0 as *mut BIGNUM;
                                    bnhigh = 0 as *mut BIGNUM;
                                    bnlow2 = 0 as *mut BIGNUM;
                                    bnhigh2 = 0 as *mut BIGNUM;
                                    ret = 0 as libc::c_int;
                                    current_block = 16307290864065614123;
                                }
                            }
                        }
                        _ => {}
                    }
                    match current_block {
                        16307290864065614123 => {}
                        _ => {
                            ret = -(2 as libc::c_int);
                        }
                    }
                }
            }
        }
        _ => {}
    }
    BN_clear_free(&mut bntarg);
    BN_clear_free(&mut bnceil);
    BN_clear_free(&mut bnfloor);
    BN_clear_free(&mut bnbase);
    BN_clear_free(&mut bntmp);
    BN_clear_free(&mut bntmp2);
    if !bnhigh.is_null() {
        BN_free(bnhigh);
    }
    if !bnlow.is_null() {
        BN_free(bnlow);
    }
    if !bnhigh2.is_null() {
        BN_free(bnhigh2);
    }
    if !bnlow2.is_null() {
        BN_free(bnlow2);
    }
    return ret;
}
unsafe extern "C" fn free_ranges(mut ranges: *mut *mut BIGNUM) {
    BN_free(*ranges.offset(0 as libc::c_int as isize));
    BN_free(*ranges.offset(1 as libc::c_int as isize));
    let ref mut fresh5 = *ranges.offset(0 as libc::c_int as isize);
    *fresh5 = 0 as *mut BIGNUM;
    let ref mut fresh6 = *ranges.offset(1 as libc::c_int as isize);
    *fresh6 = 0 as *mut BIGNUM;
    if !(*ranges.offset(2 as libc::c_int as isize)).is_null() {
        BN_free(*ranges.offset(2 as libc::c_int as isize));
        BN_free(*ranges.offset(3 as libc::c_int as isize));
        let ref mut fresh7 = *ranges.offset(2 as libc::c_int as isize);
        *fresh7 = 0 as *mut BIGNUM;
        let ref mut fresh8 = *ranges.offset(3 as libc::c_int as isize);
        *fresh8 = 0 as *mut BIGNUM;
    }
}
pub static mut vpk_nwords: libc::c_int = 0;
unsafe extern "C" fn vg_prefix_free(mut vp: *mut vg_prefix_t) {
    if !((*vp).vp_low).is_null() {
        BN_free((*vp).vp_low);
    }
    if !((*vp).vp_high).is_null() {
        BN_free((*vp).vp_high);
    }
    free(vp as *mut libc::c_void);
}
unsafe extern "C" fn vg_prefix_avl_search(
    mut rootp: *mut avl_root_t,
    mut targ: *mut BIGNUM,
) -> *mut vg_prefix_t {
    let mut vp: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    let mut itemp: *mut avl_item_t = (*rootp).ar_root;
    while !itemp.is_null() {
        vp = (itemp as *mut libc::c_uchar)
            .offset(
                -(&mut (*(0 as *mut libc::c_uchar as *mut vg_prefix_t)).vp_item
                    as *mut avl_item_t as size_t as isize),
            ) as *mut vg_prefix_t;
        if BN_cmp((*vp).vp_low, targ) > 0 as libc::c_int {
            itemp = (*itemp).ai_left;
        } else if BN_cmp((*vp).vp_high, targ) < 0 as libc::c_int {
            itemp = (*itemp).ai_right;
        } else {
            return vp
        }
    }
    return 0 as *mut vg_prefix_t;
}
unsafe extern "C" fn vg_prefix_avl_insert(
    mut rootp: *mut avl_root_t,
    mut vpnew: *mut vg_prefix_t,
) -> *mut vg_prefix_t {
    let mut vp: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    let mut itemp: *mut avl_item_t = 0 as *mut avl_item_t;
    let mut ptrp: *mut *mut avl_item_t = &mut (*rootp).ar_root;
    while !(*ptrp).is_null() {
        itemp = *ptrp;
        vp = (itemp as *mut libc::c_uchar)
            .offset(
                -(&mut (*(0 as *mut libc::c_uchar as *mut vg_prefix_t)).vp_item
                    as *mut avl_item_t as size_t as isize),
            ) as *mut vg_prefix_t;
        if BN_cmp((*vp).vp_low, (*vpnew).vp_high) > 0 as libc::c_int {
            ptrp = &mut (*itemp).ai_left;
        } else if BN_cmp((*vp).vp_high, (*vpnew).vp_low) < 0 as libc::c_int {
            ptrp = &mut (*itemp).ai_right;
        } else {
            return vp
        }
    }
    (*vpnew).vp_item.ai_up = itemp;
    itemp = &mut (*vpnew).vp_item;
    *ptrp = itemp;
    avl_insert_fix(rootp, itemp);
    return 0 as *mut vg_prefix_t;
}
unsafe extern "C" fn vg_prefix_first(mut rootp: *mut avl_root_t) -> *mut vg_prefix_t {
    let mut itemp: *mut avl_item_t = 0 as *mut avl_item_t;
    itemp = avl_first(rootp);
    if !itemp.is_null() {
        return (itemp as *mut libc::c_uchar)
            .offset(
                -(&mut (*(0 as *mut libc::c_uchar as *mut vg_prefix_t)).vp_item
                    as *mut avl_item_t as size_t as isize),
            ) as *mut vg_prefix_t;
    }
    return 0 as *mut vg_prefix_t;
}
unsafe extern "C" fn vg_prefix_next(mut vp: *mut vg_prefix_t) -> *mut vg_prefix_t {
    let mut itemp: *mut avl_item_t = &mut (*vp).vp_item;
    itemp = avl_next(itemp);
    if !itemp.is_null() {
        return (itemp as *mut libc::c_uchar)
            .offset(
                -(&mut (*(0 as *mut libc::c_uchar as *mut vg_prefix_t)).vp_item
                    as *mut avl_item_t as size_t as isize),
            ) as *mut vg_prefix_t;
    }
    return 0 as *mut vg_prefix_t;
}
unsafe extern "C" fn vg_prefix_add(
    mut rootp: *mut avl_root_t,
    mut pattern: *const libc::c_char,
    mut low: *mut BIGNUM,
    mut high: *mut BIGNUM,
) -> *mut vg_prefix_t {
    let mut vp: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    let mut vp2: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    if BN_cmp(low, high) < 0 as libc::c_int {} else {
        __assert_fail(
            b"BN_cmp(low, high) < 0\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            1047 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 75],
                &[libc::c_char; 75],
            >(
                b"vg_prefix_t *vg_prefix_add(avl_root_t *, const char *, BIGNUM *, BIGNUM *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_15264: {
        if BN_cmp(low, high) < 0 as libc::c_int {} else {
            __assert_fail(
                b"BN_cmp(low, high) < 0\0" as *const u8 as *const libc::c_char,
                b"pattern.c\0" as *const u8 as *const libc::c_char,
                1047 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"vg_prefix_t *vg_prefix_add(avl_root_t *, const char *, BIGNUM *, BIGNUM *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    vp = malloc(::std::mem::size_of::<vg_prefix_t>() as libc::c_ulong)
        as *mut vg_prefix_t;
    if !vp.is_null() {
        avl_item_init(&mut (*vp).vp_item);
        (*vp).vp_sibling = 0 as *mut _vg_prefix_s;
        (*vp).vp_pattern = pattern;
        (*vp).vp_low = low;
        (*vp).vp_high = high;
        vp2 = vg_prefix_avl_insert(rootp, vp);
        if !vp2.is_null() {
            fprintf(
                stderr,
                b"Prefix '%s' ignored, overlaps '%s'\n\0" as *const u8
                    as *const libc::c_char,
                pattern,
                (*vp2).vp_pattern,
            );
            vg_prefix_free(vp);
            vp = 0 as *mut vg_prefix_t;
        }
    }
    return vp;
}
unsafe extern "C" fn vg_prefix_delete(
    mut rootp: *mut avl_root_t,
    mut vp: *mut vg_prefix_t,
) {
    let mut sibp: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    let mut delp: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    avl_remove(rootp, &mut (*vp).vp_item);
    sibp = (*vp).vp_sibling;
    while !sibp.is_null() && sibp != vp {
        avl_remove(rootp, &mut (*sibp).vp_item);
        delp = sibp;
        sibp = (*sibp).vp_sibling;
        vg_prefix_free(delp);
    }
    vg_prefix_free(vp);
}
unsafe extern "C" fn vg_prefix_add_ranges(
    mut rootp: *mut avl_root_t,
    mut pattern: *const libc::c_char,
    mut ranges: *mut *mut BIGNUM,
    mut master: *mut vg_prefix_t,
) -> *mut vg_prefix_t {
    let mut vp: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    let mut vp2: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    if !(*ranges.offset(0 as libc::c_int as isize)).is_null() {} else {
        __assert_fail(
            b"ranges[0]\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            1089 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 88],
                &[libc::c_char; 88],
            >(
                b"vg_prefix_t *vg_prefix_add_ranges(avl_root_t *, const char *, BIGNUM **, vg_prefix_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_15346: {
        if !(*ranges.offset(0 as libc::c_int as isize)).is_null() {} else {
            __assert_fail(
                b"ranges[0]\0" as *const u8 as *const libc::c_char,
                b"pattern.c\0" as *const u8 as *const libc::c_char,
                1089 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 88],
                    &[libc::c_char; 88],
                >(
                    b"vg_prefix_t *vg_prefix_add_ranges(avl_root_t *, const char *, BIGNUM **, vg_prefix_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    vp = vg_prefix_add(
        rootp,
        pattern,
        *ranges.offset(0 as libc::c_int as isize),
        *ranges.offset(1 as libc::c_int as isize),
    );
    if vp.is_null() {
        return 0 as *mut vg_prefix_t;
    }
    if !(*ranges.offset(2 as libc::c_int as isize)).is_null() {
        vp2 = vg_prefix_add(
            rootp,
            pattern,
            *ranges.offset(2 as libc::c_int as isize),
            *ranges.offset(3 as libc::c_int as isize),
        );
        if vp2.is_null() {
            vg_prefix_delete(rootp, vp);
            return 0 as *mut vg_prefix_t;
        }
    }
    if master.is_null() {
        (*vp).vp_sibling = vp2;
        if !vp2.is_null() {
            (*vp2).vp_sibling = vp;
        }
    } else if !vp2.is_null() {
        (*vp).vp_sibling = vp2;
        (*vp2)
            .vp_sibling = if !((*master).vp_sibling).is_null() {
            (*master).vp_sibling
        } else {
            master
        };
        (*master).vp_sibling = vp;
    } else {
        (*vp)
            .vp_sibling = if !((*master).vp_sibling).is_null() {
            (*master).vp_sibling
        } else {
            master
        };
        (*master).vp_sibling = vp;
    }
    return vp;
}
unsafe extern "C" fn vg_prefix_range_sum(
    mut vp: *mut vg_prefix_t,
    mut result: *mut BIGNUM,
    mut tmp1: *mut BIGNUM,
) {
    let mut startp: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    startp = vp;
    BN_clear(result);
    loop {
        BN_sub(tmp1, (*vp).vp_high, (*vp).vp_low);
        BN_add(result, result, tmp1);
        vp = (*vp).vp_sibling;
        if !(!vp.is_null() && vp != startp) {
            break;
        }
    };
}
static mut b58_case_map: [libc::c_uchar; 256] = [
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
unsafe extern "C" fn prefix_case_iter_init(
    mut cip: *mut prefix_case_iter_t,
    mut pfx: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    (*cip).ci_nbits = 0 as libc::c_int as libc::c_char;
    (*cip).ci_value = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while *pfx.offset(i as isize) != 0 {
        if i as libc::c_ulong
            > ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong
        {
            return 0 as libc::c_int;
        }
        if b58_case_map[*pfx.offset(i as isize) as libc::c_int as usize] == 0 {
            (*cip).ci_prefix[i as usize] = *pfx.offset(i as isize);
        } else if b58_case_map[*pfx.offset(i as isize) as libc::c_int as usize]
            as libc::c_int == 2 as libc::c_int
        {
            (*cip)
                .ci_prefix[i
                as usize] = (*pfx.offset(i as isize) as libc::c_int
                ^ 0x20 as libc::c_int) as libc::c_char;
        } else {
            (*cip)
                .ci_prefix[i
                as usize] = (*pfx.offset(i as isize) as libc::c_int
                | 0x20 as libc::c_int) as libc::c_char;
            (*cip)
                .ci_case_map[(*cip).ci_nbits as libc::c_int
                as usize] = i as libc::c_char;
            (*cip).ci_nbits += 1;
            (*cip).ci_nbits;
        }
        i += 1;
        i;
    }
    (*cip).ci_prefix[i as usize] = '\0' as i32 as libc::c_char;
    return 1 as libc::c_int;
}
unsafe extern "C" fn prefix_case_iter_next(
    mut cip: *mut prefix_case_iter_t,
) -> libc::c_int {
    let mut val: libc::c_ulong = 0;
    let mut max: libc::c_ulong = 0;
    let mut mask: libc::c_ulong = 0;
    let mut i: libc::c_int = 0;
    let mut nbits: libc::c_int = 0;
    nbits = (*cip).ci_nbits as libc::c_int;
    max = ((1 as libc::c_ulong) << nbits)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    val = ((*cip).ci_value + 1 as libc::c_int) as libc::c_ulong;
    if val > max {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    mask = 1 as libc::c_int as libc::c_ulong;
    while i < nbits {
        if val & mask != 0 {
            (*cip)
                .ci_prefix[(*cip).ci_case_map[i as usize] as libc::c_int
                as usize] = ((*cip)
                .ci_prefix[(*cip).ci_case_map[i as usize] as libc::c_int as usize]
                as libc::c_int & 0xdf as libc::c_int) as libc::c_char;
        } else {
            (*cip)
                .ci_prefix[(*cip).ci_case_map[i as usize] as libc::c_int
                as usize] = ((*cip)
                .ci_prefix[(*cip).ci_case_map[i as usize] as libc::c_int as usize]
                as libc::c_int | 0x20 as libc::c_int) as libc::c_char;
        }
        i += 1;
        i;
        mask <<= 1 as libc::c_int;
    }
    (*cip).ci_value = val as libc::c_int;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn vg_prefix_context_set_case_insensitive(
    mut vcp: *mut vg_context_t,
    mut caseinsensitive: libc::c_int,
) {
    (*(vcp as *mut vg_prefix_context_t)).vcp_caseinsensitive = caseinsensitive;
}
unsafe extern "C" fn vg_prefix_context_clear_all_patterns(mut vcp: *mut vg_context_t) {
    let mut vcpp: *mut vg_prefix_context_t = vcp as *mut vg_prefix_context_t;
    let mut vp: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    let mut npfx_left: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    while avl_root_empty(&mut (*vcpp).vcp_avlroot) == 0 {
        vp = ((*vcpp).vcp_avlroot.ar_root as *mut libc::c_uchar)
            .offset(
                -(&mut (*(0 as *mut libc::c_uchar as *mut vg_prefix_t)).vp_item
                    as *mut avl_item_t as size_t as isize),
            ) as *mut vg_prefix_t;
        vg_prefix_delete(&mut (*vcpp).vcp_avlroot, vp);
        npfx_left = npfx_left.wrapping_add(1);
        npfx_left;
    }
    if npfx_left == (*vcpp).base.vc_npatterns {} else {
        __assert_fail(
            b"npfx_left == vcpp->base.vc_npatterns\0" as *const u8
                as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            1233 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"void vg_prefix_context_clear_all_patterns(vg_context_t *)\0"))
                .as_ptr(),
        );
    }
    'c_13686: {
        if npfx_left == (*vcpp).base.vc_npatterns {} else {
            __assert_fail(
                b"npfx_left == vcpp->base.vc_npatterns\0" as *const u8
                    as *const libc::c_char,
                b"pattern.c\0" as *const u8 as *const libc::c_char,
                1233 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 58],
                    &[libc::c_char; 58],
                >(b"void vg_prefix_context_clear_all_patterns(vg_context_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*vcpp).base.vc_npatterns = 0 as libc::c_int as libc::c_ulong;
    (*vcpp).base.vc_npatterns_start = 0 as libc::c_int as libc::c_ulong;
    (*vcpp).base.vc_found = 0 as libc::c_int as libc::c_ulonglong;
    BN_clear(&mut (*vcpp).vcp_difficulty);
}
unsafe extern "C" fn vg_prefix_context_free(mut vcp: *mut vg_context_t) {
    let mut vcpp: *mut vg_prefix_context_t = vcp as *mut vg_prefix_context_t;
    vg_prefix_context_clear_all_patterns(vcp);
    BN_clear_free(&mut (*vcpp).vcp_difficulty);
    free(vcpp as *mut libc::c_void);
}
unsafe extern "C" fn vg_prefix_context_next_difficulty(
    mut vcpp: *mut vg_prefix_context_t,
    mut bntmp: *mut BIGNUM,
    mut bntmp2: *mut BIGNUM,
    mut bnctx: *mut BN_CTX,
) {
    let mut dbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    BN_clear(bntmp);
    BN_set_bit(bntmp, 192 as libc::c_int);
    BN_div(bntmp2, 0 as *mut BIGNUM, bntmp, &mut (*vcpp).vcp_difficulty, bnctx);
    dbuf = BN_bn2dec(bntmp2);
    if (*vcpp).base.vc_verbose > 0 as libc::c_int {
        if (*vcpp).base.vc_npatterns > 1 as libc::c_int as libc::c_ulong {
            fprintf(
                stderr,
                b"Next match difficulty: %s (%ld prefixes)\n\0" as *const u8
                    as *const libc::c_char,
                dbuf,
                (*vcpp).base.vc_npatterns,
            );
        } else {
            fprintf(
                stderr,
                b"Difficulty: %s\n\0" as *const u8 as *const libc::c_char,
                dbuf,
            );
        }
    }
    (*vcpp).base.vc_chance = atof(dbuf);
    CRYPTO_free(dbuf as *mut libc::c_void);
}
unsafe extern "C" fn vg_prefix_context_add_patterns(
    mut vcp: *mut vg_context_t,
    patterns: *mut *const libc::c_char,
    mut npatterns: libc::c_int,
) -> libc::c_int {
    let mut vcpp: *mut vg_prefix_context_t = vcp as *mut vg_prefix_context_t;
    let mut caseiter: prefix_case_iter_t = prefix_case_iter_t {
        ci_prefix: [0; 32],
        ci_case_map: [0; 32],
        ci_nbits: 0,
        ci_value: 0,
    };
    let mut vp: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    let mut vp2: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    let mut bnctx: *mut BN_CTX = 0 as *mut BN_CTX;
    let mut bntmp: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bntmp2: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bntmp3: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut ranges: [*mut BIGNUM; 4] = [0 as *mut BIGNUM; 4];
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut impossible: libc::c_int = 0 as libc::c_int;
    let mut case_impossible: libc::c_int = 0;
    let mut npfx: libc::c_ulong = 0;
    let mut dbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    bnctx = BN_CTX_new();
    BN_init(&mut bntmp);
    BN_init(&mut bntmp2);
    BN_init(&mut bntmp3);
    npfx = 0 as libc::c_int as libc::c_ulong;
    let mut current_block_47: u64;
    i = 0 as libc::c_int;
    while i < npatterns {
        if (*vcpp).vcp_caseinsensitive == 0 {
            vp = 0 as *mut vg_prefix_t;
            ret = get_prefix_ranges(
                (*vcpp).base.vc_addrtype,
                *patterns.offset(i as isize),
                ranges.as_mut_ptr(),
                bnctx,
            );
            if ret == 0 {
                vp = vg_prefix_add_ranges(
                    &mut (*vcpp).vcp_avlroot,
                    *patterns.offset(i as isize),
                    ranges.as_mut_ptr(),
                    0 as *mut vg_prefix_t,
                );
            }
            current_block_47 = 7245201122033322888;
        } else if prefix_case_iter_init(&mut caseiter, *patterns.offset(i as isize)) == 0
        {
            fprintf(
                stderr,
                b"Prefix '%s' is too long\n\0" as *const u8 as *const libc::c_char,
                *patterns.offset(i as isize),
            );
            current_block_47 = 10886091980245723256;
        } else {
            if caseiter.ci_nbits as libc::c_int > 16 as libc::c_int {
                fprintf(
                    stderr,
                    b"WARNING: Prefix '%s' has 2^%d case-varied derivatives\n\0"
                        as *const u8 as *const libc::c_char,
                    *patterns.offset(i as isize),
                    caseiter.ci_nbits as libc::c_int,
                );
            }
            case_impossible = 0 as libc::c_int;
            vp = 0 as *mut vg_prefix_t;
            loop {
                ret = get_prefix_ranges(
                    (*vcpp).base.vc_addrtype,
                    (caseiter.ci_prefix).as_mut_ptr(),
                    ranges.as_mut_ptr(),
                    bnctx,
                );
                if ret == -(2 as libc::c_int) {
                    case_impossible += 1;
                    case_impossible;
                    ret = 0 as libc::c_int;
                } else {
                    if ret != 0 {
                        break;
                    }
                    vp2 = vg_prefix_add_ranges(
                        &mut (*vcpp).vcp_avlroot,
                        *patterns.offset(i as isize),
                        ranges.as_mut_ptr(),
                        vp,
                    );
                    if vp2.is_null() {
                        ret = -(1 as libc::c_int);
                        break;
                    } else if vp.is_null() {
                        vp = vp2;
                    }
                }
                if !(prefix_case_iter_next(&mut caseiter) != 0) {
                    break;
                }
            }
            if vp.is_null() && case_impossible != 0 {
                ret = -(2 as libc::c_int);
            }
            if ret != 0 && !vp.is_null() {
                vg_prefix_delete(&mut (*vcpp).vcp_avlroot, vp);
                vp = 0 as *mut vg_prefix_t;
            }
            current_block_47 = 7245201122033322888;
        }
        match current_block_47 {
            7245201122033322888 => {
                if ret == -(2 as libc::c_int) {
                    fprintf(
                        stderr,
                        b"Prefix '%s' not possible\n\0" as *const u8
                            as *const libc::c_char,
                        *patterns.offset(i as isize),
                    );
                    impossible += 1;
                    impossible;
                }
                if !vp.is_null() {
                    npfx = npfx.wrapping_add(1);
                    npfx;
                    vg_prefix_range_sum(vp, &mut bntmp, &mut bntmp2);
                    BN_add(&mut bntmp2, &mut (*vcpp).vcp_difficulty, &mut bntmp);
                    BN_copy(&mut (*vcpp).vcp_difficulty, &mut bntmp2);
                    if (*vcp).vc_verbose > 1 as libc::c_int {
                        BN_clear(&mut bntmp2);
                        BN_set_bit(&mut bntmp2, 192 as libc::c_int);
                        BN_div(
                            &mut bntmp3,
                            0 as *mut BIGNUM,
                            &mut bntmp2,
                            &mut bntmp,
                            bnctx,
                        );
                        dbuf = BN_bn2dec(&mut bntmp3);
                        fprintf(
                            stderr,
                            b"Prefix difficulty: %20s %s\n\0" as *const u8
                                as *const libc::c_char,
                            dbuf,
                            *patterns.offset(i as isize),
                        );
                        CRYPTO_free(dbuf as *mut libc::c_void);
                    }
                }
            }
            _ => {}
        }
        i += 1;
        i;
    }
    (*vcpp).base.vc_npatterns = ((*vcpp).base.vc_npatterns).wrapping_add(npfx);
    (*vcpp)
        .base
        .vc_npatterns_start = ((*vcpp).base.vc_npatterns_start).wrapping_add(npfx);
    if npfx == 0 && impossible != 0 {
        let mut ats: *const libc::c_char = b"bitcoin\0" as *const u8
            as *const libc::c_char;
        let mut bw: *const libc::c_char = b"\"1\"\0" as *const u8 as *const libc::c_char;
        match (*vcpp).base.vc_addrtype {
            5 => {
                ats = b"bitcoin script\0" as *const u8 as *const libc::c_char;
                bw = b"\"3\"\0" as *const u8 as *const libc::c_char;
            }
            111 => {
                ats = b"testnet\0" as *const u8 as *const libc::c_char;
                bw = b"\"m\" or \"n\"\0" as *const u8 as *const libc::c_char;
            }
            52 => {
                ats = b"namecoin\0" as *const u8 as *const libc::c_char;
                bw = b"\"M\" or \"N\"\0" as *const u8 as *const libc::c_char;
            }
            _ => {}
        }
        fprintf(
            stderr,
            b"Hint: valid %s addresses begin with %s\n\0" as *const u8
                as *const libc::c_char,
            ats,
            bw,
        );
    }
    if npfx != 0 {
        vg_prefix_context_next_difficulty(vcpp, &mut bntmp, &mut bntmp2, bnctx);
    }
    ret = (npfx != 0 as libc::c_int as libc::c_ulong) as libc::c_int;
    BN_clear_free(&mut bntmp);
    BN_clear_free(&mut bntmp2);
    BN_clear_free(&mut bntmp3);
    BN_CTX_free(bnctx);
    return ret;
}
pub unsafe extern "C" fn vg_prefix_get_difficulty(
    mut addrtype: libc::c_int,
    mut pattern: *const libc::c_char,
) -> libc::c_double {
    let mut bnctx: *mut BN_CTX = 0 as *mut BN_CTX;
    let mut result: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bntmp: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut ranges: [*mut BIGNUM; 4] = [0 as *mut BIGNUM; 4];
    let mut dbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut diffret: libc::c_double = 0.0f64;
    bnctx = BN_CTX_new();
    BN_init(&mut result);
    BN_init(&mut bntmp);
    ret = get_prefix_ranges(addrtype, pattern, ranges.as_mut_ptr(), bnctx);
    if ret == 0 as libc::c_int {
        BN_sub(
            &mut bntmp,
            ranges[1 as libc::c_int as usize],
            ranges[0 as libc::c_int as usize],
        );
        BN_add(&mut result, &mut result, &mut bntmp);
        if !(ranges[2 as libc::c_int as usize]).is_null() {
            BN_sub(
                &mut bntmp,
                ranges[3 as libc::c_int as usize],
                ranges[2 as libc::c_int as usize],
            );
            BN_add(&mut result, &mut result, &mut bntmp);
        }
        free_ranges(ranges.as_mut_ptr());
        BN_clear(&mut bntmp);
        BN_set_bit(&mut bntmp, 192 as libc::c_int);
        BN_div(&mut result, 0 as *mut BIGNUM, &mut bntmp, &mut result, bnctx);
        dbuf = BN_bn2dec(&mut result);
        diffret = strtod(dbuf, 0 as *mut *mut libc::c_char);
        CRYPTO_free(dbuf as *mut libc::c_void);
    }
    BN_clear_free(&mut result);
    BN_clear_free(&mut bntmp);
    BN_CTX_free(bnctx);
    return diffret;
}
unsafe extern "C" fn vg_prefix_test(mut vxcp: *mut vg_exec_context_t) -> libc::c_int {
    let mut vcpp: *mut vg_prefix_context_t = (*vxcp).vxc_vc as *mut vg_prefix_context_t;
    let mut vp: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    let mut res: libc::c_int = 0 as libc::c_int;
    BN_bin2bn(
        ((*vxcp).vxc_binres).as_mut_ptr(),
        25 as libc::c_int,
        &mut (*vxcp).vxc_bntarg,
    );
    loop {
        vp = vg_prefix_avl_search(&mut (*vcpp).vcp_avlroot, &mut (*vxcp).vxc_bntarg);
        if vp.is_null() {
            break;
        }
        if vg_exec_context_upgrade_lock(vxcp) != 0 {
            continue;
        }
        vg_exec_context_consolidate_key(vxcp);
        ((*vcpp).base.vc_output_match)
            .unwrap()(&mut (*vcpp).base, (*vxcp).vxc_key, (*vp).vp_pattern);
        (*vcpp).base.vc_found = ((*vcpp).base.vc_found).wrapping_add(1);
        (*vcpp).base.vc_found;
        if (*vcpp).base.vc_only_one != 0 {
            return 2 as libc::c_int;
        }
        if (*vcpp).base.vc_remove_on_match != 0 {
            vg_prefix_range_sum(vp, &mut (*vxcp).vxc_bntarg, &mut (*vxcp).vxc_bntmp);
            BN_sub(
                &mut (*vxcp).vxc_bntmp,
                &mut (*vcpp).vcp_difficulty,
                &mut (*vxcp).vxc_bntarg,
            );
            BN_copy(&mut (*vcpp).vcp_difficulty, &mut (*vxcp).vxc_bntmp);
            vg_prefix_delete(&mut (*vcpp).vcp_avlroot, vp);
            (*vcpp).base.vc_npatterns = ((*vcpp).base.vc_npatterns).wrapping_sub(1);
            (*vcpp).base.vc_npatterns;
            if avl_root_empty(&mut (*vcpp).vcp_avlroot) == 0 {
                vg_prefix_context_next_difficulty(
                    vcpp,
                    &mut (*vxcp).vxc_bntmp,
                    &mut (*vxcp).vxc_bntmp2,
                    (*vxcp).vxc_bnctx,
                );
            }
            (*vcpp).base.vc_pattern_generation += 1;
            (*vcpp).base.vc_pattern_generation;
        }
        res = 1 as libc::c_int;
        break;
    }
    if avl_root_empty(&mut (*vcpp).vcp_avlroot) != 0 {
        return 2 as libc::c_int;
    }
    return res;
}
unsafe extern "C" fn vg_prefix_hash160_sort(
    mut vcp: *mut vg_context_t,
    mut buf: *mut libc::c_void,
) -> libc::c_int {
    let mut vcpp: *mut vg_prefix_context_t = vcp as *mut vg_prefix_context_t;
    let mut vp: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    let mut cbuf: *mut libc::c_uchar = buf as *mut libc::c_uchar;
    let mut bnbuf: [libc::c_uchar; 25] = [0; 25];
    let mut nbytes: libc::c_int = 0;
    let mut ncopy: libc::c_int = 0;
    let mut nskip: libc::c_int = 0;
    let mut npfx: libc::c_int = 0 as libc::c_int;
    vp = vg_prefix_first(&mut (*vcpp).vcp_avlroot);
    while !vp.is_null() {
        npfx += 1;
        npfx;
        if !buf.is_null() {
            nbytes = BN_bn2bin((*vp).vp_low, bnbuf.as_mut_ptr());
            ncopy = if nbytes >= 24 as libc::c_int {
                20 as libc::c_int
            } else if nbytes > 4 as libc::c_int {
                nbytes - 4 as libc::c_int
            } else {
                0 as libc::c_int
            };
            nskip = if nbytes >= 24 as libc::c_int {
                nbytes - 24 as libc::c_int
            } else {
                0 as libc::c_int
            };
            if ncopy < 20 as libc::c_int {
                memset(
                    cbuf as *mut libc::c_void,
                    0 as libc::c_int,
                    (20 as libc::c_int - ncopy) as libc::c_ulong,
                );
            }
            memcpy(
                cbuf.offset((20 as libc::c_int - ncopy) as isize) as *mut libc::c_void,
                bnbuf.as_mut_ptr().offset(nskip as isize) as *const libc::c_void,
                ncopy as libc::c_ulong,
            );
            cbuf = cbuf.offset(20 as libc::c_int as isize);
            nbytes = BN_bn2bin((*vp).vp_high, bnbuf.as_mut_ptr());
            ncopy = if nbytes >= 24 as libc::c_int {
                20 as libc::c_int
            } else if nbytes > 4 as libc::c_int {
                nbytes - 4 as libc::c_int
            } else {
                0 as libc::c_int
            };
            nskip = if nbytes >= 24 as libc::c_int {
                nbytes - 24 as libc::c_int
            } else {
                0 as libc::c_int
            };
            if ncopy < 20 as libc::c_int {
                memset(
                    cbuf as *mut libc::c_void,
                    0 as libc::c_int,
                    (20 as libc::c_int - ncopy) as libc::c_ulong,
                );
            }
            memcpy(
                cbuf.offset((20 as libc::c_int - ncopy) as isize) as *mut libc::c_void,
                bnbuf.as_mut_ptr().offset(nskip as isize) as *const libc::c_void,
                ncopy as libc::c_ulong,
            );
            cbuf = cbuf.offset(20 as libc::c_int as isize);
        }
        vp = vg_prefix_next(vp);
    }
    return npfx;
}
pub unsafe extern "C" fn vg_prefix_context_new(
    mut addrtype: libc::c_int,
    mut privtype: libc::c_int,
    mut caseinsensitive: libc::c_int,
) -> *mut vg_context_t {
    let mut vcpp: *mut vg_prefix_context_t = 0 as *mut vg_prefix_context_t;
    vcpp = malloc(::std::mem::size_of::<vg_prefix_context_t>() as libc::c_ulong)
        as *mut vg_prefix_context_t;
    if !vcpp.is_null() {
        memset(
            vcpp as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<vg_prefix_context_t>() as libc::c_ulong,
        );
        (*vcpp).base.vc_addrtype = addrtype;
        (*vcpp).base.vc_privtype = privtype;
        (*vcpp).base.vc_npatterns = 0 as libc::c_int as libc::c_ulong;
        (*vcpp).base.vc_npatterns_start = 0 as libc::c_int as libc::c_ulong;
        (*vcpp).base.vc_found = 0 as libc::c_int as libc::c_ulonglong;
        (*vcpp).base.vc_chance = 0.0f64;
        (*vcpp)
            .base
            .vc_free = Some(
            vg_prefix_context_free as unsafe extern "C" fn(*mut vg_context_t) -> (),
        );
        (*vcpp)
            .base
            .vc_add_patterns = Some(
            vg_prefix_context_add_patterns
                as unsafe extern "C" fn(
                    *mut vg_context_t,
                    *mut *const libc::c_char,
                    libc::c_int,
                ) -> libc::c_int,
        );
        (*vcpp)
            .base
            .vc_clear_all_patterns = Some(
            vg_prefix_context_clear_all_patterns
                as unsafe extern "C" fn(*mut vg_context_t) -> (),
        );
        (*vcpp)
            .base
            .vc_test = Some(
            vg_prefix_test as unsafe extern "C" fn(*mut vg_exec_context_t) -> libc::c_int,
        );
        (*vcpp)
            .base
            .vc_hash160_sort = Some(
            vg_prefix_hash160_sort
                as unsafe extern "C" fn(
                    *mut vg_context_t,
                    *mut libc::c_void,
                ) -> libc::c_int,
        );
        avl_root_init(&mut (*vcpp).vcp_avlroot);
        BN_init(&mut (*vcpp).vcp_difficulty);
        (*vcpp).vcp_caseinsensitive = caseinsensitive;
    }
    return &mut (*vcpp).base;
}
unsafe extern "C" fn vg_regex_context_add_patterns(
    mut vcp: *mut vg_context_t,
    patterns: *mut *const libc::c_char,
    mut npatterns: libc::c_int,
) -> libc::c_int {
    let mut vcrp: *mut vg_regex_context_t = vcp as *mut vg_regex_context_t;
    let mut pcre_errptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut pcre_erroffset: libc::c_int = 0;
    let mut i: libc::c_ulong = 0;
    let mut nres: libc::c_ulong = 0;
    let mut count: libc::c_ulong = 0;
    let mut mem: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    if npatterns == 0 {
        return 1 as libc::c_int;
    }
    if npatterns as libc::c_ulong
        > ((*vcrp).vcr_nalloc).wrapping_sub((*vcrp).base.vc_npatterns)
    {
        count = (npatterns as libc::c_ulong).wrapping_add((*vcrp).base.vc_npatterns);
        if count < (2 as libc::c_int as libc::c_ulong).wrapping_mul((*vcrp).vcr_nalloc) {
            count = (2 as libc::c_int as libc::c_ulong).wrapping_mul((*vcrp).vcr_nalloc);
        }
        if count < 16 as libc::c_int as libc::c_ulong {
            count = 16 as libc::c_int as libc::c_ulong;
        }
        mem = malloc(
            (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(count)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_void;
        if mem.is_null() {
            return 0 as libc::c_int;
        }
        i = 0 as libc::c_int as libc::c_ulong;
        while i < (*vcrp).base.vc_npatterns {
            let ref mut fresh9 = *mem.offset(i as isize);
            *fresh9 = *((*vcrp).vcr_regex).offset(i as isize) as *mut libc::c_void;
            let ref mut fresh10 = *mem.offset(count.wrapping_add(i) as isize);
            *fresh10 = *((*vcrp).vcr_regex_extra).offset(i as isize)
                as *mut libc::c_void;
            let ref mut fresh11 = *mem
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(count)
                        .wrapping_add(i) as isize,
                );
            *fresh11 = *((*vcrp).vcr_regex_pat).offset(i as isize) as *mut libc::c_void;
            i = i.wrapping_add(1);
            i;
        }
        if (*vcrp).vcr_nalloc != 0 {
            free((*vcrp).vcr_regex as *mut libc::c_void);
        }
        (*vcrp).vcr_regex = mem as *mut *mut pcre;
        (*vcrp)
            .vcr_regex_extra = &mut *mem.offset(count as isize) as *mut *mut libc::c_void
            as *mut *mut pcre_extra;
        (*vcrp)
            .vcr_regex_pat = &mut *mem
            .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(count) as isize)
            as *mut *mut libc::c_void as *mut *const libc::c_char;
        (*vcrp).vcr_nalloc = count;
    }
    nres = (*vcrp).base.vc_npatterns;
    i = 0 as libc::c_int as libc::c_ulong;
    while i < npatterns as libc::c_ulong {
        let ref mut fresh12 = *((*vcrp).vcr_regex).offset(nres as isize);
        *fresh12 = pcre_compile(
            *patterns.offset(i as isize),
            0 as libc::c_int,
            &mut pcre_errptr,
            &mut pcre_erroffset,
            0 as *const libc::c_uchar,
        );
        if (*((*vcrp).vcr_regex).offset(nres as isize)).is_null() {
            let mut spaces: *const libc::c_char = b"                \0" as *const u8
                as *const libc::c_char;
            fprintf(
                stderr,
                b"%s\n\0" as *const u8 as *const libc::c_char,
                *patterns.offset(i as isize),
            );
            while pcre_erroffset > 16 as libc::c_int {
                fprintf(stderr, b"%s\0" as *const u8 as *const libc::c_char, spaces);
                pcre_erroffset -= 16 as libc::c_int;
            }
            if pcre_erroffset > 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    &*spaces.offset((16 as libc::c_int - pcre_erroffset) as isize)
                        as *const libc::c_char,
                );
            }
            fprintf(
                stderr,
                b"^\nRegex error: %s\n\0" as *const u8 as *const libc::c_char,
                pcre_errptr,
            );
        } else {
            let ref mut fresh13 = *((*vcrp).vcr_regex_extra).offset(nres as isize);
            *fresh13 = pcre_study(
                *((*vcrp).vcr_regex).offset(nres as isize),
                0 as libc::c_int,
                &mut pcre_errptr,
            );
            if !pcre_errptr.is_null() {
                fprintf(
                    stderr,
                    b"Regex error: %s\n\0" as *const u8 as *const libc::c_char,
                    pcre_errptr,
                );
                pcre_free
                    .unwrap()(
                    *((*vcrp).vcr_regex).offset(nres as isize) as *mut libc::c_void,
                );
            } else {
                let ref mut fresh14 = *((*vcrp).vcr_regex_pat).offset(nres as isize);
                *fresh14 = *patterns.offset(i as isize);
                nres = nres.wrapping_add(1 as libc::c_int as libc::c_ulong);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if nres == (*vcrp).base.vc_npatterns {
        return 0 as libc::c_int;
    }
    (*vcrp)
        .base
        .vc_npatterns_start = ((*vcrp).base.vc_npatterns_start)
        .wrapping_add(nres.wrapping_sub((*vcrp).base.vc_npatterns));
    (*vcrp).base.vc_npatterns = nres;
    return 1 as libc::c_int;
}
unsafe extern "C" fn vg_regex_context_clear_all_patterns(mut vcp: *mut vg_context_t) {
    let mut vcrp: *mut vg_regex_context_t = vcp as *mut vg_regex_context_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < (*vcrp).base.vc_npatterns {
        if !(*((*vcrp).vcr_regex_extra).offset(i as isize)).is_null() {
            pcre_free
                .unwrap()(
                *((*vcrp).vcr_regex_extra).offset(i as isize) as *mut libc::c_void,
            );
        }
        pcre_free.unwrap()(*((*vcrp).vcr_regex).offset(i as isize) as *mut libc::c_void);
        i += 1;
        i;
    }
    (*vcrp).base.vc_npatterns = 0 as libc::c_int as libc::c_ulong;
    (*vcrp).base.vc_npatterns_start = 0 as libc::c_int as libc::c_ulong;
    (*vcrp).base.vc_found = 0 as libc::c_int as libc::c_ulonglong;
}
unsafe extern "C" fn vg_regex_context_free(mut vcp: *mut vg_context_t) {
    let mut vcrp: *mut vg_regex_context_t = vcp as *mut vg_regex_context_t;
    vg_regex_context_clear_all_patterns(vcp);
    if (*vcrp).vcr_nalloc != 0 {
        free((*vcrp).vcr_regex as *mut libc::c_void);
    }
    free(vcrp as *mut libc::c_void);
}
unsafe extern "C" fn vg_regex_test(mut vxcp: *mut vg_exec_context_t) -> libc::c_int {
    let mut vcrp: *mut vg_regex_context_t = (*vxcp).vxc_vc as *mut vg_regex_context_t;
    let mut hash1: [libc::c_uchar; 32] = [0; 32];
    let mut hash2: [libc::c_uchar; 32] = [0; 32];
    let mut i: libc::c_int = 0;
    let mut zpfx: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut nres: libc::c_int = 0;
    let mut re_vec: [libc::c_int; 9] = [0; 9];
    let mut b58: [libc::c_char; 40] = [0; 40];
    let mut bnrem: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bn: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bndiv: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bnptmp: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut re: *mut pcre = 0 as *mut pcre;
    BN_init(&mut bnrem);
    SHA256(
        ((*vxcp).vxc_binres).as_mut_ptr(),
        21 as libc::c_int as size_t,
        hash1.as_mut_ptr(),
    );
    SHA256(
        hash1.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong,
        hash2.as_mut_ptr(),
    );
    memcpy(
        &mut *((*vxcp).vxc_binres).as_mut_ptr().offset(21 as libc::c_int as isize)
            as *mut libc::c_uchar as *mut libc::c_void,
        hash2.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    bn = &mut (*vxcp).vxc_bntmp;
    bndiv = &mut (*vxcp).vxc_bntmp2;
    BN_bin2bn(((*vxcp).vxc_binres).as_mut_ptr(), 25 as libc::c_int, bn);
    zpfx = 0 as libc::c_int;
    while zpfx < 25 as libc::c_int
        && (*vxcp).vxc_binres[zpfx as usize] as libc::c_int == 0 as libc::c_int
    {
        zpfx += 1;
        zpfx;
    }
    p = (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    b58[p as usize] = '\0' as i32 as libc::c_char;
    while !((*bn).top == 0 as libc::c_int) {
        BN_div(bndiv, &mut bnrem, bn, &mut (*vxcp).vxc_bnbase, (*vxcp).vxc_bnctx);
        bnptmp = bn;
        bn = bndiv;
        bndiv = bnptmp;
        d = BN_get_word(&mut bnrem) as libc::c_int;
        p -= 1;
        b58[p as usize] = *vg_b58_alphabet.offset(d as isize);
    }
    loop {
        let fresh15 = zpfx;
        zpfx = zpfx - 1;
        if !(fresh15 != 0) {
            break;
        }
        p -= 1;
        b58[p as usize] = *vg_b58_alphabet.offset(0 as libc::c_int as isize);
    }
    '_restart_loop: loop {
        nres = (*vcrp).base.vc_npatterns as libc::c_int;
        if nres == 0 {
            res = 2 as libc::c_int;
            break;
        } else {
            i = 0 as libc::c_int;
            loop {
                if !(i < nres) {
                    break '_restart_loop;
                }
                d = pcre_exec(
                    *((*vcrp).vcr_regex).offset(i as isize),
                    *((*vcrp).vcr_regex_extra).offset(i as isize),
                    &mut *b58.as_mut_ptr().offset(p as isize),
                    (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(p as libc::c_ulong) as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    re_vec.as_mut_ptr(),
                    (::std::mem::size_of::<[libc::c_int; 9]>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ) as libc::c_int,
                );
                if d <= 0 as libc::c_int {
                    if d != -(1 as libc::c_int) {
                        fprintf(
                            stderr,
                            b"PCRE error: %d\n\0" as *const u8 as *const libc::c_char,
                            d,
                        );
                        res = 2 as libc::c_int;
                        break '_restart_loop;
                    }
                } else {
                    re = *((*vcrp).vcr_regex).offset(i as isize);
                    if vg_exec_context_upgrade_lock(vxcp) != 0
                        && (i as libc::c_ulong >= (*vcrp).base.vc_npatterns
                            || *((*vcrp).vcr_regex).offset(i as isize) != re)
                    {
                        break;
                    }
                    vg_exec_context_consolidate_key(vxcp);
                    ((*vcrp).base.vc_output_match)
                        .unwrap()(
                        &mut (*vcrp).base,
                        (*vxcp).vxc_key,
                        *((*vcrp).vcr_regex_pat).offset(i as isize),
                    );
                    (*vcrp).base.vc_found = ((*vcrp).base.vc_found).wrapping_add(1);
                    (*vcrp).base.vc_found;
                    if (*vcrp).base.vc_only_one != 0 {
                        res = 2 as libc::c_int;
                        break '_restart_loop;
                    } else {
                        if (*vcrp).base.vc_remove_on_match != 0 {
                            pcre_free
                                .unwrap()(
                                *((*vcrp).vcr_regex).offset(i as isize) as *mut libc::c_void,
                            );
                            if !(*((*vcrp).vcr_regex_extra).offset(i as isize)).is_null()
                            {
                                pcre_free
                                    .unwrap()(
                                    *((*vcrp).vcr_regex_extra).offset(i as isize)
                                        as *mut libc::c_void,
                                );
                            }
                            nres -= 1 as libc::c_int;
                            (*vcrp).base.vc_npatterns = nres as libc::c_ulong;
                            if nres == 0 {
                                res = 2 as libc::c_int;
                                break '_restart_loop;
                            } else {
                                let ref mut fresh16 = *((*vcrp).vcr_regex)
                                    .offset(i as isize);
                                *fresh16 = *((*vcrp).vcr_regex).offset(nres as isize);
                                let ref mut fresh17 = *((*vcrp).vcr_regex_extra)
                                    .offset(i as isize);
                                *fresh17 = *((*vcrp).vcr_regex_extra).offset(nres as isize);
                                let ref mut fresh18 = *((*vcrp).vcr_regex_pat)
                                    .offset(i as isize);
                                *fresh18 = *((*vcrp).vcr_regex_pat).offset(nres as isize);
                                (*vcrp).base.vc_npatterns = nres as libc::c_ulong;
                                (*vcrp).base.vc_pattern_generation += 1;
                                (*vcrp).base.vc_pattern_generation;
                            }
                        }
                        res = 1 as libc::c_int;
                    }
                }
                i += 1;
                i;
            }
        }
    }
    BN_clear_free(&mut bnrem);
    return res;
}
pub unsafe extern "C" fn vg_regex_context_new(
    mut addrtype: libc::c_int,
    mut privtype: libc::c_int,
) -> *mut vg_context_t {
    let mut vcrp: *mut vg_regex_context_t = 0 as *mut vg_regex_context_t;
    vcrp = malloc(::std::mem::size_of::<vg_regex_context_t>() as libc::c_ulong)
        as *mut vg_regex_context_t;
    if !vcrp.is_null() {
        memset(
            vcrp as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<vg_regex_context_t>() as libc::c_ulong,
        );
        (*vcrp).base.vc_addrtype = addrtype;
        (*vcrp).base.vc_privtype = privtype;
        (*vcrp).base.vc_npatterns = 0 as libc::c_int as libc::c_ulong;
        (*vcrp).base.vc_npatterns_start = 0 as libc::c_int as libc::c_ulong;
        (*vcrp).base.vc_found = 0 as libc::c_int as libc::c_ulonglong;
        (*vcrp).base.vc_chance = 0.0f64;
        (*vcrp)
            .base
            .vc_free = Some(
            vg_regex_context_free as unsafe extern "C" fn(*mut vg_context_t) -> (),
        );
        (*vcrp)
            .base
            .vc_add_patterns = Some(
            vg_regex_context_add_patterns
                as unsafe extern "C" fn(
                    *mut vg_context_t,
                    *mut *const libc::c_char,
                    libc::c_int,
                ) -> libc::c_int,
        );
        (*vcrp)
            .base
            .vc_clear_all_patterns = Some(
            vg_regex_context_clear_all_patterns
                as unsafe extern "C" fn(*mut vg_context_t) -> (),
        );
        (*vcrp)
            .base
            .vc_test = Some(
            vg_regex_test as unsafe extern "C" fn(*mut vg_exec_context_t) -> libc::c_int,
        );
        (*vcrp).base.vc_hash160_sort = None;
        (*vcrp).vcr_regex = 0 as *mut *mut pcre;
        (*vcrp).vcr_nalloc = 0 as libc::c_int as libc::c_ulong;
    }
    return &mut (*vcrp).base;
}
unsafe extern "C" fn run_static_initializers() {
    vpk_nwords = (25 as libc::c_int as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
        as libc::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
