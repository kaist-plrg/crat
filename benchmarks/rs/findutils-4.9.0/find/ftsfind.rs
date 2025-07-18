use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type argv_iterator;
    pub type __dirstream;
    pub type cycle_check_state;
    pub type hash_table;
    pub type quoting_options;
    pub type re_dfa_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn argv_iter_free(_: *mut argv_iterator);
    fn argv_iter_init_argv(argv: *mut *mut libc::c_char) -> *mut argv_iterator;
    fn argv_iter_init_stream(fp: *mut FILE) -> *mut argv_iterator;
    fn argv_iter(_: *mut argv_iterator, _: *mut argv_iter_err) -> *mut libc::c_char;
    fn argv_iter_n_args(_: *const argv_iterator) -> size_t;
    fn set_cloexec_flag(desc: libc::c_int, value: bool) -> libc::c_int;
    fn dup_cloexec(fd: libc::c_int) -> libc::c_int;
    fn close_stdout();
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn rpl_fts_close(_: *mut FTS) -> libc::c_int;
    fn rpl_fts_open(
        _: *const *mut libc::c_char,
        _: libc::c_int,
        _: Option::<
            unsafe extern "C" fn(*mut *const FTSENT, *mut *const FTSENT) -> libc::c_int,
        >,
    ) -> *mut FTS;
    fn rpl_fts_read(_: *mut FTS) -> *mut FTSENT;
    fn rpl_fts_set(_: *mut FTS, _: *mut FTSENT, _: libc::c_int) -> libc::c_int;
    fn set_program_name(argv0: *const libc::c_char);
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn quotearg_n_style(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn xalloc_die();
    fn set_option_defaults(p: *mut options);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sharefile_init(mode: *const libc::c_char) -> sharefile_handle;
    fn debug_stat(file: *const libc::c_char, bufp: *mut stat) -> libc::c_int;
    fn cleanup();
    fn show_success_rates(node: *const predicate);
    fn build_expression_tree(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        end_of_leading_options: libc::c_int,
    ) -> *mut predicate;
    fn get_eval_tree() -> *mut predicate;
    fn complete_pending_execdirs();
    fn safely_quote_err_filename(
        n: libc::c_int,
        arg: *const libc::c_char,
    ) -> *const libc::c_char;
    fn record_initial_cwd();
    fn nonfatal_target_file_error(errno_value: libc::c_int, name: *const libc::c_char);
    fn process_leading_options(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut state: state;
    static mut options: options;
    fn apply_predicate(
        pathname: *const libc::c_char,
        stat_buf: *mut stat,
        p: *mut predicate,
    ) -> bool;
    fn digest_mode(
        mode: *mut mode_t,
        pathname: *const libc::c_char,
        name: *const libc::c_char,
        pstat: *mut stat,
        leaf: bool,
    ) -> bool;
    fn looks_like_expression(arg: *const libc::c_char, leading: bool) -> bool;
    fn remember_non_cloexec_fds();
    fn fd_leak_check_is_enabled() -> bool;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
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
pub type __syscall_slong_t = libc::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
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
pub type uintmax_t = __uintmax_t;
pub type ptrdiff_t = libc::c_long;
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
pub type argv_iter_err = libc::c_uint;
pub const AI_ERR_READ: argv_iter_err = 4;
pub const AI_ERR_MEM: argv_iter_err = 3;
pub const AI_ERR_EOF: argv_iter_err = 2;
pub const AI_ERR_OK: argv_iter_err = 1;
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct I_ring {
    pub ir_data: [libc::c_int; 4],
    pub ir_default_val: libc::c_int,
    pub ir_front: libc::c_uint,
    pub ir_back: libc::c_uint,
    pub ir_empty: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FTS {
    pub fts_cur: *mut _ftsent,
    pub fts_child: *mut _ftsent,
    pub fts_array: *mut *mut _ftsent,
    pub fts_dev: dev_t,
    pub fts_path: *mut libc::c_char,
    pub fts_rfd: libc::c_int,
    pub fts_cwd_fd: libc::c_int,
    pub fts_pathlen: size_t,
    pub fts_nitems: size_t,
    pub fts_compar: Option::<
        unsafe extern "C" fn(*mut *const _ftsent, *mut *const _ftsent) -> libc::c_int,
    >,
    pub fts_options: libc::c_int,
    pub fts_leaf_optimization_works_ht: *mut hash_table,
    pub fts_cycle: C2RustUnnamed,
    pub fts_fd_ring: I_ring,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ht: *mut hash_table,
    pub state: *mut cycle_check_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ftsent {
    pub fts_cycle: *mut _ftsent,
    pub fts_parent: *mut _ftsent,
    pub fts_link: *mut _ftsent,
    pub fts_dirp: *mut DIR,
    pub fts_number: libc::c_long,
    pub fts_pointer: *mut libc::c_void,
    pub fts_accpath: *mut libc::c_char,
    pub fts_path: *mut libc::c_char,
    pub fts_errno: libc::c_int,
    pub fts_symfd: libc::c_int,
    pub fts_pathlen: size_t,
    pub fts_fts: *mut FTS,
    pub fts_level: ptrdiff_t,
    pub fts_namelen: size_t,
    pub fts_info: libc::c_ushort,
    pub fts_flags: libc::c_ushort,
    pub fts_instr: libc::c_ushort,
    pub fts_statp: [stat; 1],
    pub fts_name: [libc::c_char; 0],
}
pub type FTSENT = _ftsent;
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
pub type DebugOption = libc::c_int;
pub const DebugAll: DebugOption = -17;
pub const DebugTime: DebugOption = 128;
pub const DebugSuccessRates: DebugOption = 64;
pub const DebugExec: DebugOption = 32;
pub const DebugHelp: DebugOption = 16;
pub const DebugTreeOpt: DebugOption = 8;
pub const DebugSearch: DebugOption = 4;
pub const DebugStat: DebugOption = 2;
pub const DebugExpressionTree: DebugOption = 1;
pub const DebugNone: DebugOption = 0;
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
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
static mut ftsoptions: libc::c_int = 0x8 as libc::c_int | 0x100 as libc::c_int
    | 0x200 as libc::c_int | 0x800 as libc::c_int;
static mut prev_depth: libc::c_int = -(2147483647 as libc::c_int) - 1 as libc::c_int;
static mut curr_fd: libc::c_int = -(1 as libc::c_int);
unsafe extern "C" fn left_dir() {
    if ftsoptions & 0x200 as libc::c_int != 0 {
        if curr_fd >= 0 as libc::c_int {
            close(curr_fd);
            curr_fd = -(1 as libc::c_int);
        }
    }
}
unsafe extern "C" fn inside_dir(mut dir_fd: libc::c_int) {
    if ftsoptions & 0x200 as libc::c_int != 0 {
        if dir_fd == -(100 as libc::c_int) || dir_fd >= 0 as libc::c_int {} else {
            __assert_fail(
                b"dir_fd == AT_FDCWD || dir_fd >= 0\0" as *const u8
                    as *const libc::c_char,
                b"ftsfind.c\0" as *const u8 as *const libc::c_char,
                107 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"void inside_dir(int)\0"))
                    .as_ptr(),
            );
        }
        'c_8331: {
            if dir_fd == -(100 as libc::c_int) || dir_fd >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"dir_fd == AT_FDCWD || dir_fd >= 0\0" as *const u8
                        as *const libc::c_char,
                    b"ftsfind.c\0" as *const u8 as *const libc::c_char,
                    107 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 21],
                        &[libc::c_char; 21],
                    >(b"void inside_dir(int)\0"))
                        .as_ptr(),
                );
            }
        };
        state.cwd_dir_fd = dir_fd;
        if curr_fd < 0 as libc::c_int {
            if -(100 as libc::c_int) == dir_fd {
                curr_fd = -(100 as libc::c_int);
            } else if dir_fd >= 0 as libc::c_int {
                curr_fd = dup_cloexec(dir_fd);
            } else {
                if curr_fd >= 0 as libc::c_int || dir_fd >= 0 as libc::c_int {} else {
                    __assert_fail(
                        b"curr_fd >= 0 || dir_fd >= 0\0" as *const u8
                            as *const libc::c_char,
                        b"ftsfind.c\0" as *const u8 as *const libc::c_char,
                        125 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 21],
                            &[libc::c_char; 21],
                        >(b"void inside_dir(int)\0"))
                            .as_ptr(),
                    );
                }
                'c_8253: {
                    if curr_fd >= 0 as libc::c_int || dir_fd >= 0 as libc::c_int
                    {} else {
                        __assert_fail(
                            b"curr_fd >= 0 || dir_fd >= 0\0" as *const u8
                                as *const libc::c_char,
                            b"ftsfind.c\0" as *const u8 as *const libc::c_char,
                            125 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 21],
                                &[libc::c_char; 21],
                            >(b"void inside_dir(int)\0"))
                                .as_ptr(),
                        );
                    }
                };
            }
        }
    }
}
unsafe extern "C" fn get_fts_info_name(mut info: libc::c_int) -> *const libc::c_char {
    static mut buf: [libc::c_char; 14] = [0; 14];
    match info {
        1 => return b"FTS_D\0" as *const u8 as *const libc::c_char,
        2 => return b"FTS_DC\0" as *const u8 as *const libc::c_char,
        3 => return b"FTS_DEFAULT\0" as *const u8 as *const libc::c_char,
        4 => return b"FTS_DNR\0" as *const u8 as *const libc::c_char,
        5 => return b"FTS_DOT\0" as *const u8 as *const libc::c_char,
        6 => return b"FTS_DP\0" as *const u8 as *const libc::c_char,
        7 => return b"FTS_ERR\0" as *const u8 as *const libc::c_char,
        8 => return b"FTS_F\0" as *const u8 as *const libc::c_char,
        9 => return b"FTS_INIT\0" as *const u8 as *const libc::c_char,
        10 => return b"FTS_NS\0" as *const u8 as *const libc::c_char,
        11 => return b"FTS_NSOK\0" as *const u8 as *const libc::c_char,
        12 => return b"FTS_SL\0" as *const u8 as *const libc::c_char,
        13 => return b"FTS_SLNONE\0" as *const u8 as *const libc::c_char,
        14 => return b"FTS_W\0" as *const u8 as *const libc::c_char,
        _ => {
            sprintf(
                buf.as_mut_ptr(),
                b"[%d]\0" as *const u8 as *const libc::c_char,
                info,
            );
            return buf.as_mut_ptr();
        }
    };
}
unsafe extern "C" fn visit(mut p: *mut FTS, mut ent: *mut FTSENT, mut pstat: *mut stat) {
    let mut eval_tree: *mut predicate = 0 as *mut predicate;
    state
        .have_stat = (*ent).fts_info as libc::c_int != 10 as libc::c_int
        && (*ent).fts_info as libc::c_int != 11 as libc::c_int;
    state.rel_pathname = (*ent).fts_accpath;
    state.cwd_dir_fd = (*p).fts_cwd_fd;
    eval_tree = get_eval_tree();
    apply_predicate((*ent).fts_path, pstat, eval_tree);
    if state.stop_at_current_level {
        rpl_fts_set(p, ent, 4 as libc::c_int);
    }
}
unsafe extern "C" fn partial_quotearg_n(
    mut n: libc::c_int,
    mut s: *mut libc::c_char,
    mut len: size_t,
    mut style: quoting_style,
) -> *const libc::c_char {
    if 0 as libc::c_int as libc::c_ulong == len {
        return quotearg_n_style(n, style, b"\0" as *const u8 as *const libc::c_char)
    } else {
        let mut saved: libc::c_char = 0;
        let mut result: *const libc::c_char = 0 as *const libc::c_char;
        saved = *s.offset(len as isize);
        *s.offset(len as isize) = 0 as libc::c_int as libc::c_char;
        result = quotearg_n_style(n, style, s);
        *s.offset(len as isize) = saved;
        return result;
    };
}
unsafe extern "C" fn issue_loop_warning(mut ent: *mut FTSENT) {
    if (*((*ent).fts_statp).as_mut_ptr()).st_mode
        & 0o170000 as libc::c_int as libc::c_uint
        == 0o120000 as libc::c_int as libc::c_uint
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Symbolic link %s is part of a loop in the directory hierarchy; we have already visited the directory to which it points.\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            safely_quote_err_filename(0 as libc::c_int, (*ent).fts_path),
        );
    } else {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"File system loop detected; %s is part of the same file system loop as %s.\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            safely_quote_err_filename(0 as libc::c_int, (*ent).fts_path),
            partial_quotearg_n(
                1 as libc::c_int,
                (*(*ent).fts_cycle).fts_path,
                (*(*ent).fts_cycle).fts_pathlen,
                options.err_quoting_style,
            ),
        );
    };
}
unsafe extern "C" fn symlink_loop(mut name: *const libc::c_char) -> bool {
    let mut stbuf: stat = stat {
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
    let rv: libc::c_int = (options.xstat).unwrap()(name, &mut stbuf);
    return 0 as libc::c_int != rv && 40 as libc::c_int == *__errno_location();
}
unsafe extern "C" fn consider_visiting(mut p: *mut FTS, mut ent: *mut FTSENT) {
    let mut statbuf: stat = stat {
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
    let mut mode: mode_t = 0;
    let mut ignore: libc::c_int = 0;
    let mut isdir: libc::c_int = 0;
    if options.debug_options & DebugSearch as libc::c_int as libc::c_ulong != 0 {
        fprintf(
            stderr,
            b"consider_visiting (early): %s: fts_info=%-6s, fts_level=%2d, prev_depth=%d fts_path=%s, fts_accpath=%s\n\0"
                as *const u8 as *const libc::c_char,
            quotearg_n_style(
                0 as libc::c_int,
                options.err_quoting_style,
                (*ent).fts_path,
            ),
            get_fts_info_name((*ent).fts_info as libc::c_int),
            (*ent).fts_level as libc::c_int,
            prev_depth,
            quotearg_n_style(
                1 as libc::c_int,
                options.err_quoting_style,
                (*ent).fts_path,
            ),
            quotearg_n_style(
                2 as libc::c_int,
                options.err_quoting_style,
                (*ent).fts_accpath,
            ),
        );
    }
    if (*ent).fts_info as libc::c_int == 6 as libc::c_int {
        left_dir();
    } else if (*ent).fts_level > prev_depth as libc::c_long
        || (*ent).fts_level == 0 as libc::c_int as libc::c_long
    {
        left_dir();
    }
    inside_dir((*p).fts_cwd_fd);
    prev_depth = (*ent).fts_level as libc::c_int;
    statbuf.st_ino = (*((*ent).fts_statp).as_mut_ptr()).st_ino;
    if (*ent).fts_info as libc::c_int == 7 as libc::c_int {
        nonfatal_target_file_error((*ent).fts_errno, (*ent).fts_path);
        return;
    }
    if (*ent).fts_info as libc::c_int == 4 as libc::c_int {
        nonfatal_target_file_error((*ent).fts_errno, (*ent).fts_path);
        if options.do_dir_first {
            return;
        }
    } else if (*ent).fts_info as libc::c_int == 2 as libc::c_int {
        issue_loop_warning(ent);
        state.exit_status = 1 as libc::c_int;
        return;
    } else if (*ent).fts_info as libc::c_int == 13 as libc::c_int {
        if symlink_loop((*ent).fts_accpath) {
            nonfatal_target_file_error(40 as libc::c_int, (*ent).fts_path);
            return;
        }
    } else if (*ent).fts_info as libc::c_int == 10 as libc::c_int {
        if (*ent).fts_level == 0 as libc::c_int as libc::c_long {
            nonfatal_target_file_error((*ent).fts_errno, (*ent).fts_path);
            return;
        } else if symlink_loop((*ent).fts_accpath) {
            nonfatal_target_file_error(40 as libc::c_int, (*ent).fts_path);
            return;
        } else {
            nonfatal_target_file_error((*ent).fts_errno, (*ent).fts_path);
        }
    }
    if (*ent).fts_info as libc::c_int == 11 as libc::c_int
        || (*ent).fts_info as libc::c_int == 10 as libc::c_int
    {
        if !state.have_stat {} else {
            __assert_fail(
                b"!state.have_stat\0" as *const u8 as *const libc::c_char,
                b"ftsfind.c\0" as *const u8 as *const libc::c_char,
                380 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"void consider_visiting(FTS *, FTSENT *)\0"))
                    .as_ptr(),
            );
        }
        'c_7777: {
            if !state.have_stat {} else {
                __assert_fail(
                    b"!state.have_stat\0" as *const u8 as *const libc::c_char,
                    b"ftsfind.c\0" as *const u8 as *const libc::c_char,
                    380 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 40],
                        &[libc::c_char; 40],
                    >(b"void consider_visiting(FTS *, FTSENT *)\0"))
                        .as_ptr(),
                );
            }
        };
        if (*ent).fts_info as libc::c_int == 11 as libc::c_int
            || state.type_0 == 0 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"ent->fts_info == FTS_NSOK || state.type == 0\0" as *const u8
                    as *const libc::c_char,
                b"ftsfind.c\0" as *const u8 as *const libc::c_char,
                381 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"void consider_visiting(FTS *, FTSENT *)\0"))
                    .as_ptr(),
            );
        }
        'c_7716: {
            if (*ent).fts_info as libc::c_int == 11 as libc::c_int
                || state.type_0 == 0 as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"ent->fts_info == FTS_NSOK || state.type == 0\0" as *const u8
                        as *const libc::c_char,
                    b"ftsfind.c\0" as *const u8 as *const libc::c_char,
                    381 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 40],
                        &[libc::c_char; 40],
                    >(b"void consider_visiting(FTS *, FTSENT *)\0"))
                        .as_ptr(),
                );
            }
        };
        mode = state.type_0;
    } else {
        state.have_stat = 1 as libc::c_int != 0;
        state.have_type = 1 as libc::c_int != 0;
        statbuf = *((*ent).fts_statp).as_mut_ptr();
        mode = statbuf.st_mode;
        state.type_0 = mode;
        if 0 as libc::c_int as libc::c_uint == mode {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"WARNING: file %s appears to have mode 0000\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style(
                    0 as libc::c_int,
                    options.err_quoting_style,
                    (*ent).fts_path,
                ),
            );
        }
    }
    state.curdepth = (*ent).fts_level as libc::c_int;
    if mode != 0 {
        if !digest_mode(
            &mut mode,
            (*ent).fts_path,
            ((*ent).fts_name).as_mut_ptr(),
            &mut statbuf,
            0 as libc::c_int != 0,
        ) {
            return;
        }
    }
    ignore = 0 as libc::c_int;
    isdir = (mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint
        || 1 as libc::c_int == (*ent).fts_info as libc::c_int
        || 6 as libc::c_int == (*ent).fts_info as libc::c_int
        || 2 as libc::c_int == (*ent).fts_info as libc::c_int) as libc::c_int;
    if isdir != 0 && (*ent).fts_info as libc::c_int == 11 as libc::c_int {
        rpl_fts_set(p, ent, 1 as libc::c_int);
        return;
    }
    if options.maxdepth >= 0 as libc::c_int {
        if (*ent).fts_level >= options.maxdepth as libc::c_long {
            rpl_fts_set(p, ent, 4 as libc::c_int);
            if (*ent).fts_level > options.maxdepth as libc::c_long {
                ignore = 1 as libc::c_int;
            }
        }
    }
    if (*ent).fts_info as libc::c_int == 1 as libc::c_int && !options.do_dir_first {
        ignore = 1 as libc::c_int;
    } else if (*ent).fts_info as libc::c_int == 6 as libc::c_int
        && options.do_dir_first as libc::c_int != 0
    {
        ignore = 1 as libc::c_int;
    } else if (*ent).fts_level < options.mindepth as libc::c_long {
        ignore = 1 as libc::c_int;
    }
    if options.debug_options & DebugSearch as libc::c_int as libc::c_ulong != 0 {
        fprintf(
            stderr,
            b"consider_visiting (late): %s: fts_info=%-6s, isdir=%d ignore=%d have_stat=%d have_type=%d \n\0"
                as *const u8 as *const libc::c_char,
            quotearg_n_style(
                0 as libc::c_int,
                options.err_quoting_style,
                (*ent).fts_path,
            ),
            get_fts_info_name((*ent).fts_info as libc::c_int),
            isdir,
            ignore,
            state.have_stat as libc::c_int,
            state.have_type as libc::c_int,
        );
    }
    if ignore == 0 {
        visit(p, ent, &mut statbuf);
    }
    if (*ent).fts_info as libc::c_int == 6 as libc::c_int {
        state.stop_at_current_level = 0 as libc::c_int != 0;
    }
}
unsafe extern "C" fn find(mut arg: *mut libc::c_char) -> bool {
    let mut arglist: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
    let mut p: *mut FTS = 0 as *mut FTS;
    let mut ent: *mut FTSENT = 0 as *mut FTSENT;
    state.starting_path_length = strlen(arg) as libc::c_int;
    inside_dir(-(100 as libc::c_int));
    arglist[0 as libc::c_int as usize] = arg;
    arglist[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
    match options.symlink_handling as libc::c_uint {
        1 => {
            ftsoptions |= 0x1 as libc::c_int | 0x2 as libc::c_int;
        }
        2 => {
            ftsoptions |= 0x1 as libc::c_int | 0x10 as libc::c_int;
        }
        0 => {
            ftsoptions |= 0x10 as libc::c_int;
        }
        _ => {}
    }
    if options.stay_on_filesystem {
        ftsoptions |= 0x40 as libc::c_int;
    }
    p = rpl_fts_open(arglist.as_mut_ptr(), ftsoptions, None);
    if p.is_null() {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"cannot search %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            safely_quote_err_filename(0 as libc::c_int, arg),
        );
        state.exit_status = 1 as libc::c_int;
    } else {
        let mut level: libc::c_int = -(2147483647 as libc::c_int) - 1 as libc::c_int;
        loop {
            *__errno_location() = 0 as libc::c_int;
            ent = rpl_fts_read(p);
            if ent.is_null() {
                break;
            }
            if state.execdirs_outstanding as libc::c_int != 0
                && (*ent).fts_level as libc::c_int != level
            {
                complete_pending_execdirs();
            }
            level = (*ent).fts_level as libc::c_int;
            state.already_issued_stat_error_msg = 0 as libc::c_int != 0;
            state.have_stat = 0 as libc::c_int != 0;
            state.have_type = (*((*ent).fts_statp).as_mut_ptr()).st_mode != 0;
            state
                .type_0 = if state.have_type as libc::c_int != 0 {
                (*((*ent).fts_statp).as_mut_ptr()).st_mode
            } else {
                0 as libc::c_int as libc::c_uint
            };
            consider_visiting(p, ent);
        }
        if *__errno_location() != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"failed to read file names from file system at or below %s\0"
                    as *const u8 as *const libc::c_char,
                safely_quote_err_filename(0 as libc::c_int, arg),
            );
            state.exit_status = 1 as libc::c_int;
            return 0 as libc::c_int != 0;
        }
        if 0 as libc::c_int != rpl_fts_close(p) {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"failed to restore working directory after searching %s\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                arg,
            );
            state.exit_status = 1 as libc::c_int;
            return 0 as libc::c_int != 0;
        }
        p = 0 as *mut FTS;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn process_all_startpoints(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> bool {
    let mut argv_starting_points: bool = (0 as libc::c_int) < argc
        && !looks_like_expression(
            *argv.offset(0 as libc::c_int as isize),
            1 as libc::c_int != 0,
        );
    let mut stream: *mut FILE = 0 as *mut FILE;
    let mut files0_filename_quoted: *const libc::c_char = 0 as *const libc::c_char;
    let mut ai: *mut argv_iterator = 0 as *mut argv_iterator;
    if !(options.files0_from).is_null() {
        if argv_starting_points {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"extra operand %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                safely_quote_err_filename(
                    0 as libc::c_int,
                    *argv.offset(0 as libc::c_int as isize),
                ),
            );
            if ::std::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"file operands cannot be combined with -files0-from\0"
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
                    b"%s\0" as *const u8 as *const libc::c_char,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"file operands cannot be combined with -files0-from\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if 0 as libc::c_int
            == strcmp(options.files0_from, b"-\0" as *const u8 as *const libc::c_char)
        {
            if options.ok_prompt_stdin {
                if ::std::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"%s\n\0" as *const u8 as *const libc::c_char,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"option -files0-from reading from standard input cannot be combined with -ok, -okdir\0"
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
                        b"%s\n\0" as *const u8 as *const libc::c_char,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"option -files0-from reading from standard input cannot be combined with -ok, -okdir\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            files0_filename_quoted = safely_quote_err_filename(
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"(standard input)\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            stream = stdin;
        } else {
            files0_filename_quoted = safely_quote_err_filename(
                0 as libc::c_int,
                options.files0_from,
            );
            stream = fopen(
                options.files0_from,
                b"r\0" as *const u8 as *const libc::c_char,
            );
            if stream.is_null() {
                if ::std::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot open %s for reading\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        files0_filename_quoted,
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
                            b"cannot open %s for reading\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        files0_filename_quoted,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            let fd: libc::c_int = fileno(stream);
            if fd >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"fd >= 0\0" as *const u8 as *const libc::c_char,
                    b"ftsfind.c\0" as *const u8 as *const libc::c_char,
                    610 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 44],
                        &[libc::c_char; 44],
                    >(b"_Bool process_all_startpoints(int, char **)\0"))
                        .as_ptr(),
                );
            }
            'c_9321: {
                if fd >= 0 as libc::c_int {} else {
                    __assert_fail(
                        b"fd >= 0\0" as *const u8 as *const libc::c_char,
                        b"ftsfind.c\0" as *const u8 as *const libc::c_char,
                        610 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 44],
                            &[libc::c_char; 44],
                        >(b"_Bool process_all_startpoints(int, char **)\0"))
                            .as_ptr(),
                    );
                }
            };
            if options.ok_prompt_stdin {
                let mut sb1: stat = stat {
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
                let mut sb2: stat = stat {
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
                if fstat(fd, &mut sb1) == 0 as libc::c_int
                    && fstat(0 as libc::c_int, &mut sb2) == 0 as libc::c_int
                    && (sb1.st_ino == sb2.st_ino && sb1.st_dev == sb2.st_dev)
                {
                    if ::std::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            b"%s: %s\n\0" as *const u8 as *const libc::c_char,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"option -files0-from: standard input must not refer to the same file when combined with -ok, -okdir\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            files0_filename_quoted,
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            b"%s: %s\n\0" as *const u8 as *const libc::c_char,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"option -files0-from: standard input must not refer to the same file when combined with -ok, -okdir\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            files0_filename_quoted,
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
            }
            set_cloexec_flag(fd, 1 as libc::c_int != 0);
        }
        ai = argv_iter_init_stream(stream);
    } else {
        if !argv_starting_points {
            let mut defaultpath: [libc::c_char; 2] = *::std::mem::transmute::<
                &[u8; 2],
                &mut [libc::c_char; 2],
            >(b".\0");
            return find(defaultpath.as_mut_ptr());
        }
        ai = argv_iter_init_argv(argv);
    }
    if ai.is_null() {
        xalloc_die();
    }
    let mut ok: bool = 1 as libc::c_int != 0;
    loop {
        let mut ai_err: argv_iter_err = 0 as argv_iter_err;
        let mut file_name: *mut libc::c_char = argv_iter(ai, &mut ai_err);
        if file_name.is_null() {
            match ai_err as libc::c_uint {
                2 => {
                    break;
                }
                4 => {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: read error\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        files0_filename_quoted,
                    );
                    state.exit_status = 1 as libc::c_int;
                    ok = 0 as libc::c_int != 0;
                    break;
                }
                3 => {
                    xalloc_die();
                }
                _ => {}
            }
            if (b"unexpected error code from argv_iter\0" as *const u8
                as *const libc::c_char)
                .is_null()
            {} else {
                __assert_fail(
                    b"!\"unexpected error code from argv_iter\"\0" as *const u8
                        as *const libc::c_char,
                    b"ftsfind.c\0" as *const u8 as *const libc::c_char,
                    675 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 44],
                        &[libc::c_char; 44],
                    >(b"_Bool process_all_startpoints(int, char **)\0"))
                        .as_ptr(),
                );
            }
            'c_9022: {
                if (b"unexpected error code from argv_iter\0" as *const u8
                    as *const libc::c_char)
                    .is_null()
                {} else {
                    __assert_fail(
                        b"!\"unexpected error code from argv_iter\"\0" as *const u8
                            as *const libc::c_char,
                        b"ftsfind.c\0" as *const u8 as *const libc::c_char,
                        675 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 44],
                            &[libc::c_char; 44],
                        >(b"_Bool process_all_startpoints(int, char **)\0"))
                            .as_ptr(),
                    );
                }
            };
        }
        if *file_name.offset(0 as libc::c_int as isize) == 0 {
            if (options.files0_from).is_null() {
                error(
                    0 as libc::c_int,
                    2 as libc::c_int,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    safely_quote_err_filename(0 as libc::c_int, file_name),
                );
            } else {
                let mut file_number: libc::c_ulong = argv_iter_n_args(ai);
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"%s:%lu: %s\0" as *const u8 as *const libc::c_char,
                    files0_filename_quoted,
                    file_number,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid zero-length file name\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            state.exit_status = 1 as libc::c_int;
            ok = 0 as libc::c_int != 0;
        } else {
            if (options.files0_from).is_null()
                && looks_like_expression(file_name, 1 as libc::c_int != 0) as libc::c_int
                    != 0
            {
                break;
            }
            state.starting_path_length = strlen(file_name) as libc::c_int;
            if find(file_name) {
                continue;
            }
            ok = 0 as libc::c_int != 0;
            break;
        }
    }
    argv_iter_free(ai);
    if ok as libc::c_int != 0 && !(options.files0_from).is_null()
        && (ferror(stream) != 0 || rpl_fclose(stream) != 0 as libc::c_int)
    {
        if ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"error reading %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                files0_filename_quoted,
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
                    b"error reading %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                files0_filename_quoted,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    return ok;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut end_of_leading_options: libc::c_int = 0 as libc::c_int;
    let mut eval_tree: *mut predicate = 0 as *mut predicate;
    if !(*argv.offset(0 as libc::c_int as isize)).is_null() {
        set_program_name(*argv.offset(0 as libc::c_int as isize));
    } else {
        set_program_name(b"find\0" as *const u8 as *const libc::c_char);
    }
    record_initial_cwd();
    state.already_issued_stat_error_msg = 0 as libc::c_int != 0;
    state.exit_status = 0 as libc::c_int;
    state.execdirs_outstanding = 0 as libc::c_int != 0;
    state.cwd_dir_fd = -(100 as libc::c_int);
    if fd_leak_check_is_enabled() {
        remember_non_cloexec_fds();
    }
    state.shared_files = sharefile_init(b"w\0" as *const u8 as *const libc::c_char);
    if (state.shared_files).is_null() {
        if ::std::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to initialize shared-file hash table\0" as *const u8
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
                    b"Failed to initialize shared-file hash table\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    set_option_defaults(&mut options);
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"findutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"findutils\0" as *const u8 as *const libc::c_char);
    if atexit(Some(close_stdout as unsafe extern "C" fn() -> ())) != 0 {
        if ::std::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"The atexit library function failed\0" as *const u8
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
                    b"The atexit library function failed\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    end_of_leading_options = process_leading_options(argc, argv);
    if options.debug_options & DebugStat as libc::c_int as libc::c_ulong != 0 {
        options
            .xstat = Some(
            debug_stat
                as unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
        );
    }
    if options.debug_options & DebugTime as libc::c_int as libc::c_ulong != 0 {
        fprintf(
            stderr,
            b"cur_day_start = %s\0" as *const u8 as *const libc::c_char,
            ctime(&mut options.cur_day_start.tv_sec),
        );
    }
    eval_tree = build_expression_tree(argc, argv, end_of_leading_options);
    !options.open_nofollow_available;
    if process_all_startpoints(
        argc - end_of_leading_options,
        argv.offset(end_of_leading_options as isize),
    ) {
        show_success_rates(eval_tree);
        cleanup();
    }
    return state.exit_status;
}
pub unsafe extern "C" fn is_fts_enabled(mut fts_options: *mut libc::c_int) -> bool {
    *fts_options = ftsoptions;
    return 1 as libc::c_int != 0;
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
