use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    pub type quoting_options;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn set_cloexec_flag(desc: libc::c_int, value: bool) -> libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn base_name(file: *const libc::c_char) -> *mut libc::c_char;
    fn mdir_name(file: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn open_safer(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    fn openat_safer(
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: ...
    ) -> libc::c_int;
    fn restore_cwd(cwd: *const saved_cwd) -> libc::c_int;
    fn free_cwd(cwd: *mut saved_cwd);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn bc_do_insert(
        ctl: *mut buildcmd_control,
        state_0: *mut buildcmd_state,
        arg: *mut libc::c_char,
        arglen: size_t,
        prefix: *const libc::c_char,
        pfxlen: size_t,
        linebuf: *const libc::c_char,
        lblen: size_t,
        initial_args: libc::c_int,
    );
    fn bc_do_exec(ctl: *mut buildcmd_control, state_0: *mut buildcmd_state);
    fn bc_push_arg(
        ctl: *mut buildcmd_control,
        state_0: *mut buildcmd_state,
        arg: *const libc::c_char,
        len: size_t,
        prefix: *const libc::c_char,
        pfxlen: size_t,
        initial_args: libc::c_int,
    );
    fn bc_args_exceed_testing_limit(argv: *mut *mut libc::c_char) -> bool;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn fork() -> __pid_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn quotearg_n_style(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn safely_quote_err_filename(
        n: libc::c_int,
        arg: *const libc::c_char,
    ) -> *const libc::c_char;
    fn is_exec_in_local_dir(pred_func: PRED_FUNC) -> bool;
    static mut state: state;
    static mut initial_wd: *mut saved_cwd;
    static mut options: options;
    fn complain_about_leaky_fds();
    fn fd_leak_check_is_enabled() -> bool;
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
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
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
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
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
pub struct saved_cwd {
    pub desc: libc::c_int,
    pub name: *mut libc::c_char,
}
pub type uintmax_t = __uintmax_t;
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
pub struct C2RustUnnamed_0 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _gl_dummy: libc::c_int,
}
unsafe extern "C" fn initialize_wd_for_exec(
    mut execp: *mut exec_val,
    mut cwd_fd: libc::c_int,
    mut dir: *const libc::c_char,
) -> bool {
    (*execp)
        .wd_for_exec = xmalloc(::std::mem::size_of::<saved_cwd>() as libc::c_ulong)
        as *mut saved_cwd;
    (*(*execp).wd_for_exec).name = 0 as *mut libc::c_char;
    (*(*execp).wd_for_exec).desc = openat_safer(cwd_fd, dir, 0 as libc::c_int);
    if (*(*execp).wd_for_exec).desc < 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    set_cloexec_flag((*(*execp).wd_for_exec).desc, 1 as libc::c_int != 0);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn record_exec_dir(mut execp: *mut exec_val) -> bool {
    if (*execp).state.todo == 0 {
        if (*execp).state.todo == 0 {} else {
            __assert_fail(
                b"!execp->state.todo\0" as *const u8 as *const libc::c_char,
                b"exec.c\0" as *const u8 as *const libc::c_char,
                71 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"_Bool record_exec_dir(struct exec_val *)\0"))
                    .as_ptr(),
            );
        }
        'c_7408: {
            if (*execp).state.todo == 0 {} else {
                __assert_fail(
                    b"!execp->state.todo\0" as *const u8 as *const libc::c_char,
                    b"exec.c\0" as *const u8 as *const libc::c_char,
                    71 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 41],
                        &[libc::c_char; 41],
                    >(b"_Bool record_exec_dir(struct exec_val *)\0"))
                        .as_ptr(),
                );
            }
        };
        if !(strchr(state.rel_pathname, '/' as i32)).is_null() {
            let mut dir: *mut libc::c_char = mdir_name(state.rel_pathname);
            let mut result: bool = initialize_wd_for_exec(execp, state.cwd_dir_fd, dir);
            rpl_free(dir as *mut libc::c_void);
            return result;
        } else {
            return initialize_wd_for_exec(
                execp,
                state.cwd_dir_fd,
                b".\0" as *const u8 as *const libc::c_char,
            )
        }
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn impl_pred_exec(
    mut pathname: *const libc::c_char,
    mut stat_buf: *mut stat,
    mut pred_ptr: *mut predicate,
) -> bool {
    let mut execp: *mut exec_val = &mut (*pred_ptr).args.exec_vec;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut target: *const libc::c_char = 0 as *const libc::c_char;
    let mut result: bool = false;
    let local: bool = is_exec_in_local_dir((*pred_ptr).pred_func);
    let mut prefix: *const libc::c_char = 0 as *const libc::c_char;
    let mut pfxlen: size_t = 0;
    if local {
        if !record_exec_dir(execp) {
            if ::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Failed to save working directory in order to run a command on %s\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    safely_quote_err_filename(0 as libc::c_int, pathname),
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
                        b"Failed to save working directory in order to run a command on %s\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    safely_quote_err_filename(0 as libc::c_int, pathname),
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        buf = base_name(state.rel_pathname);
        target = buf;
        if '/' as i32 == *target.offset(0 as libc::c_int as isize) as libc::c_int {
            prefix = 0 as *const libc::c_char;
            pfxlen = 0 as libc::c_int as size_t;
        } else {
            prefix = b"./\0" as *const u8 as *const libc::c_char;
            pfxlen = 2 as libc::c_uint as size_t;
        }
    } else {
        if (*execp).wd_for_exec == initial_wd {} else {
            __assert_fail(
                b"execp->wd_for_exec == initial_wd\0" as *const u8
                    as *const libc::c_char,
                b"exec.c\0" as *const u8 as *const libc::c_char,
                148 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 70],
                    &[libc::c_char; 70],
                >(
                    b"_Bool impl_pred_exec(const char *, struct stat *, struct predicate *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_7060: {
            if (*execp).wd_for_exec == initial_wd {} else {
                __assert_fail(
                    b"execp->wd_for_exec == initial_wd\0" as *const u8
                        as *const libc::c_char,
                    b"exec.c\0" as *const u8 as *const libc::c_char,
                    148 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 70],
                        &[libc::c_char; 70],
                    >(
                        b"_Bool impl_pred_exec(const char *, struct stat *, struct predicate *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        target = pathname;
        prefix = 0 as *const libc::c_char;
        pfxlen = 0 as libc::c_uint as size_t;
    }
    if (*execp).multiple {
        bc_push_arg(
            &mut (*execp).ctl,
            &mut (*execp).state,
            target,
            (strlen(target)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            prefix,
            pfxlen,
            0 as libc::c_int,
        );
        if (*execp).state.todo != 0 {
            state.execdirs_outstanding = 1 as libc::c_int != 0;
        }
        result = 1 as libc::c_int != 0;
    } else {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < (*execp).num_args {
            bc_do_insert(
                &mut (*execp).ctl,
                &mut (*execp).state,
                *((*execp).replace_vec).offset(i as isize),
                strlen(*((*execp).replace_vec).offset(i as isize)),
                prefix,
                pfxlen,
                target,
                strlen(target),
                0 as libc::c_int,
            );
            i += 1;
            i;
        }
        bc_do_exec(&mut (*execp).ctl, &mut (*execp).state);
        if (*execp).last_child_status & 0x7f as libc::c_int == 0 as libc::c_int {
            if 0 as libc::c_int
                == ((*execp).last_child_status & 0xff00 as libc::c_int)
                    >> 8 as libc::c_int
            {
                result = 1 as libc::c_int != 0;
            } else {
                result = 0 as libc::c_int != 0;
            }
        } else {
            result = 0 as libc::c_int != 0;
        }
        if local {
            free_cwd((*execp).wd_for_exec);
        }
    }
    if !buf.is_null() {
        if local {} else {
            __assert_fail(
                b"local\0" as *const u8 as *const libc::c_char,
                b"exec.c\0" as *const u8 as *const libc::c_char,
                208 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 70],
                    &[libc::c_char; 70],
                >(
                    b"_Bool impl_pred_exec(const char *, struct stat *, struct predicate *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_6789: {
            if local {} else {
                __assert_fail(
                    b"local\0" as *const u8 as *const libc::c_char,
                    b"exec.c\0" as *const u8 as *const libc::c_char,
                    208 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 70],
                        &[libc::c_char; 70],
                    >(
                        b"_Bool impl_pred_exec(const char *, struct stat *, struct predicate *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        rpl_free(buf as *mut libc::c_void);
    }
    return result;
}
unsafe extern "C" fn prep_child_for_exec(
    mut close_stdin: bool,
    mut wd: *const saved_cwd,
) -> bool {
    let mut ok: bool = 1 as libc::c_int != 0;
    if close_stdin {
        let inputfile: [libc::c_char; 10] = *::std::mem::transmute::<
            &[u8; 10],
            &[libc::c_char; 10],
        >(b"/dev/null\0");
        if close(0 as libc::c_int) < 0 as libc::c_int {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cannot close standard input\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            ok = 0 as libc::c_int != 0;
        } else if open_safer(inputfile.as_ptr(), 0 as libc::c_int | 0 as libc::c_int)
            < 0 as libc::c_int
        {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                safely_quote_err_filename(0 as libc::c_int, inputfile.as_ptr()),
            );
        }
    }
    if 0 as libc::c_int != restore_cwd(wd) {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to change directory%s%s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            if (*wd).desc < 0 as libc::c_int && !((*wd).name).is_null() {
                b": \0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            if (*wd).desc < 0 as libc::c_int && !((*wd).name).is_null() {
                (*wd).name as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        ok = 0 as libc::c_int != 0;
    }
    return ok;
}
pub unsafe extern "C" fn launch(
    mut ctl: *mut buildcmd_control,
    mut usercontext: *mut libc::c_void,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut child_pid: pid_t = 0;
    static mut first_time: libc::c_int = 1 as libc::c_int;
    let mut execp: *mut exec_val = usercontext as *mut exec_val;
    if options.debug_options & DebugExec as libc::c_int as libc::c_ulong != 0 {
        let mut i: libc::c_int = 0;
        fprintf(
            stderr,
            b"DebugExec: launching process (argc=%lu):\0" as *const u8
                as *const libc::c_char,
            ((*execp).state.cmd_argc).wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        i = 0 as libc::c_int;
        while (i as libc::c_ulong)
            < ((*execp).state.cmd_argc).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            fprintf(
                stderr,
                b" %s\0" as *const u8 as *const libc::c_char,
                safely_quote_err_filename(
                    0 as libc::c_int,
                    *((*execp).state.cmd_argv).offset(i as isize),
                ),
            );
            i += 1;
            i;
        }
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
    rpl_fflush(stdout);
    rpl_fflush(stderr);
    if first_time != 0 {
        first_time = 0 as libc::c_int;
        signal(17 as libc::c_int, None);
    }
    child_pid = fork();
    if child_pid == -(1 as libc::c_int) {
        if ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot fork\0" as *const u8 as *const libc::c_char,
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
                    b"cannot fork\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    if child_pid == 0 as libc::c_int {
        if !((*execp).wd_for_exec).is_null() {} else {
            __assert_fail(
                b"NULL != execp->wd_for_exec\0" as *const u8 as *const libc::c_char,
                b"exec.c\0" as *const u8 as *const libc::c_char,
                321 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"int launch(struct buildcmd_control *, void *, int, char **)\0"))
                    .as_ptr(),
            );
        }
        'c_7961: {
            if !((*execp).wd_for_exec).is_null() {} else {
                __assert_fail(
                    b"NULL != execp->wd_for_exec\0" as *const u8 as *const libc::c_char,
                    b"exec.c\0" as *const u8 as *const libc::c_char,
                    321 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 60],
                        &[libc::c_char; 60],
                    >(b"int launch(struct buildcmd_control *, void *, int, char **)\0"))
                        .as_ptr(),
                );
            }
        };
        if !prep_child_for_exec((*execp).close_stdin, (*execp).wd_for_exec) {
            _exit(1 as libc::c_int);
        } else if fd_leak_check_is_enabled() {
            complain_about_leaky_fds();
        }
        if bc_args_exceed_testing_limit(argv) {
            *__errno_location() = 7 as libc::c_int;
        } else {
            execvp(
                *argv.offset(0 as libc::c_int as isize),
                argv as *const *mut libc::c_char,
            );
        }
        error(
            0 as libc::c_int,
            *__errno_location(),
            b"%s\0" as *const u8 as *const libc::c_char,
            safely_quote_err_filename(
                0 as libc::c_int,
                *argv.offset(0 as libc::c_int as isize),
            ),
        );
        _exit(1 as libc::c_int);
    }
    while waitpid(child_pid, &mut (*execp).last_child_status, 0 as libc::c_int)
        == -(1 as libc::c_int)
    {
        if *__errno_location() != 4 as libc::c_int {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"error waiting for %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                safely_quote_err_filename(
                    0 as libc::c_int,
                    *argv.offset(0 as libc::c_int as isize),
                ),
            );
            state.exit_status = 1 as libc::c_int;
            return 0 as libc::c_int;
        }
    }
    if (((*execp).last_child_status & 0x7f as libc::c_int) + 1 as libc::c_int)
        as libc::c_schar as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s terminated by signal %d\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_n_style(
                0 as libc::c_int,
                options.err_quoting_style,
                *argv.offset(0 as libc::c_int as isize),
            ),
            (*execp).last_child_status & 0x7f as libc::c_int,
        );
        if (*execp).multiple {
            state.exit_status = 1 as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    let mut ex: libc::c_int = ((*execp).last_child_status & 0xff00 as libc::c_int)
        >> 8 as libc::c_int;
    if options.debug_options & DebugExec as libc::c_int as libc::c_ulong != 0 {
        fprintf(
            stderr,
            b"DebugExec: process (PID=%ld) terminated with exit status: %d\n\0"
                as *const u8 as *const libc::c_char,
            child_pid as libc::c_long,
            ex,
        );
    }
    if 0 as libc::c_int == ex {
        return 1 as libc::c_int
    } else {
        if (*execp).multiple {
            state.exit_status = 1 as libc::c_int;
        }
        return 1 as libc::c_int;
    };
}
