use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type quoting_options;
    pub type re_dfa_t;
    pub type saved_cwd;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn getgrgid(__gid: __gid_t) -> *mut group;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn __fxstatat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn abort() -> !;
    fn rpl_free(ptr: *mut libc::c_void);
    fn areadlinkat(fd: libc::c_int, filename: *const libc::c_char) -> *mut libc::c_char;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn base_name(file: *const libc::c_char) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn rpl_fflush(gl_stream: *mut FILE) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn filemodestring(statp: *const stat, str: *mut libc::c_char);
    fn human_readable(
        _: uintmax_t,
        _: *mut libc::c_char,
        _: libc::c_int,
        _: uintmax_t,
        _: uintmax_t,
    ) -> *mut libc::c_char;
    fn print_quoted(
        fp: *mut FILE,
        qopts: *const quoting_options,
        dest_is_tty: bool,
        format: *const libc::c_char,
        s: *const libc::c_char,
    ) -> libc::c_int;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn x2nrealloc(p: *mut libc::c_void, pn: *mut size_t, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn freecon(con: *mut libc::c_char);
    fn filesystem_type(
        statp: *const stat,
        path: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn nonfatal_target_file_error(errno_value: libc::c_int, name: *const libc::c_char);
    fn nonfatal_nontarget_file_error(
        errno_value: libc::c_int,
        name: *const libc::c_char,
    );
    fn safely_quote_err_filename(
        n: libc::c_int,
        arg: *const libc::c_char,
    ) -> *const libc::c_char;
    fn insert_primary_withpred(
        entry: *const parser_table,
        fptr: PRED_FUNC,
        arg: *const libc::c_char,
    ) -> *mut predicate;
    static mut options: options;
    static mut state: state;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = libc::c_ulong;
pub type gid_t = __gid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
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
pub type uid_t = __uid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
pub type va_list = __builtin_va_list;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
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
pub type uintmax_t = __uintmax_t;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const human_B: C2RustUnnamed_0 = 256;
pub const human_SI: C2RustUnnamed_0 = 128;
pub const human_space_before_unit: C2RustUnnamed_0 = 64;
pub const human_base_1024: C2RustUnnamed_0 = 32;
pub const human_autoscale: C2RustUnnamed_0 = 16;
pub const human_suppress_point_zero: C2RustUnnamed_0 = 8;
pub const human_group_digits: C2RustUnnamed_0 = 4;
pub const human_floor: C2RustUnnamed_0 = 2;
pub const human_round_to_nearest: C2RustUnnamed_0 = 1;
pub const human_ceiling: C2RustUnnamed_0 = 0;
pub type quoting_style = libc::c_uint;
pub const custom_quoting_style: quoting_style = 10;
pub const clocale_quoting_style: quoting_style = 9;
pub const locale_quoting_style: quoting_style = 8;
pub const escape_quoting_style: quoting_style = 7;
pub const c_maybe_quoting_style: quoting_style = 6;
pub const c_quoting_style: quoting_style = 5;
pub const shell_escape_always_quoting_style: quoting_style = 4;
pub const shell_escape_quoting_style: quoting_style = 3;
pub const shell_always_quoting_style: quoting_style = 2;
pub const shell_quoting_style: quoting_style = 1;
pub const literal_quoting_style: quoting_style = 0;
pub type __re_long_size_t = size_t;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buildcmd_state {
    pub cmd_argc: size_t,
    pub cmd_argv: *mut *mut libc::c_char,
    pub cmd_argv_alloc: size_t,
    pub argbuf: *mut libc::c_char,
    pub cmd_argv_chars: size_t,
    pub cmd_initial_argv_chars: size_t,
    pub usercontext: *mut libc::c_void,
    pub todo: libc::c_int,
    pub dir_fd: libc::c_int,
    pub largest_successful_arg_count: size_t,
    pub smallest_failed_arg_count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buildcmd_control {
    pub exit_if_size_exceeded: libc::c_int,
    pub posix_arg_size_max: size_t,
    pub posix_arg_size_min: size_t,
    pub arg_max: size_t,
    pub max_arg_count: size_t,
    pub rplen: size_t,
    pub replace_pat: *const libc::c_char,
    pub initial_argc: size_t,
    pub exec_callback: Option::<
        unsafe extern "C" fn(
            *mut buildcmd_control,
            *mut libc::c_void,
            libc::c_int,
            *mut *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub lines_per_exec: libc::c_ulong,
    pub args_per_exec: size_t,
}
pub type sharefile_handle = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct predicate {
    pub pred_func: PRED_FUNC,
    pub p_name: *const libc::c_char,
    pub p_type: predicate_type,
    pub p_prec: predicate_precedence,
    pub side_effects: bool,
    pub no_default_print: bool,
    pub need_stat: bool,
    pub need_type: bool,
    pub need_inum: bool,
    pub p_cost: EvaluationCost,
    pub est_success_rate: libc::c_float,
    pub literal_control_chars: bool,
    pub artificial: bool,
    pub arg_text: *const libc::c_char,
    pub args: C2RustUnnamed_1,
    pub pred_next: *mut predicate,
    pub pred_left: *mut predicate,
    pub pred_right: *mut predicate,
    pub perf: predicate_performance_info,
    pub parser_entry: *const parser_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_table {
    pub type_0: arg_type,
    pub parser_name: *const libc::c_char,
    pub parser_func: PARSE_FUNC,
    pub pred_func: PRED_FUNC,
}
pub type PRED_FUNC = Option::<
    unsafe extern "C" fn(*const libc::c_char, *mut stat, *mut predicate) -> bool,
>;
pub type PARSE_FUNC = Option::<
    unsafe extern "C" fn(
        *const parser_table,
        *mut *mut libc::c_char,
        *mut libc::c_int,
    ) -> bool,
>;
pub type arg_type = libc::c_uint;
pub const ARG_ACTION: arg_type = 6;
pub const ARG_PUNCTUATION: arg_type = 5;
pub const ARG_SPECIAL_PARSE: arg_type = 4;
pub const ARG_TEST: arg_type = 3;
pub const ARG_POSITIONAL_OPTION: arg_type = 2;
pub const ARG_NOOP: arg_type = 1;
pub const ARG_OPTION: arg_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct predicate_performance_info {
    pub visits: libc::c_ulong,
    pub successes: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub str_0: *const libc::c_char,
    pub regex: *mut re_pattern_buffer,
    pub exec_vec: exec_val,
    pub numinfo: long_val,
    pub size: size_val,
    pub uid: uid_t,
    pub gid: gid_t,
    pub reftime: time_val,
    pub perm: perm_val,
    pub samefileid: samefile_file_id,
    pub types: [bool; 7],
    pub printf_vec: format_val,
    pub scontext: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct format_val {
    pub segment: *mut segment,
    pub stream: *mut FILE,
    pub filename: *const libc::c_char,
    pub dest_is_tty: bool,
    pub quote_opts: *mut quoting_options,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct segment {
    pub segkind: SegmentKind,
    pub format_char: [libc::c_char; 2],
    pub text: *mut libc::c_char,
    pub text_len: libc::c_int,
    pub next: *mut segment,
}
pub type SegmentKind = libc::c_uint;
pub const KIND_FORMAT: SegmentKind = 2;
pub const KIND_STOP: SegmentKind = 1;
pub const KIND_PLAIN: SegmentKind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct samefile_file_id {
    pub ino: ino_t,
    pub dev: dev_t,
    pub fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct perm_val {
    pub kind: permissions_type,
    pub val: [mode_t; 2],
}
pub type permissions_type = libc::c_uint;
pub const PERM_EXACT: permissions_type = 2;
pub const PERM_ANY: permissions_type = 1;
pub const PERM_AT_LEAST: permissions_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct time_val {
    pub xval: xval,
    pub kind: comparison_type,
    pub ts: timespec,
}
pub type comparison_type = libc::c_uint;
pub const COMP_EQ: comparison_type = 2;
pub const COMP_LT: comparison_type = 1;
pub const COMP_GT: comparison_type = 0;
pub type xval = libc::c_uint;
pub const XVAL_TIME: xval = 4;
pub const XVAL_MTIME: xval = 3;
pub const XVAL_CTIME: xval = 2;
pub const XVAL_BIRTHTIME: xval = 1;
pub const XVAL_ATIME: xval = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct size_val {
    pub kind: comparison_type,
    pub blocksize: libc::c_int,
    pub size: uintmax_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct long_val {
    pub kind: comparison_type,
    pub negative: bool,
    pub l_val: uintmax_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exec_val {
    pub multiple: bool,
    pub ctl: buildcmd_control,
    pub state: buildcmd_state,
    pub replace_vec: *mut *mut libc::c_char,
    pub num_args: libc::c_int,
    pub close_stdin: bool,
    pub wd_for_exec: *mut saved_cwd,
    pub last_child_status: libc::c_int,
}
pub type EvaluationCost = libc::c_uint;
pub const NumEvaluationCosts: EvaluationCost = 11;
pub const NeedsUnknown: EvaluationCost = 10;
pub const NeedsUserInteraction: EvaluationCost = 9;
pub const NeedsImmediateExec: EvaluationCost = 8;
pub const NeedsEventualExec: EvaluationCost = 7;
pub const NeedsSyncDiskHit: EvaluationCost = 6;
pub const NeedsAccessInfo: EvaluationCost = 5;
pub const NeedsLinkName: EvaluationCost = 4;
pub const NeedsStatInfo: EvaluationCost = 3;
pub const NeedsType: EvaluationCost = 2;
pub const NeedsInodeNumber: EvaluationCost = 1;
pub const NeedsNothing: EvaluationCost = 0;
pub type predicate_precedence = libc::c_uint;
pub const MAX_PREC: predicate_precedence = 5;
pub const NEGATE_PREC: predicate_precedence = 4;
pub const AND_PREC: predicate_precedence = 3;
pub const OR_PREC: predicate_precedence = 2;
pub const COMMA_PREC: predicate_precedence = 1;
pub const NO_PREC: predicate_precedence = 0;
pub type predicate_type = libc::c_uint;
pub const CLOSE_PAREN: predicate_type = 5;
pub const OPEN_PAREN: predicate_type = 4;
pub const BI_OP: predicate_type = 3;
pub const UNI_OP: predicate_type = 2;
pub const PRIMARY_TYPE: predicate_type = 1;
pub const NO_TYPE: predicate_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options {
    pub do_dir_first: bool,
    pub explicit_depth: bool,
    pub maxdepth: libc::c_int,
    pub mindepth: libc::c_int,
    pub no_leaf_check: bool,
    pub stay_on_filesystem: bool,
    pub ignore_readdir_race: bool,
    pub literal_control_chars: bool,
    pub warnings: bool,
    pub posixly_correct: bool,
    pub start_time: timespec,
    pub cur_day_start: timespec,
    pub full_days: bool,
    pub output_block_size: libc::c_int,
    pub debug_options: libc::c_ulong,
    pub symlink_handling: SymlinkOption,
    pub xstat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
    pub open_nofollow_available: bool,
    pub regex_options: libc::c_int,
    pub x_getfilecon: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_char,
            *mut *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub optimisation_level: libc::c_ushort,
    pub err_quoting_style: quoting_style,
    pub files0_from: *const libc::c_char,
    pub ok_prompt_stdin: bool,
}
pub type SymlinkOption = libc::c_uint;
pub const SYMLINK_DEREF_ARGSONLY: SymlinkOption = 2;
pub const SYMLINK_ALWAYS_DEREF: SymlinkOption = 1;
pub const SYMLINK_NEVER_DEREF: SymlinkOption = 0;
pub type PREDICATEFUNCTION = unsafe extern "C" fn(
    *const libc::c_char,
    *mut stat,
    *mut predicate,
) -> bool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state {
    pub curdepth: libc::c_int,
    pub have_stat: bool,
    pub have_type: bool,
    pub type_0: mode_t,
    pub rel_pathname: *const libc::c_char,
    pub cwd_dir_fd: libc::c_int,
    pub starting_path_length: libc::c_int,
    pub stop_at_current_level: bool,
    pub exit_status: libc::c_int,
    pub execdirs_outstanding: bool,
    pub shared_files: sharefile_handle,
    pub already_issued_stat_error_msg: bool,
}
pub const NS_BUF_LEN: C2RustUnnamed_3 = 32;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const DATE_LEN_PERCENT_APLUS: C2RustUnnamed_3 = 21;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _gl_dummy: libc::c_int,
}
#[inline]
unsafe extern "C" fn fstatat(
    mut __fd: libc::c_int,
    mut __filename: *const libc::c_char,
    mut __statbuf: *mut stat,
    mut __flag: libc::c_int,
) -> libc::c_int {
    return __fxstatat(1 as libc::c_int, __fd, __filename, __statbuf, __flag);
}
#[inline]
unsafe extern "C" fn get_stat_atime(mut st: *const stat) -> timespec {
    return (*st).st_atim;
}
#[inline]
unsafe extern "C" fn get_stat_ctime(mut st: *const stat) -> timespec {
    return (*st).st_ctim;
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
#[inline]
unsafe extern "C" fn get_stat_birthtime(mut st: *const stat) -> timespec {
    let mut t: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    t.tv_sec = -(1 as libc::c_int) as __time_t;
    t.tv_nsec = -(1 as libc::c_int) as __syscall_slong_t;
    return t;
}
pub unsafe extern "C" fn make_segment(
    mut segment: *mut *mut segment,
    mut format: *mut libc::c_char,
    mut len: libc::c_int,
    mut kind: libc::c_int,
    mut format_char: libc::c_char,
    mut aux_format_char: libc::c_char,
    mut pred: *mut predicate,
) -> *mut *mut segment {
    let mut mycost: EvaluationCost = NeedsNothing;
    let mut fmt: *mut libc::c_char = 0 as *mut libc::c_char;
    if format_char as libc::c_int != '{' as i32 {} else {
        __assert_fail(
            b"format_char != '{'\0" as *const u8 as *const libc::c_char,
            b"print.c\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 99],
                &[libc::c_char; 99],
            >(
                b"struct segment **make_segment(struct segment **, char *, int, int, char, char, struct predicate *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_15557: {
        if format_char as libc::c_int != '{' as i32 {} else {
            __assert_fail(
                b"format_char != '{'\0" as *const u8 as *const libc::c_char,
                b"print.c\0" as *const u8 as *const libc::c_char,
                78 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 99],
                    &[libc::c_char; 99],
                >(
                    b"struct segment **make_segment(struct segment **, char *, int, int, char, char, struct predicate *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if format_char as libc::c_int != '[' as i32 {} else {
        __assert_fail(
            b"format_char != '['\0" as *const u8 as *const libc::c_char,
            b"print.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 99],
                &[libc::c_char; 99],
            >(
                b"struct segment **make_segment(struct segment **, char *, int, int, char, char, struct predicate *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_15519: {
        if format_char as libc::c_int != '[' as i32 {} else {
            __assert_fail(
                b"format_char != '['\0" as *const u8 as *const libc::c_char,
                b"print.c\0" as *const u8 as *const libc::c_char,
                79 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 99],
                    &[libc::c_char; 99],
                >(
                    b"struct segment **make_segment(struct segment **, char *, int, int, char, char, struct predicate *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if format_char as libc::c_int != '(' as i32 {} else {
        __assert_fail(
            b"format_char != '('\0" as *const u8 as *const libc::c_char,
            b"print.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 99],
                &[libc::c_char; 99],
            >(
                b"struct segment **make_segment(struct segment **, char *, int, int, char, char, struct predicate *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_15480: {
        if format_char as libc::c_int != '(' as i32 {} else {
            __assert_fail(
                b"format_char != '('\0" as *const u8 as *const libc::c_char,
                b"print.c\0" as *const u8 as *const libc::c_char,
                80 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 99],
                    &[libc::c_char; 99],
                >(
                    b"struct segment **make_segment(struct segment **, char *, int, int, char, char, struct predicate *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    *segment = xmalloc(::std::mem::size_of::<segment>() as libc::c_ulong)
        as *mut segment;
    (**segment).segkind = kind as SegmentKind;
    (**segment).format_char[0 as libc::c_int as usize] = format_char;
    (**segment).format_char[1 as libc::c_int as usize] = aux_format_char;
    (**segment).next = 0 as *mut segment;
    (**segment).text_len = len;
    (**segment)
        .text = xmalloc(
        (len as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong),
    ) as *mut libc::c_char;
    fmt = (**segment).text;
    strncpy(fmt, format, len as libc::c_ulong);
    fmt = fmt.offset(len as isize);
    if kind == KIND_PLAIN as libc::c_int || kind == KIND_STOP as libc::c_int {
        if 0 as libc::c_int == format_char as libc::c_int {} else {
            __assert_fail(
                b"0 == format_char\0" as *const u8 as *const libc::c_char,
                b"print.c\0" as *const u8 as *const libc::c_char,
                97 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 99],
                    &[libc::c_char; 99],
                >(
                    b"struct segment **make_segment(struct segment **, char *, int, int, char, char, struct predicate *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_15336: {
            if 0 as libc::c_int == format_char as libc::c_int {} else {
                __assert_fail(
                    b"0 == format_char\0" as *const u8 as *const libc::c_char,
                    b"print.c\0" as *const u8 as *const libc::c_char,
                    97 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 99],
                        &[libc::c_char; 99],
                    >(
                        b"struct segment **make_segment(struct segment **, char *, int, int, char, char, struct predicate *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if 0 as libc::c_int == aux_format_char as libc::c_int {} else {
            __assert_fail(
                b"0 == aux_format_char\0" as *const u8 as *const libc::c_char,
                b"print.c\0" as *const u8 as *const libc::c_char,
                98 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 99],
                    &[libc::c_char; 99],
                >(
                    b"struct segment **make_segment(struct segment **, char *, int, int, char, char, struct predicate *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_15297: {
            if 0 as libc::c_int == aux_format_char as libc::c_int {} else {
                __assert_fail(
                    b"0 == aux_format_char\0" as *const u8 as *const libc::c_char,
                    b"print.c\0" as *const u8 as *const libc::c_char,
                    98 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 99],
                        &[libc::c_char; 99],
                    >(
                        b"struct segment **make_segment(struct segment **, char *, int, int, char, char, struct predicate *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        *fmt = '\0' as i32 as libc::c_char;
        if mycost as libc::c_uint > (*pred).p_cost as libc::c_uint {
            (*pred).p_cost = NeedsNothing;
        }
        return &mut (**segment).next;
    }
    if kind == KIND_FORMAT as libc::c_int {} else {
        __assert_fail(
            b"kind == KIND_FORMAT\0" as *const u8 as *const libc::c_char,
            b"print.c\0" as *const u8 as *const libc::c_char,
            105 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 99],
                &[libc::c_char; 99],
            >(
                b"struct segment **make_segment(struct segment **, char *, int, int, char, char, struct predicate *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_15224: {
        if kind == KIND_FORMAT as libc::c_int {} else {
            __assert_fail(
                b"kind == KIND_FORMAT\0" as *const u8 as *const libc::c_char,
                b"print.c\0" as *const u8 as *const libc::c_char,
                105 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 99],
                    &[libc::c_char; 99],
                >(
                    b"struct segment **make_segment(struct segment **, char *, int, int, char, char, struct predicate *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut current_block_50: u64;
    match format_char as libc::c_int {
        37 => {
            let fresh0 = fmt;
            fmt = fmt.offset(1);
            *fresh0 = '%' as i32 as libc::c_char;
            current_block_50 = 5891011138178424807;
        }
        108 => {
            (*pred).need_stat = 1 as libc::c_int != 0;
            mycost = NeedsLinkName;
            let fresh1 = fmt;
            fmt = fmt.offset(1);
            *fresh1 = 's' as i32 as libc::c_char;
            current_block_50 = 5891011138178424807;
        }
        121 => {
            (*pred).need_type = 1 as libc::c_int != 0;
            mycost = NeedsType;
            let fresh2 = fmt;
            fmt = fmt.offset(1);
            *fresh2 = 's' as i32 as libc::c_char;
            current_block_50 = 5891011138178424807;
        }
        105 => {
            (*pred).need_inum = 1 as libc::c_int != 0;
            mycost = NeedsInodeNumber;
            let fresh3 = fmt;
            fmt = fmt.offset(1);
            *fresh3 = 's' as i32 as libc::c_char;
            current_block_50 = 5891011138178424807;
        }
        97 => {
            current_block_50 = 4822005742776393872;
        }
        65 => {
            current_block_50 = 4822005742776393872;
        }
        66 => {
            current_block_50 = 18131238365012881386;
        }
        99 => {
            current_block_50 = 10113934524333008641;
        }
        67 => {
            current_block_50 = 5497192158704825631;
        }
        70 => {
            current_block_50 = 16978346248683711281;
        }
        103 => {
            current_block_50 = 12956189135376345949;
        }
        77 => {
            current_block_50 = 13181243827780635;
        }
        115 => {
            current_block_50 = 14115841774397495319;
        }
        116 => {
            current_block_50 = 2187311453507916348;
        }
        84 | 117 => {
            current_block_50 = 1338086771676148209;
        }
        83 => {
            (*pred).need_stat = 1 as libc::c_int != 0;
            mycost = NeedsStatInfo;
            let fresh5 = fmt;
            fmt = fmt.offset(1);
            *fresh5 = 'g' as i32 as libc::c_char;
            current_block_50 = 5891011138178424807;
        }
        89 => {
            (*pred).need_stat = 1 as libc::c_int != 0;
            mycost = NeedsType;
            let fresh6 = fmt;
            fmt = fmt.offset(1);
            *fresh6 = 's' as i32 as libc::c_char;
            current_block_50 = 5891011138178424807;
        }
        102 => {
            current_block_50 = 17221946530509661022;
        }
        104 => {
            current_block_50 = 17221946530509661022;
        }
        112 | 80 => {
            current_block_50 = 15328589962203645498;
        }
        90 => {
            mycost = NeedsAccessInfo;
            let fresh8 = fmt;
            fmt = fmt.offset(1);
            *fresh8 = 's' as i32 as libc::c_char;
            current_block_50 = 5891011138178424807;
        }
        72 => {
            let fresh9 = fmt;
            fmt = fmt.offset(1);
            *fresh9 = 's' as i32 as libc::c_char;
            current_block_50 = 5891011138178424807;
        }
        71 => {
            current_block_50 = 7993423040468902026;
        }
        85 => {
            current_block_50 = 7993423040468902026;
        }
        98 => {
            current_block_50 = 322004148427622879;
        }
        68 => {
            current_block_50 = 1004782861525159561;
        }
        107 | 110 => {
            current_block_50 = 17984004858154919059;
        }
        100 => {
            let fresh11 = fmt;
            fmt = fmt.offset(1);
            *fresh11 = 'd' as i32 as libc::c_char;
            current_block_50 = 5891011138178424807;
        }
        109 => {
            let fresh12 = fmt;
            fmt = fmt.offset(1);
            *fresh12 = 'o' as i32 as libc::c_char;
            (*pred).need_stat = 1 as libc::c_int != 0;
            mycost = NeedsStatInfo;
            current_block_50 = 5891011138178424807;
        }
        _ => {
            current_block_50 = 5891011138178424807;
        }
    }
    match current_block_50 {
        4822005742776393872 => {
            current_block_50 = 18131238365012881386;
        }
        17221946530509661022 => {
            current_block_50 = 15328589962203645498;
        }
        7993423040468902026 => {
            current_block_50 = 322004148427622879;
        }
        _ => {}
    }
    match current_block_50 {
        18131238365012881386 => {
            current_block_50 = 10113934524333008641;
        }
        15328589962203645498 => {
            let fresh7 = fmt;
            fmt = fmt.offset(1);
            *fresh7 = 's' as i32 as libc::c_char;
            current_block_50 = 5891011138178424807;
        }
        322004148427622879 => {
            current_block_50 = 1004782861525159561;
        }
        _ => {}
    }
    match current_block_50 {
        10113934524333008641 => {
            current_block_50 = 5497192158704825631;
        }
        1004782861525159561 => {
            current_block_50 = 17984004858154919059;
        }
        _ => {}
    }
    match current_block_50 {
        17984004858154919059 => {
            (*pred).need_stat = 1 as libc::c_int != 0;
            mycost = NeedsStatInfo;
            let fresh10 = fmt;
            fmt = fmt.offset(1);
            *fresh10 = 's' as i32 as libc::c_char;
            current_block_50 = 5891011138178424807;
        }
        5497192158704825631 => {
            current_block_50 = 16978346248683711281;
        }
        _ => {}
    }
    match current_block_50 {
        16978346248683711281 => {
            current_block_50 = 12956189135376345949;
        }
        _ => {}
    }
    match current_block_50 {
        12956189135376345949 => {
            current_block_50 = 13181243827780635;
        }
        _ => {}
    }
    match current_block_50 {
        13181243827780635 => {
            current_block_50 = 14115841774397495319;
        }
        _ => {}
    }
    match current_block_50 {
        14115841774397495319 => {
            current_block_50 = 2187311453507916348;
        }
        _ => {}
    }
    match current_block_50 {
        2187311453507916348 => {
            current_block_50 = 1338086771676148209;
        }
        _ => {}
    }
    match current_block_50 {
        1338086771676148209 => {
            (*pred).need_stat = 1 as libc::c_int != 0;
            mycost = NeedsStatInfo;
            let fresh4 = fmt;
            fmt = fmt.offset(1);
            *fresh4 = 's' as i32 as libc::c_char;
        }
        _ => {}
    }
    *fmt = '\0' as i32 as libc::c_char;
    if mycost as libc::c_uint > (*pred).p_cost as libc::c_uint {
        (*pred).p_cost = mycost;
    }
    return &mut (**segment).next;
}
unsafe extern "C" fn is_octal_char(mut ch: libc::c_char) -> bool {
    return ch as libc::c_int >= '0' as i32 && ch as libc::c_int <= '7' as i32;
}
unsafe extern "C" fn parse_octal_escape(
    mut p: *const libc::c_char,
    mut consumed: *mut size_t,
) -> libc::c_char {
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    n = 0 as libc::c_int;
    i = n;
    while i < 3 as libc::c_int
        && is_octal_char(*p.offset(pos as isize)) as libc::c_int != 0
    {
        n = 8 as libc::c_int * n + *p.offset(pos as isize) as libc::c_int - '0' as i32;
        i += 1;
        i;
        pos = pos.wrapping_add(1);
        pos;
    }
    pos = pos.wrapping_sub(1);
    pos;
    *consumed = pos;
    return n as libc::c_char;
}
unsafe extern "C" fn parse_escape_char(ch: libc::c_char) -> libc::c_int {
    let mut value: libc::c_char = 0 as libc::c_int as libc::c_char;
    match ch as libc::c_int {
        97 => {
            value = '\u{7}' as i32 as libc::c_char;
        }
        98 => {
            value = '\u{8}' as i32 as libc::c_char;
        }
        102 => {
            value = '\u{c}' as i32 as libc::c_char;
        }
        110 => {
            value = '\n' as i32 as libc::c_char;
        }
        114 => {
            value = '\r' as i32 as libc::c_char;
        }
        116 => {
            value = '\t' as i32 as libc::c_char;
        }
        118 => {
            value = '\u{b}' as i32 as libc::c_char;
        }
        92 => {
            value = '\\' as i32 as libc::c_char;
        }
        _ => {}
    }
    return value as libc::c_int;
}
unsafe extern "C" fn get_format_flags_length(mut p: *const libc::c_char) -> size_t {
    let mut n: size_t = 0 as libc::c_int as size_t;
    loop {
        n = n.wrapping_add(1);
        if !(*p.offset(n as isize) as libc::c_int != 0
            && !(strchr(
                b"-+ #\0" as *const u8 as *const libc::c_char,
                *p.offset(n as isize) as libc::c_int,
            ))
                .is_null())
        {
            break;
        }
    }
    while *(*__ctype_b_loc())
        .offset(*p.offset(n as isize) as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        n = n.wrapping_add(1);
        n;
    }
    if *p.offset(n as isize) as libc::c_int == '.' as i32 {
        n = n.wrapping_add(1);
        n;
        while *(*__ctype_b_loc())
            .offset(*p.offset(n as isize) as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            n = n.wrapping_add(1);
            n;
        }
    }
    return n;
}
unsafe extern "C" fn get_format_specifer_length(mut ch: libc::c_char) -> size_t {
    if !(strchr(
        b"abcdDfFgGhHiklmMnpPsStuUyYZ%\0" as *const u8 as *const libc::c_char,
        ch as libc::c_int,
    ))
        .is_null()
    {
        return 1 as libc::c_int as size_t
    } else if !(strchr(b"ABCT\0" as *const u8 as *const libc::c_char, ch as libc::c_int))
        .is_null()
    {
        return 2 as libc::c_int as size_t
    } else {
        return 0 as libc::c_int as size_t
    };
}
pub unsafe extern "C" fn insert_fprintf(
    mut vec: *mut format_val,
    mut entry: *const parser_table,
    mut format: *mut libc::c_char,
) -> bool {
    let mut segstart: *mut libc::c_char = format;
    let mut fmt_editpos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut segmentp: *mut *mut segment = 0 as *mut *mut segment;
    let mut our_pred: *mut predicate = 0 as *mut predicate;
    our_pred = insert_primary_withpred(
        entry,
        Some(pred_fprintf as PREDICATEFUNCTION),
        format,
    );
    (*our_pred).no_default_print = 1 as libc::c_int != 0;
    (*our_pred).side_effects = (*our_pred).no_default_print;
    (*our_pred).args.printf_vec = *vec;
    (*our_pred).need_type = 0 as libc::c_int != 0;
    (*our_pred).need_stat = 0 as libc::c_int != 0;
    (*our_pred).p_cost = NeedsNothing;
    segmentp = &mut (*our_pred).args.printf_vec.segment;
    *segmentp = 0 as *mut segment;
    let mut current_block_51: u64;
    fmt_editpos = segstart;
    while *fmt_editpos != 0 {
        if *fmt_editpos.offset(0 as libc::c_int as isize) as libc::c_int == '\\' as i32
            && *fmt_editpos.offset(1 as libc::c_int as isize) as libc::c_int
                == 'c' as i32
        {
            make_segment(
                segmentp,
                segstart,
                fmt_editpos.offset_from(segstart) as libc::c_long as libc::c_int,
                KIND_STOP as libc::c_int,
                0 as libc::c_int as libc::c_char,
                0 as libc::c_int as libc::c_char,
                our_pred,
            );
            if (*our_pred).need_stat as libc::c_int != 0
                && ((*our_pred).p_cost as libc::c_uint)
                    < NeedsStatInfo as libc::c_int as libc::c_uint
            {
                (*our_pred).p_cost = NeedsStatInfo;
            }
            return 1 as libc::c_int != 0;
        } else {
            if *fmt_editpos as libc::c_int == '\\' as i32 {
                let mut readpos: size_t = 1 as libc::c_int as size_t;
                if *fmt_editpos.offset(readpos as isize) == 0 {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"warning: escape `\\' followed by nothing at all\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    readpos = readpos.wrapping_sub(1);
                    readpos;
                    current_block_51 = 17478428563724192186;
                } else if is_octal_char(*fmt_editpos.offset(readpos as isize)) {
                    let mut consumed: size_t = 0 as libc::c_int as size_t;
                    *fmt_editpos = parse_octal_escape(
                        fmt_editpos.offset(readpos as isize),
                        &mut consumed,
                    );
                    readpos = (readpos as libc::c_ulong).wrapping_add(consumed) as size_t
                        as size_t;
                    current_block_51 = 17478428563724192186;
                } else {
                    let val: libc::c_char = parse_escape_char(
                        *fmt_editpos.offset(readpos as isize),
                    ) as libc::c_char;
                    if val != 0 {
                        *fmt_editpos.offset(0 as libc::c_int as isize) = val;
                        current_block_51 = 17478428563724192186;
                    } else {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"warning: unrecognized escape `\\%c'\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            *fmt_editpos.offset(readpos as isize) as libc::c_int,
                        );
                        fmt_editpos = fmt_editpos.offset(readpos as isize);
                        current_block_51 = 10879442775620481940;
                    }
                }
                match current_block_51 {
                    10879442775620481940 => {}
                    _ => {
                        segmentp = make_segment(
                            segmentp,
                            segstart,
                            (fmt_editpos.offset_from(segstart) as libc::c_long
                                + 1 as libc::c_int as libc::c_long) as libc::c_int,
                            KIND_PLAIN as libc::c_int,
                            0 as libc::c_int as libc::c_char,
                            0 as libc::c_int as libc::c_char,
                            our_pred,
                        );
                        segstart = fmt_editpos
                            .offset(readpos as isize)
                            .offset(1 as libc::c_int as isize);
                        fmt_editpos = fmt_editpos.offset(readpos as isize);
                    }
                }
            } else if *fmt_editpos.offset(0 as libc::c_int as isize) as libc::c_int
                == '%' as i32
            {
                let mut len: size_t = 0;
                if *fmt_editpos.offset(1 as libc::c_int as isize) as libc::c_int
                    == 0 as libc::c_int
                {
                    if ::std::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"error: %s at end of format string\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            fmt_editpos,
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"error: %s at end of format string\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            fmt_editpos,
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                if *fmt_editpos.offset(1 as libc::c_int as isize) as libc::c_int
                    == '%' as i32
                {
                    len = 1 as libc::c_int as size_t;
                } else {
                    len = get_format_flags_length(fmt_editpos);
                }
                fmt_editpos = fmt_editpos.offset(len as isize);
                len = get_format_specifer_length(
                    *fmt_editpos.offset(0 as libc::c_int as isize),
                );
                if len != 0
                    && *fmt_editpos
                        .offset(
                            len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int != 0
                {
                    let fmt2: libc::c_char = (if len == 2 as libc::c_int as libc::c_ulong
                    {
                        *fmt_editpos.offset(1 as libc::c_int as isize) as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as libc::c_char;
                    segmentp = make_segment(
                        segmentp,
                        segstart,
                        fmt_editpos.offset_from(segstart) as libc::c_long as libc::c_int,
                        KIND_FORMAT as libc::c_int,
                        *fmt_editpos.offset(0 as libc::c_int as isize),
                        fmt2,
                        our_pred,
                    );
                    fmt_editpos = fmt_editpos
                        .offset(
                            len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                } else {
                    if !(strchr(
                        b"{[(\0" as *const u8 as *const libc::c_char,
                        *fmt_editpos.offset(0 as libc::c_int as isize) as libc::c_int,
                    ))
                        .is_null()
                    {
                        if ::std::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0
                        {
                            error(
                                1 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"error: the format directive `%%%c' is reserved for future use\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                *fmt_editpos.offset(0 as libc::c_int as isize)
                                    as libc::c_int,
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        } else {
                            error(
                                1 as libc::c_int,
                                0 as libc::c_int,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"error: the format directive `%%%c' is reserved for future use\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                *fmt_editpos.offset(0 as libc::c_int as isize)
                                    as libc::c_int,
                            );
                            if 0 as libc::c_int != 0 {} else {
                                unreachable!();
                            };
                        };
                    }
                    if len == 2 as libc::c_int as libc::c_ulong
                        && *fmt_editpos.offset(1 as libc::c_int as isize) == 0
                    {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"warning: format directive `%%%c' should be followed by another character\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            *fmt_editpos.offset(0 as libc::c_int as isize) as libc::c_int,
                        );
                    } else {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"warning: unrecognized format directive `%%%c'\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            *fmt_editpos.offset(0 as libc::c_int as isize) as libc::c_int,
                        );
                    }
                    segmentp = make_segment(
                        segmentp,
                        segstart,
                        fmt_editpos
                            .offset(1 as libc::c_int as isize)
                            .offset_from(segstart) as libc::c_long as libc::c_int,
                        KIND_PLAIN as libc::c_int,
                        0 as libc::c_int as libc::c_char,
                        0 as libc::c_int as libc::c_char,
                        our_pred,
                    );
                }
                segstart = fmt_editpos.offset(1 as libc::c_int as isize);
            }
            fmt_editpos = fmt_editpos.offset(1);
            fmt_editpos;
        }
    }
    if fmt_editpos > segstart {
        make_segment(
            segmentp,
            segstart,
            fmt_editpos.offset_from(segstart) as libc::c_long as libc::c_int,
            KIND_PLAIN as libc::c_int,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            our_pred,
        );
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn scan_for_digit_differences(
    mut p: *const libc::c_char,
    mut q: *const libc::c_char,
    mut first: *mut size_t,
    mut n: *mut size_t,
) -> bool {
    let mut seen: bool = 0 as libc::c_int != 0;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while *p.offset(i as isize) as libc::c_int != 0
        && *q.offset(i as isize) as libc::c_int != 0
    {
        if *p.offset(i as isize) as libc::c_int != *q.offset(i as isize) as libc::c_int {
            if *(*__ctype_b_loc())
                .offset(*p.offset(i as isize) as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                == 0
                || *(*__ctype_b_loc())
                    .offset(
                        *q.offset(i as isize) as libc::c_uchar as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                return 0 as libc::c_int != 0;
            }
            if !seen {
                *first = i;
                *n = 1 as libc::c_int as size_t;
                seen = 1 as libc::c_int != 0;
            } else if i.wrapping_sub(*first) == *n {
                *n = (*n).wrapping_add(1);
                *n;
            } else {
                return 0 as libc::c_int != 0
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if *p.offset(i as isize) as libc::c_int != 0
        || *q.offset(i as isize) as libc::c_int != 0
    {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn do_time_format(
    mut fmt: *const libc::c_char,
    mut p: *const tm,
    mut ns: *const libc::c_char,
    mut ns_size: size_t,
) -> *mut libc::c_char {
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut buf_size: size_t = 0;
    let mut timefmt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut altered_time: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    timefmt = xmalloc((strlen(fmt)).wrapping_add(2 as libc::c_uint as libc::c_ulong))
        as *mut libc::c_char;
    *timefmt.offset(0 as libc::c_int as isize) = '_' as i32 as libc::c_char;
    memcpy(
        timefmt.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        fmt as *const libc::c_void,
        (strlen(fmt)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    altered_time = *p;
    if altered_time.tm_sec >= 11 as libc::c_int {
        altered_time.tm_sec -= 11 as libc::c_int;
    } else {
        altered_time.tm_sec += 11 as libc::c_int;
    }
    if buf.is_null() {
        buf_size = 1 as libc::c_uint as size_t;
        buf = xmalloc(buf_size) as *mut libc::c_char;
    }
    loop {
        let mut buf_used: size_t = strftime(buf, buf_size, timefmt, p);
        if buf_used != 0 && buf_used < buf_size {
            let mut altbuf: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut i: size_t = 0 as libc::c_int as size_t;
            let mut n: size_t = 0 as libc::c_int as size_t;
            let mut final_len: size_t = buf_used
                .wrapping_add(1 as libc::c_uint as libc::c_ulong)
                .wrapping_add(ns_size);
            buf = xrealloc(buf as *mut libc::c_void, final_len) as *mut libc::c_char;
            buf_size = final_len;
            altbuf = xmalloc(final_len) as *mut libc::c_char;
            strftime(altbuf, buf_size, timefmt, &mut altered_time);
            if scan_for_digit_differences(buf, altbuf, &mut i, &mut n) as libc::c_int
                != 0 && 2 as libc::c_int as libc::c_ulong == n
                && *(*__ctype_b_loc())
                    .offset(
                        *buf.offset(i.wrapping_add(n) as isize) as libc::c_uchar
                            as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                let end_of_seconds: size_t = i.wrapping_add(n);
                let suffix_len: size_t = buf_used
                    .wrapping_sub(end_of_seconds)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                if end_of_seconds.wrapping_add(ns_size).wrapping_add(suffix_len)
                    == final_len
                {} else {
                    __assert_fail(
                        b"end_of_seconds + ns_size + suffix_len == final_len\0"
                            as *const u8 as *const libc::c_char,
                        b"print.c\0" as *const u8 as *const libc::c_char,
                        549 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 76],
                            &[libc::c_char; 76],
                        >(
                            b"char *do_time_format(const char *, const struct tm *, const char *, size_t)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_11937: {
                    if end_of_seconds.wrapping_add(ns_size).wrapping_add(suffix_len)
                        == final_len
                    {} else {
                        __assert_fail(
                            b"end_of_seconds + ns_size + suffix_len == final_len\0"
                                as *const u8 as *const libc::c_char,
                            b"print.c\0" as *const u8 as *const libc::c_char,
                            549 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 76],
                                &[libc::c_char; 76],
                            >(
                                b"char *do_time_format(const char *, const struct tm *, const char *, size_t)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                memmove(
                    buf.offset(end_of_seconds as isize).offset(ns_size as isize)
                        as *mut libc::c_void,
                    buf.offset(end_of_seconds as isize) as *const libc::c_void,
                    suffix_len,
                );
                memcpy(
                    buf.offset(i as isize).offset(n as isize) as *mut libc::c_void,
                    ns as *const libc::c_void,
                    ns_size,
                );
            }
            rpl_free(timefmt as *mut libc::c_void);
            rpl_free(altbuf as *mut libc::c_void);
            return buf.offset(1 as libc::c_int as isize);
        } else {
            buf = x2nrealloc(
                buf as *mut libc::c_void,
                &mut buf_size,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
            ) as *mut libc::c_char;
        }
    };
}
unsafe extern "C" fn format_date(
    mut ts: timespec,
    mut kind: libc::c_int,
) -> *mut libc::c_char {
    static mut buf: [libc::c_char; 791] = [0; 791];
    let mut ns_buf: [libc::c_char; 32] = [0; 32];
    let mut charsprinted: libc::c_int = 0;
    let mut need_ns_suffix: libc::c_int = 0;
    let mut tm: *mut tm = 0 as *mut tm;
    let mut fmt: [libc::c_char; 12] = [0; 12];
    charsprinted = 0 as libc::c_int;
    need_ns_suffix = 0 as libc::c_int;
    if kind == '+' as i32 {
        strcpy(fmt.as_mut_ptr(), b"%Y-%m-%d+%T\0" as *const u8 as *const libc::c_char);
        need_ns_suffix = 1 as libc::c_int;
    } else {
        fmt[0 as libc::c_int as usize] = '%' as i32 as libc::c_char;
        fmt[1 as libc::c_int as usize] = kind as libc::c_char;
        fmt[2 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        match kind {
            83 | 84 | 88 | 64 => {
                need_ns_suffix = 1 as libc::c_int;
            }
            _ => {
                need_ns_suffix = 0 as libc::c_int;
            }
        }
    }
    if need_ns_suffix != 0 {
        charsprinted = snprintf(
            ns_buf.as_mut_ptr(),
            NS_BUF_LEN as libc::c_int as libc::c_ulong,
            b".%09ld0\0" as *const u8 as *const libc::c_char,
            ts.tv_nsec,
        );
        if charsprinted < NS_BUF_LEN as libc::c_int {} else {
            __assert_fail(
                b"charsprinted < NS_BUF_LEN\0" as *const u8 as *const libc::c_char,
                b"print.c\0" as *const u8 as *const libc::c_char,
                665 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"char *format_date(struct timespec, int)\0"))
                    .as_ptr(),
            );
        }
        'c_12396: {
            if charsprinted < NS_BUF_LEN as libc::c_int {} else {
                __assert_fail(
                    b"charsprinted < NS_BUF_LEN\0" as *const u8 as *const libc::c_char,
                    b"print.c\0" as *const u8 as *const libc::c_char,
                    665 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 40],
                        &[libc::c_char; 40],
                    >(b"char *format_date(struct timespec, int)\0"))
                        .as_ptr(),
                );
            }
        };
    } else {
        charsprinted = 0 as libc::c_int;
        ns_buf[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    }
    if kind != '@' as i32 {
        tm = localtime(&mut ts.tv_sec);
        if !tm.is_null() {
            let mut s: *mut libc::c_char = do_time_format(
                fmt.as_mut_ptr(),
                tm,
                ns_buf.as_mut_ptr(),
                charsprinted as size_t,
            );
            if !s.is_null() {
                return s;
            }
        }
    }
    let mut w: uintmax_t = ts.tv_sec as uintmax_t;
    let mut used: size_t = 0;
    let mut len: size_t = 0;
    let mut remaining: size_t = 0;
    let mut p: *mut libc::c_char = human_readable(
        if ts.tv_sec < 0 as libc::c_int as libc::c_long { w.wrapping_neg() } else { w },
        buf.as_mut_ptr().offset(1 as libc::c_int as isize),
        human_ceiling as libc::c_int,
        1 as libc::c_int as uintmax_t,
        1 as libc::c_int as uintmax_t,
    );
    if p > buf.as_mut_ptr() {} else {
        __assert_fail(
            b"p > buf\0" as *const u8 as *const libc::c_char,
            b"print.c\0" as *const u8 as *const libc::c_char,
            697 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"char *format_date(struct timespec, int)\0"))
                .as_ptr(),
        );
    }
    'c_11702: {
        if p > buf.as_mut_ptr() {} else {
            __assert_fail(
                b"p > buf\0" as *const u8 as *const libc::c_char,
                b"print.c\0" as *const u8 as *const libc::c_char,
                697 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"char *format_date(struct timespec, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if p
        < buf
            .as_mut_ptr()
            .offset(
                ::std::mem::size_of::<[libc::c_char; 791]>() as libc::c_ulong as isize,
            )
    {} else {
        __assert_fail(
            b"p < (buf + (sizeof buf))\0" as *const u8 as *const libc::c_char,
            b"print.c\0" as *const u8 as *const libc::c_char,
            698 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"char *format_date(struct timespec, int)\0"))
                .as_ptr(),
        );
    }
    'c_11653: {
        if p
            < buf
                .as_mut_ptr()
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 791]>() as libc::c_ulong
                        as isize,
                )
        {} else {
            __assert_fail(
                b"p < (buf + (sizeof buf))\0" as *const u8 as *const libc::c_char,
                b"print.c\0" as *const u8 as *const libc::c_char,
                698 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"char *format_date(struct timespec, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if ts.tv_sec < 0 as libc::c_int as libc::c_long {
        p = p.offset(-1);
        *p = '-' as i32 as libc::c_char;
    }
    if need_ns_suffix != 0 {
        len = strlen(p);
        used = (p.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_ulong)
            .wrapping_add(len);
        if ::std::mem::size_of::<[libc::c_char; 791]>() as libc::c_ulong > used {} else {
            __assert_fail(
                b"sizeof buf > used\0" as *const u8 as *const libc::c_char,
                b"print.c\0" as *const u8 as *const libc::c_char,
                711 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"char *format_date(struct timespec, int)\0"))
                    .as_ptr(),
            );
        }
        'c_11578: {
            if ::std::mem::size_of::<[libc::c_char; 791]>() as libc::c_ulong > used
            {} else {
                __assert_fail(
                    b"sizeof buf > used\0" as *const u8 as *const libc::c_char,
                    b"print.c\0" as *const u8 as *const libc::c_char,
                    711 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 40],
                        &[libc::c_char; 40],
                    >(b"char *format_date(struct timespec, int)\0"))
                        .as_ptr(),
                );
            }
        };
        remaining = (::std::mem::size_of::<[libc::c_char; 791]>() as libc::c_ulong)
            .wrapping_sub(used)
            .wrapping_sub(1 as libc::c_uint as libc::c_ulong);
        if strlen(ns_buf.as_mut_ptr()) >= remaining {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                b"charsprinted=%ld but remaining=%lu: ns_buf=%s\0" as *const u8
                    as *const libc::c_char,
                charsprinted as libc::c_long,
                remaining,
                ns_buf.as_mut_ptr(),
            );
        }
        if strlen(ns_buf.as_mut_ptr()) < remaining {} else {
            __assert_fail(
                b"strlen (ns_buf) < remaining\0" as *const u8 as *const libc::c_char,
                b"print.c\0" as *const u8 as *const libc::c_char,
                720 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"char *format_date(struct timespec, int)\0"))
                    .as_ptr(),
            );
        }
        'c_11487: {
            if strlen(ns_buf.as_mut_ptr()) < remaining {} else {
                __assert_fail(
                    b"strlen (ns_buf) < remaining\0" as *const u8 as *const libc::c_char,
                    b"print.c\0" as *const u8 as *const libc::c_char,
                    720 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 40],
                        &[libc::c_char; 40],
                    >(b"char *format_date(struct timespec, int)\0"))
                        .as_ptr(),
                );
            }
        };
        strcat(p, ns_buf.as_mut_ptr());
    }
    return p;
}
static mut weekdays: [*const libc::c_char; 7] = [
    b"Sun\0" as *const u8 as *const libc::c_char,
    b"Mon\0" as *const u8 as *const libc::c_char,
    b"Tue\0" as *const u8 as *const libc::c_char,
    b"Wed\0" as *const u8 as *const libc::c_char,
    b"Thu\0" as *const u8 as *const libc::c_char,
    b"Fri\0" as *const u8 as *const libc::c_char,
    b"Sat\0" as *const u8 as *const libc::c_char,
];
static mut months: [*const libc::c_char; 12] = [
    b"Jan\0" as *const u8 as *const libc::c_char,
    b"Feb\0" as *const u8 as *const libc::c_char,
    b"Mar\0" as *const u8 as *const libc::c_char,
    b"Apr\0" as *const u8 as *const libc::c_char,
    b"May\0" as *const u8 as *const libc::c_char,
    b"Jun\0" as *const u8 as *const libc::c_char,
    b"Jul\0" as *const u8 as *const libc::c_char,
    b"Aug\0" as *const u8 as *const libc::c_char,
    b"Sep\0" as *const u8 as *const libc::c_char,
    b"Oct\0" as *const u8 as *const libc::c_char,
    b"Nov\0" as *const u8 as *const libc::c_char,
    b"Dec\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn ctime_format(mut ts: timespec) -> *mut libc::c_char {
    let mut ptm: *const tm = 0 as *const tm;
    static mut resultbuf: [libc::c_char; 1024] = [0; 1024];
    let mut nout: libc::c_int = 0;
    ptm = localtime(&mut ts.tv_sec);
    if !ptm.is_null() {
        if (*ptm).tm_wday >= 0 as libc::c_int {} else {
            __assert_fail(
                b"ptm->tm_wday >= 0\0" as *const u8 as *const libc::c_char,
                b"print.c\0" as *const u8 as *const libc::c_char,
                749 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"char *ctime_format(struct timespec)\0"))
                    .as_ptr(),
            );
        }
        'c_12970: {
            if (*ptm).tm_wday >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"ptm->tm_wday >= 0\0" as *const u8 as *const libc::c_char,
                    b"print.c\0" as *const u8 as *const libc::c_char,
                    749 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 36],
                        &[libc::c_char; 36],
                    >(b"char *ctime_format(struct timespec)\0"))
                        .as_ptr(),
                );
            }
        };
        if (*ptm).tm_wday < 7 as libc::c_int {} else {
            __assert_fail(
                b"ptm->tm_wday < 7\0" as *const u8 as *const libc::c_char,
                b"print.c\0" as *const u8 as *const libc::c_char,
                750 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"char *ctime_format(struct timespec)\0"))
                    .as_ptr(),
            );
        }
        'c_12930: {
            if (*ptm).tm_wday < 7 as libc::c_int {} else {
                __assert_fail(
                    b"ptm->tm_wday < 7\0" as *const u8 as *const libc::c_char,
                    b"print.c\0" as *const u8 as *const libc::c_char,
                    750 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 36],
                        &[libc::c_char; 36],
                    >(b"char *ctime_format(struct timespec)\0"))
                        .as_ptr(),
                );
            }
        };
        if (*ptm).tm_mon >= 0 as libc::c_int {} else {
            __assert_fail(
                b"ptm->tm_mon >= 0\0" as *const u8 as *const libc::c_char,
                b"print.c\0" as *const u8 as *const libc::c_char,
                751 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"char *ctime_format(struct timespec)\0"))
                    .as_ptr(),
            );
        }
        'c_12890: {
            if (*ptm).tm_mon >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"ptm->tm_mon >= 0\0" as *const u8 as *const libc::c_char,
                    b"print.c\0" as *const u8 as *const libc::c_char,
                    751 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 36],
                        &[libc::c_char; 36],
                    >(b"char *ctime_format(struct timespec)\0"))
                        .as_ptr(),
                );
            }
        };
        if (*ptm).tm_mon < 12 as libc::c_int {} else {
            __assert_fail(
                b"ptm->tm_mon < 12\0" as *const u8 as *const libc::c_char,
                b"print.c\0" as *const u8 as *const libc::c_char,
                752 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"char *ctime_format(struct timespec)\0"))
                    .as_ptr(),
            );
        }
        'c_12850: {
            if (*ptm).tm_mon < 12 as libc::c_int {} else {
                __assert_fail(
                    b"ptm->tm_mon < 12\0" as *const u8 as *const libc::c_char,
                    b"print.c\0" as *const u8 as *const libc::c_char,
                    752 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 36],
                        &[libc::c_char; 36],
                    >(b"char *ctime_format(struct timespec)\0"))
                        .as_ptr(),
                );
            }
        };
        if (*ptm).tm_hour >= 0 as libc::c_int {} else {
            __assert_fail(
                b"ptm->tm_hour >= 0\0" as *const u8 as *const libc::c_char,
                b"print.c\0" as *const u8 as *const libc::c_char,
                753 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"char *ctime_format(struct timespec)\0"))
                    .as_ptr(),
            );
        }
        'c_12810: {
            if (*ptm).tm_hour >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"ptm->tm_hour >= 0\0" as *const u8 as *const libc::c_char,
                    b"print.c\0" as *const u8 as *const libc::c_char,
                    753 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 36],
                        &[libc::c_char; 36],
                    >(b"char *ctime_format(struct timespec)\0"))
                        .as_ptr(),
                );
            }
        };
        if (*ptm).tm_hour < 24 as libc::c_int {} else {
            __assert_fail(
                b"ptm->tm_hour < 24\0" as *const u8 as *const libc::c_char,
                b"print.c\0" as *const u8 as *const libc::c_char,
                754 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"char *ctime_format(struct timespec)\0"))
                    .as_ptr(),
            );
        }
        'c_12770: {
            if (*ptm).tm_hour < 24 as libc::c_int {} else {
                __assert_fail(
                    b"ptm->tm_hour < 24\0" as *const u8 as *const libc::c_char,
                    b"print.c\0" as *const u8 as *const libc::c_char,
                    754 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 36],
                        &[libc::c_char; 36],
                    >(b"char *ctime_format(struct timespec)\0"))
                        .as_ptr(),
                );
            }
        };
        if (*ptm).tm_min < 60 as libc::c_int {} else {
            __assert_fail(
                b"ptm->tm_min < 60\0" as *const u8 as *const libc::c_char,
                b"print.c\0" as *const u8 as *const libc::c_char,
                755 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"char *ctime_format(struct timespec)\0"))
                    .as_ptr(),
            );
        }
        'c_12729: {
            if (*ptm).tm_min < 60 as libc::c_int {} else {
                __assert_fail(
                    b"ptm->tm_min < 60\0" as *const u8 as *const libc::c_char,
                    b"print.c\0" as *const u8 as *const libc::c_char,
                    755 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 36],
                        &[libc::c_char; 36],
                    >(b"char *ctime_format(struct timespec)\0"))
                        .as_ptr(),
                );
            }
        };
        if (*ptm).tm_sec <= 61 as libc::c_int {} else {
            __assert_fail(
                b"ptm->tm_sec <= 61\0" as *const u8 as *const libc::c_char,
                b"print.c\0" as *const u8 as *const libc::c_char,
                756 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"char *ctime_format(struct timespec)\0"))
                    .as_ptr(),
            );
        }
        'c_12689: {
            if (*ptm).tm_sec <= 61 as libc::c_int {} else {
                __assert_fail(
                    b"ptm->tm_sec <= 61\0" as *const u8 as *const libc::c_char,
                    b"print.c\0" as *const u8 as *const libc::c_char,
                    756 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 36],
                        &[libc::c_char; 36],
                    >(b"char *ctime_format(struct timespec)\0"))
                        .as_ptr(),
                );
            }
        };
        nout = snprintf(
            resultbuf.as_mut_ptr(),
            1024 as libc::c_int as libc::c_ulong,
            b"%3s %3s %2d %02d:%02d:%02d.%09ld0 %04d\0" as *const u8
                as *const libc::c_char,
            weekdays[(*ptm).tm_wday as usize],
            months[(*ptm).tm_mon as usize],
            (*ptm).tm_mday,
            (*ptm).tm_hour,
            (*ptm).tm_min,
            (*ptm).tm_sec,
            ts.tv_nsec,
            1900 as libc::c_int + (*ptm).tm_year,
        );
        if nout < 1024 as libc::c_int {} else {
            __assert_fail(
                b"nout < TIME_BUF_LEN\0" as *const u8 as *const libc::c_char,
                b"print.c\0" as *const u8 as *const libc::c_char,
                770 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"char *ctime_format(struct timespec)\0"))
                    .as_ptr(),
            );
        }
        'c_12546: {
            if nout < 1024 as libc::c_int {} else {
                __assert_fail(
                    b"nout < TIME_BUF_LEN\0" as *const u8 as *const libc::c_char,
                    b"print.c\0" as *const u8 as *const libc::c_char,
                    770 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 36],
                        &[libc::c_char; 36],
                    >(b"char *ctime_format(struct timespec)\0"))
                        .as_ptr(),
                );
            }
        };
        return resultbuf.as_mut_ptr();
    } else {
        return format_date(ts, '@' as i32)
    };
}
unsafe extern "C" fn file_sparseness(mut p: *const stat) -> libc::c_double {
    if 0 as libc::c_int as libc::c_long == (*p).st_size {
        if 0 as libc::c_int as libc::c_long == (*p).st_blocks {
            return 1.0f64
        } else {
            return if (*p).st_blocks < 0 as libc::c_int as libc::c_long {
                -::std::f64::INFINITY
            } else {
                ::std::f64::INFINITY
            }
        }
    } else {
        let mut blklen: libc::c_double = 512 as libc::c_int as libc::c_double
            * (*p).st_blocks as libc::c_double;
        return blklen / (*p).st_size as libc::c_double;
    };
}
unsafe extern "C" fn checked_fprintf(
    mut dest: *mut format_val,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut rv: libc::c_int = 0;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    rv = vfprintf((*dest).stream, fmt, ap.as_va_list());
    if rv < 0 as libc::c_int {
        nonfatal_nontarget_file_error(*__errno_location(), (*dest).filename);
    }
}
unsafe extern "C" fn checked_print_quoted(
    mut dest: *mut format_val,
    mut format: *const libc::c_char,
    mut s: *const libc::c_char,
) {
    let mut rv: libc::c_int = print_quoted(
        (*dest).stream,
        (*dest).quote_opts,
        (*dest).dest_is_tty,
        format,
        s,
    );
    if rv < 0 as libc::c_int {
        nonfatal_nontarget_file_error(*__errno_location(), (*dest).filename);
    }
}
unsafe extern "C" fn checked_fwrite(
    mut p: *mut libc::c_void,
    mut siz: size_t,
    mut nmemb: size_t,
    mut dest: *mut format_val,
) {
    let items_written: size_t = fwrite(p, siz, nmemb, (*dest).stream);
    if items_written < nmemb {
        nonfatal_nontarget_file_error(*__errno_location(), (*dest).filename);
    }
}
unsafe extern "C" fn checked_fflush(mut dest: *mut format_val) {
    if 0 as libc::c_int != rpl_fflush((*dest).stream) {
        nonfatal_nontarget_file_error(*__errno_location(), (*dest).filename);
    }
}
unsafe extern "C" fn mode_to_filetype(mut m: mode_t) -> *const libc::c_char {
    if m == 0o100000 as libc::c_int as libc::c_uint {
        return b"f\0" as *const u8 as *const libc::c_char;
    }
    if m == 0o40000 as libc::c_int as libc::c_uint {
        return b"d\0" as *const u8 as *const libc::c_char;
    }
    if m == 0o120000 as libc::c_int as libc::c_uint {
        return b"l\0" as *const u8 as *const libc::c_char;
    }
    if m == 0o140000 as libc::c_int as libc::c_uint {
        return b"s\0" as *const u8 as *const libc::c_char;
    }
    if m == 0o60000 as libc::c_int as libc::c_uint {
        return b"b\0" as *const u8 as *const libc::c_char;
    }
    if m == 0o20000 as libc::c_int as libc::c_uint {
        return b"c\0" as *const u8 as *const libc::c_char;
    }
    if m == 0o10000 as libc::c_int as libc::c_uint {
        return b"p\0" as *const u8 as *const libc::c_char;
    }
    return b"U\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn do_fprintf(
    mut dest: *mut format_val,
    mut segment: *mut segment,
    mut pathname: *const libc::c_char,
    mut stat_buf: *const stat,
) {
    let mut hbuf: [libc::c_char; 652] = [0; 652];
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    match (*segment).segkind as libc::c_uint {
        0 => {
            checked_fwrite(
                (*segment).text as *mut libc::c_void,
                1 as libc::c_int as size_t,
                (*segment).text_len as size_t,
                dest,
            );
        }
        1 => {
            checked_fwrite(
                (*segment).text as *mut libc::c_void,
                1 as libc::c_int as size_t,
                (*segment).text_len as size_t,
                dest,
            );
            checked_fflush(dest);
        }
        2 => {
            let mut current_block_93: u64;
            match (*segment).format_char[0 as libc::c_int as usize] as libc::c_int {
                97 => {
                    checked_fprintf(
                        dest,
                        (*segment).text,
                        ctime_format(get_stat_atime(stat_buf)),
                    );
                    current_block_93 = 2432552683059077439;
                }
                98 => {
                    checked_fprintf(
                        dest,
                        (*segment).text,
                        human_readable(
                            (*stat_buf).st_blocks as uintmax_t,
                            hbuf.as_mut_ptr(),
                            human_ceiling as libc::c_int,
                            512 as libc::c_int as uintmax_t,
                            512 as libc::c_int as uintmax_t,
                        ),
                    );
                    current_block_93 = 2432552683059077439;
                }
                99 => {
                    checked_fprintf(
                        dest,
                        (*segment).text,
                        ctime_format(get_stat_ctime(stat_buf)),
                    );
                    current_block_93 = 2432552683059077439;
                }
                100 => {
                    checked_fprintf(dest, (*segment).text, state.curdepth);
                    current_block_93 = 2432552683059077439;
                }
                68 => {
                    checked_fprintf(
                        dest,
                        (*segment).text,
                        human_readable(
                            (*stat_buf).st_dev,
                            hbuf.as_mut_ptr(),
                            human_ceiling as libc::c_int,
                            1 as libc::c_int as uintmax_t,
                            1 as libc::c_int as uintmax_t,
                        ),
                    );
                    current_block_93 = 2432552683059077439;
                }
                102 => {
                    let mut base: *mut libc::c_char = base_name(pathname);
                    checked_print_quoted(dest, (*segment).text, base);
                    rpl_free(base as *mut libc::c_void);
                    current_block_93 = 2432552683059077439;
                }
                70 => {
                    checked_print_quoted(
                        dest,
                        (*segment).text,
                        filesystem_type(stat_buf, pathname),
                    );
                    current_block_93 = 2432552683059077439;
                }
                103 => {
                    let mut g: *mut group = 0 as *mut group;
                    g = getgrgid((*stat_buf).st_gid);
                    if !g.is_null() {
                        *((*segment).text)
                            .offset(
                                (*segment).text_len as isize,
                            ) = 's' as i32 as libc::c_char;
                        checked_fprintf(dest, (*segment).text, (*g).gr_name);
                        current_block_93 = 2432552683059077439;
                    } else {
                        current_block_93 = 13332318748284305262;
                    }
                }
                71 => {
                    current_block_93 = 13332318748284305262;
                }
                104 => {
                    let mut pname: *mut libc::c_char = xstrdup(pathname);
                    let mut s: *mut libc::c_char = pname
                        .offset(strlen(pname) as isize)
                        .offset(-(1 as libc::c_int as isize));
                    while pname <= s {
                        if *s as libc::c_int != '/' as i32 {
                            break;
                        }
                        s = s.offset(-1);
                        s;
                    }
                    if pname < s
                        && *s.offset(1 as libc::c_int as isize) as libc::c_int
                            == '/' as i32
                    {
                        *s
                            .offset(
                                1 as libc::c_int as isize,
                            ) = '\0' as i32 as libc::c_char;
                    }
                    s = strrchr(pname, '/' as i32);
                    if s.is_null() {
                        checked_print_quoted(
                            dest,
                            (*segment).text,
                            b".\0" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        *s = '\0' as i32 as libc::c_char;
                        checked_print_quoted(dest, (*segment).text, pname);
                    }
                    rpl_free(pname as *mut libc::c_void);
                    current_block_93 = 2432552683059077439;
                }
                72 => {
                    let mut s_0: *mut libc::c_char = xmalloc(
                        (state.starting_path_length + 1 as libc::c_int) as size_t,
                    ) as *mut libc::c_char;
                    memcpy(
                        s_0 as *mut libc::c_void,
                        pathname as *const libc::c_void,
                        state.starting_path_length as libc::c_ulong,
                    );
                    *s_0
                        .offset(
                            state.starting_path_length as isize,
                        ) = 0 as libc::c_int as libc::c_char;
                    checked_fprintf(dest, (*segment).text, s_0);
                    rpl_free(s_0 as *mut libc::c_void);
                    current_block_93 = 2432552683059077439;
                }
                105 => {
                    checked_fprintf(
                        dest,
                        (*segment).text,
                        human_readable(
                            (*stat_buf).st_ino,
                            hbuf.as_mut_ptr(),
                            human_ceiling as libc::c_int,
                            1 as libc::c_int as uintmax_t,
                            1 as libc::c_int as uintmax_t,
                        ),
                    );
                    current_block_93 = 2432552683059077439;
                }
                107 => {
                    checked_fprintf(
                        dest,
                        (*segment).text,
                        human_readable(
                            (*stat_buf).st_blocks as uintmax_t,
                            hbuf.as_mut_ptr(),
                            human_ceiling as libc::c_int,
                            512 as libc::c_int as uintmax_t,
                            1024 as libc::c_int as uintmax_t,
                        ),
                    );
                    current_block_93 = 2432552683059077439;
                }
                108 => {
                    let mut linkname: *mut libc::c_char = 0 as *mut libc::c_char;
                    if (*stat_buf).st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o120000 as libc::c_int as libc::c_uint
                    {
                        linkname = areadlinkat(state.cwd_dir_fd, state.rel_pathname);
                        if linkname.is_null() {
                            nonfatal_target_file_error(*__errno_location(), pathname);
                            state.exit_status = 1 as libc::c_int;
                        }
                    }
                    if !linkname.is_null() {
                        checked_print_quoted(dest, (*segment).text, linkname);
                    } else {
                        checked_print_quoted(
                            dest,
                            (*segment).text,
                            b"\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    rpl_free(linkname as *mut libc::c_void);
                    current_block_93 = 2432552683059077439;
                }
                77 => {
                    let mut modestring: [libc::c_char; 16] = [0; 16];
                    filemodestring(stat_buf, modestring.as_mut_ptr());
                    modestring[10 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                    checked_fprintf(dest, (*segment).text, modestring.as_mut_ptr());
                    current_block_93 = 2432552683059077439;
                }
                109 => {
                    let mut m: mode_t = (*stat_buf).st_mode;
                    let mut traditional_numbering_scheme: bool = 0o4000 as libc::c_int
                        == 0o4000 as libc::c_int
                        && 0o2000 as libc::c_int == 0o2000 as libc::c_int
                        && 0o1000 as libc::c_int == 0o1000 as libc::c_int
                        && 0o400 as libc::c_int == 0o400 as libc::c_int
                        && 0o200 as libc::c_int == 0o200 as libc::c_int
                        && 0o100 as libc::c_int == 0o100 as libc::c_int
                        && 0o400 as libc::c_int >> 3 as libc::c_int
                            == 0o40 as libc::c_int
                        && 0o200 as libc::c_int >> 3 as libc::c_int
                            == 0o20 as libc::c_int
                        && 0o100 as libc::c_int >> 3 as libc::c_int
                            == 0o10 as libc::c_int
                        && 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                            == 0o4 as libc::c_int
                        && 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                            == 0o2 as libc::c_int
                        && 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                            == 0o1 as libc::c_int;
                    checked_fprintf(
                        dest,
                        (*segment).text,
                        if traditional_numbering_scheme as libc::c_int != 0 {
                            m
                                & (0o4000 as libc::c_int | 0o2000 as libc::c_int
                                    | 0o1000 as libc::c_int
                                    | (0o100 as libc::c_int
                                        | 0o100 as libc::c_int >> 3 as libc::c_int
                                        | 0o100 as libc::c_int >> 3 as libc::c_int
                                            >> 3 as libc::c_int
                                        | (0o200 as libc::c_int
                                            | 0o200 as libc::c_int >> 3 as libc::c_int
                                            | 0o200 as libc::c_int >> 3 as libc::c_int
                                                >> 3 as libc::c_int
                                            | (0o400 as libc::c_int
                                                | 0o400 as libc::c_int >> 3 as libc::c_int
                                                | 0o400 as libc::c_int >> 3 as libc::c_int
                                                    >> 3 as libc::c_int)))) as libc::c_uint
                        } else {
                            ((if m & 0o4000 as libc::c_int as libc::c_uint != 0 {
                                0o4000 as libc::c_int
                            } else {
                                0 as libc::c_int
                            })
                                | (if m & 0o2000 as libc::c_int as libc::c_uint != 0 {
                                    0o2000 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                                | (if m & 0o1000 as libc::c_int as libc::c_uint != 0 {
                                    0o1000 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                                | (if m & 0o400 as libc::c_int as libc::c_uint != 0 {
                                    0o400 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                                | (if m & 0o200 as libc::c_int as libc::c_uint != 0 {
                                    0o200 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                                | (if m & 0o100 as libc::c_int as libc::c_uint != 0 {
                                    0o100 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                                | (if m
                                    & (0o400 as libc::c_int >> 3 as libc::c_int) as libc::c_uint
                                    != 0
                                {
                                    0o40 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                                | (if m
                                    & (0o200 as libc::c_int >> 3 as libc::c_int) as libc::c_uint
                                    != 0
                                {
                                    0o20 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                                | (if m
                                    & (0o100 as libc::c_int >> 3 as libc::c_int) as libc::c_uint
                                    != 0
                                {
                                    0o10 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                                | (if m
                                    & (0o400 as libc::c_int >> 3 as libc::c_int
                                        >> 3 as libc::c_int) as libc::c_uint != 0
                                {
                                    0o4 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                                | (if m
                                    & (0o200 as libc::c_int >> 3 as libc::c_int
                                        >> 3 as libc::c_int) as libc::c_uint != 0
                                {
                                    0o2 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })
                                | (if m
                                    & (0o100 as libc::c_int >> 3 as libc::c_int
                                        >> 3 as libc::c_int) as libc::c_uint != 0
                                {
                                    0o1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })) as libc::c_uint
                        },
                    );
                    current_block_93 = 2432552683059077439;
                }
                110 => {
                    checked_fprintf(
                        dest,
                        (*segment).text,
                        human_readable(
                            (*stat_buf).st_nlink,
                            hbuf.as_mut_ptr(),
                            human_ceiling as libc::c_int,
                            1 as libc::c_int as uintmax_t,
                            1 as libc::c_int as uintmax_t,
                        ),
                    );
                    current_block_93 = 2432552683059077439;
                }
                112 => {
                    checked_print_quoted(dest, (*segment).text, pathname);
                    current_block_93 = 2432552683059077439;
                }
                80 => {
                    if state.curdepth > 0 as libc::c_int {
                        cp = pathname.offset(state.starting_path_length as isize);
                        if *cp as libc::c_int == '/' as i32 {
                            cp = cp.offset(1);
                            cp;
                        }
                    } else {
                        cp = b"\0" as *const u8 as *const libc::c_char;
                    }
                    checked_print_quoted(dest, (*segment).text, cp);
                    current_block_93 = 2432552683059077439;
                }
                115 => {
                    checked_fprintf(
                        dest,
                        (*segment).text,
                        human_readable(
                            (*stat_buf).st_size as uintmax_t,
                            hbuf.as_mut_ptr(),
                            human_ceiling as libc::c_int,
                            1 as libc::c_int as uintmax_t,
                            1 as libc::c_int as uintmax_t,
                        ),
                    );
                    current_block_93 = 2432552683059077439;
                }
                83 => {
                    checked_fprintf(dest, (*segment).text, file_sparseness(stat_buf));
                    current_block_93 = 2432552683059077439;
                }
                116 => {
                    checked_fprintf(
                        dest,
                        (*segment).text,
                        ctime_format(get_stat_mtime(stat_buf)),
                    );
                    current_block_93 = 2432552683059077439;
                }
                117 => {
                    let mut p: *mut passwd = 0 as *mut passwd;
                    p = getpwuid((*stat_buf).st_uid);
                    if !p.is_null() {
                        *((*segment).text)
                            .offset(
                                (*segment).text_len as isize,
                            ) = 's' as i32 as libc::c_char;
                        checked_fprintf(dest, (*segment).text, (*p).pw_name);
                        current_block_93 = 2432552683059077439;
                    } else {
                        current_block_93 = 389820867233307883;
                    }
                }
                85 => {
                    current_block_93 = 389820867233307883;
                }
                89 => {
                    if (*stat_buf).st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o120000 as libc::c_int as libc::c_uint
                    {
                        let mut sbuf: stat = stat {
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
                        if fstatat(
                            state.cwd_dir_fd,
                            state.rel_pathname,
                            &mut sbuf,
                            0 as libc::c_int,
                        ) != 0 as libc::c_int
                        {
                            if *__errno_location() == 2 as libc::c_int
                                || *__errno_location() == 20 as libc::c_int
                            {
                                checked_fprintf(
                                    dest,
                                    (*segment).text,
                                    b"N\0" as *const u8 as *const libc::c_char,
                                );
                            } else if *__errno_location() == 40 as libc::c_int {
                                checked_fprintf(
                                    dest,
                                    (*segment).text,
                                    b"L\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                checked_fprintf(
                                    dest,
                                    (*segment).text,
                                    b"?\0" as *const u8 as *const libc::c_char,
                                );
                                error(
                                    0 as libc::c_int,
                                    *__errno_location(),
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    safely_quote_err_filename(0 as libc::c_int, pathname),
                                );
                            }
                        } else {
                            checked_fprintf(
                                dest,
                                (*segment).text,
                                mode_to_filetype(
                                    sbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint,
                                ),
                            );
                        }
                    } else {
                        checked_fprintf(
                            dest,
                            (*segment).text,
                            mode_to_filetype(
                                (*stat_buf).st_mode
                                    & 0o170000 as libc::c_int as libc::c_uint,
                            ),
                        );
                    }
                    current_block_93 = 2432552683059077439;
                }
                121 => {
                    checked_fprintf(
                        dest,
                        (*segment).text,
                        mode_to_filetype(
                            (*stat_buf).st_mode & 0o170000 as libc::c_int as libc::c_uint,
                        ),
                    );
                    current_block_93 = 2432552683059077439;
                }
                90 => {
                    let mut scontext: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut rv: libc::c_int = (Some((options.x_getfilecon).unwrap()))
                        .unwrap()(state.cwd_dir_fd, state.rel_pathname, &mut scontext);
                    if rv < 0 as libc::c_int {
                        checked_fprintf(
                            dest,
                            (*segment).text,
                            b"\0" as *const u8 as *const libc::c_char,
                        );
                        error(
                            0 as libc::c_int,
                            *__errno_location(),
                            dcgettext(
                                0 as *const libc::c_char,
                                b"getfilecon failed: %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            safely_quote_err_filename(0 as libc::c_int, pathname),
                        );
                        state.exit_status = 1 as libc::c_int;
                    } else {
                        checked_fprintf(dest, (*segment).text, scontext);
                        freecon(scontext);
                    }
                    current_block_93 = 2432552683059077439;
                }
                37 => {
                    checked_fwrite(
                        (*segment).text as *mut libc::c_void,
                        1 as libc::c_int as size_t,
                        (*segment).text_len as size_t,
                        dest,
                    );
                    current_block_93 = 2432552683059077439;
                }
                0 => {
                    if ::std::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"error: %s at end of format string\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            b"%\0" as *const u8 as *const libc::c_char,
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"error: %s at end of format string\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            b"%\0" as *const u8 as *const libc::c_char,
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                    current_block_93 = 2432552683059077439;
                }
                _ => {
                    current_block_93 = 2432552683059077439;
                }
            }
            match current_block_93 {
                13332318748284305262 => {
                    checked_fprintf(
                        dest,
                        (*segment).text,
                        human_readable(
                            (*stat_buf).st_gid as uintmax_t,
                            hbuf.as_mut_ptr(),
                            human_ceiling as libc::c_int,
                            1 as libc::c_int as uintmax_t,
                            1 as libc::c_int as uintmax_t,
                        ),
                    );
                }
                389820867233307883 => {
                    checked_fprintf(
                        dest,
                        (*segment).text,
                        human_readable(
                            (*stat_buf).st_uid as uintmax_t,
                            hbuf.as_mut_ptr(),
                            human_ceiling as libc::c_int,
                            1 as libc::c_int as uintmax_t,
                            1 as libc::c_int as uintmax_t,
                        ),
                    );
                }
                _ => {}
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn pred_fprintf(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    let mut dest: *mut format_val = &mut (*pred_ptr).args.printf_vec;
    let mut segment: *mut segment = 0 as *mut segment;
    segment = (*dest).segment;
    while !segment.is_null() {
        if KIND_FORMAT as libc::c_int as libc::c_uint
            == (*segment).segkind as libc::c_uint
            && (*segment).format_char[1 as libc::c_int as usize] as libc::c_int != 0
        {
            let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
            let mut valid: libc::c_int = 0 as libc::c_int;
            match (*segment).format_char[0 as libc::c_int as usize] as libc::c_int {
                65 => {
                    ts = get_stat_atime(stat_buf);
                    valid = 1 as libc::c_int;
                }
                66 => {
                    ts = get_stat_birthtime(stat_buf);
                    if '@' as i32
                        == (*segment).format_char[1 as libc::c_int as usize]
                            as libc::c_int
                    {
                        valid = 1 as libc::c_int;
                    } else {
                        valid = (ts.tv_nsec >= 0 as libc::c_int as libc::c_long)
                            as libc::c_int;
                    }
                }
                67 => {
                    ts = get_stat_ctime(stat_buf);
                    valid = 1 as libc::c_int;
                }
                84 => {
                    ts = get_stat_mtime(stat_buf);
                    valid = 1 as libc::c_int;
                }
                _ => {
                    __assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"print.c\0" as *const u8 as *const libc::c_char,
                        1304 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 68],
                            &[libc::c_char; 68],
                        >(
                            b"_Bool pred_fprintf(const char *, struct stat *, struct predicate *)\0",
                        ))
                            .as_ptr(),
                    );
                    'c_14422: {
                        __assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"print.c\0" as *const u8 as *const libc::c_char,
                            1304 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 68],
                                &[libc::c_char; 68],
                            >(
                                b"_Bool pred_fprintf(const char *, struct stat *, struct predicate *)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    abort();
                }
            }
            if valid != 0 {
                checked_fprintf(
                    dest,
                    (*segment).text,
                    format_date(
                        ts,
                        (*segment).format_char[1 as libc::c_int as usize] as libc::c_int,
                    ),
                );
            } else {
                checked_fprintf(
                    dest,
                    (*segment).text,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            do_fprintf(dest, segment, pathname, stat_buf);
        }
        segment = (*segment).next;
    }
    return 1 as libc::c_int != 0;
}
