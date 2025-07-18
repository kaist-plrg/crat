use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type __dirstream;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    pub type quoting_options;
    pub type saved_cwd;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn fdopendir(__fd: libc::c_int) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn __errno_location() -> *mut libc::c_int;
    fn getgrgid(__gid: __gid_t) -> *mut group;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn freecon(con: *mut libc::c_char);
    fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;
    fn faccessat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __type: libc::c_int,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn unlinkat(
        __fd: libc::c_int,
        __name: *const libc::c_char,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn rpl_free(ptr: *mut libc::c_void);
    fn areadlinkat(fd: libc::c_int, filename: *const libc::c_char) -> *mut libc::c_char;
    fn base_name(file: *const libc::c_char) -> *mut libc::c_char;
    fn strip_trailing_slashes(file: *mut libc::c_char) -> bool;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn rpl_fflush(gl_stream: *mut FILE) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn openat_safer(
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: ...
    ) -> libc::c_int;
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn yesno() -> bool;
    fn optionl_stat(name: *const libc::c_char, p: *mut stat) -> libc::c_int;
    fn optionp_stat(name: *const libc::c_char, p: *mut stat) -> libc::c_int;
    fn set_stat_placeholders(p: *mut stat);
    fn get_statinfo(
        pathname: *const libc::c_char,
        name: *const libc::c_char,
        p: *mut stat,
    ) -> libc::c_int;
    fn cleanup();
    fn filesystem_type(
        statp: *const stat,
        path: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn rpl_re_match(
        __buffer: *mut re_pattern_buffer,
        __String: *const libc::c_char,
        __length: regoff_t,
        __start: regoff_t,
        __regs: *mut re_registers,
    ) -> regoff_t;
    fn apply_predicate(
        pathname: *const libc::c_char,
        stat_buf: *mut stat,
        p: *mut predicate,
    ) -> bool;
    fn safely_quote_err_filename(
        n: libc::c_int,
        arg: *const libc::c_char,
    ) -> *const libc::c_char;
    fn nonfatal_target_file_error(errno_value: libc::c_int, name: *const libc::c_char);
    fn following_links() -> bool;
    fn print_predicate(fp: *mut FILE, p: *const predicate);
    fn impl_pred_exec(
        pathname: *const libc::c_char,
        stat_buf: *mut stat,
        pred_ptr: *mut predicate,
    ) -> bool;
    static mut options: options;
    static mut state: state;
    fn list_file(
        name: *const libc::c_char,
        dir_fd: libc::c_int,
        relname: *const libc::c_char,
        statp: *const stat,
        current_time: time_t,
        output_block_size: libc::c_int,
        literal_control_chars: libc::c_int,
        stream: *mut FILE,
    );
    fn print_quoted(
        fp: *mut FILE,
        qopts: *const quoting_options,
        dest_is_tty: bool,
        format: *const libc::c_char,
        s: *const libc::c_char,
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
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ino_t = __ino_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
pub type size_t = libc::c_ulong;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
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
pub type uintmax_t = __uintmax_t;
pub type __re_size_t = size_t;
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
pub type regoff_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
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
    pub args: C2RustUnnamed,
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
pub union C2RustUnnamed {
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
pub type file_type = libc::c_uint;
pub const FTYPE_COUNT: file_type = 7;
pub const FTYPE_SOCK: file_type = 6;
pub const FTYPE_FIFO: file_type = 5;
pub const FTYPE_LNK: file_type = 4;
pub const FTYPE_REG: file_type = 3;
pub const FTYPE_DIR: file_type = 2;
pub const FTYPE_CHR: file_type = 1;
pub const FTYPE_BLK: file_type = 0;
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
pub type PREDICATEFUNCTION = unsafe extern "C" fn(
    *const libc::c_char,
    *mut stat,
    *mut predicate,
) -> bool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _gl_dummy: libc::c_int,
}
pub const DebugSuccessRates: DebugOption = 64;
pub type DebugOption = libc::c_int;
pub const DebugAll: DebugOption = -17;
pub const DebugTime: DebugOption = 128;
pub const DebugExec: DebugOption = 32;
pub const DebugHelp: DebugOption = 16;
pub const DebugTreeOpt: DebugOption = 8;
pub const DebugSearch: DebugOption = 4;
pub const DebugStat: DebugOption = 2;
pub const DebugExpressionTree: DebugOption = 1;
pub const DebugNone: DebugOption = 0;
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
unsafe extern "C" fn ts_difference(
    mut ts1: timespec,
    mut ts2: timespec,
) -> libc::c_double {
    let mut d: libc::c_double = difftime(ts1.tv_sec, ts2.tv_sec)
        + 1.0e-9f64 * (ts1.tv_nsec - ts2.tv_nsec) as libc::c_double;
    return d;
}
unsafe extern "C" fn compare_ts(mut ts1: timespec, mut ts2: timespec) -> libc::c_int {
    if ts1.tv_sec == ts2.tv_sec && ts1.tv_nsec == ts2.tv_nsec {
        return 0 as libc::c_int
    } else {
        let mut diff: libc::c_double = ts_difference(ts1, ts2);
        return if diff < 0.0f64 { -(1 as libc::c_int) } else { 1 as libc::c_int };
    };
}
unsafe extern "C" fn pred_timewindow(
    mut ts: timespec,
    mut pred_ptr: *const predicate,
    mut window: libc::c_int,
) -> bool {
    match (*pred_ptr).args.reftime.kind as libc::c_uint {
        0 => return compare_ts(ts, (*pred_ptr).args.reftime.ts) > 0 as libc::c_int,
        1 => return compare_ts(ts, (*pred_ptr).args.reftime.ts) < 0 as libc::c_int,
        2 => {
            let mut delta: libc::c_double = ts_difference(
                ts,
                (*pred_ptr).args.reftime.ts,
            );
            return delta > 0.0f64 && delta <= window as libc::c_double;
        }
        _ => {}
    }
    __assert_fail(
        b"0\0" as *const u8 as *const libc::c_char,
        b"pred.c\0" as *const u8 as *const libc::c_char,
        138 as libc::c_int as libc::c_uint,
        (*::std::mem::transmute::<
            &[u8; 70],
            &[libc::c_char; 70],
        >(b"_Bool pred_timewindow(struct timespec, const struct predicate *, int)\0"))
            .as_ptr(),
    );
    'c_10849: {
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"pred.c\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 70],
                &[libc::c_char; 70],
            >(
                b"_Bool pred_timewindow(struct timespec, const struct predicate *, int)\0",
            ))
                .as_ptr(),
        );
    };
    abort();
}
pub unsafe extern "C" fn pred_amin(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    &mut pathname;
    return pred_timewindow(get_stat_atime(stat_buf), pred_ptr, 60 as libc::c_int);
}
pub unsafe extern "C" fn pred_and(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    if ((*pred_ptr).pred_left).is_null()
        || apply_predicate(pathname, stat_buf, (*pred_ptr).pred_left) as libc::c_int != 0
    {
        return apply_predicate(pathname, stat_buf, (*pred_ptr).pred_right)
    } else {
        return 0 as libc::c_int != 0
    };
}
pub unsafe extern "C" fn pred_anewer(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    &mut pathname;
    if COMP_GT as libc::c_int as libc::c_uint
        == (*pred_ptr).args.reftime.kind as libc::c_uint
    {} else {
        __assert_fail(
            b"COMP_GT == pred_ptr->args.reftime.kind\0" as *const u8
                as *const libc::c_char,
            b"pred.c\0" as *const u8 as *const libc::c_char,
            166 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"_Bool pred_anewer(const char *, struct stat *, struct predicate *)\0"))
                .as_ptr(),
        );
    }
    'c_11119: {
        if COMP_GT as libc::c_int as libc::c_uint
            == (*pred_ptr).args.reftime.kind as libc::c_uint
        {} else {
            __assert_fail(
                b"COMP_GT == pred_ptr->args.reftime.kind\0" as *const u8
                    as *const libc::c_char,
                b"pred.c\0" as *const u8 as *const libc::c_char,
                166 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"_Bool pred_anewer(const char *, struct stat *, struct predicate *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return compare_ts(get_stat_atime(stat_buf), (*pred_ptr).args.reftime.ts)
        > 0 as libc::c_int;
}
pub unsafe extern "C" fn pred_atime(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    &mut pathname;
    return pred_timewindow(get_stat_atime(stat_buf), pred_ptr, 86400 as libc::c_int);
}
pub unsafe extern "C" fn pred_closeparen(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    &mut pathname;
    &mut stat_buf;
    &mut pred_ptr;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pred_cmin(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    return pred_timewindow(get_stat_ctime(stat_buf), pred_ptr, 60 as libc::c_int);
}
pub unsafe extern "C" fn pred_cnewer(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    if COMP_GT as libc::c_int as libc::c_uint
        == (*pred_ptr).args.reftime.kind as libc::c_uint
    {} else {
        __assert_fail(
            b"COMP_GT == pred_ptr->args.reftime.kind\0" as *const u8
                as *const libc::c_char,
            b"pred.c\0" as *const u8 as *const libc::c_char,
            199 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"_Bool pred_cnewer(const char *, struct stat *, struct predicate *)\0"))
                .as_ptr(),
        );
    }
    'c_11267: {
        if COMP_GT as libc::c_int as libc::c_uint
            == (*pred_ptr).args.reftime.kind as libc::c_uint
        {} else {
            __assert_fail(
                b"COMP_GT == pred_ptr->args.reftime.kind\0" as *const u8
                    as *const libc::c_char,
                b"pred.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"_Bool pred_cnewer(const char *, struct stat *, struct predicate *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return compare_ts(get_stat_ctime(stat_buf), (*pred_ptr).args.reftime.ts)
        > 0 as libc::c_int;
}
pub unsafe extern "C" fn pred_comma(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    if !((*pred_ptr).pred_left).is_null() {
        apply_predicate(pathname, stat_buf, (*pred_ptr).pred_left);
    }
    return apply_predicate(pathname, stat_buf, (*pred_ptr).pred_right);
}
pub unsafe extern "C" fn pred_ctime(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    &mut pathname;
    return pred_timewindow(get_stat_ctime(stat_buf), pred_ptr, 86400 as libc::c_int);
}
unsafe extern "C" fn perform_delete(mut flags: libc::c_int) -> bool {
    return 0 as libc::c_int == unlinkat(state.cwd_dir_fd, state.rel_pathname, flags);
}
pub unsafe extern "C" fn pred_delete(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    if strcmp(state.rel_pathname, b".\0" as *const u8 as *const libc::c_char) != 0 {
        let mut flags: libc::c_int = 0 as libc::c_int;
        if state.have_stat as libc::c_int != 0
            && (*stat_buf).st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint
        {
            flags |= 0x200 as libc::c_int;
        }
        if perform_delete(flags) {
            return 1 as libc::c_int != 0
        } else {
            if 2 as libc::c_int == *__errno_location()
                && options.ignore_readdir_race as libc::c_int != 0
            {
                *__errno_location() = 0 as libc::c_int;
                return 1 as libc::c_int != 0;
            }
            if 21 as libc::c_int == *__errno_location() {
                if flags & 0x200 as libc::c_int == 0 as libc::c_int {
                    flags |= 0x200 as libc::c_int;
                    if perform_delete(flags) {
                        return 1 as libc::c_int != 0;
                    }
                }
            }
        }
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"cannot delete %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            safely_quote_err_filename(0 as libc::c_int, pathname),
        );
        state.exit_status = 1 as libc::c_int;
        return 0 as libc::c_int != 0;
    } else {
        return 1 as libc::c_int != 0
    };
}
pub unsafe extern "C" fn pred_empty(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    if (*stat_buf).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint
    {
        let mut fd: libc::c_int = 0;
        let mut d: *mut DIR = 0 as *mut DIR;
        let mut dp: *mut dirent = 0 as *mut dirent;
        let mut empty: bool = 1 as libc::c_int != 0;
        *__errno_location() = 0 as libc::c_int;
        fd = openat_safer(
            state.cwd_dir_fd,
            state.rel_pathname,
            0 as libc::c_int | 0 as libc::c_int | 0o2000000 as libc::c_int
                | 0o200000 as libc::c_int | 0o400 as libc::c_int | 0o4000 as libc::c_int,
        );
        if fd < 0 as libc::c_int {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                safely_quote_err_filename(0 as libc::c_int, pathname),
            );
            state.exit_status = 1 as libc::c_int;
            return 0 as libc::c_int != 0;
        }
        d = fdopendir(fd);
        if d.is_null() {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                safely_quote_err_filename(0 as libc::c_int, pathname),
            );
            state.exit_status = 1 as libc::c_int;
            close(fd);
            return 0 as libc::c_int != 0;
        }
        *__errno_location() = 0 as libc::c_int;
        dp = readdir(d);
        while !dp.is_null() {
            if (*dp).d_name[0 as libc::c_int as usize] as libc::c_int != '.' as i32
                || (*dp).d_name[1 as libc::c_int as usize] as libc::c_int != '\0' as i32
                    && ((*dp).d_name[1 as libc::c_int as usize] as libc::c_int
                        != '.' as i32
                        || (*dp).d_name[2 as libc::c_int as usize] as libc::c_int
                            != '\0' as i32)
            {
                empty = 0 as libc::c_int != 0;
                break;
            } else {
                dp = readdir(d);
            }
        }
        if *__errno_location() != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                safely_quote_err_filename(0 as libc::c_int, pathname),
            );
            state.exit_status = 1 as libc::c_int;
            closedir(d);
            return 0 as libc::c_int != 0;
        }
        if closedir(d) != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                safely_quote_err_filename(0 as libc::c_int, pathname),
            );
            state.exit_status = 1 as libc::c_int;
            return 0 as libc::c_int != 0;
        }
        return empty;
    } else if (*stat_buf).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
    {
        return (*stat_buf).st_size == 0 as libc::c_int as libc::c_long
    } else {
        return 0 as libc::c_int != 0
    };
}
pub unsafe extern "C" fn pred_exec(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    return impl_pred_exec(pathname, stat_buf, pred_ptr);
}
pub unsafe extern "C" fn pred_execdir(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    &mut pathname;
    return impl_pred_exec(state.rel_pathname, stat_buf, pred_ptr);
}
pub unsafe extern "C" fn pred_false(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    &mut pathname;
    &mut stat_buf;
    &mut pred_ptr;
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn pred_fls(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    let mut stream: *mut FILE = (*pred_ptr).args.printf_vec.stream;
    list_file(
        pathname,
        state.cwd_dir_fd,
        state.rel_pathname,
        stat_buf,
        options.start_time.tv_sec,
        options.output_block_size,
        (*pred_ptr).literal_control_chars as libc::c_int,
        stream,
    );
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pred_fprint(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    &mut pathname;
    &mut stat_buf;
    print_quoted(
        (*pred_ptr).args.printf_vec.stream,
        (*pred_ptr).args.printf_vec.quote_opts,
        (*pred_ptr).args.printf_vec.dest_is_tty,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        pathname,
    );
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pred_fprint0(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    let mut fp: *mut FILE = (*pred_ptr).args.printf_vec.stream;
    &mut stat_buf;
    fputs(pathname, fp);
    putc(0 as libc::c_int, fp);
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pred_fstype(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    if strcmp(filesystem_type(stat_buf, pathname), (*pred_ptr).args.str_0)
        == 0 as libc::c_int
    {
        return 1 as libc::c_int != 0
    } else {
        return 0 as libc::c_int != 0
    };
}
pub unsafe extern "C" fn pred_gid(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    match (*pred_ptr).args.numinfo.kind as libc::c_uint {
        0 => {
            if (*stat_buf).st_gid as libc::c_ulong > (*pred_ptr).args.numinfo.l_val {
                return 1 as libc::c_int != 0;
            }
        }
        1 => {
            if ((*stat_buf).st_gid as libc::c_ulong) < (*pred_ptr).args.numinfo.l_val {
                return 1 as libc::c_int != 0;
            }
        }
        2 => {
            if (*stat_buf).st_gid as libc::c_ulong == (*pred_ptr).args.numinfo.l_val {
                return 1 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn pred_group(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    if (*pred_ptr).args.gid == (*stat_buf).st_gid {
        return 1 as libc::c_int != 0
    } else {
        return 0 as libc::c_int != 0
    };
}
pub unsafe extern "C" fn pred_ilname(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    return match_lname(pathname, stat_buf, pred_ptr, 1 as libc::c_int != 0);
}
unsafe extern "C" fn pred_name_common(
    mut pathname: *const libc::c_char,
    mut str: *const libc::c_char,
    mut flags: libc::c_int,
) -> bool {
    let mut b: bool = false;
    let mut base: *mut libc::c_char = base_name(pathname);
    strip_trailing_slashes(base);
    b = fnmatch(str, base, flags) == 0 as libc::c_int;
    rpl_free(base as *mut libc::c_void);
    return b;
}
pub unsafe extern "C" fn pred_iname(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    return pred_name_common(
        pathname,
        (*pred_ptr).args.str_0,
        (1 as libc::c_int) << 4 as libc::c_int,
    );
}
pub unsafe extern "C" fn pred_inum(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    match (*pred_ptr).args.numinfo.kind as libc::c_uint {
        0 => {
            if (*stat_buf).st_ino > (*pred_ptr).args.numinfo.l_val {
                return 1 as libc::c_int != 0;
            }
        }
        1 => {
            if (*stat_buf).st_ino < (*pred_ptr).args.numinfo.l_val {
                return 1 as libc::c_int != 0;
            }
        }
        2 => {
            if (*stat_buf).st_ino == (*pred_ptr).args.numinfo.l_val {
                return 1 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn pred_ipath(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    if fnmatch((*pred_ptr).args.str_0, pathname, (1 as libc::c_int) << 4 as libc::c_int)
        == 0 as libc::c_int
    {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn pred_links(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    match (*pred_ptr).args.numinfo.kind as libc::c_uint {
        0 => {
            if (*stat_buf).st_nlink > (*pred_ptr).args.numinfo.l_val {
                return 1 as libc::c_int != 0;
            }
        }
        1 => {
            if (*stat_buf).st_nlink < (*pred_ptr).args.numinfo.l_val {
                return 1 as libc::c_int != 0;
            }
        }
        2 => {
            if (*stat_buf).st_nlink == (*pred_ptr).args.numinfo.l_val {
                return 1 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn pred_lname(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    return match_lname(pathname, stat_buf, pred_ptr, 0 as libc::c_int != 0);
}
unsafe extern "C" fn match_lname(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
    mut ignore_case: bool,
) -> bool {
    let mut ret: bool = 0 as libc::c_int != 0;
    if (*stat_buf).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o120000 as libc::c_int as libc::c_uint
    {
        let mut linkname: *mut libc::c_char = areadlinkat(
            state.cwd_dir_fd,
            state.rel_pathname,
        );
        if !linkname.is_null() {
            if fnmatch(
                (*pred_ptr).args.str_0,
                linkname,
                if ignore_case as libc::c_int != 0 {
                    (1 as libc::c_int) << 4 as libc::c_int
                } else {
                    0 as libc::c_int
                },
            ) == 0 as libc::c_int
            {
                ret = 1 as libc::c_int != 0;
            }
        } else {
            nonfatal_target_file_error(*__errno_location(), pathname);
            state.exit_status = 1 as libc::c_int;
        }
        rpl_free(linkname as *mut libc::c_void);
    }
    return ret;
}
pub unsafe extern "C" fn pred_ls(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    return pred_fls(pathname, stat_buf, pred_ptr);
}
pub unsafe extern "C" fn pred_mmin(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    &mut pathname;
    return pred_timewindow(get_stat_mtime(stat_buf), pred_ptr, 60 as libc::c_int);
}
pub unsafe extern "C" fn pred_mtime(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    return pred_timewindow(get_stat_mtime(stat_buf), pred_ptr, 86400 as libc::c_int);
}
pub unsafe extern "C" fn pred_name(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    return pred_name_common(pathname, (*pred_ptr).args.str_0, 0 as libc::c_int);
}
pub unsafe extern "C" fn pred_negate(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    return !apply_predicate(pathname, stat_buf, (*pred_ptr).pred_right);
}
pub unsafe extern "C" fn pred_newer(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    if COMP_GT as libc::c_int as libc::c_uint
        == (*pred_ptr).args.reftime.kind as libc::c_uint
    {} else {
        __assert_fail(
            b"COMP_GT == pred_ptr->args.reftime.kind\0" as *const u8
                as *const libc::c_char,
            b"pred.c\0" as *const u8 as *const libc::c_char,
            621 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"_Bool pred_newer(const char *, struct stat *, struct predicate *)\0"))
                .as_ptr(),
        );
    }
    'c_12888: {
        if COMP_GT as libc::c_int as libc::c_uint
            == (*pred_ptr).args.reftime.kind as libc::c_uint
        {} else {
            __assert_fail(
                b"COMP_GT == pred_ptr->args.reftime.kind\0" as *const u8
                    as *const libc::c_char,
                b"pred.c\0" as *const u8 as *const libc::c_char,
                621 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"_Bool pred_newer(const char *, struct stat *, struct predicate *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return compare_ts(get_stat_mtime(stat_buf), (*pred_ptr).args.reftime.ts)
        > 0 as libc::c_int;
}
pub unsafe extern "C" fn pred_newerXY(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut collected: bool = 0 as libc::c_int != 0;
    if COMP_GT as libc::c_int as libc::c_uint
        == (*pred_ptr).args.reftime.kind as libc::c_uint
    {} else {
        __assert_fail(
            b"COMP_GT == pred_ptr->args.reftime.kind\0" as *const u8
                as *const libc::c_char,
            b"pred.c\0" as *const u8 as *const libc::c_char,
            631 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"_Bool pred_newerXY(const char *, struct stat *, struct predicate *)\0"))
                .as_ptr(),
        );
    }
    'c_13179: {
        if COMP_GT as libc::c_int as libc::c_uint
            == (*pred_ptr).args.reftime.kind as libc::c_uint
        {} else {
            __assert_fail(
                b"COMP_GT == pred_ptr->args.reftime.kind\0" as *const u8
                    as *const libc::c_char,
                b"pred.c\0" as *const u8 as *const libc::c_char,
                631 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 68],
                    &[libc::c_char; 68],
                >(
                    b"_Bool pred_newerXY(const char *, struct stat *, struct predicate *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    match (*pred_ptr).args.reftime.xval as libc::c_uint {
        4 => {
            if (*pred_ptr).args.reftime.xval as libc::c_uint
                != XVAL_TIME as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"pred_ptr->args.reftime.xval != XVAL_TIME\0" as *const u8
                        as *const libc::c_char,
                    b"pred.c\0" as *const u8 as *const libc::c_char,
                    636 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 68],
                        &[libc::c_char; 68],
                    >(
                        b"_Bool pred_newerXY(const char *, struct stat *, struct predicate *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_13122: {
                if (*pred_ptr).args.reftime.xval as libc::c_uint
                    != XVAL_TIME as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"pred_ptr->args.reftime.xval != XVAL_TIME\0" as *const u8
                            as *const libc::c_char,
                        b"pred.c\0" as *const u8 as *const libc::c_char,
                        636 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 68],
                            &[libc::c_char; 68],
                        >(
                            b"_Bool pred_newerXY(const char *, struct stat *, struct predicate *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            return 0 as libc::c_int != 0;
        }
        0 => {
            ts = get_stat_atime(stat_buf);
            collected = 1 as libc::c_int != 0;
        }
        1 => {
            ts = get_stat_birthtime(stat_buf);
            collected = 1 as libc::c_int != 0;
            if ts.tv_nsec < 0 as libc::c_int as libc::c_long {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"WARNING: cannot determine birth time of file %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    safely_quote_err_filename(0 as libc::c_int, pathname),
                );
                return 0 as libc::c_int != 0;
            }
        }
        2 => {
            ts = get_stat_ctime(stat_buf);
            collected = 1 as libc::c_int != 0;
        }
        3 => {
            ts = get_stat_mtime(stat_buf);
            collected = 1 as libc::c_int != 0;
        }
        _ => {}
    }
    if collected {} else {
        __assert_fail(
            b"collected\0" as *const u8 as *const libc::c_char,
            b"pred.c\0" as *const u8 as *const libc::c_char,
            667 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 68],
                &[libc::c_char; 68],
            >(b"_Bool pred_newerXY(const char *, struct stat *, struct predicate *)\0"))
                .as_ptr(),
        );
    }
    'c_12965: {
        if collected {} else {
            __assert_fail(
                b"collected\0" as *const u8 as *const libc::c_char,
                b"pred.c\0" as *const u8 as *const libc::c_char,
                667 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 68],
                    &[libc::c_char; 68],
                >(
                    b"_Bool pred_newerXY(const char *, struct stat *, struct predicate *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return compare_ts(ts, (*pred_ptr).args.reftime.ts) > 0 as libc::c_int;
}
pub unsafe extern "C" fn pred_nogroup(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    return (getgrgid((*stat_buf).st_gid)).is_null();
}
pub unsafe extern "C" fn pred_nouser(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    return (getpwuid((*stat_buf).st_uid)).is_null();
}
unsafe extern "C" fn is_ok(
    mut program: *const libc::c_char,
    mut arg: *const libc::c_char,
) -> bool {
    rpl_fflush(stdout);
    if fprintf(
        stderr,
        dcgettext(
            0 as *const libc::c_char,
            b"< %s ... %s > ? \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        program,
        arg,
    ) < 0 as libc::c_int
    {
        if ::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to write prompt for -ok\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
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
                    b"Failed to write prompt for -ok\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    rpl_fflush(stderr);
    return yesno();
}
pub unsafe extern "C" fn pred_ok(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    if is_ok(
        *((*pred_ptr).args.exec_vec.replace_vec).offset(0 as libc::c_int as isize),
        pathname,
    ) {
        return impl_pred_exec(pathname, stat_buf, pred_ptr)
    } else {
        return 0 as libc::c_int != 0
    };
}
pub unsafe extern "C" fn pred_okdir(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    if is_ok(
        *((*pred_ptr).args.exec_vec.replace_vec).offset(0 as libc::c_int as isize),
        pathname,
    ) {
        return impl_pred_exec(state.rel_pathname, stat_buf, pred_ptr)
    } else {
        return 0 as libc::c_int != 0
    };
}
pub unsafe extern "C" fn pred_openparen(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pred_or(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    if ((*pred_ptr).pred_left).is_null()
        || !apply_predicate(pathname, stat_buf, (*pred_ptr).pred_left)
    {
        return apply_predicate(pathname, stat_buf, (*pred_ptr).pred_right)
    } else {
        return 1 as libc::c_int != 0
    };
}
pub unsafe extern "C" fn pred_path(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    if fnmatch((*pred_ptr).args.str_0, pathname, 0 as libc::c_int) == 0 as libc::c_int {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn pred_perm(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    let mut mode: mode_t = (*stat_buf).st_mode;
    let mut perm_val: mode_t = (*pred_ptr)
        .args
        .perm
        .val[((mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int != 0 as libc::c_int)
        as libc::c_int as usize];
    match (*pred_ptr).args.perm.kind as libc::c_uint {
        0 => return mode & perm_val == perm_val,
        1 => {
            if 0 as libc::c_int as libc::c_uint == perm_val {
                return 1 as libc::c_int != 0
            } else {
                return mode & perm_val != 0 as libc::c_int as libc::c_uint
            }
        }
        2 => {
            return mode
                & (0o4000 as libc::c_int | 0o2000 as libc::c_int | 0o1000 as libc::c_int
                    | (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                        | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                        | (0o200 as libc::c_int
                            | 0o200 as libc::c_int >> 3 as libc::c_int
                            | 0o200 as libc::c_int >> 3 as libc::c_int
                                >> 3 as libc::c_int
                            | (0o400 as libc::c_int
                                | 0o400 as libc::c_int >> 3 as libc::c_int
                                | 0o400 as libc::c_int >> 3 as libc::c_int
                                    >> 3 as libc::c_int)))) as libc::c_uint == perm_val;
        }
        _ => {
            abort();
        }
    };
}
pub unsafe extern "C" fn pred_executable(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    return 0 as libc::c_int
        == faccessat(
            state.cwd_dir_fd,
            state.rel_pathname,
            1 as libc::c_int,
            0 as libc::c_int,
        );
}
pub unsafe extern "C" fn pred_readable(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    return 0 as libc::c_int
        == faccessat(
            state.cwd_dir_fd,
            state.rel_pathname,
            4 as libc::c_int,
            0 as libc::c_int,
        );
}
pub unsafe extern "C" fn pred_writable(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    return 0 as libc::c_int
        == faccessat(
            state.cwd_dir_fd,
            state.rel_pathname,
            2 as libc::c_int,
            0 as libc::c_int,
        );
}
pub unsafe extern "C" fn pred_print(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    print_quoted(
        (*pred_ptr).args.printf_vec.stream,
        (*pred_ptr).args.printf_vec.quote_opts,
        (*pred_ptr).args.printf_vec.dest_is_tty,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        pathname,
    );
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pred_print0(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    return pred_fprint0(pathname, stat_buf, pred_ptr);
}
pub unsafe extern "C" fn pred_prune(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    if options.do_dir_first as libc::c_int == 1 as libc::c_int {
        if state.have_stat {} else {
            __assert_fail(
                b"state.have_stat\0" as *const u8 as *const libc::c_char,
                b"pred.c\0" as *const u8 as *const libc::c_char,
                853 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"_Bool pred_prune(const char *, struct stat *, struct predicate *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_10416: {
            if state.have_stat {} else {
                __assert_fail(
                    b"state.have_stat\0" as *const u8 as *const libc::c_char,
                    b"pred.c\0" as *const u8 as *const libc::c_char,
                    853 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 66],
                        &[libc::c_char; 66],
                    >(
                        b"_Bool pred_prune(const char *, struct stat *, struct predicate *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if !stat_buf.is_null()
            && (*stat_buf).st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint
        {
            state.stop_at_current_level = 1 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pred_quit(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> ! {
    cleanup();
    exit(state.exit_status);
}
pub unsafe extern "C" fn pred_regex(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    let mut len: libc::c_int = strlen(pathname) as libc::c_int;
    if rpl_re_match(
        (*pred_ptr).args.regex,
        pathname,
        len as regoff_t,
        0 as libc::c_int as regoff_t,
        0 as *mut libc::c_void as *mut re_registers,
    ) == len as libc::c_long
    {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn pred_size(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    let mut f_val: uintmax_t = 0;
    f_val = ((*stat_buf).st_size / (*pred_ptr).args.size.blocksize as libc::c_long
        + ((*stat_buf).st_size % (*pred_ptr).args.size.blocksize as libc::c_long
            != 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long)
        as uintmax_t;
    match (*pred_ptr).args.size.kind as libc::c_uint {
        0 => {
            if f_val > (*pred_ptr).args.size.size {
                return 1 as libc::c_int != 0;
            }
        }
        1 => {
            if f_val < (*pred_ptr).args.size.size {
                return 1 as libc::c_int != 0;
            }
        }
        2 => {
            if f_val == (*pred_ptr).args.size.size {
                return 1 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn pred_samefile(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    if (*stat_buf).st_ino != 0 {
        if (*stat_buf).st_ino != (*pred_ptr).args.samefileid.ino {
            return 0 as libc::c_int != 0;
        }
    }
    if 0 as libc::c_int == get_statinfo(pathname, state.rel_pathname, stat_buf) {
        return (*stat_buf).st_ino == (*pred_ptr).args.samefileid.ino
            && (*stat_buf).st_dev == (*pred_ptr).args.samefileid.dev
    } else {
        return 0 as libc::c_int != 0
    };
}
#[no_mangle] pub unsafe extern "C" fn pred_true(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn pred_type(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    let mut mode: mode_t = 0;
    let mut type_0: file_type = FTYPE_COUNT;
    if state.have_type {} else {
        __assert_fail(
            b"state.have_type\0" as *const u8 as *const libc::c_char,
            b"pred.c\0" as *const u8 as *const libc::c_char,
            977 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"_Bool pred_type(const char *, struct stat *, struct predicate *)\0"))
                .as_ptr(),
        );
    }
    'c_14283: {
        if state.have_type {} else {
            __assert_fail(
                b"state.have_type\0" as *const u8 as *const libc::c_char,
                b"pred.c\0" as *const u8 as *const libc::c_char,
                977 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"_Bool pred_type(const char *, struct stat *, struct predicate *)\0"))
                    .as_ptr(),
            );
        }
    };
    if 0 as libc::c_int as libc::c_uint == state.type_0 {
        return 0 as libc::c_int != 0;
    }
    if state.have_stat {
        mode = (*stat_buf).st_mode;
    } else {
        mode = state.type_0;
    }
    match mode & 0o170000 as libc::c_int as libc::c_uint {
        32768 => {
            type_0 = FTYPE_REG;
        }
        16384 => {
            type_0 = FTYPE_DIR;
        }
        40960 => {
            type_0 = FTYPE_LNK;
        }
        24576 => {
            type_0 = FTYPE_BLK;
        }
        8192 => {
            type_0 = FTYPE_CHR;
        }
        49152 => {
            type_0 = FTYPE_SOCK;
        }
        4096 => {
            type_0 = FTYPE_FIFO;
        }
        _ => {}
    }
    if type_0 as libc::c_uint != FTYPE_COUNT as libc::c_int as libc::c_uint
        && (*pred_ptr).args.types[type_0 as usize] as libc::c_int != 0
    {
        return 1 as libc::c_int != 0
    } else {
        return 0 as libc::c_int != 0
    };
}
pub unsafe extern "C" fn pred_uid(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    match (*pred_ptr).args.numinfo.kind as libc::c_uint {
        0 => {
            if (*stat_buf).st_uid as libc::c_ulong > (*pred_ptr).args.numinfo.l_val {
                return 1 as libc::c_int != 0;
            }
        }
        1 => {
            if ((*stat_buf).st_uid as libc::c_ulong) < (*pred_ptr).args.numinfo.l_val {
                return 1 as libc::c_int != 0;
            }
        }
        2 => {
            if (*stat_buf).st_uid as libc::c_ulong == (*pred_ptr).args.numinfo.l_val {
                return 1 as libc::c_int != 0;
            }
        }
        _ => {}
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn pred_used(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    let mut delta: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut at: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut ct: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    at = get_stat_atime(stat_buf);
    ct = get_stat_ctime(stat_buf);
    if compare_ts(at, ct) < 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    delta.tv_sec = ct.tv_sec - at.tv_sec;
    delta.tv_nsec = ct.tv_nsec - at.tv_nsec;
    if delta.tv_nsec < 0 as libc::c_int as libc::c_long {
        delta.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
        delta.tv_sec -= 1 as libc::c_int as libc::c_long;
    }
    return pred_timewindow(delta, pred_ptr, 86400 as libc::c_int);
}
pub unsafe extern "C" fn pred_user(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    if (*pred_ptr).args.uid == (*stat_buf).st_uid {
        return 1 as libc::c_int != 0
    } else {
        return 0 as libc::c_int != 0
    };
}
pub unsafe extern "C" fn pred_xtype(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
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
    let mut ystat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    > = None;
    if following_links() {
        ystat = Some(
            optionp_stat
                as unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
        );
    } else {
        ystat = Some(
            optionl_stat
                as unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
        );
    }
    set_stat_placeholders(&mut sbuf);
    if (Some(ystat.unwrap())).unwrap()(state.rel_pathname, &mut sbuf) != 0 as libc::c_int
    {
        if following_links() as libc::c_int != 0
            && *__errno_location() == 2 as libc::c_int
        {
            return pred_type(pathname, stat_buf, pred_ptr)
        } else {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                safely_quote_err_filename(0 as libc::c_int, pathname),
            );
            state.exit_status = 1 as libc::c_int;
        }
        return 0 as libc::c_int != 0;
    }
    return pred_type(pathname, &mut sbuf, pred_ptr);
}
pub unsafe extern "C" fn pred_context(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    let mut scontext: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rv: libc::c_int = (Some((options.x_getfilecon).unwrap()))
        .unwrap()(state.cwd_dir_fd, state.rel_pathname, &mut scontext);
    if rv < 0 as libc::c_int {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"getfilecon failed: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            safely_quote_err_filename(0 as libc::c_int, pathname),
        );
        return 0 as libc::c_int != 0;
    }
    rv = (fnmatch((*pred_ptr).args.scontext, scontext, 0 as libc::c_int)
        == 0 as libc::c_int) as libc::c_int;
    freecon(scontext);
    return rv != 0;
}
unsafe extern "C" fn blank_rtrim(
    mut str: *const libc::c_char,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    if str.is_null() {
        return 0 as *mut libc::c_char;
    }
    strcpy(buf, str);
    i = (strlen(buf)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    while i >= 0 as libc::c_int
        && (*buf.offset(i as isize) as libc::c_int == ' ' as i32
            || *buf.offset(i as isize) as libc::c_int == '\t' as i32)
    {
        i -= 1;
        i;
    }
    i += 1;
    *buf.offset(i as isize) = '\0' as i32 as libc::c_char;
    return buf;
}
pub unsafe extern "C" fn print_list(mut fp: *mut FILE, mut node: *mut predicate) {
    let mut cur: *mut predicate = 0 as *mut predicate;
    let mut name: [libc::c_char; 256] = [0; 256];
    cur = node;
    while !cur.is_null() {
        fprintf(
            fp,
            b"[%s] \0" as *const u8 as *const libc::c_char,
            blank_rtrim((*cur).p_name, name.as_mut_ptr()),
        );
        cur = (*cur).pred_next;
    }
    fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn print_parenthesised(mut fp: *mut FILE, mut node: *mut predicate) {
    let mut parens: libc::c_int = 0 as libc::c_int;
    if !node.is_null() {
        if ((*node).pred_func == Some(pred_or as PREDICATEFUNCTION)
            || (*node).pred_func == Some(pred_and as PREDICATEFUNCTION))
            && ((*node).pred_left).is_null()
        {
            print_parenthesised(fp, (*node).pred_right);
        } else {
            if !((*node).pred_left).is_null() || !((*node).pred_right).is_null() {
                parens = 1 as libc::c_int;
            }
            if parens != 0 {
                fprintf(
                    fp,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    b" ( \0" as *const u8 as *const libc::c_char,
                );
            }
            print_optlist(fp, node);
            if parens != 0 {
                fprintf(
                    fp,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    b" ) \0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
}
pub unsafe extern "C" fn print_optlist(mut fp: *mut FILE, mut p: *const predicate) {
    if !p.is_null() {
        print_parenthesised(fp, (*p).pred_left);
        fprintf(
            fp,
            b"%s%s%s\0" as *const u8 as *const libc::c_char,
            if (*p).need_stat as libc::c_int != 0 {
                b"[call stat] \0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            if (*p).need_type as libc::c_int != 0 {
                b"[need type] \0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            if (*p).need_inum as libc::c_int != 0 {
                b"[need inum] \0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        print_predicate(fp, p);
        fprintf(
            fp,
            b" [est success rate %.4g] \0" as *const u8 as *const libc::c_char,
            (*p).est_success_rate as libc::c_double,
        );
        if options.debug_options & DebugSuccessRates as libc::c_int as libc::c_ulong != 0
        {
            fprintf(
                fp,
                b"[real success rate %lu/%lu\0" as *const u8 as *const libc::c_char,
                (*p).perf.successes,
                (*p).perf.visits,
            );
            if (*p).perf.visits != 0 {
                let mut real_rate: libc::c_double = (*p).perf.successes as libc::c_double
                    / (*p).perf.visits as libc::c_double;
                fprintf(fp, b"=%.4g] \0" as *const u8 as *const libc::c_char, real_rate);
            } else {
                fprintf(fp, b"=_] \0" as *const u8 as *const libc::c_char);
            }
        }
        print_parenthesised(fp, (*p).pred_right);
    }
}
pub unsafe extern "C" fn show_success_rates(mut p: *const predicate) {
    if options.debug_options & DebugSuccessRates as libc::c_int as libc::c_ulong != 0 {
        fprintf(
            stderr,
            b"Predicate success rates after completion:\n\0" as *const u8
                as *const libc::c_char,
        );
        print_optlist(stderr, p);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn pred_sanity_check(mut predicates: *const predicate) {
    let mut p: *const predicate = 0 as *const predicate;
    p = predicates;
    while !p.is_null() {
        if ((*p).pred_func).is_some() {} else {
            __assert_fail(
                b"p->pred_func != NULL\0" as *const u8 as *const libc::c_char,
                b"pred.c\0" as *const u8 as *const libc::c_char,
                1310 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 49],
                    &[libc::c_char; 49],
                >(b"void pred_sanity_check(const struct predicate *)\0"))
                    .as_ptr(),
            );
        }
        'c_10731: {
            if ((*p).pred_func).is_some() {} else {
                __assert_fail(
                    b"p->pred_func != NULL\0" as *const u8 as *const libc::c_char,
                    b"pred.c\0" as *const u8 as *const libc::c_char,
                    1310 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 49],
                        &[libc::c_char; 49],
                    >(b"void pred_sanity_check(const struct predicate *)\0"))
                        .as_ptr(),
                );
            }
        };
        if !((*p).parser_entry).is_null() {} else {
            __assert_fail(
                b"p->parser_entry != NULL\0" as *const u8 as *const libc::c_char,
                b"pred.c\0" as *const u8 as *const libc::c_char,
                1313 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 49],
                    &[libc::c_char; 49],
                >(b"void pred_sanity_check(const struct predicate *)\0"))
                    .as_ptr(),
            );
        }
        'c_10684: {
            if !((*p).parser_entry).is_null() {} else {
                __assert_fail(
                    b"p->parser_entry != NULL\0" as *const u8 as *const libc::c_char,
                    b"pred.c\0" as *const u8 as *const libc::c_char,
                    1313 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 49],
                        &[libc::c_char; 49],
                    >(b"void pred_sanity_check(const struct predicate *)\0"))
                        .as_ptr(),
                );
            }
        };
        if ((*(*p).parser_entry).pred_func).is_some() {
            if (*(*p).parser_entry).pred_func == (*p).pred_func {} else {
                __assert_fail(
                    b"p->parser_entry->pred_func == p->pred_func\0" as *const u8
                        as *const libc::c_char,
                    b"pred.c\0" as *const u8 as *const libc::c_char,
                    1322 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 49],
                        &[libc::c_char; 49],
                    >(b"void pred_sanity_check(const struct predicate *)\0"))
                        .as_ptr(),
                );
            }
            'c_10628: {
                if (*(*p).parser_entry).pred_func == (*p).pred_func {} else {
                    __assert_fail(
                        b"p->parser_entry->pred_func == p->pred_func\0" as *const u8
                            as *const libc::c_char,
                        b"pred.c\0" as *const u8 as *const libc::c_char,
                        1322 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 49],
                            &[libc::c_char; 49],
                        >(b"void pred_sanity_check(const struct predicate *)\0"))
                            .as_ptr(),
                    );
                }
            };
        }
        match (*(*p).parser_entry).type_0 as libc::c_uint {
            0 | 2 => {
                if (*(*p).parser_entry).type_0 as libc::c_uint
                    != ARG_OPTION as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"p->parser_entry->type != ARG_OPTION\0" as *const u8
                            as *const libc::c_char,
                        b"pred.c\0" as *const u8 as *const libc::c_char,
                        1338 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 49],
                            &[libc::c_char; 49],
                        >(b"void pred_sanity_check(const struct predicate *)\0"))
                            .as_ptr(),
                    );
                }
                'c_10566: {
                    if (*(*p).parser_entry).type_0 as libc::c_uint
                        != ARG_OPTION as libc::c_int as libc::c_uint
                    {} else {
                        __assert_fail(
                            b"p->parser_entry->type != ARG_OPTION\0" as *const u8
                                as *const libc::c_char,
                            b"pred.c\0" as *const u8 as *const libc::c_char,
                            1338 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 49],
                                &[libc::c_char; 49],
                            >(b"void pred_sanity_check(const struct predicate *)\0"))
                                .as_ptr(),
                        );
                    }
                };
                if (*(*p).parser_entry).type_0 as libc::c_uint
                    != ARG_POSITIONAL_OPTION as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"p->parser_entry->type != ARG_POSITIONAL_OPTION\0" as *const u8
                            as *const libc::c_char,
                        b"pred.c\0" as *const u8 as *const libc::c_char,
                        1339 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 49],
                            &[libc::c_char; 49],
                        >(b"void pred_sanity_check(const struct predicate *)\0"))
                            .as_ptr(),
                    );
                }
                'c_10513: {
                    if (*(*p).parser_entry).type_0 as libc::c_uint
                        != ARG_POSITIONAL_OPTION as libc::c_int as libc::c_uint
                    {} else {
                        __assert_fail(
                            b"p->parser_entry->type != ARG_POSITIONAL_OPTION\0"
                                as *const u8 as *const libc::c_char,
                            b"pred.c\0" as *const u8 as *const libc::c_char,
                            1339 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 49],
                                &[libc::c_char; 49],
                            >(b"void pred_sanity_check(const struct predicate *)\0"))
                                .as_ptr(),
                        );
                    }
                };
            }
            6 => {
                if (*p).side_effects {} else {
                    __assert_fail(
                        b"p->side_effects\0" as *const u8 as *const libc::c_char,
                        b"pred.c\0" as *const u8 as *const libc::c_char,
                        1343 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 49],
                            &[libc::c_char; 49],
                        >(b"void pred_sanity_check(const struct predicate *)\0"))
                            .as_ptr(),
                    );
                }
                'c_10474: {
                    if (*p).side_effects {} else {
                        __assert_fail(
                            b"p->side_effects\0" as *const u8 as *const libc::c_char,
                            b"pred.c\0" as *const u8 as *const libc::c_char,
                            1343 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 49],
                                &[libc::c_char; 49],
                            >(b"void pred_sanity_check(const struct predicate *)\0"))
                                .as_ptr(),
                        );
                    }
                };
                if !((*p).pred_func == Some(pred_prune as PREDICATEFUNCTION))
                    && !((*p).pred_func
                        == ::std::mem::transmute::<
                            Option::<
                                unsafe extern "C" fn(
                                    *const libc::c_char,
                                    *mut stat,
                                    *mut predicate,
                                ) -> !,
                            >,
                            PRED_FUNC,
                        >(
                            Some(
                                pred_quit
                                    as unsafe extern "C" fn(
                                        *const libc::c_char,
                                        *mut stat,
                                        *mut predicate,
                                    ) -> !,
                            ),
                        ))
                {
                    if (*p).no_default_print {} else {
                        __assert_fail(
                            b"p->no_default_print\0" as *const u8 as *const libc::c_char,
                            b"pred.c\0" as *const u8 as *const libc::c_char,
                            1349 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 49],
                                &[libc::c_char; 49],
                            >(b"void pred_sanity_check(const struct predicate *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_10257: {
                        if (*p).no_default_print {} else {
                            __assert_fail(
                                b"p->no_default_print\0" as *const u8
                                    as *const libc::c_char,
                                b"pred.c\0" as *const u8 as *const libc::c_char,
                                1349 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 49],
                                    &[libc::c_char; 49],
                                >(b"void pred_sanity_check(const struct predicate *)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                }
            }
            4 | 3 | 5 | 1 => {
                if !(*p).no_default_print {} else {
                    __assert_fail(
                        b"!p->no_default_print\0" as *const u8 as *const libc::c_char,
                        b"pred.c\0" as *const u8 as *const libc::c_char,
                        1363 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 49],
                            &[libc::c_char; 49],
                        >(b"void pred_sanity_check(const struct predicate *)\0"))
                            .as_ptr(),
                    );
                }
                'c_10207: {
                    if !(*p).no_default_print {} else {
                        __assert_fail(
                            b"!p->no_default_print\0" as *const u8
                                as *const libc::c_char,
                            b"pred.c\0" as *const u8 as *const libc::c_char,
                            1363 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 49],
                                &[libc::c_char; 49],
                            >(b"void pred_sanity_check(const struct predicate *)\0"))
                                .as_ptr(),
                        );
                    }
                };
                if !(*p).side_effects {} else {
                    __assert_fail(
                        b"!p->side_effects\0" as *const u8 as *const libc::c_char,
                        b"pred.c\0" as *const u8 as *const libc::c_char,
                        1364 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 49],
                            &[libc::c_char; 49],
                        >(b"void pred_sanity_check(const struct predicate *)\0"))
                            .as_ptr(),
                    );
                }
                'c_10155: {
                    if !(*p).side_effects {} else {
                        __assert_fail(
                            b"!p->side_effects\0" as *const u8 as *const libc::c_char,
                            b"pred.c\0" as *const u8 as *const libc::c_char,
                            1364 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 49],
                                &[libc::c_char; 49],
                            >(b"void pred_sanity_check(const struct predicate *)\0"))
                                .as_ptr(),
                        );
                    }
                };
            }
            _ => {}
        }
        p = (*p).pred_next;
    }
}
