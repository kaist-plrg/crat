use ::libc;
use ::c2rust_bitfields;
extern "C" {
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
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn rpl_free(ptr: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn xzalloc(s: size_t) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    static mut state: state;
    fn looks_like_expression(arg: *const libc::c_char, leading: bool) -> bool;
    fn default_prints(pred: *mut predicate) -> bool;
    static mut options: options;
    fn print_optlist(fp: *mut FILE, node: *const predicate);
    fn print_list(_: *mut FILE, node: *mut predicate);
    fn pred_quit(
        pathname: *const libc::c_char,
        stat_buf: *mut stat,
        pred_ptr: *mut predicate,
    ) -> !;
    fn pred_context(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_xtype(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_writable(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_user(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_used(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_uid(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_type(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_true(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_size(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_samefile(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_regex(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_readable(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_prune(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_print0(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_print(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_perm(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_path(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_or(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_openparen(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_okdir(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_ok(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_nouser(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_nogroup(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_newerXY(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_newer(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_negate(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_name(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_mtime(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_mmin(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_ls(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_lname(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_links(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_ipath(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_inum(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_iname(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_ilname(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_group(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_gid(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_fstype(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_fprintf(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_fprint0(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_fprint(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_fls(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_false(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_executable(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_execdir(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn find_parser(search_name: *const libc::c_char) -> *const parser_table;
    fn parse_print(
        _: *const parser_table,
        argv: *mut *mut libc::c_char,
        arg_ptr: *mut libc::c_int,
    ) -> bool;
    fn pred_sanity_check(predicates_0: *const predicate);
    fn check_option_combinations(p: *const predicate);
    fn parse_begin_user_args(
        args: *mut *mut libc::c_char,
        argno: libc::c_int,
        last: *const predicate,
        predicates_0: *const predicate,
    );
    fn parse_end_user_args(
        args: *mut *mut libc::c_char,
        argno: libc::c_int,
        last: *const predicate,
        predicates_0: *const predicate,
    );
    fn parse_openparen(
        entry: *const parser_table,
        argv: *mut *mut libc::c_char,
        arg_ptr: *mut libc::c_int,
    ) -> bool;
    fn parse_closeparen(
        entry: *const parser_table,
        argv: *mut *mut libc::c_char,
        arg_ptr: *mut libc::c_int,
    ) -> bool;
    fn pred_amin(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_and(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_anewer(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_atime(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_closeparen(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_cmin(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_cnewer(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_comma(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_ctime(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_delete(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_empty(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn pred_exec(_: *const libc::c_char, _: *mut stat, _: *mut predicate) -> bool;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub type uintmax_t = __uintmax_t;
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
pub type PREDICATEFUNCTION = unsafe extern "C" fn(
    *const libc::c_char,
    *mut stat,
    *mut predicate,
) -> bool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cost_assoc {
    pub cost: EvaluationCost,
    pub name: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct prec_assoc {
    pub prec: libc::c_short,
    pub prec_name: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct op_assoc {
    pub type_0: libc::c_short,
    pub type_name: *const libc::c_char,
}
pub const DebugTreeOpt: DebugOption = 8;
pub const DebugExpressionTree: DebugOption = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct predlist {
    pub head: *mut predicate,
    pub tail: *mut predicate,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pred_cost_lookup {
    pub fn_0: PRED_FUNC,
    pub cost: EvaluationCost,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub pfn: PRED_FUNC,
    pub mem: [libc::c_char; 8],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub _gl_dummy: libc::c_int,
}
pub type DebugOption = libc::c_int;
pub const DebugAll: DebugOption = -17;
pub const DebugTime: DebugOption = 128;
pub const DebugSuccessRates: DebugOption = 64;
pub const DebugExec: DebugOption = 32;
pub const DebugHelp: DebugOption = 16;
pub const DebugSearch: DebugOption = 4;
pub const DebugStat: DebugOption = 2;
pub const DebugNone: DebugOption = 0;
#[inline]
unsafe extern "C" fn bsearch(
    mut __key: *const libc::c_void,
    mut __base: *const libc::c_void,
    mut __nmemb: size_t,
    mut __size: size_t,
    mut __compar: __compar_fn_t,
) -> *mut libc::c_void {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p: *const libc::c_void = 0 as *const libc::c_void;
    let mut __comparison: libc::c_int = 0;
    __l = 0 as libc::c_int as size_t;
    __u = __nmemb;
    while __l < __u {
        __idx = __l.wrapping_add(__u).wrapping_div(2 as libc::c_int as libc::c_ulong);
        __p = (__base as *const libc::c_char).offset(__idx.wrapping_mul(__size) as isize)
            as *mut libc::c_void;
        __comparison = (Some(__compar.unwrap())).unwrap()(__key, __p);
        if __comparison < 0 as libc::c_int {
            __u = __idx;
        } else if __comparison > 0 as libc::c_int {
            __l = __idx.wrapping_add(1 as libc::c_int as libc::c_ulong);
        } else {
            return __p as *mut libc::c_void
        }
    }
    return 0 as *mut libc::c_void;
}
static mut predicates: *mut predicate = 0 as *const predicate as *mut predicate;
static mut eval_tree: *mut predicate = 0 as *const predicate as *mut predicate;
static mut last_pred: *mut predicate = 0 as *const predicate as *mut predicate;
static mut start_points: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut num_start_points: size_t = 0 as libc::c_int as size_t;
pub unsafe extern "C" fn matches_start_point(
    mut glob: *const libc::c_char,
    mut foldcase: bool,
) -> bool {
    let mut fnmatch_flags: libc::c_int = 0 as libc::c_int;
    if foldcase {
        fnmatch_flags |= (1 as libc::c_int) << 4 as libc::c_int;
    }
    if num_start_points != 0 {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < num_start_points {
            if fnmatch(glob, *start_points.offset(i as isize), fnmatch_flags)
                == 0 as libc::c_int
            {
                return 1 as libc::c_int != 0;
            }
            i = i.wrapping_add(1);
            i;
        }
        return 0 as libc::c_int != 0;
    } else {
        return fnmatch(glob, b".\0" as *const u8 as *const libc::c_char, fnmatch_flags)
            == 0 as libc::c_int
    };
}
unsafe extern "C" fn get_expr(
    mut input: *mut *mut predicate,
    mut prev_prec: libc::c_short,
    mut prev_pred: *const predicate,
) -> *mut predicate {
    let mut next: *mut predicate = 0 as *mut predicate;
    let mut this_pred: *mut predicate = *input;
    if (*input).is_null() {
        if ::std::mem::size_of::<C2RustUnnamed_20>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"invalid expression\0" as *const u8 as *const libc::c_char,
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
                    b"invalid expression\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    match (**input).p_type as libc::c_uint {
        0 => {
            if ::std::mem::size_of::<C2RustUnnamed_19>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid expression\0" as *const u8 as *const libc::c_char,
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
                        b"invalid expression\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        3 => {
            if ::std::mem::size_of::<C2RustUnnamed_18>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid expression; you have used a binary operator '%s' with nothing before it.\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*this_pred).p_name,
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
                        b"invalid expression; you have used a binary operator '%s' with nothing before it.\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*this_pred).p_name,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        5 => {
            if prev_pred.is_null() {
                if ::std::mem::size_of::<C2RustUnnamed_17>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"invalid expression: expected expression before closing parentheses '%s'.\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*this_pred).p_name,
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
                            b"invalid expression: expected expression before closing parentheses '%s'.\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*this_pred).p_name,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            if (UNI_OP as libc::c_int as libc::c_uint
                == (*prev_pred).p_type as libc::c_uint
                || BI_OP as libc::c_int as libc::c_uint
                    == (*prev_pred).p_type as libc::c_uint) && !(*this_pred).artificial
            {
                if ::std::mem::size_of::<C2RustUnnamed_16>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"expected an expression between '%s' and ')'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*prev_pred).p_name,
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
                            b"expected an expression between '%s' and ')'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*prev_pred).p_name,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            } else if (**input).artificial {
                if ::std::mem::size_of::<C2RustUnnamed_15>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"expected an expression after '%s'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*prev_pred).p_name,
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
                            b"expected an expression after '%s'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*prev_pred).p_name,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            } else {
                if ::std::mem::size_of::<C2RustUnnamed_14>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"invalid expression; you have too many ')'\0" as *const u8
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
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"invalid expression; you have too many ')'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
        }
        1 => {
            next = *input;
            *input = (**input).pred_next;
        }
        2 => {
            next = *input;
            *input = (**input).pred_next;
            (*next)
                .pred_right = get_expr(
                input,
                NEGATE_PREC as libc::c_int as libc::c_short,
                next,
            );
        }
        4 => {
            if ((**input).pred_next).is_null()
                || (*(**input).pred_next).artificial as libc::c_int != 0
            {
                if ::std::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"invalid expression; expected to find a ')' but didn't see one. Perhaps you need an extra predicate after '%s'\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*this_pred).p_name,
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
                            b"invalid expression; expected to find a ')' but didn't see one. Perhaps you need an extra predicate after '%s'\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*this_pred).p_name,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            prev_pred = *input;
            *input = (**input).pred_next;
            if (**input).p_type as libc::c_uint
                == CLOSE_PAREN as libc::c_int as libc::c_uint
            {
                if (*prev_pred).artificial {
                    if ::std::mem::size_of::<C2RustUnnamed_12>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"invalid expression: expected expression before closing parentheses '%s'.\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (**input).p_name,
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
                                b"invalid expression: expected expression before closing parentheses '%s'.\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (**input).p_name,
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
                if ::std::mem::size_of::<C2RustUnnamed_11>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"invalid expression; empty parentheses are not allowed.\0"
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
                            b"invalid expression; empty parentheses are not allowed.\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            next = get_expr(input, NO_PREC as libc::c_int as libc::c_short, prev_pred);
            if (*input).is_null()
                || (**input).p_type as libc::c_uint
                    != CLOSE_PAREN as libc::c_int as libc::c_uint
            {
                if ::std::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"invalid expression; I was expecting to find a ')' somewhere but did not see one.\0"
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
                            b"invalid expression; I was expecting to find a ')' somewhere but did not see one.\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            *input = (**input).pred_next;
        }
        _ => {
            if ::std::mem::size_of::<C2RustUnnamed_9>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"oops -- invalid expression type!\0" as *const u8
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
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"oops -- invalid expression type!\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    if (*input).is_null() {
        return next;
    }
    if (**input).p_prec as libc::c_int > prev_prec as libc::c_int {
        next = scan_rest(input, next, prev_prec);
        if next.is_null() {
            if ::std::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid expression\0" as *const u8 as *const libc::c_char,
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
                        b"invalid expression\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    return next;
}
unsafe extern "C" fn scan_rest(
    mut input: *mut *mut predicate,
    mut head: *mut predicate,
    mut prev_prec: libc::c_short,
) -> *mut predicate {
    let mut tree: *mut predicate = 0 as *mut predicate;
    if (*input).is_null()
        || (**input).p_type as libc::c_uint == CLOSE_PAREN as libc::c_int as libc::c_uint
    {
        return 0 as *mut predicate;
    }
    tree = head;
    while !(*input).is_null()
        && (**input).p_prec as libc::c_int > prev_prec as libc::c_int
    {
        match (**input).p_type as libc::c_uint {
            0 | 1 | 2 | 4 => {
                if ::std::mem::size_of::<C2RustUnnamed_8>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"invalid expression\0" as *const u8 as *const libc::c_char,
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
                            b"invalid expression\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            3 => {
                let mut prev: *mut predicate = *input;
                (**input).pred_left = tree;
                tree = *input;
                *input = (**input).pred_next;
                (*tree)
                    .pred_right = get_expr(input, (*tree).p_prec as libc::c_short, prev);
            }
            5 => return tree,
            _ => {
                if ::std::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"oops -- invalid expression type (%d)!\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (**input).p_type as libc::c_int,
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
                            b"oops -- invalid expression type (%d)!\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (**input).p_type as libc::c_int,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
        }
    }
    return tree;
}
unsafe extern "C" fn predicate_is_cost_free(mut p: *const predicate) -> bool {
    if (*p).pred_func == Some(pred_name as PREDICATEFUNCTION)
        || (*p).pred_func == Some(pred_path as PREDICATEFUNCTION)
        || (*p).pred_func == Some(pred_iname as PREDICATEFUNCTION)
        || (*p).pred_func == Some(pred_ipath as PREDICATEFUNCTION)
    {
        return 1 as libc::c_int != 0
    } else if options.optimisation_level as libc::c_int > 0 as libc::c_int {
        if (*p).pred_func == Some(pred_and as PREDICATEFUNCTION)
            || (*p).pred_func == Some(pred_negate as PREDICATEFUNCTION)
            || (*p).pred_func == Some(pred_comma as PREDICATEFUNCTION)
            || (*p).pred_func == Some(pred_or as PREDICATEFUNCTION)
        {
            return 0 as libc::c_int != 0
        } else {
            return NeedsNothing as libc::c_int as libc::c_uint
                == (*p).p_cost as libc::c_uint
        }
    } else {
        return 0 as libc::c_int != 0
    };
}
pub unsafe extern "C" fn print_predicate(mut fp: *mut FILE, mut p: *const predicate) {
    if !((*p).arg_text).is_null() {
        fprintf(
            fp,
            b"%s %s\0" as *const u8 as *const libc::c_char,
            (*p).p_name,
            (*p).arg_text,
        );
    } else {
        fprintf(fp, b"%s\0" as *const u8 as *const libc::c_char, (*p).p_name);
    };
}
unsafe extern "C" fn predlist_init(mut p: *mut predlist) {
    (*p).tail = 0 as *mut predicate;
    (*p).head = (*p).tail;
}
unsafe extern "C" fn predlist_insert(
    mut list: *mut predlist,
    mut curr: *mut predicate,
    mut pprev: *mut *mut predicate,
) {
    let mut insertpos: *mut *mut predicate = &mut (*list).head;
    *pprev = (*curr).pred_left;
    (*curr).pred_left = *insertpos;
    *insertpos = curr;
    if ((*list).tail).is_null() {
        (*list).tail = (*list).head;
    }
}
unsafe extern "C" fn pred_cost_compare(
    mut p1: *const predicate,
    mut p2: *const predicate,
    mut wantfailure: bool,
) -> libc::c_int {
    if (*p1).p_cost as libc::c_uint == (*p2).p_cost as libc::c_uint {
        if (*p1).est_success_rate == (*p2).est_success_rate {
            return 0 as libc::c_int
        } else if wantfailure {
            return if (*p1).est_success_rate < (*p2).est_success_rate {
                -(1 as libc::c_int)
            } else {
                1 as libc::c_int
            }
        } else {
            return if (*p1).est_success_rate < (*p2).est_success_rate {
                1 as libc::c_int
            } else {
                -(1 as libc::c_int)
            }
        }
    } else {
        return if ((*p1).p_cost as libc::c_uint) < (*p2).p_cost as libc::c_uint {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        }
    };
}
unsafe extern "C" fn predlist_merge_sort(
    mut list: *mut predlist,
    mut last: *mut *mut predicate,
) {
    let mut new_list: predlist = predlist {
        head: 0 as *mut predicate,
        tail: 0 as *mut predicate,
    };
    let mut p: *mut predicate = 0 as *mut predicate;
    let mut q: *mut predicate = 0 as *mut predicate;
    if ((*list).head).is_null() {
        return;
    }
    if options.debug_options & DebugTreeOpt as libc::c_int as libc::c_ulong != 0 {
        fprintf(
            stderr,
            b"%s:\n\0" as *const u8 as *const libc::c_char,
            b"predlist before merge sort\0" as *const u8 as *const libc::c_char,
        );
        print_tree(stderr, (*list).head, 2 as libc::c_int);
    }
    calculate_derived_rates((*list).head);
    predlist_init(&mut new_list);
    while !((*list).head).is_null() {
        q = (*list).head;
        (*list).head = (*(*list).head).pred_left;
        (*q).pred_left = 0 as *mut predicate;
        p = new_list.head;
        while !p.is_null() {
            let wantfailure: bool = OR_PREC as libc::c_int as libc::c_uint
                != (*p).p_prec as libc::c_uint;
            if pred_cost_compare((*p).pred_right, (*q).pred_right, wantfailure)
                >= 0 as libc::c_int
            {
                break;
            }
            p = (*p).pred_left;
        }
        if !p.is_null() {
            (*q).pred_left = (*p).pred_left;
            if ((*q).pred_left).is_null() {
                new_list.tail = q;
            }
            (*p).pred_left = q;
        } else {
            (*q).pred_left = new_list.head;
            new_list.head = q;
            if (new_list.tail).is_null() {
                new_list.tail = q;
            }
        }
    }
    if options.debug_options & DebugTreeOpt as libc::c_int as libc::c_ulong != 0 {
        fprintf(
            stderr,
            b"%s:\n\0" as *const u8 as *const libc::c_char,
            b"predlist after merge sort\0" as *const u8 as *const libc::c_char,
        );
        print_tree(stderr, new_list.head, 2 as libc::c_int);
    }
    calculate_derived_rates(new_list.head);
    merge_pred(new_list.head, new_list.tail, last);
    predlist_init(list);
}
unsafe extern "C" fn merge_lists(
    mut lists: *mut predlist,
    mut nlists: libc::c_int,
    mut name_list: *mut predlist,
    mut regex_list: *mut predlist,
    mut last: *mut *mut predicate,
) {
    let mut i: libc::c_int = 0;
    static mut mergefn: Option::<
        unsafe extern "C" fn(*mut predlist, *mut *mut predicate) -> (),
    > = None;
    mergefn = Some(
        predlist_merge_sort
            as unsafe extern "C" fn(*mut predlist, *mut *mut predicate) -> (),
    );
    mergefn.unwrap()(name_list, last);
    mergefn.unwrap()(regex_list, last);
    i = 0 as libc::c_int;
    while i < nlists {
        mergefn.unwrap()(&mut *lists.offset(i as isize), last);
        i += 1;
        i;
    }
}
unsafe extern "C" fn subtree_has_side_effects(mut p: *const predicate) -> bool {
    if !p.is_null() {
        return (*p).side_effects as libc::c_int != 0
            || subtree_has_side_effects((*p).pred_left) as libc::c_int != 0
            || subtree_has_side_effects((*p).pred_right) as libc::c_int != 0
    } else {
        return 0 as libc::c_int != 0
    };
}
unsafe extern "C" fn worst_cost(mut p: *const predicate) -> libc::c_int {
    if !p.is_null() {
        let mut cost_r: libc::c_uint = 0;
        let mut cost_l: libc::c_uint = 0;
        let mut worst: libc::c_uint = 0;
        cost_l = worst_cost((*p).pred_left) as libc::c_uint;
        cost_r = worst_cost((*p).pred_right) as libc::c_uint;
        worst = if cost_l > cost_r { cost_l } else { cost_r };
        if worst < (*p).p_cost as libc::c_uint {
            worst = (*p).p_cost as libc::c_uint;
        }
        return worst as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn perform_arm_swap(mut p: *mut predicate) {
    let mut tmp: *mut predicate = (*(*p).pred_left).pred_right;
    (*(*p).pred_left).pred_right = (*p).pred_right;
    (*p).pred_right = tmp;
}
unsafe extern "C" fn consider_arm_swap(mut p: *mut predicate) -> bool {
    let mut left_cost: libc::c_int = 0;
    let mut right_cost: libc::c_int = 0;
    let mut reason: *const libc::c_char = 0 as *const libc::c_char;
    let mut pl: *mut *mut predicate = 0 as *mut *mut predicate;
    let mut pr: *mut *mut predicate = 0 as *mut *mut predicate;
    if BI_OP as libc::c_int as libc::c_uint != (*p).p_type as libc::c_uint {
        reason = b"Not a binary operation\0" as *const u8 as *const libc::c_char;
    }
    if reason.is_null() {
        if ((*p).pred_left).is_null() || ((*p).pred_right).is_null() {
            reason = b"Doesn't have two arms\0" as *const u8 as *const libc::c_char;
        }
    }
    if reason.is_null() {
        if ((*(*p).pred_left).pred_right).is_null() {
            reason = b"Left arm has no child on RHS\0" as *const u8
                as *const libc::c_char;
        }
    }
    pr = &mut (*p).pred_right;
    pl = &mut (*(*p).pred_left).pred_right;
    if reason.is_null() {
        if subtree_has_side_effects(*pl) {
            reason = b"Left subtree has side-effects\0" as *const u8
                as *const libc::c_char;
        }
    }
    if reason.is_null() {
        if subtree_has_side_effects(*pr) {
            reason = b"Right subtree has side-effects\0" as *const u8
                as *const libc::c_char;
        }
    }
    if reason.is_null() {
        left_cost = worst_cost(*pl);
        right_cost = worst_cost(*pr);
        if left_cost < right_cost {
            reason = b"efficient as-is\0" as *const u8 as *const libc::c_char;
        }
    }
    if reason.is_null() {
        let mut want_swap: bool = false;
        if left_cost == right_cost {
            let mut succ_rate_l: libc::c_float = (**pl).est_success_rate;
            let mut succ_rate_r: libc::c_float = (**pr).est_success_rate;
            if options.debug_options & DebugTreeOpt as libc::c_int as libc::c_ulong != 0
            {
                fprintf(
                    stderr,
                    b"Success rates: l=%f, r=%f\n\0" as *const u8 as *const libc::c_char,
                    succ_rate_l as libc::c_double,
                    succ_rate_r as libc::c_double,
                );
            }
            if (*p).pred_func == Some(pred_or as PREDICATEFUNCTION) {
                want_swap = succ_rate_r < succ_rate_l;
                if !want_swap {
                    reason = b"Operation is OR; right success rate >= left\0"
                        as *const u8 as *const libc::c_char;
                }
            } else if (*p).pred_func == Some(pred_and as PREDICATEFUNCTION) {
                want_swap = succ_rate_r > succ_rate_l;
                if !want_swap {
                    reason = b"Operation is AND; right success rate <= left\0"
                        as *const u8 as *const libc::c_char;
                }
            } else {
                want_swap = 0 as libc::c_int != 0;
                reason = b"Not 'AND' or 'OR'\0" as *const u8 as *const libc::c_char;
            }
        } else {
            want_swap = 1 as libc::c_int != 0;
        }
        if want_swap {
            if options.debug_options & DebugTreeOpt as libc::c_int as libc::c_ulong != 0
            {
                fprintf(
                    stderr,
                    b"Performing arm swap on:\n\0" as *const u8 as *const libc::c_char,
                );
                print_tree(stderr, p, 0 as libc::c_int);
            }
            perform_arm_swap(p);
            return 1 as libc::c_int != 0;
        }
    }
    if options.debug_options & DebugTreeOpt as libc::c_int as libc::c_ulong != 0 {
        fprintf(
            stderr,
            b"Not an arm swap candidate (%s):\n\0" as *const u8 as *const libc::c_char,
            reason,
        );
        print_tree(stderr, p, 0 as libc::c_int);
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn do_arm_swaps(mut p: *mut predicate) -> bool {
    if !p.is_null() {
        let mut swapped: bool = false;
        loop {
            swapped = 0 as libc::c_int != 0;
            if consider_arm_swap(p) as libc::c_int != 0
                || do_arm_swaps((*p).pred_left) as libc::c_int != 0
                || do_arm_swaps((*p).pred_right) as libc::c_int != 0
            {
                swapped = 1 as libc::c_int != 0;
            }
            if !swapped {
                break;
            }
        }
        return swapped;
    } else {
        return 0 as libc::c_int != 0
    };
}
unsafe extern "C" fn opt_expr(mut eval_treep: *mut *mut predicate) -> bool {
    let mut regex_list: predlist = {
        let mut init = predlist {
            head: 0 as *mut predicate,
            tail: 0 as *mut predicate,
        };
        init
    };
    let mut name_list: predlist = {
        let mut init = predlist {
            head: 0 as *mut predicate,
            tail: 0 as *mut predicate,
        };
        init
    };
    let mut cbo_list: [predlist; 11] = [predlist {
        head: 0 as *mut predicate,
        tail: 0 as *mut predicate,
    }; 11];
    let mut i: libc::c_int = 0;
    let mut curr: *mut predicate = 0 as *mut predicate;
    let mut prevp: *mut *mut predicate = 0 as *mut *mut predicate;
    let mut last_sidep: *mut *mut predicate = 0 as *mut *mut predicate;
    let mut pred_func: PRED_FUNC = None;
    let mut p_type: predicate_type = NO_TYPE;
    let mut has_side_effects: bool = 0 as libc::c_int != 0;
    let mut prev_prec: predicate_precedence = NO_PREC;
    let mut biop_prec: predicate_precedence = NO_PREC;
    if eval_treep.is_null() || (*eval_treep).is_null() {
        return 0 as libc::c_int != 0;
    }
    i = 0 as libc::c_int;
    while i < NumEvaluationCosts as libc::c_int {
        predlist_init(&mut *cbo_list.as_mut_ptr().offset(i as isize));
        i += 1;
        i;
    }
    prevp = eval_treep;
    prev_prec = AND_PREC;
    curr = *prevp;
    while !((*curr).pred_left).is_null() {
        prevp = &mut (*curr).pred_left;
        prev_prec = (*curr).p_prec;
        curr = (*curr).pred_left;
    }
    if (*curr).p_type as libc::c_uint != BI_OP as libc::c_int as libc::c_uint {
        set_new_parent(curr, prev_prec, prevp);
    }
    if options.debug_options
        & (DebugExpressionTree as libc::c_int | DebugTreeOpt as libc::c_int)
            as libc::c_ulong != 0
    {
        fprintf(
            stderr,
            b"Normalized Eval Tree:\n\0" as *const u8 as *const libc::c_char,
        );
        print_tree(stderr, *eval_treep, 0 as libc::c_int);
    }
    prevp = eval_treep;
    biop_prec = NO_PREC;
    if !(*prevp).is_null()
        && (**prevp).p_type as libc::c_uint == BI_OP as libc::c_int as libc::c_uint
    {
        biop_prec = (**prevp).p_prec;
    }
    loop {
        curr = *prevp;
        if curr.is_null() {
            break;
        }
        if (*curr).p_type as libc::c_uint == BI_OP as libc::c_int as libc::c_uint {
            if (*curr).p_prec as libc::c_uint != biop_prec as libc::c_uint {
                curr = set_new_parent(curr, biop_prec, prevp);
            }
        }
        p_type = (*(*curr).pred_right).p_type;
        pred_func = (*(*curr).pred_right).pred_func;
        match p_type as libc::c_uint {
            0 | 1 => {
                if !(biop_prec as libc::c_uint
                    == COMMA_PREC as libc::c_int as libc::c_uint)
                {
                    if !(*(*curr).pred_right).side_effects {
                        let mut reorder: bool = false;
                        if predicate_is_cost_free((*curr).pred_right) {
                            if options.debug_options
                                & DebugTreeOpt as libc::c_int as libc::c_ulong != 0
                            {
                                fprintf(
                                    stderr,
                                    b"-O%d: promoting cheap predicate \0" as *const u8
                                        as *const libc::c_char,
                                    options.optimisation_level as libc::c_int,
                                );
                                print_predicate(stderr, (*curr).pred_right);
                                fprintf(
                                    stderr,
                                    b" into name_list\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            predlist_insert(&mut name_list, curr, prevp);
                            continue;
                        } else if pred_func == Some(pred_regex as PREDICATEFUNCTION) {
                            predlist_insert(&mut regex_list, curr, prevp);
                            continue;
                        } else {
                            reorder = options.optimisation_level as libc::c_int
                                > 1 as libc::c_int
                                && (NeedsType as libc::c_int as libc::c_uint
                                    == (*(*curr).pred_right).p_cost as libc::c_uint
                                    || NeedsInodeNumber as libc::c_int as libc::c_uint
                                        == (*(*curr).pred_right).p_cost as libc::c_uint)
                                && !(*(*curr).pred_right).need_stat
                                || options.optimisation_level as libc::c_int
                                    > 2 as libc::c_int;
                            if reorder {
                                if options.debug_options
                                    & DebugTreeOpt as libc::c_int as libc::c_ulong != 0
                                {
                                    fprintf(
                                        stderr,
                                        b"-O%d: categorising predicate \0" as *const u8
                                            as *const libc::c_char,
                                        options.optimisation_level as libc::c_int,
                                    );
                                    print_predicate(stderr, (*curr).pred_right);
                                    fprintf(
                                        stderr,
                                        b" by cost (%s)\n\0" as *const u8 as *const libc::c_char,
                                        cost_name((*(*curr).pred_right).p_cost),
                                    );
                                }
                                predlist_insert(
                                    &mut *cbo_list
                                        .as_mut_ptr()
                                        .offset((*(*curr).pred_right).p_cost as isize),
                                    curr,
                                    prevp,
                                );
                                continue;
                            }
                        }
                    }
                }
            }
            2 => {
                (*(*curr).pred_right)
                    .side_effects = opt_expr(&mut (*(*curr).pred_right).pred_right);
            }
            3 => {
                (*(*curr).pred_right).side_effects = opt_expr(&mut (*curr).pred_right);
            }
            _ => {
                if ::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"oops -- invalid expression type!\0" as *const u8
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
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"oops -- invalid expression type!\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
        }
        if (*(*curr).pred_right).side_effects as libc::c_int == 1 as libc::c_int {
            last_sidep = prevp;
            merge_lists(
                cbo_list.as_mut_ptr(),
                NumEvaluationCosts as libc::c_int,
                &mut name_list,
                &mut regex_list,
                last_sidep,
            );
            has_side_effects = 1 as libc::c_int != 0;
        }
        prevp = &mut (*curr).pred_left;
    }
    last_sidep = prevp;
    merge_lists(
        cbo_list.as_mut_ptr(),
        NumEvaluationCosts as libc::c_int,
        &mut name_list,
        &mut regex_list,
        last_sidep,
    );
    return has_side_effects;
}
unsafe extern "C" fn constrain_rate(mut rate: libc::c_float) -> libc::c_float {
    if rate > 1.0f32 {
        return 1.0f64 as libc::c_float
    } else if (rate as libc::c_double) < 0.0f64 {
        return 0.0f64 as libc::c_float
    } else {
        return rate
    };
}
unsafe extern "C" fn set_new_parent(
    mut curr: *mut predicate,
    mut high_prec: predicate_precedence,
    mut prevp: *mut *mut predicate,
) -> *mut predicate {
    let mut new_parent: *mut predicate = 0 as *mut predicate;
    new_parent = xzalloc(::std::mem::size_of::<predicate>() as libc::c_ulong)
        as *mut predicate;
    (*new_parent).p_type = BI_OP;
    (*new_parent).p_prec = high_prec;
    (*new_parent).p_cost = NeedsNothing;
    match high_prec as libc::c_uint {
        1 => {
            (*new_parent).pred_func = Some(pred_comma as PREDICATEFUNCTION);
            (*new_parent).p_name = b",\0" as *const u8 as *const libc::c_char;
            (*new_parent).est_success_rate = 1.0f64 as libc::c_float;
        }
        2 => {
            (*new_parent).pred_func = Some(pred_or as PREDICATEFUNCTION);
            (*new_parent).p_name = b"-o\0" as *const u8 as *const libc::c_char;
            (*new_parent).est_success_rate = constrain_rate((*curr).est_success_rate);
        }
        3 => {
            (*new_parent).pred_func = Some(pred_and as PREDICATEFUNCTION);
            (*new_parent).p_name = b"-a\0" as *const u8 as *const libc::c_char;
            (*new_parent).est_success_rate = constrain_rate((*curr).est_success_rate);
        }
        _ => {}
    }
    (*new_parent).pred_right = curr;
    *prevp = new_parent;
    return new_parent;
}
unsafe extern "C" fn merge_pred(
    mut beg_list: *mut predicate,
    mut end_list: *mut predicate,
    mut last_p: *mut *mut predicate,
) {
    (*end_list).pred_left = *last_p;
    *last_p = beg_list;
}
static mut costlookup: [pred_cost_lookup; 59] = unsafe {
    [
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_amin as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_and as PREDICATEFUNCTION),
                cost: NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_anewer as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_atime as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_closeparen as PREDICATEFUNCTION),
                cost: NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_cmin as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_cnewer as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_comma as PREDICATEFUNCTION),
                cost: NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_context as PREDICATEFUNCTION),
                cost: NeedsAccessInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_ctime as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_delete as PREDICATEFUNCTION),
                cost: NeedsSyncDiskHit,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_empty as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_exec as PREDICATEFUNCTION),
                cost: NeedsEventualExec,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_execdir as PREDICATEFUNCTION),
                cost: NeedsEventualExec,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_executable as PREDICATEFUNCTION),
                cost: NeedsAccessInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_false as PREDICATEFUNCTION),
                cost: NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_fprint as PREDICATEFUNCTION),
                cost: NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_fprint0 as PREDICATEFUNCTION),
                cost: NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_fprintf as PREDICATEFUNCTION),
                cost: NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_fstype as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_gid as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_group as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_ilname as PREDICATEFUNCTION),
                cost: NeedsLinkName,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_iname as PREDICATEFUNCTION),
                cost: NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_inum as PREDICATEFUNCTION),
                cost: NeedsInodeNumber,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_ipath as PREDICATEFUNCTION),
                cost: NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_links as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_lname as PREDICATEFUNCTION),
                cost: NeedsLinkName,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_ls as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_fls as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_mmin as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_mtime as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_name as PREDICATEFUNCTION),
                cost: NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_negate as PREDICATEFUNCTION),
                cost: NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_newer as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_newerXY as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_nogroup as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_nouser as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_ok as PREDICATEFUNCTION),
                cost: NeedsUserInteraction,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_okdir as PREDICATEFUNCTION),
                cost: NeedsUserInteraction,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_openparen as PREDICATEFUNCTION),
                cost: NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_or as PREDICATEFUNCTION),
                cost: NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_path as PREDICATEFUNCTION),
                cost: NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_perm as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_print as PREDICATEFUNCTION),
                cost: NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_print0 as PREDICATEFUNCTION),
                cost: NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_prune as PREDICATEFUNCTION),
                cost: NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: ::std::mem::transmute::<
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
                ),
                cost: NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_readable as PREDICATEFUNCTION),
                cost: NeedsAccessInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_regex as PREDICATEFUNCTION),
                cost: NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_samefile as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_size as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_true as PREDICATEFUNCTION),
                cost: NeedsNothing,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_type as PREDICATEFUNCTION),
                cost: NeedsType,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_uid as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_used as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_user as PREDICATEFUNCTION),
                cost: NeedsStatInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_writable as PREDICATEFUNCTION),
                cost: NeedsAccessInfo,
            };
            init
        },
        {
            let mut init = pred_cost_lookup {
                fn_0: Some(pred_xtype as PREDICATEFUNCTION),
                cost: NeedsType,
            };
            init
        },
    ]
};
static mut pred_table_sorted: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn check_sorted(
    mut base: *mut libc::c_void,
    mut members: size_t,
    mut membersize: size_t,
    mut cmpfn: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
) -> bool {
    let mut p: *const libc::c_char = base as *const libc::c_char;
    let mut i: size_t = 0;
    i = 1 as libc::c_uint as size_t;
    while i < members {
        let mut result: libc::c_int = cmpfn
            .unwrap()(
            p.offset(i.wrapping_mul(membersize) as isize) as *const libc::c_void,
            p
                .offset(
                    i
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(membersize) as isize,
                ) as *const libc::c_void,
        );
        if result < 0 as libc::c_int {
            return 0 as libc::c_int != 0;
        }
        result = cmpfn
            .unwrap()(
            p
                .offset(
                    i
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(membersize) as isize,
                ) as *const libc::c_void,
            p.offset(i.wrapping_mul(membersize) as isize) as *const libc::c_void,
        );
        if result <= 0 as libc::c_int {} else {
            __assert_fail(
                b"result <= 0\0" as *const u8 as *const libc::c_char,
                b"tree.c\0" as *const u8 as *const libc::c_char,
                1013 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 80],
                    &[libc::c_char; 80],
                >(
                    b"_Bool check_sorted(void *, size_t, size_t, int (*)(const void *, const void *))\0",
                ))
                    .as_ptr(),
            );
        }
        'c_10475: {
            if result <= 0 as libc::c_int {} else {
                __assert_fail(
                    b"result <= 0\0" as *const u8 as *const libc::c_char,
                    b"tree.c\0" as *const u8 as *const libc::c_char,
                    1013 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 80],
                        &[libc::c_char; 80],
                    >(
                        b"_Bool check_sorted(void *, size_t, size_t, int (*)(const void *, const void *))\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        i = i.wrapping_add(1);
        i;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn cost_table_comparison(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> libc::c_int {
    let mut pc1: *const pred_cost_lookup = p1 as *const pred_cost_lookup;
    let mut pc2: *const pred_cost_lookup = p2 as *const pred_cost_lookup;
    let mut u1: C2RustUnnamed_1 = C2RustUnnamed_1 { pfn: None };
    let mut u2: C2RustUnnamed_1 = C2RustUnnamed_1 { pfn: None };
    u1.pfn = (*pc1).fn_0;
    u2.pfn = (*pc2).fn_0;
    return memcmp(
        (u1.mem).as_mut_ptr() as *const libc::c_void,
        (u2.mem).as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<PRED_FUNC>() as libc::c_ulong,
    );
}
unsafe extern "C" fn get_pred_cost(mut p: *const predicate) -> EvaluationCost {
    let mut data_requirement_cost: EvaluationCost = NeedsNothing;
    let mut inherent_cost: EvaluationCost = NeedsUnknown;
    if (*p).need_stat {
        data_requirement_cost = NeedsStatInfo;
    } else if (*p).need_inum {
        data_requirement_cost = NeedsInodeNumber;
    } else if (*p).need_type {
        data_requirement_cost = NeedsType;
    } else {
        data_requirement_cost = NeedsNothing;
    }
    if (*p).pred_func == Some(pred_exec as PREDICATEFUNCTION)
        || (*p).pred_func == Some(pred_execdir as PREDICATEFUNCTION)
    {
        if (*p).args.exec_vec.multiple {
            inherent_cost = NeedsEventualExec;
        } else {
            inherent_cost = NeedsImmediateExec;
        }
    } else if (*p).pred_func == Some(pred_fprintf as PREDICATEFUNCTION) {
        inherent_cost = (*p).p_cost;
    } else {
        let mut key: pred_cost_lookup = pred_cost_lookup {
            fn_0: None,
            cost: NeedsNothing,
        };
        let mut entry: *mut libc::c_void = 0 as *mut libc::c_void;
        if pred_table_sorted == 0 {
            qsort(
                costlookup.as_mut_ptr() as *mut libc::c_void,
                (::std::mem::size_of::<[pred_cost_lookup; 59]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<pred_cost_lookup>() as libc::c_ulong,
                    ),
                ::std::mem::size_of::<pred_cost_lookup>() as libc::c_ulong,
                Some(
                    cost_table_comparison
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
            if !check_sorted(
                costlookup.as_mut_ptr() as *mut libc::c_void,
                (::std::mem::size_of::<[pred_cost_lookup; 59]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<pred_cost_lookup>() as libc::c_ulong,
                    ),
                ::std::mem::size_of::<pred_cost_lookup>() as libc::c_ulong,
                Some(
                    cost_table_comparison
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            ) {
                if ::std::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"failed to sort the costlookup array\0" as *const u8
                            as *const libc::c_char,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                } else {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"failed to sort the costlookup array\0" as *const u8
                            as *const libc::c_char,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            pred_table_sorted = 1 as libc::c_int;
        }
        key.fn_0 = (*p).pred_func;
        entry = bsearch(
            &mut key as *mut pred_cost_lookup as *const libc::c_void,
            costlookup.as_mut_ptr() as *const libc::c_void,
            (::std::mem::size_of::<[pred_cost_lookup; 59]>() as libc::c_ulong)
                .wrapping_div(
                    ::std::mem::size_of::<pred_cost_lookup>() as libc::c_ulong,
                ),
            ::std::mem::size_of::<pred_cost_lookup>() as libc::c_ulong,
            Some(
                cost_table_comparison
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        if !entry.is_null() {
            inherent_cost = (*(entry as *const pred_cost_lookup)).cost;
        } else {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"warning: there is no entry in the predicate evaluation cost table for predicate %s; please report this as a bug\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*p).p_name,
            );
            inherent_cost = NeedsUnknown;
        }
    }
    if inherent_cost as libc::c_uint > data_requirement_cost as libc::c_uint {
        return inherent_cost
    } else {
        return data_requirement_cost
    };
}
unsafe extern "C" fn estimate_costs(mut tree: *mut predicate) {
    if !tree.is_null() {
        estimate_costs((*tree).pred_right);
        estimate_costs((*tree).pred_left);
        (*tree).p_cost = get_pred_cost(tree);
    }
}
pub unsafe extern "C" fn get_eval_tree() -> *mut predicate {
    return eval_tree;
}
unsafe extern "C" fn getrate(mut p: *const predicate) -> libc::c_float {
    if !p.is_null() { return (*p).est_success_rate } else { return 1.0f32 };
}
pub unsafe extern "C" fn calculate_derived_rates(
    mut p: *mut predicate,
) -> libc::c_float {
    if !p.is_null() {} else {
        __assert_fail(
            b"NULL != p\0" as *const u8 as *const libc::c_char,
            b"tree.c\0" as *const u8 as *const libc::c_char,
            1156 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"float calculate_derived_rates(struct predicate *)\0"))
                .as_ptr(),
        );
    }
    'c_8475: {
        if !p.is_null() {} else {
            __assert_fail(
                b"NULL != p\0" as *const u8 as *const libc::c_char,
                b"tree.c\0" as *const u8 as *const libc::c_char,
                1156 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"float calculate_derived_rates(struct predicate *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*p).pred_right).is_null() {
        calculate_derived_rates((*p).pred_right);
    }
    if !((*p).pred_left).is_null() {
        calculate_derived_rates((*p).pred_left);
    }
    if (*p).p_type as libc::c_uint != CLOSE_PAREN as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"p->p_type != CLOSE_PAREN\0" as *const u8 as *const libc::c_char,
            b"tree.c\0" as *const u8 as *const libc::c_char,
            1163 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"float calculate_derived_rates(struct predicate *)\0"))
                .as_ptr(),
        );
    }
    'c_8407: {
        if (*p).p_type as libc::c_uint != CLOSE_PAREN as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"p->p_type != CLOSE_PAREN\0" as *const u8 as *const libc::c_char,
                b"tree.c\0" as *const u8 as *const libc::c_char,
                1163 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"float calculate_derived_rates(struct predicate *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*p).p_type as libc::c_uint != OPEN_PAREN as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"p->p_type != OPEN_PAREN\0" as *const u8 as *const libc::c_char,
            b"tree.c\0" as *const u8 as *const libc::c_char,
            1164 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"float calculate_derived_rates(struct predicate *)\0"))
                .as_ptr(),
        );
    }
    'c_8362: {
        if (*p).p_type as libc::c_uint != OPEN_PAREN as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"p->p_type != OPEN_PAREN\0" as *const u8 as *const libc::c_char,
                b"tree.c\0" as *const u8 as *const libc::c_char,
                1164 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"float calculate_derived_rates(struct predicate *)\0"))
                    .as_ptr(),
            );
        }
    };
    match (*p).p_type as libc::c_uint {
        0 => {
            if ((*p).pred_right).is_null() {} else {
                __assert_fail(
                    b"NULL == p->pred_right\0" as *const u8 as *const libc::c_char,
                    b"tree.c\0" as *const u8 as *const libc::c_char,
                    1169 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 50],
                        &[libc::c_char; 50],
                    >(b"float calculate_derived_rates(struct predicate *)\0"))
                        .as_ptr(),
                );
            }
            'c_8310: {
                if ((*p).pred_right).is_null() {} else {
                    __assert_fail(
                        b"NULL == p->pred_right\0" as *const u8 as *const libc::c_char,
                        b"tree.c\0" as *const u8 as *const libc::c_char,
                        1169 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 50],
                            &[libc::c_char; 50],
                        >(b"float calculate_derived_rates(struct predicate *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if ((*p).pred_left).is_null() {} else {
                __assert_fail(
                    b"NULL == p->pred_left\0" as *const u8 as *const libc::c_char,
                    b"tree.c\0" as *const u8 as *const libc::c_char,
                    1170 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 50],
                        &[libc::c_char; 50],
                    >(b"float calculate_derived_rates(struct predicate *)\0"))
                        .as_ptr(),
                );
            }
            'c_8262: {
                if ((*p).pred_left).is_null() {} else {
                    __assert_fail(
                        b"NULL == p->pred_left\0" as *const u8 as *const libc::c_char,
                        b"tree.c\0" as *const u8 as *const libc::c_char,
                        1170 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 50],
                            &[libc::c_char; 50],
                        >(b"float calculate_derived_rates(struct predicate *)\0"))
                            .as_ptr(),
                    );
                }
            };
            return (*p).est_success_rate;
        }
        1 => {
            if ((*p).pred_right).is_null() {} else {
                __assert_fail(
                    b"NULL == p->pred_right\0" as *const u8 as *const libc::c_char,
                    b"tree.c\0" as *const u8 as *const libc::c_char,
                    1174 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 50],
                        &[libc::c_char; 50],
                    >(b"float calculate_derived_rates(struct predicate *)\0"))
                        .as_ptr(),
                );
            }
            'c_8210: {
                if ((*p).pred_right).is_null() {} else {
                    __assert_fail(
                        b"NULL == p->pred_right\0" as *const u8 as *const libc::c_char,
                        b"tree.c\0" as *const u8 as *const libc::c_char,
                        1174 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 50],
                            &[libc::c_char; 50],
                        >(b"float calculate_derived_rates(struct predicate *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if ((*p).pred_left).is_null() {} else {
                __assert_fail(
                    b"NULL == p->pred_left\0" as *const u8 as *const libc::c_char,
                    b"tree.c\0" as *const u8 as *const libc::c_char,
                    1175 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 50],
                        &[libc::c_char; 50],
                    >(b"float calculate_derived_rates(struct predicate *)\0"))
                        .as_ptr(),
                );
            }
            'c_8162: {
                if ((*p).pred_left).is_null() {} else {
                    __assert_fail(
                        b"NULL == p->pred_left\0" as *const u8 as *const libc::c_char,
                        b"tree.c\0" as *const u8 as *const libc::c_char,
                        1175 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 50],
                            &[libc::c_char; 50],
                        >(b"float calculate_derived_rates(struct predicate *)\0"))
                            .as_ptr(),
                    );
                }
            };
            return (*p).est_success_rate;
        }
        2 => {
            if (*p).pred_func == Some(pred_negate as PREDICATEFUNCTION) {} else {
                __assert_fail(
                    b"pred_is (p, pred_negate)\0" as *const u8 as *const libc::c_char,
                    b"tree.c\0" as *const u8 as *const libc::c_char,
                    1180 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 50],
                        &[libc::c_char; 50],
                    >(b"float calculate_derived_rates(struct predicate *)\0"))
                        .as_ptr(),
                );
            }
            'c_8106: {
                if (*p).pred_func == Some(pred_negate as PREDICATEFUNCTION) {} else {
                    __assert_fail(
                        b"pred_is (p, pred_negate)\0" as *const u8
                            as *const libc::c_char,
                        b"tree.c\0" as *const u8 as *const libc::c_char,
                        1180 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 50],
                            &[libc::c_char; 50],
                        >(b"float calculate_derived_rates(struct predicate *)\0"))
                            .as_ptr(),
                    );
                }
            };
            if ((*p).pred_left).is_null() {} else {
                __assert_fail(
                    b"NULL == p->pred_left\0" as *const u8 as *const libc::c_char,
                    b"tree.c\0" as *const u8 as *const libc::c_char,
                    1181 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 50],
                        &[libc::c_char; 50],
                    >(b"float calculate_derived_rates(struct predicate *)\0"))
                        .as_ptr(),
                );
            }
            'c_8057: {
                if ((*p).pred_left).is_null() {} else {
                    __assert_fail(
                        b"NULL == p->pred_left\0" as *const u8 as *const libc::c_char,
                        b"tree.c\0" as *const u8 as *const libc::c_char,
                        1181 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 50],
                            &[libc::c_char; 50],
                        >(b"float calculate_derived_rates(struct predicate *)\0"))
                            .as_ptr(),
                    );
                }
            };
            (*p)
                .est_success_rate = (1.0f64
                - (*(*p).pred_right).est_success_rate as libc::c_double)
                as libc::c_float;
            return (*p).est_success_rate;
        }
        3 => {
            let mut rate: libc::c_float = 0.;
            if (*p).pred_func == Some(pred_and as PREDICATEFUNCTION) {
                rate = getrate((*p).pred_right) * getrate((*p).pred_left);
            } else if (*p).pred_func == Some(pred_comma as PREDICATEFUNCTION) {
                rate = 1.0f32;
            } else if (*p).pred_func == Some(pred_or as PREDICATEFUNCTION) {
                rate = getrate((*p).pred_right) + getrate((*p).pred_left);
            } else {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"tree.c\0" as *const u8 as *const libc::c_char,
                    1204 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 50],
                        &[libc::c_char; 50],
                    >(b"float calculate_derived_rates(struct predicate *)\0"))
                        .as_ptr(),
                );
                'c_7917: {
                    __assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"tree.c\0" as *const u8 as *const libc::c_char,
                        1204 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 50],
                            &[libc::c_char; 50],
                        >(b"float calculate_derived_rates(struct predicate *)\0"))
                            .as_ptr(),
                    );
                };
                abort();
            }
            (*p).est_success_rate = constrain_rate(rate);
            return (*p).est_success_rate;
        }
        4 | 5 => {
            (*p).est_success_rate = 1.0f64 as libc::c_float;
            return (*p).est_success_rate;
        }
        _ => {}
    }
    __assert_fail(
        b"0\0" as *const u8 as *const libc::c_char,
        b"tree.c\0" as *const u8 as *const libc::c_char,
        1216 as libc::c_int as libc::c_uint,
        (*::std::mem::transmute::<
            &[u8; 50],
            &[libc::c_char; 50],
        >(b"float calculate_derived_rates(struct predicate *)\0"))
            .as_ptr(),
    );
    'c_7794: {
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"tree.c\0" as *const u8 as *const libc::c_char,
            1216 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"float calculate_derived_rates(struct predicate *)\0"))
                .as_ptr(),
        );
    };
    abort();
}
unsafe extern "C" fn check_normalization(mut p: *mut predicate, mut at_root: bool) {
    if at_root {
        if BI_OP as libc::c_int as libc::c_uint == (*p).p_type as libc::c_uint {} else {
            __assert_fail(
                b"BI_OP == p->p_type\0" as *const u8 as *const libc::c_char,
                b"tree.c\0" as *const u8 as *const libc::c_char,
                1230 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"void check_normalization(struct predicate *, _Bool)\0"))
                    .as_ptr(),
            );
        }
        'c_6905: {
            if BI_OP as libc::c_int as libc::c_uint == (*p).p_type as libc::c_uint
            {} else {
                __assert_fail(
                    b"BI_OP == p->p_type\0" as *const u8 as *const libc::c_char,
                    b"tree.c\0" as *const u8 as *const libc::c_char,
                    1230 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 52],
                        &[libc::c_char; 52],
                    >(b"void check_normalization(struct predicate *, _Bool)\0"))
                        .as_ptr(),
                );
            }
        };
    }
    if !((*p).pred_left).is_null() {
        if BI_OP as libc::c_int as libc::c_uint
            == (*(*p).pred_left).p_type as libc::c_uint
        {} else {
            __assert_fail(
                b"BI_OP == p->pred_left->p_type\0" as *const u8 as *const libc::c_char,
                b"tree.c\0" as *const u8 as *const libc::c_char,
                1235 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"void check_normalization(struct predicate *, _Bool)\0"))
                    .as_ptr(),
            );
        }
        'c_6847: {
            if BI_OP as libc::c_int as libc::c_uint
                == (*(*p).pred_left).p_type as libc::c_uint
            {} else {
                __assert_fail(
                    b"BI_OP == p->pred_left->p_type\0" as *const u8
                        as *const libc::c_char,
                    b"tree.c\0" as *const u8 as *const libc::c_char,
                    1235 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 52],
                        &[libc::c_char; 52],
                    >(b"void check_normalization(struct predicate *, _Bool)\0"))
                        .as_ptr(),
                );
            }
        };
        check_normalization((*p).pred_left, 0 as libc::c_int != 0);
    }
    if !((*p).pred_right).is_null() {
        check_normalization((*p).pred_right, 0 as libc::c_int != 0);
    }
}
pub unsafe extern "C" fn build_expression_tree(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut end_of_leading_options: libc::c_int,
) -> *mut predicate {
    let mut parse_entry: *const parser_table = 0 as *const parser_table;
    let mut predicate_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cur_pred: *mut predicate = 0 as *mut predicate;
    let mut entry_close: *const parser_table = 0 as *const parser_table;
    let mut entry_print: *const parser_table = 0 as *const parser_table;
    let mut entry_open: *const parser_table = 0 as *const parser_table;
    let mut i: libc::c_int = 0;
    let mut oldi: libc::c_int = 0;
    predicates = 0 as *mut predicate;
    start_points = argv.offset(end_of_leading_options as isize);
    i = end_of_leading_options;
    while i < argc
        && !looks_like_expression(*argv.offset(i as isize), 1 as libc::c_int != 0)
    {
        num_start_points = num_start_points.wrapping_add(1);
        num_start_points;
        i += 1;
        i;
    }
    entry_open = find_parser(b"(\0" as *const u8 as *const libc::c_char);
    entry_close = find_parser(b")\0" as *const u8 as *const libc::c_char);
    entry_print = find_parser(b"print\0" as *const u8 as *const libc::c_char);
    if !entry_open.is_null() {} else {
        __assert_fail(
            b"entry_open != NULL\0" as *const u8 as *const libc::c_char,
            b"tree.c\0" as *const u8 as *const libc::c_char,
            1270 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"struct predicate *build_expression_tree(int, char **, int)\0"))
                .as_ptr(),
        );
    }
    'c_13520: {
        if !entry_open.is_null() {} else {
            __assert_fail(
                b"entry_open != NULL\0" as *const u8 as *const libc::c_char,
                b"tree.c\0" as *const u8 as *const libc::c_char,
                1270 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"struct predicate *build_expression_tree(int, char **, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if !entry_close.is_null() {} else {
        __assert_fail(
            b"entry_close != NULL\0" as *const u8 as *const libc::c_char,
            b"tree.c\0" as *const u8 as *const libc::c_char,
            1271 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"struct predicate *build_expression_tree(int, char **, int)\0"))
                .as_ptr(),
        );
    }
    'c_13478: {
        if !entry_close.is_null() {} else {
            __assert_fail(
                b"entry_close != NULL\0" as *const u8 as *const libc::c_char,
                b"tree.c\0" as *const u8 as *const libc::c_char,
                1271 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"struct predicate *build_expression_tree(int, char **, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if !entry_print.is_null() {} else {
        __assert_fail(
            b"entry_print != NULL\0" as *const u8 as *const libc::c_char,
            b"tree.c\0" as *const u8 as *const libc::c_char,
            1272 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"struct predicate *build_expression_tree(int, char **, int)\0"))
                .as_ptr(),
        );
    }
    'c_13434: {
        if !entry_print.is_null() {} else {
            __assert_fail(
                b"entry_print != NULL\0" as *const u8 as *const libc::c_char,
                b"tree.c\0" as *const u8 as *const libc::c_char,
                1272 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"struct predicate *build_expression_tree(int, char **, int)\0"))
                    .as_ptr(),
            );
        }
    };
    parse_openparen(entry_open, argv, &mut argc);
    (*last_pred).p_name = b"(\0" as *const u8 as *const libc::c_char;
    (*predicates).artificial = 1 as libc::c_int != 0;
    parse_begin_user_args(argv, argc, last_pred, predicates);
    pred_sanity_check(last_pred);
    while i < argc {
        state.already_issued_stat_error_msg = 0 as libc::c_int != 0;
        if !looks_like_expression(*argv.offset(i as isize), 0 as libc::c_int != 0) {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"paths must precede expression: `%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                *argv.offset(i as isize),
            );
            if access(*argv.offset(i as isize), 0 as libc::c_int) == 0 as libc::c_int {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"possible unquoted pattern after predicate `%s'?\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*last_pred).p_name,
                );
            }
            exit(1 as libc::c_int);
        }
        predicate_name = *argv.offset(i as isize);
        parse_entry = find_parser(predicate_name);
        if parse_entry.is_null() {
            if ::std::mem::size_of::<C2RustUnnamed_24>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"unknown predicate `%s'\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    predicate_name,
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
                        b"unknown predicate `%s'\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    predicate_name,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        if (*parse_entry).type_0 as libc::c_uint
            != ARG_SPECIAL_PARSE as libc::c_int as libc::c_uint
        {
            i += 1;
            i;
        }
        oldi = i;
        if !(Some(((*parse_entry).parser_func).unwrap()))
            .unwrap()(parse_entry, argv, &mut i)
        {
            if !(*argv.offset(i as isize)).is_null() {
                if ARG_SPECIAL_PARSE as libc::c_int as libc::c_uint
                    == (*parse_entry).type_0 as libc::c_uint && i == oldi
                {
                    if ::std::mem::size_of::<C2RustUnnamed_23>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"invalid predicate `%s'\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            predicate_name,
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
                                b"invalid predicate `%s'\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            predicate_name,
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                } else {
                    if ::std::mem::size_of::<C2RustUnnamed_22>() as libc::c_ulong != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"invalid argument `%s' to `%s'\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            *argv.offset(i as isize),
                            predicate_name,
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
                                b"invalid argument `%s' to `%s'\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            *argv.offset(i as isize),
                            predicate_name,
                        );
                        if 0 as libc::c_int != 0 {} else {
                            unreachable!();
                        };
                    };
                }
            } else {
                if ::std::mem::size_of::<C2RustUnnamed_21>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"missing argument to `%s'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        predicate_name,
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
                            b"missing argument to `%s'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        predicate_name,
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
        } else {
            (*last_pred).p_name = predicate_name;
            if i != oldi {
                (*last_pred).arg_text = *argv.offset(oldi as isize);
            } else {
                (*last_pred).arg_text = 0 as *const libc::c_char;
            }
        }
        pred_sanity_check(last_pred);
        pred_sanity_check(predicates);
    }
    parse_end_user_args(argv, argc, last_pred, predicates);
    if ((*predicates).pred_next).is_null() {
        cur_pred = predicates;
        last_pred = (*predicates).pred_next;
        predicates = last_pred;
        rpl_free(cur_pred as *mut libc::c_void);
        parse_print(entry_print, argv, &mut argc);
        (*last_pred).p_name = b"-print\0" as *const u8 as *const libc::c_char;
        pred_sanity_check(last_pred);
        pred_sanity_check(predicates);
    } else if !default_prints((*predicates).pred_next) {
        cur_pred = predicates;
        predicates = (*predicates).pred_next;
        pred_sanity_check(predicates);
        rpl_free(cur_pred as *mut libc::c_void);
    } else {
        parse_closeparen(entry_close, argv, &mut argc);
        (*last_pred).p_name = b")\0" as *const u8 as *const libc::c_char;
        (*last_pred).artificial = 1 as libc::c_int != 0;
        pred_sanity_check(last_pred);
        parse_print(entry_print, argv, &mut argc);
        (*last_pred).p_name = b"-print\0" as *const u8 as *const libc::c_char;
        (*last_pred).artificial = 1 as libc::c_int != 0;
        pred_sanity_check(last_pred);
        pred_sanity_check(predicates);
    }
    if options.debug_options
        & (DebugExpressionTree as libc::c_int | DebugTreeOpt as libc::c_int)
            as libc::c_ulong != 0
    {
        fprintf(stderr, b"Predicate List:\n\0" as *const u8 as *const libc::c_char);
        print_list(stderr, predicates);
    }
    check_option_combinations(predicates);
    pred_sanity_check(predicates);
    cur_pred = predicates;
    eval_tree = get_expr(
        &mut cur_pred,
        NO_PREC as libc::c_int as libc::c_short,
        0 as *const predicate,
    );
    calculate_derived_rates(eval_tree);
    if !cur_pred.is_null() {
        if (*cur_pred).pred_func == Some(pred_closeparen as PREDICATEFUNCTION) {
            if ::std::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"you have too many ')'\0" as *const u8 as *const libc::c_char,
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
                        b"you have too many ')'\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        } else if !((*cur_pred).p_name).is_null() {
            if ::std::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"unexpected extra predicate '%s'\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*cur_pred).p_name,
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
                        b"unexpected extra predicate '%s'\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*cur_pred).p_name,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        } else {
            if ::std::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"unexpected extra predicate\0" as *const u8
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
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"unexpected extra predicate\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
    }
    if options.debug_options
        & (DebugExpressionTree as libc::c_int | DebugTreeOpt as libc::c_int)
            as libc::c_ulong != 0
    {
        fprintf(stderr, b"Eval Tree:\n\0" as *const u8 as *const libc::c_char);
        print_tree(stderr, eval_tree, 0 as libc::c_int);
    }
    estimate_costs(eval_tree);
    opt_expr(&mut eval_tree);
    check_normalization(eval_tree, 1 as libc::c_int != 0);
    do_arm_swaps(eval_tree);
    check_normalization(eval_tree, 1 as libc::c_int != 0);
    if options.debug_options
        & (DebugExpressionTree as libc::c_int | DebugTreeOpt as libc::c_int)
            as libc::c_ulong != 0
    {
        fprintf(stderr, b"Optimized Eval Tree:\n\0" as *const u8 as *const libc::c_char);
        print_tree(stderr, eval_tree, 0 as libc::c_int);
        fprintf(
            stderr,
            b"Optimized command line:\n\0" as *const u8 as *const libc::c_char,
        );
        print_optlist(stderr, eval_tree);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
    return eval_tree;
}
unsafe extern "C" fn init_pred_perf(mut pred: *mut predicate) {
    let mut p: *mut predicate_performance_info = &mut (*pred).perf;
    (*p).successes = 0 as libc::c_int as libc::c_ulong;
    (*p).visits = (*p).successes;
}
pub unsafe extern "C" fn get_new_pred_noarg(
    mut entry: *const parser_table,
) -> *mut predicate {
    let mut p: *mut predicate = get_new_pred(entry);
    if !p.is_null() {
        (*p).arg_text = 0 as *const libc::c_char;
    }
    return p;
}
pub unsafe extern "C" fn get_new_pred(mut entry: *const parser_table) -> *mut predicate {
    let mut new_pred: *mut predicate = 0 as *mut predicate;
    if (*entry).type_0 as libc::c_uint != ARG_OPTION as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"entry->type != ARG_OPTION\0" as *const u8 as *const libc::c_char,
            b"tree.c\0" as *const u8 as *const libc::c_char,
            1485 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"struct predicate *get_new_pred(const struct parser_table *)\0"))
                .as_ptr(),
        );
    }
    'c_13833: {
        if (*entry).type_0 as libc::c_uint != ARG_OPTION as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"entry->type != ARG_OPTION\0" as *const u8 as *const libc::c_char,
                b"tree.c\0" as *const u8 as *const libc::c_char,
                1485 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"struct predicate *get_new_pred(const struct parser_table *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*entry).type_0 as libc::c_uint
        != ARG_POSITIONAL_OPTION as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"entry->type != ARG_POSITIONAL_OPTION\0" as *const u8
                as *const libc::c_char,
            b"tree.c\0" as *const u8 as *const libc::c_char,
            1486 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"struct predicate *get_new_pred(const struct parser_table *)\0"))
                .as_ptr(),
        );
    }
    'c_13787: {
        if (*entry).type_0 as libc::c_uint
            != ARG_POSITIONAL_OPTION as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"entry->type != ARG_POSITIONAL_OPTION\0" as *const u8
                    as *const libc::c_char,
                b"tree.c\0" as *const u8 as *const libc::c_char,
                1486 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"struct predicate *get_new_pred(const struct parser_table *)\0"))
                    .as_ptr(),
            );
        }
    };
    new_pred = xzalloc(::std::mem::size_of::<predicate>() as libc::c_ulong)
        as *mut predicate;
    if predicates.is_null() {
        predicates = new_pred;
        last_pred = predicates;
    } else {
        (*last_pred).pred_next = new_pred;
        last_pred = new_pred;
    }
    (*last_pred).parser_entry = entry;
    (*last_pred).p_type = NO_TYPE;
    (*last_pred).p_prec = NO_PREC;
    (*last_pred).need_stat = 1 as libc::c_int != 0;
    (*last_pred).need_type = 1 as libc::c_int != 0;
    (*last_pred).p_cost = NeedsUnknown;
    (*last_pred)
        .arg_text = b"ThisShouldBeSetToSomethingElse\0" as *const u8
        as *const libc::c_char;
    (*last_pred).literal_control_chars = options.literal_control_chars;
    (*last_pred).est_success_rate = 1.0f64 as libc::c_float;
    init_pred_perf(last_pred);
    return last_pred;
}
pub unsafe extern "C" fn get_new_pred_chk_op(
    mut entry: *const parser_table,
    mut arg: *const libc::c_char,
) -> *mut predicate {
    let mut new_pred: *mut predicate = 0 as *mut predicate;
    static mut entry_and: *const parser_table = 0 as *const parser_table;
    if entry_and.is_null() {
        entry_and = find_parser(b"and\0" as *const u8 as *const libc::c_char);
    }
    if !entry_and.is_null() {} else {
        __assert_fail(
            b"entry_and != NULL\0" as *const u8 as *const libc::c_char,
            b"tree.c\0" as *const u8 as *const libc::c_char,
            1528 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 81],
                &[libc::c_char; 81],
            >(
                b"struct predicate *get_new_pred_chk_op(const struct parser_table *, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_14115: {
        if !entry_and.is_null() {} else {
            __assert_fail(
                b"entry_and != NULL\0" as *const u8 as *const libc::c_char,
                b"tree.c\0" as *const u8 as *const libc::c_char,
                1528 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 81],
                    &[libc::c_char; 81],
                >(
                    b"struct predicate *get_new_pred_chk_op(const struct parser_table *, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !last_pred.is_null() {
        match (*last_pred).p_type as libc::c_uint {
            0 => {
                if ::std::mem::size_of::<C2RustUnnamed_25>() as libc::c_ulong != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"oops -- invalid default insertion of and!\0" as *const u8
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
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"oops -- invalid default insertion of and!\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    if 0 as libc::c_int != 0 {} else {
                        unreachable!();
                    };
                };
            }
            1 | 5 => {
                new_pred = get_new_pred_noarg(entry_and);
                (*new_pred).pred_func = Some(pred_and as PREDICATEFUNCTION);
                (*new_pred).p_name = b"-a\0" as *const u8 as *const libc::c_char;
                (*new_pred).p_type = BI_OP;
                (*new_pred).p_prec = AND_PREC;
                (*new_pred).need_stat = 0 as libc::c_int != 0;
                (*new_pred).need_type = 0 as libc::c_int != 0;
                (*new_pred).need_inum = 0 as libc::c_int != 0;
                (*new_pred).arg_text = 0 as *const libc::c_char;
                (*new_pred).args.str_0 = 0 as *const libc::c_char;
                (*new_pred).side_effects = 0 as libc::c_int != 0;
                (*new_pred).no_default_print = 0 as libc::c_int != 0;
            }
            _ => {}
        }
    }
    new_pred = get_new_pred(entry);
    (*new_pred).arg_text = arg;
    (*new_pred).parser_entry = entry;
    return new_pred;
}
pub static mut cost_table: [cost_assoc; 11] = [
    {
        let mut init = cost_assoc {
            cost: NeedsNothing,
            name: b"Nothing\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = cost_assoc {
            cost: NeedsInodeNumber,
            name: b"InodeNumber\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = cost_assoc {
            cost: NeedsType,
            name: b"Type\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = cost_assoc {
            cost: NeedsStatInfo,
            name: b"StatInfo\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = cost_assoc {
            cost: NeedsLinkName,
            name: b"LinkName\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = cost_assoc {
            cost: NeedsAccessInfo,
            name: b"AccessInfo\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = cost_assoc {
            cost: NeedsSyncDiskHit,
            name: b"SyncDiskHit\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = cost_assoc {
            cost: NeedsEventualExec,
            name: b"EventualExec\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = cost_assoc {
            cost: NeedsImmediateExec,
            name: b"ImmediateExec\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = cost_assoc {
            cost: NeedsUserInteraction,
            name: b"UserInteraction\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = cost_assoc {
            cost: NeedsUnknown,
            name: b"Unknown\0" as *const u8 as *const libc::c_char,
        };
        init
    },
];
static mut prec_table: [prec_assoc; 7] = [
    {
        let mut init = prec_assoc {
            prec: NO_PREC as libc::c_int as libc::c_short,
            prec_name: b"no\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = prec_assoc {
            prec: COMMA_PREC as libc::c_int as libc::c_short,
            prec_name: b"comma\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = prec_assoc {
            prec: OR_PREC as libc::c_int as libc::c_short,
            prec_name: b"or\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = prec_assoc {
            prec: AND_PREC as libc::c_int as libc::c_short,
            prec_name: b"and\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = prec_assoc {
            prec: NEGATE_PREC as libc::c_int as libc::c_short,
            prec_name: b"negate\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = prec_assoc {
            prec: MAX_PREC as libc::c_int as libc::c_short,
            prec_name: b"max\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = prec_assoc {
            prec: -(1 as libc::c_int) as libc::c_short,
            prec_name: b"unknown \0" as *const u8 as *const libc::c_char,
        };
        init
    },
];
static mut type_table: [op_assoc; 7] = [
    {
        let mut init = op_assoc {
            type_0: NO_TYPE as libc::c_int as libc::c_short,
            type_name: b"no\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = op_assoc {
            type_0: PRIMARY_TYPE as libc::c_int as libc::c_short,
            type_name: b"primary\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = op_assoc {
            type_0: UNI_OP as libc::c_int as libc::c_short,
            type_name: b"uni_op\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = op_assoc {
            type_0: BI_OP as libc::c_int as libc::c_short,
            type_name: b"bi_op\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = op_assoc {
            type_0: OPEN_PAREN as libc::c_int as libc::c_short,
            type_name: b"open_paren  \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = op_assoc {
            type_0: CLOSE_PAREN as libc::c_int as libc::c_short,
            type_name: b"close_paren \0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = op_assoc {
            type_0: -(1 as libc::c_int) as libc::c_short,
            type_name: b"unknown\0" as *const u8 as *const libc::c_char,
        };
        init
    },
];
unsafe extern "C" fn cost_name(mut cost: EvaluationCost) -> *const libc::c_char {
    let mut i: libc::c_uint = 0;
    let mut n: libc::c_uint = (::std::mem::size_of::<[cost_assoc; 11]>()
        as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cost_assoc>() as libc::c_ulong)
        as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while i < n {
        if cost_table[i as usize].cost as libc::c_uint == cost as libc::c_uint {
            return cost_table[i as usize].name;
        }
        i = i.wrapping_add(1);
        i;
    }
    return b"unknown\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn type_name(mut type_0: libc::c_short) -> *const libc::c_char {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while type_table[i as usize].type_0 as libc::c_int
        != -(1 as libc::c_int) as libc::c_short as libc::c_int
    {
        if type_table[i as usize].type_0 as libc::c_int == type_0 as libc::c_int {
            break;
        }
        i += 1;
        i;
    }
    return type_table[i as usize].type_name;
}
unsafe extern "C" fn prec_name(mut prec: libc::c_short) -> *const libc::c_char {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while prec_table[i as usize].prec as libc::c_int
        != -(1 as libc::c_int) as libc::c_short as libc::c_int
    {
        if prec_table[i as usize].prec as libc::c_int == prec as libc::c_int {
            break;
        }
        i += 1;
        i;
    }
    return prec_table[i as usize].prec_name;
}
pub unsafe extern "C" fn print_tree(
    mut fp: *mut FILE,
    mut node: *mut predicate,
    mut indent: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if node.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while i < indent {
        fprintf(fp, b"    \0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    fprintf(fp, b"pred=[\0" as *const u8 as *const libc::c_char);
    print_predicate(fp, node);
    fprintf(
        fp,
        b"] type=%s prec=%s\0" as *const u8 as *const libc::c_char,
        type_name((*node).p_type as libc::c_short),
        prec_name((*node).p_prec as libc::c_short),
    );
    fprintf(
        fp,
        b" cost=%s est_success_rate=%#.4g %sside effects \0" as *const u8
            as *const libc::c_char,
        cost_name((*node).p_cost),
        (*node).est_success_rate as libc::c_double,
        if (*node).side_effects as libc::c_int != 0 {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"no \0" as *const u8 as *const libc::c_char
        },
    );
    if (*node).need_stat as libc::c_int != 0 || (*node).need_type as libc::c_int != 0
        || (*node).need_inum as libc::c_int != 0
    {
        let mut comma: libc::c_int = 0 as libc::c_int;
        fprintf(fp, b"Needs \0" as *const u8 as *const libc::c_char);
        if (*node).need_stat {
            fprintf(fp, b"stat\0" as *const u8 as *const libc::c_char);
            comma = 1 as libc::c_int;
        }
        if (*node).need_inum {
            fprintf(
                fp,
                b"%sinode\0" as *const u8 as *const libc::c_char,
                if comma != 0 {
                    b",\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
            comma = 1 as libc::c_int;
        }
        if (*node).need_type {
            fprintf(
                fp,
                b"%stype\0" as *const u8 as *const libc::c_char,
                if comma != 0 {
                    b",\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
        }
    }
    fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < indent {
        fprintf(fp, b"    \0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    if ((*node).pred_left).is_null() && ((*node).pred_right).is_null() {
        fprintf(fp, b"no children.\n\0" as *const u8 as *const libc::c_char);
    } else {
        if !((*node).pred_left).is_null() {
            fprintf(fp, b"left:\n\0" as *const u8 as *const libc::c_char);
            print_tree(fp, (*node).pred_left, indent + 1 as libc::c_int);
        } else {
            fprintf(fp, b"no left.\n\0" as *const u8 as *const libc::c_char);
        }
        i = 0 as libc::c_int;
        while i < indent {
            fprintf(fp, b"    \0" as *const u8 as *const libc::c_char);
            i += 1;
            i;
        }
        if !((*node).pred_right).is_null() {
            fprintf(fp, b"right:\n\0" as *const u8 as *const libc::c_char);
            print_tree(fp, (*node).pred_right, indent + 1 as libc::c_int);
        } else {
            fprintf(fp, b"no right.\n\0" as *const u8 as *const libc::c_char);
        }
    };
}
