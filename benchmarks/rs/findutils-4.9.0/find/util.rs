use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type quoting_options;
    pub type re_dfa_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn rpl_free(_: *mut libc::c_void);
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __fxstatat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn uname(__name: *mut utsname) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn rpl_fflush(gl_stream: *mut FILE) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn forget_non_cloexec_fds();
    fn complain_about_leaky_fds();
    fn fd_leak_check_is_enabled() -> bool;
    static mut program_name: *const libc::c_char;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn quotearg_n_style(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn save_cwd(cwd: *mut saved_cwd) -> libc::c_int;
    fn restore_cwd(cwd: *const saved_cwd) -> libc::c_int;
    fn free_cwd(cwd: *mut saved_cwd);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn bc_do_exec(ctl: *mut buildcmd_control, state_0: *mut buildcmd_state);
    fn sharefile_destroy(_: sharefile_handle);
    static mut state: state;
    static mut options: options;
    fn set_follow_state(opt: SymlinkOption);
    static mut initial_wd: *mut saved_cwd;
    fn pred_fprint0(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_fls(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_fprintf(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_fprint(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn get_eval_tree() -> *mut predicate;
    fn pred_okdir(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_execdir(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_exec(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn get_new_pred_chk_op(
        entry: *const parser_table,
        arg: *const libc::c_char,
    ) -> *mut predicate;
    fn _exit(_: libc::c_int) -> !;
    fn run_in_dir(
        _: *const saved_cwd,
        callback: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
        usercontext: *mut libc::c_void,
    ) -> libc::c_int;
    fn explain_how_to_report_bugs(
        f: *mut FILE,
        program_name_0: *const libc::c_char,
    ) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
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
pub type __suseconds_t = libc::c_long;
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
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub domainname: [libc::c_char; 65],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saved_cwd {
    pub desc: libc::c_int,
    pub name: *mut libc::c_char,
}
pub type uintmax_t = __uintmax_t;
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
pub const DebugStat: DebugOption = 2;
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
    pub args: C2RustUnnamed_0,
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
pub union C2RustUnnamed_0 {
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
pub type PREDICATEFUNCTION = unsafe extern "C" fn(
    *const libc::c_char,
    *mut stat,
    *mut predicate,
) -> bool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_option_assoc {
    pub name: *const libc::c_char,
    pub val: libc::c_int,
    pub docstring: *const libc::c_char,
}
pub const DebugHelp: DebugOption = 16;
pub const DebugAll: DebugOption = -17;
pub const DebugExpressionTree: DebugOption = 1;
pub const DebugTime: DebugOption = 128;
pub const DebugSearch: DebugOption = 4;
pub const DebugSuccessRates: DebugOption = 64;
pub const DebugTreeOpt: DebugOption = 8;
pub const DebugExec: DebugOption = 32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub _gl_dummy: libc::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub _gl_dummy: libc::c_int,
}
pub type DebugOption = libc::c_int;
pub const DebugNone: DebugOption = 0;
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
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
static mut debugassoc: [debug_option_assoc; 9] = [
    {
        let mut init = debug_option_assoc {
            name: b"exec\0" as *const u8 as *const libc::c_char,
            val: DebugExec as libc::c_int,
            docstring: b"Show diagnostic information relating to -exec, -execdir, -ok and -okdir\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = debug_option_assoc {
            name: b"opt\0" as *const u8 as *const libc::c_char,
            val: DebugExpressionTree as libc::c_int | DebugTreeOpt as libc::c_int,
            docstring: b"Show diagnostic information relating to optimisation\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = debug_option_assoc {
            name: b"rates\0" as *const u8 as *const libc::c_char,
            val: DebugSuccessRates as libc::c_int,
            docstring: b"Indicate how often each predicate succeeded\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = debug_option_assoc {
            name: b"search\0" as *const u8 as *const libc::c_char,
            val: DebugSearch as libc::c_int,
            docstring: b"Navigate the directory tree verbosely\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = debug_option_assoc {
            name: b"stat\0" as *const u8 as *const libc::c_char,
            val: DebugStat as libc::c_int,
            docstring: b"Trace calls to stat(2) and lstat(2)\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = debug_option_assoc {
            name: b"time\0" as *const u8 as *const libc::c_char,
            val: DebugTime as libc::c_int,
            docstring: b"Show diagnostic information relating to time-of-day and timestamp comparisons\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = debug_option_assoc {
            name: b"tree\0" as *const u8 as *const libc::c_char,
            val: DebugExpressionTree as libc::c_int,
            docstring: b"Display the expression tree\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = debug_option_assoc {
            name: b"all\0" as *const u8 as *const libc::c_char,
            val: DebugAll as libc::c_int,
            docstring: b"Set all of the debug flags (but help)\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
    {
        let mut init = debug_option_assoc {
            name: b"help\0" as *const u8 as *const libc::c_char,
            val: DebugHelp as libc::c_int,
            docstring: b"Explain the various -D options\0" as *const u8
                as *const libc::c_char,
        };
        init
    },
];
pub unsafe extern "C" fn insert_primary_withpred(
    mut entry: *const parser_table,
    mut pred_func: PRED_FUNC,
    mut arg: *const libc::c_char,
) -> *mut predicate {
    let mut new_pred: *mut predicate = 0 as *mut predicate;
    new_pred = get_new_pred_chk_op(entry, arg);
    (*new_pred).pred_func = pred_func;
    (*new_pred).p_name = (*entry).parser_name;
    (*new_pred).args.str_0 = 0 as *const libc::c_char;
    (*new_pred).p_type = PRIMARY_TYPE;
    (*new_pred).p_prec = NO_PREC;
    return new_pred;
}
pub unsafe extern "C" fn insert_primary(
    mut entry: *const parser_table,
    mut arg: *const libc::c_char,
) -> *mut predicate {
    if ((*entry).pred_func).is_some() {} else {
        __assert_fail(
            b"entry->pred_func != NULL\0" as *const u8 as *const libc::c_char,
            b"util.c\0" as *const u8 as *const libc::c_char,
            123 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 76],
                &[libc::c_char; 76],
            >(
                b"struct predicate *insert_primary(const struct parser_table *, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_7719: {
        if ((*entry).pred_func).is_some() {} else {
            __assert_fail(
                b"entry->pred_func != NULL\0" as *const u8 as *const libc::c_char,
                b"util.c\0" as *const u8 as *const libc::c_char,
                123 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 76],
                    &[libc::c_char; 76],
                >(
                    b"struct predicate *insert_primary(const struct parser_table *, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return insert_primary_withpred(entry, (*entry).pred_func, arg);
}
pub unsafe extern "C" fn insert_primary_noarg(
    mut entry: *const parser_table,
) -> *mut predicate {
    return insert_primary(entry, 0 as *const libc::c_char);
}
unsafe extern "C" fn show_valid_debug_options(mut full: libc::c_int) {
    let mut i: size_t = 0;
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"Valid arguments for -D:\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    if full != 0 {
        i = 0 as libc::c_int as size_t;
        while i
            < (::std::mem::size_of::<[debug_option_assoc; 9]>() as libc::c_ulong)
                .wrapping_div(
                    ::std::mem::size_of::<debug_option_assoc>() as libc::c_ulong,
                )
        {
            fprintf(
                stdout,
                b"%-10s %s\n\0" as *const u8 as *const libc::c_char,
                debugassoc[i as usize].name,
                debugassoc[i as usize].docstring,
            );
            i = i.wrapping_add(1);
            i;
        }
    } else {
        i = 0 as libc::c_int as size_t;
        while i
            < (::std::mem::size_of::<[debug_option_assoc; 9]>() as libc::c_ulong)
                .wrapping_div(
                    ::std::mem::size_of::<debug_option_assoc>() as libc::c_ulong,
                )
        {
            fprintf(
                stdout,
                b"%s%s\0" as *const u8 as *const libc::c_char,
                if i > 0 as libc::c_int as libc::c_ulong {
                    b", \0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                debugassoc[i as usize].name,
            );
            i = i.wrapping_add(1);
            i;
        }
    };
}
pub unsafe extern "C" fn usage(mut status: libc::c_int) -> ! {
    if status != 0 as libc::c_int {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Try '%s --help' for more information.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program_name,
        );
        exit(status);
    }
    fprintf(
        stdout,
        dcgettext(
            0 as *const libc::c_char,
            b"Usage: %s [-H] [-L] [-P] [-Olevel] [-D debugopts] [path...] [expression]\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        program_name,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\nDefault path is the current directory; default expression is -print.\nExpression may consist of: operators, options, tests, and actions.\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\nOperators (decreasing precedence; -and is implicit where no others are given):\n      ( EXPR )   ! EXPR   -not EXPR   EXPR1 -a EXPR2   EXPR1 -and EXPR2\n      EXPR1 -o EXPR2   EXPR1 -or EXPR2   EXPR1 , EXPR2\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\nPositional options (always true):\n      -daystart -follow -nowarn -regextype -warn\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\nNormal options (always true, specified before other expressions):\n      -depth -files0-from FILE -maxdepth LEVELS -mindepth LEVELS\n       -mount -noleaf -xdev -ignore_readdir_race -noignore_readdir_race\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\nTests (N can be +N or -N or N):\n      -amin N -anewer FILE -atime N -cmin N -cnewer FILE -context CONTEXT\n      -ctime N -empty -false -fstype TYPE -gid N -group NAME -ilname PATTERN\n      -iname PATTERN -inum N -iwholename PATTERN -iregex PATTERN\n      -links N -lname PATTERN -mmin N -mtime N -name PATTERN -newer FILE\n      -nouser -nogroup -path PATTERN -perm [-/]MODE -regex PATTERN\n      -readable -writable -executable\n      -wholename PATTERN -size N[bcwkMG] -true -type [bcdpflsD] -uid N\n      -used N -user NAME -xtype [bcdpfls]\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\nActions:\n      -delete -print0 -printf FORMAT -fprintf FILE FORMAT -print \n      -fprint0 FILE -fprint FILE -ls -fls FILE -prune -quit\n      -exec COMMAND ; -exec COMMAND {} + -ok COMMAND ;\n      -execdir COMMAND ; -execdir COMMAND {} + -okdir COMMAND ;\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\nOther common options:\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"      --help                   display this help and exit\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"      --version                output version information and exit\n\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    show_valid_debug_options(0 as libc::c_int);
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\nUse '-D help' for a description of the options, or see find(1)\n\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    explain_how_to_report_bugs(stdout, program_name);
    exit(status);
}
pub unsafe extern "C" fn set_stat_placeholders(mut p: *mut stat) {}
pub unsafe extern "C" fn get_statinfo(
    mut pathname: *const libc::c_char,
    mut name: *const libc::c_char,
    mut p: *mut stat,
) -> libc::c_int {
    if !state.have_stat {
        set_stat_placeholders(p);
        if 0 as libc::c_int == (Some((options.xstat).unwrap())).unwrap()(name, p) {
            if 0 as libc::c_int as libc::c_uint == (*p).st_mode {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"WARNING: file %s appears to have mode 0000\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_n_style(0 as libc::c_int, options.err_quoting_style, name),
                );
                state.exit_status = 1 as libc::c_int;
            }
        } else {
            if !options.ignore_readdir_race || *__errno_location() != 2 as libc::c_int {
                nonfatal_target_file_error(*__errno_location(), pathname);
            }
            return -(1 as libc::c_int);
        }
    }
    state.have_stat = 1 as libc::c_int != 0;
    state.have_type = 1 as libc::c_int != 0;
    state.type_0 = (*p).st_mode;
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_info(
    mut pathname: *const libc::c_char,
    mut p: *mut stat,
    mut pred_ptr: *mut predicate,
) -> libc::c_int {
    let mut todo: bool = 0 as libc::c_int != 0;
    if (*pred_ptr).need_stat as libc::c_int != 0 && !state.have_stat {
        todo = 1 as libc::c_int != 0;
    } else if (*pred_ptr).need_type as libc::c_int != 0 && !state.have_type {
        todo = 1 as libc::c_int != 0;
    } else if (*pred_ptr).need_inum {
        if (*p).st_ino == 0 {
            todo = 1 as libc::c_int != 0;
        } else if !state.have_type
            || (*p).st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint
        {
            todo = 1 as libc::c_int != 0;
        }
    }
    if todo {
        if get_statinfo(pathname, state.rel_pathname, p) != 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn check_nofollow() -> bool {
    let mut uts: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };
    let mut release: libc::c_float = 0.;
    if 0 as libc::c_int == 0o400000 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    if 0 as libc::c_int == uname(&mut uts) {
        let mut conversion: Option::<
            unsafe extern "C" fn(*const libc::c_char) -> libc::c_double,
        > = Some(atof as unsafe extern "C" fn(*const libc::c_char) -> libc::c_double);
        release = conversion.unwrap()((uts.release).as_mut_ptr()) as libc::c_float;
        if 0 as libc::c_int
            == strcmp(
                b"Linux\0" as *const u8 as *const libc::c_char,
                (uts.sysname).as_mut_ptr(),
            )
        {
            return release >= 2.2f32
        } else if 0 as libc::c_int
            == strcmp(
                b"FreeBSD\0" as *const u8 as *const libc::c_char,
                (uts.sysname).as_mut_ptr(),
            )
        {
            return release >= 3.1f32
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn exec_cb(mut context: *mut libc::c_void) -> libc::c_int {
    let mut execp: *mut exec_val = context as *mut exec_val;
    bc_do_exec(&mut (*execp).ctl, &mut (*execp).state);
    return 0 as libc::c_int;
}
unsafe extern "C" fn do_exec(mut execp: *mut exec_val) {
    run_in_dir(
        (*execp).wd_for_exec,
        Some(exec_cb as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int),
        execp as *mut libc::c_void,
    );
    if (*execp).wd_for_exec != initial_wd {
        free_cwd((*execp).wd_for_exec);
        rpl_free((*execp).wd_for_exec as *mut libc::c_void);
        (*execp).wd_for_exec = 0 as *mut saved_cwd;
    }
}
unsafe extern "C" fn do_complete_pending_execdirs(mut p: *mut predicate) {
    if p.is_null() {
        return;
    }
    if state.execdirs_outstanding {} else {
        __assert_fail(
            b"state.execdirs_outstanding\0" as *const u8 as *const libc::c_char,
            b"util.c\0" as *const u8 as *const libc::c_char,
            396 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"void do_complete_pending_execdirs(struct predicate *)\0"))
                .as_ptr(),
        );
    }
    'c_7208: {
        if state.execdirs_outstanding {} else {
            __assert_fail(
                b"state.execdirs_outstanding\0" as *const u8 as *const libc::c_char,
                b"util.c\0" as *const u8 as *const libc::c_char,
                396 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"void do_complete_pending_execdirs(struct predicate *)\0"))
                    .as_ptr(),
            );
        }
    };
    do_complete_pending_execdirs((*p).pred_left);
    if (*p).pred_func == Some(pred_execdir as PREDICATEFUNCTION)
        || (*p).pred_func == Some(pred_okdir as PREDICATEFUNCTION)
    {
        if (*p).args.exec_vec.multiple {
            let mut execp: *mut exec_val = &mut (*p).args.exec_vec;
            if (*execp).state.todo != 0 {
                do_exec(execp);
            }
        }
    }
    do_complete_pending_execdirs((*p).pred_right);
}
pub unsafe extern "C" fn complete_pending_execdirs() {
    if state.execdirs_outstanding {
        do_complete_pending_execdirs(get_eval_tree());
        state.execdirs_outstanding = 0 as libc::c_int != 0;
    }
}
pub unsafe extern "C" fn complete_pending_execs(mut p: *mut predicate) {
    if p.is_null() {
        return;
    }
    complete_pending_execs((*p).pred_left);
    if (*p).pred_func == Some(pred_exec as PREDICATEFUNCTION)
        && (*p).args.exec_vec.multiple as libc::c_int != 0
    {
        let mut execp: *mut exec_val = &mut (*p).args.exec_vec;
        if (*execp).state.todo != 0 {
            bc_do_exec(&mut (*execp).ctl, &mut (*execp).state);
        }
    }
    complete_pending_execs((*p).pred_right);
}
pub unsafe extern "C" fn record_initial_cwd() {
    initial_wd = xmalloc(::std::mem::size_of::<saved_cwd>() as libc::c_ulong)
        as *mut saved_cwd;
    if 0 as libc::c_int != save_cwd(initial_wd) {
        if ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to save initial working directory%s%s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                if (*initial_wd).desc < 0 as libc::c_int
                    && !((*initial_wd).name).is_null()
                {
                    b": \0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                if (*initial_wd).desc < 0 as libc::c_int
                    && !((*initial_wd).name).is_null()
                {
                    (*initial_wd).name as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to save initial working directory%s%s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                if (*initial_wd).desc < 0 as libc::c_int
                    && !((*initial_wd).name).is_null()
                {
                    b": \0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                if (*initial_wd).desc < 0 as libc::c_int
                    && !((*initial_wd).name).is_null()
                {
                    (*initial_wd).name as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
}
unsafe extern "C" fn cleanup_initial_cwd() {
    if 0 as libc::c_int == restore_cwd(initial_wd) {
        free_cwd(initial_wd);
        rpl_free(initial_wd as *mut libc::c_void);
        initial_wd = 0 as *mut saved_cwd;
    } else {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to restore initial working directory%s%s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            if (*initial_wd).desc < 0 as libc::c_int && !((*initial_wd).name).is_null() {
                b": \0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            if (*initial_wd).desc < 0 as libc::c_int && !((*initial_wd).name).is_null() {
                (*initial_wd).name as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        _exit(1 as libc::c_int);
    };
}
unsafe extern "C" fn traverse_tree(
    mut tree: *mut predicate,
    mut callback: Option::<unsafe extern "C" fn(*mut predicate) -> ()>,
) {
    if !((*tree).pred_left).is_null() {
        traverse_tree((*tree).pred_left, callback);
    }
    callback.unwrap()(tree);
    if !((*tree).pred_right).is_null() {
        traverse_tree((*tree).pred_right, callback);
    }
}
unsafe extern "C" fn undangle_file_pointers(mut p: *mut predicate) {
    if (*p).pred_func == Some(pred_fprint as PREDICATEFUNCTION)
        || (*p).pred_func == Some(pred_fprintf as PREDICATEFUNCTION)
        || (*p).pred_func == Some(pred_fls as PREDICATEFUNCTION)
        || (*p).pred_func == Some(pred_fprint0 as PREDICATEFUNCTION)
    {
        (*p).args.printf_vec.stream = 0 as *mut FILE;
    }
}
pub unsafe extern "C" fn cleanup() {
    let mut eval_tree: *mut predicate = get_eval_tree();
    if !eval_tree.is_null() {
        traverse_tree(
            eval_tree,
            Some(complete_pending_execs as unsafe extern "C" fn(*mut predicate) -> ()),
        );
        complete_pending_execdirs();
    }
    sharefile_destroy(state.shared_files);
    if !eval_tree.is_null() {
        traverse_tree(
            eval_tree,
            Some(undangle_file_pointers as unsafe extern "C" fn(*mut predicate) -> ()),
        );
    }
    cleanup_initial_cwd();
    if fd_leak_check_is_enabled() {
        complain_about_leaky_fds();
        forget_non_cloexec_fds();
    }
    if rpl_fflush(stdout) == -(1 as libc::c_int) {
        nonfatal_nontarget_file_error(
            *__errno_location(),
            b"standard output\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn fallback_stat(
    mut name: *const libc::c_char,
    mut p: *mut stat,
    mut prev_rv: libc::c_int,
) -> libc::c_int {
    match *__errno_location() {
        2 | 20 => {
            if options.debug_options & DebugStat as libc::c_int as libc::c_ulong != 0 {
                fprintf(
                    stderr,
                    b"fallback_stat(): stat(%s) failed; falling back on lstat()\n\0"
                        as *const u8 as *const libc::c_char,
                    name,
                );
            }
            return fstatat(state.cwd_dir_fd, name, p, 0x100 as libc::c_int);
        }
        13 | 5 | 40 | 36 | 75 | _ => return prev_rv,
    };
}
pub unsafe extern "C" fn optionh_stat(
    mut name: *const libc::c_char,
    mut p: *mut stat,
) -> libc::c_int {
    if -(100 as libc::c_int) != state.cwd_dir_fd {
        if state.cwd_dir_fd >= 0 as libc::c_int {} else {
            __assert_fail(
                b"state.cwd_dir_fd >= 0\0" as *const u8 as *const libc::c_char,
                b"util.c\0" as *const u8 as *const libc::c_char,
                605 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 46],
                    &[libc::c_char; 46],
                >(b"int optionh_stat(const char *, struct stat *)\0"))
                    .as_ptr(),
            );
        }
        'c_6104: {
            if state.cwd_dir_fd >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"state.cwd_dir_fd >= 0\0" as *const u8 as *const libc::c_char,
                    b"util.c\0" as *const u8 as *const libc::c_char,
                    605 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 46],
                        &[libc::c_char; 46],
                    >(b"int optionh_stat(const char *, struct stat *)\0"))
                        .as_ptr(),
                );
            }
        };
    }
    set_stat_placeholders(p);
    if 0 as libc::c_int == state.curdepth {
        let mut rv: libc::c_int = 0;
        rv = fstatat(state.cwd_dir_fd, name, p, 0 as libc::c_int);
        if 0 as libc::c_int == rv {
            return 0 as libc::c_int
        } else {
            return fallback_stat(name, p, rv)
        }
    } else {
        return fstatat(state.cwd_dir_fd, name, p, 0x100 as libc::c_int)
    };
}
pub unsafe extern "C" fn optionl_stat(
    mut name: *const libc::c_char,
    mut p: *mut stat,
) -> libc::c_int {
    let mut rv: libc::c_int = 0;
    if -(100 as libc::c_int) != state.cwd_dir_fd {
        if state.cwd_dir_fd >= 0 as libc::c_int {} else {
            __assert_fail(
                b"state.cwd_dir_fd >= 0\0" as *const u8 as *const libc::c_char,
                b"util.c\0" as *const u8 as *const libc::c_char,
                636 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 46],
                    &[libc::c_char; 46],
                >(b"int optionl_stat(const char *, struct stat *)\0"))
                    .as_ptr(),
            );
        }
        'c_5909: {
            if state.cwd_dir_fd >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"state.cwd_dir_fd >= 0\0" as *const u8 as *const libc::c_char,
                    b"util.c\0" as *const u8 as *const libc::c_char,
                    636 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 46],
                        &[libc::c_char; 46],
                    >(b"int optionl_stat(const char *, struct stat *)\0"))
                        .as_ptr(),
                );
            }
        };
    }
    set_stat_placeholders(p);
    rv = fstatat(state.cwd_dir_fd, name, p, 0 as libc::c_int);
    if 0 as libc::c_int == rv {
        return 0 as libc::c_int
    } else {
        return fallback_stat(name, p, rv)
    };
}
pub unsafe extern "C" fn optionp_stat(
    mut name: *const libc::c_char,
    mut p: *mut stat,
) -> libc::c_int {
    if state.cwd_dir_fd >= 0 as libc::c_int || state.cwd_dir_fd == -(100 as libc::c_int)
    {} else {
        __assert_fail(
            b"(state.cwd_dir_fd >= 0) || (state.cwd_dir_fd==AT_FDCWD)\0" as *const u8
                as *const libc::c_char,
            b"util.c\0" as *const u8 as *const libc::c_char,
            653 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"int optionp_stat(const char *, struct stat *)\0"))
                .as_ptr(),
        );
    }
    'c_5981: {
        if state.cwd_dir_fd >= 0 as libc::c_int
            || state.cwd_dir_fd == -(100 as libc::c_int)
        {} else {
            __assert_fail(
                b"(state.cwd_dir_fd >= 0) || (state.cwd_dir_fd==AT_FDCWD)\0" as *const u8
                    as *const libc::c_char,
                b"util.c\0" as *const u8 as *const libc::c_char,
                653 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 46],
                    &[libc::c_char; 46],
                >(b"int optionp_stat(const char *, struct stat *)\0"))
                    .as_ptr(),
            );
        }
    };
    set_stat_placeholders(p);
    return fstatat(state.cwd_dir_fd, name, p, 0x100 as libc::c_int);
}
static mut stat_count: uintmax_t = 0 as libc::c_uint as uintmax_t;
pub unsafe extern "C" fn debug_stat(
    mut file: *const libc::c_char,
    mut bufp: *mut stat,
) -> libc::c_int {
    stat_count = stat_count.wrapping_add(1);
    stat_count;
    fprintf(stderr, b"debug_stat (%s)\n\0" as *const u8 as *const libc::c_char, file);
    match options.symlink_handling as libc::c_uint {
        1 => return optionl_stat(file, bufp),
        2 => return optionh_stat(file, bufp),
        0 => return optionp_stat(file, bufp),
        _ => {}
    }
    __assert_fail(
        b"0\0" as *const u8 as *const libc::c_char,
        b"util.c\0" as *const u8 as *const libc::c_char,
        677 as libc::c_int as libc::c_uint,
        (*::std::mem::transmute::<
            &[u8; 44],
            &[libc::c_char; 44],
        >(b"int debug_stat(const char *, struct stat *)\0"))
            .as_ptr(),
    );
    'c_6158: {
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"util.c\0" as *const u8 as *const libc::c_char,
            677 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"int debug_stat(const char *, struct stat *)\0"))
                .as_ptr(),
        );
    };
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn following_links() -> bool {
    match options.symlink_handling as libc::c_uint {
        1 => return 1 as libc::c_int != 0,
        2 => return state.curdepth == 0 as libc::c_int,
        0 | _ => return 0 as libc::c_int != 0,
    };
}
pub unsafe extern "C" fn digest_mode(
    mut mode: *mut mode_t,
    mut pathname: *const libc::c_char,
    mut name: *const libc::c_char,
    mut pstat: *mut stat,
    mut leaf: bool,
) -> bool {
    if *mode != 0 {
        if *mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
            && following_links() as libc::c_int != 0
        {
            if get_statinfo(pathname, name, pstat) != 0 as libc::c_int {
                return 0 as libc::c_int != 0;
            }
            state.type_0 = (*pstat).st_mode;
            *mode = state.type_0;
            state.have_type = 1 as libc::c_int != 0;
        } else {
            state.have_type = 1 as libc::c_int != 0;
            state.type_0 = *mode;
            (*pstat).st_mode = state.type_0;
        }
    } else if leaf {
        state.have_stat = 0 as libc::c_int != 0;
        state.have_type = 0 as libc::c_int != 0;
        state.type_0 = 0 as libc::c_int as mode_t;
    } else {
        if get_statinfo(pathname, name, pstat) != 0 as libc::c_int {
            return 0 as libc::c_int != 0;
        }
        *mode = (*pstat).st_mode;
        state.type_0 = *mode;
        state.have_type = 1 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn default_prints(mut pred: *mut predicate) -> bool {
    while !pred.is_null() {
        if (*pred).no_default_print {
            return 0 as libc::c_int != 0;
        }
        pred = (*pred).pred_next;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn looks_like_expression(
    mut arg: *const libc::c_char,
    mut leading: bool,
) -> bool {
    match *arg.offset(0 as libc::c_int as isize) as libc::c_int {
        45 => {
            if *arg.offset(1 as libc::c_int as isize) != 0 {
                return 1 as libc::c_int != 0
            } else {
                return 0 as libc::c_int != 0
            }
        }
        41 | 44 => {
            if *arg.offset(1 as libc::c_int as isize) != 0 {
                return 0 as libc::c_int != 0
            } else {
                return !leading
            }
        }
        33 | 40 => {
            if *arg.offset(1 as libc::c_int as isize) != 0 {
                return 0 as libc::c_int != 0
            } else {
                return 1 as libc::c_int != 0
            }
        }
        _ => return 0 as libc::c_int != 0,
    };
}
unsafe extern "C" fn process_debug_options(mut arg: *mut libc::c_char) {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut token_context: *mut libc::c_char = 0 as *mut libc::c_char;
    let delimiters: [libc::c_char; 2] = *::std::mem::transmute::<
        &[u8; 2],
        &[libc::c_char; 2],
    >(b",\0");
    let mut empty: bool = 1 as libc::c_int != 0;
    let mut i: size_t = 0;
    p = strtok_r(arg, delimiters.as_ptr(), &mut token_context);
    while !p.is_null() {
        empty = 0 as libc::c_int != 0;
        i = 0 as libc::c_int as size_t;
        while i
            < (::std::mem::size_of::<[debug_option_assoc; 9]>() as libc::c_ulong)
                .wrapping_div(
                    ::std::mem::size_of::<debug_option_assoc>() as libc::c_ulong,
                )
        {
            if 0 as libc::c_int == strcmp(debugassoc[i as usize].name, p) {
                options.debug_options |= debugassoc[i as usize].val as libc::c_ulong;
                break;
            } else {
                i = i.wrapping_add(1);
                i;
            }
        }
        if i
            >= (::std::mem::size_of::<[debug_option_assoc; 9]>() as libc::c_ulong)
                .wrapping_div(
                    ::std::mem::size_of::<debug_option_assoc>() as libc::c_ulong,
                )
        {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Ignoring unrecognised debug flag %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style(0 as libc::c_int, options.err_quoting_style, arg),
            );
        }
        p = strtok_r(0 as *mut libc::c_char, delimiters.as_ptr(), &mut token_context);
    }
    if empty {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Empty argument to the -D option.\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(1 as libc::c_int);
    } else if options.debug_options & DebugHelp as libc::c_int as libc::c_ulong != 0 {
        show_valid_debug_options(1 as libc::c_int);
        exit(0 as libc::c_int);
    }
}
unsafe extern "C" fn process_optimisation_option(mut arg: *const libc::c_char) {
    if 0 as libc::c_int == *arg.offset(0 as libc::c_int as isize) as libc::c_int {
        if ::std::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"The -O option must be immediately followed by a decimal integer\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
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
                    b"The -O option must be immediately followed by a decimal integer\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    } else {
        let mut opt_level: libc::c_ulong = 0;
        let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
        if *(*__ctype_b_loc())
            .offset(
                *arg.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                    as isize,
            ) as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            if ::std::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Please specify a decimal number immediately after -O\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
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
                        b"Please specify a decimal number immediately after -O\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        } else {
            let mut prev_errno: libc::c_int = *__errno_location();
            *__errno_location() = 0 as libc::c_int;
            opt_level = strtoul(arg, &mut end, 10 as libc::c_int);
            if 0 as libc::c_int as libc::c_ulong == opt_level
                && end == arg as *mut libc::c_char
            {
                if ::std::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Please specify a decimal number immediately after -O\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
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
                            b"Please specify a decimal number immediately after -O\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            } else if *end != 0 {
                if ::std::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Invalid optimisation level %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        arg,
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
                            b"Invalid optimisation level %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        arg,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            } else if (9223372036854775807 as libc::c_long as libc::c_ulong)
                .wrapping_mul(2 as libc::c_ulong)
                .wrapping_add(1 as libc::c_ulong) == opt_level
                && *__errno_location() != 0
            {
                if ::std::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Invalid optimisation level %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        arg,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Invalid optimisation level %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        arg,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            } else if opt_level
                > (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                    as libc::c_ulong
            {
                if ::std::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Optimisation level %lu is too high.  If you want to find files very quickly, consider using GNU locate.\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        opt_level,
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
                            b"Optimisation level %lu is too high.  If you want to find files very quickly, consider using GNU locate.\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        opt_level,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            } else {
                options.optimisation_level = opt_level as libc::c_ushort;
                *__errno_location() = prev_errno;
            }
        }
    };
}
pub unsafe extern "C" fn process_leading_options(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut end_of_leading_options: libc::c_int = 0;
    i = 1 as libc::c_int;
    loop {
        end_of_leading_options = i;
        if !(end_of_leading_options < argc) {
            break;
        }
        if 0 as libc::c_int
            == strcmp(
                b"-H\0" as *const u8 as *const libc::c_char,
                *argv.offset(i as isize),
            )
        {
            set_follow_state(SYMLINK_DEREF_ARGSONLY);
        } else if 0 as libc::c_int
            == strcmp(
                b"-L\0" as *const u8 as *const libc::c_char,
                *argv.offset(i as isize),
            )
        {
            set_follow_state(SYMLINK_ALWAYS_DEREF);
        } else if 0 as libc::c_int
            == strcmp(
                b"-P\0" as *const u8 as *const libc::c_char,
                *argv.offset(i as isize),
            )
        {
            set_follow_state(SYMLINK_NEVER_DEREF);
        } else if 0 as libc::c_int
            == strcmp(
                b"--\0" as *const u8 as *const libc::c_char,
                *argv.offset(i as isize),
            )
        {
            end_of_leading_options = i + 1 as libc::c_int;
            break;
        } else if 0 as libc::c_int
            == strcmp(
                b"-D\0" as *const u8 as *const libc::c_char,
                *argv.offset(i as isize),
            )
        {
            if argc <= i + 1 as libc::c_int {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Missing argument after the -D option.\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                usage(1 as libc::c_int);
            }
            process_debug_options(*argv.offset((i + 1 as libc::c_int) as isize));
            i += 1;
            i;
        } else if 0 as libc::c_int
            == strncmp(
                b"-O\0" as *const u8 as *const libc::c_char,
                *argv.offset(i as isize),
                2 as libc::c_int as libc::c_ulong,
            )
        {
            process_optimisation_option(
                (*argv.offset(i as isize)).offset(2 as libc::c_int as isize),
            );
        } else {
            end_of_leading_options = i;
            break;
        }
        i += 1;
        i;
    }
    return end_of_leading_options;
}
unsafe extern "C" fn now() -> timespec {
    let mut retval: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut t: time_t = 0;
    if 0 as libc::c_int == gettimeofday(&mut tv, 0 as *mut libc::c_void) {
        retval.tv_sec = tv.tv_sec;
        retval.tv_nsec = tv.tv_usec * 1000 as libc::c_int as libc::c_long;
        return retval;
    }
    t = time(0 as *mut time_t);
    if t != -(1 as libc::c_int) as time_t {} else {
        __assert_fail(
            b"t != (time_t)-1\0" as *const u8 as *const libc::c_char,
            b"util.c\0" as *const u8 as *const libc::c_char,
            978 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"struct timespec now(void)\0"))
                .as_ptr(),
        );
    }
    'c_9828: {
        if t != -(1 as libc::c_int) as time_t {} else {
            __assert_fail(
                b"t != (time_t)-1\0" as *const u8 as *const libc::c_char,
                b"util.c\0" as *const u8 as *const libc::c_char,
                978 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"struct timespec now(void)\0"))
                    .as_ptr(),
            );
        }
    };
    retval.tv_sec = t;
    retval.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
    return retval;
}
pub unsafe extern "C" fn set_option_defaults(mut p: *mut options) {
    if !(getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char)).is_null() {
        (*p).posixly_correct = 1 as libc::c_int != 0;
    } else {
        (*p).posixly_correct = 0 as libc::c_int != 0;
    }
    (*p).open_nofollow_available = check_nofollow();
    (*p).regex_options = 0 as libc::c_int;
    if isatty(0 as libc::c_int) != 0 {
        (*p).warnings = 1 as libc::c_int != 0;
        (*p).literal_control_chars = 0 as libc::c_int != 0;
    } else {
        (*p).warnings = 0 as libc::c_int != 0;
        (*p).literal_control_chars = 0 as libc::c_int != 0;
    }
    if (*p).posixly_correct {
        (*p).warnings = 0 as libc::c_int != 0;
    }
    (*p).do_dir_first = 1 as libc::c_int != 0;
    (*p).explicit_depth = 0 as libc::c_int != 0;
    (*p).mindepth = -(1 as libc::c_int);
    (*p).maxdepth = (*p).mindepth;
    (*p).start_time = now();
    (*p)
        .cur_day_start
        .tv_sec = (*p).start_time.tv_sec - 86400 as libc::c_int as libc::c_long;
    (*p).cur_day_start.tv_nsec = (*p).start_time.tv_nsec;
    (*p).full_days = 0 as libc::c_int != 0;
    (*p).stay_on_filesystem = 0 as libc::c_int != 0;
    (*p).ignore_readdir_race = 0 as libc::c_int != 0;
    if (*p).posixly_correct {
        (*p).output_block_size = 512 as libc::c_int;
    } else {
        (*p).output_block_size = 1024 as libc::c_int;
    }
    (*p).debug_options = 0 as libc::c_ulong;
    (*p).optimisation_level = 2 as libc::c_int as libc::c_ushort;
    if !(getenv(b"FIND_BLOCK_SIZE\0" as *const u8 as *const libc::c_char)).is_null() {
        if ::std::mem::size_of::<C2RustUnnamed_8>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"The environment variable FIND_BLOCK_SIZE is not supported, the only thing that affects the block size is the POSIXLY_CORRECT environment variable\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
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
                    b"The environment variable FIND_BLOCK_SIZE is not supported, the only thing that affects the block size is the POSIXLY_CORRECT environment variable\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    (*p).no_leaf_check = 0 as libc::c_int != 0;
    set_follow_state(SYMLINK_NEVER_DEREF);
    (*p).err_quoting_style = locale_quoting_style;
    (*p).files0_from = 0 as *const libc::c_char;
    (*p).ok_prompt_stdin = 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn apply_predicate(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut p: *mut predicate,
) -> bool {
    (*p).perf.visits = ((*p).perf.visits).wrapping_add(1);
    (*p).perf.visits;
    if (*p).need_stat as libc::c_int != 0 || (*p).need_type as libc::c_int != 0
        || (*p).need_inum as libc::c_int != 0
    {
        if get_info(pathname, stat_buf, p) != 0 as libc::c_int {
            return 0 as libc::c_int != 0;
        }
    }
    if ((*p).pred_func).unwrap()(pathname, stat_buf, p) {
        (*p).perf.successes = ((*p).perf.successes).wrapping_add(1);
        (*p).perf.successes;
        return 1 as libc::c_int != 0;
    } else {
        return 0 as libc::c_int != 0
    };
}
pub unsafe extern "C" fn is_exec_in_local_dir(pred_func: PRED_FUNC) -> bool {
    return Some(pred_execdir as PREDICATEFUNCTION) == pred_func
        || Some(pred_okdir as PREDICATEFUNCTION) == pred_func;
}
pub unsafe extern "C" fn safely_quote_err_filename(
    mut n: libc::c_int,
    mut arg: *const libc::c_char,
) -> *const libc::c_char {
    return quotearg_n_style(n, options.err_quoting_style, arg);
}
unsafe extern "C" fn report_file_err(
    mut exitval: libc::c_int,
    mut errno_value: libc::c_int,
    mut is_target_file: bool,
    mut name: *const libc::c_char,
) {
    if !is_target_file || !state.already_issued_stat_error_msg {
        error(
            exitval,
            errno_value,
            b"%s\0" as *const u8 as *const libc::c_char,
            safely_quote_err_filename(0 as libc::c_int, name),
        );
        state.exit_status = 1 as libc::c_int;
    }
    if is_target_file {
        state.already_issued_stat_error_msg = 1 as libc::c_int != 0;
    }
}
pub unsafe extern "C" fn nonfatal_target_file_error(
    mut errno_value: libc::c_int,
    mut name: *const libc::c_char,
) {
    report_file_err(0 as libc::c_int, errno_value, 1 as libc::c_int != 0, name);
}
pub unsafe extern "C" fn fatal_target_file_error(
    mut errno_value: libc::c_int,
    mut name: *const libc::c_char,
) -> ! {
    report_file_err(1 as libc::c_int, errno_value, 1 as libc::c_int != 0, name);
    abort();
}
pub unsafe extern "C" fn nonfatal_nontarget_file_error(
    mut errno_value: libc::c_int,
    mut name: *const libc::c_char,
) {
    report_file_err(0 as libc::c_int, errno_value, 0 as libc::c_int != 0, name);
}
pub unsafe extern "C" fn fatal_nontarget_file_error(
    mut errno_value: libc::c_int,
    mut name: *const libc::c_char,
) -> ! {
    state.already_issued_stat_error_msg = 0 as libc::c_int != 0;
    report_file_err(1 as libc::c_int, errno_value, 0 as libc::c_int != 0, name);
    abort();
}
